//! **Turrets you buy** (owner, 2026-11-27 21:36Z): "There is no separate
//! turret vs mortar. You buy "turrets" like you buy guns now. They have
//! different rates of fire, range, some could shoot over walls, Some
//! can't. Some will end up acting more like the mortar and some will end
//! up acting like the current turret. Cost will depend on how good it is.
//! Loadouts are built by combining guns or turrets with bullet modifier,
//! etc cards."
//!
//! A `Turret` is a design the run owns: its kind (how it delivers a shot,
//! the building it raises: `Kind::Turret`, `Mortar`, `Sprayer`, `Hunter`)
//! and its rolls against that kind's `BuildRules` numbers: reach and rate
//! as multipliers, whether it fires over walls, and its card rack. Rolled
//! as a gun is (`guns`): each stat the kind's average times
//! `exp(sigma z)`, `z` a standard normal draw, the averages lifted a
//! little a planet deeper; luck is the normal curve and nothing is
//! labelled lucky. Its price in shop points follows how good it is
//! (`Turret::value`); its food price on a planet is its kind's whatever
//! its rolls (owner, 21:42Z: "turret food cost doesn't change based on
//! quality. Only the shop cost is variable"). Before this, a
//! turret, a mortar, a sprayer and a hunter were each one fixed building
//! unlocked once in the shop (the turrets thread, 2026-11-16/17, and
//! 11-24), and from 11-27 05:50Z to 21:36Z a turret fired its loadout's
//! gun instead (#871, #875).
//!
//! Prices and spreads are first guesses (build first, balance later).

use super::guns::Gun;

/// How a turret delivers its shot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TurretKind {
    /// Aimed shots at the thickest slime in reach (today's turret).
    Gun,
    /// Lobbed shells from far off, over walls, never close in (the mortar).
    Lobber,
    /// Lays its cards in a strip across the slime's way (the sprayer).
    Sprayer,
    /// One fast flat shot at what matters most (the hunter).
    Hunter,
}

impl TurretKind {
    pub const ALL: [TurretKind; 4] = [TurretKind::Gun, TurretKind::Lobber, TurretKind::Sprayer, TurretKind::Hunter];

    pub fn name(self) -> &'static str {
        match self {
            TurretKind::Gun => "Gun turret",
            TurretKind::Lobber => "Lobber",
            TurretKind::Sprayer => "Sprayer",
            TurretKind::Hunter => "Hunter",
        }
    }

    /// One word, for saves and replays.
    pub fn word(self) -> &'static str {
        match self {
            TurretKind::Gun => "gun",
            TurretKind::Lobber => "lobber",
            TurretKind::Sprayer => "sprayer",
            TurretKind::Hunter => "hunter",
        }
    }

    pub fn parse(s: &str) -> Option<TurretKind> {
        TurretKind::ALL.into_iter().find(|k| k.word() == s)
    }

    /// How it fires, for the shop's line.
    pub fn about(self) -> &'static str {
        match self {
            TurretKind::Gun => "Fires its cards at the thickest slime in reach.",
            TurretKind::Lobber => "Lobs shells over walls from far off; can't hit close in.",
            TurretKind::Sprayer => "Lays its cards in a strip across the slime's way.",
            TurretKind::Hunter => "One fast shot at what matters most: slime on crystals, mini-bosses, raids.",
        }
    }

    /// Whether every one of the kind fires over walls (a lobbed shell);
    /// the others roll it (`OVER_CHANCE`).
    pub fn always_over(self) -> bool {
        self == TurretKind::Lobber
    }

    /// Whether it rolls the over-walls trait at all (the hunter's flat
    /// shot never goes over).
    pub fn rolls_over(self) -> bool {
        matches!(self, TurretKind::Gun | TurretKind::Sprayer)
    }
}

