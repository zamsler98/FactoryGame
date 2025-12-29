#!/bin/bash

echo "=== Starting Opencode automation script ==="

# Get the latest session ID
echo "Fetching latest Opencode session ID..."
SESSION_ID=$(opencode session list | awk '/^ses_/ {print $1; exit}')
echo "Using session: $SESSION_ID"

# Step 1: Run the implement prompt
echo "Running implement prompt in session..."
opencode run "Implement. Just modify files. Do not build, run tests, or commit." \
    -s "$SESSION_ID" --model github-copilot/gpt-5-mini
echo "Implement prompt complete."

# Step 2: Build the project silently, capturing output
echo "Building project (silently)..."
BUILD_OUTPUT=$(cargo build --target wasm32-unknown-unknown --release 2>&1)
BUILD_STATUS=$?

# Step 3: If build fails, pass the output to IssueReporter
if [ $BUILD_STATUS -ne 0 ]; then
    echo "Build failed. Sending output to IssueReporter agent for summarization..."
    SUMMARY=$(opencode run "$BUILD_OUTPUT" --agent IssueReporter)
    
    echo "Build summary received from IssueReporter:"
    echo "$SUMMARY"
    
    echo "Sending summary back to main session for AI to fix..."
    opencode run "The build failed with the following summarized errors:\n$SUMMARY\nPlease fix them." \
        -s "$SESSION_ID" --model github-copilot/gpt-5-mini
else
    echo "Build succeeded! No further action needed."
fi

echo "=== Script finished ==="

