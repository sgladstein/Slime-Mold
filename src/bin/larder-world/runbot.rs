//! **The run bot** (the Engine and tools lane's job 4, the plan of
//! 2026-11-17; Milestone 1's exit test 1): `runbot=SEED[:TO]` plays a run
//! from planet 1 with the `bot` hand (`hands::bot_hand`), with no window,
//! to a loss or to the end of planet TO (3 by default), shopping between
//! planets (`shop`). It prints a line a planet and a last line for the
//! run, which CI reads.
//!
//! Why its own loop and not the film's (`headless`): a film draws a
//! picture at each `at=` time and stops at the round's verdict; the bot
//! needs neither pictures nor a stop, and it carries the run from one
//! planet to the next as the end screen's carry-on does (`App::carry_on`),
//! with its own shop in place of the shop page. The game's steps are the
//! window's own (`App::step`, then `App::settle` so each step is whole),
//! so what the bot plays is what the window plays.

use super::*;

/// One planet as the bot played it.
#[derive(Clone, Debug)]
pub(super) struct Planet {
    pub depth: u32,
    /// 0 for a loss, else the stars.
    pub stars: u8,
    /// Game seconds from landing to the verdict.
    pub secs: f32,
    pub why: String,
    pub soldiers: usize,
    pub mould: usize,
    pub killed: u32,
    /// What the shop after it sold the bot.
    pub bought: Vec<String>,
}

impl Planet {
    pub fn line(&self) -> String {
        let s = self.secs.round() as u32;
        let how = if self.stars > 0 { format!("won, {} star{}", self.stars, if self.stars == 1 { "" } else { "s" }) } else { "lost".into() };
        let shop = if self.bought.is_empty() { String::new() } else { format!("; bought {}", self.bought.join(", ")) };
        format!("planet {}: {how} at {}:{:02} ({}); soldiers {}, slime {}, killed {}{shop}", self.depth, s / 60, s % 60, self.why, self.soldiers, self.mould, self.killed)
    }
}

/// The shop's items the bot buys, in this order, one level of each per
/// pass while points last: what makes the squad kill faster and live
/// longer first, as the bot's plan is to win by fighting.
const WANTS: [&str; 7] = ["strike", "rate", "hp", "squad", "harvest", "range", "bank"];

/// Spends the run's points (`Run::pick`), returning what it bought:
/// first the card the planet ahead asks for (`run::bring`, the planet
/// card's advice: Tinder for planet 2's Sprawler), then `WANTS`.
pub(super) fn shop(r: &mut Run) -> Vec<String> {
    let mut bought = Vec::new();
    if let Some(c) = slime_mould::mould::run::bring(r.level().tier) {
        if let Some(i) = r.offers.iter().position(|&o| o == c) {
            if r.pick(slime_mould::mould::run::Pick::Offer(i)) {
                bought.push(format!("{c:?} (the gun now fires {})", r.gun_text()));
            }
        }
    }
    loop {
        let mut any = false;
        for key in WANTS {
            let Some(i) = slime_mould::mould::run::ITEMS.iter().position(|it| it.key == key) else { continue };
            if r.pick(slime_mould::mould::run::Pick::Item(i)) {
                bought.push(slime_mould::mould::run::ITEMS[i].name.to_string());
                any = true;
            }
        }
        if !any {
            return bought;
        }
    }
}

/// Plays run `seed` from planet 1 to a loss or the end of planet `to`.
/// `base` holds everything but the run (a test's scale, keys given).
/// Each planet is given at most its clock and a minute more of game time,
/// so a round that never reaches a verdict (a soft lock) shows as one.
pub(super) fn run_bot(base: Args, seed: u64, to: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Vec<Planet>, String> {
    let mut r = Run::new(seed);
    r.restock();
    let mut out = Vec::new();
    loop {
        let mut args = base.clone();
        let l = r.level();
        args.seed = l.seed;
        args.walkers = run_walkers(l.walkers);
        args.run = Some(r.clone());
        args.squads = true;
        // As the menu's Run gives a level (`run_args`): no sandbox map.
        args.map = None;
        args.mission = None;
        let mut app = offscreen(args, (320, 200), device, queue)?;
        let g = app.game.as_ref().ok_or("no game")?;
        let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        let limit = g.rules.cap + 60.0;
        let mut last_sec = u64::MAX;
        let t0 = std::time::Instant::now();
        loop {
            // A game that ended in the last step gets the mission's verdict
            // (the window's next step would do the same).
            app.follow_mission();
            let verdict = app.mission.as_ref().and_then(|m| m.verdict);
            let g = app.game.as_mut().unwrap();
            if verdict.is_some() || g.end.is_some() || g.time() - g.landed as f32 / g.rules.tps as f32 > limit {
                break;
            }
            let sec = g.world.ticks() / g.rules.tps as u64;
            if sec != last_sec {
                last_sec = sec;
                hands::bot_hand(g);
            }
            app.step(step);
            app.settle();
        }
        let g = app.game.as_ref().unwrap();
        let verdict = app.mission.as_ref().and_then(|m| m.verdict);
        let landed = g.landed as f32 / g.rules.tps as f32;
        let mut p = Planet {
            depth: r.depth,
            stars: verdict.map_or(0, |v| v.stars),
            secs: verdict.map_or(g.time(), |v| v.at) - landed,
            why: match (verdict, g.end) {
                (Some(_), _) => app.mission.as_ref().unwrap().cause_line(),
                (None, Some(e)) => format!("the game ended: {}", e.band()),
                (None, None) => "no verdict by the clock and a minute: a soft lock".into(),
            },
            soldiers: g.alive(),
            mould: g.mould,
            killed: g.killed,
            bought: Vec::new(),
        };
        eprintln!("runbot: planet {} took {:.0} s to play", r.depth, t0.elapsed().as_secs_f32());
        if p.stars == 0 {
            r.lost();
            println!("runbot {}", p.line());
            out.push(p);
            break;
        }
        r.won(p.stars);
        if r.over || r.depth > to {
            println!("runbot {}", p.line());
            out.push(p);
            break;
        }
        p.bought = shop(&mut r);
        println!("runbot {}", p.line());
        out.push(p);
    }
    let won = out.iter().take_while(|p| p.stars > 0).count();
    println!("runbot: run {seed}, {won} of {} planets won", out.len().max(to as usize));
    Ok(out)
}
