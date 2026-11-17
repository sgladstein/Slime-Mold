//! The scripted hands the headless films and benches play. Moved out of
//! `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// The headless run's hand, once a game second.
/// The look mode's `volley` hand (the owner, 11-12 16:55Z, on the crater
/// clips: "in these videos they were mostly shooting at individuals so it
/// was hard to evaluate until the end when they shot at the core of
/// slime"). The squad spends its bank on soldiers in the first second and
/// from 15 s walks to a point in range of the thick ring of mould on the
/// core crystal (short of the core's pull). At `VOLLEY` it burns one
/// line across the ring's near side (`Order::Burn`, the window's F), so
/// every shot goes into the body, not at stray walkers; at `VOLLEY_STOP`
/// it stops firing and steps back, so the clip shows the hole refilling
/// with nobody shooting. (A first cut walked as `stand` does and burned a
/// line 0.6 of the range ahead; on the Fork that line lay on thin
/// tendrils, 48 kills in 12 s.)
///
/// `edge` (2026-11-12, for the sink, `Rules::sink`): the same walk and
/// stop, then from `VOLLEY` to `VOLLEY_STOP` the squad stands still on free
/// fire (`Stance::Free`, the densest mould in range) instead of burning a
/// line: the owner's 22:06Z case, a group kept in one spot firing on a
/// large mass.
pub(super) const VOLLEY: f32 = 22.0;
pub(super) const VOLLEY_STOP: f32 = 34.0;

pub(super) fn volley(g: &mut Game<GpuWorld>, free: bool) {
    let t = g.time();
    let sc = g.world.scale();
    let (dx, dy) = (g.core.0 - g.larder.0, g.core.1 - g.larder.1);
    let d = dx.hypot(dy).max(1.0);
    let u = (dx / d, dy / d);
    // The line's middle just outside the core crystal, and where a
    // burning soldier stands back from it (0.6 of its range).
    let reach = g.world.crystal_r(0) + 2.0 * sc;
    let m = (g.core.0 - u.0 * reach, g.core.1 - u.1 * reach);
    let keep = 0.6 * g.gun_range(0) * sc;
    let stand = (m.0 - u.0 * keep, m.1 - u.1 * keep);
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    let burning = live.iter().any(|&k| matches!(g.soldiers[k].order, Order::Burn(..)));
    if t < VOLLEY {
        // Bounded by the squad's cap too: with a bank past the cap's price
        // (`bank=` films) the loop never ended, and the command log grew
        // until the run ran out of memory (11-14).
        while t < 1.0 && g.bank >= g.rules.soldier_cost && g.alive() < g.rules.max_squad {
            g.apply(Cmd::Buy);
        }
        // The starting disc of walkers fills the stem until it thins out
        // (5,000 at 14 s, about 2,200 by 22 s): a squad sent at once died
        // in it by 14 s. So it waits, then walks on its Defend stance.
        for k in 0..g.soldiers.len() {
            if t >= VOLLEY - 7.0 && g.soldiers[k].alive && g.soldiers[k].order == Order::Hold {
                g.apply(Cmd::Order(k, Order::Move(stand.0, stand.1)));
            }
        }
    } else if free && t < VOLLEY_STOP {
        for &k in &live {
            if g.soldiers[k].stance != Stance::Free {
                g.apply(Cmd::Stance(k, Stance::Free));
            }
        }
    } else if free {
        let back = (stand.0 - u.0 * 8.0 * sc, stand.1 - u.1 * 8.0 * sc);
        for &k in &live {
            if g.soldiers[k].stance != Stance::HoldFire {
                g.apply(Cmd::Stance(k, Stance::HoldFire));
                g.apply(Cmd::Order(k, Order::Move(back.0, back.1)));
            }
        }
    } else if t < VOLLEY_STOP && !burning {
        // Ten dish cells long, across the way to the core.
        let half = 5.0 * sc;
        let (a, b) = ((m.0 - u.1 * half, m.1 + u.0 * half), (m.0 + u.1 * half, m.1 - u.0 * half));
        for &k in &live {
            g.apply(Cmd::Order(k, Order::Burn(a, b)));
        }
    } else if t >= VOLLEY_STOP && burning {
        let back = (stand.0 - u.0 * 8.0 * sc, stand.1 - u.1 * 8.0 * sc);
        for &k in &live {
            g.apply(Cmd::Stance(k, Stance::HoldFire));
            g.apply(Cmd::Order(k, Order::Move(back.0, back.1)));
        }
    }
}

