//! The `World` the game plays on (the CPU `BigDish` or the GPU engine) and
//! the CPU race world. Moved out of `larder_world.rs` unchanged (C9,
//! 2026-11-14).

use super::*;

/// The world under the game: what the rules need of it, and nothing else.
pub trait World {
    /// Fine cells across and down.
    fn size(&self) -> (usize, usize);
    /// Fine cells a dish cell.
    fn scale(&self) -> f32;
    /// Ticks run.
    fn ticks(&self) -> u64;
    /// One squad frame around `ticks` ticks (`squad::frame`).
    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut;
    /// Starts frame `f` as `frame` does, without waiting for its results
    /// where the world can (the GPU world): then None, and `frame_finish`
    /// gives them. The default runs the frame at once.
    fn frame_start(&mut self, f: &Frame, ticks: u32) -> Option<FrameOut> {
        Some(self.frame(f, ticks))
    }
    /// The results of the frame `frame_start` left running, waiting for
    /// them if they are not in. Only called after a `frame_start` that
    /// returned None.
    fn frame_finish(&mut self) -> FrameOut {
        unreachable!("no frame running")
    }
    /// Whether `frame_finish` would return without waiting.
    fn frame_ready(&self) -> bool {
        true
    }
    /// A wall from `a` to `b`, dish cells, `width` dish cells thick.
    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32);
    /// Walkers alive (not counting the dead).
    fn living(&self) -> usize;
    /// The player's own mould's living walkers (`own_mould`'s garden, on
    /// either engine); 0 where the player has none. The campaign's
    /// `goal outgrow` reads it (mission 8).
    fn own_living(&self) -> usize {
        0
    }
    /// Soldier `k`'s gardener's task (`own_mould::Task`) where the player
    /// has a mould of their own; elsewhere nothing. The campaign's own-mould
    /// plans give it (`plans::sow`).
    fn set_task(&mut self, _k: usize, _t: crate::mould::own_mould::Task) {}
    /// Each crystal's centre (fine cells) and charge left.
    fn crystals(&self) -> Vec<((f32, f32), f32)>;
    /// Crystal `c`'s radius, fine cells.
    fn crystal_r(&self, c: usize) -> f32;
    /// Where the squad lands, fine cells, if the world places it (a map,
    /// `BigDish::with_map`); else the game puts the larder on the crystal
    /// farthest from the mould's home (`far_crystal`).
    fn landing(&self) -> Option<(f32, f32)> {
        None
    }
    /// Open ground a soldier can stand on (agar: not rock, wall or rim).
    fn open(&self, x: f32, y: f32) -> bool;
    /// The mould's core is taken: no more births.
    fn stop_births(&mut self);
    /// The mould's upkeep a fine step (`BigParams::upkeep`; 0 with growth
    /// off), and setting it: the broken mould (`Rules::broken`) starves its
    /// stragglers by raising it, and gives it back when the mould recovers.
    fn upkeep(&self) -> f32;
    fn set_upkeep(&mut self, up: f32);
    /// How far a scared walker sees a unit to charge (`BigParams::panic_r`,
    /// dish cells): the game keeps it past the gun's range
    /// (`Rules::panic_margin`). A world with no panic ignores it.
    fn set_panic_r(&mut self, _r: f32) {}
    /// A full pile's scent a step, which a soldier's smell is a share of.
    fn food_deposit(&self) -> f32;
    /// Fine steps a tick (`BigParams::substeps`), for rules timed in
    /// fine-step fields (`Rules::crater`).
    fn fine_steps(&self) -> u32;
    fn hash(&self) -> u64;
    /// Walkers on each `bin` x `bin` square of fine cells, row by row (the
    /// tube a player sees on screen, coarsened), for the player AI's eyes
    /// (`larder_world_ai::View`). None where the world can't give it
    /// cheaply: the GPU world needs an occupancy read-back first.
    fn mould_grid(&self, _bin: usize) -> Option<Vec<u32>> {
        None
    }
    /// For each point (fine cells), the gassed sleeping walkers within `r`
    /// (`BigDish::gassed_near`), which don't bite; 0 where the world has
    /// no sleep gas (the GPU world, so far).
    fn calm(&self, at: &[(f32, f32)], _r: f32) -> Vec<u32> {
        vec![0; at.len()]
    }
    /// The player's own mould (`own_mould::Garden`): the soldiers' tasks,
    /// once a frame before the frame's `ticks`, with the bank they may
    /// spend; returns the food spent. Worlds with no player mould do
    /// nothing.
    fn tend(&mut self, _soldiers: &[Soldier], _bank: f32, _ticks: u32) -> f32 {
        0.0
    }
    /// A squad toolkit burst (`squad::Burst`), now, between frames: the
    /// kill, clear, alarm and crater on every world; gas and germ only
    /// where `has_gas` and `has_germ` say so (a world without them leaves
    /// those parts out, and the toolkit doesn't sell them there).
    fn burst(&mut self, b: &squad::Burst, k: &squad::BurstRules) -> squad::BurstOut;
    /// Whether `burst`'s gas and germ parts work on this world.
    fn has_gas(&self) -> bool {
        false
    }
    fn has_germ(&self) -> bool {
        false
    }
    /// Walkers holding a cell within `r` fine cells of `at` (fine cells),
    /// for a mine's trigger. The default sums the mould grid's squares of
    /// one dish cell whose centre is in reach (the GPU world's, from the
    /// frame's readout); 0 with no grid.
    fn walkers_in(&self, at: (f32, f32), r: f32) -> u32 {
        let bin = (self.scale() as usize).max(1);
        let Some(g) = self.mould_grid(bin) else { return 0 };
        let (w, _) = self.size();
        let gw = w.div_ceil(bin);
        let b = bin as f32;
        g.iter()
            .enumerate()
            .filter(|&(i, _)| ((((i % gw) as f32 + 0.5) * b - at.0).hypot((((i / gw) as f32 + 0.5) * b) - at.1)) <= r)
            .map(|(_, &n)| n)
            .sum()
    }
}

