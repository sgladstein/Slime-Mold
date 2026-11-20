//! **Fruiting** (2026-11-20, the thread "Make the slime a more interesting
//! enemy"). The owner, 11-20 05:46Z: "It is a bit uniform of an enemy
//! sometimes. It can be interesting really early in a planet when I am
//! poor and weak but once I get strong enough there is just lots of
//! shooting through masses of uniform orange. Think outside the box".
//!
//! Real slime mould fruits when it is starving: the plasmodium stops
//! foraging, streams into a few knots, and each rises into a sporangium
//! that scatters spores far from the dying body. Here the slime fruits
//! when it is hurt: once your squad has killed a share of its mass in a
//! short window (or it has fallen well under its peak), it picks the
//! thickest slime out of your guns' reach and streams walkers into one
//! knot there (a lump, `big::Lump`, the mini-bosses' and the marches'
//! machinery). The knot packs, swells as more slime joins it, and ripens
//! for `FruitRules::ripen` seconds. Ripe, it bursts: every walker in it is
//! thrown out over the planet as a spore that homes on food and lands as
//! living slime (`World::sow`, the GPU's `sow_pass`), seeding new colonies
//! on crystals the slime had lost or never held, your pumps' among them.
//! Shot down under `quorum` of its most members, it breaks and is slime
//! again.
//!
//! What it does to the game: when the player is winning, the slime stops
//! only feeding itself into the guns and makes a target inside its body
//! that the player has to push in and break, against a clock. A strong
//! squad that ignores it pays with new slime across the map (and the
//! planet is won only at the last walker); a squad that breaks it gets the
//! knot's walkers in one place. Noita's rule (the owner, 11-18 22:33Z): a
//! burst is never a loss alone, and it has a clear counter.
//!
//! The picture is the simulation: the knot is its own walkers packed past
//! the crowd limit (`big::LUMP_CROWD`), drawn with a look of their own
//! from the lump's members (`Game::fruit_looks`, `LumpLook::Fruit`), its
//! strength the ripeness; the burst is those walkers in the air.
//!
//! GPU only (the owner's 11-17 19:21Z card): a world that cannot sow
//! (`World::can_sow`, the CPU engine) never fruits.

use super::{Game, World};
use crate::mould::big::{Lump, LUMPS};

/// Fruiting's numbers. Times in game seconds, distances in dish cells.
/// Off by default (the campaign and the sandbox are frozen); a run turns
/// it on for every planet (`run::ENEMY_RULES`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FruitRules {
    pub on: bool,
    /// No fruit until this long after the landing: the planet's first
    /// minutes, when the player is poor and weak, stay as they were (the
    /// owner liked them).
    pub first: f32,
    /// Hurt: the squad's kills over the last `window` seconds at least
    /// this share of the slime's mass now.
    pub hurt: f32,
    pub window: f32,
    /// Or down to this share of its peak (0 off). Off since its first
    /// films: on planet 3 the Trunk fell from 43,000 to 26,000 by 1:50
    /// against a squad of four that had killed 3,556, and fruited with the
    /// squad nowhere near it, so it fired on a slime that was not being
    /// beaten. The hurt test covers the end of a planet too: a fifth of a
    /// small slime is a few kills.
    pub below: f32,
    /// None under this share of its peak: the last of a planet's slime is
    /// a mop-up, not a new spread.
    pub floor: f32,
    /// Seconds from one fruit's end to the next one's start.
    pub every: f32,
    /// The knot: this share of the slime's mass, at least `least` and at
    /// most `most` walkers, gathered round the site; it swells to at most
    /// `swell` times what it first held (and `most`).
    pub size: f32,
    pub swell: f32,
    pub least: f32,
    pub most: f32,
    /// Seconds it ripens before it bursts.
    pub ripen: f32,
    /// Every `grow_every` seconds while it ripens it takes in the slime
    /// within its radius and `grow` dish cells more, up to `most`.
    pub grow_every: f32,
    pub grow: f32,
    /// Shot down under this share of its most members, it breaks.
    pub quorum: f32,
    /// The site is at least this far past the guns' reach from every
    /// soldier (dish cells).
    pub clear: f32,
    /// Its steering (`big::Lump`): the pull to its centre, the leash and
    /// the blows each member takes.
    pub pull: f32,
    pub leash: f32,
    pub armour: u32,
    /// The burst: spores thrown for this many steps (straight while over
    /// the dish's `spore_steps` / 2, then homing on food), from a disc
    /// `spread` times the knot's radius.
    pub throw: u16,
    pub spread: f32,
}

impl Default for FruitRules {
    fn default() -> Self {
        FruitRules {
            on: false,
            first: 90.0,
            hurt: 0.2,
            window: 30.0,
            below: 0.0,
            floor: 0.08,
            every: 40.0,
            size: 0.08,
            swell: 2.0,
            least: 200.0,
            most: 2500.0,
            ripen: 30.0,
            grow_every: 3.0,
            grow: 2.0,
            quorum: 0.35,
            clear: 15.0,
            pull: 40.0,
            leash: 1.0,
            armour: 2,
            throw: 200,
            spread: 1.5,
        }
    }
}

