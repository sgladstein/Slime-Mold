//! **The strategy league** (2026-11-19, the thread "Build a smarter bot
//! that explores strategies"): plays the strategy bot (`strat.rs`) with
//! one plan, with no window, and prints one `league` line a planet of
//! `key=value` pairs a census reads (`scripts/league.py` runs many).
//!
//! - `league=DEPTH:SEED[:SECS] plan=PLAN`: run SEED's planet DEPTH. The
//!   planets before it are taken as won with two stars each and their
//!   points spent by the plan's shop (`shop`), so every plan meets the
//!   planet with what its own shop would have bought; the gun lab
//!   (`runbot::gun_lab`, the gun-building thread) lands the same way with
//!   the run bot's shop. SECS caps the game time (0: the planet's clock
//!   and a minute, as the run bot).
//! - `league=run:SEED[:TO[:SECS]] plan=PLAN`: a whole run from planet 1 to
//!   TO (3) or a loss, shopping by the plan between planets.
//!
//! A plan that starts `util:` is the utility bot's weights (`utility.rs`)
//! with its own `shop` and `cards`; any other is the strategy bot's.
//!
//! The game's steps are the window's (`App::step`, then `App::settle`), as
//! the run bot's, so what the league plays is what the window plays.

use super::strat::{self, Plan};
use super::*;
use slime_mould::mould::run::{self, Pick, ITEMS};

/// The unlocks a plan may build with, as `Run::has` keys.
const UNLOCKS: [&str; 7] = ["walls", "turret", "mortar", "blast", "mine", "cord", "gas"];

/// Spends the run's points by the plan: the cards it wants (`want`), the
/// card the planet ahead asks for (`cards` 1 or 2), then the plan's items,
/// one level of each a pass, then (`cards` 2) any card on offer that fits
/// the gun. Returns what it bought.
pub(super) fn shop(r: &mut Run, p: &Plan) -> Vec<String> {
    let mut bought = Vec::new();
    // Cards the plan wants (`Plan::want`), set on the shelf as if offered
    // and bought while the points reach, before anything else.
    for name in &p.want {
        let Some(c) = want_card(name, r.depth) else { continue };
        if r.cards.iter().any(|&(x, _)| card_key(x) == card_key(c)) {
            continue;
        }
        r.offers.push(c);
        let i = r.offers.len() - 1;
        if r.pick(Pick::Offer(i)) {
            bought.push(card_key(c));
        } else {
            r.offers.pop();
        }
    }
    if p.cards >= 1 {
        if let Some(c) = run::bring(r.level().tier) {
            if let Some(i) = r.offers.iter().position(|&o| o == c) {
                if r.pick(Pick::Offer(i)) {
                    bought.push(card_key(c));
                }
            }
        }
    }
    loop {
        let mut any = false;
        for key in &p.shop {
            let Some(i) = ITEMS.iter().position(|it| it.key == key) else { continue };
            if r.pick(Pick::Item(i)) {
                bought.push(ITEMS[i].key.to_string());
                any = true;
            }
        }
        if !any {
            break;
        }
    }
    if p.cards >= 2 {
        let mut i = 0;
        while i < r.offers.len() {
            let c = r.offers[i];
            if run::card_price(c) <= r.points && r.fits(c) && r.pick(Pick::Offer(i)) {
                bought.push(card_key(c));
                continue;
            }
            i += 1;
        }
    }
    bought
}

/// A wanted card by name; a payload may name its set after a dot
/// (`blast.needle`): that shell's bullet as a shop at `depth` rolls it on
/// average. The gun lab's `runbot::lab_deck` (on the gun-building thread's
/// branch, 11-19) reads its decks the same way; the gun lab found Needle
/// Blast the best single card for damage, +9k to +14k slime against plain
/// bullets on planets 1, 3 and 4. One of the two goes once both land.
fn want_card(w: &str, depth: u32) -> Option<slime_mould::mould::rounds::Card> {
    use slime_mould::mould::bullets::{Ammo, Set, Shell, COST_POWER, DAMAGE, DEPTH_DAMAGE, DEPTH_DAMAGE_MAX};
    use slime_mould::mould::rounds::Card;
    let Some((shell, set)) = w.split_once('.') else { return Card::from_name(w) };
    let shell = Shell::ALL.into_iter().find(|p| p.key() == shell)?;
    let set = Set::parse(set)?;
    let l = set.lean();
    let h = |x: f32| (x * 100.0).round() as u16;
    let d = (1.0 + DEPTH_DAMAGE * (depth.max(1) - 1) as f32).min(DEPTH_DAMAGE_MAX);
    let mut a = Ammo { shell, set, depth: depth.clamp(1, 255) as u8, damage: (DAMAGE * l.damage * d * 10.0).round() as u16, speed: h(l.speed), range: h(l.range), spread: h(l.spread), cost: 0 };
    a.cost = (shell.base() as f32 * a.grade().powf(COST_POWER)).round().clamp(1.0, 255.0) as u8;
    Some(Card::Ammo(a))
}

