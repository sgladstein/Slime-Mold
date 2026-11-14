//! **The look's ground field**: what the ground is in each cell, for the
//! view's shading (`view.wgsl`, *The look*). One `u32` a fine cell: byte 0
//! the kind, byte 1 a value 0-255 where the kind is graded (a wall's share
//! of its health, a crystal's share of its charge). The view takes the
//! cell's height, colour and glow from these, so a wall sinks and dims as
//! the mould wears it and a crystal shrinks and dims as it drains: the
//! picture is the data (owner, 2026-11-09/10), not art laid on top.
//!
//! Byte 0's high nibble is the soil kind (`Fields::soil`, 2026-11-11, the
//! maps thread): sand, clay and barren ground each change the mould, so
//! each is its own ground in the picture. The kind is byte 0's low nibble.
//!
//! Byte 2 is depth: how far a crystal cell is inside its crystal, 0 at
//! the rim to 255 at the deepest cell (the shape, so a crystal stands as
//! a cone its charge tall; `ground` works it out).
//!
//! Byte 3 is the land's elevation, 0-255, from the map (`Fields::elev`;
//! 0 where a map has none): the view raises the ground, and everything on
//! it, by it and colours bare ground from damp low loam to dry high soil,
//! so the land reads as terrain lit like the mould. Added 2026-11-11 after
//! the owner saw bare ground as "just black background" (19:29Z): before,
//! bare agar took the engine's near-black frame colour, flat.
//! Maps leave it 0 until the simulation uses height (no drawn-only
//! relief; the Larder thread, 19:33Z); bare ground is then mid soil.
//!
//! The game fills it from its own fields every frame something changes
//! (the Larder world's `BigDish::{agar, rock, wet, wall, wall_health,
//! charge, pile_of}`, PR #64); the sandbox fills it from the dish's agar
//! and piles.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ground {
    Agar = 0,
    Wet = 1,
    Rock = 2,
    Wall = 3,
    Rubble = 4,
    Crystal = 5,
}

/// One cell's code: `value` 0-1 (clamped).
pub fn code(kind: Ground, value: f32) -> u32 {
    kind as u32 | ((value.clamp(0.0, 1.0) * 255.0).round() as u32) << 8
}

/// The world's own fields, a value a fine cell (index y * w + x); an empty
/// slice is a feature that is off.
#[derive(Default)]
pub struct Fields<'a> {
    pub agar: &'a [bool],
    pub rock: &'a [bool],
    pub wet: &'a [bool],
    pub wall: &'a [bool],
    /// Each wall cell's share of its full health (empty: walls can't break).
    pub wall_share: &'a [f32],
    /// Which crystal a cell belongs to (index + 1, 0 none).
    pub crystal: &'a [u16],
    /// Each crystal cell's share of its full charge (empty: crystals full).
    pub charge_share: &'a [f32],
    /// The land's elevation a cell, 0-255 (empty: flat).
    pub elev: &'a [u8],
    /// The soil kind a cell (`BigDish::soil`: 1 sand, 2 clay, 3 barren; empty
    /// or 0 plain), in byte 0's high nibble: the view draws each kind as its
    /// own ground under the mould.
    pub soil: &'a [u8],
    /// Each crystal cell's depth (`crystal_depth` of `crystal`), worked
    /// out once by a caller whose crystals keep their cells; empty: worked
    /// out here every call. It is most of `ground`'s cost (the Larder
    /// world, 2026-11-12: 3.9-6.1 ms a rebuild on the cloud CPU).
    pub depth: &'a [u8],
}

/// The ground field for a `w`-wide world of `cells` cells. Off the agar
/// and not a wall is rock (the rim of a dish is rock too).
pub fn ground(w: usize, cells: usize, f: &Fields) -> Vec<u32> {
    let at = |v: &[bool], i: usize| v.get(i).copied().unwrap_or(false);
    let depth = if f.depth.len() == cells { f.depth.to_vec() } else { crystal_depth(w, cells, f.crystal) };
    let mut g: Vec<u32> = (0..cells)
        .map(|i| {
            if at(f.wall, i) {
                code(Ground::Wall, f.wall_share.get(i).copied().unwrap_or(1.0))
            } else if f.crystal.get(i).copied().unwrap_or(0) > 0 {
                code(Ground::Crystal, f.charge_share.get(i).copied().unwrap_or(1.0))
            } else if at(f.rock, i) || (!f.agar.is_empty() && !at(f.agar, i)) {
                code(Ground::Rock, 1.0)
            } else if at(f.wet, i) {
                code(Ground::Wet, 1.0)
            } else {
                code(Ground::Agar, 0.0)
            }
        })
        .collect();
    for (i, (c, d)) in g.iter_mut().zip(depth).enumerate() {
        *c |= (d as u32) << 16 | (f.elev.get(i).copied().unwrap_or(0) as u32) << 24 | ((f.soil.get(i).copied().unwrap_or(0) & 7) as u32) << 4;
    }
    g
}

