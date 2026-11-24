//! **The squad toolkit** (2026-11-12, the owner's "all six together" on
//! the toolkit thread): six tools for the Larder world's squad, each built
//! on one term of the mould's model, so what a tool does is a change in the
//! simulation the picture draws, not an effect painted over it.
//!
//! **On every world** (2026-11-12, the technical lead: port now, retune
//! later): each tool comes down to bursts (`squad::Burst`), which a world
//! applies (`World::burst`). The blast, mine and det cord run on the CPU
//! and the GPU world alike (the GPU's strike passes lay the crater too);
//! germ and gas need `World::has_germ` and `has_gas` (the CPU world's
//! both, the GPU world's the gas since 11-14), and are refused (not paid)
//! elsewhere. Until then every
//! tool called `BigDish` itself, so none ran on the GPU.
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
//! - **Provoke** (`Tool::Provoke`, the simulation-use plan's B1,
//!   2026-11-17): the walkers within `mind_r` go deaf to food news for
//!   `provoke_secs` (`big::MIND_DEAF`): once their news is older than the
//!   wither lag they believe they are cut off, grey and panic, charging the
//!   nearest soldier in reach or running for a crystal.
//! - **Still** (`Tool::Still`, B2): the walkers within `mind_r` neither
//!   panic, surge nor raid for `still_secs`, and take no crystal
//!   (`big::MIND_STILL`): a charge stops, and a held body starves.
//!
//! Every tool is paid from the bank and must be used within `reach` of a
//! living soldier, so tools are part of the squad's play, not a cursor's.
use super::larder_world::{Game, World};
use super::squad::{Burst, BurstRules};

#[derive(Clone, Copy, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
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
    /// **Det cord's cards** (gun step E, 2026-11-24): a cord sets the
    /// devices' cards (`Game::device_deck`) off at both ends and every
    /// this many dish cells between, not at each of its blasts: at every
    /// 2-cell blast a 48-cell cord would cast 25 times.
    pub cord_cards: f32,
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
    /// The gas cloud (`BigParams::gas_keep`, 11-14): a gas burst also lays
    /// `gas_dose` of cloud over its disc, which spreads through the agar,
    /// halves every `gas_half` game seconds besides (0: no cloud), and
    /// puts to sleep every walker on a cell over `gas_sleep`, gassed
    /// `gas_linger` seconds more. A cloud you can see and walk the mould
    /// into, after Noita's mists (Reports/noita-spell-look-2026-11-14.md).
    pub gas_half: f32,
    pub gas_dose: f32,
    pub gas_sleep: f32,
    pub gas_linger: f32,
    /// **Sleep wears off** (2026-11-18; `BurstRules::gas_immune`): game
    /// seconds a walker that woke from gas cannot be put back to sleep,
    /// and no gas lengthens a sleep already running. The owner's playtest
    /// that day: Mist on the turrets "basically stopped everything", and
    /// "if you review how that happens in noita, there are no single OP
    /// spells". A turret's mist laid a cloud along every shot; a walker in
    /// it slept for as long as the turrets kept firing, so nothing reached
    /// them. Now a sleep is a window: the slime sleeps once, wakes and
    /// comes on through the cloud for this long.
    pub gas_immune: f32,
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
    /// The density `mine_trigger` was set at: a mine's trigger is
    /// `mine_trigger` x the game's density (`Game::density`, walkers per
    /// open cell at the start) over this, so a mine means the same share of
    /// mould at 5,000 walkers or 100,000 (the technical lead, 2026-11-12:
    /// write walker counts against density). The race world's, measured
    /// over seeds 1-36 at scale 2 and 5,000 walkers (so the
    /// trigger is 38 to 42 by seed there, where it was 40 on every seed).
    pub mine_density: f32,
    /// Provoke and Still (2026-11-17): their reach (dish cells), how long
    /// each lasts and its price. Provoke must outlast the wither lag and
    /// the lean (5 s and 1.5 s for the default slime) to make a panic.
    pub mind_r: f32,
    pub provoke_secs: f32,
    pub provoke_price: f32,
    pub still_secs: f32,
    pub still_price: f32,
    /// **The device slot's cooldowns**, seconds (`KitRules::cooldown`;
    /// owner 2026-11-21 06:55Z, his 11-16 gun-system ruling built: "One
    /// device slot per squad, on a cooldown"). In the squad mode each squad
    /// sets off one tool at a time from its own slot (`Cmd::Device`), free,
    /// and the slot waits this long, by the tool, before the next: the
    /// cooldown is the device's own, guns keep none. First guesses,
    /// untuned: the blast, the commonest, quickest; the gas and the germ,
    /// which hold the slime longest, slowest.
    pub blast_cd: f32,
    pub mine_cd: f32,
    pub cord_cd: f32,
    pub germ_cd: f32,
    pub gas_cd: f32,
    /// Provoke's and Still's.
    pub mind_cd: f32,
}

