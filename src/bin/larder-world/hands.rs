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
    let keep = 0.6 * g.rules.range * sc;
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
    if hand == "volley" || hand == "edge" {
        volley(g, hand == "edge");
        return;
    }
    // `hold` and `push` (2026-11-13, the wading and arming clips; the owner,
    // 15:55Z: "this is really hard to judge when the soliders are just
    // running into the slime"): eight soldiers, bought at once with 400
    // food kept for rations, either stand at the larder on free fire as
    // the mould comes up the stem, or walk into the body from 3 s.
    // `hold@X,Y` stands them at (X, Y) instead, world units as `centre=`.
    let post = hand.strip_prefix("hold@").and_then(|p| p.split_once(',')).and_then(|(x, y)| Some((x.parse::<f32>().ok()?, y.parse::<f32>().ok()?)));
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
            if let (Some(p), true) = (post, g.time() >= 1.0 && g.soldiers[k].order == Order::Hold) {
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
