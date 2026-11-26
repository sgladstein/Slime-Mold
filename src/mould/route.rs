//! **The run's star map** (2026-11-26; the owner's pick, 11-25 23:45Z: "I
//! choose 1 + 4 and each of those planet options should have clearly
//! different difficulties with greater rewards on the harder planets").
//! Brief: `Reports/review-2026-11-25/route-choice-brief.md`.
//!
//! A run is `run::PLANETS` steps from the landing world to the slime's
//! home world. The first and the last step have one planet; every step
//! between offers two or three, joined to the next step's by lanes that
//! cross, so a pick now decides which picks come later. Each planet is a
//! **node**: which one it is at its step (`k`), its difficulty (`Pip`),
//! its lane on the map, the next step's nodes it leads to, and one or two
//! optional **contracts** (direction 4) that pay on top of the win.
//!
//! The map is drawn from the run's seed alone (`star_map`), so a saved
//! run keeps its map and only the route taken is saved (`Run::route`).
//! Node 0 of every step is the Fair one; a run with no route (an old
//! save's, the bots') plays node 0 at every step.
//!
//! What a pip does to the planet is `run`'s: the pip moves the place on
//! the tuned eight-planet curve the planet is read at (`Pip::offset`), so
//! a harder planet is the same tables a step and a half further on (a
//! bigger slime and heart, sooner raids, a later species and biome), not
//! new numbers. Rejected: a second set of per-pip tables, which would be
//! one more thing to tune for every planet.

/// How hard a planet on the map is, and what it pays (the owner's "clearly
/// different difficulties with greater rewards on the harder planets").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pip {
    Safe,
    Fair,
    Hard,
}

impl Pip {
    pub const ALL: [Pip; 3] = [Pip::Safe, Pip::Fair, Pip::Hard];

    /// The pips drawn on its node: 1, 2 or 3.
    pub fn count(self) -> u8 {
        match self {
            Pip::Safe => 1,
            Pip::Fair => 2,
            Pip::Hard => 3,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Pip::Safe => "Safe",
            Pip::Fair => "Fair",
            Pip::Hard => "Hard",
        }
    }

    /// Where on the eight-planet curve a planet of this pip is read, from
    /// the step's own place (`run::curve_depth`): the brief's -1, 0 and
    /// +1.5. The steps of a twelve-planet run are 0.64 of a curve place
    /// apart, so Safe is about a step and a half back and Hard more than
    /// two steps on: a clear gap either way.
    pub fn offset(self) -> f32 {
        match self {
            Pip::Safe => -1.0,
            Pip::Fair => 0.0,
            Pip::Hard => 1.5,
        }
    }

    /// The planet's reward, in tenths of the Fair planet's (`run::reward`):
    /// the brief's 1 : 1.5 : 2.25, held with Fair at today's pay so the
    /// shop's prices still fit (rejected: Fair at 1.5 times today's,
    /// which would have raised every planet's pay by half).
    pub fn pay_x10(self) -> u32 {
        match self {
            Pip::Safe => 7,
            Pip::Fair => 10,
            Pip::Hard => 15,
        }
    }

    /// Its one line on the node's card.
    pub fn about(self) -> &'static str {
        match self {
            Pip::Safe => "A smaller slime from earlier in the run, no twist. Pays less.",
            Pip::Fair => "The run's own pace.",
            Pip::Hard => "A bigger slime from further on, always a twist. Pays half again, and a rare card or gun waits in the shop.",
        }
    }
}

/// An optional goal on a planet (direction 4): met when the planet is won,
/// it pays its points on top of the planet's reward.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Contract {
    /// Lose at most this many soldiers.
    LoseAtMost(u32),
    /// Win in under this share of the planet's speed star, in percent.
    Quick(u32),
    /// Break this many mini-bosses.
    Breaker(u32),
    /// Draw this much food with pumps (the review, 11-25: the run bot wins
    /// planets 1 to 3 with no pump; this pays for building some).
    Pump(u32),
    /// Pick up this many points of starfall ore and cores.
    Forage(u32),
}