/// **A turret family** (owner, 2026-11-28 05:49Z: "There should be many
/// familys of turrets with there base stats randomly selected from a
/// distribution and cost based on how good the turret is"; 06:05Z, yes to
/// families as a firing style plus a stat profile). A family is to a
/// turret what a `GunKind` is to a gun: one firing style (`TurretKind`,
/// the building it raises) and the average of every stat at planet 1.
/// Each design is rolled round that average as a gun is (`Turret::roll`),
/// wide enough that two families of one style overlap. Before this a
/// design was its style's one average with only reach, rate and rack
/// rolled (#876, 11-27).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Family {
    /// Aimed, the turret as it always was.
    #[default]
    Sentry,
    /// Aimed, fast and short, a little wild.
    Repeater,
    /// Aimed, slow and far, a big budget in few slots.
    Picket,
    /// Aimed, a fan of shots each pull, close in.
    Flak,
    /// Lobbed, the mortar.
    Mortar,
    /// Lobbed, shorter and quicker, a cluster of small shells.
    Bombard,
    /// Lobbed, the farthest and slowest, one big shell.
    Howitzer,
    /// Sprayed, a strip across the slime's way.
    Sprayer,
    /// Sprayed, a wide short arc.
    Fan,
    /// Picky, one fast flat shot at what matters most.
    Hunter,
    /// Picky, lighter shots twice as often, nearer.
    Lancer,
}

/// A family's average at planet 1. `range`, `rate` and `blast` are times
/// its style's `BuildRules` numbers (the turret's 24 cells and 2 shots a
/// second, the mortar's 80 cells, a shell every 4 s and a 3-cell strike,
/// the sprayer's 24 cells, 1.5 s and 60 degrees, the hunter's 60 cells,
/// 5 s and 3 cells), so a tuning key moves every family of a style;
/// `spread` is dish cells (aimed, lobbed) or degrees (sprayed);
/// `shots` is bullets a shot, shells a volley or spots a spray.
#[derive(Clone, Copy, Debug)]
pub struct Average {
    pub range: f32,
    pub rate: f32,
    pub spread: f32,
    pub shots: u32,
    pub blast: f32,
    pub slots: u32,
    pub budget: u32,
    /// The share that roll firing over walls (styles that roll it).
    pub over: f32,
    /// The first planet whose shop sells it.
    pub from: u32,
}

impl Family {
    pub const ALL: [Family; 11] = [Family::Sentry, Family::Repeater, Family::Picket, Family::Flak, Family::Mortar, Family::Bombard, Family::Howitzer, Family::Sprayer, Family::Fan, Family::Hunter, Family::Lancer];

    pub fn style(self) -> TurretKind {
        match self {
            Family::Sentry | Family::Repeater | Family::Picket | Family::Flak => TurretKind::Gun,
            Family::Mortar | Family::Bombard | Family::Howitzer => TurretKind::Lobber,
            Family::Sprayer | Family::Fan => TurretKind::Sprayer,
            Family::Hunter | Family::Lancer => TurretKind::Hunter,
        }
    }

    /// The family a style's plain design is (what an old save's design
    /// and the old unlocks become).
    pub fn first(kind: TurretKind) -> Family {
        match kind {
            TurretKind::Gun => Family::Sentry,
            TurretKind::Lobber => Family::Mortar,
            TurretKind::Sprayer => Family::Sprayer,
            TurretKind::Hunter => Family::Hunter,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Family::Sentry => "Sentry",
            Family::Repeater => "Repeater",
            Family::Picket => "Picket",
            Family::Flak => "Flak",
            Family::Mortar => "Mortar",
            Family::Bombard => "Bombard",
            Family::Howitzer => "Howitzer",
            Family::Sprayer => "Sprayer",
            Family::Fan => "Fan",
            Family::Hunter => "Hunter",
            Family::Lancer => "Lancer",
        }
    }

