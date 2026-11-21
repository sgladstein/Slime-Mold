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
//! **A plan by planet** (2026-11-20): plans joined by `|` are played one a
//! planet, the last for every planet after (`stage`), and the shop before
//! a planet spends by that planet's plan. On today's game the plans that
//! win planet 3 (build at home first) won 9 of 66 planet 2 games and the
//! rush that wins planet 2 lost planet 3 more often, so a run needs both:
//! `rush,aim=6|rush,aim=6|econ,go=4,aim=6`.
//!
//! The game's steps are the window's (`App::step`, then `App::settle`), as
//! the run bot's, so what the league plays is what the window plays.

use super::strat::{self, Plan};
use super::*;
use slime_mould::mould::bullets::Shell;
use slime_mould::mould::guns::{Gun, GunKind};
use slime_mould::mould::rounds::Card;
use slime_mould::mould::run::{self, Pick, ITEMS};

/// The unlocks a plan may build with, as `Run::has` keys.
const UNLOCKS: [&str; 7] = ["walls", "turret", "mortar", "blast", "mine", "cord", "gas"];

/// Spends the run's points by the plan: the cards it wants (`want`), the
/// card the planet ahead asks for (`cards` 1 or 2), then the plan's items,
/// one level of each a pass, then (`cards` 2) any card on offer that fits
/// the gun. Returns what it bought.
pub(super) fn shop(r: &mut Run, p: &Plan) -> Vec<String> {
    let mut bought = Vec::new();
    // Cards the plan wants (`Plan::want`), in its order, before anything
    // else: set on the shelf as if offered (`shelf`, the default), or
    // bought only from what the run offers.
    buy_wants(r, &p.want, p.shelf, &mut bought);
    // A gun body (`Plan::gun`), then the holding groups' cards for it.
    if buy_gun(r, p, &mut bought) || (p.arm == 1 && r.edit(strat::GROUPS)) {
        buy_wants(r, &p.want2, p.shelf, &mut bought);
        r.edit(0);
    }
    // The turrets' cards (`Plan::tcards`), in their loadout: the turrets
    // first if the plan's items buy them, or the items would spend the
    // points before the cards had turrets to go on.
    if !p.tcards.is_empty() && !r.has("turret") && p.shop.iter().any(|k| k == "turret") {
        if let Some(i) = ITEMS.iter().position(|it| it.key == "turret") {
            if r.pick(Pick::Item(i)) {
                bought.push("turret".to_string());
            }
        }
    }
    if !p.tcards.is_empty() && r.edit_turrets(true) {
        buy_wants(r, &p.tcards, p.shelf, &mut bought);
        r.edit(0);
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
    reorder(r, &p.order);
    bought
}

/// The squads' gun's cards in the order named (`Plan::order`): each named
/// card in the gun moves to the next slot from the first.
fn reorder(r: &mut Run, order: &[String]) {
    r.edit(0);
    let mut slot = 0;
    for name in order {
        let Some(c) = want_card(name, r.depth) else { continue };
        let shell = (!name.contains('.')).then(|| Shell::ALL.into_iter().find(|s| s.card() == c)).flatten();
        let same = |o: Card| card_key(o) == card_key(c) || matches!((o, shell), (Card::Ammo(a), Some(s)) if a.shell == s);
        // The first such card in the gun past the slots already set.
        let mut seen = 0;
        let at = r.cards.iter().position(|&(x, on)| {
            seen += usize::from(on);
            on && seen > slot && same(x)
        });
        if at.is_some_and(|i| r.pick(Pick::Move(i, slot))) {
            slot += 1;
        }
    }
}

/// Buys the cards named in `want` into the loadout shown (see `shop`).
fn buy_wants(r: &mut Run, want: &[String], shelf: bool, bought: &mut Vec<String>) {
    for name in want {
        let Some(c) = want_card(name, r.depth) else { continue };
        // A payload named bare (`torch`) is any bullet of that shell, in
        // whatever set it rolled: off the shelf it takes the shop's own (the
        // shop before the Sprawler and the Trunk puts a Torch first,
        // `run::bring_shell`, and the planet's card says to bring one).
        let shell = (!name.contains('.')).then(|| Shell::ALL.into_iter().find(|s| s.card() == c)).flatten();
        let same = |o: Card| card_key(o) == card_key(c) || matches!((o, shell), (Card::Ammo(a), Some(s)) if a.shell == s);
        if r.cards.iter().any(|&(x, _)| same(x)) {
            continue;
        }
        if !shelf {
            if let Some(i) = r.offers.iter().position(|&o| same(o)) {
                let o = r.offers[i];
                if r.pick(Pick::Offer(i)) {
                    bought.push(card_key(o));
                }
            }
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
}

/// Buys the plan's gun (`Plan::gun`) once, into the main squad's loadout
/// (`arm=0`) or the holding groups' (`arm=1`, `strat::GROUPS`, made if
/// missing). With the shelf, a kind not on offer is rolled for this shop
/// as if offered (as `Run::restock` rolls one). Returns true if it bought
/// one for the groups, whose loadout is then shown (`shop` buys `want2`
/// into it and shows the first again).
fn buy_gun(r: &mut Run, p: &Plan, bought: &mut Vec<String>) -> bool {
    let Some(want) = p.gun.as_deref() else { return false };
    // Bought once: the run starts with one gun, and only this buys more.
    if r.guns.len() > 1 {
        return false;
    }
    let kind = GunKind::parse(want);
    if let Some(k) = kind.filter(|&k| p.shelf && !r.gun_offers.iter().any(|g| g.kind == k)) {
        r.gun_offers.push(Gun::roll_kind(k, r.seed ^ 0x5348_454C_4647, r.depth));
    }
    let Some(i) = (0..r.gun_offers.len()).filter(|&i| kind.is_none_or(|k| r.gun_offers[i].kind == k)).max_by(|&a, &b| r.gun_offers[a].grade.total_cmp(&r.gun_offers[b].grade)) else { return false };
    let g = r.gun_offers[i];
    if run::gun_price(&g) > r.points {
        return false;
    }
    let groups = p.arm == 1;
    if groups {
        while r.loadouts.len() <= strat::GROUPS && r.pick(Pick::NewLoadout) {}
        r.edit(strat::GROUPS);
    } else {
        r.edit(0);
    }
    if !r.pick(Pick::Gun(i)) {
        r.edit(0);
        return false;
    }
    bought.push(format!("gun.{}@{:.2}", g.kind.key(), g.grade));
    // Cards the new gun had no room for went to the bag; any that fit go
    // back in, in their order.
    let mut k = 0;
    while k < r.cards.len() {
        if !r.cards[k].1 && r.fits(r.cards[k].0) {
            r.pick(Pick::Card(k));
        }
        k += 1;
    }
    groups
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

/// One planet played: the `league` line's numbers. The window's planet 1
/// test (`the_window_plays_planet_1_with_the_run_bot`) reads it too.
pub(super) struct Played {
    pub how: &'static str,
    pub stars: u8,
    pub secs: f32,
    pub line: String,
    /// Shop points from starfall and boss cores (`Game::starfall`).
    pub found: u32,
}

/// Plays the run's current planet with plan `spec` to its verdict or
/// `secs` game seconds (0: its clock and a minute).
pub(super) fn play(base: &Args, r: &Run, spec: &str, secs: f32, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Played, String> {
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
    // The tools pass needs the run's unlocks (`own`), which the hand's
    // spec carries and the shop plan does not.
    let tool_plan = hand.strip_prefix("strat:").map(Plan::parse).transpose()?.filter(|p| p.mines > 0 || p.blast > 0);
    let (aim_hz, kite, mop, standoff) = (pl.aim, pl.kite, pl.mop, pl.standoff);
    // Between turns: at the aim rate, or five times a second to kite.
    let tick_hz = if kite > 0.0 { aim_hz.max(5.0) } else { aim_hz };
    // The holding groups' own gun, if the run has one (`Plan::arm`).
    strat::arm_groups(r);
    let mut app = offscreen(args, (320, 200), device, queue)?;
    let g = app.game.as_ref().ok_or("no game")?;
    let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let limit = if secs > 0.0 { secs } else { g.rules.cap + 60.0 };
    let start_mould = g.mould;
    let mut last_sec = u64::MAX;
    let mut marks: Vec<String> = Vec::new();
    let mut fell: Vec<String> = Vec::new();
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
            if let Some(tp) = &tool_plan {
                g.with_kit(|kit, g| strat::tools(g, kit, tp));
            }
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
                // Soldiers fallen so far (dead soldiers stay in the list), then
                // those the slime bit to death and those fire killed:
                // the 11-20 measure of how much a big army bleeds late in a
                // planet ("Keeping a big squad under threat"). Its own key,
                // so `marks`' four numbers keep their meaning for league.py.
                fell.push(format!("{}:{}/{}/{}", sec, g.soldiers.len() - g.alive(), g.fell_bitten, g.fell_burned));
            }
        } else if tick_hz > 1.0 && g.world.ticks() >= next_aim {
            // Between turns the volley follows the slime (`Plan::aim`) and
            // the squad keeps its distance (`Plan::kite`).
            next_aim = g.world.ticks() + (g.rules.tps as f32 / tick_hz).round().max(1.0) as u64;
            strat::between(g, aim_hz > 1.0, kite, mop, standoff);
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
    let switched = if spec.starts_with("util:") { [0, 0] } else { strat::switched() };
    let mark = |m: Option<(u32, usize, usize)>| m.map_or("-".to_string(), |(k, s, a)| format!("{k}/{s}/{a}"));
    // Blasts thrown, mines laid, and the walkers each killed (`Plan::blast`, `mines`).
    let used = if spec.starts_with("util:") { [0, 0] } else { strat::tools_used() };
    let kills = app.game.as_ref().map_or([0; 8], |g| g.kit.killed);
    let tools = format!("{}/{}/{}/{}", used[0], used[1], kills[0], kills[1]);
    let line = format!(
        "depth={} seed={} planet_seed={} tier={} how={how} won={} stars={stars} secs={end:.1} killed={} turret_killed={} mortar_killed={} mould={} start_mould={start_mould} peak_mould={peak_mould} alive={} peak_alive={peak_alive} min_alive={} bought={bought} bank={:.0} income={:.0} pumped={:.0} harvested={:.0} lost_buildings={} pumps={} turrets={} walls={} lures={} t_home={} t_out={} t_build={} t_back={} t_rush={} t_regroup={} t_burnt={} tools={tools} at150={} at300={} at600={} wall={:.0} marks={} fell={} fell_end={} bitten_end={} burned_end={} late={} stand={} rest={}",
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
        switched[0],
        switched[1],
        if spec.starts_with("util:") { 0 } else { strat::burnt() },
        mark(at[0]),
        mark(at[1]),
        mark(at[2]),
        t0.elapsed().as_secs_f32(),
        marks.join(","),
        fell.join(","),
        g.soldiers.len() - g.alive(),
        g.fell_bitten,
        g.fell_burned,
        u8::from(g.late),
        // The last-stand heart: the second it formed (0 none), its lunges,
        // soldiers bitten to death in its first 10 s and in the worst 10 s
        // from a lunge's start, and all it bit (`Stand`).
        format!("{}:{}:{}:{}:{}", g.stand.born_sec, g.stand.lunges, g.stand.first, g.stand.worst, g.stand.killed),
        // Where the last walkers were, when a game ends with a few left.
        if how != "won" && g.mould <= 50 { strat::remnant(g) } else { "-".to_string() }
    );
    let found = g.starfall.earned();
    Ok(Played { how, stars, secs: end, line, found })
}

/// Each loadout with a gun, for the league line: `LOADOUT:KIND@GRADE:CARDS`
/// joined by `,` (`0:long@1.07:wide+blast.needle`), then the turrets'
/// cards if any (`t:wide+rate`).
fn arms(r: &Run) -> String {
    let keys = |v: Vec<Card>| v.into_iter().map(card_key).collect::<Vec<_>>().join("+");
    let mut v: Vec<String> = (0..r.loadouts.len()).filter_map(|l| r.gun_of(l).map(|g| format!("{l}:{}@{:.2}:{}", g.kind.key(), g.grade, keys(r.cards_of(l))))).collect();
    let t = r.turret_cards();
    if !t.is_empty() {
        v.push(format!("t:{}", keys(t)));
    }
    v.join(",")
}

/// The plan for planet `depth` of a plan by planet (`a|b|c`): the
/// `depth`th, or the last for any planet after it.
fn stage(spec: &str, depth: u32) -> &str {
    let parts: Vec<&str> = spec.split('|').collect();
    parts[(depth.max(1) as usize - 1).min(parts.len() - 1)]
}

/// `league=DEPTH:SEED[:SECS]` (see the module's docs).
pub(super) fn planet(base: Args, depth: u32, seed: u64, secs: f32, spec: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), String> {
    let mut r = Run::new(seed);
    r.restock();
    // `Plan::points`: added before the first shop, which on planet 1 is a
    // shop of its own before the landing.
    let extra = shop_plan(stage(spec, depth))?.points;
    r.points += extra;
    let mut bought = Vec::new();
    if depth == 1 && extra > 0 {
        bought.extend(shop(&mut r, &shop_plan(stage(spec, 1))?));
    }
    while r.depth < depth {
        r.won(2);
        let p = shop_plan(stage(spec, r.depth))?;
        bought.extend(shop(&mut r, &p));
    }
    let out = play(&base, &r, stage(spec, depth), secs, device, queue)?;
    println!("league plan={spec} {} shop={} gun={} arms={}", out.line, bought.join("/"), r.gun_text(), arms(&r));
    Ok(())
}

/// `league=run:SEED[:TO[:SECS]]` (see the module's docs).
pub(super) fn whole_run(base: Args, seed: u64, to: u32, secs: f32, spec: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), String> {
    for part in spec.split('|') {
        shop_plan(part)?;
    }
    let mut r = Run::new(seed);
    r.restock();
    let mut won = 0;
    let mut total = 0.0;
    loop {
        let out = play(&base, &r, stage(spec, r.depth), secs, device, queue)?;
        total += out.secs;
        println!("league plan={spec} run=1 {} gun={} arms={}", out.line, r.gun_text(), arms(&r));
        if out.how != "won" {
            r.lost();
            break;
        }
        won += 1;
        r.found(out.found);
        r.won(out.stars.max(1));
        if r.over || r.depth > to {
            break;
        }
        let p = shop_plan(stage(spec, r.depth))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_payload_off_the_shelf_takes_the_shops_own_bullet() {
        // The shop before the Sprawler puts a Torch first; without the shelf
        // `want=torch` buys that one, in whatever set it rolled.
        for seed in 1..=4 {
            let mut r = Run::new(seed);
            r.restock();
            r.won(2);
            let offered = r.offers.iter().find_map(|&o| match o {
                Card::Ammo(a) if a.shell == Shell::Torch => Some(card_key(o)),
                _ => None,
            });
            let p = Plan::parse("want=torch,shelf=0,cards=0,shop=").unwrap();
            let bought = shop(&mut r, &p);
            assert_eq!(bought.first(), offered.as_ref(), "seed {seed}: {bought:?}");
            assert!(bought.first().is_some_and(|b| b.starts_with("torch.")), "seed {seed}: {bought:?}");
        }
    }

    /// `gun=KIND` buys that gun once, for the main squad (`arm=0`) or for
    /// the holding groups' loadout with `want2`'s cards (`arm=1`), whose
    /// gun and cards `strat::arm_groups` hands to the hand.
    #[test]
    fn a_plan_buys_a_second_gun_for_the_squad_or_the_groups() {
        use slime_mould::mould::guns::GunKind;
        for seed in 1..=3 {
            let mut r = Run::new(seed);
            r.restock();
            r.won(2);
            r.points += 500;
            let p = Plan::parse("gun=long,want=wide,cards=0,shop=").unwrap();
            let bought = shop(&mut r, &p);
            assert!(bought.iter().any(|b| b.starts_with("gun.long@")), "seed {seed}: {bought:?}");
            assert_eq!(r.gun_of(0).map(|g| g.kind), Some(GunKind::Long), "seed {seed}");
            assert_eq!((r.editing, r.guns.len()), (0, 2), "seed {seed}");
            assert!(r.cards_of(0).contains(&Card::Wide), "seed {seed}: the wanted card moves to the new gun");
            // Once: the next shop buys no other gun.
            r.won(2);
            r.points += 500;
            assert!(!shop(&mut r, &p).iter().any(|b| b.starts_with("gun.")), "seed {seed}");

            let mut r = Run::new(seed);
            r.restock();
            r.won(2);
            r.points += 500;
            let p = Plan::parse("gun=carbine,arm=1,split=2,want=wide,want2=rate,cards=0,shop=").unwrap();
            let bought = shop(&mut r, &p);
            assert!(bought.iter().any(|b| b.starts_with("gun.carbine@")), "seed {seed}: {bought:?}");
            assert_eq!(r.gun_of(0).map(|g| g.kind), Some(GunKind::Rifle), "seed {seed}: the main squad keeps the rifle");
            assert_eq!(r.gun_of(strat::GROUPS).map(|g| g.kind), Some(GunKind::Carbine), "seed {seed}");
            assert_eq!((r.cards_of(0), r.cards_of(strat::GROUPS)), (vec![Card::Wide], vec![Card::Rate]), "seed {seed}");
            assert_eq!(r.editing, 0, "seed {seed}");
            strat::arm_groups(&r);
        }
    }

    /// `tcards` fills the turrets' loadout, and `order` sets the squads'
    /// firing order.
    #[test]
    fn a_plan_loads_its_turrets_and_orders_its_gun() {
        let mut r = Run::new(4);
        r.restock();
        r.won(2);
        r.points += 2_000;
        let p = Plan::parse("want=wide/rate,tcards=pierce/wide,order=wide/rate,cards=0,shop=turret").unwrap();
        let bought = shop(&mut r, &p);
        assert!(r.has("turret"), "{bought:?}");
        assert_eq!(r.turret_cards(), vec![Card::Pierce, Card::Wide], "{bought:?}");
        assert_eq!(r.squad_cards(), vec![Card::Wide, Card::Rate], "the order named, not the shop's (rate first)");
        assert_eq!(r.editing, 0);
        let p = Plan::parse("order=rate/wide,cards=0,shop=").unwrap();
        shop(&mut r, &p);
        assert_eq!(r.squad_cards(), vec![Card::Rate, Card::Wide]);
    }

    #[test]
    fn a_plan_by_planet_plays_one_a_planet_and_the_last_after() {
        let spec = "rush|econ,go=4|lure";
        assert_eq!([1, 2, 3, 4, 8].map(|d| stage(spec, d)), ["rush", "econ,go=4", "lure", "lure", "lure"]);
        assert_eq!(stage("rush,aim=6", 5), "rush,aim=6");
    }
}