/// What a planet's end tells a contract (the window fills it from the
/// game and its tracker).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Outcome {
    pub won: bool,
    /// Game seconds from the landing.
    pub at: f32,
    /// The speed star's time (`Rules::par`).
    pub par: f32,
    pub lost: u32,
    /// Mini-bosses broken.
    pub broken: u32,
    pub pumped: f32,
    pub found: u32,
}

impl Contract {
    /// Its words on the node and the end screen.
    pub fn name(self) -> String {
        match self {
            Contract::LoseAtMost(0) => "Lose no soldier".into(),
            Contract::LoseAtMost(n) => format!("Lose {n} soldiers at most"),
            Contract::Quick(p) => format!("Win in {p}% of the star's time"),
            Contract::Breaker(1) => "Break a mini-boss".into(),
            Contract::Breaker(n) => format!("Break {n} mini-bosses"),
            Contract::Pump(f) => format!("Pump {f} food"),
            Contract::Forage(p) => format!("Pick up {p} points of ore and cores"),
        }
    }

    /// The planet was won and the contract kept.
    pub fn met(self, o: &Outcome) -> bool {
        o.won
            && match self {
                Contract::LoseAtMost(n) => o.lost <= n,
                Contract::Quick(p) => o.par > 0.0 && o.at < o.par * p as f32 / 100.0,
                Contract::Breaker(n) => o.broken >= n,
                Contract::Pump(f) => o.pumped >= f as f32,
                Contract::Forage(p) => o.found >= p,
            }
    }

    /// What it pays at curve place `e` (`run::curve_depth`), in points:
    /// about a third of a planet's win for the plain ones, half for the
    /// hard ones, rounded to fives. First guesses; balance later.
    pub fn pay(self, e: f32) -> u32 {
        let base = 60.0 + 20.0 * (e - 1.0).max(0.0);
        let k = match self {
            Contract::LoseAtMost(0) | Contract::Quick(..=60) | Contract::Breaker(2..) => 1.6,
            _ => 1.0,
        };
        ((base * k / 5.0).round() * 5.0) as u32
    }

    /// The contracts that can be drawn for a planet at curve place `e`: no
    /// mini-boss before they come (`BossRules`: from planet 2), no pump
    /// goal before pumps are common, and targets that grow with the
    /// planet. First guesses from the curve passes' medians (losses 3 to
    /// 12 a planet, win at about half the star's time); balance later.
    pub fn pool(e: f32) -> Vec<Contract> {
        let mut v = vec![Contract::LoseAtMost(if e < 3.0 { 0 } else { 2 }), Contract::Quick(if e < 4.0 { 60 } else { 70 }), Contract::Forage((50.0 + 25.0 * e).round() as u32 / 10 * 10)];
        if e >= 2.0 {
            v.push(Contract::Breaker(if e >= 6.0 { 2 } else { 1 }));
        }
        if e >= 1.5 {
            v.push(Contract::Pump((1000.0 + 500.0 * e).round() as u32 / 100 * 100));
        }
        v
    }

    /// Its save word (`run::Run::save_text`).
    pub fn word(self) -> String {
        match self {
            Contract::LoseAtMost(n) => format!("lose{n}"),
            Contract::Quick(p) => format!("quick{p}"),
            Contract::Breaker(n) => format!("break{n}"),
            Contract::Pump(f) => format!("pump{f}"),
            Contract::Forage(p) => format!("forage{p}"),
        }
    }
}

/// One planet on the map.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// Its step, from 1 (`Run::depth`).
    pub step: u32,
    /// Which node it is at its step: 0 the Fair one.
    pub k: u8,
    pub pip: Pip,
    /// Its place across the map, -1 to 1 (0 alone at its step).
    pub lane: f32,
    /// The next step's nodes it leads to (by `k`).
    pub to: Vec<u8>,
    /// The world's seed (`levelgen::level`), from the run's, the step's and
    /// its own.
    pub seed: u64,
    pub contracts: Vec<Contract>,
}

/// The run's star map: `steps[s - 1]` holds step `s`'s nodes, by `k`.
#[derive(Clone, Debug, PartialEq)]
pub struct StarMap {
    pub steps: Vec<Vec<Node>>,
}

