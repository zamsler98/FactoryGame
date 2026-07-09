# Top-level Makefile for FactoryGame workspace
# Provides common targets for development and CI

SHELL := /bin/bash
WASM_NAME ?= factorygame.wasm
PORT      ?= 8080

.PHONY: help fmt fmt-check clippy build run test add-wasm-target wasm serve dev clean dist

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
	rustup target add wasm32-unknown-unknown

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

clean:
	cargo clean
	rm -rf dist

dist:
	mkdir -p dist
