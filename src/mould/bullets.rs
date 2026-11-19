//! **Bullets** (owner, 2026-11-14 16:46Z): "Bullets define damage and have
//! multipliers (0.25 to 4x, but it can be decimals 1.4x) for speed, range,
//! spread, there will also be base bullet sets that are more likely to have
//! certain multipliers. The bullet cost will depend on how good the damage
//! and multiplier role plus random luck." And 18:05Z: the payloads (blast,
//! bomb, gas, germ) are bullets, not modifiers.
//!
//! An `Ammo` is one rolled bullet card: a payload (`Shell`: plain, blast,
//! bomb, gas, germ), a set (`Set`) that leans its rolls, and its numbers.
//! In a gun it is a card (`rounds::Card::Ammo`) and takes a slot; the
//! modifiers and trigger before it go with it, and every shot fires the
//! next bullet in turn (`rounds`). The fixed payload cards (`Card::Blast`,
//! ...) are each payload's standard bullet (`Ammo::standard`).
//!
//! **Damage** is how big a hole a hit makes: a strike's radius goes as the
//! square root of damage over `DAMAGE` (so its area, and what it kills, as
//! the damage), and so does a payload's burst. A plain strike stops at
//! `squad::R_MAX` fine cells (the GPU's 15 x 15 strike box): 1.4 times the
//! standard strike on a scale-2 dish. A payload's burst has no cap.
//!
//! **Multipliers** times the gun's: speed (the bullet's flight), range (how
//! far it flies) and spread (the gun's scatter), each in hundredths, from
//! `MULT_MIN` to `MULT_MAX` (0.25 to 4). A shot still aims where its gun
//! sees (`Game::gun_range`); a short bullet falls short.
//!
//! **Rolls** (`Ammo::roll`): each number is its set's lean times
//! `exp(sigma z)`, `z` a standard normal draw, as the guns roll (owner
//! 18:07Z: luck comes out of the normal curve, and nothing is labelled
//! lucky, 18:18Z). Deeper planets roll more damage (`DEPTH_DAMAGE` a
//! planet, up to `DEPTH_DAMAGE_MAX` times).
//!
//! **Cost** (budget points, `Card::cost`): the payload's base (`Shell::base`,
//! the fixed card's cost; plain 1) times the roll's grade to the
//! `COST_POWER` (a better roll costs more) times a luck factor
//! `exp(COST_LUCK z)`, rounded, at least 1.
use super::rounds::Card;
use super::Rng;

/// A standard bullet's damage: a strike of `Rules::strike`, a payload's
/// burst at its toolkit radius.
pub const DAMAGE: f32 = 10.0;
/// Multipliers, in hundredths: 0.25 to 4 times the gun's.
pub const MULT_MIN: u16 = 25;
pub const MULT_MAX: u16 = 400;
/// Spread of each roll (sigma of its log): damage, then the multipliers.
const SIGMA_DAMAGE: f32 = 0.2;
const SIGMA_MULT: f32 = 0.25;
/// Damage a planet deeper (and the cap), as the guns' stats.
pub const DEPTH_DAMAGE: f32 = 0.04;
pub const DEPTH_DAMAGE_MAX: f32 = 1.4;
/// How the cost follows the grade, and its own luck (sigma of its log).
pub const COST_POWER: f32 = 1.5;
pub const COST_LUCK: f32 = 0.15;

/// What a bullet does where it hits: nothing more than its strike (plain),
/// or one of the toolkit's payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Shell {
    Plain,
    Blast,
    Bomb,
    Gas,
    Germ,
    /// The Mind bullets (2026-11-16, `Card::Blind` and on).
    Blind,
    Sour,
    Spin,
    /// The readers (2026-11-16, `Card::Shatter`, `Detonate`).
    Shatter,
    Detonate,
    /// The route family's payload (2026-11-17, `Card::Flare`).
    Flare,
    /// The element family's (2026-11-17, `Card::Torch`).
    Torch,
    /// The Mind family's last two (2026-11-17, `Card::Provoke`, `Still`).
    Provoke,
    Still,
}

impl Shell {
    pub const ALL: [Shell; 14] = [Shell::Plain, Shell::Blast, Shell::Bomb, Shell::Gas, Shell::Germ, Shell::Blind, Shell::Sour, Shell::Spin, Shell::Shatter, Shell::Detonate, Shell::Flare, Shell::Torch, Shell::Provoke, Shell::Still];

