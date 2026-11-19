//! **The utility bot** (2026-11-19, the strategy thread; the owner, 04:30Z:
//! "Is there anything in between a full neural network that needs
//! millions of games to learn and fully scripted behavior?").
//!
//! `hand=util:WEIGHTS`. Where the strategy bot (`strat.rs`) works down a
//! fixed to-do list, this one scores every choice it has each game second
//! and takes the best: for the bank a soldier, a pump on the best crystal,
//! a turret, a lure or a wall piece by the pump that needs it most, a pump
//! upgrade, or saving; for the squad falling back, raising a ghost,
//! guarding a pump, draining a crystal, or hunting one of the heaviest
//! blobs. Each score is a weighted sum of what it reads off the game
//! (slime by each pump, the pump's draw, the squad's size and health, how
//! thick the slime is where it would go). The weights (`WEIGHTS`) are what
//! a search learns (`league.py breed`); `util:` alone plays the defaults.
//!
//! It plays through `Game::apply` as the window does, builds only what
//! the run has unlocked (`own`, which the league fills), and changes no
//! number of the game's.

use super::strat::{dist, spot, Plan, Sense};
use super::*;
use std::cell::RefCell;

/// Every weight: its key, its default and what it weighs. A score is the
/// base weight of its kind plus each term's weight times the term.
pub(super) const WEIGHTS: [(&str, f32, &str); 41] = [
    // The bank.
    ("sol", 1.0, "a soldier"),
    ("sol_few", 2.0, "a soldier: 1 - soldiers / 40"),
    ("sol_threat", 1.0, "a soldier: slime pressing the squad"),
    ("sol_late", 0.5, "a soldier: game minutes / 10"),
    ("pump", 1.2, "a pump"),
    ("pump_val", 1.0, "a pump: its crystal's worth near home"),
    ("pump_have", 1.0, "a pump: minus pumps standing / 4"),
    ("pump_risk", 2.0, "a pump: minus slime by its crystal"),
    ("tur", 0.6, "a turret"),
    ("tur_threat", 1.5, "a turret: slime by the pump"),
    ("tur_draw", 0.5, "a turret: the pump's draw"),
    ("tur_have", 1.5, "a turret: minus turrets by the pump / 4"),
    ("lure", 0.3, "a lure past a pump's turrets"),
    ("lure_tur", 1.0, "a lure: turrets by the pump / 4"),
    ("wall", 0.2, "a wall piece in front of a pump"),
    ("wall_threat", 1.0, "a wall piece: slime by the pump"),
    ("wall_have", 2.0, "a wall piece: minus pieces by the pump / 4"),
    ("up", 0.5, "a pump upgrade"),
    ("up_draw", 1.0, "an upgrade: the pump's draw"),
    ("save", 0.8, "the bar any buy must clear"),
    // The squad.
    ("back", 3.0, "fall back: (1 - health) while bitten"),
    ("raise", 1.5, "raise a ghost"),
    ("raise_far", 1.0, "raise: minus the walk"),
    ("guard", 0.3, "guard a pump"),
    ("guard_threat", 2.0, "guard: slime by the pump"),
    ("guard_far", 0.5, "guard: minus the walk"),
    ("guard_few", 1.0, "guard: 1 - soldiers / 40"),
    ("drain", 0.8, "drain a crystal by hand"),
    ("drain_val", 1.0, "drain: the crystal's charge"),
    ("drain_far", 1.0, "drain: minus the walk"),
    ("drain_risk", 1.5, "drain: minus slime there a soldier"),
    ("hunt", 0.2, "hunt a blob"),
    ("hunt_mass", 1.0, "hunt: the blob's walkers a soldier"),
    ("hunt_far", 0.7, "hunt: minus the walk"),
    ("hunt_danger", 1.5, "hunt: minus the slime round it against the squad"),
    ("hunt_many", 1.0, "hunt: soldiers / 40"),
    ("stay", 0.3, "the squad's choice of last second, kept"),
    // Scales.
    ("per", 200.0, "walkers a soldier the squad counts as dangerous"),
    ("cap", 200.0, "the most soldiers bought"),
    ("threat_r", 15.0, "dish cells round a pump its slime is counted in"),
    ("threat_n", 500.0, "walkers by a pump that count as one unit of threat"),
];

