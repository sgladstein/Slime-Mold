//! **The slime knows only what it can smell** (2026-11-28, the scent
//! thread's experiment; the owner, 11-28 08:31Z: "I think it could be fun
//! if 'the slime know only what it can smell' ... Lets test it out
//! though", built off primary play; design `Reports/scent-review-2026-11-28/`).
//!
//! Today the slime's aims are map-wide: a raid goes for the pump drawing
//! most wherever it stands, the hunt's trail starts at every soldier, a
//! wave gathers round the thickest knot of soldiers and a mini-boss walks
//! at the nearest soldier, none of them asking whether the slime could
//! smell any of it. Here a soldier or a building is *smelled* while it
//! stands within `Rules::know_reach` dish cells of walking from the
//! slime's body (the walking distance from every dish cell holding
//! `BODY` walkers or more, round rock, walls, the fort and deep water as
//! the raids' ways go), or within the guns' reach and the panic margin of
//! it as the crow flies (what shoots it, it smells: without this a squad
//! across a river would shoot the slime unseen). It is *known* for
//! `Rules::know_memory` game seconds after it was last smelled, and the
//! aims take only what is known (`smelt_soldier`, `smelt_building`).
//! A masked soldier (the toolkit's mask, smell 0) is never smelled.
//!
//! Left as they were: the Lure (a smell made to be followed, with its own
//! field), the crystals' far smell, the charges (`time_unit_paths`: a
//! charge starts only within the guns' reach and the panic margin of a
//! unit, which the slime smells by air anyway), and every check that asks
//! where the guns are so as to keep out of them (that is the slime
//! feeling pain, which it does wherever it is).
//!
//! The charges' field was gated too at first, and on planet 2 run seed 1
//! (llvmpipe) a turtle that won at 278 s lost at 127 s with it: the
//! gated buildings were those still going up, which no reach smelled,
//! so the charges that wore them went for the soldiers building them
//! (the other gates one at a time left the game as it was to 140 s).
//!
//! Stage 0 of the design: no new field on the GPU and nothing drawn. The
//! reviews (11-28) asked to test the knowledge first, keeping every tuned
//! behaviour, and to build a prey smell the player can see only if this
//! survives the bots. Built off the frame and read a game second later,
//! as the hunt's trail, so the game is the same however fast the thread.

use super::raids::{round_walk_from, Way};
use super::*;
use crate::mould::big::RAID_BLOCK;

/// Walkers a dish cell holds to count as the slime's body, not a stray.
const BODY: u32 = 2;

/// What the slime has smelled, and its walking distance being built.
#[derive(Default)]
pub(super) struct Know {
    /// The walking distance from the body started a second ago.
    way: Option<Way>,
    /// The game second each soldier was last smelled (by index).
    soldier: Vec<Option<u64>>,
    /// The game second each building was last smelled (by index).
    building: Vec<Option<u64>>,
    /// For the log (`know_line`), summed over the game's seconds with the
    /// gate on: soldiers smelled, soldiers alive, buildings smelled,
    /// buildings standing.
    sums: [u64; 4],
}

impl<W: World> Game<W> {
    /// Whether the gate is on (`Rules::know_reach` > 0; below 0 the
    /// placebo, `time_know`).
    pub(super) fn know_on(&self) -> bool {
        self.rules.know_reach > 0.0
    }

    fn know_now(&self) -> u64 {
        self.world.ticks() / self.rules.tps.max(1) as u64
    }

    fn known(&self, at: Option<u64>) -> bool {
        let memory = self.rules.know_memory.max(0.0).round() as u64;
        at.is_some_and(|t| self.know_now().saturating_sub(t) <= memory)
    }

    /// Soldier `k` is alive and the slime knows of it (always, with the
    /// gate off).
    pub(super) fn smelt_soldier(&self, k: usize) -> bool {
        self.soldiers.get(k).is_some_and(|s| s.alive) && (!self.know_on() || self.known(self.know.soldier.get(k).copied().flatten()))
    }

    /// The slime knows of building `k` (always, with the gate off). A Lure
    /// is always known: it is a smell made to be followed.
    pub(super) fn smelt_building(&self, k: usize) -> bool {
        !self.know_on() || self.buildings.get(k).is_some_and(|b| b.kind == Kind::Lure) || self.known(self.know.building.get(k).copied().flatten())
    }

