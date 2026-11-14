//! **Scripted building plans** for the Larder world: the census's stand-ins
//! for a player who builds (turrets by the farmed crystals, pumps on them, a
//! ring of turrets, a kill zone) and the campaign missions' own plans
//! (cutting the Trunk's cords). Moved here from `examples/larderworld.rs`
//! on 2026-11-14 (campaign thread, cloud plan C3) so the campaign census
//! (`examples/campaign.rs`, hands `PLAN+STYLE`) and the world census play
//! the same plans; the bodies are unchanged but for the reserve, which the
//! caller now passes (the world census's farm hand keeps 600, the player
//! AI's styles 400).
//!
//! A plan only adds buildings and a few soldiers' orders on top of a hand
//! that farms (`larder_world_ai::Hand`): call the hand first, then the plan,
//! once a game second, so the plan's orders are the ones that stand.

use super::campaign::BIN;
use super::larder_world::{Cmd, Game, Kind, Order, World};

/// Ring plan: turrets `RING` dish cells from the larder, `RING_N` of them.
pub const RING: f32 = 7.0;
pub const RING_N: usize = 6;
/// Kill zone plan: two turrets either side of the larder-to-core road half
/// `ZONE` dish cells out and a lure at `ZONE`.
pub const ZONE: f32 = 13.0;

/// Crystals with charge, nearest the larder first (the core last of all).
pub fn by_distance<W: World>(g: &Game<W>) -> Vec<(f32, f32)> {
    let l = g.larder;
    // A crystal under one frame's full ask is dry: the soldier the game
    // sent home from it (`Rules::home_dry`) is not sent back for crumbs.
    let r = g.rules;
    let dry = r.harvest * r.frame_ticks as f32 / r.tps as f32 * r.charge;
    let mut cs: Vec<(f32, f32)> = g.world.crystals().iter().filter(|c| c.1 >= dry).map(|c| c.0).filter(|c| *c != g.core).collect();
    cs.sort_by(|a, b| (a.0 - l.0).hypot(a.1 - l.1).total_cmp(&(b.0 - l.0).hypot(b.1 - l.1)));
    cs
}

/// A building of `kind` at `at`, if the bank holds its price over `reserve`
/// and it can go there now.
pub fn spare_build<W: World>(g: &mut Game<W>, kind: Kind, at: (f32, f32), reserve: f32) -> bool {
    let ok = g.bank >= g.build_price(kind, at) + reserve && g.build_blocked(kind, at).is_none() && g.apply(Cmd::Build(kind, at));
    if ok {
        send_builder(g);
    }
    ok
}

/// The newest building, if still a ghost (`BuildRules::raise` > 0): the
/// nearest live soldier goes to raise it (`Order::Build`), as a player
/// would send one; the farm hand leaves it be until it stands.
pub fn send_builder<W: World>(g: &mut Game<W>) {
    let Some(b) = g.buildings.len().checked_sub(1) else { return };
    let bd = g.buildings[b];
    if bd.built >= 1.0 {
        return;
    }
    let d = |k: usize| (g.soldiers[k].x - bd.at.0).hypot(g.soldiers[k].y - bd.at.1);
    if let Some(k) = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).min_by(|&a, &b| d(a).total_cmp(&d(b))) {
        g.apply(Cmd::Order(k, Order::Build(b)));
    }
}

/// An open cell within 3 dish cells of `p`, nearest first.
pub fn open_by<W: World>(g: &Game<W>, p: (f32, f32)) -> Option<(f32, f32)> {
    let s = g.world.scale();
    (0..=6).flat_map(|r| (0..12).map(move |k| (r, k))).map(|(r, k)| {
        let a = k as f32 / 12.0 * std::f32::consts::TAU;
        (p.0 + 0.5 * r as f32 * s * a.cos(), p.1 + 0.5 * r as f32 * s * a.sin())
    }).find(|q| g.world.open(q.0, q.1))
}

pub fn alive_kind<W: World>(g: &Game<W>, kind: Kind) -> usize {
    g.buildings.iter().filter(|b| b.alive && b.kind == kind).count()
}

