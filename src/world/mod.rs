//! **The world's camera and UI** (the Larder world, 2026-11-11): a world
//! bigger than the screen, seen through a scrolling, zooming camera, with a
//! minimap and RTS controls. No game rules: a game gives its units and
//! marks, and maps the `UiCommand`s to its own commands.
//!
//! - `camera`: world <-> screen, pan, zoom at the cursor, edge scroll.
//! - `view`: the GPU pass that draws any world-sized frame (the engine's)
//!   onto the screen through the camera, with the minimap, the marks and a
//!   screen-space overlay.
//! - `marks`: what a game draws over the world (units, bars, buildings,
//!   lines, labels), drawn on the GPU.
//! - `look`: the ground field the view shades the world from (kind and
//!   value a cell), beside the engine's own look field.
//! - `controls`: box select, orders, burn lines, groups, minimap clicks.
//! - `ui`: the game's panels (resources, selection, command card,
//!   tooltips, alerts, menu), drawn by `canvas` (a GPU 2D pass with Inter
//!   text and `icons`) over the view.
//!
//! Not built for wasm32 (the view is wgpu).

pub mod camera;
pub mod canvas;
pub mod controls;
pub mod demo;
pub mod icons;
pub mod look;
pub mod marks;
pub mod start;
pub mod ui;
pub mod view;

pub use camera::Camera;
pub use controls::{Button, Controls, UiCommand, Unit};
pub use marks::{Mark, Marks, Shape};
pub use canvas::{Canvas, CanvasPass};
pub use ui::{Hud, HudEvent, Model};
pub use view::View;