    /// One word, for saves and replays (none a `TurretKind` word but the
    /// sprayer's and hunter's, which the word's length tells apart).
    pub fn word(self) -> &'static str {
        match self {
            Family::Sentry => "sentry",
            Family::Repeater => "repeater",
            Family::Picket => "picket",
            Family::Flak => "flak",
            Family::Mortar => "mortar",
            Family::Bombard => "bombard",
            Family::Howitzer => "howitzer",
            Family::Sprayer => "sprayer",
            Family::Fan => "fan",
            Family::Hunter => "hunter",
            Family::Lancer => "lancer",
        }
    }

    pub fn parse(s: &str) -> Option<Family> {
        Family::ALL.into_iter().find(|f| f.word() == s)
    }

    /// How it fires, for the shop's line.
    pub fn about(self) -> &'static str {
        match self {
            Family::Sentry => "Aimed shots at the thickest slime in reach.",
            Family::Repeater => "Aimed, fast and short, a little wild. More slots.",
            Family::Picket => "Aimed, slow and far. A big budget in few slots.",
            Family::Flak => "Aimed, a fan of shots each pull, close in.",
            Family::Mortar => "Lobs shells over walls from far off; can't hit close in.",
            Family::Bombard => "Lobs a cluster of small shells over walls, nearer and quicker than a mortar.",
            Family::Howitzer => "Lobs one big shell over walls from farther than anything; slow.",
            Family::Sprayer => "Lays its cards in a strip across the slime's way.",
            Family::Fan => "Lays its cards in a wide, short arc.",
            Family::Hunter => "One fast shot at what matters most: slime on crystals, mini-bosses, raids.",
            Family::Lancer => "A hunter's picky aim with lighter shots, twice as often, nearer.",
        }
    }

    /// Its average at planet 1 (first guesses; build first, balance later).
    pub fn average(self) -> Average {
        let a = |range, rate, spread, shots, blast, slots, budget, over, from| Average { range, rate, spread, shots, blast, slots, budget, over, from };
        match self {
            Family::Sentry => a(1.0, 1.0, 0.5, 1, 1.0, 3, 6, 0.35, 1),
            Family::Repeater => a(0.67, 2.2, 1.5, 1, 1.0, 4, 5, 0.2, 1),
            Family::Picket => a(1.6, 0.45, 0.2, 1, 1.0, 2, 9, 0.5, 2),
            Family::Flak => a(0.6, 0.6, 5.0, 4, 1.0, 3, 6, 0.2, 2),
            Family::Mortar => a(1.0, 1.0, 0.0, 1, 1.0, 2, 6, 1.0, 1),
            Family::Bombard => a(0.65, 1.8, 4.0, 3, 0.65, 3, 6, 1.0, 3),
            Family::Howitzer => a(1.35, 0.55, 0.0, 1, 1.7, 2, 10, 1.0, 4),
            Family::Sprayer => a(1.0, 1.0, 60.0, 6, 1.0, 3, 6, 0.35, 1),
            Family::Fan => a(0.6, 1.5, 120.0, 9, 1.0, 3, 5, 0.2, 3),
            Family::Hunter => a(1.0, 1.0, 0.0, 1, 1.0, 2, 5, 0.0, 2),
            Family::Lancer => a(0.7, 2.2, 0.0, 1, 0.6, 2, 5, 0.0, 4),
        }
    }

    /// Whether its shots-a-pull number is rolled (a fan, a cluster, a
    /// spray): those with more than one.
    fn rolls_shots(self) -> bool {
        self.average().shots > 1
    }

    /// The fewest and most shots a pull it rolls.
    fn shots_range(self) -> (u32, u32) {
        match self.style() {
            TurretKind::Sprayer => (3, 14),
            _ => (2, 6),
        }
    }
}

/// Spread of each roll (sigma of its log), as `guns`: range, rate,
/// spread, budget, blast.
const SIGMA: [f32; 5] = [0.12, 0.15, 0.3, 0.15, 0.15];
/// Spread of the slots' and the shots' rolls before rounding (as `guns`).
const SLOT_SIGMA: f32 = 0.7;
const SHOT_SIGMA: f32 = 1.0;
/// Reach and rate a planet deeper, and their cap (as `guns::DEPTH_STAT`);
/// the budget as `guns::DEPTH_BUDGET`, slots as `guns::DEPTH_SLOTS`.
pub const DEPTH_STAT: f32 = 0.04;
pub const DEPTH_STAT_MAX: f32 = 1.4;
pub const DEPTH_BUDGET: f32 = 0.12;
pub const DEPTH_BUDGET_MAX: f32 = 2.5;
pub const DEPTH_SLOTS: f32 = 0.1;
pub const DEPTH_SLOTS_MAX: f32 = 0.6;
/// What firing over walls is worth in price and `Turret::value`, where
/// the style rolls it: the owner's "obviously a huge strength" (05:52Z).
pub const OVER_VALUE: f32 = 1.35;
pub const SLOTS_RANGE: (u32, u32) = (1, 6);
pub const BUDGET_RANGE: (u32, u32) = (2, 30);