/// Byte 0's top bit: no hard terrain (rock, wall, rubble, or the world's
/// edge) within `FAR_REACH` cells (`mark_far`). The view draws rock, walls
/// and rubble at the screen pixel from a smooth field of the 4 x 4 cells
/// round each point (`view.wgsl`, *Hard terrain*), which doubled the look's
/// cost on llvmpipe (GPU tail 37 to 79 ms a frame at 1280 x 800,
/// 2026-11-14) when every pixel worked it out; with the bit, a pixel far
/// from terrain reads one cell and skips it. A caller that never marks
/// still draws right, only slower. The soil kind keeps bits 4-6.
pub const FAR: u32 = 1 << 7;
/// The terrain field's reach: its spline (2 cells), the edge's box (0.75)
/// and the shadow's offset (1.2), rounded up.
pub const FAR_REACH: usize = 4;

/// Sets `FAR` on each cell with no rock, wall or rubble within `FAR_REACH`
/// cells (a square), and clears it elsewhere; the world's edge counts as
/// rock. Call it last, after any kind is changed by hand (Larder World's
/// rubble). Two running-count passes, so it costs the cells, not the reach.
pub fn mark_far(w: usize, codes: &mut [u32]) {
    if w == 0 {
        return;
    }
    let h = codes.len() / w;
    let r = FAR_REACH;
    let hard = |c: u32| matches!(c & 15, 2..=4);
    // Rows: how many hard cells within r along x (off the row counts).
    let mut rows = vec![0u32; w * h];
    for y in 0..h {
        let row = &codes[y * w..(y + 1) * w];
        let at = |x: isize| -> u32 { if x < 0 || x >= w as isize { 1 } else { hard(row[x as usize]) as u32 } };
        let mut n: u32 = (-(r as isize)..=r as isize).map(at).sum();
        for x in 0..w {
            rows[y * w + x] = n;
            n = n + at(x as isize + r as isize + 1) - at(x as isize - r as isize);
        }
    }
    // Columns of the row counts.
    for x in 0..w {
        let at = |y: isize| -> u32 { if y < 0 || y >= h as isize { 1 } else { rows[y as usize * w + x] } };
        let mut n: u32 = (-(r as isize)..=r as isize).map(at).sum();
        for y in 0..h {
            let c = &mut codes[y * w + x];
            *c = if n == 0 { *c | FAR } else { *c & !FAR };
            n = n + at(y as isize + r as isize + 1) - at(y as isize - r as isize);
        }
    }
}

/// Where a shot along a line ends (`first_hit`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shot {
    /// It met mould first, at this point (fine cells): cut there.
    Mould(f32, f32),
    /// It met something that stops it (rock) first, at this point.
    Stopped(f32, f32),
    /// Nothing on the way: it flies to the line's end.
    Clear,
}

/// A shot from `from` to `to` (fine cells) on a `w` x `h` world, marched
/// in half-cell steps from the muzzle: the first cell where `mould(i)` is
/// true ends it (the owner, 2026-11-11 16:48Z: "Gun fire should hit the
/// first slime in its path"), and so does the first where `stops(i)` is
/// (rock; the Larder thread lets shots pass over player walls for now). A
/// game passes its own tests, such as the engine's look thickness over a
/// threshold; the cut itself is `Engine::set_hits` at the point.
pub fn first_hit(w: usize, h: usize, from: (f32, f32), to: (f32, f32), mould: impl Fn(usize) -> bool, stops: impl Fn(usize) -> bool) -> Shot {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let n = ((dx.hypot(dy)) * 2.0).ceil().max(1.0) as usize;
    for k in 0..=n {
        let t = k as f32 / n as f32;
        let (x, y) = (from.0 + dx * t, from.1 + dy * t);
        if x < 0.0 || y < 0.0 || x >= w as f32 || y >= h as f32 {
            return Shot::Stopped(x.clamp(0.0, w as f32), y.clamp(0.0, h as f32));
        }
        let i = y as usize * w + x as usize;
        if stops(i) {
            return Shot::Stopped(x, y);
        }
        if mould(i) {
            return Shot::Mould(x, y);
        }
    }
    Shot::Clear
}

