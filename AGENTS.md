# AGENTS.md

## Build, Lint, and Test Commands
- Build the project: `cargo build`
- Lint and check style: `cargo clippy --all-targets -- -D warnings` and `cargo fmt -- --check`
- Run all tests: `cargo test`
- Run a single test: `cargo test <test_name>` (exact or substring)

**REQUIRED:** All code must be formatted with `cargo fmt` and pass `cargo clippy --all-targets -- -D warnings` with zero warnings/errors before any commit or pull request. Pull requests that are not lint-clean will not be accepted.

## Code Style Guidelines
- Use `rustfmt` (`cargo fmt`) for formatting; adhere to standard Rust style.
- Imports: Group `use` statements at the top; prefer glob imports from `macroquad::prelude::*` for macroquad APIs.
- Naming: Use `snake_case` for functions/variables, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Types: Prefer explicit types for public APIs; leverage type inference in local scopes.
- Error Handling: Use `Result<T, E>` or panic meaningfully; propagate errors upward or document panics.
- Test organization: Place tests in a `#[cfg(test)] mod tests` module in source files.
- Avoid async/await in test functions; macroquad's main loop is async, but tests run synchronously.
- Each Rust file must start with module-level docs or comments when providing public APIs.
- Avoid unused imports/variables (use `cargo clippy` for enforcement).
- Do not include `AGENTS.md`-specific bots or instructions in code.

_No Cursor or Copilot rules detected as of this writing._


## Branching Strategy
- All new features and large changes should be worked on in a dedicated feature branch (e.g., `feature/factory-ecs-core`).
- Use descriptive branch names: `feature/automation-pipeline`, `feature/inventory-ui`, etc.
- Merge feature branches into `main` only via pull requests, ensuring all formatting and lint checks pass before merging.

