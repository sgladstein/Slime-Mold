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
pub const SOIL_CAP: [f32; SOIL_KINDS] = [
    1.0, 0.35, 0.75, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

/// Each plant kind's numbers: capacity, pace, and the share of capacity
/// it keeps off wet ground.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kind {
    pub cap: f32,
    pub pace: f32,
    pub dry: f32,
    /// How readily it catches fire (`fire_step`): 0 never.
    pub burn: f32,
}

/// Meadow grass low, quick and burns; jungle brush full and slow, burns
/// less (green); reeds full but a third as thick off the wet, and never
/// burn; scrub sparse and slow, and burns best.
pub const KIND: [Kind; KINDS] = [
    Kind {
        cap: 0.8,
        pace: 1.2,
        dry: 1.0,
        burn: 1.0,
    },
    Kind {
        cap: 1.0,
        pace: 0.6,
        dry: 1.0,
        burn: 0.6,
    },
    Kind {
        cap: 1.0,
        pace: 1.0,
        dry: 0.3,
        burn: 0.0,
    },
    Kind {
        cap: 0.5,
        pace: 0.5,
        dry: 1.0,
        burn: 1.6,
    },
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
            for (ok, j) in [
                (x > 0, i.wrapping_sub(1)),
                (x + 1 < w, i + 1),
                (y > 0, i.wrapping_sub(w)),
                (y + 1 < h, i + w),
            ] {
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
        .map(|(i, &a)| {
            if !a {
                0.0
            } else {
                cap[if soil.is_empty() {
                    0
                } else {
                    soil[i] as usize & (SOIL_KINDS - 1)
                }]
            }
        })
        .collect()
}

/// Fire's numbers (`BigParams::fire_*`), a plant step's.
#[derive(Clone, Copy, Debug)]
pub struct Fire {
    /// Heat a burning cell gains a step while it has fuel, up to 1.
    pub heat: f32,
    /// Plants a burning cell eats a step, x its heat.
    pub eat: f32,
    /// A cell catches by chance (owner 11-14 19:18Z: "Fire should spread
    /// 5x slower. It shouldn't just be a wavefront"): a step's chance is
    /// `spread` x the mean of its neighbours' heat x its plants x its
    /// kind's `burn` x its dryness squared (1 - `moisture`); it starts at
    /// `heat`. Before, it caught once that product (without moisture)
    /// reached 0.08, so every front was an even ribbon 9 cells deep moving
    /// 5 fine cells a second.
    pub spread: f32,
    /// A burning cell goes out (heat 0, its plants left for a later fire)
    /// with chance `out` x moisture² a step: damp ground snuffs fires and
    /// leaves unburnt patches (the owner: "Sometimes it should burn out in
    /// areas").
    pub out: f32,
    /// Damp burns slower and longer: a cell eats x (1 - `smoulder` x
    /// moisture) ("It should stay burning in some areas").
    pub smoulder: f32,
    /// ... and cooler: its heat tops out at 1 - `cool` x moisture, so a
    /// damp fire glows dim, spreads less and (under `fire_kill`) kills less.
    pub cool: f32,
    /// A burning cell's heat a step is its rise x (1 - `flicker` x its
    /// chance), so flames flicker cell by cell and step by step (owner
    /// 11-14 19:26Z: "The fire should also look better"); the picture is
    /// the heat field, so the flicker is the fire's own.
    pub flicker: f32,
    /// Out of fuel, a cell's heat is kept x `ember` a step: a glowing tail
    /// of embers behind the front. Was 0.5 (gone in a few steps).
    pub ember: f32,
    /// Heat follows the fuel left: a cell tops out at its plants /
    /// `full` (at most 1), so a fresh front burns hottest and the inside
    /// dims to embers as it eats (graphics lead 11-14 20:10Z: the band was
    /// the same brightness through its depth).
    pub full: f32,
    /// Below this many plants a burning cell dies down (heat halves a step)
    /// and a cell can't catch.
    pub fuel: f32,
    /// Trail at or over this is living slime, wet: it doesn't catch, and a
    /// fire on it goes out. The owner (11-14 17:29Z): fire must not make
    /// it easy to wipe out the whole slime, so the body is its own
    /// firebreak and fire takes only its thin edge and stragglers.
    pub damp: f32,
    /// Char (burnt ground, the graphics lead's ask 11-14 17:55Z: without
    /// it burnt ground reads as bare ground): a cell's char is kept x
    /// `char_keep` a step, gains the plants burnt x `char_gain`, up to 1,
    /// and loses plants x `char_clear`, so regrowth clears it. From what
    /// burnt, not the heat (graphics lead's review 18:20Z), so a burnt
    /// jungle leaves darker ground than a meadow; the first cut gained
    /// heat x 0.2 and every burnt cell went the same flat black.
    pub char_gain: f32,
    pub char_keep: f32,
    pub char_clear: f32,
}

/// **Fire** (owner 11-14 17:29Z: "I like this but need to think hard about
/// how hard fire hurts slime"): one plant step of the heat field, `from` to
/// `to`, eating `plant` where it burns. Wet ground and slime (`trail` at or
/// over `damp`) never catch and put a fire out. `moist` is each cell's
/// moisture (`moisture`, 0-1), and `tick` the fire steps run, which keys
/// each cell's chance (`roll`). Only `+`, `*`, `/` and integer hashing, so
/// the GPU matches. Returns cells burning after the step.
///
/// `crust` is the ground the slime has lived on (`BigDish::crust`, empty
/// for none) and the plants a unit of it is worth (`BigParams::crust_fuel`,
/// the simulation-use plan's C2): a cell's fuel is its plants plus that,
/// and a fire eats the plants first, then the crust.
#[allow(clippy::too_many_arguments)]
pub fn fire_step(
    g: &Ground,
    r: &Rates,
    f: &Fire,
    trail: &[f32],
    kind: &[u8],
    plant: &mut [f32],
    char: &mut [f32],
    from: &[f32],
    to: &mut [f32],
    moist: &[f32],
    tick: u32,
    (crust, cf): (&mut [f32], f32),
) -> usize {
    let (w, h) = (g.w, g.h);
    let crusts = cf > 0.0 && !crust.is_empty();
    let mut lit = 0;
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let wet = (!g.wet.is_empty() && g.wet[i]) || trail[i] >= f.damp || !g.agar[i];
            let heat = from[i];
            let pl = plant[i];
            let p = if crusts { pl + cf * crust[i] } else { pl };
            let m = moist[i];
            let dice = roll(i as u32, tick);
            let mut ate = 0.0;
            if wet {
                to[i] = 0.0;
            } else if heat > 0.0 {
                let slow = 1.0 - f.smoulder * m;
                let eat = (f.eat * heat * slow).min(p);
                if eat <= pl {
                    plant[i] = pl - eat;
                } else {
                    plant[i] = 0.0;
                    crust[i] = (crust[i] - (eat - pl) / cf).max(0.0);
                }
                ate = eat;
                let top = (1.0 - f.cool * m) * (p / f.full).min(1.0);
                to[i] = if p - eat < f.fuel {
                    heat * f.ember
                } else if dice < f.out * (m * m) {
                    0.0
                } else {
                    (heat + f.heat).min(top) * (1.0 - f.flicker * dice)
                };
                if to[i] < 0.01 {
                    to[i] = 0.0;
                }
            } else {
                let mut n = 0.0;
                for (ok, j) in [
                    (x > 0, i.wrapping_sub(1)),
                    (x + 1 < w, i + 1),
                    (y > 0, i.wrapping_sub(w)),
                    (y + 1 < h, i + w),
                ] {
                    if ok {
                        n += from[j];
                    }
                }
                let burn = r.kinds[kind[i] as usize & (KINDS - 1)].burn;
                let dry = 1.0 - m;
                to[i] = if p >= f.fuel && dice < f.spread * (n * 0.25) * p * burn * dry {
                    f.heat
                } else {
                    0.0
                };
            }
            if to[i] > 0.0 {
                lit += 1;
            }
            // Regrowth clears char only once the fire is out (cleared while
            // it burned, a jungle's char came out at 0.2, a third of its
            // fuel), and only plants over the embers' last `fuel` count:
            // those cleared a burn in about 15 s, before `char_keep` faded it.
            let clear = if to[i] > 0.0 { 0.0 } else { (plant[i] - f.fuel).max(0.0) * f.char_clear };
            char[i] = ((char[i] * f.char_keep + ate * f.char_gain).min(1.0) - clear).max(0.0);
        }
    }
    lit
}

