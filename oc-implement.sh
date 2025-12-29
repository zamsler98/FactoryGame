#!/bin/bash

# Directory to store logs
LOG_DIR="./oc_logs"
mkdir -p "$LOG_DIR"

# Create a new log file with timestamp
LOG_FILE="$LOG_DIR/oc_run_$(date +%Y%m%d_%H%M%S).log"

timestamp() {
    date +"%Y-%m-%d %H:%M:%S"
}

log() {
    echo "[$(timestamp)] $1" | tee -a "$LOG_FILE"
}

log "=== Starting Opencode automation script ==="

MAX_ATTEMPTS=5
ATTEMPT=1

log "Fetching latest Opencode session ID..."
SESSION_ID=$(opencode session list | awk '/^ses_/ {print $1; exit}')
log "Using session: $SESSION_ID"

run_and_fix() {
    local STEP_NAME="$1"
    local COMMAND="$2"

    log "Running $STEP_NAME (silently)..."

    OUTPUT=$(eval "$COMMAND" 2>&1 | tee -a "$LOG_FILE")
    STATUS=${PIPESTATUS[0]}

    if [ $STATUS -eq 0 ]; then
        log "$STEP_NAME passed."
        return 0
    fi

    log "$STEP_NAME failed. Sending output to issue-reporter..."
    
    SUMMARY=$(opencode run "$OUTPUT" \
        --agent issue-reporter \
        --model github-copilot/gpt-5-mini 2>>"$LOG_FILE")
    
    log "Summary received for $STEP_NAME. Sending to main session..."
    
    opencode run "The following step failed: $STEP_NAME

$SUMMARY

Please fix the issues.
Only modify files.
Do not run formatting, linting, builds, tests, or commit." \
        -s "$SESSION_ID" \
        --model github-copilot/gpt-5-mini \
        >>"$LOG_FILE" 2>&1

    return 1
}

while [ $ATTEMPT -le $MAX_ATTEMPTS ]; do
    log "=== Attempt $ATTEMPT of $MAX_ATTEMPTS ==="

    log "Running implement prompt in session..."
    opencode run "Implement. Just modify files. Do not build, run tests, or commit." \
        -s "$SESSION_ID" \
        --model github-copilot/gpt-5-mini \
        >>"$LOG_FILE" 2>&1
    log "Implement prompt complete."

    run_and_fix "cargo fmt"    "cargo fmt -- --check" || { ATTEMPT=$((ATTEMPT + 1)); continue; }
    run_and_fix "cargo clippy" "cargo clippy --all-targets -- -D warnings" || { ATTEMPT=$((ATTEMPT + 1)); continue; }
    run_and_fix "cargo build"  "cargo build --target wasm32-unknown-unknown --release" || { ATTEMPT=$((ATTEMPT + 1)); continue; }
    run_and_fix "cargo test"   "cargo test" || { ATTEMPT=$((ATTEMPT + 1)); continue; }

    log "🎉 All checks passed on attempt $ATTEMPT!"

    # Generate commit message
    log "Requesting commit message from main session..."
    COMMIT_MSG=$(opencode run "All changes are implemented and verified. Generate a concise git commit message describing the changes." \
        -s "$SESSION_ID" \
        --model github-copilot/gpt-5-mini 2>>"$LOG_FILE")
    log "Commit message received: $COMMIT_MSG"

    # Commit and push
    log "Adding changes..."
    git add . >>"$LOG_FILE" 2>&1
    log "Committing changes..."
    git commit -m "$COMMIT_MSG" >>"$LOG_FILE" 2>&1

    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    log "Pushing changes to branch '$CURRENT_BRANCH'..."
    git push --set-upstream origin "$CURRENT_BRANCH" >>"$LOG_FILE" 2>&1
    log "✅ Commit and push completed successfully."

    # Handle PR creation or update
    if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
        # Check for existing PR
        EXISTING_PR=$(gh pr list --head "$CURRENT_BRANCH" --state open --json number,url,title 2>>"$LOG_FILE" | jq -r '.[0]')
        
        # Generate PR title and body from main session
        log "Requesting PR title and body from main session..."
        PR_INFO=$(opencode run "Generate a descriptive pull request title and body for branch '$CURRENT_BRANCH'. Return as JSON: {\"title\":\"...\", \"body\":\"...\"}" \
            -s "$SESSION_ID" \
            --model github-copilot/gpt-5-mini 2>>"$LOG_FILE")
        PR_TITLE=$(echo "$PR_INFO" | jq -r '.title')
        PR_BODY=$(echo "$PR_INFO" | jq -r '.body')

        if [[ -z "$EXISTING_PR" || "$EXISTING_PR" == "null" ]]; then
            log "No existing PR found. Creating new pull request..."
            PR_URL=$(gh pr create --base main --head "$CURRENT_BRANCH" --title "$PR_TITLE" --body "$PR_BODY" 2 --draft >>"$LOG_FILE")
            log "Pull request created: $PR_URL"
        else
            PR_NUMBER=$(echo "$EXISTING_PR" | jq -r '.number')
            log "Existing PR #$PR_NUMBER found. Updating title and body..."
            gh pr edit "$PR_NUMBER" --title "$PR_TITLE" --body "$PR_BODY" 2>>"$LOG_FILE"
            log "Pull request #$PR_NUMBER updated successfully."
        fi
    else
        log "On main/master branch; skipping PR creation."
    fi

    log "=== Script finished successfully ==="
    exit 0
done

log "🚨 Failed after $MAX_ATTEMPTS attempts."
log "=== Script finished with failure ==="
exit 1