/// A weight set: `WEIGHTS`' values, by index.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Weights {
    pub w: Vec<f32>,
    /// The shop between planets and the unlocks, as the strategy bot's.
    pub plan: Plan,
}

impl Weights {
    pub fn parse(spec: &str) -> Result<Weights, String> {
        let mut w: Vec<f32> = WEIGHTS.iter().map(|x| x.1).collect();
        let mut plan_keys = Vec::new();
        for item in spec.split(',').map(str::trim).filter(|x| !x.is_empty()) {
            let (k, v) = item.split_once('=').ok_or_else(|| format!("util: {item}: key=value"))?;
            match WEIGHTS.iter().position(|x| x.0 == k) {
                Some(i) => w[i] = v.parse::<f32>().map_err(|e| format!("util: {k}={v}: {e}"))?,
                None if ["shop", "cards", "own", "name", "aim", "kite"].contains(&k) => plan_keys.push(item),
                None => return Err(format!("util: no weight {k}")),
            }
        }
        Ok(Weights { w, plan: Plan::parse(&plan_keys.join(","))? })
    }

    fn get(&self, k: &str) -> f32 {
        WEIGHTS.iter().position(|x| x.0 == k).map_or(0.0, |i| self.w[i])
    }
}

/// What the bank buys.
#[derive(Clone, Copy, Debug)]
enum Buy {
    Soldier,
    Pump(usize),
    At(Kind, (f32, f32)),
    Upgrade(usize),
}

/// What the squad does.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Act {
    Back,
    Raise(usize),
    Guard((f32, f32)),
    Drain(usize),
    Hunt((f32, f32)),
}

impl Act {
    fn kind(self) -> usize {
        match self {
            Act::Back => 3,
            Act::Raise(_) => 2,
            Act::Guard(_) => 0,
            Act::Drain(_) | Act::Hunt(_) => 1,
        }
    }
}

#[derive(Default)]
struct Memory {
    spec: String,
    w: Option<Weights>,
    last: Option<Act>,
    eaten: Vec<(usize, f32)>,
    pumps: Vec<usize>,
    raising: Option<(usize, f32)>,
    bought: u32,
    peak: usize,
    secs: [u32; 4],
}

thread_local! {
    static MEM: RefCell<Memory> = RefCell::new(Memory::default());
}

/// Soldiers bought, the most alive, and seconds guarding, out, building
/// and falling back (the league's row).
pub(super) fn tally() -> (u32, usize, [u32; 4]) {
    MEM.with_borrow(|m| (m.bought, m.peak, m.secs))
}

/// One pump as the bank and the squad weigh it.
struct PumpView {
    b: usize,
    at: (f32, f32),
    r: f32,
    /// Slime round it, in `threat_n` units.
    threat: f32,
    /// Its draw a second, in 50s.
    draw: f32,
    turrets: usize,
    lures: usize,
    walls: usize,
    /// The way to the slime from it, radians.
    face: f32,
}