/// Up to `RING_N` turrets round the larder, the first facing the core.
pub fn ring<W: World>(g: &mut Game<W>, reserve: f32) {
    if g.time() < 5.0 || alive_kind(g, Kind::Turret) >= RING_N {
        return;
    }
    let s = g.world.scale();
    let (c, l) = (g.core, g.larder);
    let a0 = (c.1 - l.1).atan2(c.0 - l.0);
    // The first faces the core, then alternately either side of it.
    for i in 0..RING_N {
        let k = (i as f32 / 2.0).ceil() * if i % 2 == 0 { 1.0 } else { -1.0 };
        let a = a0 + k * std::f32::consts::TAU / RING_N as f32;
        if let Some(at) = open_by(g, (l.0 + RING * s * a.cos(), l.1 + RING * s * a.sin())) {
            if spare_build(g, Kind::Turret, at, reserve) {
                return;
            }
        }
    }
}

/// A pump on each crystal a soldier is clearing (`pumps`), and a turret 4
/// dish cells core-side of each (`turrets`). The turtle's two halves can be
/// played alone: `pumps` (pumps, no turrets) and `guard` (turrets, no
/// pumps).
pub fn turtle<W: World>(g: &mut Game<W>, pumps: bool, turrets: bool, reserve: f32) {
    let s = g.world.scale();
    let sites: Vec<(f32, f32)> = g.soldiers.iter().filter(|q| q.alive).filter_map(|q| match q.order {
        Order::Clear(x, y) => g.crystal_at((x, y)).map(|(_, c)| c),
        _ => None,
    }).collect();
    for c in sites {
        let pumped = g.buildings.iter().any(|b| b.alive && b.kind == Kind::Pump && (b.at.0 - c.0).hypot(b.at.1 - c.1) < 3.0 * s);
        if pumps && !pumped {
            if let Some(at) = open_by(g, c) {
                if spare_build(g, Kind::Pump, at, reserve) {
                    return;
                }
            }
            continue;
        }
        let guarded = g.buildings.iter().any(|b| b.alive && b.kind == Kind::Turret && (b.at.0 - c.0).hypot(b.at.1 - c.1) < 6.0 * s);
        if turrets && !guarded {
            let (dx, dy) = (g.core.0 - c.0, g.core.1 - c.1);
            let d = dx.hypot(dy).max(1.0);
            if let Some(at) = open_by(g, (c.0 + dx / d * 4.0 * s, c.1 + dy / d * 4.0 * s)) {
                if spare_build(g, Kind::Turret, at, reserve) {
                    return;
                }
            }
        }
    }
}

/// Two turrets either side of the larder-to-core road and a lure past
/// them: the mould is drawn into crossing fire.
pub fn zone<W: World>(g: &mut Game<W>, reserve: f32) {
    if g.time() < 5.0 {
        return;
    }
    let s = g.world.scale();
    let (c, l) = (g.core, g.larder);
    let (dx, dy) = (c.0 - l.0, c.1 - l.1);
    let d = dx.hypot(dy).max(1.0);
    let (ux, uy) = (dx / d, dy / d);
    let half = 0.5 * ZONE * s;
    let wants = [
        (Kind::Turret, (l.0 + ux * half - uy * 4.0 * s, l.1 + uy * half + ux * 4.0 * s)),
        (Kind::Turret, (l.0 + ux * half + uy * 4.0 * s, l.1 + uy * half - ux * 4.0 * s)),
        (Kind::Lure, (l.0 + ux * ZONE * s, l.1 + uy * ZONE * s)),
    ];
    for (kind, p) in wants {
        let have = g.buildings.iter().any(|b| b.alive && b.kind == kind && (b.at.0 - p.0).hypot(b.at.1 - p.1) < 3.0 * s);
        if have {
            continue;
        }
        if let Some(at) = open_by(g, p) {
            if spare_build(g, kind, at, reserve) {
                return;
            }
        }
        // In order: the turrets before the lure.
        return;
    }
}

/// How far out from a farmed crystal's edge the cut lies, and its length,
/// dish cells.
pub const CUT_OUT: f32 = 6.0;
pub const CUT_LEN: f32 = 12.0;