impl StarMap {
    pub fn node(&self, step: u32, k: u8) -> Option<&Node> {
        self.steps.get((step as usize).checked_sub(1)?)?.get(k as usize)
    }

    /// The nodes a run can pick at `step` coming from node `from` of the
    /// step before (all of them at step 1).
    pub fn choices(&self, step: u32, from: Option<u8>) -> Vec<&Node> {
        let Some(here) = self.steps.get((step as usize).saturating_sub(1)) else { return Vec::new() };
        match (step.checked_sub(1).and_then(|p| from.and_then(|k| self.node(p, k))), step) {
            (Some(prev), _) => here.iter().filter(|n| prev.to.contains(&n.k)).collect(),
            _ => here.iter().collect(),
        }
    }
}

/// A small seeded generator (splitmix64), as `run`'s.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The world seed of node `k` at `step` of the run with `seed`.
pub fn node_seed(seed: u64, step: u32, k: u8) -> u64 {
    mix(seed ^ u64::from(step).wrapping_mul(0x5851_F42D_4C95_7F2D) ^ u64::from(k).wrapping_mul(0x1405_7B7E_F767_814F) ^ 0x5354_4152)
}

/// How many nodes step `step` of `planets` has: one at the landing and at
/// the home world, three at the first fork, then three or (one step in
/// four) two.
fn width(seed: u64, step: u32, planets: u32) -> usize {
    if step <= 1 || step >= planets {
        1
    } else if step == 2 || !mix(seed ^ 0x5749_4454 ^ u64::from(step)).is_multiple_of(4) {
        3
    } else {
        2
    }
}

