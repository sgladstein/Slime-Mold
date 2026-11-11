//! **Slime Mould** -- herd a slime mould with a lamp, oats, salt, a knife and
//! water. The model and the game are `mould`; `hud` is the bitmap font every
//! screen draws with, `sound` the game's sound (listener, voice bank, device),
//! and `wav` the offline sound writer. `gpu` is the big dish on the GPU (not
//! built for the browser); `world` is the camera, view and controls for a
//! world bigger than the screen.

#[cfg(not(target_arch = "wasm32"))]
pub mod gpu;
pub mod hud;
pub mod mould;
pub mod sound;
pub mod wav;
#[cfg(not(target_arch = "wasm32"))]
pub mod world;