pub(super) fn plan(g: &mut Game<GpuWorld>, hand: &str) {
    if hand == "squads" {
        return squads_hand(g);
    }
    if hand == "squadwork" {
        return squadwork_hand(g);
    }
    if hand == "pumps" {
        return pumps_hand(g);
    }
    if hand == "guardwork" {
        return guardwork_hand(g);
    }
    if hand == "greedy" {
        return greedy_hand(g);
    }
    if hand == "guard" {
        return guard_hand(g);
    }
    // `build`: farm, with a pump on the larder's crystal, two turrets on
    // the road to the core and a lure past them (the look at buildings);
    // `siren` the same with a siren in the lure's place (a tier's look).
    if (hand == "build" || hand == "siren") && g.time() > 1.0 {
        let sc = g.world.scale();
        let (c, l) = (g.core, g.larder);
        let (dx, dy) = (c.0 - l.0, c.1 - l.1);
        let d = dx.hypot(dy).max(1.0);
        let (ux, uy) = (dx / d, dy / d);
        let mut wants = vec![(Kind::Turret, (l.0 + ux * 7.0 * sc - uy * 4.0 * sc, l.1 + uy * 7.0 * sc + ux * 4.0 * sc)), (Kind::Turret, (l.0 + ux * 7.0 * sc + uy * 4.0 * sc, l.1 + uy * 7.0 * sc - ux * 4.0 * sc)), (Kind::Lure, (l.0 + ux * 13.0 * sc, l.1 + uy * 13.0 * sc))];
        // The siren 0.6 of the way to the core, in the stem the Fork's mould
        // comes up, where its call reaches the mould.
        if hand == "siren" {
            let p = (l.0 + dx * 0.42, l.1 + dy * 0.42);
            wants[2] = (Kind::Siren, p);
            // Soldier 2 walks there so the siren is in a builder's reach.
            if !g.buildings.iter().any(|b| b.kind == Kind::Siren) && g.soldiers.len() > 2 && g.soldiers[2].alive && g.soldiers[2].order != Order::Move(p.0, p.1) {
                g.apply(Cmd::Order(2, Order::Move(p.0, p.1)));
            }
        }
        // The pump on the nearest big crystal (half the biggest's charge or
        // more; the Fork's nearest is a small one a pump drains in 5 s).
        let cs = g.world.crystals();
        let big = cs.iter().map(|c| c.1).fold(0.0f32, f32::max) * 0.5;
        if let Some(cc) = cs.iter().skip(1).filter(|c| c.1 >= big).map(|c| c.0).min_by(|a, b| (a.0 - l.0).hypot(a.1 - l.1).total_cmp(&(b.0 - l.0).hypot(b.1 - l.1))) {
            wants.insert(0, (Kind::Pump, cc));
            // Soldier 0 harvests it, so the pump is in a soldier's reach.
            if g.soldiers[0].alive && g.soldiers[0].order == Order::Hold {
                g.apply(Cmd::Order(0, Order::Clear(cc.0, cc.1)));
            }
        }
        for (kind, p) in wants {
            if g.buildings.iter().any(|b| b.kind == kind && (b.at.0 - p.0).hypot(b.at.1 - p.1) < 3.0 * sc) {
                continue;
            }
            let at = (0..=12).flat_map(|r| (0..12).map(move |k| (r, k))).map(|(r, k)| {
                let a = k as f32 / 12.0 * std::f32::consts::TAU;
                (p.0 + 0.5 * r as f32 * sc * a.cos(), p.1 + 0.5 * r as f32 * sc * a.sin())
            }).find(|q| g.build_blocked(kind, *q).is_none());
            if let Some(at) = at {
                g.apply(Cmd::Build(kind, at));
            }
        }
        // Buildings rise over time (`BuildRules::raise`): soldier 1, or
        // soldier 0 alone, raises the first ghost, then farms again.
        if let Some(b) = g.buildings.iter().position(|b| b.alive && !b.up()) {
            let k = if g.soldiers.len() > 1 { 1 } else { 0 };
            if g.soldiers[k].alive && !matches!(g.soldiers[k].order, Order::Build(_)) {
                g.apply(Cmd::Order(k, Order::Build(b)));
            }
        } else if let Some(k) = (0..g.soldiers.len()).find(|&k| matches!(g.soldiers[k].order, Order::Build(_))) {
            g.apply(Cmd::Order(k, Order::Hold));
        }
    }
    // `buildq` (2026-11-14, the build-flow thread's clips): eight soldiers
    // at the larder; at 2 s every one is ordered to a turret 28 cells
    // toward the core, then two more beside it are queued (Shift + clicks
    // in the window), with the bank topped up to pay for all three. Shows
    // where builders stand and the queue rising in turn.
    if hand == "buildq" {
        while g.time() < 1.0 && g.soldiers.len() < 8 && g.bank >= g.rules.soldier_cost {
            g.apply(Cmd::Buy);
        }
        if g.time() > 2.0 && g.buildings.is_empty() {
            g.bank = g.bank.max(2000.0);
            let sc = g.world.scale();
            let (c, l) = (g.core, g.larder);
            let d = (c.0 - l.0).hypot(c.1 - l.1).max(1.0);
            let (ux, uy) = ((c.0 - l.0) / d, (c.1 - l.1) / d);
            for (i, side) in [0.0f32, 9.0, -9.0].into_iter().enumerate() {
                let p = (l.0 + ux * 28.0 * sc - uy * side * sc, l.1 + uy * 28.0 * sc + ux * side * sc);
                let at = (0..=12).flat_map(|r| (0..12).map(move |k| (r, k))).map(|(r, k)| {
                    let a = k as f32 / 12.0 * std::f32::consts::TAU;
                    (p.0 + 0.5 * r as f32 * sc * a.cos(), p.1 + 0.5 * r as f32 * sc * a.sin())
                }).find(|q| g.build_blocked(Kind::Turret, *q).is_none());
                let Some(at) = at else { continue };
                if !g.apply(Cmd::Build(Kind::Turret, at)) {
                    continue;
                }
                let b = g.buildings.len() - 1;
                for k in 0..g.soldiers.len() {
                    if g.soldiers[k].alive {
                        g.apply(if i == 0 { Cmd::Order(k, Order::Build(b)) } else { Cmd::Queue(k, Order::Build(b)) });
                    }
                }
            }
        }
        return;
    }
    // `pen` (2026-11-16, the walls thread's clips; run it with
    // `mode=squads`, `set=wall_width=W` for the thickness): ten soldiers
    // walk `PEN_AT` of the way from the landing toward the core; at
    // `PEN_T` a closed ring of wall `PEN_R` cells round their middle stands
    // whole, and they hold inside it. The slime surges at them and can only
    // eat in (owner, 2026-11-16 06:06Z: "slime should not be able to go
    // over walls but they should be able to slowly eat through"). Prints,
    // each game second, the ring's fine cells still wall, walkers inside,
    // soldiers alive. `door`: the same ring, with a door cut in it that
    // the squad sorties through, then thickened (`pen_hand`).
    // `turretring` (2026-11-16 19:50Z ruling: turrets fire over walls, soldiers'
    // guns do not): the pen's ring with three turrets inside facing the
    // core (`FORT_T`). Each second it also prints the shots so far and
    // the walkers the turrets and the soldiers' guns have killed.
    if hand == "pen" || hand == "door" || hand == "turretring" {
        return pen_hand(g, hand == "door", hand == "turretring");
    }
    // `wallq` (2026-11-16, the wall-sides thread's clips; run it with
    // `mode=squads`): eight soldiers walk 16 cells from where they land,
    // away from the core; at 8 s a 30-cell wall goes up square to that way
    // 1 cell from the squad's middle toward the core, so the squad
    // straddles its line, and the squad is ordered to build it, defenders
    // called out (`CoverRules::near` 0, as by slime). Shows which side each soldier
    // ends on once it stands (owner, 2026-11-16 01:45Z: "some soliders
    // always end up on the wrong side, trapped with the slime").
    if hand == "wallq" {
        while g.time() < 1.0 && g.soldiers.len() < 8 && g.bank >= g.rules.soldier_cost {
            g.apply(Cmd::Buy);
        }
        g.rules.cover.near = 0;
        let sc = g.world.scale();
        let (c, l) = (g.core, g.larder);
        let d = (c.0 - l.0).hypot(c.1 - l.1).max(1.0);
        let u = ((c.0 - l.0) / d, (c.1 - l.1) / d);
        if g.time() > 1.5 && g.time() < 2.5 {
            let to = (l.0 - u.0 * 16.0 * sc, l.1 - u.1 * 16.0 * sc);
            let to = g.nearest_standable(to, 10.0 * sc).unwrap_or(to);
            g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
        }
        // Each game second once it is placed: how many stand on the core's
        // side of its line (the clip's count).
        if let Some(bd) = g.buildings.first().copied() {
            if let Kind::Wall { to } = bd.kind {
                let mid = ((bd.at.0 + to.0) / 2.0, (bd.at.1 + to.1) / 2.0);
                let past = g.soldiers.iter().filter(|s| s.alive && (s.x - mid.0) * u.0 + (s.y - mid.1) * u.1 > 0.0).count();
                println!("  wallq t {:.0}: built {:.0}%, {past} of {} past the wall", g.time(), bd.built * 100.0, g.alive());
            }
        }
        if g.time() > 8.0 && g.buildings.is_empty() {
            g.bank = g.bank.max(2000.0);
            let Some(m) = g.middle(0) else { return };
            let m = (m.0 + u.0 * sc, m.1 + u.1 * sc);
            let p = (-u.1, u.0);
            let (a, b) = ((m.0 - p.0 * 15.0 * sc, m.1 - p.1 * 15.0 * sc), (m.0 + p.0 * 15.0 * sc, m.1 + p.1 * 15.0 * sc));
            if g.apply(Cmd::Wall(a, b)) {
                let w = g.buildings.len() - 1;
                g.apply(Cmd::SquadOrder(0, Order::Build(w)));
            }
        }
        return;
    }
    if hand == "volley" || hand == "edge" {
        volley(g, hand == "edge");
        return;
    }
    // `hold` and `push` (2026-11-13, the wading and arming clips; the owner,
    // 15:55Z: "this is really hard to judge when the soliders are just
    // running into the slime"): eight soldiers, bought at once with 400
    // food kept back (once for rations, removed 11-14), either stand at the larder on free fire as
    // the mould comes up the stem, or walk into the body from 3 s.
    // `hold@X,Y` stands them at (X, Y) instead, world units as `centre=`.
    // `hold@X,Y,T,X2,Y2` (2026-11-17, the setting films): from game second
    // T the post is (X2, Y2), so the squad moves on.
    let nums: Option<Vec<f32>> = hand.strip_prefix("hold@").and_then(|p| p.split(',').map(|v| v.parse::<f32>().ok()).collect());
    // A soldier that reached its post keeps its Move order, so the new
    // post is ordered for half a second from T whatever the order.
    let (post, moving) = match nums.as_deref() {
        Some(&[x, y]) => (Some((x, y)), false),
        Some(&[x, y, t, x2, y2]) => (Some(if g.time() >= t { (x2, y2) } else { (x, y) }), (t..t + 0.5).contains(&g.time())),
        _ => (None, false),
    };
    let hand = if post.is_some() { "hold" } else { hand };
    if hand == "hold" || hand == "push" {
        while g.time() < 1.0 && g.soldiers.len() < 8 && g.bank >= g.rules.soldier_cost {
            g.apply(Cmd::Buy);
        }
        for k in 0..g.soldiers.len() {
            if !g.soldiers[k].alive {
                continue;
            }
            if hand == "hold" && g.soldiers[k].stance != Stance::Free {
                g.apply(Cmd::Stance(k, Stance::Free));
            }
            if let (Some(p), true) = (post, g.time() >= 1.0 && (moving || g.soldiers[k].order == Order::Hold)) {
                g.apply(Cmd::Order(k, Order::Move(p.0, p.1)));
            }
            if hand == "push" && g.time() >= 3.0 && g.soldiers[k].order == Order::Hold {
                g.apply(Cmd::Order(k, Order::Attack));
            }
        }
        return;
    }
    // `sow` (own=1): farm, and soldiers 0 and 1 walk from the larder to a
    // crystal and back sowing the player's own mould (Feed while the bank
    // is over 600, else Road): `examples/garden.rs`'s sow2.
    let sowers: &[usize] = if hand == "sow" && g.world.garden.is_some() { &[0, 1] } else { &[] };
    if !sowers.is_empty() {
        let sc = g.world.scale();
        let cs: Vec<(f32, f32)> = g.world.crystals().iter().skip(1).filter(|c| c.1 > 0.0).map(|c| c.0).filter(|&c| c != g.core && c != g.larder).collect();
        for &k in sowers {
            if k >= g.soldiers.len() || !g.soldiers[k].alive {
                continue;
            }
            let t = if g.bank > 600.0 { Task::Feed } else { Task::Road };
            g.world.garden.as_mut().unwrap().set_task(k, t);
            if g.soldiers[k].order == Order::Hold {
                let so = g.soldiers[k];
                let home = (so.x - g.larder.0).hypot(so.y - g.larder.1) < 4.0 * sc;
                let to = if home { cs.get(k % cs.len().max(1)).copied().unwrap_or(g.larder) } else { g.larder };
                g.apply(Cmd::Order(k, Order::Move(to.0, to.1)));
            }
        }
    }
    // `far`: farm, and at 2 s a turret ordered halfway to the core and a
    // wall past it, both out of every soldier's range, with nobody
    // selected (the window's `builders`: `Cmd::Send`), so the look shows
    // a soldier walking out and the ghosts' bars and times filling.
    if hand == "far" && g.time() > 2.0 && g.buildings.is_empty() {
        let sc = g.world.scale();
        let (c, l) = (g.core, g.larder);
        let (ux, uy) = ((c.0 - l.0) / (c.0 - l.0).hypot(c.1 - l.1).max(1.0), (c.1 - l.1) / (c.0 - l.0).hypot(c.1 - l.1).max(1.0));
        let mid = (l.0 + ux * 30.0 * sc, l.1 + uy * 30.0 * sc);
        let near_open = |g: &Game<GpuWorld>, p: (f32, f32), kind: Kind| {
            (0..=12).flat_map(|r| (0..12).map(move |k| (r, k))).map(|(r, k)| {
                let a = k as f32 / 12.0 * std::f32::consts::TAU;
                (p.0 + 0.5 * r as f32 * sc * a.cos(), p.1 + 0.5 * r as f32 * sc * a.sin())
            }).find(|q| g.build_blocked(kind, *q).is_none())
        };
        if let Some(at) = near_open(g, mid, Kind::Turret) {
            g.apply(Cmd::Build(Kind::Turret, at));
            g.apply(Cmd::Send(g.buildings.len() - 1));
        }
        let w = (l.0 + ux * 24.0 * sc, l.1 + uy * 24.0 * sc);
        let (a, b) = ((w.0 - uy * 4.0 * sc, w.1 + ux * 4.0 * sc), (w.0 + uy * 4.0 * sc, w.1 - ux * 4.0 * sc));
        if g.apply(Cmd::Wall(a, b)) {
            g.apply(Cmd::Send(g.buildings.len() - 1));
        }
    }
    let hand = match hand {
        "build" | "siren" | "far" | "sow" => "farm",
        h => h,
    };
    if g.time() > 0.5 && hand == "farm" && g.bank >= 1200.0 {
        g.apply(Cmd::Buy);
    }
    let cs = g.world.crystals();
    let target = cs.iter().enumerate().skip(1).filter(|c| c.1 .1 > 0.0).map(|(_, c)| c.0).min_by(|a, b| (a.0 - g.larder.0).hypot(a.1 - g.larder.1).total_cmp(&(b.0 - g.larder.0).hypot(b.1 - g.larder.1)));
    for k in 0..g.soldiers.len() {
        // New soldiers, and harvesters of a crystal gone dry, take a new one.
        let dry = match g.soldiers[k].order {
            Order::Clear(x, y) => g.crystal_at((x, y)).is_none_or(|(i, _)| cs[i].1 <= 0.0),
            _ => false,
        };
        if sowers.contains(&k) || !g.soldiers[k].alive || (g.soldiers[k].order != Order::Hold && !dry) {
            continue;
        }
        match (hand, target) {
            ("rush", _) => {
                g.apply(Cmd::Order(k, Order::Attack));
            }
            // Attack-move on the core: the squad stops where the mould comes
            // in range and fires into it from there (the owner, 11-12
            // 14:41Z, on the rush's clips: "I was more expecting soldiers
            // standing shooting into slime. Not running through slime
            // leaving craters under their feet").
            ("stand", _) => {
                let c = g.core;
                g.apply(Cmd::Order(k, Order::Fight(c.0, c.1)));
            }
            ("farm", Some(c)) => {
                g.apply(Cmd::Order(k, Order::Clear(c.0, c.1)));
            }
            _ => {}
        }
    }
}