/// A cell's chance this fire step, 0-1: an integer hash of the cell and
/// the step, as engine.wgsl's `froll`.
pub fn roll(i: u32, tick: u32) -> f32 {
    let mut h = i.wrapping_mul(0x9E37_79B1) ^ tick.wrapping_mul(0x85EB_CA77);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^= h >> 15;
    (h >> 8) as f32 * (1.0 / 16_777_216.0)
}

/// **Moisture** (owner 11-14 19:18Z: "a heterogenous moisture field that
/// affects how fast it spreads, how long it burns, and the chance of it
/// going out"; 19:26Z: "should match where there are rivers/lakes, and
/// change by biome"), 0.05-1 a cell, fixed for a map: the map's `base`
/// (`Items::moisture`, by biome from levelgen) plus `spread` x two value
/// noises (40 and 12 cells) about it, wetter within 8 cells of water
/// (`water`: wet ground and deep water), jungle +0.15, scrub -0.15, reeds
/// and water 1. Fire reads it (`fire_step`); nothing else yet.
pub fn moisture(w: usize, h: usize, water: &[bool], kind: &[u8], seed: u32, base: f32, spread: f32) -> Vec<f32> {
    // The distance to water, in cells (chamfer 1-1, two passes), to 8.
    let mut d: Vec<u8> = (0..w * h).map(|i| if !water.is_empty() && water[i] { 0 } else { 8 }).collect();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if x > 0 {
                d[i] = d[i].min(d[i - 1] + 1);
            }
            if y > 0 {
                d[i] = d[i].min(d[i - w] + 1);
            }
        }
    }
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let i = y * w + x;
            if x + 1 < w {
                d[i] = d[i].min(d[i + 1] + 1);
            }
            if y + 1 < h {
                d[i] = d[i].min(d[i + w] + 1);
            }
        }
    }
    (0..w * h)
        .map(|i| {
            let (x, y) = (i % w, i / w);
            if d[i] == 0 || kind[i] == REEDS {
                return 1.0;
            }
            let n = 0.5 * noise(x, y, 40, seed ^ 0x5EED_0001) + 0.3 * noise(x, y, 12, seed ^ 0x5EED_0002) + 0.2 * noise(x, y, 4, seed ^ 0x5EED_0003);
            // The noise's middle half spans about 0.4-0.6, so x 2 x 2: a
            // spread of 0.3 gives patches some 0.25 drier and wetter.
            let mut m = base + spread * 4.0 * (n - 0.5);
            m += 0.4 * (1.0 - d[i] as f32 / 8.0);
            if kind[i] == JUNGLE {
                m += 0.15;
            } else if kind[i] == SCRUB {
                m -= 0.15;
            }
            m.clamp(0.05, 1.0)
        })
        .collect()
}