/// A turret design the run owns or the shop sells: a family's numbers,
/// each rolled (owner, 2026-11-28: turrets mirror the guns).
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Turret {
    /// Its firing style, the building it raises (`Family::style`).
    pub kind: TurretKind,
    #[serde(default)]
    pub family: Family,
    /// Its reach, times its style's (`BuildRules::turret_range`, ...).
    pub range: f32,
    /// Its rate of fire, times its style's (its gap divided by this).
    pub rate: f32,
    /// A fan's or cluster's width, dish cells; a spray's arc, degrees.
    #[serde(default)]
    pub spread: f32,
    /// Bullets a shot, shells a volley, spots a spray.
    #[serde(default = "one")]
    pub shots: u32,
    /// A shell's strike, times its style's (lobbed and picky styles).
    #[serde(default = "onef")]
    pub blast: f32,
    /// Whether it sees and fires over walls (rock and water still block).
    pub over: bool,
    /// Its card rack.
    pub slots: u32,
    pub budget: u32,
    /// The planet it was rolled for (from 1).
    pub depth: u32,
    /// The roll against its family's average at its depth, as
    /// `Gun::grade`: 1 is average.
    #[serde(default = "onef")]
    pub grade: f32,
}

fn one() -> u32 {
    1
}

fn onef() -> f32 {
    1.0
}

impl Turret {
    /// A family's average design at planet 1.
    pub fn average(family: Family) -> Turret {
        let a = family.average();
        let over = family.style().always_over() || (family.style().rolls_over() && a.over >= 0.5);
        Turret { kind: family.style(), family, range: a.range, rate: a.rate, spread: a.spread, shots: a.shots, blast: a.blast, over, slots: a.slots, budget: a.budget, depth: 1, grade: 1.0 }
    }

    /// The style's plain design at planet 1: what a run starts with (the
    /// Sentry, over walls as turrets always were), what an old unlock
    /// becomes (`Run::from_text`), and the fixed building outside a run.
    pub fn plain(kind: TurretKind) -> Turret {
        let f = Family::first(kind);
        let mut t = Turret::average(f);
        t.over = kind.always_over() || kind == TurretKind::Gun || kind == TurretKind::Sprayer;
        t
    }

