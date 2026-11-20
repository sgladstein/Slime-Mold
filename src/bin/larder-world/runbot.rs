//! **The run bot** (the Engine and tools lane's job 4, the plan of
//! 2026-11-17; Milestone 1's exit test 1) is the strategy bot since
//! 2026-11-20 (NOW.md: "one bot through planets 1 to 3 in a run, joined
//! with the strategy bots rather than a second bot"): `runbot=SEED[:TO]`
//! is the short form of `league=run:SEED:TO plan=PLAN` (`league::whole_run`),
//! with `PLAN` unless `plan=` names another. CI's curve
//! (`.github/workflows/run-bot.yml`) plays the plan in
//! `scripts/run-bot-plan.txt`, which the strategy bot's thread sets.
//!
//! Until then this file held a loop of its own with the `bot` hand
//! (`hands::bot_hand`) and the shop below, a second bot beside the
//! league's; it went with this change. The shop stays for the gun lab
//! (`gun_lab`), which lands every deck on the same squad by it.

use super::*;

/// The run bot's plan when `plan=` names none, as the first line of
/// `scripts/run-bot-plan.txt` (CI's) until the strategy bot's thread sets
/// that file: the old run bot's play (`bot`) with the Torch bought before
/// planet 2, as the old bot's shop did after the balance review (#539).
pub(super) const PLAN: &str = "bot,want=torch.standard";

/// The shop's items the bot buys, in this order, one level of each per
/// pass while points last: what makes the squad kill faster and live
/// longer first, as the bot's plan is to win by fighting.
const WANTS: [&str; 7] = ["strike", "rate", "hp", "squad", "harvest", "range", "bank"];

