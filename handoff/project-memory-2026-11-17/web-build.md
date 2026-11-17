---
name: web-build
description: The mould library compiles to wasm32 and runs in a browser Artifact (web/ crate, plain C exports, no wasm-bindgen); how to build and publish
metadata:
  type: reference
---

Found 2026-11-10 (RTS thread): `cargo build --lib --target wasm32-unknown-unknown` works on the slime-mould library as is (pixels/winit compile; no imports in the output). `rustup target add wasm32-unknown-unknown` installs in the cloud container. The `web/` crate (cdylib, `sk_*` extern "C" functions) wraps `render::draw` into an RGBA buffer the page copies to a canvas; `web/build.sh` writes `web/page/skirmish.wasm` (~500 KB). Publish `web/page/index.html` as an Artifact with `files: {"skirmish.wasm": {from, contentType: "application/wasm"}}`; the page fetches it by relative URL.

**Why:** the owner plays from claude.ai links on whichever machine they are at; a browser build avoids `cargo run`.

**How to apply:** any new playable demo can reuse `web/` (add exports) instead of the native window. `std::time::Instant` panics on wasm32: keep it out of per-step paths (the step-cost timers are opt-in). Related: [[rts-direction]].