impl KitRules {
    /// How long a squad's device slot waits after tool `t` (`blast_cd`
    /// and on); 0 for the smell, which is no device.
    pub fn cooldown(&self, t: Tool) -> f32 {
        match t {
            Tool::Blast(_) => self.blast_cd,
            Tool::Mine(_) => self.mine_cd,
            Tool::Cord(..) => self.cord_cd,
            Tool::Germ(_) => self.germ_cd,
            Tool::Gas(_) => self.gas_cd,
            Tool::Provoke(_) | Tool::Still(_) => self.mind_cd,
            Tool::Smell(..) => 0.0,
        }
    }

    /// The cloud's keep a fine step (0: no cloud) and its linger in fine
    /// steps, with `fine` fine steps a game second.
    pub fn gas_cloud(&self, fine: f32) -> (f32, u16) {
        let keep = if self.gas_half > 0.0 { 0.5f32.powf(1.0 / (self.gas_half * fine)) } else { 0.0 };
        (keep, (self.gas_linger * fine).round().clamp(0.0, u16::MAX as f32) as u16)
    }
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
            cord_cards: 8.0,
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
            gas_half: 4.0,
            gas_dose: 1.0,
            gas_sleep: 0.2,
            gas_linger: 3.0,
            gas_immune: 20.0,
            reek: 4.0,
            // Round 2 (2026-11-12): the runner holds 5 at 50 each. At 2 the
            // loud plan fell to the farm hand's level on every tier (+0.18
            // to +0.27) and plague led all of them; weight 100 or 50 made
            // no difference (the cost is the trips home, not the carry).
            // Weight 0 since 2026-11-14: in the window a soldier selected at
            // the larder took 5 tools (250 of its 300 carry) without a
            // sign, so it went home after 50 food and walked straight back
            // to the larder when sent to harvest. The owner read it as
            // soldiers refusing a harvest order. `tool_weight=50` (kitplans)
            // plays the old rule.
            carry_max: 5,
            carry_each: 1,
            tool_weight: 0.0,
            restock_r: 6.0,
            alarm_keep: 0.95,
            mine_density: MINE_DENSITY,
            // A first guess, untuned: the gas's reach, a little cheaper.
            mind_r: 12.0,
            provoke_secs: 12.0,
            provoke_price: 100.0,
            still_secs: 10.0,
            still_price: 100.0,
            blast_cd: 20.0,
            mine_cd: 25.0,
            cord_cd: 30.0,
            germ_cd: 40.0,
            gas_cd: 35.0,
            mind_cd: 30.0,
        }
    }
}

/// The race world's density (`Game::density`) at scale 2 and 5,000 walkers,
/// seeds 1-36 (0.0337 to 0.0368, mean 0.0349; `the_mine_trigger_follows_density`).
pub const MINE_DENSITY: f32 = 0.035;

/// A tool use; points in fine cells, as soldiers' are.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Tool {
    Blast((f32, f32)),
    Mine((f32, f32)),
    Cord((f32, f32), (f32, f32)),
    Germ((f32, f32)),
    Gas((f32, f32)),
    /// Soldier k's smell multiplier (0 mask, 1 plain, `reek`).
    Smell(usize, f32),
    Provoke((f32, f32)),
    Still((f32, f32)),
}

pub const TOOLS: [&str; 8] = ["blast", "mine", "cord", "germ", "gas", "smell", "provoke", "still"];

impl Tool {
    pub fn index(self) -> usize {
        match self {
            Tool::Blast(_) => 0,
            Tool::Mine(_) => 1,
            Tool::Cord(..) => 2,
            Tool::Germ(_) => 3,
            Tool::Gas(_) => 4,
            Tool::Smell(..) => 5,
            Tool::Provoke(_) => 6,
            Tool::Still(_) => 7,
        }
    }
}

/// One blast to draw: where, its radius (fine cells), the tick it went off.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bang {
    pub at: (f32, f32),
    pub r: f32,
    pub tick: u64,
    pub killed: usize,
}