fn hash(x: u32, y: u32, s: u32) -> u32 {
    let mut h = x.wrapping_mul(0x9E37_79B1) ^ y.wrapping_mul(0x85EB_CA77) ^ s.wrapping_mul(0xC2B2_AE3D);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h
}

fn unit(h: u32) -> f32 {
    (h >> 8) as f32 / 16_777_216.0
}

/// Value noise on a `cell`-cell lattice, 0-1 (integer hash, no platform
/// maths).
fn noise(x: usize, y: usize, cell: usize, s: u32) -> f32 {
    let (gx, gy) = ((x / cell) as u32, (y / cell) as u32);
    let (fx, fy) = ((x % cell) as f32 / cell as f32, (y % cell) as f32 / cell as f32);
    let v = |a: u32, b: u32| unit(hash(a, b, s));
    let top = v(gx, gy) * (1.0 - fx) + v(gx + 1, gy) * fx;
    let bot = v(gx, gy + 1) * (1.0 - fx) + v(gx + 1, gy + 1) * fx;
    top * (1.0 - fy) + bot * fy
}

/// **A stand-in start** for a map with no plants of its own, until
/// levelgen writes each biome's (11-14): reeds on wet ground; elsewhere a
/// value noise 96 map cells across picks jungle in its high part (over
/// 0.62), bare ground in its low part (under 0.3), scrub on sand and meadow
/// between; `share` of the cells start planted (1 full grown), full. A map
/// that has plants keeps them.
pub fn stand_in(m: &mut crate::mould::map::Map, seed: u32, share: f32) {
    use crate::mould::map::{G_ROCK, G_WET};
    if !m.plants.is_empty() {
        return;
    }
    m.plants = (0..m.ground.len())
        .map(|i| {
            let (x, y) = (i % m.w, i / m.w);
            let g = m.ground[i];
            if g & G_ROCK != 0 || unit(hash(x as u32, y as u32, seed ^ 77)) > share {
                return 0;
            }
            if g & G_WET != 0 {
                return pack(REEDS, 1.0);
            }
            let n = noise(x, y, 96, seed);
            if n > 0.62 {
                pack(JUNGLE, 1.0)
            } else if n < 0.3 {
                0
            } else if g >> 4 == 1 {
                pack(SCRUB, 1.0)
            } else {
                pack(MEADOW, 1.0)
            }
        })
        .collect();
}