/// How a fruit ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FruitEnd {
    /// Ripe: it sowed its walkers.
    Burst,
    /// Shot down under its quorum.
    Broken,
}

/// One fruit.
#[derive(Clone, Copy, Debug)]
pub struct Fruit {
    /// The lump it is (`World::form_lump`).
    pub lump: usize,
    /// The site (fine cells) and the tick it began.
    pub at: (f32, f32),
    pub born: u64,
    /// Its members at its first look, at the last, the most it has had,
    /// and its centre (fine cells).
    pub first: u32,
    pub members: u32,
    pub most: u32,
    pub centre: (f32, f32),
    /// 0 as it forms to 1 ripe.
    pub ripe: f32,
    /// The tick it last took in more slime.
    grown: u64,
}

/// The game's fruiting state.
#[derive(Debug, Default)]
pub struct Fruits {
    /// The fruit ripening now, if any (one at a time).
    pub live: Option<Fruit>,
    /// Fruits that burst and broke, the walkers sown, and the last one's
    /// end (tick, how, members then).
    pub burst: u32,
    pub broken: u32,
    pub sown: u64,
    pub last: Option<(u64, FruitEnd, u32)>,
    /// The squad's kills at each of the last game seconds, oldest first.
    kills: std::collections::VecDeque<u32>,
}

impl Fruits {
    /// Whether lump `l` is a fruit's.
    pub fn holds(&self, l: usize) -> bool {
        self.live.is_some_and(|f| f.lump == l)
    }
}

impl<W: World> Game<W> {
    /// Once a game second: start a fruit when the slime is hurt, ripen the
    /// one there is, and burst or break it.
    pub(super) fn time_fruit(&mut self) {
        let r = self.rules.fruit;
        let tps = self.rules.tps as f32;
        self.fruits.kills.push_back(self.killed);
        let keep = (r.window.max(1.0)) as usize + 1;
        while self.fruits.kills.len() > keep {
            self.fruits.kills.pop_front();
        }
        if !r.on || !self.world.can_sow() {
            return;
        }
        if self.fruits.live.is_some() {
            self.ripen_fruit();
            return;
        }
        let now = self.world.ticks();
        let secs = |s: f32| (s.max(0.0) * tps) as u64;
        if now < self.landed + secs(r.first) {
            return;
        }
        if let Some((t, _, _)) = self.fruits.last {
            if now < t + secs(r.every) {
                return;
            }
        }
        let (mass, peak) = (self.mould as f32, self.peak as f32);
        if peak <= 0.0 || mass < r.floor * peak || mass * r.size < r.least {
            return;
        }
        let hurt = self.killed.saturating_sub(*self.fruits.kills.front().unwrap_or(&self.killed)) as f32;
        if hurt < r.hurt * mass && mass > r.below * peak {
            // (`below` 0: `mass > 0` always, so only the hurt test starts one.)
            return;
        }
        self.start_fruit();
    }

    /// The fruit's site (fine cells): the slime's thickest square (a box of
    /// 9 x 9 dish cells) at least the guns' reach and `clear` from every
    /// live soldier; None with no grid or no such slime.
    fn fruit_site(&self) -> Option<(f32, f32)> {
        let sc = self.world.scale();
        let bin = (sc as usize).max(1);
        let g = self.world.mould_grid(bin)?;
        let (w, h) = self.world.size();
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        if g.len() < gw * gh {
            return None;
        }
        // A summed table, so each box is four reads.
        let mut s = vec![0u64; (gw + 1) * (gh + 1)];
        for y in 0..gh {
            for x in 0..gw {
                s[(y + 1) * (gw + 1) + x + 1] = g[y * gw + x] as u64 + s[y * (gw + 1) + x + 1] + s[(y + 1) * (gw + 1) + x] - s[y * (gw + 1) + x];
            }
        }
        let box_r = 4usize;
        let far = (self.gun_range_max() + self.rules.fruit.clear) * sc;
        let soldiers: Vec<(f32, f32)> = self.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let b = bin as f32;
        let mut best: Option<(u64, (f32, f32))> = None;
        for y in 0..gh {
            for x in 0..gw {
                if g[y * gw + x] == 0 {
                    continue;
                }
                let (x0, y0, x1, y1) = (x.saturating_sub(box_r), y.saturating_sub(box_r), (x + box_r + 1).min(gw), (y + box_r + 1).min(gh));
                let n = s[y1 * (gw + 1) + x1] + s[y0 * (gw + 1) + x0] - s[y0 * (gw + 1) + x1] - s[y1 * (gw + 1) + x0];
                if best.is_some_and(|(m, _)| m >= n) {
                    continue;
                }
                let p = ((x as f32 + 0.5) * b, (y as f32 + 0.5) * b);
                if soldiers.iter().any(|q| (q.0 - p.0).hypot(q.1 - p.1) < far) {
                    continue;
                }
                best = Some((n, p));
            }
        }
        best.map(|(_, p)| p)
    }