    /// The fixed card that is this payload's standard bullet.
    pub fn card(self) -> Card {
        match self {
            Shell::Plain => Card::Plain,
            Shell::Blast => Card::Blast,
            Shell::Bomb => Card::Bomb,
            Shell::Gas => Card::Gas,
            Shell::Germ => Card::Germ,
            Shell::Blind => Card::Blind,
            Shell::Sour => Card::Sour,
            Shell::Spin => Card::Spin,
            Shell::Shatter => Card::Shatter,
            Shell::Detonate => Card::Detonate,
            Shell::Flare => Card::Flare,
            Shell::Torch => Card::Torch,
            Shell::Provoke => Card::Provoke,
            Shell::Still => Card::Still,
        }
    }

    /// One of the Mind bullets (Blind, Sour, Spin, Provoke, Still).
    pub fn mind(self) -> bool {
        matches!(self, Shell::Blind | Shell::Sour | Shell::Spin | Shell::Provoke | Shell::Still)
    }

    /// One of the readers (Shatter, Detonate).
    pub fn reader(self) -> bool {
        matches!(self, Shell::Shatter | Shell::Detonate)
    }

    /// A bullet whose work is a rule on the slime (a Mind status, a
    /// reader's, a flare's alarm, a torch's fire), not its set's numbers:
    /// the gun range's armory has these in the standard set only, on a tab
    /// of their own (`Run::restock`).
    pub fn special(self) -> bool {
        self.mind() || self.reader() || matches!(self, Shell::Flare | Shell::Torch)
    }

    pub fn of(c: Card) -> Option<Shell> {
        Shell::ALL.into_iter().find(|s| s.card() == c)
    }

    pub fn key(self) -> &'static str {
        self.card().name()
    }

    /// Budget points of its standard bullet (the fixed card's cost).
    pub fn base(self) -> u32 {
        self.card().cost()
    }
}

/// A bullet set: which way its rolls lean (owner: "base bullet sets that
/// are more likely to have certain multipliers").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Set {
    /// Every number round 1.
    Standard,
    /// Heavy: more damage, slow, a little short, tight.
    Slug,
    /// Light: little damage, fast, far, tight.
    Needle,
    /// Wide: a little less damage, short, spread wide.
    Buckshot,
}

/// A set's lean: damage, speed, range and spread, each times the standard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lean {
    pub damage: f32,
    pub speed: f32,
    pub range: f32,
    pub spread: f32,
}

impl Set {
    pub const ALL: [Set; 4] = [Set::Standard, Set::Slug, Set::Needle, Set::Buckshot];

    pub fn key(self) -> &'static str {
        match self {
            Set::Standard => "standard",
            Set::Slug => "slug",
            Set::Needle => "needle",
            Set::Buckshot => "buckshot",
        }
    }

    pub fn parse(s: &str) -> Option<Set> {
        Set::ALL.into_iter().find(|k| k.key() == s)
    }

    pub fn lean(self) -> Lean {
        match self {
            Set::Standard => Lean { damage: 1.0, speed: 1.0, range: 1.0, spread: 1.0 },
            Set::Slug => Lean { damage: 1.6, speed: 0.6, range: 0.9, spread: 0.6 },
            Set::Needle => Lean { damage: 0.6, speed: 2.0, range: 1.5, spread: 0.5 },
            Set::Buckshot => Lean { damage: 0.8, speed: 0.9, range: 0.6, spread: 2.5 },
        }
    }

    /// What its bullets do that the other sets' don't (`Shape`).
    pub fn shape(self) -> Shape {
        let one = Shape { pierce: 0, pellets: 1, fan: 0.0, r: 1.0, shove: 0.0, shove_secs: 0.0 };
        match self {
            Set::Standard => one,
            Set::Slug => Shape { shove: SLUG_SHOVE, shove_secs: SLUG_SHOVE_SECS, ..one },
            Set::Needle => Shape { pierce: NEEDLE_PIERCE, ..one },
            Set::Buckshot => Shape { pellets: BUCK_PELLETS, fan: BUCK_FAN, r: BUCK_R, ..one },
        }
    }
}

