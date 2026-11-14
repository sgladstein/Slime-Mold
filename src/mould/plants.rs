//! **Plants** (2026-11-14; owner 17:02Z: "fully new from the old grass").
//! A per-cell field, 0 to the cell's capacity, with a life of its own: it
//! grows where it already stands (logistic, toward the capacity) and spreads
//! into its four neighbours, so bare ground only greens from an edge and a
//! grazed or burnt patch closes up as a front moving in from its rim. The
//! old grass (`BigParams::graze_*`) is a uniform carpet that regrows in
//! place everywhere at once; this one has fronts, patches and holes, and its
//! capacity and pace come from the ground (soil kind, wet) the map draws.
//!
//! The rule, once a tick, for a cell of plant kind `k` with density `p`
//! and capacity `c` (its soil's capacity x the kind's, x the kind's `dry`
//! share off wet ground):
//!
//! ```text
//! room = 1 - p / c
//! p += (grow x p + spread x mean of the four neighbours) x room x pace
//! ```
//!
//! `pace` is the kind's own, x `plant_wet` on wet ground. A neighbour off
//! the grid counts 0. A bare cell that greens takes the kind of its densest
//! neighbour (lowest kind on a tie), so meadow and jungle meet at an edge
//! that moves; only bare cells change kind, and a bare cell is never read
//! for one, so the order cells are done in does not matter. Only `+`, `*`
//! and `/` (no platform maths), so a run is the same on Windows and Linux,
//! and the GPU pass does the same sums (FMA aside).
//!
//! A cell that is not open ground (rock, rim, a built wall) or holds a
//! crystal has capacity 0 and loses its plants at once; a scorched cell
//! (`BigDish::scorch`, a crater or a burn) loses `plant_scorch` of them a
//! tick and does not grow, so a crater shows as a bare scar that greens back
//! from its edge once the scorch is gone.
//!
//! Where the start comes from: a map's B channel (`Map::plants`, agreed
//! with levelgen 2026-11-14: kind in the top 2 bits, density 1-63 in the
//! low 6, 0 no plant), capped by the cell's capacity; a noise world with no
//! map starts at `plant_fill` of kind 0 everywhere.

/// Soil kinds the capacity table covers (the ground word's high nibble).
pub const SOIL_KINDS: usize = 16;
/// Plant kinds (B's top 2 bits).
pub const KINDS: usize = 4;
pub const MEADOW: u8 = 0;
pub const JUNGLE: u8 = 1;
pub const REEDS: u8 = 2;
pub const SCRUB: u8 = 3;

/// The default capacity a soil kind (index = `BigDish::soil`): plain 1,
/// sand thin, clay a little less than plain, barren nothing, deep water
/// (4) and bridges (5, both the ground thread's, 2026-11-14) nothing; kinds
/// added later grow as plain until they say.
pub const SOIL_CAP: [f32; SOIL_KINDS] = [1.0, 0.35, 0.75, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

/// Each plant kind's numbers: capacity, pace, and the share of capacity
/// it keeps off wet ground.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kind {
    pub cap: f32,
    pub pace: f32,
    pub dry: f32,
}

/// Meadow grass low and quick; jungle brush full and slow; reeds full but
/// a third as thick off the wet; scrub sparse and slow.
pub const KIND: [Kind; KINDS] = [
    Kind { cap: 0.8, pace: 1.2, dry: 1.0 },
    Kind { cap: 1.0, pace: 0.6, dry: 1.0 },
    Kind { cap: 1.0, pace: 1.0, dry: 0.3 },
    Kind { cap: 0.5, pace: 0.5, dry: 1.0 },
];

/// What the rule reads of the world, a fine cell.
pub struct Ground<'a> {
    pub w: usize,
    pub h: usize,
    /// The soil's capacity a cell (`soil_capacity`; 0 where nothing grows).
    pub cap: &'a [f32],
    /// Open ground (`BigDish::agar`): false under rock, rim and walls.
    pub agar: &'a [bool],
    /// Crystal a cell (`BigDish::pile_of`), empty with crystals off.
    pub pile_of: &'a [u16],
    /// Wet ground, empty without terrain.
    pub wet: &'a [bool],
    /// Scorch steps left (`BigDish::scorch`), empty before the first.
    pub scorch: &'a [u16],
}