/// The run's map, from its seed (`curve` is `run::curve_depth`, so this
/// module needs no run numbers).
pub fn star_map(seed: u64, planets: u32, curve: impl Fn(u32) -> f32) -> StarMap {
    let mut steps: Vec<Vec<Node>> = Vec::new();
    for step in 1..=planets.max(1) {
        let n = width(seed, step, planets);
        let z = mix(seed ^ 0x4E4F_4445 ^ u64::from(step).wrapping_mul(0x2545_F491_4F6C_DD1D));
        // Node 0 is Fair; a two-node step's other is Safe or Hard.
        let pips: Vec<Pip> = match n {
            1 => vec![Pip::Fair],
            2 => vec![Pip::Fair, if z.is_multiple_of(2) { Pip::Safe } else { Pip::Hard }],
            _ => vec![Pip::Fair, Pip::Safe, Pip::Hard],
        };
        // Lanes: the nodes shuffled across the map.
        let mut order: Vec<usize> = (0..n).collect();
        let mut s = z >> 8;
        for i in (1..n).rev() {
            let j = (s % (i as u64 + 1)) as usize;
            order.swap(i, j);
            s = mix(s);
        }
        let nodes = (0..n)
            .map(|k| {
                let lane = if n == 1 { 0.0 } else { -1.0 + 2.0 * order[k] as f32 / (n - 1) as f32 };
                let e = (curve(step) + pips[k].offset()).clamp(1.0, 8.0);
                // One contract, or two from step 3 one time in two; none on
                // the landing.
                let mut pool = if step == 1 { Vec::new() } else { Contract::pool(e) };
                let mut c = mix(z ^ (0xC0_0000 + k as u64));
                let want = if step >= 3 && c.is_multiple_of(2) { 2 } else { 1 };
                let mut contracts = Vec::new();
                while contracts.len() < want && !pool.is_empty() {
                    c = mix(c);
                    contracts.push(pool.remove((c % pool.len() as u64) as usize));
                }
                Node { step, k: k as u8, pip: pips[k], lane, to: Vec::new(), seed: node_seed(seed, step, k as u8), contracts }
            })
            .collect();
        steps.push(nodes);
    }
    // The lanes: each node leads to the next step's nodes beside it (at
    // most one lane over), each such edge kept two times in three, then
    // every node given a way on and a way in (rejected: a lane from Fair
    // to Fair at every step, so an all-zero route was always a path; it
    // drew lanes clean across the map, top to bottom).
    for i in 0..steps.len().saturating_sub(1) {
        let (here, next) = (&steps[i], &steps[i + 1]);
        let mut to: Vec<Vec<u8>> = vec![Vec::new(); here.len()];
        let mut e = mix(seed ^ 0x4544_4745 ^ i as u64);
        for (a, na) in here.iter().enumerate() {
            for nb in next {
                e = mix(e);
                if (na.lane - nb.lane).abs() <= 1.01 && (!e.is_multiple_of(3) || here.len() == 1 || next.len() == 1) {
                    to[a].push(nb.k);
                }
            }
        }
        for (a, na) in here.iter().enumerate() {
            if to[a].is_empty() {
                let near = next.iter().min_by(|x, y| (x.lane - na.lane).abs().total_cmp(&(y.lane - na.lane).abs())).map_or(0, |n| n.k);
                to[a].push(near);
            }
        }
        for nb in next {
            if !to.iter().any(|t| t.contains(&nb.k)) {
                let a = here.iter().min_by(|x, y| (x.lane - nb.lane).abs().total_cmp(&(y.lane - nb.lane).abs())).map_or(0, |n| n.k as usize);
                to[a].push(nb.k);
            }
        }
        for (a, t) in to.into_iter().enumerate() {
            let mut t = t;
            t.sort_unstable();
            t.dedup();
            steps[i][a].to = t;
        }
    }
    StarMap { steps }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn curve(s: u32) -> f32 {
        1.0 + (s - 1) as f32 * 7.0 / 11.0
    }

    #[test]
    fn every_map_is_a_whole_route_from_landing_to_home() {
        for seed in 0..200 {
            let m = star_map(seed, 12, curve);
            assert_eq!(m.steps.len(), 12);
            assert_eq!((m.steps[0].len(), m.steps[11].len()), (1, 1), "seed {seed}: one landing, one home world");
            for (i, step) in m.steps.iter().enumerate() {
                assert_eq!(step[0].pip, Pip::Fair, "seed {seed}");
                if i + 1 < m.steps.len() {
                    for n in step {
                        assert!(n.to.iter().all(|&k| (m.steps[i + 1][k as usize].lane - n.lane).abs() <= 1.01), "seed {seed}: a lane across the map");
                        assert!(!n.to.is_empty(), "seed {seed}: a dead end at step {}", i + 1);
                    }
                    for n in &m.steps[i + 1] {
                        assert!(step.iter().any(|a| a.to.contains(&n.k)), "seed {seed}: node {} at step {} has no way in", n.k, i + 2);
                    }
                }
                if i > 0 && i + 1 < m.steps.len() {
                    assert!(step.len() >= 2, "seed {seed}: every step between is a fork");
                    let mut p: Vec<Pip> = step.iter().map(|n| n.pip).collect();
                    p.dedup();
                    assert_eq!(p.len(), step.len(), "seed {seed}: a step's planets differ in difficulty");
                }
            }
        }
    }

    #[test]
    fn the_map_comes_from_the_seed() {
        assert_eq!(star_map(5, 12, curve), star_map(5, 12, curve));
        assert_ne!(star_map(5, 12, curve), star_map(6, 12, curve));
    }

    #[test]
    fn contracts_need_a_win_and_their_goal() {
        let o = Outcome { won: true, at: 200.0, par: 480.0, lost: 1, broken: 1, pumped: 900.0, found: 120 };
        assert!(Contract::LoseAtMost(2).met(&o) && !Contract::LoseAtMost(0).met(&o));
        assert!(Contract::Quick(60).met(&o) && !Contract::Quick(40).met(&o));
        assert!(Contract::Breaker(1).met(&o) && !Contract::Breaker(2).met(&o));
        assert!(!Contract::Pump(1000).met(&o) && Contract::Forage(100).met(&o));
        assert!(!Contract::Forage(100).met(&Outcome { won: false, ..o }), "a lost planet pays nothing");
        assert!(Contract::Pump(2000).pay(4.0) > Contract::Pump(2000).pay(1.0));
    }
}