    /// A design of `family` rolled from `z` for planet `depth`: each stat
    /// the family's average (lifted a planet deeper as a gun's) times
    /// `exp(sigma z)`, `z` a standard normal draw; slots and shots the
    /// average plus a normal draw, rounded (as `guns::roll_with`).
    pub fn roll_family(family: Family, z: u64, depth: u32) -> Turret {
        let mut z = z;
        let mut u = || {
            z = split(z);
            ((z >> 11) as f32 / (1u64 << 53) as f32).clamp(1e-7, 1.0 - 1e-7)
        };
        let mut normal = || (-2.0 * u().ln()).sqrt() * (std::f32::consts::TAU * u()).cos();
        let a = family.average();
        let d = depth.max(1) as f32 - 1.0;
        let lift = (1.0 + DEPTH_STAT * d).min(DEPTH_STAT_MAX);
        let blift = (1.0 + DEPTH_BUDGET * d).min(DEPTH_BUDGET_MAX);
        let (zr, zt, zs, zb, zn, zk, zx, zo) = (normal(), normal(), normal(), normal(), normal(), normal(), normal(), u());
        let (ar, at, asp, ab) = (a.range * lift, a.rate * lift, a.spread / lift, a.budget as f32 * blift);
        let range = round2(ar * (SIGMA[0] * zr).exp());
        let rate = round2(at * (SIGMA[1] * zt).exp());
        // A spray's arc is a strength (wider), a gun's fan a weakness
        // (wilder), as `guns`' spread; a cluster's spread neither, rolled.
        let spread = if asp > 0.0 { round2(asp * (SIGMA[2] * zs).exp()) } else { 0.0 };
        let budget = ((ab * (SIGMA[3] * zb).exp()).round() as u32).clamp(BUDGET_RANGE.0, BUDGET_RANGE.1);
        let deeper = (DEPTH_SLOTS * d).min(DEPTH_SLOTS_MAX);
        let slots = (a.slots as f32 + deeper + SLOT_SIGMA * zn).round().clamp(SLOTS_RANGE.0 as f32, SLOTS_RANGE.1 as f32) as u32;
        let (lo, hi) = family.shots_range();
        let shots = if family.rolls_shots() { (a.shots as f32 + SHOT_SIGMA * zk).round().clamp(lo as f32, hi as f32) as u32 } else { 1 };
        let blast = if matches!(family.style(), TurretKind::Lobber | TurretKind::Hunter) { round2(a.blast * (SIGMA[4] * zx).exp()) } else { 1.0 };
        let kind = family.style();
        let over = kind.always_over() || (kind.rolls_over() && zo < a.over);
        let mut parts = vec![range / ar, rate / at, budget as f32 / ab, slots as f32 / (a.slots as f32 + deeper)];
        if asp > 0.0 && kind == TurretKind::Gun {
            parts.push(asp / spread.max(1e-3));
        }
        if asp > 0.0 && kind == TurretKind::Sprayer {
            parts.push(spread / asp);
        }
        if family.rolls_shots() {
            parts.push(shots as f32 / a.shots as f32);
        }
        if blast != 1.0 || matches!(kind, TurretKind::Lobber | TurretKind::Hunter) {
            parts.push(blast / a.blast);
        }
        let grade = round2((parts.iter().map(|p| p.ln()).sum::<f32>() / parts.len() as f32).exp());
        Turret { kind, family, range, rate, spread, shots, blast, over, slots, budget, depth: depth.max(1), grade }
    }

    /// A design of a family of `kind`'s style, for planet `depth`: the
    /// families the shop sells there, one drawn from `z`.
    pub fn roll(kind: TurretKind, z: u64, depth: u32) -> Turret {
        let fams: Vec<Family> = Family::ALL.into_iter().filter(|f| f.style() == kind && f.average().from <= depth.max(1)).collect();
        let f = fams[(split(z ^ 0xFA41) % fams.len().max(1) as u64) as usize];
        Turret::roll_family(f, z, depth)
    }

    /// How good it is against its family's average design at planet 1
    /// (about 1 there; deeper designs more): the geometric mean of its
    /// rolls over that average, over walls `OVER_VALUE` where the style
    /// rolls it. What the strategy bot compares (`league::buy_designs`).
    pub fn value(&self) -> f32 {
        let a = self.family.average();
        let mut parts = vec![self.range / a.range, self.rate / a.rate, self.budget as f32 / a.budget as f32, self.slots as f32 / a.slots as f32];
        if self.family.rolls_shots() {
            parts.push(self.shots as f32 / a.shots as f32);
        }
        if matches!(self.kind, TurretKind::Lobber | TurretKind::Hunter) {
            parts.push(self.blast / a.blast);
        }
        let g = (parts.iter().map(|p| p.max(1e-3).ln()).sum::<f32>() / parts.len() as f32).exp();
        g * if self.over && self.kind.rolls_over() { OVER_VALUE } else { 1.0 }
    }