/// `m` as the game plays it (`stand_in` on seed `seed`, full), for the
/// census and campaign tools: until 2026-11-15 only the window planted
/// designed maps, so the CPU tools played them bare (bug check 11-15).
pub fn planted(m: &crate::mould::map::Map, seed: u64) -> crate::mould::map::Map {
    let mut m = m.clone();
    stand_in(&mut m, seed as u32, 1.0);
    m
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

    const R: Rates = Rates {
        grow: 0.002,
        spread: 0.004,
        wet: 2.0,
        scorch: 0.05,
        kinds: [Kind {
            cap: 1.0,
            pace: 1.0,
            dry: 1.0,
            burn: 1.0,
        }; KINDS],
    };

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
        let g = Ground {
            w,
            h,
            cap: &cap,
            agar: &agar,
            pile_of: &[],
            wet: &[],
            scorch: &[],
        };
        assert!(
            run(&g, vec![0.0; w * h], 500).iter().all(|&v| v == 0.0),
            "bare stays bare"
        );
        let mut p = vec![0.0; w * h];
        for y in 0..h {
            p[y * w] = 1.0; // a green column at x 0
        }
        let p = run(&g, p, 120 * 30);
        let row = |x: usize| p[4 * w + x];
        assert!(p.iter().all(|&v| v <= 1.0));
        assert!(
            row(1) > 0.9 && row(10) > row(20) && row(20) > row(30),
            "a front: {} {} {} {}",
            row(1),
            row(10),
            row(20),
            row(30)
        );
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
        let dry = run(
            &Ground {
                w,
                h,
                cap: &cap,
                agar: &agar,
                pile_of: &[],
                wet: &[],
                scorch: &[],
            },
            start.clone(),
            1200,
        );
        let wetp = run(
            &Ground {
                w,
                h,
                cap: &cap,
                agar: &agar,
                pile_of: &[],
                wet: &wet,
                scorch: &[],
            },
            start,
            1200,
        );
        let sum = |v: &[f32]| v.iter().sum::<f32>();
        assert!(sum(&wetp) > sum(&dry) * 1.3, "wet {} dry {}", sum(&wetp), sum(&dry));
        let scorch = vec![100u16; w];
        let burnt = run(
            &Ground {
                w,
                h,
                cap: &cap,
                agar: &agar,
                pile_of: &[],
                wet: &[],
                scorch: &scorch,
            },
            vec![1.0; w],
            60,
        );
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
        let g = Ground {
            w,
            h,
            cap: &cap,
            agar: &agar,
            pile_of: &[],
            wet: &wet,
            scorch: &[],
        };
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

    const F: Fire = Fire {
        heat: 0.15,
        eat: 0.012,
        spread: 0.3,
        out: 0.05,
        smoulder: 0.75,
        cool: 0.4,
        flicker: 0.35,
        ember: 0.95,
        full: 0.8,
        fuel: 0.05,
        damp: 1.0,
        char_gain: 1.0,
        char_keep: 1.0,
        char_clear: 0.02,
    };

    /// Crust is fuel (`fire_step`'s `crust`, the simulation-use plan's C2):
    /// on bare ground a fire runs along a strip of crust, eats it and leaves
    /// char, stops where the crust ends, and with no crust worth goes out
    /// at once.
    #[test]
    fn fire_runs_along_crust_on_bare_ground() {
        let (w, h) = (60, 3);
        let cap = vec![1.0; w * h];
        let agar = vec![true; w * h];
        let g = Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &[], scorch: &[] };
        let r = Rates { kinds: KIND, ..R };
        let kind = vec![MEADOW; w * h];
        let trail = vec![0.0; w * h];
        let moist = vec![0.0; w * h];
        let run = |cf: f32| {
            let mut plant = vec![0.0; w * h];
            // Crust from x 0 to 39, none past it.
            let mut crust: Vec<f32> = (0..w * h).map(|i| if i % w < 40 { 0.8 } else { 0.0 }).collect();
            let (mut a, mut b) = (vec![0.0; w * h], vec![0.0; w * h]);
            let mut ch = vec![0.0; w * h];
            a[w] = 1.0;
            for tick in 1..=3000 {
                fire_step(&g, &r, &F, &trail, &kind, &mut plant, &mut ch, &a, &mut b, &moist, tick, (&mut crust, cf));
                std::mem::swap(&mut a, &mut b);
            }
            (crust, ch)
        };
        let (crust, ch) = run(1.0);
        assert!(crust[w + 1] < 0.1 && crust[w + 35] < 0.1, "burnt along the crust: {} {}", crust[w + 1], crust[w + 35]);
        assert!(ch[w + 35] > 0.0, "the crust left char");
        assert!(ch[w + 45] == 0.0, "no fire past the crust's end");
        let (crust, _) = run(0.0);
        assert!(crust[w + 5] == 0.8, "worth nothing, the crust doesn't burn");
    }

    /// A fire runs through dry plants and burns them out, stops at wet
    /// ground and at slime, and never crosses bare ground.
    #[test]
    fn fire_runs_through_plants_and_stops_at_slime_and_water() {
        let (w, h) = (60, 3);
        let cap = vec![1.0; w * h];
        let agar = vec![true; w * h];
        let mut wet = vec![false; w * h];
        let mut trail = vec![0.0; w * h];
        for y in 0..h {
            wet[y * w + 30] = true; // a wet line at x 30
            trail[y * w + 50] = 2.0; // slime at x 50
        }
        let g = Ground {
            w,
            h,
            cap: &cap,
            agar: &agar,
            pile_of: &[],
            wet: &wet,
            scorch: &[],
        };
        let r = Rates { kinds: KIND, ..R };
        let kind = vec![MEADOW; w * h];
        let mut plant = vec![0.8; w * h];
        for y in 0..h {
            plant[y * w + 20] = 0.0; // bare at x 20
        }
        let (mut a, mut b) = (vec![0.0; w * h], vec![0.0; w * h]);
        let mut ch = vec![0.0; w * h];
        a[w] = 1.0; // lit at (0, 1)
        // Dry ground (moisture 0): nothing goes out by chance.
        let moist = vec![0.0; w * h];
        let mut tick = 0;
        for _ in 0..3000 {
            tick += 1;
            fire_step(&g, &r, &F, &trail, &kind, &mut plant, &mut ch, &a, &mut b, &moist, tick, (&mut [], 0.0));
            std::mem::swap(&mut a, &mut b);
        }
        assert!(
            plant[w + 1] < 0.1 && plant[w + 15] < 0.1,
            "burnt to x 19: {} {}",
            plant[w + 1],
            plant[w + 15]
        );
        assert!(plant[w + 25] > 0.7, "bare ground at x 20 stops it: {}", plant[w + 25]);
        // x 10 burnt its 0.8 (char kept whole here, `char_keep` 1; the
        // embers' last plants clear it slowly after).
        assert!(ch[w + 10] > 0.5 && ch[w + 20] == 0.0 && ch[w + 25] == 0.0, "char where it burnt: {} {} {}", ch[w + 10], ch[w + 20], ch[w + 25]);
        // Lit beyond the bare line, it burns to the water and stops.
        a.fill(0.0);
        a[w + 21] = 1.0;
        for _ in 0..3000 {
            tick += 1;
            fire_step(&g, &r, &F, &trail, &kind, &mut plant, &mut ch, &a, &mut b, &moist, tick, (&mut [], 0.0));
            std::mem::swap(&mut a, &mut b);
        }
        assert!(
            plant[w + 25] < 0.1 && plant[w + 35] > 0.7,
            "wet at x 30 stops it: {} {}",
            plant[w + 25],
            plant[w + 35]
        );
        a.fill(0.0);
        a[w + 31] = 1.0;
        for _ in 0..3000 {
            tick += 1;
            fire_step(&g, &r, &F, &trail, &kind, &mut plant, &mut ch, &a, &mut b, &moist, tick, (&mut [], 0.0));
            std::mem::swap(&mut a, &mut b);
        }
        assert!(
            plant[w + 45] < 0.1 && plant[w + 55] > 0.7,
            "slime at x 50 stops it: {} {}",
            plant[w + 45],
            plant[w + 55]
        );
        assert!(a.iter().all(|&v| v == 0.0), "out");
        // Reeds never catch.
        let reeds = vec![REEDS; w * h];
        let mut plant = vec![0.8; w * h];
        a.fill(0.0);
        a[w] = 1.0;
        for _ in 0..300 {
            tick += 1;
            fire_step(&g, &r, &F, &trail, &reeds, &mut plant, &mut ch, &a, &mut b, &moist, tick, (&mut [], 0.0));
            std::mem::swap(&mut a, &mut b);
        }
        assert!(plant[w + 5] > 0.79, "reeds don't burn");
    }

    /// A meadow strip lit along its left edge at one moisture: where the
    /// front is (cells) after 10 s and 20 s (150 steps a 10 s), cells
    /// burning at 10 s, and the share of the strip that burnt by 60 s.
    fn front(m: f32) -> (usize, usize, usize, f32) {
        let (w, h) = (200, 9);
        let cap = vec![1.0; w * h];
        let agar = vec![true; w * h];
        let g = Ground { w, h, cap: &cap, agar: &agar, pile_of: &[], wet: &[], scorch: &[] };
        let r = Rates { kinds: KIND, ..R };
        let kind = vec![MEADOW; w * h];
        let trail = vec![0.0; w * h];
        let moist = vec![m; w * h];
        let mut plant = vec![1.0; w * h];
        let mut ch = vec![0.0; w * h];
        let (mut a, mut b) = (vec![0.0; w * h], vec![0.0; w * h]);
        for y in 0..h {
            a[y * w] = 1.0;
        }
        let (mut f10, mut f20, mut lit10) = (0, 0, 0);
        for t in 1..=900u32 {
            let lit = fire_step(&g, &r, &F, &trail, &kind, &mut plant, &mut ch, &a, &mut b, &moist, t, (&mut [], 0.0));
            std::mem::swap(&mut a, &mut b);
            let far = |a: &[f32]| (0..w * h).filter(|&i| a[i] > 0.0).map(|i| i % w).max().unwrap_or(0);
            if t == 150 {
                (f10, lit10) = (far(&a), lit);
            }
            if t == 300 {
                f20 = far(&a);
            }
        }
        let burnt = plant.iter().filter(|&&p| p < 0.5).count() as f32 / (w * h) as f32;
        (f10, f20, lit10, burnt)
    }

    /// The owner (11-14 19:18Z): fire 5x slower than the first cut (5 cells
    /// a second, an even ribbon), patchy, sometimes going out where damp.
    #[test]
    fn moisture_slows_fire_and_puts_it_out() {
        for m in [0.0, 0.2, 0.4, 0.6, 0.8] {
            let (f10, f20, lit, burnt) = front(m);
            println!("moisture {m}: front {f10} at 10 s, {f20} at 20 s; {lit} burning at 10 s; {:.0}% burnt by 60 s", 100.0 * burnt);
        }
        let (dry10, dry20, _, dry_burnt) = front(0.2);
        assert!((5..=20).contains(&dry10) && dry20 > dry10, "about a cell a second on dry ground: {dry10} {dry20}");
        let (_, damp20, _, damp_burnt) = front(0.6);
        assert!(damp20 < dry20 && damp_burnt < dry_burnt, "damp is slower and burns less: {damp20} {damp_burnt}");
    }
}
