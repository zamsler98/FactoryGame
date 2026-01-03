# AGENTS.md

## Project Overview
This repository is a Cargo workspace with three crates that separate pure game state, game rules/logic, and the platform-specific Macroquad app:

- `game_core` — pure Rust game state and deterministic updates (ECS-ish). MUST NOT depend on Macroquad or other platform APIs. Headless-testable.
- `game_logic` — game rules, AI, and input handling. Depends on `game_core`. Accepts an `InputFrame` abstraction and modifies the `World`. May define abstract drawing traits but must not call Macroquad directly.
- `game_app` — Macroquad entry point and platform glue. Depends on `game_core` and `game_logic`. Captures platform input, fills `InputFrame`, calls logic, and performs rendering and asset loading via Macroquad.

Assets: use the `assets/` folder at repository root for images/audio/etc. `game_app` is responsible for loading assets at runtime.

## Build, Lint, and Test Commands
- Format everything: `cargo fmt --all`
- Lint (enforced): `cargo clippy --all-targets -- -D warnings`
- Build the whole workspace: `cargo build`
- Build the Macroquad web binary (WASM) for release: `cargo build -p game_app --target wasm32-unknown-unknown --release`
- Run the Macroquad app (desktop): `cargo run -p game_app`
- Run tests (all crates): `cargo test`
- Run tests for a single crate: `cargo test -p game_core` (or exact/substring test names)
- Check one package quickly: `cargo check -p game_app`

**REQUIRED:** All code must be formatted with `cargo fmt` and pass `cargo clippy --all-targets -- -D warnings` with zero warnings/errors before any commit or pull request. Pull requests that are not lint-clean will not be accepted.

## Architecture Rules (must be followed)
- `game_core` MUST NOT depend on Macroquad or platform APIs. Keep it pure, deterministic, and headless-testable.
- `game_logic` depends on `game_core`. It receives platform-agnostic `InputFrame` snapshots and updates the `World`. It may expose abstract drawing hooks (traits) but must not perform platform drawing or input capture.
- `game_app` depends on `game_core` and `game_logic`. It handles Macroquad, input mapping, rendering, and asset loading.
- Keep cross-cutting concerns (I/O, rendering, platform-specific details) in `game_app`. Keep game rules and state in the lower crates.

## Input & Rendering Abstractions
- Use an `InputFrame` struct (in `game_logic`) as the canonical, platform-agnostic input snapshot.
  - `InputFrame` should capture movement axes, actions (pressed), and pointer/touch coordinates (optional).
- `game_logic` may define a `DrawBackend` trait for optional abstract drawing. Implementations live in `game_app`.

## ECS & Determinism
- `game_core` contains the world state and deterministic update functions (e.g., `World::update_physics`).
- If/when switching to a full ECS (e.g., `hecs`, `legion`), do so inside `game_core` to keep the rest of the project agnostic.

## Web (WASM) & CI Notes
- The GitHub Actions workflows should build the `game_app` package explicitly and copy the produced WASM into the `dist/` directory expected by `index.html`.
  - Build command in CI: `cargo build -p game_app --target wasm32-unknown-unknown --release`
  - Copy produced wasm in CI (example): `cp target/wasm32-unknown-unknown/release/game_app.wasm dist/factorygame.wasm`
- `index.html` in the repo currently loads `factorygame.wasm`. CI renames `game_app.wasm` to `factorygame.wasm` to maintain compatibility. Alternatively, the HTML can be updated to reference `game_app.wasm` (choose one approach consistently).
- Optional: add `wasm-opt` step for optimization and caching for Rust dependencies to speed CI.

## CI: recommended checks
- Build WASM for `game_app`
- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Prepare `dist/` and deploy (GitHub Pages or other hosts)

## Code Style & Guidelines
- Use `cargo fmt` for formatting.
- Group `use` statements at the top of files.
- Prefer `snake_case` for functions/variables, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Prefer explicit types for public APIs; use type inference locally as appropriate.
- Provide module-level docs/comments for public crates/files.
- Avoid unused imports/variables; enforce via `cargo clippy`.

## Branching Strategy
- Use feature branches for non-trivial work: `feature/<name>`.
- Open PRs to `main` and ensure `cargo fmt` and `cargo clippy --all-targets -- -D warnings` pass before merging.

## Notes for Contributors
- If you modify crate layout, update CI workflows and `index.html` accordingly.
- Keep `game_core` unit-testable — avoid platform-specific dependencies there.