/// The rule's numbers (`BigParams::plant_*`).
#[derive(Clone, Copy, Debug)]
pub struct Rates {
    pub grow: f32,
    pub spread: f32,
    pub wet: f32,
    pub scorch: f32,
    pub kinds: [Kind; KINDS],
}

/// Cell `i`'s capacity for kind `k`.
pub fn cap_of(g: &Ground, r: &Rates, i: usize, k: u8) -> f32 {
    let kd = r.kinds[k as usize & (KINDS - 1)];
    let wet = !g.wet.is_empty() && g.wet[i];
    g.cap[i] * kd.cap * if wet { 1.0 } else { kd.dry }
}

/// One tick of the rule, `from` to `to` (both `w` x `h`); `kind` is
/// updated in place (only bare cells change).
pub fn step(g: &Ground, r: Rates, from: &[f32], to: &mut [f32], kind: &mut [u8]) {
    let (w, h) = (g.w, g.h);
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if g.cap[i] <= 0.0 || !g.agar[i] || (!g.pile_of.is_empty() && g.pile_of[i] != 0) {
                to[i] = 0.0;
                continue;
            }
            let p = from[i];
            if !g.scorch.is_empty() && g.scorch[i] > 0 {
                to[i] = p * (1.0 - r.scorch);
                continue;
            }
            let mut n = 0.0;
            let (mut best, mut bk) = (0.0, kind[i]);
            for (ok, j) in [(x > 0, i.wrapping_sub(1)), (x + 1 < w, i + 1), (y > 0, i.wrapping_sub(w)), (y + 1 < h, i + w)] {
                if ok {
                    let v = from[j];
                    n += v;
                    if v > best || (v == best && v > 0.0 && kind[j] < bk) {
                        (best, bk) = (v, kind[j]);
                    }
                }
            }
            if p <= 0.0 {
                if n <= 0.0 {
                    to[i] = 0.0;
                    continue;
                }
                kind[i] = bk;
            }
            let k = kind[i];
            let c = cap_of(g, &r, i, k);
            if c <= 0.0 {
                to[i] = 0.0;
                continue;
            }
            let wet = !g.wet.is_empty() && g.wet[i];
            let pace = r.kinds[k as usize & (KINDS - 1)].pace * if wet { r.wet } else { 1.0 };
            let room = 1.0 - p / c;
            to[i] = (p + (r.grow * p + r.spread * (n * 0.25)) * room * pace).clamp(0.0, c);
        }
    }
}

/// Each cell's soil capacity from its soil kind (empty soil is all plain)
/// and the table `cap`; 0 where the cell is not open ground.
pub fn soil_capacity(agar: &[bool], soil: &[u8], cap: &[f32; SOIL_KINDS]) -> Vec<f32> {
    agar.iter()
        .enumerate()
        .map(|(i, &a)| if !a { 0.0 } else { cap[if soil.is_empty() { 0 } else { soil[i] as usize & (SOIL_KINDS - 1) }] })
        .collect()
}

/// A B byte's kind and start density (0-1).
pub fn unpack(b: u8) -> (u8, f32) {
    (b >> 6, (b & 63) as f32 / 63.0)
}

