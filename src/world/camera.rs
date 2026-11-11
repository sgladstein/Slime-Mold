//! **The camera**: which part of a world bigger than the screen is shown,
//! and how big. Pure arithmetic, no window and no GPU, so it is tested on
//! its own and any game can use it.
//!
//! The camera is the world point at the screen's top-left corner (`x`, `y`,
//! in world cells) and a zoom (screen pixels a world cell). Zoom runs from
//! "the whole world fits" up to `MAX_ZOOM`. Below 1 the view pass averages
//! the cells under each pixel; at 1 and above it shows each cell as a block,
//! so the picture is still the simulation's data, never a smoothing of it.
//! Zoom 1 and above snaps to whole steps (`ZOOMS`), so every cell is the
//! same number of pixels across.

/// The largest zoom: a cell 8 pixels across.
pub const MAX_ZOOM: f32 = 8.0;

/// Zoom levels the wheel steps through at 1 and above; below 1 each notch
/// is a factor of `ZOOM_OUT`, down to the fit.
pub const ZOOMS: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 6.0, 8.0];
pub const ZOOM_OUT: f32 = 1.25;

/// How fast the edge scroll and the arrow keys pan, in screen pixels a
/// second (so the same at every zoom), and how close to the edge the
/// cursor must be.
pub const PAN_SPEED: f32 = 1100.0;
pub const EDGE: f32 = 6.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    /// The world's size, in cells.
    pub world: (f32, f32),
    /// The screen's size, in pixels.
    pub screen: (f32, f32),
    /// The world point at the screen's top-left, in cells.
    pub x: f32,
    pub y: f32,
    /// Screen pixels a world cell.
    pub zoom: f32,
}

impl Camera {
    /// A camera on the world's middle: at zoom 1 when the world is larger
    /// than the screen (you scroll it), at the largest whole zoom that fits
    /// when it is smaller (a small dish fills the window as it always did).
    pub fn new(world: (usize, usize), screen: (u32, u32)) -> Self {
        let mut c = Camera { world: (world.0 as f32, world.1 as f32), screen: (screen.0 as f32, screen.1 as f32), x: 0.0, y: 0.0, zoom: 1.0 };
        let fit = c.fit();
        c.zoom = if fit >= 1.0 { fit.floor().min(MAX_ZOOM) } else { 1.0 };
        c.centre_on((c.world.0 / 2.0, c.world.1 / 2.0));
        c
    }

    /// The zoom at which the whole world just fits on the screen.
    pub fn fit(&self) -> f32 {
        (self.screen.0 / self.world.0).min(self.screen.1 / self.world.1)
    }

    /// The smallest zoom allowed: the fit, or 1 if the world fits at 1.
    pub fn min_zoom(&self) -> f32 {
        self.fit().min(1.0)
    }

    /// The world cells the screen shows across and down.
    pub fn span(&self) -> (f32, f32) {
        (self.screen.0 / self.zoom, self.screen.1 / self.zoom)
    }

    /// Whether the whole world is on screen.
    pub fn sees_all(&self) -> bool {
        let (w, h) = self.span();
        w >= self.world.0 - 0.5 && h >= self.world.1 - 0.5
    }

    pub fn to_world(&self, s: (f32, f32)) -> (f32, f32) {
        (self.x + s.0 / self.zoom, self.y + s.1 / self.zoom)
    }

    pub fn to_screen(&self, w: (f32, f32)) -> (f32, f32) {
        ((w.0 - self.x) * self.zoom, (w.1 - self.y) * self.zoom)
    }

    /// A world point under the cursor, if it is on the world.
    pub fn world_at(&self, s: (f32, f32)) -> Option<(f32, f32)> {
        let w = self.to_world(s);
        (w.0 >= 0.0 && w.1 >= 0.0 && w.0 < self.world.0 && w.1 < self.world.1).then_some(w)
    }