pub(super) fn util_hand(g: &mut Game<GpuWorld>, spec: &str) {
    let t = g.time();
    if MEM.with_borrow(|m| m.spec != spec) || t < 1.0 {
        let w = match Weights::parse(spec) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        MEM.with_borrow_mut(|m| *m = Memory { spec: spec.to_string(), w: Some(w), ..Memory::default() });
    }
    let w = MEM.with_borrow(|m| m.w.clone().unwrap());
    let k = |name: &str| w.get(name);
    let sc = g.world.scale();
    let live = g.live_squads();
    let Some(&q) = live.first() else { return };
    for &o in &live[1..] {
        g.apply(Cmd::Merge(q as u8, o as u8));
    }
    if g.active != Some(q) {
        g.apply(Cmd::Active(q as u8));
    }
    let Some(m) = g.middle(q) else { return };
    let range = g.gun_range(q) * sc;
    let sense = Sense::of(g);
    let cs = g.world.crystals();
    let to_slime = sense.middle().unwrap_or(cs[0].0);
    let per = k("per").max(1.0);

    // What it reads.
    MEM.with_borrow_mut(|mm| {
        let gone: Vec<usize> = mm.pumps.iter().copied().filter(|&c| g.pump_on(c).is_none()).collect();
        for c in gone {
            mm.eaten.push((c, t + 90.0));
        }
        mm.eaten.retain(|e| e.1 > t);
        mm.pumps = (0..cs.len()).filter(|&c| g.pump_on(c).is_some()).collect();
    });
    let near = |kind: fn(&Kind) -> bool, at: (f32, f32), r: f32| g.buildings.iter().filter(|b| b.alive && kind(&b.kind) && dist(b.at, at) <= r).count();
    let pumps: Vec<PumpView> = (0..g.buildings.len())
        .filter(|&b| g.buildings[b].alive && g.buildings[b].kind == Kind::Pump)
        .map(|b| {
            let at = g.buildings[b].at;
            let r = g.buildings[b].crystal.map_or(2.0 * sc, |c| g.world.crystal_r(c));
            let reach = r + 14.0 * sc;
            PumpView {
                b,
                at,
                r,
                threat: (sense.mass_near(at, r + k("threat_r") * sc) as f32 / k("threat_n").max(1.0)).min(3.0),
                draw: if g.buildings[b].up() { g.pump_draw(b) / 50.0 } else { 0.0 },
                turrets: near(|x| *x == Kind::Turret, at, reach),
                lures: near(|x| *x == Kind::Lure, at, reach),
                walls: near(|x| matches!(x, Kind::Wall { .. }), at, reach),
                face: (to_slime.1 - at.1).atan2(to_slime.0 - at.0),
            }
        })
        .collect();
    let n = g.alive();
    let few = 1.0 - (n as f32 / 40.0).min(1.0);
    let pressing = (sense.mass_near(m, range) as f32 / (per * n.max(1) as f32)).min(3.0);
    let base = pumps.first().map_or(g.larder, |p| p.at);
    let eaten: Vec<usize> = MEM.with_borrow(|mm| mm.eaten.iter().map(|e| e.0).collect());
    let worth = |c: usize, from: (f32, f32)| (cs[c].1.min(400_000.0) / 400_000.0) / (1.0 + dist(cs[c].0, from) / (4.0 * range).max(1.0));
    let crs: Vec<f32> = (0..cs.len()).map(|c| g.world.crystal_r(c)).collect();
    let risk = |c: usize| (sense.mass_near(cs[c].0, crs[c] + range) as f32 / per).min(5.0);

    // ---- The bank: the best-scoring buy, if it clears `save`; paid when
    // the bank holds it, else saved for.
    let mut bought = 0;
    for _ in 0..6 {
        let mut opts: Vec<(f32, Buy)> = Vec::new();
        if g.alive() < (k("cap").max(1.0) as usize).min(g.rules.max_squad) {
            opts.push((k("sol") + k("sol_few") * few + k("sol_threat") * pressing + k("sol_late") * t / 600.0, Buy::Soldier));
        }
        let free: Vec<usize> = (1..cs.len()).filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none() && g.pump_flows(c) && !eaten.contains(&c)).collect();
        let best_crystal = free.into_iter().max_by(|&a, &b| (worth(a, base) - 0.3 * risk(a)).total_cmp(&(worth(b, base) - 0.3 * risk(b))));
        if let Some(c) = best_crystal {
            opts.push((k("pump") + k("pump_val") * worth(c, base) - k("pump_have") * pumps.len() as f32 / 4.0 - k("pump_risk") * risk(c), Buy::Pump(c)));
        }
        let on = |p: &PumpView, rad: f32, a: f32| (p.at.0 + (p.r + rad * sc) * a.cos(), p.at.1 + (p.r + rad * sc) * a.sin());
        const FAN: [f32; 8] = [0.0, 40.0, -40.0, 80.0, -80.0, 120.0, -120.0, 180.0];
        for p in pumps.iter().filter(|p| g.buildings[p.b].up()) {
            if w.plan.owns("turret") && p.turrets < FAN.len() {
                let s = k("tur") + k("tur_threat") * p.threat + k("tur_draw") * p.draw - k("tur_have") * p.turrets as f32 / 4.0;
                opts.push((s, Buy::At(Kind::Turret, on(p, 4.0, p.face + FAN[p.turrets].to_radians()))));
            }
            if p.lures == 0 && p.turrets > 0 {
                opts.push((k("lure") + k("lure_tur") * p.turrets as f32 / 4.0, Buy::At(Kind::Lure, on(p, 11.0, p.face))));
            }
            if w.plan.owns("walls") && p.walls < 4 {
                let (a0, a1) = [(-60.0f32, 0.0f32), (0.0, 60.0), (60.0, 120.0), (-120.0, -60.0)][p.walls];
                let s = k("wall") + k("wall_threat") * p.threat - k("wall_have") * p.walls as f32 / 4.0;
                opts.push((s, Buy::At(Kind::Wall { to: on(p, 8.0, p.face + a1.to_radians()) }, on(p, 8.0, p.face + a0.to_radians()))));
            }
            if g.upgrade_price(p.b).is_some() {
                opts.push((k("up") + k("up_draw") * p.draw, Buy::Upgrade(p.b)));
            }
        }
        let Some(&(score, buy)) = opts.iter().max_by(|a, b| a.0.total_cmp(&b.0)) else { break };
        if score < k("save") {
            break;
        }
        let price = match buy {
            Buy::Soldier => g.rules.soldier_cost,
            Buy::Pump(c) => g.build_price(Kind::Pump, cs[c].0),
            Buy::At(kind, at) => g.build_price(kind, at),
            Buy::Upgrade(b) => g.upgrade_price(b).unwrap_or(f32::INFINITY),
        };
        if g.bank < price {
            break;
        }
        let ok = match buy {
            Buy::Soldier => g.apply(Cmd::Buy),
            Buy::Pump(c) => {
                let (cx, cy) = cs[c].0;
                let d = dist(m, (cx, cy)).max(1.0);
                let r = crs[c] * 0.9;
                g.apply(Cmd::Build(Kind::Pump, (cx + (m.0 - cx) / d * r, cy + (m.1 - cy) / d * r)))
            }
            Buy::At(kind @ Kind::Wall { .. }, at) => g.build_blocked(kind, at).is_none() && g.apply(Cmd::Build(kind, at)),
            Buy::At(kind, at) => spot(g, kind, at).is_some_and(|s| g.apply(Cmd::Build(kind, s))),
            Buy::Upgrade(b) => g.apply(Cmd::Upgrade(b)),
        };
        if !ok {
            break;
        }
        if matches!(buy, Buy::Soldier) {
            bought += 1;
        }
    }
    MEM.with_borrow_mut(|mm| {
        mm.bought += bought;
        mm.peak = mm.peak.max(g.alive());
    });

    // ---- The squad: the best-scoring act, with the last one kept a little.
    let members = g.members(q);
    let n = members.len().max(1);
    let hp = members.iter().map(|&i| g.soldiers[i].hp).sum::<f32>() / n as f32 / g.rules.hp.max(1.0);
    let bitten = members.iter().filter(|&&i| g.soldiers[i].touching > 0).count();
    let target = sense.ranked(m, range).into_iter().find(|&(a, d)| d <= range && sense.clear(g, m, a)).map(|(a, _)| a);
    g.apply(Cmd::Fire(q as u8, target));
    let last = MEM.with_borrow(|mm| mm.last);
    let walk = (4.0 * range).max(1.0);
    let mut acts: Vec<(f32, Act)> = Vec::new();
    if bitten > 0 {
        acts.push((k("back") * (1.0 - hp), Act::Back));
    }
    if let Some(b) = (0..g.buildings.len()).filter(|&b| g.buildings[b].alive && !g.buildings[b].up()).min_by(|&a, &b| dist(g.buildings[a].at, m).total_cmp(&dist(g.buildings[b].at, m))) {
        let at = g.buildings[b].at;
        let danger = sense.mass_near(at, range) as f32 / (per * n as f32);
        acts.push((k("raise") - k("raise_far") * dist(at, m) / walk - k("hunt_danger") * danger, Act::Raise(b)));
    }
    for p in &pumps {
        acts.push((k("guard") + k("guard_threat") * p.threat - k("guard_far") * dist(p.at, m) / walk + k("guard_few") * few, Act::Guard(p.at)));
    }
    for c in (1..cs.len()).filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none()) {
        let danger = sense.mass_near(cs[c].0, g.world.crystal_r(c) + range) as f32 / (per * n as f32);
        acts.push((k("drain") + k("drain_val") * (cs[c].1.min(400_000.0) / 400_000.0) - k("drain_far") * dist(cs[c].0, m) / walk - k("drain_risk") * danger, Act::Drain(c)));
    }
    for (a, d) in sense.ranked(m, range).into_iter().take(6) {
        let mass = sense.mass_near(a, 0.5 * range) as f32 / (30.0 * n as f32);
        let danger = sense.mass_near(a, range) as f32 / (per * n as f32);
        acts.push((k("hunt") + k("hunt_mass") * mass.min(2.0) - k("hunt_far") * d / walk - k("hunt_danger") * danger + k("hunt_many") * (n as f32 / 40.0).min(2.0), Act::Hunt(a)));
    }
    // The last act's kind and place, kept by `stay`.
    for (s, a) in &mut acts {
        let same = match (last, *a) {
            (Some(Act::Hunt(x)), Act::Hunt(y)) | (Some(Act::Guard(x)), Act::Guard(y)) => dist(x, y) < range,
            (Some(x), y) => x == y,
            _ => false,
        };
        if same {
            *s += k("stay");
        }
    }
    let Some(&(_, act)) = acts.iter().max_by(|a, b| a.0.total_cmp(&b.0)) else { return };
    MEM.with_borrow_mut(|mm| {
        mm.last = Some(act);
        mm.secs[act.kind()] += 1;
    });
    let fresh = |g: &Game<GpuWorld>, o: Order| !members.iter().all(|&i| g.soldiers[i].order == o);
    match act {
        Act::Back => {
            let away = target.unwrap_or(base);
            let d = dist(away, m).max(1.0);
            let back = (m.0 - (away.0 - m.0) / d * range, m.1 - (away.1 - m.1) / d * range);
            if target.is_some() {
                g.apply(Cmd::SquadOrder(q as u8, Order::Move(back.0, back.1)));
            } else if !g.squads[q].heal && g.bank > 0.0 {
                g.apply(Cmd::Heal(q as u8));
            }
        }
        Act::Raise(b) => {
            let since = MEM.with_borrow_mut(|mm| match mm.raising {
                Some((x, at)) if x == b => at,
                _ => {
                    mm.raising = Some((b, t));
                    t
                }
            });
            if t - since > 40.0 {
                g.apply(Cmd::Unbuild(b));
            } else if fresh(g, Order::Build(b)) {
                g.apply(Cmd::SquadOrder(q as u8, Order::Build(b)));
            }
        }
        Act::Guard(at) => {
            let o = Order::Guard(at.0, at.1);
            if fresh(g, o) && dist(m, at) > 0.25 * range {
                g.apply(Cmd::SquadOrder(q as u8, o));
            }
        }
        Act::Drain(c) => {
            let o = Order::Clear(cs[c].0 .0, cs[c].0 .1);
            if fresh(g, o) {
                g.apply(Cmd::SquadOrder(q as u8, o));
            }
        }
        Act::Hunt(a) => {
            let d = dist(a, m).max(1.0);
            let stand = (a.0 - (a.0 - m.0) / d * 0.7 * range, a.1 - (a.1 - m.1) / d * 0.7 * range);
            if d > 0.8 * range || !sense.clear(g, m, a) {
                g.apply(Cmd::SquadOrder(q as u8, Order::Fight(stand.0, stand.1)));
            }
        }
    }
    if (t - t.round()).abs() < 0.02 && (t.round() as u64).is_multiple_of(10) {
        eprintln!("UTIL t {t:.0} {act:?} mould {} soldiers {} hp {hp:.2} bank {:.0} killed {} turret {} pumps {} buildings {}", g.mould, g.alive(), g.bank, g.killed, g.turret_killed, pumps.len(), g.buildings.iter().filter(|b| b.alive).count());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_parse_and_unknown_keys_are_refused() {
        let w = Weights::parse("sol=2,shop=turret/strike,own=walls/turret").unwrap();
        assert_eq!(w.get("sol"), 2.0);
        assert_eq!(w.get("save"), 0.8);
        assert!(w.plan.owns("turret"));
        assert_eq!(w.plan.shop, vec!["turret", "strike"]);
        assert!(Weights::parse("nonsense=1").is_err());
        assert_eq!(Weights::parse("").unwrap().w.len(), WEIGHTS.len());
    }
}