/// The look mode's `squads` hand (`mode=squads`, the squad mode's film),
/// read off the game's own log, so it keeps no state of its own:
/// 1. eight soldiers bought into the controlled squad, sent as a squad to
///    the slimed crystal nearest the start (the anchor walks round walls);
/// 2. thick slime in range, or slime on a soldier: a stop and a volley at
///    the thickest of it (the aim follows it once a second) for 10 s;
/// 3. a split toward the nearest crystal with no slime on it: that half,
///    now controlled, goes to guard it, while the rest, let go, fight on
///    by themselves (weaker: the boost is the controlled squad's);
/// 4. 12 s later control goes back to the fighting half, which volleys
///    again.
pub(super) fn squads_hand(g: &mut Game<GpuWorld>) {
    let t = g.time();
    let tps = g.rules.tps as f32;
    let sc = g.world.scale();
    while t < 1.0 && g.bank >= g.rules.soldier_cost && g.alive() < 8 {
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    let Some(q) = g.active else { return };
    let Some(m) = g.middle(q) else { return };
    let when = |g: &Game<GpuWorld>, f: &dyn Fn(&Cmd) -> bool| g.log.iter().find(|(_, c)| f(c)).map(|(tk, _)| *tk as f32 / tps);
    let fired = when(g, &|c| matches!(c, Cmd::Fire(_, Some(_))));
    let split = when(g, &|c| matches!(c, Cmd::Split(..)));
    let back = split.and_then(|s| g.log.iter().find(|(tk, c)| matches!(c, Cmd::Active(_)) && *tk as f32 / tps > s).map(|(tk, _)| *tk as f32 / tps));
    // The thickest slime a member of `q` sees in range, and how thick.
    let sight = |g: &Game<GpuWorld>, q: usize| {
        g.members(q).iter().filter_map(|&k| g.soldiers[k].aim.map(|a| (g.soldiers[k].dense, a, (a.0 - g.soldiers[k].x).hypot(a.1 - g.soldiers[k].y)))).filter(|&(n, _, d)| n >= g.rules.aim_min && d <= g.gun_range(q) * sc).max_by_key(|&(n, ..)| n).map(|(n, a, _)| (a, n))
    };
    let cs = g.world.crystals();
    let slimed: Vec<bool> = (0..cs.len()).map(|i| g.world.walkers_in(cs[i].0, g.world.crystal_r(i) + 2.0 * sc) > 200).collect();
    let near = |from: (f32, f32), want: bool| cs.iter().enumerate().skip(1).filter(|&(i, c)| c.1 > 0.0 && slimed[i] == want).map(|(_, c)| c.0).min_by(|a, b| (a.0 - from.0).hypot(a.1 - from.1).total_cmp(&(b.0 - from.0).hypot(b.1 - from.1)));
    if std::env::var("SQ_DEBUG").is_ok() {
        eprintln!("t {t:.1} q {q} n {} mid {m:?} dest {:?} fire {:?} sight {:?} fired {fired:?} split {split:?} back {back:?}", g.members(q).len(), g.squads[q].dest, g.squads[q].fire, sight(g, q));
    }
    // A squad that never got to fire still splits by 20 s, so the film
    // shows two squads (and the squad panel's two rows) whatever held it.
    if fired.is_none() && split.is_none() && t >= 20.0 {
        let sc = g.world.scale();
        g.apply(Cmd::Split(q as u8, (m.0 + 10.0 * sc, m.1)));
        return;
    }
    match (fired, split, back) {
        (None, ..) if t >= 1.0 => {
            // Fire once sent and either arrived or bitten by more than a
            // straggler (the thin film near the start is no target).
            let touched = g.members(q).iter().map(|&k| g.soldiers[k].touching).sum::<u32>();
            let sent = g.log.iter().any(|(_, c)| matches!(c, Cmd::SquadOrder(..)));
            let arrived = sent && g.squads[q].dest.is_none();
            match sight(g, q).filter(|_| (arrived && t >= 8.0) || touched >= 12) {
                Some((a, _)) => {
                    g.apply(Cmd::SquadOrder(q as u8, Order::Move(m.0, m.1)));
                    g.apply(Cmd::Fire(q as u8, Some(a)));
                }
                None if g.squads[q].dest.is_none() => {
                    if let Some(c) = near(g.larder, true).or(Some(g.core)) {
                        g.apply(Cmd::SquadOrder(q as u8, Order::Move(c.0, c.1)));
                    }
                }
                None => {}
            }
        }
        (Some(f), None, _) => {
            if let Some((a, _)) = sight(g, q) {
                g.apply(Cmd::Fire(q as u8, Some(a)));
            }
            if t - f >= 10.0 {
                if let Some(c) = near(m, false) {
                    g.apply(Cmd::Split(q as u8, c));
                    if let Some(n) = g.active {
                        g.apply(Cmd::SquadOrder(n as u8, Order::Move(c.0, c.1)));
                    }
                }
            }
        }
        (_, Some(s), None) if t - s >= 12.0 => {
            // Back to the half that stayed to fight.
            if let Some(&o) = g.live_squads().iter().find(|&&o| o != q) {
                g.apply(Cmd::Active(o as u8));
            }
        }
        (_, Some(_), Some(_)) => {
            if let Some((a, _)) = sight(g, q) {
                g.apply(Cmd::Fire(q as u8, Some(a)));
            }
        }
        _ => {}
    }
}

/// The look mode's `squadwork` hand (`mode=squads`): guards, links and
/// queues (owner 2026-11-14 15:06Z).
/// 1. eight soldiers sent to drain the slimed crystal nearest them: with
///    slime near, a third stand guard on its side while the rest drain;
/// 2. at 30 s the squad splits, the halves are linked, and one order sends
///    both halfway to the nearest big crystal, with draining it queued.
pub(super) fn squadwork_hand(g: &mut Game<GpuWorld>) {
    let t = g.time();
    let sc = g.world.scale();
    while t < 1.0 && g.bank >= g.rules.soldier_cost && g.alive() < 8 {
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    let Some(q) = g.active else { return };
    let Some(m) = g.middle(q) else { return };
    let cs = g.world.crystals();
    // A big crystal (half the largest's charge or more), so the work lasts.
    let big = cs.iter().skip(1).map(|c| c.1).fold(0.0, f32::max);
    // The big crystal with the most slime within 30 cells, its own rim
    // still clear (a crystal under the body can't be drained).
    let pick = || {
        cs.iter()
            .enumerate()
            .skip(1)
            .filter(|&(i, c)| c.1 > 0.2 * big && g.world.walkers_in(c.0, g.world.crystal_r(i) + 2.0 * sc) < 50)
            .max_by_key(|&(_, c)| g.world.walkers_in(c.0, 30.0 * sc))
            .map(|(_, c)| c.0)
    };
    if std::env::var("SQ_DEBUG").is_ok() && t < 0.1 {
        for (i, c) in cs.iter().enumerate() {
            eprintln!("crystal {i} at {:?} charge {:.0} rim {} near {}", c.0, c.1, g.world.walkers_in(c.0, g.world.crystal_r(i) + 2.0 * sc), g.world.walkers_in(c.0, 30.0 * sc));
        }
    }
    let ordered = g.log.iter().any(|(_, c)| matches!(c, Cmd::SquadOrder(..)));
    let linked = g.log.iter().any(|(_, c)| matches!(c, Cmd::Link(..)));
    if t >= 1.0 && !ordered {
        if let Some(c) = pick() {
            if std::env::var("SQ_NOCOVER").is_ok() {
                g.apply(Cmd::Cover(q as u8, slime_mould::mould::larder_world::Cover::Never));
            }
            g.apply(Cmd::SquadOrder(q as u8, Order::Clear(c.0, c.1)));
        }
    } else if t >= 30.0 && !linked && g.apply(Cmd::Split(q as u8, (m.0 + 10.0 * sc, m.1))) {
        let n = g.active.unwrap();
        g.apply(Cmd::Link(q as u8, n as u8));
        // Halfway to the biggest crystal on this side of the wall, then
        // queued: drain it.
        let far = cs.iter().skip(1).filter(|c| c.1 > 0.5 * big).map(|c| c.0).min_by(|a, b| (a.0 - m.0).hypot(a.1 - m.1).total_cmp(&(b.0 - m.0).hypot(b.1 - m.1))).unwrap_or(m);
        let a = g.nearest_standable(((m.0 + far.0) / 2.0, (m.1 + far.1) / 2.0), 20.0 * sc).unwrap_or(m);
        g.apply(Cmd::SquadOrder(n as u8, Order::Move(a.0, a.1)));
        g.apply(Cmd::SquadQueue(n as u8, Order::Clear(far.0, far.1)));
    }
    if std::env::var("SQ_DEBUG").is_ok() {
        let guards = g.members(q).iter().filter(|&&k| g.soldiers[k].cover).count();
        eprintln!("t {t:.1} q {q} work {:?} guards {guards} queued {} linked {:?} healed {:.0}", g.squads[q].work, g.squads[q].queued, g.linked(q), g.healed);
        if (t - 31.0).abs() < 0.05 || (t - 36.0).abs() < 0.05 {
            for r in g.live_squads() {
                let k0 = g.members(r)[0];
                eprintln!("  squad {r} mid {:?} dest {:?} anchor {:?} steer {:?} work {:?} link {} k0 order {:?}", g.middle(r), g.squads[r].dest, g.squads[r].anchor, g.squads[r].steer, g.squads[r].work, g.squads[r].link, g.soldiers[k0].order);
            }
        }
        if (t - 20.0).abs() < 0.05 {
            for k in g.members(q) {
                let s = &g.soldiers[k];
                eprintln!("  k {k} at {:.0},{:.0} order {:?} cover {} queued {} hp {:.0}", s.x / sc, s.y / sc, s.order, s.cover, s.queued, s.hp);
            }
        }
    }
}

/// `pumps` (2026-11-15, the pump economy's clips): from 1 s the squad
/// pumps the crystals nearest it, smallest walk first, placing each pump
/// by a click on the crystal's rim (it snaps to the middle) and raising it
/// together; once a pump stands and the bank allows it is upgraded, up to
/// its top level, before the next is placed. No draining by hand.
fn pumps_hand(g: &mut Game<GpuWorld>) {
    if g.time() < 1.0 {
        return;
    }
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    let Some(&k0) = live.first() else { return };
    // A ghost: everyone raises it.
    if let Some(b) = g.buildings.iter().position(|b| b.alive && !b.up()) {
        for &k in &live {
            if g.soldiers[k].order != Order::Build(b) {
                g.apply(Cmd::Order(k, Order::Build(b)));
            }
        }
        return;
    }
    // A standing pump below its top level: upgrade it when the bank allows.
    if let Some(b) = (0..g.buildings.len()).find(|&b| g.buildings[b].kind == Kind::Pump && g.upgrade_price(b).is_some()) {
        g.apply(Cmd::Upgrade(b));
        return;
    }
    // The next crystal with charge and no pump, nearest the squad.
    let s = g.soldiers[k0];
    let cs = g.world.crystals();
    let next = (1..cs.len()).filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none()).min_by(|&a, &b| {
        let d = |c: usize| (cs[c].0 .0 - s.x).hypot(cs[c].0 .1 - s.y);
        d(a).total_cmp(&d(b))
    });
    if let Some(c) = next {
        // On its rim, toward the squad: the pump snaps to the middle.
        let (cx, cy) = cs[c].0;
        let d = (s.x - cx).hypot(s.y - cy).max(1.0);
        let r = g.world.crystal_r(c) * 0.9;
        g.apply(Cmd::Build(Kind::Pump, (cx + (s.x - cx) / d * r, cy + (s.y - cy) / d * r)));
    }
}

/// `greedy` (2026-11-15, the owner's opening: "I just run to the biggest
/// Crystal harvest the whole thing and I have a full squad of 40 within
/// the first 30 seconds"): every soldier drains the biggest crystal with
/// charge left, and a soldier is bought whenever the bank can pay.
fn greedy_hand(g: &mut Game<GpuWorld>) {
    while g.bank >= g.rules.soldier_cost && g.alive() < g.rules.max_squad {
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    let cs = g.world.crystals();
    let Some(c) = (1..cs.len()).filter(|&c| cs[c].1 > 0.0).max_by(|&a, &b| cs[a].1.total_cmp(&cs[b].1)).map(|c| cs[c].0) else { return };
    for k in 0..g.soldiers.len() {
        let on = matches!(g.soldiers[k].order, Order::Clear(x, y) if g.crystal_at((x, y)).is_some_and(|(i, _)| cs[i].1 > 0.0));
        if g.soldiers[k].alive && !on {
            g.apply(Cmd::Order(k, Order::Clear(c.0, c.1)));
        }
    }
}

/// `guardwork` (2026-11-15, the guards review): eight soldiers, one squad,
/// sent at 1 s to raise a pump (`GW_WORK=pump`, the default) or drain by
/// hand (`GW_WORK=drain`) at the slimed crystal `squadwork` picks, with
/// guards `GW_COVER` (none, some, more; the squad's default without it).
/// Prints a `GW` line a game second: guards, alive, health, the work's
/// progress, the slime killed and the guards' distance from the work.
fn guardwork_hand(g: &mut Game<GpuWorld>) {
    let t = g.time();
    let sc = g.world.scale();
    while t < 1.0 && g.bank >= g.rules.soldier_cost && g.alive() < 8 {
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    let Some(q) = g.active else { return };
    let cs = g.world.crystals();
    let big = cs.iter().skip(1).map(|c| c.1).fold(0.0, f32::max);
    let pick = cs
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(i, c)| c.1 > 0.2 * big && g.world.walkers_in(c.0, g.world.crystal_r(i) + 2.0 * sc) < 50)
        .max_by_key(|&(_, c)| g.world.walkers_in(c.0, 30.0 * sc));
    let pump = std::env::var("GW_WORK").map_or(true, |w| w != "drain");
    let ordered = g.log.iter().any(|(_, c)| matches!(c, Cmd::SquadOrder(..)));
    if t >= 1.0 && !ordered {
        let Some((i, c)) = pick else { return };
        if let Some(cv) = std::env::var("GW_COVER").ok().and_then(|v| slime_mould::mould::larder_world::Cover::parse(&v)) {
            g.apply(Cmd::Cover(q as u8, cv));
        }
        if pump {
            let Some(m) = g.middle(q) else { return };
            let d = (m.0 - c.0 .0).hypot(m.1 - c.0 .1).max(1.0);
            let r = g.world.crystal_r(i) * 0.9;
            g.bank = g.bank.max(5000.0);
            if g.apply(Cmd::Build(Kind::Pump, (c.0 .0 + (m.0 - c.0 .0) / d * r, c.0 .1 + (m.1 - c.0 .1) / d * r))) {
                let b = g.buildings.len() - 1;
                g.apply(Cmd::SquadOrder(q as u8, Order::Build(b)));
            }
        } else {
            g.apply(Cmd::SquadOrder(q as u8, Order::Clear(c.0 .0, c.0 .1)));
        }
        eprintln!("GW pick crystal {i} at {:.0},{:.0} pump {pump}", c.0 .0 / sc, c.0 .1 / sc);
    }
    if (t - t.round()).abs() < 0.02 && t >= 1.0 {
        let m = g.members(q);
        let at = g.squads[q].work.and_then(|o| match o {
            Order::Clear(x, y) => Some((x, y)),
            Order::Build(b) => g.buildings.get(b).map(|b| b.at),
            _ => None,
        });
        let guards: Vec<usize> = m.iter().copied().filter(|&k| g.soldiers[k].cover).collect();
        let dist: Vec<String> = guards.iter().map(|&k| at.map_or("-".into(), |a| format!("{:.0}", (g.soldiers[k].x - a.0).hypot(g.soldiers[k].y - a.1) / sc))).collect();
        let bitten = m.iter().filter(|&&k| g.soldiers[k].touching > 0).count();
        let gbitten = guards.iter().filter(|&&k| g.soldiers[k].touching > 0).count();
        let hp: f32 = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).map(|k| g.soldiers[k].hp).sum();
        let built = g.buildings.first().map_or(0.0, |b| b.built);
        eprintln!(
            "GW t {t:.0} work {} guards {} dist [{}] alive {} hp {hp:.0} bitten {bitten} gbitten {gbitten} built {built:.2} harvested {:.0} killed {}",
            at.is_some(),
            guards.len(),
            dist.join(","),
            g.alive(),
            g.harvested,
            g.killed
        );
    }
}

/// `hand=guard` (squad mode; the command card's Guard, 2026-11-15): eight
/// soldiers, then at 1 s the squad is told to guard the slimed crystal
/// nearest it (the nearest crystal if none is slimed), and holds there on
/// Defend, so the film shows the zone it keeps and what it shoots in it.
pub(super) fn guard_hand(g: &mut Game<GpuWorld>) {
    let t = g.time();
    let sc = g.world.scale();
    while t < 1.0 && g.bank >= g.rules.soldier_cost && g.alive() < 8 {
        if !g.apply(Cmd::Buy) {
            break;
        }
    }
    let Some(q) = g.active else { return };
    // `GUARD_MOVE=1`: the same crystal by a plain move (the right click
    // before Guard), for the paired check; the zone's walkers each second.
    let plain = std::env::var("GUARD_MOVE").is_ok();
    let sent = g.log.iter().find_map(|(_, c)| match c {
        Cmd::SquadOrder(_, Order::Guard(x, y) | Order::Move(x, y)) => Some((*x, *y)),
        _ => None,
    });
    if let Some(p) = sent {
        let (m, r) = g.zone_at(p);
        println!("  guard zone t {t:.0} s: {} walkers in it, {} killed", g.world.walkers_in(m, r), g.killed);
        return;
    }
    if t < 1.0 {
        return;
    }
    let Some(m) = g.middle(q) else { return };
    let cs = g.world.crystals();
    let d = |c: (f32, f32)| (c.0 - m.0).hypot(c.1 - m.1);
    let slimed = |i: usize| g.world.walkers_in(cs[i].0, g.world.crystal_r(i) + 4.0 * sc) > 50;
    let pick = (0..cs.len()).filter(|&i| cs[i].1 > 0.0 && slimed(i)).min_by(|&a, &b| d(cs[a].0).total_cmp(&d(cs[b].0))).or_else(|| (0..cs.len()).filter(|&i| cs[i].1 > 0.0).min_by(|&a, &b| d(cs[a].0).total_cmp(&d(cs[b].0))));
    if let Some(i) = pick {
        let c = cs[i].0;
        println!("  guard t {t:.1} s: squad {} guards crystal {i} at {:.0},{:.0} ({:.0} cells off)", q + 1, c.0, c.1, d(c) / sc);
        g.apply(Cmd::SquadOrder(q as u8, if plain { Order::Move(c.0, c.1) } else { Order::Guard(c.0, c.1) }));
    }
}

const PEN_AT: f32 = 0.3;
const PEN_T: f32 = 12.0;
const PEN_R: f32 = 12.0;
const PEN_SIDES: usize = 16;
const DOOR_T: f32 = 15.0;
const OUT_T: f32 = 18.0;
const IN_T: f32 = 34.0;
const THICK_T: f32 = 42.0;
const FORT_T: f32 = 13.0;

/// The walls thread's film (2026-11-16): ten soldiers walk out and a ring
/// of 16 walls stands round them at `PEN_T`. With `door` (`hand=door`),
/// every segment's home side is the inside, a door is cut in the segment
/// facing the core (`DOOR_T`), the squad goes out through it (`OUT_T`) and
/// back (`IN_T`), and the door's segment and two either side are
/// thickened twice (`THICK_T`, to three bands).
fn pen_hand(g: &mut Game<GpuWorld>, door: bool, fort: bool) {
    let t = g.time();
    while t < 1.0 && g.soldiers.len() < 10 && g.bank >= g.rules.soldier_cost {
        g.apply(Cmd::Buy);
    }
    let sc = g.world.scale();
    let (c, l) = (g.core, g.larder);
    let d = (c.0 - l.0).hypot(c.1 - l.1).max(1.0);
    let u = ((c.0 - l.0) / d, (c.1 - l.1) / d);
    let spot = (l.0 + u.0 * d * PEN_AT, l.1 + u.1 * d * PEN_AT);
    let spot = g.nearest_standable(spot, 20.0 * sc).unwrap_or(spot);
    if t > 1.5 && t < 2.5 {
        g.apply(Cmd::SquadOrder(0, Order::Move(spot.0, spot.1)));
    }
    let r = PEN_R * sc;
    let half = g.rules.wall_width * sc / 2.0;
    if t > PEN_T && !g.buildings.iter().any(|b| matches!(b.kind, Kind::Wall { .. })) {
        g.bank = g.bank.max(1e6);
        g.rules.build.anywhere = true;
        let m = g.middle(0).unwrap_or(spot);
        let rr = r + half;
        for q in 0..PEN_SIDES {
            let (a0, a1) = (q as f32 / PEN_SIDES as f32 * std::f32::consts::TAU, (q + 1) as f32 / PEN_SIDES as f32 * std::f32::consts::TAU);
            let (a, b) = ((m.0 + rr * a0.cos(), m.1 + rr * a0.sin()), (m.0 + rr * a1.cos(), m.1 + rr * a1.sin()));
            if g.apply(Cmd::Wall(a, b)) {
                let w = g.buildings.len() - 1;
                g.raise(w, 1.0);
            } else {
                println!("  pen: wall {q} refused: {:?}", g.build_blocked(Kind::Wall { to: b }, a));
            }
        }
        if door {
            for b in g.buildings.iter_mut() {
                if let Kind::Wall { to } = b.kind {
                    let mid = ((b.at.0 + to.0) / 2.0, (b.at.1 + to.1) / 2.0);
                    let (dx, dy) = (m.0 - mid.0, m.1 - mid.1);
                    let l = dx.hypot(dy).max(1e-3);
                    b.home = (dx / l, dy / l);
                }
            }
        }
        g.apply(Cmd::SquadOrder(0, Order::Move(m.0, m.1)));
        PEN.with(|p| p.set(Some(m)));
    }
    let Some(m) = PEN.with(|p| p.get()) else { return };
    let rr = r + half;
    // The segments by how squarely they face the core, nearest first.
    let facing = |g: &Game<GpuWorld>| {
        let mut v: Vec<(usize, f32)> = (0..g.buildings.len())
            .filter_map(|b| match g.buildings[b].kind {
                Kind::Wall { to } if !g.buildings[b].door => {
                    let at = g.buildings[b].at;
                    let mid = ((at.0 + to.0) / 2.0 - m.0, (at.1 + to.1) / 2.0 - m.1);
                    Some((b, -(mid.0 * u.0 + mid.1 * u.1) / mid.0.hypot(mid.1).max(1e-3)))
                }
                _ => None,
            })
            .collect();
        v.sort_by(|a, b| a.1.total_cmp(&b.1));
        v.into_iter().map(|(b, _)| b).collect::<Vec<usize>>()
    };
    // Where a sortie through segment `b` goes: 10 cells out from its middle.
    let sortie = |g: &Game<GpuWorld>, b: usize| {
        let Kind::Wall { to } = g.buildings[b].kind else { unreachable!() };
        let at = g.buildings[b].at;
        let (mid, n) = (((at.0 + to.0) / 2.0, (at.1 + to.1) / 2.0), g.buildings[b].home);
        (mid.0 - n.0 * 10.0 * sc, mid.1 - n.1 * 10.0 * sc)
    };
    // The door goes in the segment most squarely facing the core with
    // ground on both sides (on planet 1 seed 7 the ring stands half in a
    // lake, and a door onto deep water leads nowhere).
    if door && t > DOOR_T && !g.buildings.iter().any(|b| b.door) {
        let pick = facing(g).into_iter().find(|&b| {
            let o = sortie(g, b);
            let n = g.buildings[b].home;
            let Kind::Wall { to } = g.buildings[b].kind else { return false };
            let at = g.buildings[b].at;
            let mid = ((at.0 + to.0) / 2.0, (at.1 + to.1) / 2.0);
            let (near_out, near_in) = ((mid.0 - n.0 * 4.0 * sc, mid.1 - n.1 * 4.0 * sc), (mid.0 + n.0 * 4.0 * sc, mid.1 + n.1 * 4.0 * sc));
            g.standable(o.0, o.1) && g.standable(near_out.0, near_out.1) && g.standable(near_in.0, near_in.1)
        });
        if let Some(b) = pick {
            let Kind::Wall { to } = g.buildings[b].kind else { unreachable!() };
            let at = g.buildings[b].at;
            let p = |f: f32| (at.0 + (to.0 - at.0) * f, at.1 + (to.1 - at.1) * f);
            let ok = g.apply(Cmd::Door(p(0.15), p(0.85)));
            println!("  door t {t:.1}: door cut in wall {b}: {ok}");
            DOOR_IN.with(|d| d.set(Some(b)));
        }
    }
    if door && (OUT_T..OUT_T + 1.0).contains(&t) {
        if let Some(b) = DOOR_IN.with(|d| d.get()) {
            let o = sortie(g, b);
            g.apply(Cmd::SquadOrder(0, Order::Move(o.0, o.1)));
        }
    }
    if door && (IN_T..IN_T + 1.0).contains(&t) {
        g.apply(Cmd::SquadOrder(0, Order::Move(m.0, m.1)));
    }
    if door && t > THICK_T && g.buildings.iter().all(|b| b.level < 3) {
        // The door's wall and the four beside it, each side.
        let Some(d) = DOOR_IN.with(|d| d.get()) else { return };
        let near: Vec<usize> = (0..PEN_SIDES).filter(|&q| (q as i32 - d as i32).rem_euclid(PEN_SIDES as i32) <= 2 || (d as i32 - q as i32).rem_euclid(PEN_SIDES as i32) <= 2).collect();
        for b in near {
            let ok = g.apply(Cmd::Upgrade(b)) && g.apply(Cmd::Upgrade(b));
            println!("  door t {t:.1}: wall {b} thickened to level {}: {ok}", g.buildings[b].level);
        }
    }
    if fort && t > FORT_T && !g.buildings.iter().any(|b| b.kind == Kind::Turret) {
        // Three turrets 6 cells in from the ring's middle, toward the
        // core and 60 degrees either side.
        let a0 = u.1.atan2(u.0);
        for da in [0.0f32, 1.05, -1.05] {
            let a = a0 + da;
            let p = (m.0 + 6.0 * sc * a.cos(), m.1 + 6.0 * sc * a.sin());
            let p = g.nearest_standable(p, 3.0 * sc).unwrap_or(p);
            let ok = g.apply(Cmd::Build(Kind::Turret, p));
            if let Some(k) = ok.then(|| g.buildings.len() - 1) {
                g.raise(k, 1.0);
            }
            println!("  fort t {t:.1}: turret at {p:?}: {ok}");
        }
    }
    if fort {
        // The hand runs once a game second, so it reads the game's
        // counters: kills by the turrets and by the soldiers' guns.
        let soldiers = g.killed - g.turret_killed - g.fire_killed;
        println!("  fort t {t:.0}: shots {}, turret kills {}, soldier kills {soldiers}", g.shots, g.turret_killed);
    }
    let (w, h) = g.world.size();
    let open = g.world.open_cells();
    let ring = |i: usize| {
        let (x, y) = ((i % w) as f32 + 0.5 - m.0, (i / w) as f32 + 0.5 - m.1);
        let rad = x.hypot(y);
        // With `door` the thickened bands too, inside the ring.
        let inner = if door { r - sc - 2.0 * g.rules.wall_width * sc } else { r - sc };
        rad > inner && rad < r + 2.0 * half + 2.0 * sc
    };
    let wall = (0..w * h).filter(|&i| ring(i) && !open[i]).count();
    let bin = 4;
    let inside = g.world.mould_grid(bin).map_or(0, |gr| {
        let gw = w.div_ceil(bin);
        gr.iter().enumerate().filter(|&(i, _)| {
            let (x, y) = (((i % gw) * bin) as f32 + 2.0 - m.0, ((i / gw) * bin) as f32 + 2.0 - m.1);
            x.hypot(y) < r - sc
        }).map(|(_, &n)| n as usize).sum()
    });
    let _ = h;
    let out = g.soldiers.iter().filter(|s| s.alive && (s.x - m.0).hypot(s.y - m.1) > rr + half).count();
    println!("  pen t {t:.0}: wall cells {wall}, walkers inside {inside}, soldiers {}, outside {out}", g.alive());
}

thread_local! {
    static PEN: std::cell::Cell<Option<(f32, f32)>> = const { std::cell::Cell::new(None) };
    static DOOR_IN: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}
