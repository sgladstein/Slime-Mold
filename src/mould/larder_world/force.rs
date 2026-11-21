//! **The slime viewer's spawns** (2026-11-20, the thread "Build a slime
//! viewer to tune looks"). The owner, 23:17Z: "I want to review some of the
//! changes in the slime looks. Can we have a thread build a slime viewer so
//! I can play around with the features/looks that we have build and modify
//! them myself?"
//!
//! Each of the slime's events comes on its own signal in play (a mini-boss
//! on a crowded knot, a fruit and a wave when the squad hurts it, the heart
//! late against an army, ore on a drawn clock), so seeing one means playing
//! until it happens. These start one now, where the viewer is clicked, by
//! the same code play uses: nothing here is a second copy of a rule, and
//! the game never calls them.

use super::{BossCause, BossKind, CoreFrom, Drop, Game, World};

impl<W: World> Game<W> {
    /// A mini-boss of `kind` round `at` (fine cells), as `boss=` stages one
    /// (`BossCause::Staged`), its grade drawn as play draws it; false with
    /// no free lump or too few walkers there.
    pub fn force_boss(&mut self, kind: BossKind, at: (f32, f32)) -> bool {
        let was = self.rules.boss.kind;
        self.rules.boss.kind = (BossKind::ALL.iter().position(|&k| k == kind).unwrap_or(0) + 1) as f32;
        let reach = self.rules.boss.reach;
        let formed = self.form_boss(at, reach, BossCause::Staged);
        self.rules.boss.kind = was;
        formed
    }

    /// A fruit round `at` (fine cells), turning fruiting on; false while
    /// one ripens, with no free lump, or on a world that cannot sow (the
    /// CPU engine).
    pub fn force_fruit(&mut self, at: (f32, f32)) -> bool {
        if self.fruits.live.is_some() || !self.world.can_sow() {
            return false;
        }
        self.rules.fruit.on = true;
        self.start_fruit_at(at)
    }

    /// A wave gathering round the squad now, turning waves on; false while
    /// one is under way, with no soldier, or on a world without waves.
    pub fn force_wave(&mut self) -> bool {
        if self.waves.live.is_some() || !self.world.can_wave() || !self.soldiers.iter().any(|s| s.alive) {
            return false;
        }
        self.rules.wave.on = true;
        self.start_wave()
    }

    /// The last-stand heart gathering now on the heart's crystal, turning
    /// it on (and letting it form again this planet); false while one
    /// lives or with no slime on a crystal.
    pub fn force_stand(&mut self) -> bool {
        if self.stand.live.is_some() || !self.world.can_sow() {
            return false;
        }
        self.rules.stand.on = true;
        self.start_stand()
    }

    /// A raid at the next game second, at what play's raids aim at; false
    /// with raids off (`raid_every` 0).
    pub fn force_raid(&mut self) -> bool {
        if self.rules.raid.every <= 0.0 {
            return false;
        }
        self.raids.next = Some(self.world.ticks());
        true
    }

    /// Ore falling on `at` (fine cells) after the planet's warning, worth
    /// what the planet's ore is worth; false on rock or off the map.
    pub fn force_ore(&mut self, at: (f32, f32)) -> bool {
        if !self.world.open(at.0, at.1) {
            return false;
        }
        let r = self.rules.star;
        let tps = self.rules.tps as f32;
        let now = self.world.ticks();
        let lands = now + (r.warn.max(0.0) * tps).round() as u64;
        let ends = lands + (r.life.max(0.0) * tps).round() as u64;
        let sc = self.world.scale();
        self.starfall.live.push(Drop::ore(at, r.worth, now, lands, ends, r.ore_r * sc));
        true
    }

    /// A mini-boss's core lying at `at` (fine cells), worth a core's base
    /// worth: it runs home and re-forms its boss if the slime takes it back
    /// (`cores.rs`); false on rock or off the map.
    pub fn force_core(&mut self, at: (f32, f32)) -> bool {
        if !self.world.open(at.0, at.1) {
            return false;
        }
        let r = self.rules.star;
        let now = self.world.ticks();
        let ends = now + (r.core_life.max(0.0) * self.rules.tps as f32).round() as u64;
        let sc = self.world.scale();
        self.starfall.cores += 1;
        self.starfall.live.push(Drop::core(at, r.core_worth, CoreFrom::Boss { kind: BossKind::Hunter, grade: 0.5 }, now, ends, r.core_r * sc));
        true
    }
}
