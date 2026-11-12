//! **The squad toolkit** (2026-11-12, the owner's "all six together" on
//! the toolkit thread): six tools for the Larder world's squad, each built
//! on one term of the mould's model, so what a tool does is a change in the
//! simulation the picture draws, not an effect painted over it. CPU world
//! only so far (`Game<BigDish>`); the GPU world has none of them yet.
//!
//! The model in one line: walkers read the trail ahead with three sensors,
//! turn to the strongest, step and lay trail; trail spreads and fades;
//! scent (crystals, soldiers, the larder) pulls them; each walker holds
//! energy, pays upkeep, divides when fed and starves when not; with the
//! life cycle on a walker can sleep (dormant) and an alarm field wakes it.
//!
//! - **Blast** (`Tool::Blast`): kills the walkers within `blast_r`, clears
//!   their trail, lays alarm, and scorches a crater (`BigDish::scorch_disc`,
//!   trail x `scorch_keep` a step) for `crater_secs`, so the hole stays open
//!   instead of re-knitting at once. The crater is the toolkit's scorch: the
//!   owner asked "do we need scortch when we have walls?" (04:25Z); a wall
//!   stops walkers, soldiers and shots and is gnawed; scorch only stops trail
//!   holding, which matters right after a blast or a cut.
//! - **Mine** (`Tool::Mine`): laid unscented; goes off as a blast when
//!   `mine_trigger` walkers stand within `mine_r` of it.
//! - **Det cord** (`Tool::Cord`): a line of small blasts every
//!   `cord_every`, each with its crater, fired at once: a cut that stays
//!   open (the knife's cut healed in about 4 s, `Reports/dead-ends.md`).
//! - **Germ** (`Tool::Germ`): infects the walkers within `germ_r`. Sick
//!   walkers lay germ on their cells, so it passes along the tubes to the
//!   walkers that follow them, and pay `sick_upkeep` x upkeep, so the wave
//!   leaves starved walkers behind. Steering-only contagion left no mark
//!   (`Reports/dead-ends.md`, contagion).
//! - **Sleep gas** (`Tool::Gas`): the walkers within `gas_r` sleep at once,
//!   deaf to scent for `gas_secs`; alarm (a blast, gunfire) wakes them. A
//!   sleeping walker neither eats nor bites (`World::calm`).
//! - **Mask and reek** (`Tool::Smell`): a soldier's smell x 0 (the mould
//!   can't smell it) or x `reek` (a decoy).
//!
//! Every tool is paid from the bank and must be used within `reach` of a
//! living soldier, so tools are part of the squad's play, not a cursor's.
use super::big::BigDish;
use super::larder_world::{Game, World};

#[derive(Clone, Copy, Debug)]
pub struct KitRules {
    /// Dish cells from a living soldier a tool can be used at.
    pub reach: f32,
    /// Blast radius and crater, dish cells; alarm laid within 2 x radius.
    pub blast_r: f32,
    pub blast_alarm: f32,
    pub crater_r: f32,
    pub crater_secs: f32,
    pub blast_price: f32,
    pub mine_r: f32,
    pub mine_trigger: u32,
    pub mine_price: f32,
    /// Det cord: blasts of `cord_r` every `cord_every` dish cells.
    pub cord_r: f32,
    pub cord_every: f32,
    /// Price a dish cell of cord.
    pub cord_price: f32,
    pub cord_max: f32,
    pub germ_r: f32,
    pub sick_secs: f32,
    pub immune_secs: f32,
    pub sick_upkeep: f32,
    pub sick_catch: f32,
    pub sick_deposit: f32,
    pub sick_keep: f32,
    pub germ_price: f32,
    pub gas_r: f32,
    pub gas_secs: f32,
    pub gas_death: f32,
    pub gas_price: f32,
    pub reek: f32,
    /// Tools a soldier can hold (0: no carry rule, tools come from the bank
    /// wherever a soldier stands). With it on, a soldier takes tools at the
    /// larder (`Kit::restock`, within `restock_r` dish cells), each takes
    /// `tool_weight` of its carry (`Soldier::load`), and each use spends
    /// one held by a soldier in reach. Why (2026-11-12, round 2): in round 1
    /// a runner carried blasts to the mould's body without end and won
    /// every tier; the game designer's first pick for a cost that comes
    /// from the game rather than a price was carry, a trade between army
    /// and economy.
    pub carry_max: u32,
    /// What a plan's other soldiers take at the larder (kitplans' hands).
    pub carry_each: u32,
    pub tool_weight: f32,
    pub restock_r: f32,
    /// The alarm's keep, when a tool turns the alarm on.
    pub alarm_keep: f32,
}