/// **Cut the cords** (mission 4's plan): for each crystal a soldier is
/// clearing, a cut across the line from it to the core, `CUT_OUT` past the
/// crystal's edge, kept burning (`Order::Burn`) by one of the `cutters`
/// newest live soldiers while mould lies on it. The body on a farmed
/// crystal then hears no news of food from the core's body and, with the
/// one-body keys (`energy_flow`, `wither`), starves while the farmer takes
/// its food. A cutter with no cord to cut is left to the hand.
pub fn cut<W: World>(g: &mut Game<W>, cutters: usize) {
    let s = g.world.scale();
    let core = g.core;
    let mut cords: Vec<((f32, f32), (f32, f32))> = Vec::new();
    for q in g.soldiers.iter().filter(|q| q.alive) {
        let Order::Clear(x, y) = q.order else { continue };
        let Some((i, c)) = g.crystal_at((x, y)) else { continue };
        let (dx, dy) = (core.0 - c.0, core.1 - c.1);
        let d = dx.hypot(dy);
        let out = g.world.crystal_r(i) + CUT_OUT * s;
        if d < out + 2.0 * s {
            continue;
        }
        let (ux, uy) = (dx / d, dy / d);
        let m = (c.0 + ux * out, c.1 + uy * out);
        let h = CUT_LEN * s / 2.0;
        let seg = ((m.0 - uy * h, m.1 + ux * h), (m.0 + uy * h, m.1 - ux * h));
        if !cords.contains(&seg) {
            cords.push(seg);
        }
    }
    // Only cords with mould on them: a clean line needs no cutter.
    if let Some(grid) = g.world.mould_grid(BIN) {
        let gw = g.world.size().0.div_ceil(BIN);
        let on = |p: (f32, f32)| grid.get((p.1 as usize / BIN) * gw + p.0 as usize / BIN).is_some_and(|&n| n > 0);
        cords.retain(|&(a, b)| (0..=8).any(|k| {
            let t = k as f32 / 8.0;
            on((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t))
        }));
    }
    let live: Vec<usize> = (0..g.soldiers.len()).rev().filter(|&k| g.soldiers[k].alive && !matches!(g.soldiers[k].order, Order::Build(_))).take(cutters).collect();
    for (k, seg) in live.into_iter().zip(cords) {
        if g.soldiers[k].order != Order::Burn(seg.0, seg.1) {
            g.apply(Cmd::Order(k, Order::Burn(seg.0, seg.1)));
        }
    }
}

/// How many soldiers sow (`sow`), and the bank a sower keeps before it
/// spends on Feed (below it, it lays a road, which is free).
pub const SOWERS: usize = 2;
pub const SOW_KEEP: f32 = 600.0;

/// **Sow your mould as a moving front** (mission 8's first plan, after
/// `examples/garden.rs`'s sow2): the `sowers` oldest live soldiers walk
/// from the larder to the nearest crystal with charge that has no enemy on
/// it and back, on Feed while the bank is over `SOW_KEEP` (charge sown thin
/// along the way, which the player's mould eats and grows on) and Road
/// below it (free; the mould follows it). A sower's leg is read off its
/// order, so the plan keeps no state: at its target it turns for home, at
/// home for the target, and any other order (the farming hand's) sends it
/// out. Only where the player has a mould (`World::own_living`).
pub fn sow<W: World>(g: &mut Game<W>, sowers: usize) {
    use super::own_mould::Task;
    if g.world.own_living() == 0 {
        return;
    }
    let s = g.world.scale();
    let home = g.larder;
    let grid = g.world.mould_grid(BIN);
    let gw = g.world.size().0.div_ceil(BIN);
    let clean = |c: (f32, f32), r: f32| match &grid {
        Some(gr) => {
            let (bx0, bx1) = (((c.0 - r) / BIN as f32).max(0.0) as usize, ((c.0 + r) / BIN as f32) as usize);
            let (by0, by1) = (((c.1 - r) / BIN as f32).max(0.0) as usize, ((c.1 + r) / BIN as f32) as usize);
            !(by0..=by1).any(|by| (bx0..=bx1.min(gw - 1)).any(|bx| gr.get(by * gw + bx).is_some_and(|&n| n > 0)))
        }
        None => true,
    };
    let cs = g.world.crystals();
    let target = by_distance(g).into_iter().filter(|&c| (c.0 - home.0).hypot(c.1 - home.1) > 4.0 * s).find(|&c| {
        let i = cs.iter().position(|q| q.0 == c).unwrap_or(0);
        clean(c, g.world.crystal_r(i))
    });
    let Some(to) = target else { return };
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive && !matches!(g.soldiers[k].order, Order::Build(_))).take(sowers).collect();
    let task = if g.bank > SOW_KEEP { Task::Feed } else { Task::Road };
    for k in live {
        let q = g.soldiers[k];
        let at = |p: (f32, f32)| (q.x - p.0).hypot(q.y - p.1) < 3.0 * s;
        let next = match q.order {
            Order::Move(x, y) if (x, y) == home && !at(home) => home,
            Order::Move(x, y) if (x, y) == home => to,
            Order::Move(x, y) if !at((x, y)) => (x, y),
            Order::Move(..) => home,
            _ => to,
        };
        if q.order != Order::Move(next.0, next.1) {
            g.apply(Cmd::Order(k, Order::Move(next.0, next.1)));
        }
        g.world.set_task(k, task);
    }
}