    /// For a census line (the league's `know=`): soldier-seconds smelled
    /// and alive, building-seconds smelled and standing (all 0 with the
    /// gate off), then the raids sent by aim (building, lure, soldiers,
    /// last stand) and the waves sent, `S/A/B/U:b/l/s/x:w`.
    pub fn know_line(&self) -> String {
        let s = &self.know.sums;
        let mut aims = [0u32; 4];
        for d in &self.raids.sent {
            let i = if d.last_stand {
                3
            } else {
                match d.aim {
                    super::RaidAim::Building(_) => 0,
                    super::RaidAim::Lure { .. } => 1,
                    super::RaidAim::Soldiers(_) => 2,
                }
            };
            aims[i] += 1;
        }
        format!("{}/{}/{}/{}:{}/{}/{}/{}:{}", s[0], s[1], s[2], s[3], aims[0], aims[1], aims[2], aims[3], self.waves.sent)
    }

    /// Once a game second, before the aims: what the slime smells by the
    /// walking distance started a second ago and by the crow's reach of
    /// the guns now, then a new walking distance from its body.
    ///
    /// A negative reach is the placebo: the same reading at its size, and
    /// nothing gated, so a census can tell the gate's effect from the
    /// reading's own (a game is chaotic: one more grid read flips seeds).
    pub(super) fn time_know(&mut self) {
        if self.rules.know_reach == 0.0 {
            if self.know.way.is_some() || !self.know.soldier.is_empty() {
                self.know = Know::default();
            }
            return;
        }
        let now = self.know_now();
        let sc = self.world.scale();
        let (w, h) = self.world.size();
        let bin = (sc as usize).max(1);
        let grid = self.world.mould_grid(bin);
        let reach = (self.rules.know_reach.abs() * sc) as u32;
        let field = self.know.way.take().map(Way::finish).unwrap_or_default();
        let bw = w.div_ceil(RAID_BLOCK);
        let walk = |p: (f32, f32)| {
            let (x, y) = ((p.0.max(0.0) as usize).min(w - 1) / RAID_BLOCK, (p.1.max(0.0) as usize).min(h - 1) / RAID_BLOCK);
            field.get(y * bw + x).is_some_and(|&d| d <= reach)
        };
        // As the crow flies, the guns' reach and the panic margin: a scan
        // of the grid's squares in the box round the point.
        let air = (self.gun_range_max() + self.rules.panic_margin.max(0.0)) * sc;
        let (gw, gh) = (w.div_ceil(bin), h.div_ceil(bin));
        let near = |p: (f32, f32)| {
            let Some(g) = grid.as_ref().filter(|g| g.len() >= gw * gh) else { return false };
            let b = bin as f32;
            let r = (air / b).ceil() as i64;
            let (cx, cy) = ((p.0 / b) as i64, (p.1 / b) as i64);
            for y in (cy - r).max(0)..=(cy + r).min(gh as i64 - 1) {
                for x in (cx - r).max(0)..=(cx + r).min(gw as i64 - 1) {
                    if g[y as usize * gw + x as usize] < BODY {
                        continue;
                    }
                    if ((x as f32 + 0.5) * b - p.0).hypot((y as f32 + 0.5) * b - p.1) <= air {
                        return true;
                    }
                }
            }
            false
        };
        let smells = |p: (f32, f32)| walk(p) || near(p);
        self.know.soldier.resize(self.soldiers.len(), None);
        self.know.building.resize(self.buildings.len(), None);
        let mut sums = [0u64; 4];
        for (k, s) in self.soldiers.iter().enumerate() {
            sums[1] += s.alive as u64;
            if s.alive && s.smell > 0.0 && smells((s.x, s.y)) {
                self.know.soldier[k] = Some(now);
                sums[0] += 1;
            }
        }
        // A building still going up smells as one standing: the soldiers
        // building it are there.
        for (k, b) in self.buildings.iter().enumerate() {
            sums[3] += b.alive as u64;
            if b.alive && smells(b.at) {
                self.know.building[k] = Some(now);
                sums[2] += 1;
            }
        }
        for (s, n) in self.know.sums.iter_mut().zip(sums) {
            *s += n;
        }
        // The new walking distance, from the body's squares.
        let Some(g) = grid else { return };
        if g.len() < gw * gh {
            return;
        }
        let from: Vec<(u32, u32)> = g
            .iter()
            .enumerate()
            .filter(|&(_, &n)| n >= BODY)
            .map(|(i, _)| {
                let (x, y) = ((i % gw) * bin + bin / 2, (i / gw) * bin + bin / 2);
                ((x.min(w - 1) as u32) | ((y.min(h - 1) as u32) << 16), 0)
            })
            .collect();
        if from.is_empty() {
            return;
        }
        let Some((agar, aw, ah)) = self.slime_ground() else { return };
        self.know.way = Some(Way::start_with(move || round_walk_from(&agar, aw, ah, &from, reach)));
    }
}