/// **A set's shape** (2026-11-16, step A of the gun-system review,
/// `Reports/gun-system-review-2026-11-16/README.md`): what a set's bullets
/// do to the slime beyond their numbers, so a set reads by its hits and not
/// by a stat. The owner (02:34Z): "within those all the cards in each
/// category are relatively similar. Just changing in some of the base
/// stats". Until then a set only leaned its rolls (`Set::lean`): at a front
/// that was a hole 1.26x (slug), 0.77x (needle) or 0.89x (buckshot) as wide
/// as a standard one, and speed, range and spread hardly showed with the
/// slime already in reach. Every shell takes its set's shape: a needle
/// blast blasts at each of its hits, a buckshot gas is four small clouds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
    /// Slime hits it flies on through after its first, each striking and
    /// casting, as `Card::Pierce`'s do (and on top of them): a needle's
    /// line of holes into the body.
    pub pierce: u8,
    /// Bullets each of its gun's shots fires, fanned evenly `fan` radians
    /// either side of the aim, each with `r` of the bullet's strike and
    /// burst radius: buckshot's spray of small holes along the front.
    pub pellets: u8,
    pub fan: f32,
    pub r: f32,
    /// A shove (0: none): where it hits slime, the living walkers out to
    /// `shove` strike radii are thrown away from the gun for up to
    /// `shove_secs` (`squad::Burst::fling` with no kill), so a slug pushes
    /// the front back as well as holing it. The knock-back the 11-15 splash
    /// tried as a hit's look (`Reports/dead-ends.md`: kills fell 22% as the
    /// thrown walkers left the next shots' discs) is here the slug's trade.
    pub shove: f32,
    pub shove_secs: f32,
}

/// The needle's extra slime hits: three holes in a line.
pub const NEEDLE_PIERCE: u8 = 2;
/// Buckshot's pellets a shot, their fan either side (radians: about 2.6
/// cells either side at 10 cells, so the holes stand apart) and each
/// pellet's share of the radius (area 0.25 each, a standard hit in all).
pub const BUCK_PELLETS: u8 = 4;
pub const BUCK_FAN: f32 = 0.25;
pub const BUCK_R: f32 = 0.5;
/// The slug's shove: out to 3 strike radii, the walkers nearest the hole
/// thrown for 0.25 s (about 5 dish cells at the race world's 30 fine steps
/// a second and `fling_speed` 1.5).
pub const SLUG_SHOVE: f32 = 3.0;
pub const SLUG_SHOVE_SECS: f32 = 0.25;

/// One bullet card: its payload, set and rolled numbers, all whole so a
/// card stays `Copy + Eq + Hash` and its text form exact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ammo {
    pub shell: Shell,
    pub set: Set,
    /// The planet it was found on (from 1).
    pub depth: u8,
    /// Damage in tenths (`DAMAGE` is 100).
    pub damage: u16,
    /// Times the gun's, in hundredths (`MULT_MIN` to `MULT_MAX`).
    pub speed: u16,
    pub range: u16,
    pub spread: u16,
    /// Budget points (`Card::cost`).
    pub cost: u8,
}

impl Default for Ammo {
    /// The standard plain bullet: what a shot with no cards fires.
    fn default() -> Ammo {
        Ammo::standard(Shell::Plain)
    }
}

impl Ammo {
    /// A payload's standard bullet: damage `DAMAGE`, every multiplier 1,
    /// its fixed card's cost. What `Card::Blast` and the rest fire.
    pub fn standard(shell: Shell) -> Ammo {
        Ammo { shell, set: Set::Standard, depth: 1, damage: (DAMAGE * 10.0) as u16, speed: 100, range: 100, spread: 100, cost: shell.base() as u8 }
    }

    /// A bullet of any payload and set, rolled for the planet at `depth`
    /// (from 1) of the run with `seed`: what a run's shop or a drop hands
    /// out. The same `(seed, depth)` always gives the same bullet.
    pub fn roll(seed: u64, depth: u32) -> Ammo {
        let mut r = rng(seed, depth);
        let shell = Shell::ALL[r.below(Shell::ALL.len() as u32) as usize];
        let set = Set::ALL[r.below(Set::ALL.len() as u32) as usize];
        roll_with(shell, set, depth, &mut r)
    }

    /// A bullet of `shell` and `set` rolled for `(seed, depth)`.
    pub fn roll_of(shell: Shell, set: Set, seed: u64, depth: u32) -> Ammo {
        roll_with(shell, set, depth, &mut rng(seed, depth))
    }

    pub fn damage(&self) -> f32 {
        self.damage as f32 / 10.0
    }

