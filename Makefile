# Top-level Makefile for FactoryGame workspace
# Provides common targets for development and CI

SHELL := /bin/bash
WASM_NAME ?= factorygame.wasm
PORT      ?= 8080

.PHONY: help fmt fmt-check clippy build run test add-wasm-target wasm serve dev watch dev-watch clean dist

help:
	@echo "Available targets:"
	@echo "  help            Show this message"
	@echo "  fmt             Run cargo fmt --all"
	@echo "  fmt-check       Run cargo fmt --all -- --check (CI formatting gate)"
	@echo "  clippy          Run cargo clippy --all-targets -- -D warnings"
	@echo "  build           cargo build"
	@echo "  run             cargo run -p game_app"
	@echo "  test            cargo test"
	@echo "  add-wasm-target Add the wasm target: rustup target add wasm32-unknown-unknown"
	@echo "  wasm            Build game_app for wasm and copy to dist/$(WASM_NAME)"
	@echo "  serve           Serve dist/ on http://localhost:$(PORT)"
	@echo "  dev             wasm + serve (build and open in browser)"
	@echo "  watch           rebuild dist/ automatically whenever source files change (requires entr)"
	@echo "  dev-watch       serve dist/ and rebuild automatically on change (requires entr)"
	@echo "  clean           cargo clean and remove dist/"

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets -- -D warnings

build:
	cargo build

run:
	cargo run -p game_app

test:
	cargo test

add-wasm-target:
	@if command -v rustup >/dev/null 2>&1; then \
		rustup target add wasm32-unknown-unknown; \
	elif [ -d "$$(rustc --print sysroot)/lib/rustlib/wasm32-unknown-unknown/lib" ]; then \
		echo "rustup not found; wasm32-unknown-unknown already available via system rustc, skipping"; \
	else \
		echo "error: rustup not found and wasm32-unknown-unknown target is unavailable." >&2; \
		echo "Install rustup, or install your distro's wasm target package (e.g. 'pacman -S rust-wasm')." >&2; \
		exit 1; \
	fi

wasm: add-wasm-target
	cargo build -p game_app --target wasm32-unknown-unknown --release
	mkdir -p dist
	cp target/wasm32-unknown-unknown/release/game_app.wasm dist/$(WASM_NAME)
	cp index.html dist/
	@echo "WASM built and copied to dist/$(WASM_NAME)"

serve:
	@echo "Open http://localhost:$(PORT) in your browser"
	python3 -m http.server $(PORT) --directory dist

dev: wasm serve

WATCH_PATHS := game_core/src game_logic/src game_app/src index.html Cargo.toml game_core/Cargo.toml game_logic/Cargo.toml game_app/Cargo.toml

watch:
	@command -v entr >/dev/null 2>&1 || { echo "error: entr not found. Install it with 'sudo apt-get install entr'." >&2; exit 1; }
	@echo "Watching for changes... (Ctrl+C to stop)"
	@find $(WATCH_PATHS) -type f | entr -r $(MAKE) wasm

dev-watch:
	@command -v entr >/dev/null 2>&1 || { echo "error: entr not found. Install it with 'sudo apt-get install entr'." >&2; exit 1; }
	@$(MAKE) wasm
	@python3 -m http.server $(PORT) --directory dist & \
	SERVER_PID=$$!; \
	trap "kill $$SERVER_PID" EXIT; \
	echo "Serving http://localhost:$(PORT) — watching for changes... (Ctrl+C to stop)"; \
	find $(WATCH_PATHS) -type f | entr -r $(MAKE) wasm

clean:
	cargo clean
	rm -rf dist

dist:
	mkdir -p dist