#[derive(Clone, Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Kit {
    pub rules: KitRules,
    /// Mines laid and not yet gone off, fine cells.
    pub mines: Vec<(f32, f32)>,
    /// Walkers within `mine_r` of each mine (`mines`' order) at the last
    /// step's check (`frame`), so the window can show how near each is to
    /// going off without counting again.
    pub mine_near: Vec<u32>,
    pub bangs: Vec<Bang>,
    /// Per tool (`TOOLS` order): uses, bank spent, walkers killed.
    pub uses: [u32; 8],
    pub spent: [f32; 8],
    pub killed: [usize; 8],
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
            Tool::Provoke(_) => r.provoke_price,
            Tool::Still(_) => r.still_price,
        }
    }

    /// Whether a living soldier is within `reach` of `p` (with the carry
    /// rule, one holding a tool).
    pub fn in_reach<W: World>(&self, g: &Game<W>, p: (f32, f32)) -> bool {
        self.user(g, p).is_some()
    }

    /// The nearest living soldier within reach of `p` that may use a tool.
    fn user<W: World>(&self, g: &Game<W>, p: (f32, f32)) -> Option<usize> {
        let r = self.rules.reach * g.world.scale();
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

    /// Who would throw at `p`, for the aiming ghost: the nearest living
    /// soldier that may use a tool (with the carry rule, one holding one),
    /// in reach or not, and whether it is in reach. `user` is the same
    /// soldier when it is in reach, so the ghost's thrower is the one whose
    /// held tool `use_tool` spends.
    pub fn thrower<W: World>(&self, g: &Game<W>, p: (f32, f32)) -> Option<(usize, bool)> {
        let r = self.rules.reach * g.world.scale();
        let d = |k: usize| (g.soldiers[k].x - p.0).hypot(g.soldiers[k].y - p.1);
        (0..g.soldiers.len())
            .filter(|&k| g.soldiers[k].alive && (self.rules.carry_max == 0 || self.held(k) > 0))
            .min_by(|&a, &b| d(a).total_cmp(&d(b)))
            .map(|k| (k, d(k) <= r))
    }

    /// Tools soldier `k` holds.
    pub fn held(&self, k: usize) -> u32 {
        self.held.get(k).copied().unwrap_or(0)
    }

    /// Soldier `k` takes tools at the larder up to `n` held (and
    /// `carry_max`); returns whether it is close enough. Nothing is paid
    /// here: a tool is paid when used.
    pub fn restock<W: World>(&mut self, g: &mut Game<W>, k: usize, n: u32) -> bool {
        let s = g.soldiers[k];
        let r = self.rules.restock_r * g.world.scale();
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
    /// was used. Logs nothing in the game's replay (CPU prototype); the
    /// squad mode's devices do (`Cmd::Device`, `Kit::device`).
    pub fn use_tool<W: World>(&mut self, g: &mut Game<W>, t: Tool) -> bool {
        let price = self.price(t);
        if g.bank < price {
            return false;
        }
        let ok = match t {
            Tool::Smell(k, _) => g.soldiers.get(k).is_some_and(|x| x.alive),
            _ => self.aimable(g, t) && self.in_reach(g, Self::aim(t)),
        };
        if !ok {
            return false;
        }
        if self.rules.carry_max > 0 && !matches!(t, Tool::Smell(..)) {
            let k = self.user(g, Self::aim(t)).expect("in reach");
            self.held[k] -= 1;
            g.soldiers[k].load = self.held[k] as f32 * self.rules.tool_weight;
        }
        g.bank -= price;
        g.spent += price;
        self.spent[t.index()] += price;
        self.cast(g, t);
        true
    }

    /// Where tool `t` is aimed: its point, a cord's middle (the smell's
    /// soldier has none: the origin).
    pub fn aim(t: Tool) -> (f32, f32) {
        match t {
            Tool::Blast(p) | Tool::Mine(p) | Tool::Germ(p) | Tool::Gas(p) | Tool::Provoke(p) | Tool::Still(p) => p,
            Tool::Cord(a, b) => ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0),
            Tool::Smell(..) => (0.0, 0.0),
        }
    }

    /// Whether tool `t` can go off where it is aimed, whoever throws it:
    /// the world casts it, on open ground, a cord no longer than
    /// `cord_max`. Not the smell.
    fn aimable<W: World>(&self, g: &Game<W>, t: Tool) -> bool {
        let p = Self::aim(t);
        match t {
            Tool::Germ(_) if !g.world.has_germ() => false,
            Tool::Gas(_) if !g.world.has_gas() => false,
            Tool::Smell(..) => false,
            Tool::Cord(a, b) => (a.0 - b.0).hypot(a.1 - b.1) <= self.rules.cord_max * g.world.scale(),
            _ => g.world.open(p.0, p.1),
        }
    }

    /// The living soldier of squad `q` nearest `p`, and whether it is
    /// within `reach`: who throws squad `q`'s device there (`Kit::device`),
    /// for the aiming ghost too.
    pub fn squad_thrower<W: World>(&self, g: &Game<W>, q: usize, p: (f32, f32)) -> Option<(usize, bool)> {
        let r = self.rules.reach * g.world.scale();
        let d = |k: usize| (g.soldiers[k].x - p.0).hypot(g.soldiers[k].y - p.1);
        g.members(q).into_iter().min_by(|&a, &b| d(a).total_cmp(&d(b))).map(|k| (k, d(k) <= r))
    }

    /// Squad `q`'s device slot sets tool `t` off (`Cmd::Device`, which
    /// keeps the slot's cooldown): a soldier of the squad within `reach`
    /// throws it, free, with no tool held (the carry rule is the RTS
    /// round's). Returns whether it went off.
    pub fn device<W: World>(&mut self, g: &mut Game<W>, q: usize, t: Tool) -> bool {
        if !self.aimable(g, t) || !self.squad_thrower(g, q, Self::aim(t)).is_some_and(|(_, near)| near) {
            return false;
        }
        self.cast(g, t);
        true
    }

    /// Tool `t` goes off, paid and thrown: counted, then its effect.
    fn cast<W: World>(&mut self, g: &mut Game<W>, t: Tool) {
        let s = g.world.scale();
        let i = t.index();
        self.uses[i] += 1;
        let r = self.rules;
        let br = self.burst_rules(g);
        match t {
            Tool::Blast(p) => {
                self.killed[i] += self.bang(g, p, r.blast_r, r.crater_r);
                Self::carry(g, p);
            }
            Tool::Mine(p) => {
                self.mines.push(p);
                self.mine_near.push(0);
            }
            Tool::Cord(a, b) => {
                let len = (a.0 - b.0).hypot(a.1 - b.1);
                let n = (len / (r.cord_every * s)).ceil().max(1.0) as usize;
                for j in 0..=n {
                    let f = j as f32 / n as f32;
                    let p = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
                    self.killed[i] += self.bang(g, p, r.cord_r, r.cord_r);
                }
                let m = (len / (r.cord_cards * s)).ceil().max(1.0) as usize;
                for j in 0..=m {
                    let f = j as f32 / m as f32;
                    Self::carry(g, (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f));
                }
            }
            Tool::Germ(p) => {
                g.world.burst(&Burst { at: (p.0 / s, p.1 / s), r: r.germ_r, germ: true, ..Burst::default() }, &br);
                Self::carry(g, p);
            }
            Tool::Gas(p) => {
                let steps = (r.gas_secs * self.fine_tps(g)).round().min(u16::MAX as f32) as u16;
                self.gassed += g.world.burst(&Burst { at: (p.0 / s, p.1 / s), r: r.gas_r, gas: steps, ..Burst::default() }, &br).gassed as usize;
                Self::carry(g, p);
            }
            Tool::Smell(k, m) => g.soldiers[k].smell = m,
            Tool::Provoke(p) | Tool::Still(p) => {
                let (kind, secs) = if matches!(t, Tool::Provoke(_)) { (super::big::MIND_DEAF, r.provoke_secs) } else { (super::big::MIND_STILL, r.still_secs) };
                let mind_steps = (secs * self.fine_tps(g)).round().clamp(1.0, u16::MAX as f32) as u16;
                g.world.burst(&Burst { at: (p.0 / s, p.1 / s), r: r.mind_r, mind: kind as u8, ..Burst::default() }, &BurstRules { mind_steps, ..br });
                Self::carry(g, p);
            }
        }
    }

    /// **A device carries gun cards** (gun step E, 2026-11-24; the owner's
    /// 11-16 ruling, "a body holding one to three cards in the gun's
    /// language"): where a device goes off (`p`, fine cells) every cast of
    /// the devices' cards (`Game::device_deck`) goes off too, as a hit
    /// there would cast it (`Rounds::cast_cards_at`), with the same harm to
    /// the squad: a Torch on a blast lights the ground it lands on, and
    /// burns soldiers who stand in it as a Tinder brush does. A mine
    /// carries them to where the slime sets it off.
    fn carry<W: World>(g: &mut Game<W>, p: (f32, f32)) {
        let Some(d) = g.device_deck else { return };
        let cards = d.cards();
        if !cards.is_empty() {
            g.with_rounds(|rd, g| rd.cast_cards_at(g, p, &cards));
        }
    }

    /// Fine steps a game second.
    fn fine_tps<W: World>(&self, g: &Game<W>) -> f32 {
        g.rules.tps as f32 * g.world.fine_steps() as f32
    }

    /// The bursts' shared settings, from these rules.
    fn burst_rules<W: World>(&self, g: &Game<W>) -> BurstRules {
        let r = &self.rules;
        let fine = self.fine_tps(g);
        let sick_steps = (r.sick_secs * fine).round().max(1.0) as u16;
        let (gas_keep, gas_linger) = r.gas_cloud(fine);
        BurstRules {
            alarm_keep: r.alarm_keep,
            gas_death: r.gas_death,
            gas_keep,
            gas_dose: r.gas_dose,
            gas_sleep: r.gas_sleep,
            gas_linger,
            sick_steps,
            immune_steps: (r.immune_secs * fine).round().min(u16::MAX as f32 - sick_steps as f32) as u16,
            gas_immune: (r.gas_immune * fine).round().clamp(0.0, u16::MAX as f32) as u16,
            sick_upkeep: r.sick_upkeep,
            sick_catch: r.sick_catch,
            sick_deposit: r.sick_deposit,
            sick_keep: r.sick_keep,
            // The toolkit throws nothing (the rounds' `Bomb` does), nor
            // has a Mind bullet (the rounds' `Blind` and on do).
            fling_speed: 0.0,
            mind_steps: 0,
            spin_curl: 0.0,
            // The devices set `mind_steps` to their own seconds (`use_`).
            deaf_steps: 0,
            still_steps: 0,
            mind_rest: 0,
        }
    }

    /// One blast: kill, clear, alarm, crater, as one burst. Fine-cell
    /// point, dish-cell radii.
    fn bang<W: World>(&mut self, g: &mut Game<W>, p: (f32, f32), r: f32, crater: f32) -> usize {
        let s = g.world.scale();
        let steps = (self.rules.crater_secs * self.fine_tps(g)).round().min(u16::MAX as f32) as u16;
        let b = Burst { at: (p.0 / s, p.1 / s), r, kill: true, alarm: self.rules.blast_alarm, alarm_r: 2.0 * r, crater: steps, crater_r: crater, ..Burst::default() };
        let br = self.burst_rules(g);
        let killed = g.world.burst(&b, &br).killed as usize;
        self.bangs.push(Bang { at: p, r: r * s, tick: g.world.ticks(), killed });
        killed
    }

    /// Before each game step: mines with `mine_trigger` walkers within
    /// `mine_r` go off. Returns the mines that went off.
    pub fn frame<W: World>(&mut self, g: &mut Game<W>) -> usize {
        if self.mines.is_empty() {
            return 0;
        }
        let r = self.rules;
        let rr = r.mine_r * g.world.scale();
        let trigger = self.mine_trigger(g);
        self.mine_near = self.mines.iter().map(|&m| g.world.walkers_in(m, rr)).collect();
        let fired: Vec<usize> = (0..self.mines.len()).filter(|&m| self.mine_near[m] >= trigger).collect();
        for &m in fired.iter().rev() {
            self.mine_near.remove(m);
            let p = self.mines.remove(m);
            self.killed[1] += self.bang(g, p, r.blast_r, r.crater_r);
            Self::carry(g, p);
        }
        fired.len()
    }

    /// Walkers that set a mine off in this game: `mine_trigger` at
    /// `mine_density`, in proportion to the game's density.
    pub fn mine_trigger<W: World>(&self, g: &Game<W>) -> u32 {
        let r = &self.rules;
        if r.mine_density <= 0.0 {
            return r.mine_trigger;
        }
        ((r.mine_trigger as f32 * g.density / r.mine_density).round() as u32).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::larder_world::{race_world, Rules};

    /// A mine means the same share of mould at any walker count: 40 at the
    /// race world's 5,000 walkers, four times that at 20,000.
    #[test]
    fn the_mine_trigger_follows_density() {
        let kit = Kit::new(KitRules::default());
        let a = Game::new(race_world(5, 2, 5_000), Rules::default(), 5);
        let b = Game::new(race_world(5, 2, 20_000), Rules::default(), 5);
        assert!((a.density / MINE_DENSITY - 1.0).abs() < 0.06, "density {}", a.density);
        assert_eq!(kit.mine_trigger(&a), 40);
        let (ta, tb) = (kit.mine_trigger(&a) as f32, kit.mine_trigger(&b) as f32);
        assert!((tb / ta - 4.0).abs() < 0.1, "{ta} then {tb}");
    }

    /// **Thrown fire goes over walls** (owner, 2026-11-16 19:50Z: "lobbed
    /// fire goes over walls, but regular guns shots do not"): a tool is
    /// thrown, so only the reach and open ground where it lands count, not
    /// the line. A soldier boxed in by wall throws a blast out over it.
    #[test]
    fn a_thrown_tool_goes_over_a_wall() {
        let mut g = Game::new(race_world(3, 1, 1_000), Rules { start_bank: 1e6, ..Rules::default() }, 3);
        let mut kit = Kit::new(KitRules { carry_max: 0, ..KitRules::default() });
        let at = (0..g.world.h as i32 * g.world.w as i32)
            .map(|i| ((i % g.world.w as i32) as f32 + 0.5, (i / g.world.w as i32) as f32 + 0.5))
            .find(|&(x, y)| (-12..=12).all(|d| g.world.open(x + d as f32, y) && g.world.open(x, y + d as f32)))
            .expect("open ground");
        for k in 0..g.soldiers.len() {
            (g.soldiers[k].x, g.soldiers[k].y) = if k == 0 { at } else { (0.5, 0.5) };
        }
        let r = 5.0;
        for (a, b) in [((-r, -r), (r, -r)), ((r, -r), (r, r)), ((r, r), (-r, r)), ((-r, r), (-r, -r))] {
            World::build_wall(&mut g.world, (at.0 + a.0, at.1 + a.1), (at.0 + b.0, at.1 + b.1), 2.0);
        }
        let out = (at.0 + 10.0, at.1);
        assert!(!g.clear_line(at, out), "the wall is in the way of a shot");
        assert!(kit.use_tool(&mut g, Tool::Blast(out)), "the blast went over the wall");
    }

    /// **Provoke and Still** (the simulation-use plan's B1 and B2,
    /// 2026-11-17): each, used where a soldier stands on the mould, stamps
    /// its Mind status on the walkers within `mind_r` for its seconds, is
    /// paid, and kills nothing.
    #[test]
    fn provoke_and_still_stamp_their_status_for_their_seconds() {
        use crate::mould::big::{MIND_DEAF, MIND_STILL, MIND_T};
        for (tool, kind, secs) in [(Tool::Provoke as fn((f32, f32)) -> Tool, MIND_DEAF, KitRules::default().provoke_secs), (Tool::Still, MIND_STILL, KitRules::default().still_secs)] {
            let mut g = Game::new(race_world(3, 2, 5_000), Rules { start_bank: 1e6, ..Rules::default() }, 3);
            let mut kit = Kit::new(KitRules { carry_max: 0, ..KitRules::default() });
            for _ in 0..20 {
                g.step();
            }
            let k = (0..g.world.walkers.len()).find(|&k| g.world.lives(k)).expect("a walker");
            let (x, y, _) = g.world.walkers[k];
            (g.soldiers[0].x, g.soldiers[0].y) = (x, y);
            let (bank, living) = (g.bank, World::living(&g.world));
            assert!(kit.use_tool(&mut g, tool((x, y))), "used");
            let d = &g.world;
            let rr = kit.rules.mind_r * d.p.scale as f32;
            let near: Vec<usize> = (0..d.walkers.len()).filter(|&j| d.lives(j) && (d.walkers[j].0 - x).hypot(d.walkers[j].1 - y) <= rr - 0.01).collect();
            let fine = g.rules.tps as f32 * d.p.substeps as f32;
            let end = (d.steps as u32 + (secs * fine).round() as u32) & MIND_T;
            assert!(near.len() > 10 && near.iter().all(|&j| d.mind_of(j) == kind && d.mind[j] & MIND_T == end), "{kind}: {} walkers near, each stamped to step {end}", near.len());
            assert_eq!(World::living(d), living, "nothing killed");
            assert!(g.bank < bank, "paid");
        }
    }
}