impl World for BigDish {
    fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    fn scale(&self) -> f32 {
        self.p.scale as f32
    }

    fn ticks(&self) -> u64 {
        self.ticks
    }

    fn frame(&mut self, f: &Frame, ticks: u32) -> FrameOut {
        squad::frame(self, f, ticks)
    }

    fn build_wall(&mut self, a: (f32, f32), b: (f32, f32), width: f32) {
        BigDish::build_wall(self, a.0, a.1, b.0, b.1, width);
    }

    fn living(&self) -> usize {
        let c = self.life_counts();
        c.iter().sum::<usize>() - c[LIFE_DEAD as usize]
    }

    fn crystals(&self) -> Vec<((f32, f32), f32)> {
        let s = self.p.scale as f32;
        self.piles.iter().enumerate().map(|(p, &(x, y))| ((x * s, y * s), self.pile_charge.get(p).copied().unwrap_or(0.0))).collect()
    }

    fn crystal_r(&self, c: usize) -> f32 {
        self.pile_rs[c] * self.p.scale as f32
    }

    fn landing(&self) -> Option<(f32, f32)> {
        let s = self.p.scale as f32;
        self.landing.map(|(x, y)| (x * s, y * s))
    }

    fn open(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && y >= 0.0 && (x as usize) < self.w && (y as usize) < self.h && self.agar[y as usize * self.w + x as usize]
    }

    fn stop_births(&mut self) {
        self.p.birth_energy = f32::INFINITY;
    }

    fn upkeep(&self) -> f32 {
        self.p.upkeep
    }

    fn set_upkeep(&mut self, up: f32) {
        self.p.upkeep = up;
    }

    fn set_panic_r(&mut self, r: f32) {
        self.p.panic_r = r;
    }

    fn food_deposit(&self) -> f32 {
        self.p.food_deposit
    }

    fn fine_steps(&self) -> u32 {
        self.p.substeps as u32
    }

    fn hash(&self) -> u64 {
        BigDish::hash(self)
    }

    fn calm(&self, at: &[(f32, f32)], r: f32) -> Vec<u32> {
        self.gassed_near(at, r)
    }

    fn burst(&mut self, b: &squad::Burst, k: &squad::BurstRules) -> squad::BurstOut {
        squad::burst(self, b, k)
    }

    fn has_gas(&self) -> bool {
        true
    }

    fn has_germ(&self) -> bool {
        true
    }