/// A card as one word for the league's line: its name, or a bullet's
/// shell and set (`blast.needle`), as `want` takes them.
fn card_key(c: slime_mould::mould::rounds::Card) -> String {
    match c {
        slime_mould::mould::rounds::Card::Ammo(a) => format!("{}.{}", a.shell.key(), a.set.key()),
        c => format!("{c:?}"),
    }
}

/// The hand that plays plan `spec` on this run, with the unlocks the run
/// owns (`own`): `util:...` as given, else `strat:...`.
fn hand_for(spec: &str, r: &Run) -> String {
    let own: Vec<&str> = UNLOCKS.into_iter().filter(|k| r.has(k)).collect();
    let hand = if spec.starts_with("util:") { spec.to_string() } else { format!("strat:{spec}") };
    format!("{hand},own={}", own.join("/"))
}

/// The shop policy of plan `spec` (`util:` weights carry one too).
fn shop_plan(spec: &str) -> Result<Plan, String> {
    match spec.strip_prefix("util:") {
        Some(w) => Ok(super::utility::Weights::parse(w)?.plan),
        None => Plan::parse(spec),
    }
}

/// One planet played: the `league` line's numbers.
struct Played {
    how: &'static str,
    stars: u8,
    secs: f32,
    line: String,
}

/// Plays the run's current planet with plan `spec` to its verdict or
/// `secs` game seconds (0: its clock and a minute).
fn play(base: &Args, r: &Run, spec: &str, secs: f32, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Played, String> {
    let l = r.level();
    let mut args = base.clone();
    args.seed = l.seed;
    args.walkers = run_walkers(l.walkers);
    args.run = Some(r.clone());
    args.squads = true;
    args.map = None;
    args.mission = None;
    let hand = hand_for(spec, r);
    let pl = shop_plan(spec)?;
    let (aim_hz, kite, mop) = (pl.aim, pl.kite, pl.mop);
    // Between turns: at the aim rate, or five times a second to kite.
    let tick_hz = if kite > 0.0 { aim_hz.max(5.0) } else { aim_hz };
    let mut app = offscreen(args, (320, 200), device, queue)?;
    let g = app.game.as_ref().ok_or("no game")?;
    let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let limit = if secs > 0.0 { secs } else { g.rules.cap + 60.0 };
    let start_mould = g.mould;
    let mut last_sec = u64::MAX;
    let mut marks: Vec<String> = Vec::new();
    let (mut peak_mould, mut min_alive) = (start_mould, usize::MAX);
    let mut at = [None::<(u32, usize, usize)>; 3];
    let t0 = std::time::Instant::now();
    let mut next_aim = 0u64;
    loop {
        app.follow_mission();
        let verdict = app.mission.as_ref().and_then(|m| m.verdict);
        let g = app.game.as_mut().unwrap();
        let since = g.time() - g.landed as f32 / g.rules.tps as f32;
        if verdict.is_some() || g.end.is_some() || since > limit {
            break;
        }
        let sec = g.world.ticks() / g.rules.tps as u64;
        if sec != last_sec {
            last_sec = sec;
            hands::plan(g, &hand);
            peak_mould = peak_mould.max(g.mould);
            if since > 5.0 {
                min_alive = min_alive.min(g.alive());
            }
            // Kills, slime and soldiers at 2:30, 5:00 and 10:00 (the
            // owner's planet 3 at 2:30: 10,613 kills, 4 to 12 soldiers).
            for (k, s) in [150.0, 300.0, 600.0].into_iter().enumerate() {
                if at[k].is_none() && since >= s {
                    at[k] = Some((g.killed, g.mould, g.alive()));
                }
            }
            if sec.is_multiple_of(30) {
                marks.push(format!("{}:{}/{}/{}/{:.0}", sec, g.killed, g.mould, g.alive(), g.bank));
            }
        } else if tick_hz > 1.0 && g.world.ticks() >= next_aim {
            // Between turns the volley follows the slime (`Plan::aim`) and
            // the squad keeps its distance (`Plan::kite`).
            next_aim = g.world.ticks() + (g.rules.tps as f32 / tick_hz).round().max(1.0) as u64;
            strat::between(g, aim_hz > 1.0, kite, mop);
        }
        app.step(step);
        app.settle();
    }
    let g = app.game.as_ref().unwrap();
    let verdict = app.mission.as_ref().and_then(|m| m.verdict);
    let landed = g.landed as f32 / g.rules.tps as f32;
    let stars = verdict.map_or(0, |v| v.stars);
    let end = verdict.map_or(g.time(), |v| v.at) - landed;
    let how = match (verdict, g.end) {
        (Some(v), _) if v.stars > 0 => "won",
        (Some(_), _) | (None, Some(_)) => "lost",
        (None, None) => "open",
    };
    let count = |k: fn(&Kind) -> bool| g.buildings.iter().filter(|b| b.alive && b.up() && k(&b.kind)).count();
    let (bought, peak_alive, secs_in) = if spec.starts_with("util:") { super::utility::tally() } else { strat::tally() };
    let mark = |m: Option<(u32, usize, usize)>| m.map_or("-".to_string(), |(k, s, a)| format!("{k}/{s}/{a}"));
    let line = format!(
        "depth={} seed={} planet_seed={} tier={} how={how} won={} stars={stars} secs={end:.1} killed={} turret_killed={} mortar_killed={} mould={} start_mould={start_mould} peak_mould={peak_mould} alive={} peak_alive={peak_alive} min_alive={} bought={bought} bank={:.0} income={:.0} pumped={:.0} harvested={:.0} lost_buildings={} pumps={} turrets={} walls={} lures={} t_home={} t_out={} t_build={} t_back={} at150={} at300={} at600={} wall={:.0} marks={} rest={}",
        r.depth,
        r.seed,
        l.seed,
        l.tier.name(),
        u8::from(how == "won"),
        g.killed,
        g.turret_killed,
        g.mortar_killed,
        g.mould,
        g.alive(),
        if min_alive == usize::MAX { 0 } else { min_alive },
        g.bank,
        g.income(),
        g.pumped,
        g.harvested,
        g.lost_buildings,
        count(|k| *k == Kind::Pump),
        count(|k| *k == Kind::Turret),
        count(|k| matches!(k, Kind::Wall { .. })),
        count(|k| *k == Kind::Lure),
        secs_in[0],
        secs_in[1],
        secs_in[2],
        secs_in[3],
        mark(at[0]),
        mark(at[1]),
        mark(at[2]),
        t0.elapsed().as_secs_f32(),
        marks.join(","),
        // Where the last walkers were, when a game ends with a few left.
        if how != "won" && g.mould <= 50 { strat::remnant(g) } else { "-".to_string() }
    );
    Ok(Played { how, stars, secs: end, line })
}

/// `league=DEPTH:SEED[:SECS]` (see the module's docs).
pub(super) fn planet(base: Args, depth: u32, seed: u64, secs: f32, spec: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), String> {
    let p = shop_plan(spec)?;
    let mut r = Run::new(seed);
    r.restock();
    let mut bought = Vec::new();
    while r.depth < depth {
        r.won(2);
        bought.extend(shop(&mut r, &p));
    }
    let out = play(&base, &r, spec, secs, device, queue)?;
    println!("league plan={spec} {} shop={} gun={}", out.line, bought.join("/"), r.gun_text());
    Ok(())
}

/// `league=run:SEED[:TO[:SECS]]` (see the module's docs).
pub(super) fn whole_run(base: Args, seed: u64, to: u32, secs: f32, spec: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), String> {
    let p = shop_plan(spec)?;
    let mut r = Run::new(seed);
    r.restock();
    let mut won = 0;
    let mut total = 0.0;
    loop {
        let out = play(&base, &r, spec, secs, device, queue)?;
        total += out.secs;
        println!("league plan={spec} run=1 {} gun={}", out.line, r.gun_text());
        if out.how != "won" {
            r.lost();
            break;
        }
        won += 1;
        r.won(out.stars.max(1));
        if r.over || r.depth > to {
            break;
        }
        let b = shop(&mut r, &p);
        println!("league plan={spec} shop depth={} bought={}", r.depth, b.join("/"));
    }
    println!("league plan={spec} run={seed} won={won} to={to} secs={total:.0}");
    Ok(())
}

/// Parses `league=`'s value: `DEPTH:SEED[:SECS]` or `run:SEED[:TO[:SECS]]`.
pub(super) enum Job {
    Planet { depth: u32, seed: u64, secs: f32 },
    Run { seed: u64, to: u32, secs: f32 },
}

pub(super) fn parse(v: &str) -> Result<Job, String> {
    let w: Vec<&str> = v.split(':').collect();
    let num = |s: &str| s.parse::<f32>().map_err(|e| format!("league={v}: {e}"));
    let seed = |s: &str| s.parse::<u64>().map_err(|e| format!("league={v}: {e}"));
    match w.as_slice() {
        ["run", s, rest @ ..] => Ok(Job::Run {
            seed: seed(s)?,
            to: rest.first().map_or(Ok(3.0), |x| num(x))?.clamp(1.0, run::PLANETS as f32) as u32,
            secs: rest.get(1).map_or(Ok(0.0), |x| num(x))?,
        }),
        [d, s, rest @ ..] => Ok(Job::Planet {
            depth: (num(d)? as u32).clamp(1, run::PLANETS),
            seed: seed(s)?,
            secs: rest.first().map_or(Ok(0.0), |x| num(x))?,
        }),
        _ => Err(format!("league={v}: DEPTH:SEED[:SECS] or run:SEED[:TO[:SECS]]")),
    }
}