impl Default for KitRules {
    fn default() -> Self {
        KitRules {
            reach: 24.0,
            blast_r: 6.0,
            blast_alarm: 50.0,
            crater_r: 7.0,
            crater_secs: 20.0,
            blast_price: 60.0,
            mine_r: 5.0,
            mine_trigger: 40,
            mine_price: 80.0,
            cord_r: 2.5,
            cord_every: 2.0,
            cord_price: 3.0,
            cord_max: 48.0,
            germ_r: 5.0,
            sick_secs: 6.0,
            immune_secs: 40.0,
            sick_upkeep: 60.0,
            sick_catch: 0.2,
            sick_deposit: 1.0,
            sick_keep: 0.9,
            germ_price: 300.0,
            gas_r: 12.0,
            gas_secs: 20.0,
            gas_death: 0.0,
            gas_price: 150.0,
            reek: 4.0,
            carry_max: 0,
            carry_each: 1,
            tool_weight: 100.0,
            restock_r: 6.0,
            alarm_keep: 0.95,
        }
    }
}

/// A tool use; points in fine cells, as soldiers' are.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    Blast((f32, f32)),
    Mine((f32, f32)),
    Cord((f32, f32), (f32, f32)),
    Germ((f32, f32)),
    Gas((f32, f32)),
    /// Soldier k's smell multiplier (0 mask, 1 plain, `reek`).
    Smell(usize, f32),
}

pub const TOOLS: [&str; 6] = ["blast", "mine", "cord", "germ", "gas", "smell"];

impl Tool {
    pub fn index(self) -> usize {
        match self {
            Tool::Blast(_) => 0,
            Tool::Mine(_) => 1,
            Tool::Cord(..) => 2,
            Tool::Germ(_) => 3,
            Tool::Gas(_) => 4,
            Tool::Smell(..) => 5,
        }
    }
}

/// One blast to draw: where, its radius (fine cells), the tick it went off.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bang {
    pub at: (f32, f32),
    pub r: f32,
    pub tick: u64,
    pub killed: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Kit {
    pub rules: KitRules,
    /// Mines laid and not yet gone off, fine cells.
    pub mines: Vec<(f32, f32)>,
    pub bangs: Vec<Bang>,
    /// Per tool (`TOOLS` order): uses, bank spent, walkers killed.
    pub uses: [u32; 6],
    pub spent: [f32; 6],
    pub killed: [usize; 6],
    pub gassed: usize,
    /// Tools each soldier holds (the carry rule), by soldier index.
    pub held: Vec<u32>,
}

impl Kit {
    pub fn new(rules: KitRules) -> Self {
        Kit { rules, ..Default::default() }
    }

    fn price(&self, t: Tool) -> f32 {
        let r = &self.rules;
        match t {
            Tool::Blast(_) => r.blast_price,
            Tool::Mine(_) => r.mine_price,
            Tool::Cord(a, b) => r.cord_price * (a.0 - b.0).hypot(a.1 - b.1),
            Tool::Germ(_) => r.germ_price,
            Tool::Gas(_) => r.gas_price,
            Tool::Smell(..) => 0.0,
        }
    }

    /// Whether a living soldier is within `reach` of `p` (with the carry
    /// rule, one holding a tool).
    pub fn in_reach(&self, g: &Game<BigDish>, p: (f32, f32)) -> bool {
        self.user(g, p).is_some()
    }

    /// The nearest living soldier within reach of `p` that may use a tool.
    fn user(&self, g: &Game<BigDish>, p: (f32, f32)) -> Option<usize> {
        let r = self.rules.reach * g.world.p.scale as f32;
        (0..g.soldiers.len())
            .filter(|&k| {
                let s = g.soldiers[k];
                s.alive && (s.x - p.0).hypot(s.y - p.1) <= r && (self.rules.carry_max == 0 || self.held(k) > 0)
            })
            .min_by(|&a, &b| {
                let d = |k: usize| (g.soldiers[k].x - p.0).hypot(g.soldiers[k].y - p.1);
                d(a).total_cmp(&d(b))
            })
    }

    /// Tools soldier `k` holds.
    pub fn held(&self, k: usize) -> u32 {
        self.held.get(k).copied().unwrap_or(0)
    }

    /// Soldier `k` takes tools at the larder up to `n` held (and
    /// `carry_max`); returns whether it is close enough. Nothing is paid
    /// here: a tool is paid when used.
    pub fn restock(&mut self, g: &mut Game<BigDish>, k: usize, n: u32) -> bool {
        let s = g.soldiers[k];
        let r = self.rules.restock_r * g.world.p.scale as f32;
        if !s.alive || (s.x - g.larder.0).hypot(s.y - g.larder.1) > r {
            return false;
        }
        if self.held.len() <= k {
            self.held.resize(k + 1, 0);
        }
        self.held[k] = n.min(self.rules.carry_max).max(self.held[k]);
        g.soldiers[k].load = self.held[k] as f32 * self.rules.tool_weight;
        true
    }