    /// Its damage against a standard bullet's: what a strike's or a
    /// burst's radius is multiplied by (the area goes as the damage).
    pub fn reach(&self) -> f32 {
        (self.damage() / DAMAGE).max(0.0).sqrt()
    }

    pub fn speed(&self) -> f32 {
        self.speed as f32 / 100.0
    }

    pub fn range(&self) -> f32 {
        self.range as f32 / 100.0
    }

    pub fn spread(&self) -> f32 {
        self.spread as f32 / 100.0
    }

    /// The roll against a standard bullet: the geometric mean of damage,
    /// speed (half weight), range and tightness (half weight); 1 is a
    /// standard bullet. Its cost follows it.
    pub fn grade(&self) -> f32 {
        let l = [(self.damage() / DAMAGE).ln(), 0.5 * self.speed().ln(), self.range().ln(), -0.5 * self.spread().ln()];
        (l.iter().sum::<f32>() / 3.0).exp()
    }

    /// "Needle gas", "Standard blast", "Slug bullet".
    pub fn title(&self) -> String {
        match (self.set, self.shell) {
            (s, Shell::Plain) => format!("{} bullet", cap(s.key())),
            (s, p) => format!("{} {}", cap(s.key()), p.key()),
        }
    }

    /// One line of its numbers: "damage 12.4, speed 1.8x, range 1.3x,
    /// spread 0.6x, cost 4".
    pub fn line(&self) -> String {
        format!("damage {:.1}, speed {:.1}x, range {:.1}x, spread {:.1}x, cost {}", self.damage(), self.speed(), self.range(), self.spread(), self.cost)
    }

    /// The text form inside a round (`rounds::parse`, `Cmd::Load`):
    /// `ammo/SHELL/SET/DEPTH/DAMAGE/SPEED/RANGE/SPREAD/COST`, whole numbers.
    pub fn word(&self) -> String {
        format!("ammo/{}/{}/{}/{}/{}/{}/{}/{}", self.shell.key(), self.set.key(), self.depth, self.damage, self.speed, self.range, self.spread, self.cost)
    }

