---
name: create-pr
description: Create a pull request for FactoryGame end-to-end — branch off main if needed, run and auto-fix the same checks CI enforces (fmt, clippy, tests, wasm build), push, and open the PR via gh (or report the existing one if there already is one for this branch). Use this whenever the user asks to "create a PR", "open a PR", "put up a pull request", "ship this", or says the current changes are ready to submit/review — even if they don't mention branches, checks, or gh explicitly.
---

This skill turns a set of local changes into an open, CI-clean pull
request against `main` with no back-and-forth. It exists because
"create a PR" implicitly means "create a PR that will pass CI" — pushing
something that immediately fails `make fmt-check` or `make clippy`
(both zero-warning gates per `CLAUDE.md`) just creates rework.

Run the steps below in order. Don't skip ahead to `gh pr create` before
the checks in step 4 are actually green.

## 1. Check the current branch

```bash
git branch --show-current
```

If it's anything other than `main`, skip to step 3 — you're already on
a feature branch, just keep working on it.

## 2. If on main, branch first — before committing anything

Look at what's actually changed (`git status`, `git diff`, `git diff
--staged`, and the conversation context) to understand the nature of
the change, then create a branch **before** making any commits, so
`main` never accumulates local commits even transiently:

```bash
git checkout -b <prefix>/<slug>
```

This repo's existing branches (`git branch -a` / `gh pr list`) follow a
consistent convention — reuse it rather than inventing a new one:

- `feature/<slug>` — new functionality (most common)
- `fix/<slug>` — bug fixes
- `chore/<slug>` — tooling, deps, CI, cleanup
- `docs/<slug>` — documentation only

`<slug>` is a short kebab-case description of the change (2-4 words),
e.g. `feature/inserter-arm`, `fix/conveyor-render-flicker`.

## 3. Make sure there's something to commit and push

```bash
git status
```

- If there are uncommitted changes, commit them using the standard git
  commit workflow (check `git status`/`git diff`/`git log`, draft an
  accurate message explaining *why*, stage relevant files by name, commit
  with the `Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>`
  trailer). Don't stage anything that looks like it could contain secrets.
- If the working tree is clean *and* the branch has no commits ahead of
  `main`, there is nothing to open a PR for — stop and tell the user.

## 4. Run the PR checks — mirror what CI actually runs

CI (`.github/workflows/pr-preview.yml`, `deploy-prod.yml`) runs `make
wasm`, `make fmt-check`, `make clippy`, and `make test`. Run all four
locally before pushing:

```bash
make fmt-check
make clippy
make test
make wasm
```

**Known environment quirk, verified in this container:** `make wasm`
depends on `add-wasm-target`, which shells out to `rustup target add
wasm32-unknown-unknown`. `rustup` is not on `PATH` here even though the
`wasm32-unknown-unknown` target itself is already installed (via the
system Rust package). If `make wasm` fails with `rustup: command not
found`, that's this known issue, not a real build failure — fall back to
running the underlying compile step directly, which is the part CI
actually cares about for mergeability:

```bash
cargo build -p game_app --target wasm32-unknown-unknown --release
```

(See the `run-factorygame` skill for more background on this
container's quirks, e.g. `make serve` also needing a workaround.)

## 5. On failure, auto-fix what's mechanically fixable, then re-check

- **`make fmt-check` fails** → run `make fmt` (rewrites files in
  place), then fold the fix into the existing commit with `git commit
  --amend --no-edit`. Amending is safe here specifically because these
  commits haven't been pushed to the remote yet — this is not the
  general "avoid amend" case, it's a same-commit formatting touch-up
  before the branch has ever left your machine.
- **`make clippy` fails** → run `cargo clippy --fix --all-targets
  --allow-dirty --allow-staged`, then re-run `make clippy` and amend as
  above. Not every lint is auto-fixable; if warnings remain, stop and
  show the user the remaining output rather than guessing at a semantic
  fix.
- **`make test` fails** → there's no mechanical fix for a failing test.
  Stop, report the failure output, and do not push or open a PR.
- **wasm build fails** → almost always a genuine compile error. Stop and
  report it, same as a test failure.

After any auto-fix round, re-run the full check list from the top once
more — a `clippy --fix` pass can change formatting, for instance — before
moving on to push.

Never skip hooks (`--no-verify`) or force-push to make a check "pass."

## 6. Push

```bash
git push -u origin <branch>   # first push on this branch
git push                      # subsequent pushes, already tracking
```

## 7. Check whether a PR already exists for this branch

```bash
gh pr list --head <branch> --state open --json number,url
```

If one exists, you're done — report its URL to the user and stop. Don't
create a duplicate.

## 8. Otherwise, create the PR

Look at the *entire* set of commits the PR will contain, not just the
latest one:

```bash
git log main..HEAD
git diff main...HEAD
```

Then open it against `main`:

```bash
gh pr create --base main --title "<short, imperative, <70 chars>" --body "$(cat <<'EOF'
## Summary
- <bullet 1>
- <bullet 2>

## Test plan
- [x] make fmt-check
- [x] make clippy
- [x] make test
- [x] make wasm (or the cargo build fallback from step 4)
EOF
)"
```

Report the resulting PR URL to the user.

## Verified environment facts (this container, this session)

- `gh` is installed and authenticated as `zamsler98` over HTTPS; `origin`
  points at `https://github.com/zamsler98/FactoryGame.git`.
- `rustup` is **not** on `PATH`, but the `wasm32-unknown-unknown` target
  is already present — see step 4's fallback.
- Existing PRs in this repo are consistently `<branch> → main`; there's
  no precedent for PRs targeting anything else.