/// A kind and a density (0-1) as a B byte (0 for none).
pub fn pack(kind: u8, density: f32) -> u8 {
    let d = (density.clamp(0.0, 1.0) * 63.0 + 0.5) as u8;
    if d == 0 {
        0
    } else {
        (kind & 3) << 6 | d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const R: Rates = Rates { grow: 0.002, spread: 0.004, wet: 2.0, scorch: 0.05, kinds: [Kind { cap: 1.0, pace: 1.0, dry: 1.0 }; KINDS] };

    fn run(g: &Ground, mut p: Vec<f32>, ticks: usize) -> Vec<f32> {
        let mut q = vec![0.0; p.len()];
        let mut k = vec![0u8; p.len()];
        for _ in 0..ticks {
            step(g, R, &p, &mut q, &mut k);
            std::mem::swap(&mut p, &mut q);
        }
        p
    }

    /// Bare ground stays bare; plants spread from an edge as a front, never
    /// past the capacity, and nothing grows where the capacity is 0.
    #[test]
    fn plants_spread_from_an_edge_and_stop_at_capacity() {
        let (w, h) = (64, 8);
        let mut cap = vec![1.0; w * h];
        for y in 0..h {
            cap[y * w + 40] = 0.0; // a line of rock at x 40
        }
        let agar = vec![true; w * h];
        let g = Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &[], scorch: &[] };
        assert!(run(&g, vec![0.0; w * h], 500).iter().all(|&v| v == 0.0), "bare stays bare");
        let mut p = vec![0.0; w * h];
        for y in 0..h {
            p[y * w] = 1.0; // a green column at x 0
        }
        let p = run(&g, p, 120 * 30);
        let row = |x: usize| p[4 * w + x];
        assert!(p.iter().all(|&v| v <= 1.0));
        assert!(row(1) > 0.9 && row(10) > row(20) && row(20) > row(30), "a front: {} {} {} {}", row(1), row(10), row(20), row(30));
        assert!(row(40) == 0.0 && row(45) == 0.0, "the rock stops it");
    }

    /// Wet ground greens faster; scorch strips plants and holds them off.
    #[test]
    fn wet_is_faster_and_scorch_strips() {
        let (w, h) = (32, 1);
        let cap = vec![1.0; w];
        let agar = vec![true; w];
        let wet = vec![true; w];
        let mut start = vec![0.0; w];
        start[0] = 1.0;
        let dry = run(&Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &[], scorch: &[] }, start.clone(), 1200);
        let wetp = run(&Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &wet, scorch: &[] }, start, 1200);
        let sum = |v: &[f32]| v.iter().sum::<f32>();
        assert!(sum(&wetp) > sum(&dry) * 1.3, "wet {} dry {}", sum(&wetp), sum(&dry));
        let scorch = vec![100u16; w];
        let burnt = run(&Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &[], scorch: &scorch }, vec![1.0; w], 60);
        assert!(burnt.iter().all(|&v| v < 0.05), "scorch strips");
    }

    /// A bare cell takes its densest neighbour's kind, so two kinds meet at
    /// an edge; reeds stay thin off the wet; B bytes round-trip.
    #[test]
    fn kinds_spread_and_reeds_want_water() {
        let (w, h) = (40, 1);
        let cap = vec![1.0; w];
        let agar = vec![true; w];
        let mut wet = vec![false; w];
        wet[20..].fill(true);
        let r = Rates { kinds: KIND, ..R };
        let g = Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &wet, scorch: &[] };
        let (mut p, mut q) = (vec![0.0; w], vec![0.0; w]);
        let mut k = vec![0u8; w];
        (p[0], k[0], p[39], k[39]) = (0.8, MEADOW, 1.0, REEDS);
        for _ in 0..120 * 120 {
            step(&g, r, &p, &mut q, &mut k);
            std::mem::swap(&mut p, &mut q);
        }
        assert_eq!((k[2], k[37]), (MEADOW, REEDS));
        assert!(k.iter().all(|&v| v == MEADOW || v == REEDS));
        assert!(p.iter().all(|&v| v > 0.0), "filled: {p:?}");
        // Wherever reeds stand on dry ground they are at most 0.3 thick.
        assert!((0..20).all(|i| k[i] != REEDS || p[i] <= 0.3 + 1e-6));
        for b in 0..=255u8 {
            let (kd, d) = unpack(b);
            assert_eq!(pack(kd, d), if b & 63 == 0 { 0 } else { b });
        }
    }
}