    /// Its shop price in points, as a gun's (`run::gun_price`): dearer the
    /// deeper it was found and the better it rolled (its grade squared),
    /// and a third more if it rolled firing over walls; in tens.
    pub fn price(&self) -> u32 {
        let over = if self.over && self.kind.rolls_over() { OVER_VALUE } else { 1.0 };
        let p = (110.0 + 40.0 * self.depth as f32) * self.grade.max(0.5).powi(2) * over;
        ((p.clamp(60.0, 1200.0) / 10.0).round() as u32).max(1) * 10
    }

    /// What its loadout's cards fit against (`rounds::fit_gun`).
    pub fn rack(&self) -> Gun {
        Gun { slots: self.slots, budget: self.budget, ..super::run::first_gun() }
    }

    /// Its numbers as the game fires them at the default rules: reach in
    /// dish cells, shots a second, and a shell's strike in dish cells (0
    /// for aimed and sprayed turrets).
    pub fn numbers(&self) -> (f32, f32, f32) {
        let b = super::larder_world::BuildRules::default();
        match self.kind {
            TurretKind::Gun => (b.turret_range * self.range, self.rate / b.turret_every, 0.0),
            TurretKind::Lobber => (b.mortar_range * self.range, self.rate / b.mortar_every, b.mortar_blast * self.blast),
            TurretKind::Sprayer => (b.sprayer_range * self.range, self.rate / b.sprayer_every, 0.0),
            TurretKind::Hunter => (b.hunter_range * self.range, self.rate / b.hunter_every, b.hunter_blast * self.blast),
        }
    }

    /// Its family's name: "Picket".
    pub fn title(&self) -> String {
        self.family.name().to_string()
    }

    /// One line of its numbers: "Picket: 38 cells, 0.9 shots/s, over
    /// walls, 2 slots, budget 9".
    pub fn line(&self) -> String {
        let (reach, rate, blast) = self.numbers();
        let shots = match self.kind {
            TurretKind::Sprayer => format!(", {} spots over {:.0} degrees", self.shots, self.spread),
            _ if self.shots > 1 => format!(", {} a pull", self.shots),
            _ => String::new(),
        };
        let blast = if blast > 0.0 { format!(", strike {blast:.1}") } else { String::new() };
        let over = if self.over { ", over walls" } else { "" };
        format!("{}: {reach:.0} cells, {rate:.2} shots/s{shots}{blast}{over}, {} slot{}, budget {}", self.family.name(), self.slots, if self.slots == 1 { "" } else { "s" }, self.budget)
    }

    /// The save's and replay's word:
    /// `picket/1.62/0.41/0.22/1/1.00/1/2/9/2/1.04` (family, range, rate,
    /// spread, shots, blast, over, slots, budget, depth, grade). Reads the
    /// 7-word form of 11-27 too (`gun/1.05/0.92/1/3/6/2`: a kind, its
    /// reach and rate, over, slots, budget, depth), as its kind's first
    /// family with that family's spread, shots and blast.
    pub fn word(&self) -> String {
        format!("{}/{:.2}/{:.2}/{:.2}/{}/{:.2}/{}/{}/{}/{}/{:.2}", self.family.word(), self.range, self.rate, self.spread, self.shots, self.blast, u8::from(self.over), self.slots, self.budget, self.depth, self.grade)
    }

    pub fn from_word(s: &str) -> Option<Turret> {
        let w: Vec<&str> = s.split('/').collect();
        let f = |i: usize| w[i].parse::<f32>().ok().filter(|v| v.is_finite() && *v > 0.0);
        match w.len() {
            7 => {
                let kind = TurretKind::parse(w[0])?;
                let a = Turret::plain(kind);
                Some(Turret { range: f(1)?, rate: f(2)?, over: w[3] == "1", slots: w[4].parse().ok()?, budget: w[5].parse().ok()?, depth: w[6].parse().ok()?, ..a })
            }
            11 => {
                let family = Family::parse(w[0])?;
                let spread = w[3].parse::<f32>().ok().filter(|v| v.is_finite() && *v >= 0.0)?;
                Some(Turret { kind: family.style(), family, range: f(1)?, rate: f(2)?, spread, shots: w[4].parse().ok()?, blast: f(5)?, over: w[6] == "1", slots: w[7].parse().ok()?, budget: w[8].parse().ok()?, depth: w[9].parse().ok()?, grade: f(10)? })
            }
            _ => None,
        }
    }
}