/// The dish's piles as crystals (pile index + 1 a fine cell, in a
/// pointy-top hexagon of circumradius `pile_r` round each pile, on the
/// agar): the sandbox's crystals until the Larder world's own `pile_of`
/// reaches the GPU. The owner, 2026-11-11 16:48Z: "The crystals should have
/// hexagonal outline, not circle"; the Larder thread makes the same hexagon
/// the crystal's footprint in the simulation (PR #64). In the sandbox the
/// food the engine lays is still the pile's disc round it.
pub fn dish_crystals(d: &crate::mould::big::BigDish) -> Vec<u16> {
    let (w, h) = (d.w, d.h);
    let sc = d.p.scale as f32;
    let r = d.p.pile_r * sc;
    let mut crystal = vec![0u16; w * h];
    for (k, &(px, py)) in d.piles.iter().enumerate() {
        let (px, py) = (px * sc, py * sc);
        for y in (py - r).max(0.0) as usize..((py + r).max(0.0) as usize).min(h) {
            for x in (px - r).max(0.0) as usize..((px + r).max(0.0) as usize).min(w) {
                let (dx, dy) = ((x as f32 + 0.5 - px).abs(), (y as f32 + 0.5 - py).abs());
                if dx <= r * 3f32.sqrt() / 2.0 && dy <= r - dx / 3f32.sqrt() && d.agar[y * w + x] {
                    crystal[y * w + x] = k as u16 + 1;
                }
            }
        }
    }
    crystal
}

