# Top-level Makefile for FactoryGame workspace
# Provides common targets for development and CI

SHELL := /bin/bash
WASM_NAME ?= factorygame.wasm

.PHONY: help fmt clippy build run test add-wasm-target wasm clean dist

help:
	@echo "Available targets:"
	@echo "  help            Show this message"
	@echo "  fmt             Run cargo fmt --all"
	@echo "  clippy          Run cargo clippy --all-targets -- -D warnings"
	@echo "  build           cargo build"
	@echo "  run             cargo run -p game_app"
	@echo "  test            cargo test"
	@echo "  add-wasm-target Add the wasm target: rustup target add wasm32-unknown-unknown"
	@echo "  wasm            Build game_app for wasm and copy to dist/$(WASM_NAME)"
	@echo "  clean           cargo clean and remove dist/"

fmt:
	cargo fmt --all

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
	@echo "WASM built and copied to dist/$(WASM_NAME)"

clean:
	cargo clean
	rm -rf dist

dist:
	mkdir -p dist

# Notes:
# - The wasm target copies game_app.wasm to dist/factorygame.wasm to remain compatible with index.html
# - CI may prefer calling `make wasm` to build and prepare dist/