    /// Keeps the view on the world: an axis the screen spans wider than the
    /// world is centred, otherwise the view stops at the world's edge.
    pub fn clamp(&mut self) {
        let (w, h) = self.span();
        self.x = if w >= self.world.0 { (self.world.0 - w) / 2.0 } else { self.x.clamp(0.0, self.world.0 - w) };
        self.y = if h >= self.world.1 { (self.world.1 - h) / 2.0 } else { self.y.clamp(0.0, self.world.1 - h) };
    }

    pub fn centre_on(&mut self, w: (f32, f32)) {
        let (sw, sh) = self.span();
        self.x = w.0 - sw / 2.0;
        self.y = w.1 - sh / 2.0;
        self.clamp();
    }

    /// The world point at the screen's middle.
    pub fn centre(&self) -> (f32, f32) {
        let (sw, sh) = self.span();
        (self.x + sw / 2.0, self.y + sh / 2.0)
    }

    /// Moves the view by a drag of `d` screen pixels (the world follows the
    /// mouse).
    pub fn drag(&mut self, d: (f32, f32)) {
        self.x -= d.0 / self.zoom;
        self.y -= d.1 / self.zoom;
        self.clamp();
    }

    /// Pans by `d` screen pixels (the view moves that way).
    pub fn pan(&mut self, d: (f32, f32)) {
        self.drag((-d.0, -d.1));
    }

    /// Sets the zoom, keeping the world point under screen point `at` where
    /// it is.
    pub fn zoom_to(&mut self, zoom: f32, at: (f32, f32)) {
        let before = self.to_world(at);
        self.zoom = zoom.clamp(self.min_zoom(), MAX_ZOOM);
        self.x = before.0 - at.0 / self.zoom;
        self.y = before.1 - at.1 / self.zoom;
        self.clamp();
    }

    /// The next zoom in (`notches` > 0) or out, `notches` steps on.
    pub fn stepped(&self, notches: i32) -> f32 {
        let mut z = self.zoom;
        for _ in 0..notches.unsigned_abs() {
            z = if notches > 0 {
                if z < 1.0 {
                    (z * ZOOM_OUT).min(1.0)
                } else {
                    ZOOMS.iter().copied().find(|&s| s > z + 1e-3).unwrap_or(MAX_ZOOM)
                }
            } else if z > 1.0 + 1e-3 {
                ZOOMS.iter().rev().copied().find(|&s| s < z - 1e-3).unwrap_or(1.0)
            } else {
                z / ZOOM_OUT
            };
        }
        z.clamp(self.min_zoom(), MAX_ZOOM)
    }

    /// The wheel: `notches` steps in or out at the cursor.
    pub fn wheel(&mut self, notches: i32, at: (f32, f32)) {
        let z = self.stepped(notches);
        self.zoom_to(z, at);
    }

    /// The edge scroll over `dt` seconds: a cursor within `EDGE` pixels of
    /// a screen edge pans that way. Returns whether it moved.
    pub fn edge_scroll(&mut self, cursor: (f32, f32), dt: f32) -> bool {
        let step = PAN_SPEED * dt;
        let dx = if cursor.0 < EDGE { -step } else if cursor.0 >= self.screen.0 - EDGE { step } else { 0.0 };
        let dy = if cursor.1 < EDGE { -step } else if cursor.1 >= self.screen.1 - EDGE { step } else { 0.0 };
        let before = (self.x, self.y);
        self.pan((dx, dy));
        (self.x, self.y) != before
    }