    pub fn from_word(s: &str) -> Option<Ammo> {
        let w: Vec<&str> = s.split('/').collect();
        if w.len() != 9 || w[0] != "ammo" {
            return None;
        }
        let n = |i: usize| w[i].parse::<u16>().ok();
        let m = |i: usize| n(i).map(|v| v.clamp(MULT_MIN, MULT_MAX));
        Some(Ammo {
            shell: Shell::ALL.into_iter().find(|p| p.key() == w[1])?,
            set: Set::parse(w[2])?,
            depth: w[3].parse().ok()?,
            damage: n(4)?,
            speed: m(5)?,
            range: m(6)?,
            spread: m(7)?,
            cost: w[8].parse().ok()?,
        })
    }
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

fn rng(seed: u64, depth: u32) -> Rng {
    // Its own stream, apart from the planet's and the guns'.
    Rng::new(seed ^ (depth as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x616d_6d6f)
}

fn roll_with(shell: Shell, set: Set, depth: u32, r: &mut Rng) -> Ammo {
    let lean = set.lean();
    let d = (1.0 + DEPTH_DAMAGE * (depth.max(1) - 1) as f32).min(DEPTH_DAMAGE_MAX);
    let mut z = || super::guns::normal(r);
    let damage = DAMAGE * lean.damage * d * (SIGMA_DAMAGE * z()).exp();
    let mult = |lean: f32, z: f32| ((lean * (SIGMA_MULT * z).exp() * 100.0).round() as u16).clamp(MULT_MIN, MULT_MAX);
    let (zs, zr, zp, zc) = (z(), z(), z(), z());
    let mut a = Ammo {
        shell,
        set,
        depth: depth.clamp(1, u8::MAX as u32) as u8,
        damage: (damage * 10.0).round().clamp(1.0, u16::MAX as f32) as u16,
        speed: mult(lean.speed, zs),
        range: mult(lean.range, zr),
        spread: mult(lean.spread, zp),
        cost: 0,
    };
    let cost = shell.base() as f32 * a.grade().powf(COST_POWER) * (COST_LUCK * zc).exp();
    a.cost = cost.round().clamp(1.0, u8::MAX as f32) as u8;
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn many(n: u64, depth: u32) -> Vec<Ammo> {
        (0..n).map(|s| Ammo::roll(s, depth)).collect()
    }

    #[test]
    fn a_standard_bullet_is_the_fixed_card() {
        for s in Shell::ALL {
            let a = Ammo::standard(s);
            assert_eq!((a.damage(), a.speed(), a.range(), a.spread(), a.reach()), (DAMAGE, 1.0, 1.0, 1.0, 1.0));
            assert_eq!(a.cost as u32, s.card().cost());
            assert!((a.grade() - 1.0).abs() < 1e-6);
            assert_eq!(Shell::of(s.card()), Some(s));
        }
    }

    #[test]
    fn rolls_are_the_same_for_a_seed_and_stay_in_bounds() {
        assert_eq!(Ammo::roll(5, 2), Ammo::roll(5, 2));
        assert_ne!(Ammo::roll(5, 2), Ammo::roll(6, 2));
        let all = many(4000, 1);
        for a in &all {
            for m in [a.speed, a.range, a.spread] {
                assert!((MULT_MIN..=MULT_MAX).contains(&m), "{a:?}");
            }
            assert!(a.damage > 0 && a.cost >= 1, "{a:?}");
            assert_eq!(Ammo::from_word(&a.word()), Some(*a), "the text form round-trips");
        }
        for s in Shell::ALL {
            assert!(all.iter().any(|a| a.shell == s));
        }
        // The owner's 0.25 to 4: both ends turn up (a needle's speed leans 2).
        assert!(all.iter().any(|a| a.speed >= 300) && all.iter().any(|a| a.spread <= 40));
    }

    #[test]
    fn each_set_leans_its_rolls_the_way_it_says() {
        let mean = |set: Set, f: &dyn Fn(&Ammo) -> f32| (0..2000).map(|s| f(&Ammo::roll_of(Shell::Plain, set, s, 1))).sum::<f32>() / 2000.0;
        let (st, sl, ne, bu) = (Set::Standard, Set::Slug, Set::Needle, Set::Buckshot);
        assert!(mean(sl, &|a| a.damage()) > 1.4 * mean(st, &|a| a.damage()), "a slug hits harder");
        assert!(mean(ne, &|a| a.speed()) > 1.7 * mean(st, &|a| a.speed()), "a needle flies faster");
        assert!(mean(ne, &|a| a.range()) > mean(st, &|a| a.range()) && mean(ne, &|a| a.damage()) < mean(st, &|a| a.damage()));
        assert!(mean(bu, &|a| a.spread()) > 2.0 * mean(st, &|a| a.spread()), "buckshot spreads");
        // Standard bullets roll round 1: as often better as worse.
        let g: Vec<f32> = (0..2000).map(|s| Ammo::roll_of(Shell::Blast, st, s, 1).grade()).collect();
        let better = g.iter().filter(|&&x| x > 1.0).count();
        assert!((800..1200).contains(&better), "{better} of 2000 better than standard");
    }

    #[test]
    fn a_better_roll_and_a_payload_cost_more() {
        let all = many(6000, 1);
        let mean_cost = |f: &dyn Fn(&Ammo) -> bool| {
            let v: Vec<f32> = all.iter().filter(|a| f(a)).map(|a| a.cost as f32).collect();
            v.iter().sum::<f32>() / v.len() as f32
        };
        assert!(mean_cost(&|a| a.shell == Shell::Germ) > mean_cost(&|a| a.shell == Shell::Plain) + 1.5, "a payload costs more");
        let blasts = |f: &dyn Fn(f32) -> bool| mean_cost(&|a| a.shell == Shell::Blast && f(a.grade()));
        assert!(blasts(&|g| g > 1.15) > blasts(&|g| g < 0.85), "a better roll costs more");
        // Luck: two blasts of one grade need not cost the same.
        let near: Vec<u8> = all.iter().filter(|a| a.shell == Shell::Blast && (a.grade() - 1.0).abs() < 0.03).map(|a| a.cost).collect();
        assert!(near.iter().any(|&c| c != near[0]), "{near:?}");
    }

    #[test]
    fn deeper_planets_roll_more_damage() {
        let mean = |d: u32| many(3000, d).iter().map(|a| a.damage() / a.set.lean().damage).sum::<f32>() / 3000.0;
        let (a, b) = (mean(1), mean(10));
        assert!(b > 1.25 * a, "{a} then {b}");
    }
}
