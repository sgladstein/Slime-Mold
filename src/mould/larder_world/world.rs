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
    /// The mould's mass for the goals (the run's whole-game review, 11-15
    /// W30): `living` here, the enemy's living on the GPU world. Until
    /// 11-17 a condensed or thickened walker counted twice (`BigDish::mass`,
    /// deleted with condensing).
    fn mass(&self) -> usize {
        self.living()
    }
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
    /// Open ground a soldier can stand on (agar: not rock, wall or rim, and
    /// not deep water, `big::SOIL_DEEP`). Buildings go only here too.
    fn open(&self, x: f32, y: f32) -> bool;
    /// `open` at every cell's middle, row by row: the grid a walking map
    /// is built on (`Game::standable_cells`). A world that holds its cells
    /// reads them straight (7.6 ms through `open` at 6x a planet's area,
    /// 2026-11-14).
    fn open_cells(&self) -> Vec<bool> {
        let (w, h) = self.size();
        (0..w * h).map(|i| self.open((i % w) as f32 + 0.5, (i / w) as f32 + 0.5)).collect()
    }
    /// Ground a shot flies over: open ground and deep water (shots fly over
    /// water; only rock, a wall or the rim stops them). The default is
    /// `open`, for a world with no deep water.
    fn clear(&self, x: f32, y: f32) -> bool {
        self.open(x, y)
    }
    /// Cell `i` (row by row) is a built wall's, standing or worn to rubble
    /// (`BigDish::wall`), not rock: what a door's footprint marks
    /// (`Game::mark_doors`). No default, so a world that wraps another
    /// can't forget to pass it on and leave its doors shut.
    fn wall_cell(&self, i: usize) -> bool;
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
    /// The surges' switch and longest wait, fine steps
    /// (`BigParams::surge_every`; 0 off, or a world with no surge).
    fn surge_every(&self) -> u32 {
        0
    }
    /// The step the game's last surge began on (`BigParams::surge_at`):
    /// the game times the surges by what the slime eats (`Rules::surge_food`).
    /// A world with no surge ignores it.
    fn set_surge_at(&mut self, _at: u32) {}
    /// A raid (`BigParams::raid_at`): from step `at` the walkers within
    /// `r` dish cells of `to` (fine cells) whose draw is under `share` run
    /// for it; `at` 0 is none. A world with no raids ignores it.
    /// `field` is its walking distance (`big::raid_field_for` on
    /// `raid_ground`), empty to steer straight.
    fn set_raid(&mut self, _at: u32, _to: (f32, f32), _r: f32, _share: f32, _field: Vec<u32>) {}
    /// The units' walking distance (`big::BigDish::units_field`, S1;
    /// `big::units_field_for` on `raid_ground`), which charging walkers
    /// descend from the next step; empty to steer straight. A world with
    /// no slime of this kind ignores it.
    fn set_units_field(&mut self, _field: Vec<u32>) {}
    /// **The far smell** (`BigParams::far_smell`, the simulation-use
    /// plan's S2, 2026-11-17): whether the slime reads one, so the game
    /// builds its field (`Game::time_food_paths`).
    fn far_smell(&self) -> bool {
        false
    }
    /// The far smell's sources: the untouched crystals' centres (fine
    /// cells, x | y << 16; `BigDish::fresh_crystals`).
    fn fresh_crystals(&self) -> Vec<u32> {
        Vec::new()
    }
    /// Sets the far smell's field (`BigDish::food_field`): each block's
    /// walking distance to the nearest untouched crystal.
    fn set_food_field(&mut self, _field: Vec<u32>) {}
    /// **The lure's field** (`gpu::Settings::lure_pull`, 2026-11-18, the
    /// Enemy lane): whether the slime reads one, so the game builds it
    /// (`Game::time_lure_paths`). A rule of the GPU engine only (the
    /// owner's card, 11-17 19:21Z: no new CPU twin), so the CPU world
    /// keeps these defaults and a lure there is its scent alone.
    fn lure_pull(&self) -> bool {
        false
    }
    /// Sets the lure's field: each block's walking distance to the
    /// nearest lure within its reach (`u32::MAX` past it), or empty for
    /// none.
    fn set_lure_field(&mut self, _field: Vec<u32>) {}
    /// **The fort** (2026-11-18, the Enemy lane): whether the slime sets
    /// on a mask the game gives it, so the game builds one
    /// (`Game::time_fort`). A rule of the GPU engine only, as the lure's
    /// field is, so the CPU world keeps these defaults.
    fn forts(&self) -> bool {
        false
    }
    /// Sets the fort's mask: a word a `big::RAID_BLOCK` block, row by row,
    /// not 0 where a walker sets with no scar; empty for none.
    fn set_fort(&mut self, _mask: Vec<u32>) {}
    /// **Lumps** (held bodies, `BigParams::lumps`, the mini-boss thread
    /// 2026-11-16): forms lump `b` of the ordinary walkers within `r` dish
    /// cells of `at` (fine cells), or releases it (None). A world with no
    /// lumps ignores it.
    fn form_lump(&mut self, _b: usize, _at: Option<((f32, f32), f32)>) {}
    /// Lump `b` takes in the slime round (`c`, `r`) in no lump, its own
    /// members kept as they are (`big::BigDish::grow_lump`).
    fn grow_lump(&mut self, _b: usize, _at: ((f32, f32), f32)) {}
    /// Lump `b`'s steering (`big::Lump`) from the next step.
    fn set_lump(&mut self, _b: usize, _l: crate::mould::big::Lump) {}
    /// Each lump's centre (fine cells) and members as the last step began
    /// (None for none; always None in a world with no lumps).
    fn lumps(&self) -> [Option<(f32, f32, u32)>; crate::mould::big::LUMPS] {
        [None; crate::mould::big::LUMPS]
    }
    /// **Sowing** (2026-11-20, `fruit.rs`): every walker within `r` dish
    /// cells of `at` (fine cells) leaves as a spore thrown straight out for
    /// `steps` steps, which homes on food and lands as living slime
    /// (`gpu::Engine::sow_pass`). A world that cannot (`can_sow` false: the
    /// CPU engine, which takes no new slime rules) ignores it.
    fn sow(&mut self, _at: (f32, f32), _r: f32, _steps: u16) {}
    /// Whether `sow` does anything, so a world that cannot never fruits.
    fn can_sow(&self) -> bool {
        false
    }
    /// **The wave's field** (2026-11-20, `Game::time_waves`): each block's
    /// walking distance to the nearest of the wave's points (`u32::MAX`
    /// past its reach), or empty for none, and the pull a calm walker
    /// leans down it with (`gpu::Engine::write_wave_field`). A rule of the
    /// GPU engine only, as the lure's field is.
    fn set_wave_field(&mut self, _field: Vec<u32>, _pull: f32) {}
    /// Whether `set_wave_field` does anything, so a world that cannot
    /// never sends a wave.
    fn can_wave(&self) -> bool {
        false
    }
    /// **The heartbeat** (2026-11-20, `Game::time_heart`): where it beats
    /// from (fine cells; None off), a calm walker's lean, the steps a beat
    /// and its wavelength in fine cells (`gpu::Engine::heart`). A rule of
    /// the GPU engine only.
    fn set_heart(&mut self, _at: Option<(f32, f32)>, _lean: f32, _period: u32, _len: u32) {}
    /// The ground a raid walks (the slime's agar, `w` x `h` fine cells),
    /// copied so the game can build the raid's way round walls off the
    /// frame; None for a world with no raids.
    fn raid_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        None
    }
    /// A full pile's scent a step, which a soldier's smell is a share of.
    fn food_deposit(&self) -> f32;
    /// Fine steps a tick (`BigParams::substeps`), for rules timed in
    /// fine-step fields (`Rules::crater`).
    fn fine_steps(&self) -> u32;
    /// A walker's step in fine cells a fine step (`BigParams::step`): a
    /// flung or pushed walker flies `fling_speed` of them a fine step, so a
    /// push that must stop short (`Card::Pull`) sets its length by it.
    fn walker_step(&self) -> f32 {
        1.0
    }
    fn hash(&self) -> u64;
    /// Set walkers a fine cell (`BigDish::set_cells`, E2b), row by row;
    /// None while setting is off.
    fn set_cells(&self) -> Option<Vec<u32>> {
        None
    }
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
    /// The sleep gas cloud at each point of the last `calm`, in levels that
    /// put the slime to sleep (1 is `gas_sleep`); empty where the world
    /// reads none (2026-11-18, `Rules::doze`; GPU only).
    fn clouds(&self) -> Vec<f32> {
        Vec::new()
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
    /// Several bursts in their order, each as `burst`, with their counts
    /// (2026-11-16, the gun-card plan's read-back batching): the GPU world
    /// waits on one read-back for the whole batch where `burst` waited on
    /// up to three a burst (the kill, the fling and the gas counts), so a
    /// volley of payloads stalls the frame once.
    fn bursts(&mut self, list: &[(squad::Burst, squad::BurstRules)]) -> Vec<squad::BurstOut> {
        list.iter().map(|(b, k)| self.burst(b, k)).collect()
    }
    /// Walkers fire killed since the last call that no `burst` counted:
    /// the plants' fire on every world, and on a GPU world the
    /// `Burst::burn` bursts too (counted here in one read-back; a CPU world
    /// counts a burn in its `burst`). `Game::finish_step` asks once a
    /// frame, so the plants' kills count in every game (bug check 11-15).
    fn burned(&mut self) -> u32 {
        0
    }
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
        self.cell_at(x, y).is_some_and(|i| self.agar[i] && !self.deep(i))
    }

    fn open_cells(&self) -> Vec<bool> {
        (0..self.w * self.h).map(|i| self.agar[i] && !self.deep(i)).collect()
    }

    fn clear(&self, x: f32, y: f32) -> bool {
        self.cell_at(x, y).is_some_and(|i| self.agar[i])
    }

    fn wall_cell(&self, i: usize) -> bool {
        self.wall.get(i).copied().unwrap_or(false)
    }

    fn stop_births(&mut self) {
        self.p.birth_energy = f32::INFINITY;
    }

    /// Walkers the plants' fire killed since the last call (a burst's own
    /// kills are counted in `burst`).
    fn burned(&mut self) -> u32 {
        let n = self.fire_killed - self.fire_told;
        self.fire_told = self.fire_killed;
        n as u32
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

    fn surge_every(&self) -> u32 {
        self.p.surge_every
    }

    fn set_surge_at(&mut self, at: u32) {
        self.p.surge_at = at;
    }

    fn set_raid(&mut self, at: u32, to: (f32, f32), r: f32, share: f32, field: Vec<u32>) {
        self.p.raid_at = at;
        self.p.raid_to = raid_cell(to, (self.w, self.h));
        self.p.raid_r = r;
        self.p.raid_share = share;
        self.raid_field = field;
    }

    fn raid_ground(&self) -> Option<(Vec<bool>, usize, usize)> {
        Some((self.agar.clone(), self.w, self.h))
    }

    fn set_units_field(&mut self, field: Vec<u32>) {
        self.units_field = field;
    }

    fn far_smell(&self) -> bool {
        self.p.far_smell > 0.0
    }

    fn fresh_crystals(&self) -> Vec<u32> {
        BigDish::fresh_crystals(self)
    }

    fn set_food_field(&mut self, field: Vec<u32>) {
        self.food_field = field;
    }

    fn form_lump(&mut self, b: usize, at: Option<((f32, f32), f32)>) {
        let sc = self.p.scale as f32;
        BigDish::form_lump(self, b, at.map(|(c, r)| lump_reach(c, r * sc, (self.w, self.h))));
    }

    fn grow_lump(&mut self, b: usize, (c, r): ((f32, f32), f32)) {
        let sc = self.p.scale as f32;
        BigDish::grow_lump(self, b, lump_reach(c, r * sc, (self.w, self.h)));
    }

    fn set_lump(&mut self, b: usize, l: crate::mould::big::Lump) {
        self.p.lumps[b] = l;
    }

    fn lumps(&self) -> [Option<(f32, f32, u32)>; crate::mould::big::LUMPS] {
        std::array::from_fn(|b| self.lump_centre(b))
    }

    fn food_deposit(&self) -> f32 {
        self.p.food_deposit
    }

    fn fine_steps(&self) -> u32 {
        self.p.substeps as u32
    }

    fn walker_step(&self) -> f32 {
        self.p.step
    }

    fn set_cells(&self) -> Option<Vec<u32>> {
        (self.p.set_scar > 0.0 && !self.set_cells.is_empty() && !self.scar.is_empty()).then(|| self.set_cells.clone())
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
        // Each start an organic outline, not a disc (`BigParams::start_shape`).
        start_shape: true,
        scorch_keep: 0.0,
        // The game ticks 15 times a game second (`Rules::tps`), an eighth
        // of the dish's 120, so the plants step every tick: 15 plant steps
        // a second either way (their rates are a plant step's).
        plant_every: 1,
        seed,
        ..BigParams::today()
    }
}

/// A raid's target as the engines hold it (`BigParams::raid_to`): the
/// whole fine cell, x | y << 16, inside the world.
pub fn raid_cell(to: (f32, f32), (w, h): (usize, usize)) -> u32 {
    let cx = (to.0.floor().max(0.0) as u32).min(w.saturating_sub(1) as u32).min(0xFFFF);
    let cy = (to.1.floor().max(0.0) as u32).min(h.saturating_sub(1) as u32).min(0xFFFF);
    cx | cy << 16
}

/// A lump's forming as both engines take it (`BigDish::form_lump`,
/// `Engine::form_lump`): its centre as a fine cell (`raid_cell`) and its
/// reach `r` (fine cells) squared in whole cells.
pub fn lump_reach(at: (f32, f32), r: f32, size: (usize, usize)) -> (u32, u32) {
    let r = r.clamp(0.0, 60_000.0);
    (raid_cell(at, size), (r * r).floor() as u32)
}
