# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

Common tasks are wrapped in the top-level `Makefile`; run `make help` to list all targets.

```bash
make fmt      # cargo fmt --all — format all crates
make clippy   # cargo clippy --all-targets -- -D warnings — lint (zero warnings enforced)
make build    # cargo build — build workspace
make run      # cargo run -p game_app — run desktop app
make test     # cargo test — run all tests
make wasm     # cargo build -p game_app --target wasm32-unknown-unknown --release, then copies the output into dist/
make serve    # serve dist/ at http://localhost:8080
make dev      # wasm + serve — build WASM and open it locally
make clean    # cargo clean and remove dist/
```

**All PRs must pass `make fmt` and `make clippy` with zero warnings.**

## Architecture

This is a Cargo workspace with three crates enforcing strict dependency layering:

```
game_core  ←  game_logic  ←  game_app
```

- **`game_core`** — Pure game state: `World` and `TileGrid`. No Macroquad, no platform APIs. Deterministic and headless-testable. Contains the 1000×1000 tile grid and building placement/removal.
- **`game_logic`** — Game rules and building placement. Depends only on `game_core`. Exposes `InputFrame` (platform-agnostic input snapshot) and placement helpers (`try_place_building`, `grid_snapshot`) that `game_app` calls each frame. Must never call Macroquad directly.
- **`game_app`** — Macroquad entry point. Captures platform input, fills `InputFrame`, drives building placement, and renders. Only crate that depends on Macroquad. Handles camera/panning, HUD, and WASM glue.

### Key types

- `TileGrid` (`game_core/src/grid.rs`) — sparse 1000×1000 grid backed by `Vec<Option<InstanceId>>` + `HashMap<InstanceId, BuildingInstance>`. Buildings have `spec_id` (1=conveyor, 2=miner, 3=smelter) and `Rotation`.
- `InputFrame` (`game_logic/src/lib.rs`) — platform-agnostic per-frame input: `action`, `pointer`.
- `render_grid.rs` (`game_app`) — viewport-culled tile renderer; `TILE_PX = 32.0`.

### Web deployment

CI builds `game_app` as WASM, renames `game_app.wasm` → `factorygame.wasm`, and deploys `dist/` to GitHub Pages. `index.html` loads `factorygame.wasm` via the miniquad JS bundle.

## Architectural rules

- `game_core` must never depend on Macroquad or any platform API.
- `game_logic` must never call Macroquad directly — only define abstract traits.
- Keep rendering, input capture, and asset loading in `game_app`.
- If adding a full ECS, do it inside `game_core` to keep the other crates agnostic.