    /// A new screen size (the window resized), keeping the middle.
    pub fn resize(&mut self, screen: (u32, u32)) {
        let c = self.centre();
        self.screen = (screen.0 as f32, screen.1 as f32);
        self.zoom = self.zoom.clamp(self.min_zoom(), MAX_ZOOM);
        self.centre_on(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: (f32, f32), b: (f32, f32)) -> bool {
        (a.0 - b.0).abs() < 1e-3 && (a.1 - b.1).abs() < 1e-3
    }

    #[test]
    fn screen_and_world_are_inverse() {
        let mut c = Camera::new((2048, 1280), (1280, 800));
        c.zoom_to(3.0, (100.0, 50.0));
        for s in [(0.0, 0.0), (640.0, 400.0), (1279.0, 799.0), (13.5, 700.25)] {
            assert!(near(c.to_screen(c.to_world(s)), s));
        }
    }

    #[test]
    fn a_big_world_opens_at_zoom_1_in_the_middle_and_a_small_one_fills_the_window() {
        let c = Camera::new((2048, 1280), (1280, 800));
        assert_eq!(c.zoom, 1.0);
        assert!(near(c.centre(), (1024.0, 640.0)));
        // Today's 1024 x 640 dish in a 1024 x 640 window: all of it, at 1.
        let d = Camera::new((1024, 640), (1024, 640));
        assert!(d.sees_all() && d.zoom == 1.0 && near((d.x, d.y), (0.0, 0.0)));
        // A 256 x 160 dish in a 1024 x 700 window: zoom 4, centred down.
        let e = Camera::new((256, 160), (1024, 700));
        assert_eq!(e.zoom, 4.0);
        assert!(near((e.x, e.y), (0.0, -(700.0 / 4.0 - 160.0) / 2.0)));
    }

    #[test]
    fn the_wheel_keeps_the_point_under_the_cursor() {
        let mut c = Camera::new((2048, 1280), (1280, 800));
        let at = (900.0, 300.0);
        let w = c.to_world(at);
        for n in [1, 1, 2, -1, -3, -1, -1] {
            c.wheel(n, at);
            // Unless the clamp had to move the view, the point stays put.
            let (sw, sh) = c.span();
            if c.x > 0.0 && c.y > 0.0 && c.x < c.world.0 - sw && c.y < c.world.1 - sh {
                assert!(near(c.to_world(at), w), "zoom {}", c.zoom);
            }
        }
    }

    #[test]
    fn zoom_steps_are_whole_above_1_and_stop_at_the_fit() {
        let mut c = Camera::new((2048, 1280), (1280, 800));
        let mut seen = vec![c.zoom];
        for _ in 0..8 {
            c.wheel(1, (640.0, 400.0));
            seen.push(c.zoom);
        }
        assert_eq!(&seen[..6], &[1.0, 2.0, 3.0, 4.0, 6.0, 8.0]);
        assert_eq!(*seen.last().unwrap(), MAX_ZOOM);
        for _ in 0..30 {
            c.wheel(-1, (640.0, 400.0));
        }
        assert!((c.zoom - c.fit()).abs() < 1e-6 && c.sees_all(), "zoomed out to {}", c.zoom);
    }

    #[test]
    fn the_view_never_leaves_the_world() {
        let mut c = Camera::new((2048, 1280), (1280, 800));
        c.drag((5000.0, 5000.0));
        assert!(near((c.x, c.y), (0.0, 0.0)));
        c.drag((-9000.0, -9000.0));
        assert!(near((c.x, c.y), (2048.0 - 1280.0, 1280.0 - 800.0)));
        // Zoomed in, the far corner is still the world's.
        c.zoom_to(4.0, (1280.0, 800.0));
        let far = c.to_world((1280.0, 800.0));
        assert!(far.0 <= 2048.0 + 1e-3 && far.1 <= 1280.0 + 1e-3);
    }

    #[test]
    fn the_edge_scrolls_and_the_middle_does_not() {
        let mut c = Camera::new((2048, 1280), (1280, 800));
        let x = c.x;
        assert!(!c.edge_scroll((640.0, 400.0), 0.1));
        assert!(c.edge_scroll((1279.0, 400.0), 0.1));
        assert!((c.x - x - PAN_SPEED * 0.1).abs() < 1e-3);
        // At zoom 4 the same time pans a quarter of the cells.
        c.zoom_to(4.0, (640.0, 400.0));
        let x = c.x;
        c.edge_scroll((0.0, 400.0), 0.1);
        assert!((x - c.x - PAN_SPEED * 0.1 / 4.0).abs() < 1e-3);
    }

    #[test]
    fn the_cursor_off_the_world_is_none() {
        let c = Camera::new((256, 160), (1024, 700));
        assert!(c.world_at((10.0, 5.0)).is_none(), "the band above the dish");
        assert!(c.world_at((10.0, 350.0)).is_some());
    }
}