/// Spends the run's points (`Run::pick`), returning what it bought:
/// first the bullet the planet ahead asks for (`run::bring_shell`, a Torch
/// for planets 2 and 3), then its card (`run::bring`, the planet card's
/// advice: Tinder for planet 2's Sprawler), then `WANTS`.
///
/// The Torch comes first because Tinder lays brush and kills nothing until
/// a fire runs along it. Until 2026-11-20 the bot bought Tinder alone, so
/// on every run bot check its planet 2 gun laid brush that never burned
/// (the balance review, `Reports/balance-review-2026-11-20/`).
pub(super) fn shop(r: &mut Run) -> Vec<String> {
    let mut bought = Vec::new();
    let tier = r.level().tier;
    if let Some(shell) = slime_mould::mould::run::bring_shell(tier) {
        let asked = |o: &slime_mould::mould::rounds::Card| matches!(o, slime_mould::mould::rounds::Card::Ammo(a) if a.shell == shell);
        if let Some(i) = r.offers.iter().position(asked) {
            let c = r.offers[i];
            if r.pick(slime_mould::mould::run::Pick::Offer(i)) {
                bought.push(format!("{} (the gun now fires {})", c.title(), r.gun_text()));
            }
        }
    }
    if let Some(c) = slime_mould::mould::run::bring(tier) {
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

/// **The gun lab** (2026-11-18, the gun-building research thread: which
/// card combinations make the strongest guns, against the owner's rule
/// that no single card should win alone; `Reports/gun-builds-2026-11-19/`):
/// `gunlab=DEPTH:SEED[:SECS] deck=CARDS [gun=KIND] [arena=N[:focus]]
/// [tail=T]` lands on run SEED's planet DEPTH with the gun KIND (the
/// kind's average for that depth, rifle by default) loaded with `deck`
/// (`lab_deck`). The planets before are taken as won with two stars each
/// and their points spent as the run bot's shop spends them on upgrades
/// (`WANTS`), with no cards bought, so every deck meets the same squad.
///
/// A deck no player could build is refused with `how=unfit`: over the
/// gun's budget or slots (`rounds::fit_gun`), or a card that is no bullet
/// twice (a run's shop never offers one again until every card is owned;
/// the adversarial review, 11-19, finding 5). `from=` is the first planet
/// whose shop sells every card in it.
///
/// With no arena the run bot (`hands::bot_hand`) plays to the verdict or
/// SECS. With `arena=N` the arena hand does (`arena_hand`), SECS then
/// `tail` more seconds with fire held, so slow killers (germ, fire) and
/// what wakes from sleep are counted at the end.
pub(super) fn gun_lab(base: Args, (depth, seed, secs, arena): (u32, u64, f32, usize), (kind, deck, focus, tail, guard): (&str, &str, bool, f32, bool), device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), String> {
    use slime_mould::mould::guns::GunKind;
    use slime_mould::mould::rounds::{self as rd, Kind};
    let mut r = Run::new(seed);
    r.restock();
    while r.depth < depth {
        r.won(2);
        r.offers.clear();
        shop(&mut r);
    }
    let kind = GunKind::parse(kind).ok_or_else(|| format!("gun={kind}: rifle, carbine, long or scatter"))?;
    let gun = kind.average_at(depth);
    let cards = lab_deck(deck, depth)?;
    let cost = rd::cost(&cards);
    let from = cards.iter().map(|&c| slime_mould::mould::run::power_from(c.tier())).max().unwrap_or(1);
    let head = format!("gunlab depth={depth} seed={seed} arena={arena} focus={} guard={} tail={tail} gun={} deck={deck} cost={cost} budget={} slots={} from={from}", u8::from(focus), u8::from(guard), gun.kind.key(), gun.budget, gun.slots);
    let twice = cards.iter().enumerate().any(|(i, c)| c.kind() != Kind::Payload && !matches!(c, rd::Card::Ammo(_)) && cards[..i].contains(c));
    if rd::fit_gun(cards.clone(), &gun).is_err() || twice {
        println!("{head} how=unfit");
        return Ok(());
    }
    r.guns = vec![gun];
    // The shown loadout fires the lab's gun (loadouts, 2026-11-19).
    if let Some(l) = r.loadouts.get_mut(r.editing) {
        l.gun = Some(0);
    }
    r.cards = cards.iter().map(|&c| (c, true)).collect();
    let l = r.level();
    let mut args = base;
    args.seed = l.seed;
    args.walkers = run_walkers(l.walkers);
    args.run = Some(r.clone());
    args.squads = true;
    args.map = None;
    args.mission = None;
    // The deck and gun are the run's (`r`), not the film's `gun=`.
    args.gun = None;
    let mut app = offscreen(args, (320, 200), device, queue)?;
    let g = app.game.as_ref().ok_or("no game")?;
    let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
    let fire_secs = if secs > 0.0 { secs } else { g.rules.cap + 60.0 };
    let limit = if arena > 0 { fire_secs + tail } else { fire_secs };
    let start_mould = g.mould;
    let mut last_sec = u64::MAX;
    let mut marks: Vec<String> = Vec::new();
    let mut hp_sum = 0.0f64;
    let mut hp_n = 0u32;
    let mut min_alive = usize::MAX;
    let mut peak_alive = 0usize;
    // The owner's 04:10Z ask: measures that do not saturate when a strong
    // deck clears everything. Shot kills by range (soldier to hit, dish
    // cells), kills each 10 s (the best window), health lost in all (the
    // dead's too), the slime within two gun ranges of the squad (the
    // pressure on it), the mean slime over the fight (its area, which a
    // deck that clears early wins on), and the slime when the fire stops.
    let mut shot_kills = 0u64;
    let mut range_sum = 0.0f64;
    let mut far_kills = 0u64;
    let mut hp_lost = 0.0f64;
    let mut last_hp: Vec<f32> = Vec::new();
    let mut kill10: Vec<u32> = Vec::new();
    let mut near_sum = 0.0f64;
    let mut near_n = 0u32;
    let mut mould_sum = 0.0f64;
    let mut mould_n = 0u32;
    let mut mould_fire: Option<usize> = None;
    // Soldiers alive and health lost when the fire stops: a squad that
    // holds fire in the tail is eaten, which is not the deck's doing.
    let mut fire_end: Option<(usize, f64)> = None;
    let mut first_kill: Option<f32> = None;
    // The guard (`guard_hand`): slime on the guarded crystal, a mean over
    // the fight's seconds, and the food taken in the fight.
    let mut guard_sum = 0.0f64;
    let mut guard_n = 0u32;
    let mut harvest_fire: Option<f32> = None;
    let harvest0 = app.game.as_ref().map_or(0.0, |g| g.harvested);
    let mut ended: f32;
    let t0 = std::time::Instant::now();
    loop {
        app.follow_mission();
        let verdict = app.mission.as_ref().and_then(|m| m.verdict);
        let g = app.game.as_mut().unwrap();
        let since = g.time() - g.landed as f32 / g.rules.tps as f32;
        ended = since;
        if verdict.is_some() || g.end.is_some() || since > limit {
            break;
        }
        let sec = g.world.ticks() / g.rules.tps as u64;
        if sec != last_sec {
            last_sec = sec;
            if arena > 0 {
                if guard {
                    guard_hand(g, arena, since, since > fire_secs);
                } else {
                    arena_hand(g, arena, since, focus, since > fire_secs);
                }
                if let Some(c) = GUARDED.with(std::cell::Cell::get) {
                    let cs = g.world.crystals();
                    guard_sum += f64::from(g.world.walkers_in(cs[c].0, g.world.crystal_r(c)));
                    guard_n += 1;
                }
            } else {
                hands::bot_hand(g);
            }
            let alive = g.alive();
            peak_alive = peak_alive.max(alive);
            if since > 5.0 {
                min_alive = min_alive.min(alive);
            }
            let hps: Vec<f32> = g.soldiers.iter().filter(|s| s.hp > 0.0).map(|s| s.hp / g.rules.hp.max(1.0)).collect();
            if !hps.is_empty() {
                hp_sum += f64::from(hps.iter().sum::<f32>() / hps.len() as f32);
                hp_n += 1;
            }
            if since <= fire_secs {
                mould_sum += g.mould as f64;
                mould_n += 1;
            } else if mould_fire.is_none() {
                mould_fire = Some(g.mould);
                fire_end = Some((alive, hp_lost));
                harvest_fire = Some(g.harvested);
            }
            let s30 = since.round() as i64;
            if s30 % 30 == 0 {
                marks.push(format!("{}:{}/{}/{}", s30, g.killed, g.mould, alive));
            }
        }
        app.step(step);
        app.settle();
        let g = app.game.as_ref().unwrap();
        let sc = g.world.scale();
        let since = g.time() - g.landed as f32 / g.rules.tps as f32;
        let range = g.live_squads().first().map_or(24.0, |&q| g.gun_range(q));
        for sh in &g.last_shots {
            if let (Some(k), Some(h), true) = (sh.by, sh.hit, sh.killed > 0) {
                let so = &g.soldiers[k];
                let d = (h.0 - so.x).hypot(h.1 - so.y) / sc;
                shot_kills += u64::from(sh.killed);
                range_sum += f64::from(d) * f64::from(sh.killed);
                if d > 0.5 * range {
                    far_kills += u64::from(sh.killed);
                }
            }
        }
        if g.killed > 0 && first_kill.is_none() {
            first_kill = Some(since);
        }
        if last_hp.len() < g.soldiers.len() {
            last_hp.resize(g.soldiers.len(), g.rules.hp);
        }
        for (k, so) in g.soldiers.iter().enumerate() {
            let now = so.hp.max(0.0);
            if now < last_hp[k] {
                hp_lost += f64::from(last_hp[k] - now) / f64::from(g.rules.hp.max(1.0));
            }
            last_hp[k] = now;
        }
        let tick = (since / 10.0).floor().max(0.0) as usize;
        if kill10.len() <= tick {
            kill10.resize(tick + 1, g.killed);
            if let Some(m) = g.live_squads().first().and_then(|&q| g.middle(q)) {
                near_sum += g.world.walkers_in(m, 2.0 * range * sc) as f64;
                near_n += 1;
            }
        }
        if let Some(last) = kill10.last_mut() {
            *last = g.killed;
        }
    }
    let g = app.game.as_ref().unwrap();
    let verdict = app.mission.as_ref().and_then(|m| m.verdict);
    let mut prev = 0u32;
    let per10: Vec<u32> = kill10
        .iter()
        .map(|&k| {
            let d = k.saturating_sub(prev);
            prev = k;
            d
        })
        .collect();
    let best10 = per10.iter().copied().max().unwrap_or(0);
    let stars = verdict.map_or(0, |v| v.stars);
    // A win is the goal met (the review's finding 1: a stall, `Stuck`, or
    // the clock, `Cap`, can grade a star too).
    let cause = verdict.map_or("none".to_string(), |v| format!("{:?}", v.cause).to_lowercase());
    let won = verdict.is_some_and(|v| v.stars > 0 && matches!(v.cause, slime_mould::mould::campaign::Cause::Goal | slime_mould::mould::campaign::Cause::MouldWiped));
    let how = match (verdict, g.end) {
        _ if won => "won",
        (Some(v), _) if v.stars > 0 => "graded",
        (Some(_), _) | (None, Some(_)) => "lost",
        (None, None) => "open",
    };
    let ro = &app.rounds;
    println!(
        "{head} how={how} cause={cause} won={} stars={stars} secs={ended:.1} killed={} mould={} start_mould={start_mould} mould_fire={} mould_mean={:.0} alive_fire={} hp_lost_fire={:.2} alive={} peak={peak_alive} min_alive={} hp={:.3} hp_lost={hp_lost:.2} bank={:.0} shots={} hits={} hit_cells={} shot_kills={shot_kills} kill_range={:.1} far_share={:.3} round_kills={} fire_kills={} gassed={} flung={} dropped={} unsupported={} bullets_dropped={} harvest={:.0} on_crystal={:.0} best10={best10} first_kill={:.1} near={:.0} per10={} wall={:.0} marks={}",
        u8::from(won),
        g.killed,
        g.mould,
        mould_fire.unwrap_or(g.mould),
        if mould_n > 0 { mould_sum / f64::from(mould_n) } else { 0.0 },
        fire_end.map_or(g.alive(), |f| f.0),
        fire_end.map_or(hp_lost, |f| f.1),
        g.alive(),
        if min_alive == usize::MAX { 0 } else { min_alive },
        if hp_n > 0 { hp_sum / f64::from(hp_n) } else { 0.0 },
        g.bank,
        g.shots,
        g.hits,
        g.hit_cells,
        if shot_kills > 0 { range_sum / shot_kills as f64 } else { 0.0 },
        if shot_kills > 0 { far_kills as f64 / shot_kills as f64 } else { 0.0 },
        ro.killed,
        g.fire_killed,
        ro.gassed,
        ro.flung,
        ro.dropped,
        ro.unsupported,
        g.bullets_dropped,
        harvest_fire.unwrap_or(g.harvested) - harvest0,
        if guard_n > 0 { guard_sum / f64::from(guard_n) } else { -1.0 },
        first_kill.unwrap_or(-1.0),
        if near_n > 0 { near_sum / f64::from(near_n) } else { 0.0 },
        per10.iter().map(u32::to_string).collect::<Vec<_>>().join("/"),
        t0.elapsed().as_secs_f32(),
        marks.join(",")
    );
    Ok(())
}

/// A lab deck: cards by name joined with `+` (`rounds::parse`), where a
/// payload may name its set after a dot (`blast.buckshot`, `gas.slug`):
/// that shell's bullet as a shop at `depth` rolls it on average, the set's
/// exact lean (`Set::lean`) and shape, the depth's damage, and the cost
/// its grade gives (`bullets::roll_with` with every draw at its mean).
pub(super) fn lab_deck(s: &str, depth: u32) -> Result<Vec<slime_mould::mould::rounds::Card>, String> {
    use slime_mould::mould::bullets::{Ammo, Set, Shell, COST_POWER, DAMAGE, DEPTH_DAMAGE, DEPTH_DAMAGE_MAX};
    use slime_mould::mould::rounds::Card;
    if s.is_empty() || s == "none" {
        return Ok(Vec::new());
    }
    s.split('+')
        .map(|w| match w.split_once('.') {
            Some((shell, set)) => {
                let shell = Shell::ALL.into_iter().find(|p| p.key() == shell).ok_or_else(|| format!("deck: no shell {shell}"))?;
                let set = Set::parse(set).ok_or_else(|| format!("deck: no set {set} (standard, slug, needle, buckshot)"))?;
                let l = set.lean();
                let h = |x: f32| (x * 100.0).round() as u16;
                let d = (1.0 + DEPTH_DAMAGE * (depth.max(1) - 1) as f32).min(DEPTH_DAMAGE_MAX);
                let mut a = Ammo { shell, set, depth: depth.clamp(1, 255) as u8, damage: (DAMAGE * l.damage * d * 10.0).round() as u16, speed: h(l.speed), range: h(l.range), spread: h(l.spread), cost: 0 };
                a.cost = (shell.base() as f32 * a.grade().powf(COST_POWER)).round().clamp(1.0, 255.0) as u8;
                Ok(Card::Ammo(a))
            }
            None => Card::from_name(w).ok_or_else(|| format!("deck: no card {w}")),
        })
        .collect()
}

/// **The arena** (2026-11-19, the owner's 00:05Z ask: "place x number of
/// soldiers in a box with a certain amount of slime and see how much
/// damage it does"): N soldiers bought at once on landing on the planet's
/// real slime and ground, with no bank after (no more soldiers, no
/// healing), in one squad. No crystals, pumps or shop: only the gun
/// differs from one deck to the next, on the same slime.
///
/// By default the squad is not the player's (no lead edge: the review's
/// finding 3, the lead's x1.4 strike pinned every plain strike at
/// `R_MAX`) and fires on its own, free, at the densest slime each soldier
/// sees (the game's own aim, `Stance::Free`); with none in range it
/// attack-moves to stand short of the heaviest slime. `focus` is the run
/// bot's way instead: the player's squad, every soldier volleying at the
/// one heaviest bin in range with a clear line (finding 2: that piles
/// every bullet on one point, which favours area cards). Bitten and under
/// half health it steps back a range. `hold` (the tail): fire held, the
/// squad walks back to the larder.
fn arena_hand(g: &mut Game<GpuWorld>, n: usize, since: f32, focus: bool, hold: bool) {
    if since < 1.0 {
        g.bank = 1.0e9;
        while g.alive() < n {
            if !g.apply(Cmd::Buy) {
                break;
            }
        }
        g.bank = 0.0;
    }
    let live = g.live_squads();
    let Some(&q) = live.first() else { return };
    for &o in &live[1..] {
        g.apply(Cmd::Merge(q as u8, o as u8));
    }
    let lead = if focus { Some(q) } else { None };
    if g.active != lead {
        g.apply(Cmd::Active(lead.map_or(u8::MAX, |q| q as u8)));
    }
    let members = g.members(q);
    let want = if hold { Stance::HoldFire } else if focus { Stance::Defend } else { Stance::Free };
    for &k in &members {
        if g.soldiers[k].stance != want {
            g.apply(Cmd::Stance(k, want));
        }
    }
    let Some(m) = g.middle(q) else { return };
    if hold {
        // Out of the fight to the larder: a squad standing with its fire
        // held is eaten (the first calibration, 11-19: 9 soldiers to 3 in
        // 30 s), which is no measure of the deck.
        g.apply(Cmd::Fire(q as u8, None));
        let home = g.larder;
        g.apply(Cmd::SquadOrder(q as u8, Order::Move(home.0, home.1)));
        return;
    }
    let k = members.len().max(1);
    let hp = members.iter().map(|&i| g.soldiers[i].hp).sum::<f32>() / k as f32 / g.rules.hp.max(1.0);
    let bitten = members.iter().filter(|&&i| g.soldiers[i].touching > 0).count();
    let sc = g.world.scale();
    let range = g.gun_range(q) * sc;
    let bin = (4.0 * sc).round().max(1.0) as usize;
    let (w, _) = g.world.size();
    let gw = w.div_ceil(bin);
    let grid = g.world.mould_grid(bin).unwrap_or_default();
    let set = g.world.set_cells().unwrap_or_default();
    // The run bot's clear line (`hands::bot_hand`; the review's finding
    // 8): open ground, no set walker on it.
    let clear = |g: &Game<GpuWorld>, a: (f32, f32), b: (f32, f32)| {
        let n = (b.0 - a.0).hypot(b.1 - a.1).ceil().max(1.0) as usize;
        (0..=n).all(|i| {
            let f = i as f32 / n as f32;
            let (x, y) = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            g.ground(x, y) && (x < 0.0 || y < 0.0 || x as usize >= w || set.get(y as usize * w + x as usize).is_none_or(|&c| c == 0))
        })
    };
    let centre = |i: usize| (((i % gw) as f32 + 0.5) * bin as f32, ((i / gw) as f32 + 0.5) * bin as f32);
    let mut ranked: Vec<((f32, f32), f32, f32)> = grid
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c >= 8)
        .map(|(i, &c)| {
            let p = centre(i);
            let d = (p.0 - m.0).hypot(p.1 - m.1);
            (p, d, c as f32 / (1.0 + d / (2.0 * range.max(1.0))))
        })
        .collect();
    ranked.sort_by(|a, b| b.2.total_cmp(&a.2));
    let near = ranked.iter().take(40).find(|&&(p, d, _)| d <= range && clear(g, m, p)).map(|r| r.0);
    if focus {
        g.apply(Cmd::Fire(q as u8, near));
    }
    if hp < 0.5 && bitten > 0 {
        if let Some(a) = near {
            let d = (a.0 - m.0).hypot(a.1 - m.1).max(1.0);
            g.apply(Cmd::SquadOrder(q as u8, Order::Move(m.0 - (a.0 - m.0) / d * range, m.1 - (a.1 - m.1) / d * range)));
        }
        return;
    }
    if near.is_none() {
        if let Some(&(p, d, _)) = ranked.first() {
            let f = (d - 0.7 * range).max(0.0) / d.max(1.0);
            let to = (m.0 + (p.0 - m.0) * f, m.1 + (p.1 - m.1) * f);
            g.apply(Cmd::SquadOrder(q as u8, if focus { Order::Move(to.0, to.1) } else { Order::Fight(to.0, to.1) }));
        }
    } else if !focus && !members.iter().all(|&k| g.soldiers[k].order == Order::Hold) {
        g.apply(Cmd::SquadOrder(q as u8, Order::Hold));
    }
}

thread_local! {
    /// The crystal `guard_hand` holds this game.
    static GUARDED: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

/// **The guard** (2026-11-19, the owner's play of 19:16Z: guns with a
/// strong push killed less until he split his squad to hold one crystal
/// each, where the push kept the slime off and he took every crystal):
/// `arena=N:guard`. The arena's N soldiers, not the player's squad, hold
/// the crystal (not the home) with the most slime round it at landing
/// with the game's Guard order and Defend stance, so they stand and shoot
/// what comes onto it. The measures are
/// the food taken and the slime on the crystal, not the kills, so a card
/// that keeps slime away is judged on that.
fn guard_hand(g: &mut Game<GpuWorld>, n: usize, since: f32, hold: bool) {
    if since < 1.0 {
        GUARDED.with(|c| c.set(None));
        g.bank = 1.0e9;
        while g.alive() < n {
            if !g.apply(Cmd::Buy) {
                break;
            }
        }
        g.bank = 0.0;
    }
    let live = g.live_squads();
    let Some(&q) = live.first() else { return };
    for &o in &live[1..] {
        g.apply(Cmd::Merge(q as u8, o as u8));
    }
    if g.active.is_some() {
        g.apply(Cmd::Active(u8::MAX));
    }
    let cs = g.world.crystals();
    let pick = GUARDED.with(std::cell::Cell::get).or_else(|| {
        let range = g.gun_range(q) * g.world.scale();
        let c = (1..cs.len()).filter(|&c| cs[c].1 > 0.0).max_by_key(|&c| g.world.walkers_in(cs[c].0, g.world.crystal_r(c) + range));
        GUARDED.with(|x| x.set(c));
        c
    });
    let Some(c) = pick else { return };
    let members = g.members(q);
    // The game's own Guard order with the Defend stance, as a player
    // guards a crystal (owner 19:28Z: "a turret guarding a wall or just
    // using the defend action"): no walking out to drain or chase.
    let want = if hold { Stance::HoldFire } else { Stance::Defend };
    for &k in &members {
        if g.soldiers[k].stance != want {
            g.apply(Cmd::Stance(k, want));
        }
    }
    let o = if hold { Order::Move(g.larder.0, g.larder.1) } else { Order::Guard(cs[c].0 .0, cs[c].0 .1) };
    if !members.iter().all(|&k| g.soldiers[k].order == o) {
        g.apply(Cmd::SquadOrder(q as u8, o));
    }
}