fn round2(v: f32) -> f32 {
    (v * 100.0).round() / 100.0
}

/// splitmix64's step.
fn split(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rolls spread round the family's average, a better design costs
    /// more, over walls is a minority roll for sentries and every
    /// mortar's, deeper planets roll better, a design survives its word,
    /// and the 11-27 word still reads.
    #[test]
    fn turrets_roll_like_guns_and_price_by_how_good() {
        let n = 2000;
        let rolls: Vec<Turret> = (0..n).map(|z| Turret::roll_family(Family::Sentry, z * 7919 + 3, 1)).collect();
        let mean = |f: &dyn Fn(&Turret) -> f32| rolls.iter().map(f).sum::<f32>() / n as f32;
        assert!((mean(&|t| t.range) - 1.0).abs() < 0.05 && (mean(&|t| t.rate) - 1.0).abs() < 0.05);
        assert!((mean(&|t| t.grade) - 1.0).abs() < 0.05, "{}", mean(&|t| t.grade));
        let over = rolls.iter().filter(|t| t.over).count() as f32 / n as f32;
        assert!((over - Family::Sentry.average().over).abs() < 0.05, "{over}");
        assert!(rolls.iter().all(|t| Turret::from_word(&t.word()).is_some_and(|b| b.word() == t.word())));
        assert!(rolls.iter().all(|t| (SLOTS_RANGE.0..=SLOTS_RANGE.1).contains(&t.slots) && (BUDGET_RANGE.0..=BUDGET_RANGE.1).contains(&t.budget)));
        let p = Turret::plain(TurretKind::Gun);
        assert_eq!(p.family, Family::Sentry);
        assert!((120..=250).contains(&p.price()), "{}", p.price());
        let better = Turret { grade: 1.3, ..p };
        assert!(better.price() > p.price());
        let walled = Turret { over: false, ..p };
        assert!(walled.price() < p.price(), "over walls costs more");
        assert!((0..200).all(|z| Turret::roll(TurretKind::Lobber, z, 6).over), "every lobbed one goes over");
        assert!((0..200).all(|z| !Turret::roll(TurretKind::Hunter, z, 6).over), "no hunter does");
        let deep: f32 = (0..n).map(|z| Turret::roll_family(Family::Sentry, z * 13 + 1, 8).rate).sum::<f32>() / n as f32;
        assert!(deep > 1.2, "{deep}");
        let old = Turret::from_word("lobber/1.10/0.95/1/2/6/2").unwrap();
        assert_eq!((old.family, old.range, old.rate, old.depth), (Family::Mortar, 1.1, 0.95, 2));
    }

    /// Every family is sold somewhere, each in its own style, and a
    /// family's rolls overlap its style's others (a long Sentry reaches
    /// past a short Picket now and then).
    #[test]
    fn every_family_rolls_and_families_overlap() {
        for f in Family::ALL {
            assert_eq!(Turret::average(f).kind, f.style());
            assert_eq!(Family::parse(f.word()), Some(f));
            let seen = (0..400u64).any(|z| Turret::roll(f.style(), z, 8).family == f);
            assert!(seen, "{f:?} never sold");
        }
        assert!((0..400u64).all(|z| Turret::roll(TurretKind::Gun, z, 1).family.average().from <= 1));
        let sentry_far = (0..2000u64).map(|z| Turret::roll_family(Family::Sentry, z, 2).range).fold(0.0f32, f32::max);
        let picket_near = (0..2000u64).map(|z| Turret::roll_family(Family::Picket, z, 2).range).fold(9.0f32, f32::min);
        assert!(sentry_far > picket_near, "{sentry_far} {picket_near}");
        let flak: Vec<u32> = (0..400u64).map(|z| Turret::roll_family(Family::Flak, z, 1).shots).collect();
        assert!(flak.iter().all(|&s| (2..=6).contains(&s)) && flak.iter().any(|&s| s != 4));
    }
}
