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

    /// The average rack at planet 1: (slots, budget).
    fn rack(self) -> (f32, f32) {
        match self {
            TurretKind::Gun => (3.0, 6.0),
            TurretKind::Lobber => (2.0, 6.0),
            TurretKind::Sprayer => (3.0, 6.0),
            TurretKind::Hunter => (2.0, 5.0),
        }
    }
}

/// Spread of each roll (sigma of its log): reach, rate.
const SIGMA: [f32; 2] = [0.15, 0.18];
/// Spread of the slots' and budget's rolls before rounding.
const SLOT_SIGMA: f32 = 0.7;
const BUDGET_SIGMA: f32 = 1.5;
/// Reach and rate a planet deeper, and their cap (as `guns::DEPTH_STAT`).
pub const DEPTH_STAT: f32 = 0.04;
pub const DEPTH_STAT_MAX: f32 = 1.4;
/// The share of gun turrets and sprayers that roll firing over walls:
/// the owner's "obviously a huge strength" (05:52Z), so a minority, and
/// priced (`OVER_VALUE`).
pub const OVER_CHANCE: f32 = 0.35;
/// What firing over walls is worth in `Turret::value`.
pub const OVER_VALUE: f32 = 1.35;
pub const SLOTS_RANGE: (u32, u32) = (1, 5);
pub const BUDGET_RANGE: (u32, u32) = (2, 14);

/// A turret design the run owns or the shop sells.
#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Turret {
    pub kind: TurretKind,
    /// Its reach, times its kind's (`BuildRules::turret_range`, ...).
    pub range: f32,
    /// Its rate of fire, times its kind's (its gap divided by this).
    pub rate: f32,
    /// Whether it sees and fires over walls (rock and water still block).
    pub over: bool,
    /// Its card rack.
    pub slots: u32,
    pub budget: u32,
    /// The planet it was rolled for (from 1).
    pub depth: u32,
}

impl Turret {
    /// The kind's average design at planet 1: what a run that owned the
    /// old unlock gets (`Run::from_text`), and the fixed building outside
    /// a run.
    pub fn plain(kind: TurretKind) -> Turret {
        let (s, b) = kind.rack();
        Turret { kind, range: 1.0, rate: 1.0, over: kind.always_over() || kind == TurretKind::Gun || kind == TurretKind::Sprayer, slots: s as u32, budget: b as u32, depth: 1 }
    }

    /// A design of `kind` rolled from `z` for planet `depth`.
    pub fn roll(kind: TurretKind, z: u64, depth: u32) -> Turret {
        let mut z = z;
        let mut u = || {
            z = split(z);
            ((z >> 11) as f32 / (1u64 << 53) as f32).clamp(1e-7, 1.0 - 1e-7)
        };
        let mut normal = || (-2.0 * u().ln()).sqrt() * (std::f32::consts::TAU * u()).cos();
        let d = depth.max(1) as f32 - 1.0;
        let lift = (1.0 + DEPTH_STAT * d).min(DEPTH_STAT_MAX);
        let range = round2(lift * (SIGMA[0] * normal()).exp());
        let rate = round2(lift * (SIGMA[1] * normal()).exp());
        let (s, b) = kind.rack();
        let slots = (s + 0.1 * d.min(6.0) + SLOT_SIGMA * normal()).round().clamp(SLOTS_RANGE.0 as f32, SLOTS_RANGE.1 as f32) as u32;
        let budget = (b * (1.0 + 0.12 * d).min(2.5) + BUDGET_SIGMA * normal()).round().clamp(BUDGET_RANGE.0 as f32, BUDGET_RANGE.1 as f32) as u32;
        let over = kind.always_over() || (kind.rolls_over() && u() < OVER_CHANCE);
        Turret { kind, range, rate, over, slots, budget, depth: depth.max(1) }
    }