    /// The smallest reach round `at` (dish cells) whose squares of the
    /// mould grid hold `want` walkers.
    fn fruit_reach(&self, at: (f32, f32), want: f32) -> f32 {
        let sc = self.world.scale();
        let bin = (sc as usize).max(1);
        let Some(g) = self.world.mould_grid(bin) else { return 6.0 };
        let (w, _) = self.world.size();
        let gw = w.div_ceil(bin);
        let b = bin as f32;
        let mut near: Vec<(f32, u32)> = g.iter().enumerate().filter(|&(_, &n)| n > 0).map(|(i, &n)| ((((i % gw) as f32 + 0.5) * b - at.0).hypot(((i / gw) as f32 + 0.5) * b - at.1), n)).collect();
        near.sort_by(|a, c| a.0.total_cmp(&c.0));
        let mut held = 0.0;
        for (d, n) in near {
            held += n as f32;
            if held >= want {
                return (d + 0.5 * b) / sc;
            }
        }
        60.0
    }

    /// Lump `l` taken by a mini-boss, a march or a fruit.
    pub(super) fn lump_taken(&self, l: usize) -> bool {
        self.bosses.live.iter().chain(self.bosses.fading.iter().map(|f| &f.0)).any(|b| b.lump == l) || self.raids.marching.iter().any(|m| m.lump == l) || self.fruits.holds(l)
    }

    fn start_fruit(&mut self) {
        let r = self.rules.fruit;
        let Some(lump) = (0..LUMPS).find(|&l| !self.lump_taken(l)) else { return };
        let Some(at) = self.fruit_site() else { return };
        let want = (r.size * self.mould as f32).clamp(r.least, r.most.max(r.least));
        let reach = self.fruit_reach(at, want);
        self.world.form_lump(lump, Some((at, reach)));
        let size = self.world.size();
        self.world.set_lump(lump, Lump { aim: super::raid_cell(at, size), pull: r.pull, leash: r.leash, armour: r.armour, ..Lump::default() });
        let now = self.world.ticks();
        self.fruits.live = Some(Fruit { lump, at, born: now, first: 0, members: 0, most: 0, centre: at, ripe: 0.0, grown: now });
    }

    /// The fruit there is, once a game second: its count, its growth, and
    /// its end.
    fn ripen_fruit(&mut self) {
        let Some(mut f) = self.fruits.live else { return };
        let r = self.rules.fruit;
        let tps = self.rules.tps as f32;
        let sc = self.world.scale();
        let now = self.world.ticks();
        let lumps = self.world.lumps();
        let (centre, n) = lumps[f.lump].map_or((f.centre, 0), |(x, y, n)| ((x, y), n));
        (f.centre, f.members) = (centre, n);
        if f.first == 0 {
            f.first = n;
        }
        f.most = f.most.max(n);
        f.ripe = (now.saturating_sub(f.born) as f32 / (r.ripen.max(1.0) * tps)).min(1.0);
        // Its radius in dish cells, as the leash holds it (a walker a fine
        // cell, packed up to `LUMP_CROWD` more).
        let radius = (n as f32 / std::f32::consts::PI).sqrt() / sc;
        // The lump's sums lag a step behind its forming, so a fruit with
        // none yet is given three seconds before it counts as gone.
        let empty = if f.first == 0 { now > f.born + (3.0 * tps) as u64 } else { n == 0 };
        let end = if empty || (f.most > 0 && (n as f32) < r.quorum * f.most as f32) {
            Some(FruitEnd::Broken)
        } else if f.ripe >= 1.0 {
            Some(FruitEnd::Burst)
        } else {
            None
        };
        match end {
            Some(how) => {
                self.world.form_lump(f.lump, None);
                if how == FruitEnd::Burst {
                    self.world.sow(f.centre, (radius * r.spread).max(3.0), r.throw);
                    self.fruits.burst += 1;
                    self.fruits.sown += n as u64;
                } else {
                    self.fruits.broken += 1;
                }
                self.fruits.last = Some((now, how, n));
                self.fruits.live = None;
            }
            None => {
                // It swells: the slime round it streams in, up to `swell`
                // times its first members and `most`.
                let cap = r.most.min(r.swell * f.first as f32);
                if now >= f.grown + (r.grow_every.max(0.5) * tps) as u64 && (n as f32) < cap {
                    self.world.grow_lump(f.lump, (f.centre, radius + r.grow));
                    f.grown = now;
                }
                self.fruits.live = Some(f);
            }
        }
    }

    /// What the view draws of the fruit: its lump, centre (fine cells) and
    /// ripeness 0-1 (`LumpLook::Fruit`'s strength).
    pub fn fruit_looks(&self) -> Option<(usize, (f32, f32), f32)> {
        self.fruits.live.map(|f| (f.lump, f.centre, f.ripe))
    }
}
