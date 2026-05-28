# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo fmt --all                                                        # format all crates
cargo clippy --all-targets -- -D warnings                             # lint (zero warnings enforced)
cargo build                                                            # build workspace
cargo run -p game_app                                                  # run desktop app
cargo test                                                             # run all tests
cargo test -p game_core                                                # run tests for one crate
cargo check -p game_app                                                # fast type-check one crate
cargo build -p game_app --target wasm32-unknown-unknown --release      # build WASM for web
```

**All PRs must pass `cargo fmt -- --check` and `cargo clippy --all-targets -- -D warnings` with zero warnings.**

## Architecture

This is a Cargo workspace with three crates enforcing strict dependency layering:

```
game_core  ←  game_logic  ←  game_app
```

- **`game_core`** — Pure game state: `World`, `Entity`, and `TileGrid`. No Macroquad, no platform APIs. Deterministic and headless-testable. Contains the 1000×1000 tile grid, building placement/removal, entity physics integration.
- **`game_logic`** — Game rules and input processing. Depends only on `game_core`. Consumes `InputFrame` (platform-agnostic input snapshot) and calls `update_world`. May define abstract traits (e.g., `DrawBackend`) but must never call Macroquad.
- **`game_app`** — Macroquad entry point. Captures platform input, fills `InputFrame`, calls `update_world`, and renders. Only crate that depends on Macroquad. Handles camera/panning, HUD, and WASM glue.

### Key types

- `TileGrid` (`game_core/src/grid.rs`) — sparse 1000×1000 grid backed by `Vec<Option<InstanceId>>` + `HashMap<InstanceId, BuildingInstance>`. Buildings have `spec_id` (1=conveyor, 2=miner, 3=smelter) and `Rotation`.
- `InputFrame` (`game_logic/src/lib.rs`) — platform-agnostic per-frame input: `move_x/y`, `action`, `pointer`.
- `render_grid.rs` (`game_app`) — viewport-culled tile renderer; `TILE_PX = 32.0`.

### Web deployment

CI builds `game_app` as WASM, renames `game_app.wasm` → `factorygame.wasm`, and deploys `dist/` to GitHub Pages. `index.html` loads `factorygame.wasm` via the miniquad JS bundle.

## Architectural rules

- `game_core` must never depend on Macroquad or any platform API.
- `game_logic` must never call Macroquad directly — only define abstract traits.
- Keep rendering, input capture, and asset loading in `game_app`.
- If adding a full ECS, do it inside `game_core` to keep the other crates agnostic.