    /// Uses a tool if the bank holds its price and a soldier is in reach
    /// (cord: of its middle, at most `cord_max` long). Returns whether it
    /// was used. Logs nothing in the game's replay (CPU prototype).
    pub fn use_tool(&mut self, g: &mut Game<BigDish>, t: Tool) -> bool {
        let s = g.world.p.scale as f32;
        let price = self.price(t);
        if g.bank < price {
            return false;
        }
        let ok = match t {
            Tool::Blast(p) | Tool::Mine(p) | Tool::Germ(p) | Tool::Gas(p) => self.in_reach(g, p) && g.world.open(p.0, p.1),
            Tool::Cord(a, b) => (a.0 - b.0).hypot(a.1 - b.1) <= self.rules.cord_max * s && self.in_reach(g, ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)),
            Tool::Smell(k, _) => g.soldiers.get(k).is_some_and(|x| x.alive),
        };
        if !ok {
            return false;
        }
        if self.rules.carry_max > 0 && !matches!(t, Tool::Smell(..)) {
            let mid = match t {
                Tool::Blast(p) | Tool::Mine(p) | Tool::Germ(p) | Tool::Gas(p) => p,
                Tool::Cord(a, b) => ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0),
                Tool::Smell(..) => unreachable!(),
            };
            let k = self.user(g, mid).expect("in reach");
            self.held[k] -= 1;
            g.soldiers[k].load = self.held[k] as f32 * self.rules.tool_weight;
        }
        g.bank -= price;
        g.spent += price;
        let i = t.index();
        self.uses[i] += 1;
        self.spent[i] += price;
        let r = self.rules;
        match t {
            Tool::Blast(p) => {
                self.killed[i] += self.bang(&mut g.world, g.rules.tps, p, r.blast_r, r.crater_r);
            }
            Tool::Mine(p) => self.mines.push(p),
            Tool::Cord(a, b) => {
                let len = (a.0 - b.0).hypot(a.1 - b.1);
                let n = (len / (r.cord_every * s)).ceil().max(1.0) as usize;
                for j in 0..=n {
                    let f = j as f32 / n as f32;
                    let p = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
                    self.killed[i] += self.bang(&mut g.world, g.rules.tps, p, r.cord_r, r.cord_r);
                }
            }
            Tool::Germ(p) => {
                let d = &mut g.world;
                let tps = (g.rules.tps as f32) * d.p.substeps as f32;
                if d.sick.is_empty() {
                    d.p.sick_steps = (r.sick_secs * tps).round().max(1.0) as u16;
                    d.p.immune_steps = (r.immune_secs * tps).round().min(u16::MAX as f32 - d.p.sick_steps as f32) as u16;
                    d.p.sick_upkeep = r.sick_upkeep;
                    d.p.sick_catch = r.sick_catch;
                    d.p.sick_deposit = r.sick_deposit;
                    d.p.sick_keep = r.sick_keep;
                    d.enable_contagion();
                }
                d.infect(p.0 / s, p.1 / s, r.germ_r);
            }
            Tool::Gas(p) => {
                let d = &mut g.world;
                d.enable_alarm(r.alarm_keep);
                d.p.gas_death = r.gas_death;
                let steps = (r.gas_secs * g.rules.tps as f32 * d.p.substeps as f32).round().min(u16::MAX as f32) as u16;
                self.gassed += d.gas(p.0, p.1, r.gas_r * s, steps);
            }
            Tool::Smell(k, m) => g.soldiers[k].smell = m,
        }
        true
    }

    /// One blast: kill, clear, alarm, crater. Fine-cell point, dish-cell radii.
    fn bang(&mut self, d: &mut BigDish, tps: u32, p: (f32, f32), r: f32, crater: f32) -> usize {
        let s = d.p.scale as f32;
        d.enable_alarm(self.rules.alarm_keep);
        let killed = d.strike(p.0 / s, p.1 / s, r, self.rules.blast_alarm, 2.0 * r);
        let steps = (self.rules.crater_secs * tps as f32 * d.p.substeps as f32).round().min(u16::MAX as f32) as u16;
        d.scorch_disc(p.0, p.1, crater * s, steps);
        self.bangs.push(Bang { at: p, r: r * s, tick: d.ticks, killed });
        killed
    }

    /// Before each game step: mines with `mine_trigger` walkers within
    /// `mine_r` go off. Returns the mines that went off.
    pub fn frame(&mut self, g: &mut Game<BigDish>) -> usize {
        if self.mines.is_empty() {
            return 0;
        }
        let r = self.rules;
        let s = g.world.p.scale as f32;
        let rr = r.mine_r * s;
        let occ = g.world.occupancy().to_vec();
        let w = g.world.w;
        let mut fired = Vec::new();
        for (m, &(x, y)) in self.mines.iter().enumerate() {
            let n = rr.ceil() as i64;
            let mut c = 0u32;
            for dy in -n..=n {
                for dx in -n..=n {
                    if ((dx * dx + dy * dy) as f32) > rr * rr {
                        continue;
                    }
                    let (cx, cy) = (x as i64 + dx, y as i64 + dy);
                    if cx < 0 || cy < 0 || cx as usize >= w || cy as usize >= g.world.h {
                        continue;
                    }
                    c += occ[cy as usize * w + cx as usize] as u32;
                }
            }
            if c >= r.mine_trigger {
                fired.push(m);
            }
        }
        for &m in fired.iter().rev() {
            let p = self.mines.remove(m);
            self.killed[1] += self.bang(&mut g.world, g.rules.tps, p, r.blast_r, r.crater_r);
        }
        fired.len()
    }
}
