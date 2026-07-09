---
name: run-factorygame
description: Build, test, lint, and package FactoryGame (a Macroquad/Rust factory-building game). Use when asked to build FactoryGame, run its tests, lint/format it, or produce the WASM web build. Does NOT cover interactively driving the running game — this container is headless and no GUI/browser driver is set up (see Limitations).
---

FactoryGame is a Cargo workspace (`game_core` ← `game_logic` ← `game_app`)
built with Macroquad. It has two run targets: a native desktop binary
(`cargo run -p game_app`, opens an OS window) and a WASM build loaded by
`index.html` in a browser (this is what CI deploys to GitHub Pages). All
commands below are wrapped by the top-level `Makefile` — run `make help`
to list targets. All paths are relative to the repo root.

## Limitations (read this first)

**There is no interactive driver in this skill.** This container has no
X server and no browser installed, so nothing here can click, type into,
or screenshot the running game. What follows is verified build/test/lint
tooling only — every command below was actually run in this container
this session and its real output is reflected in Gotchas/Troubleshooting.

If a future task needs to *see* the game running, someone will need to
either install Chromium + drive the WASM build (`make wasm` + `make
serve`, then a browser automation tool against `dist/index.html`), or
install `libXi.so.6` + an X server (e.g. Xvfb) to run the native binary
under `cargo run -p game_app`. Neither is set up here.

## Prerequisites

- Rust toolchain with the `wasm32-unknown-unknown` target for WASM
  builds. In this container the target was already present (Arch's
  system `rust` package), but there is **no `rustup` binary**, so
  `make wasm`'s `add-wasm-target` prerequisite (`rustup target add
  wasm32-unknown-unknown`) fails here — see Troubleshooting.
- `python3` for `make serve` — **not installed in this container** (see
  Troubleshooting for a workaround).

## Build

```bash
cargo build                # native debug build of the whole workspace
make build                 # same thing, via the Makefile
```

For the WASM/web build, since `make wasm`'s `add-wasm-target` step fails
without `rustup` (see Troubleshooting), run its remaining steps directly:

```bash
cargo build -p game_app --target wasm32-unknown-unknown --release
mkdir -p dist
cp target/wasm32-unknown-unknown/release/game_app.wasm dist/factorygame.wasm
cp index.html dist/
```

This produces `dist/index.html` + `dist/factorygame.wasm` — confirmed
servable (verified with a throwaway Node static server, since `python3`
isn't installed here):

```bash
curl -sI http://localhost:PORT/index.html      # → 200
curl -sI http://localhost:PORT/factorygame.wasm # → 200
```

## Run (human path — needs a display/browser, untested past this point)

- **Native:** `make run` (i.e. `cargo run -p game_app`). In this
  container this fails immediately — see Troubleshooting
  (`libXi.so.6` not found). Even with that lib installed, it still
  needs a real or virtual (Xvfb) X display, which isn't set up here.
- **Web:** `make wasm && make serve`, then open `http://localhost:8080`
  in a browser. `make serve` requires `python3`, which isn't installed
  in this container (see Troubleshooting for a workaround using Node).

## Test

```bash
cargo test              # or: make test
```

Verified this session: all suites pass — `game_core`'s
`tests/grid_tests.rs` (3 tests: placement, overlap/bounds, rotation
footprint) plus the inline unit tests, `game_logic`, and `game_app`
(0 tests in the latter two).

## Lint / format

```bash
cargo fmt --all -- --check   # or: make fmt-check
cargo clippy --all-targets -- -D warnings   # or: make clippy
```

Verified clean (zero warnings, zero formatting diffs) this session.

## Gotchas

- **No `rustup` in this container.** The `wasm32-unknown-unknown` target
  is present (installed as part of the system Rust package), but
  `rustup` itself isn't on `PATH`. `make wasm` calls `add-wasm-target`
  (`rustup target add wasm32-unknown-unknown`) as a prerequisite and
  fails with `rustup: command not found` before it ever reaches the
  actual build. Skip `make wasm` here; run the `cargo build -p game_app
  --target wasm32-unknown-unknown --release` + `cp` steps from the Build
  section directly instead.
- **No `python3` in this container.** `make serve` shells out to
  `python3 -m http.server`, which fails with `python3: command not
  found`. Node is installed, so a one-off static server works as a
  substitute (see Troubleshooting) — but this isn't wired into the
  Makefile.

## Troubleshooting

- **`make wasm` fails with `rustup: command not found` (exit 127) on the
  `add-wasm-target` step**: no `rustup` binary in this container. Run
  the underlying cargo build manually — see Build section above.
- **`make serve` fails with `python3: command not found` (exit 127)**:
  no Python in this container. Serve `dist/` with Node instead:
  ```bash
  node -e "
  const http=require('http'),fs=require('fs'),path=require('path');
  http.createServer((req,res)=>{
    fs.readFile(path.join('dist', req.url==='/'?'index.html':req.url), (err,data)=>{
      if (err) { res.writeHead(404); res.end(); return; }
      res.writeHead(200); res.end(data);
    });
  }).listen(8080);
  "
  ```
- **`cargo run -p game_app` panics with `X11 backend failed:
  LibraryNotFound(DlOpenError("libXi.so.6"))`**: no X11 Xinput extension
  library installed, and no X server running anyway. Confirmed by
  actually running it in this container — this is not a documentation
  guess. Installing `libxi` (`pacman -S libxi` on Arch) would clear this
  specific error, but a display (real or Xvfb) is still required beyond
  that, which is out of scope for this skill as written.
