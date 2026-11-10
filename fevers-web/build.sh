#!/bin/sh
# Builds the browser Two Fevers into page/: the page loads page/fevers.wasm.
# Needs the wasm target once: rustup target add wasm32-unknown-unknown
set -e
cd "$(dirname "$0")"
cargo build --release --locked --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/fevers_web.wasm page/fevers.wasm
ls -l page/fevers.wasm
