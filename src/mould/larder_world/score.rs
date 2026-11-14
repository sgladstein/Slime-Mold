//! The score, the take share, playing a hand and replays. Moved out of
//! `larder_world.rs` unchanged (C9, 2026-11-14).

use super::*;

impl<W: World> Game<W> {
    /// **Share of the crystals held**, the HUD's and W2b's score's: of the
    /// crystals with charge left that either side holds, the fraction the
    /// squad holds. The squad holds one with a soldier on it and no mould
    /// touching that soldier; the mould holds the others it has eaten from
    /// (charge down by more than 1% of its start beyond what the squad
    /// took) with no soldier on it. One half when neither holds any. (The
    /// first version gave the mould every crystal without a soldier,
    /// reached or not, so an idle squad scored as if overrun from the first
    /// second.)
    pub fn share(&self) -> f32 {
        let (mut ours, mut theirs) = (0, 0);
        for (i, (c, charge)) in self.world.crystals().into_iter().enumerate() {
            if charge <= 0.0 {
                continue;
            }
            let cr = self.world.crystal_r(i);
            let eaten = self.start_charge[i] - charge - self.taken[i] > 0.01 * self.start_charge[i];
            let on: Vec<&Soldier> = self.soldiers.iter().filter(|s| s.alive && (s.x - c.0).hypot(s.y - c.1) <= cr + 2.0 * self.s()).collect();
            // Presence: a crystal is held while no walker at all is on its soldiers.
            if !on.is_empty() && on.iter().all(|s| s.touching == 0) {
                ours += 1;
            } else if on.is_empty() && eaten {
                theirs += 1;
            }
        }
        if ours + theirs == 0 {
            0.5
        } else {
            ours as f32 / (ours + theirs) as f32
        }
    }

    /// **Share of the crystals' charge taken** (score v2's stalemate): of
    /// the charge gone from the crystals, the fraction the squad took, less
    /// the food the mould stole from the larder (which counts as the
    /// mould's). One half when nothing is gone. A sum over the whole game,
    /// so it rewards holding early (the AI thread's census, 2026-11-12: read
    /// at the cap, a style that grabbed crystals in the last 90 s tied one
    /// that held from 10:00, 7-7) and gives nothing for guarding a crystal
    /// with crumbs left (WM1: every rule that brought a carrier home from a
    /// dry crystal cost the planned hands under `share`, which counts a
    /// crystal with 0.8 charge left as one held). Weighing `share` by the
    /// charge left was tried first and put farm below idle on race seed 5
    /// (-0.265 against -0.232): harvesting empties the crystals you hold.
    pub fn take_share(&self) -> f32 {
        let cs = self.world.crystals();
        let ours: f64 = self.taken.iter().map(|&t| t as f64).sum::<f64>();
        let gone: f64 = cs.iter().enumerate().map(|(i, c)| (self.start_charge[i] - c.1).max(0.0) as f64).sum();
        let mould = (gone - ours).max(0.0);
        let stolen = (self.stolen * self.rules.charge) as f64;
        let (ours, mould) = ((ours - stolen).max(0.0), mould + stolen);
        if ours + mould <= 0.0 {
            0.5
        } else {
            (ours / (ours + mould)) as f32
        }
    }

    /// **One ordered number a round** (score v2, 2026-11-12). Wins and losses
    /// are `End::score`'s bands. A stalemate at the cap, in (-0.5, 0.5), is
    /// half the charge share (`take_share`), less a half, plus a size
    /// term, `0.25 (1 - m) / (1 + m)` with `m` the walkers left over the
    /// walkers at the start: +0.25 for a mould cut to nothing, 0 for one
    /// the size it started, toward -0.25 as it grows. (W2b's stalemate was
    /// the share at the cap alone, so a strike that took the core and left
    /// 7 to 574 walkers scored as a farm that left 9,000: the AI thread's
    /// census.)
    pub fn score(&self) -> Option<f32> {
        self.end.map(|e| match e {
            End::Cap(took) => {
                // A ratio of whole walkers to the start's, so the same at any mass.
                let m = self.mould as f32 / self.start_mould.max(1) as f32;
                0.5 * (took - 0.5) + 0.25 * (1.0 - m) / (1.0 + m)
            }
            _ => e.score(self.alive(), self.rules.cap),
        })
    }

    /// W2b's score (the crystals held at the cap, no size term), so a
    /// census can still be paired with W2b's and the AI's earlier ones.
    pub fn score_w2b(&self) -> Option<f32> {
        self.end.map(|e| match e {
            End::Cap(_) => End::Cap(self.cap_held.unwrap_or(0.5)).score(self.alive(), self.rules.cap),
            _ => e.score(self.alive(), self.rules.cap),
        })
    }

    /// Run to the end, with `plan` called once a game second before the
    /// frame.
    pub fn play(&mut self, mut plan: impl FnMut(&mut Game<W>)) -> End {
        let tps = self.rules.tps as u64;
        let mut last = u64::MAX;
        while self.end.is_none() {
            let sec = self.world.ticks() / tps;
            if sec != last {
                plan(self);
                last = sec;
            }
            self.step();
        }
        self.end.expect("ended")
    }

    /// The log as a replay file: a header with the seed and the end tick,
    /// then one command a line after its tick.
    pub fn replay_text(&self, seed: u64) -> String {
        let mut out = format!("larderworld replay seed {seed} end {}\n", self.world.ticks());
        for (t, c) in &self.log {
            out += &format!("{t} {}\n", c.to_line());
        }
        out
    }

    /// Play a replay's commands on `game` (fresh, same seed and rules): the
    /// same world reaches the same hash. Returns false on a bad file.
    pub fn replay(game: &mut Game<W>, text: &str) -> bool {
        let mut lines = text.lines();
        let Some(head) = lines.next() else { return false };
        let head: Vec<&str> = head.split_whitespace().collect();
        let Some(end) = head.get(5).and_then(|v| v.parse::<u64>().ok()) else { return false };
        let mut cmds = Vec::new();
        for l in lines {
            let Some((t, rest)) = l.split_once(' ') else { return false };
            let (Ok(t), Some(c)) = (t.parse::<u64>(), Cmd::parse(rest)) else { return false };
            cmds.push((t, c));
        }
        let mut next = 0;
        while game.world.ticks() < end && game.end.is_none() {
            while next < cmds.len() && cmds[next].0 == game.world.ticks() {
                game.apply(cmds[next].1);
                next += 1;
            }
            game.step();
        }
        while next < cmds.len() && cmds[next].0 == game.world.ticks() {
            game.apply(cmds[next].1);
            next += 1;
        }
        true
    }
}
