#!/bin/sh
# Builds the browser skirmish into page/: the page loads page/skirmish.wasm.
# Needs the wasm target once: rustup target add wasm32-unknown-unknown
set -e
cd "$(dirname "$0")"
cargo build --release --locked --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/slime_web.wasm page/skirmish.wasm
ls -l page/skirmish.wasm