/// A ground buffer on the GPU from codes (`ground`), for `View::set_look`.
pub fn ground_buffer(device: &pixels::wgpu::Device, queue: &pixels::wgpu::Queue, codes: &[u32]) -> pixels::wgpu::Buffer {
    use pixels::wgpu;
    let b = device.create_buffer(&wgpu::BufferDescriptor { label: Some("ground"), size: (codes.len() * 4).max(16) as u64, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
    queue.write_buffer(&b, 0, &crate::gpu::u32_bytes(codes));
    b
}

/// Each crystal cell's depth inside its crystal (the distance to the
/// nearest cell that is not of the same crystal), scaled so each crystal's
/// deepest cell is 255; 0 outside crystals. Exact distances to each
/// crystal's rim cells, so a round crystal stands as a round cone.
pub fn crystal_depth(w: usize, cells: usize, crystal: &[u16]) -> Vec<u8> {
    let mut out = vec![0u8; cells];
    if crystal.is_empty() || w == 0 {
        return out;
    }
    let h = cells / w;
    let piles = crystal.iter().copied().max().unwrap_or(0) as usize;
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); piles + 1];
    let mut rims: Vec<Vec<(f32, f32)>> = vec![Vec::new(); piles + 1];
    for i in 0..cells.min(crystal.len()) {
        let k = crystal[i] as usize;
        if k == 0 {
            continue;
        }
        members[k].push(i);
        let (x, y) = (i % w, i / w);
        let same = |xx: usize, yy: usize| crystal.get(yy * w + xx).copied() == Some(k as u16);
        if x == 0 || y == 0 || x + 1 == w || y + 1 == h || !same(x - 1, y) || !same(x + 1, y) || !same(x, y - 1) || !same(x, y + 1) {
            rims[k].push((x as f32, y as f32));
        }
    }
    for k in 1..=piles {
        let d: Vec<f32> = members[k]
            .iter()
            .map(|&i| {
                let (x, y) = ((i % w) as f32, (i / w) as f32);
                rims[k].iter().map(|&(rx, ry)| (rx - x).hypot(ry - y)).fold(f32::MAX, f32::min) + 1.0
            })
            .collect();
        let top = d.iter().cloned().fold(1.0f32, f32::max);
        for (&i, v) in members[k].iter().zip(d) {
            out[i] = (v / top * 255.0).round() as u8;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soil_sits_in_the_high_nibble_of_the_kind_byte() {
        let soil = [0u8, 1, 2, 3];
        let wet = [false, false, false, true];
        let g = ground(4, 4, &Fields { soil: &soil, wet: &wet, ..Default::default() });
        assert_eq!(g, vec![0, 1 << 4, 2 << 4, 1 | 255 << 8 | 3 << 4]);
        assert!(g.iter().all(|c| c & 15 <= Ground::Crystal as u32), "the kind stays in the low nibble");
    }

    #[test]
    fn kinds_and_values_pack_into_the_low_bytes() {
        assert_eq!(code(Ground::Crystal, 1.0), 5 | 255 << 8);
        assert_eq!(code(Ground::Wall, 0.5), 3 | 128 << 8);
        let agar = [true, false, true, true, true];
        let wall = [false, true, false, false, false];
        let share = [1.0, 0.25, 1.0, 1.0, 1.0];
        let crystal = [0u16, 0, 1, 0, 0];
        let wet = [false, false, false, false, true];
        let g = ground(5, 5, &Fields { agar: &agar, wall: &wall, wall_share: &share, crystal: &crystal, wet: &wet, ..Default::default() });
        assert_eq!(g, vec![0, 3 | 64 << 8, 5 | 255 << 8 | 255 << 16, 0, 1 | 255 << 8], "a lone crystal cell is its own deepest");
    }

    #[test]
    fn a_shot_stops_at_the_first_mould_or_rock_on_its_line() {
        let (w, h) = (20, 3);
        // Mould at x 8 and 14, rock at x 11, on row 1.
        let mould = |i: usize| i == 20 + 8 || i == 20 + 14;
        let rock = |i: usize| i == 20 + 11;
        match first_hit(w, h, (1.5, 1.5), (18.5, 1.5), mould, rock) {
            Shot::Mould(x, _) => assert!((8.0..9.0).contains(&x), "first mould, not the thicker one behind: {x}"),
            s => panic!("{s:?}"),
        }
        match first_hit(w, h, (18.5, 1.5), (9.5, 1.5), |i| i == 20 + 8, rock) {
            Shot::Stopped(x, _) => assert!((11.0..12.0).contains(&x)),
            s => panic!("{s:?}"),
        }
        assert_eq!(first_hit(w, h, (1.5, 0.5), (18.5, 0.5), mould, rock), Shot::Clear);
    }

    #[test]
    fn far_marks_only_cells_with_no_terrain_in_reach() {
        let (w, h) = (24, 20);
        let mut codes = vec![0u32; w * h];
        codes[10 * w + 12] = code(Ground::Wall, 1.0);
        mark_far(w, &mut codes);
        let far = |c: &[u32], x: usize, y: usize| c[y * w + x] & FAR != 0;
        // The world's edge counts as rock; the wall at (12, 10) reaches 4.
        assert!(!far(&codes, 3, 10) && !far(&codes, 12, 3) && far(&codes, 4, 10) && far(&codes, 12, 4) && far(&codes, 7, 10));
        assert!(!far(&codes, 8, 10) && !far(&codes, 16, 14) && far(&codes, 17, 10) && far(&codes, 12, 15));
        assert_eq!(codes[10 * w + 12] & 15, Ground::Wall as u32, "the kind is kept");
        // Marking twice changes nothing; a wall gone clears its reach.
        let once = codes.clone();
        mark_far(w, &mut codes);
        assert_eq!(codes, once);
        codes[10 * w + 12] = code(Ground::Agar, 0.0);
        mark_far(w, &mut codes);
        assert!(far(&codes, 12, 10) && far(&codes, 8, 10));
    }

    #[test]
    fn a_crystal_is_deepest_in_its_middle() {
        let w = 9;
        let crystal: Vec<u16> = (0..81).map(|i| ((i % w) as i32 - 4).pow(2) + ((i / w) as i32 - 4).pow(2) <= 9).map(|c| c as u16).collect();
        let g = ground(w, 81, &Fields { crystal: &crystal, ..Default::default() });
        let depth = |x: usize, y: usize| (g[y * w + x] >> 16) & 255;
        assert_eq!(depth(4, 4), 255);
        assert!(depth(4, 2) < depth(4, 3) && depth(4, 3) < 255);
        assert_eq!(depth(0, 0), 0);
    }

    #[test]
    fn a_given_depth_draws_the_same_ground() {
        let w = 9;
        let crystal: Vec<u16> = (0..81usize).map(|i| if (i % 9).abs_diff(4) + (i / 9).abs_diff(4) < 4 { 1 } else { 0 }).collect();
        let depth = crystal_depth(w, 81, &crystal);
        let a = ground(w, 81, &Fields { crystal: &crystal, ..Default::default() });
        let b = ground(w, 81, &Fields { crystal: &crystal, depth: &depth, ..Default::default() });
        assert_eq!(a, b);
    }
}
