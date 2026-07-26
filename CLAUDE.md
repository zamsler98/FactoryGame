# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

Common tasks are wrapped in the top-level `Makefile`; run `make help` to list all targets.

```bash
make fmt        # cargo fmt --all — format all crates
make fmt-check  # cargo fmt --all -- --check — verify formatting without rewriting files (CI gate)
make clippy     # cargo clippy --all-targets -- -D warnings — lint (zero warnings enforced)
make build      # cargo build — build workspace
make run        # cargo run -p game_app — run desktop app
make test       # cargo test — run all tests
make wasm       # cargo build -p game_app --target wasm32-unknown-unknown --release, then copies the output into dist/
make serve      # serve dist/ at http://localhost:8080
make dev        # wasm + serve — build WASM and open it locally
make clean      # cargo clean and remove dist/
```

**All PRs must pass `make fmt-check` and `make clippy` with zero warnings.** CI (`.github/workflows/pr-preview.yml`, `deploy-prod.yml`) runs `make wasm`, `make fmt-check`, `make clippy`, and `make test`.

## Architecture

This is a Cargo workspace with three crates enforcing strict dependency layering:

```
game_core  ←  game_logic  ←  game_app
```

- **`game_core`** — Pure game state: `World`, `TileGrid`, the `ResourceLayer` (finite ore patches), the `Factory` simulation, the player `Inventory`, and the hand-`CraftQueue`. No Macroquad, no platform APIs. Deterministic and headless-testable.
- **`game_logic`** — Game rules and building placement/mining. Depends only on `game_core`. Exposes `InputFrame`, placement/mining helpers (`try_place_building`, `mine_at`), and the read-only `view`/`placement` snapshots (`grid_snapshot`, `resource_snapshot`, `factory_snapshot`) that `game_app` renders. Must never call Macroquad directly.
- **`game_app`** — Macroquad entry point. Captures input, drives placement/mining/crafting, and renders the world plus the HUD (inventory, crafting, building palette, inspector). Only crate that depends on Macroquad. Handles camera/pan/zoom and WASM glue.

### Factorio-style model

- **Items** (`item.rs`) are the universal currency — raw ore, smelted plates, intermediates, *and* the building items you place. `BuildingKind::item()` maps a building to the item consumed to place it (returned when mined).
- **Resources** (`resource.rs`) are finite ore patches under the grid; a `Miner` must sit on one and depletes it.
- **Recipes** (`recipe.rs`) are shared: `Smelting` (furnaces, auto-selected from input ore) and `Crafting` (assemblers + the player's hands).
- **Buildings** (`building.rs` / `factory.rs`): `Belt`, `Miner`, `Furnace`, `Inserter`, `Assembler`, `Chest`. Miners/furnaces are **burners** (consume coal fuel). Belts only hand off to other belts; loading/unloading machines requires **inserters** (grab from behind, drop in front) — like Factorio. Assemblers run a player-selected recipe (no electricity yet — a documented simplification).

### Key types

- `TileGrid` (`game_core/src/grid.rs`) — sparse 1000×1000 grid backed by `Vec<Option<InstanceId>>` + `HashMap<InstanceId, BuildingInstance>`. Buildings have `spec_id` (1=belt, 2=miner, 3=furnace, 4=inserter, 5=assembler, 6=chest) and `Rotation`.
- `BuildingState` (`game_core/src/factory.rs`) — per-instance runtime state; `Factory::tick` runs a progress phase then a transfer phase in id order.
- `InputFrame` (`game_logic/src/lib.rs`) — platform-agnostic per-frame input: `action`, `pointer`.
- `render_grid.rs` (`game_app`) — viewport-culled tile renderer; `TILE_PX = 32.0`.

### Web deployment

CI builds `game_app` as WASM, renames `game_app.wasm` → `factorygame.wasm`, and deploys `dist/` to GitHub Pages. `index.html` loads `factorygame.wasm` via the miniquad JS bundle.

## Architectural rules

- `game_core` must never depend on Macroquad or any platform API.
- `game_logic` must never call Macroquad directly — only define abstract traits.
- Keep rendering, input capture, and asset loading in `game_app`.
- If adding a full ECS, do it inside `game_core` to keep the other crates agnostic.