    /// How good it is against its kind's plain design at planet 1 (about
    /// 1 for an average design there): reach counts half (area), rate
    /// whole, over walls `OVER_VALUE` where the kind rolls it, and the
    /// rack a fifth.
    pub fn value(&self) -> f32 {
        let (s, b) = self.kind.rack();
        let rack = (self.slots as f32 / s * self.budget as f32 / b).sqrt();
        let over = if self.over && !self.kind.always_over() { OVER_VALUE } else { 1.0 };
        let base = if self.kind.rolls_over() { 1.0 + (OVER_VALUE - 1.0) * OVER_CHANCE } else { 1.0 };
        self.range.sqrt() * self.rate * over / base * rack.powf(0.2)
    }

    /// Its shop price in points: 150 for an average design, more the
    /// better it is, in tens.
    pub fn price(&self) -> u32 {
        ((150.0 * self.value()).clamp(60.0, 600.0) / 10.0).round() as u32 * 10
    }

    /// What its loadout's cards fit against (`rounds::fit_gun`).
    pub fn rack(&self) -> Gun {
        Gun { slots: self.slots, budget: self.budget, ..super::run::first_gun() }
    }

    /// "Lobber: reach x1.10, rate x0.95, over walls, 2 slots, budget 6".
    pub fn title(&self) -> String {
        let over = if self.over { ", over walls" } else { "" };
        format!("{}: reach x{:.2}, rate x{:.2}{over}, {} slot{}, budget {}", self.kind.name(), self.range, self.rate, self.slots, if self.slots == 1 { "" } else { "s" }, self.budget)
    }

    /// The save's and replay's word: `gun/1.05/0.92/1/3/6/2`.
    pub fn word(&self) -> String {
        format!("{}/{:.2}/{:.2}/{}/{}/{}/{}", self.kind.word(), self.range, self.rate, u8::from(self.over), self.slots, self.budget, self.depth)
    }

    pub fn from_word(s: &str) -> Option<Turret> {
        let w: Vec<&str> = s.split('/').collect();
        if w.len() != 7 {
            return None;
        }
        let f = |i: usize| w[i].parse::<f32>().ok().filter(|v| v.is_finite() && *v > 0.0);
        Some(Turret { kind: TurretKind::parse(w[0])?, range: f(1)?, rate: f(2)?, over: w[3] == "1", slots: w[4].parse().ok()?, budget: w[5].parse().ok()?, depth: w[6].parse().ok()? })
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

    /// Rolls spread round the kind's average, a better design costs more,
    /// over walls is a minority roll for gun turrets and every lobber's,
    /// deeper planets roll better on average, and a design survives its
    /// word.
    #[test]
    fn turrets_roll_like_guns_and_price_by_how_good() {
        let n = 2000;
        let rolls: Vec<Turret> = (0..n).map(|z| Turret::roll(TurretKind::Gun, z * 7919 + 3, 1)).collect();
        let mean = |f: &dyn Fn(&Turret) -> f32| rolls.iter().map(f).sum::<f32>() / n as f32;
        assert!((mean(&|t| t.range) - 1.0).abs() < 0.05 && (mean(&|t| t.rate) - 1.0).abs() < 0.05);
        let over = rolls.iter().filter(|t| t.over).count() as f32 / n as f32;
        assert!((over - OVER_CHANCE).abs() < 0.05, "{over}");
        assert!(rolls.iter().all(|t| Turret::from_word(&t.word()) == Some(*t)));
        assert!(rolls.iter().all(|t| (SLOTS_RANGE.0..=SLOTS_RANGE.1).contains(&t.slots) && (BUDGET_RANGE.0..=BUDGET_RANGE.1).contains(&t.budget)));
        let p = Turret::plain(TurretKind::Gun);
        assert!((120..=180).contains(&p.price()), "{}", p.price());
        let better = Turret { rate: 1.3, ..p };
        assert!(better.price() > p.price());
        let walled = Turret { over: false, ..p };
        assert!(walled.price() < p.price(), "over walls costs more");
        assert!((0..200).all(|z| Turret::roll(TurretKind::Lobber, z, 3).over), "every lobber goes over");
        assert!((0..200).all(|z| !Turret::roll(TurretKind::Hunter, z, 3).over), "no hunter does");
        let deep: f32 = (0..n).map(|z| Turret::roll(TurretKind::Gun, z * 13 + 1, 8).rate).sum::<f32>() / n as f32;
        assert!(deep > 1.2, "{deep}");
    }
}