    /// Exact: the cells within `r` (the mine count `Kit::frame` used
    /// before the toolkit ran on both worlds).
    fn walkers_in(&self, at: (f32, f32), r: f32) -> u32 {
        let occ = self.occupancy();
        let n = r.ceil() as i64;
        let mut c = 0u32;
        for dy in -n..=n {
            for dx in -n..=n {
                if ((dx * dx + dy * dy) as f32) > r * r {
                    continue;
                }
                let (cx, cy) = (at.0 as i64 + dx, at.1 as i64 + dy);
                if cx < 0 || cy < 0 || cx as usize >= self.w || cy as usize >= self.h {
                    continue;
                }
                c += occ[cy as usize * self.w + cx as usize] as u32;
            }
        }
        c
    }

    fn mould_grid(&self, bin: usize) -> Option<Vec<u32>> {
        let (gw, gh) = (self.w.div_ceil(bin), self.h.div_ceil(bin));
        let mut out = vec![0u32; gw * gh];
        for (i, &n) in self.occupancy().iter().enumerate() {
            if n > 0 {
                out[(i / self.w / bin) * gw + (i % self.w) / bin] += n as u32;
            }
        }
        Some(out)
    }
}

/// **The race world** on the CPU (owner, 14:54Z: the mould starts in one
/// spot and must expand, with food for a long game; an idle player should
/// lose to a strengthening mould): the W1b world's rect, 23 crystals that
/// run dry at charge 512 a cell, rock and wet ground, and W0's growth from a
/// blob of `walkers` on the home crystal, with room for ten times as many.
/// `scale` 2 is the 512 x 320 world, 1 a 256 x 160 one for tests.
pub fn race_world(seed: u64, scale: usize, walkers: usize) -> BigDish {
    BigDish::new(race_params(seed, scale, walkers))
}

/// The mould's turn from fresh gunfire, an opt-in (`flee=W` in the census
/// and the hole test), not the race world's default. With the alarm field on
/// (`alarm_keep` 0.98, the Swarmer's) and read at weight -1, the alarm each
/// hit lays (`squad::SHOT_ALARM`, out to twice the strike's radius) is
/// ground the mould backs away from while it is fresh. It answers the
/// owner's 04:39Z bar ("A group of soldiers shooting the slime should start
/// carving out a bigger and bigger hole that doesn't refill while being
/// shot at") in the hole test (`examples/larderhole.rs`, 16 seeds): 102
/// dish cells after 20 s of four soldiers' fire against 29 without it. It
/// was the default for a few hours on 2026-11-12 and failed the census gate
/// (race seeds 5-20, free shots): random wiped the mould on 12 of 16 seeds
/// (2 without it) and beat farm, wall and rush 13-3, because random's
/// Attack orders hold the core once the mould no longer overruns soldiers
/// there, and a mould without births starves; the first touch on a squad
/// came later too (farm +36 s median). `Reports/dead-ends.md` has the
/// condition: a flee that holds the core (or the mould's own crystals)
/// could pass.
pub const FLEE_KEEP: f32 = 0.98;
pub const FLEE_WEIGHT: f32 = -1.0;

pub fn race_params(seed: u64, scale: usize, walkers: usize) -> BigParams {
    BigParams {
        scale,
        substeps: scale,
        walkers,
        capacity: walkers * 10,
        scent_offset: 18.0 * scale as f32,
        scent_coarse: false,
        piles: 23,
        rect: true,
        crystal_charge: 512.0,
        crystal_eat: 0.005,
        terrain_rock: 0.2,
        terrain_wet: 0.2,
        upkeep: 0.001,
        // The shot's crater as a cone (the owner's pick of three blind
        // looks, 2026-11-12 14:41Z, "I think 3 is best"): a hit bites
        // fully at its middle and less toward its rim (`scorch_cone`,
        // `scorch_full` 150), and the bite holds no trail (`scorch_keep`
        // 0), so the hole has a soft shadow where the flat disc drew a dark
        // rim. On the 8-seed hole test it played as the flat disc did (hole
        // 54 dish cells at 20 s against 60, 2 larger, 5 smaller, p 0.45).
        // The toolkit's blasts keep their flat disc but share keep and
        // full.
        scorch_cone: true,
        scorch_full: 150,
        scorch_keep: 0.0,
        seed,
        ..BigParams::today()
    }
}