/// The bait plan's lures over a game: one standing at a time, and no more
/// than this many built (each is a price paid).
pub const BAIT_MAX: usize = 3;

/// **Bait the flood away** (mission 8's second plan; the designer's note,
/// 11-14: "lure the flood away from your front while your mould takes the
/// valley"): `sow`, and one lure on open ground with no mould on it, round
/// the enemy's core (12 to 24 dish cells out) on the side farthest from
/// the larder, so the enemy's food scent pulls it home and away from the
/// player's front. The lure goes before the hand's reserve: kept over it
/// (400 for boom) none was ever built (census v3). The first version stood
/// it 6 cells past the core on the larder's line, inside the enemy: it was
/// eaten as it went up, 300 food each time, and the bank sat at 100 for ten
/// minutes (census v4, seed 1).
pub fn bait<W: World>(g: &mut Game<W>, sowers: usize) {
    sow(g, sowers);
    let lures = g.buildings.iter().filter(|b| b.kind == Kind::Lure).count();
    if g.world.own_living() == 0 || g.time() < 5.0 || alive_kind(g, Kind::Lure) > 0 || lures >= BAIT_MAX {
        return;
    }
    let s = g.world.scale();
    let (c, l) = (g.core, g.larder);
    let grid = g.world.mould_grid(BIN);
    let gw = g.world.size().0.div_ceil(BIN);
    let clean = |p: (f32, f32)| match &grid {
        Some(gr) => {
            let r = 3.0 * s;
            let (bx0, bx1) = (((p.0 - r) / BIN as f32).max(0.0) as usize, ((p.0 + r) / BIN as f32) as usize);
            let (by0, by1) = (((p.1 - r) / BIN as f32).max(0.0) as usize, ((p.1 + r) / BIN as f32) as usize);
            !(by0..=by1).any(|by| (bx0..=bx1.min(gw - 1)).any(|bx| gr.get(by * gw + bx).is_some_and(|&n| n > 0)))
        }
        None => true,
    };
    let spots = (0..16).flat_map(|k| [12.0f32, 18.0, 24.0].map(move |r| (k, r))).map(|(k, r)| {
        let a = k as f32 / 16.0 * std::f32::consts::TAU;
        (c.0 + r * s * a.cos(), c.1 + r * s * a.sin())
    });
    let far = |p: &(f32, f32)| (p.0 - l.0).hypot(p.1 - l.1);
    let best = spots.filter(|&p| g.world.open(p.0, p.1) && clean(p)).max_by(|a, b| far(a).total_cmp(&far(b)));
    if let Some(at) = best {
        spare_build(g, Kind::Lure, at, 0.0);
    }
}

/// A plan by name, for the censuses' `PLAN+STYLE` hands: `ring`, `pumps`,
/// `guard`, `turtle`, `zone`, `cut` (three cutters), `sow` (two sowers),
/// `bait` (sow and a lure behind the enemy), or None.
pub fn named(name: &str) -> Option<Plan> {
    Some(match name {
        "ring" => Plan::Ring,
        "pumps" => Plan::Turtle { pumps: true, turrets: false },
        "guard" => Plan::Turtle { pumps: false, turrets: true },
        "turtle" => Plan::Turtle { pumps: true, turrets: true },
        "zone" => Plan::Zone,
        "cut" => Plan::Cut(3),
        "sow" => Plan::Sow(SOWERS),
        "bait" => Plan::Bait(SOWERS),
        _ => return None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Plan {
    Ring,
    Turtle { pumps: bool, turrets: bool },
    Zone,
    Cut(usize),
    Sow(usize),
    Bait(usize),
}

impl Plan {
    /// Its turn: once a game second, after the farming hand's.
    pub fn play<W: World>(self, g: &mut Game<W>, reserve: f32) {
        match self {
            Plan::Ring => ring(g, reserve),
            Plan::Turtle { pumps, turrets } => turtle(g, pumps, turrets, reserve),
            Plan::Zone => zone(g, reserve),
            Plan::Cut(n) => cut(g, n),
            Plan::Sow(n) => sow(g, n),
            Plan::Bait(n) => bait(g, n),
        }
    }
}
