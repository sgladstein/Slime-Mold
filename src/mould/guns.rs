//! **Guns** (owner, 2026-11-14 16:46Z, answering the gun thread's three
//! questions): "1) The squad owns the gun. 2) Each gun defines the budget
//! and slot count and range, spread, rate of fire as actual numbers. There
//! should be a handful of base gun types (fast and short, slow and far, wide
//! slow (with default 2-6x multiplier), etc. but each stat is drawn from a
//! distribution, so those are the average per gun type, but you could get
//! better or worse. Sometimes you get lucky with a really good gun." And
//! 16:47Z: "As you progress in the game, gun are more likely to have
//! increased budget" / "probably better base stats overall".
//!
//! A `Gun` is those numbers; a squad holds one (`Squad::gun`, `Game::gun`)
//! and every member fires it. Its cards are the rounds' (`rounds`): the
//! budget and slots are what they may cost and hold.
//!
//! **Kinds** (`GunKind`), each an average gun at depth 1: the rifle is the
//! gun every soldier had before (24 cells, a shot a second, dead on,
//! budget 10, 4 slots), so a game with no rolled guns plays as it did. The
//! carbine is fast and short, the long gun slow and far, the scattergun
//! wide and slow: 2 to 6 bullets a shot in a fan (the owner's "2-6x";
//! confirmed 18:08Z, "2-6 bullets per shot").
//!
//! **Rolls** (`Gun::roll`): range, rate, spread and budget are each the
//! kind's average times `exp(sigma z)`, `z` a standard normal draw; slots
//! and a scattergun's bullets a shot are the kind's number plus a normal
//! draw, rounded. Luck is in each roll and comes from the normal curve
//! alone (owner, 17:32Z: "A lucky gun happens due to each individual random
//! roll"; 18:07Z: "this should just come out of the normal distribution
//! ... chance for every stat to be slightly better or slightly worse, and
//! smaller chance for significantly better or worse, and really small
//! change for way better or way worse. The luckiness of the gun depends on
//! all those rolls, but the whole thing is a distribution"). At a rate
//! sigma of 0.15, two stats in three land within 15% of average, one in 44
//! is 35% better or more and one in 740 is 57% better or more, the bad
//! side the same. `Gun::grade` sums the rolls (a shop's price). No gun is
//! labelled lucky (owner 18:18Z: "Nothing needs to be labeled lucky"; from
//! 17:00Z a `Gun::lucky` flag named the best twelfth "lucky carbine"). From
//! 17:40Z to 18:07Z each draw also had its own 1-in-6 lucky lift of 1.5
//! sigma (a fatter good tail on top of the curve), and before that one
//! roll a gun (1 in 12) made every draw good at once. Deeper planets
//! (`depth`, from 1, as `levelgen::level`) raise the averages: the budget
//! by `DEPTH_BUDGET` a planet up to `DEPTH_BUDGET_MAX` times, range and
//! rate by `DEPTH_STAT` up to `DEPTH_STAT_MAX`, spread down by as much, and
//! slots by `DEPTH_SLOTS` a planet up to `DEPTH_SLOTS_MAX`.
use super::larder_world::Rules;
use super::rounds::{fit_gun, Card, Over, RoundRules, SLOTS};
use super::Rng;

/// Most cards a gun can hold (`Gun::slots`); `rounds::SLOTS` is the
/// rifle's 4.
pub const SLOTS_MAX: u32 = 6;
/// Fewest a gun holds.
pub const SLOTS_MIN: u32 = 2;
/// Most bullets a shot.
pub const SHOTS_MAX: u32 = 6;
/// Budget a planet deeper, as a fraction of the kind's (and the cap).
pub const DEPTH_BUDGET: f32 = 0.12;
pub const DEPTH_BUDGET_MAX: f32 = 2.5;
/// Range and rate a planet deeper (and the cap); spread falls as much.
pub const DEPTH_STAT: f32 = 0.04;
pub const DEPTH_STAT_MAX: f32 = 1.4;
/// Spread of each roll (sigma of its log): range, rate, spread, budget.
const SIGMA: [f32; 4] = [0.12, 0.15, 0.3, 0.15];
/// Spread of the slots' and the scattergun's bullets' rolls, in whole
/// slots and bullets before rounding: a quarter of guns get a slot more
/// and a quarter one fewer, one in 60 two more.
const SLOT_SIGMA: f32 = 0.7;
const SHOT_SIGMA: f32 = 1.0;
/// Slots a planet deeper, before rounding (and the cap).
pub const DEPTH_SLOTS: f32 = 0.1;
pub const DEPTH_SLOTS_MAX: f32 = 0.6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GunKind {
    Rifle,
    Carbine,
    Long,
    Scatter,
}

impl GunKind {
    pub const ALL: [GunKind; 4] = [GunKind::Rifle, GunKind::Carbine, GunKind::Long, GunKind::Scatter];

    pub fn name(self) -> &'static str {
        match self {
            GunKind::Rifle => "rifle",
            GunKind::Carbine => "carbine",
            GunKind::Long => "long gun",
            GunKind::Scatter => "scattergun",
        }
    }

    /// One word, for `gun=` and replays.
    pub fn key(self) -> &'static str {
        match self {
            GunKind::Rifle => "rifle",
            GunKind::Carbine => "carbine",
            GunKind::Long => "long",
            GunKind::Scatter => "scatter",
        }
    }

    pub fn parse(s: &str) -> Option<GunKind> {
        GunKind::ALL.into_iter().find(|k| k.key() == s || k.name() == s)
    }

    pub fn about(self) -> &'static str {
        match self {
            GunKind::Rifle => "Steady: a shot a second, dead on, at middle range.",
            GunKind::Carbine => "Fast and short: twice the shots, a little wild, more slots for cheap cards.",
            GunKind::Long => "Slow and far: a shot every two seconds from far off, a big budget in few slots.",
            GunKind::Scatter => "Wide and slow: a fan of bullets each shot, close in.",
        }
    }

    /// The kind's average gun at depth 1, before any roll.
    pub fn average(self) -> Gun {
        let (range, rate, spread, shots, budget, slots) = match self {
            GunKind::Rifle => (24.0, 1.0, 0.0, 1, 10, 4),
            GunKind::Carbine => (15.0, 2.2, 1.5, 1, 8, 5),
            GunKind::Long => (40.0, 0.5, 0.0, 1, 12, 3),
            GunKind::Scatter => (13.0, 0.6, 5.0, 4, 9, 3),
        };
        Gun { kind: self, depth: 1, range, rate, spread, shots, budget, slots, grade: 1.0 }
    }

    /// The kind's average at `depth` (from 1): budget and stats grown as
    /// the module says.
    pub fn average_at(self, depth: u32) -> Gun {
        let d = depth.max(1) as f32 - 1.0;
        let (b, s) = ((1.0 + DEPTH_BUDGET * d).min(DEPTH_BUDGET_MAX), (1.0 + DEPTH_STAT * d).min(DEPTH_STAT_MAX));
        let a = self.average();
        Gun { depth: depth.max(1), range: a.range * s, rate: a.rate * s, spread: a.spread / s, budget: (a.budget as f32 * b).round() as u32, ..a }
    }
}

/// A squad's gun: the numbers every member fires by.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gun {
    pub kind: GunKind,
    /// The planet it was found on (from 1).
    pub depth: u32,
    /// How far it sees and fires, dish cells (before `Range` cards).
    pub range: f32,
    /// Shots a game second, a soldier (before `Rate` cards and the
    /// controlled squad's edge).
    pub rate: f32,
    /// Aim spread, dish cells at the target, as `Rules::spread`.
    pub spread: f32,
    /// Bullets a shot, each with its own spread (the scattergun's fan).
    pub shots: u32,
    /// What its cards may cost (`rounds::fit`).
    pub budget: u32,
    /// Cards it holds.
    pub slots: u32,
    /// The roll against its kind's average at its depth: the geometric
    /// mean of range, rate, budget, slots and (where it has any) spread,
    /// each as better-over-average; 1 is average.
    pub grade: f32,
}

impl Gun {
    /// The gun every soldier had before guns were rolled: the rifle at the
    /// game's own `Rules` (range, `shot_every`, spread), the rounds'
    /// default budget and `rounds::SLOTS`. `Game::gun` for a squad with
    /// none of its own.
    pub fn standard(r: &Rules) -> Gun {
        Gun {
            range: r.range,
            rate: 1.0 / r.shot_every.max(1e-3),
            spread: r.spread,
            budget: RoundRules::default().budget,
            slots: SLOTS as u32,
            ..GunKind::Rifle.average()
        }
    }

    /// A gun of a kind drawn evenly, rolled for the planet at `depth`
    /// (from 1) of the run with `seed`: what a run's shop or a drop hands
    /// out. The same `(seed, depth)` always gives the same gun; a shop
    /// offering several varies the seed.
    pub fn roll(seed: u64, depth: u32) -> Gun {
        let mut r = rng(seed, depth);
        let kind = GunKind::ALL[r.below(GunKind::ALL.len() as u32) as usize];
        roll_with(kind, depth, &mut r)
    }

    /// A gun of `kind` rolled for `(seed, depth)`.
    pub fn roll_kind(kind: GunKind, seed: u64, depth: u32) -> Gun {
        roll_with(kind, depth, &mut rng(seed, depth))
    }

    /// Seconds between one soldier's shots, before cards.
    pub fn every(&self) -> f32 {
        1.0 / self.rate.max(1e-3)
    }

    /// "long gun", "carbine": its kind, never "lucky" (owner 18:18Z).
    pub fn title(&self) -> String {
        self.kind.name().to_string()
    }

    /// One line of its numbers.
    pub fn line(&self) -> String {
        let shots = if self.shots > 1 { format!(", {} bullets a shot", self.shots) } else { String::new() };
        format!(
            "{:.0} cells, {:.1} shots/s, spread {:.1}{shots}, budget {}, {} slots",
            self.range, self.rate, self.spread, self.budget, self.slots
        )
    }

    /// The text form (`Cmd::Arm`): kind, depth, range, rate, spread,
    /// shots, budget, slots, grade. `from_words` also reads the lines of
    /// #283, which had a lucky flag (0 or 1) before the grade.
    pub fn to_words(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {}",
            self.kind.key(),
            self.depth,
            self.range,
            self.rate,
            self.spread,
            self.shots,
            self.budget,
            self.slots,
            self.grade
        )
    }

    pub fn from_words(w: &[&str]) -> Option<Gun> {
        let f = |i: usize| w.get(i)?.parse::<f32>().ok();
        let u = |i: usize| w.get(i)?.parse::<u32>().ok();
        Some(Gun {
            kind: GunKind::parse(w.first()?)?,
            depth: u(1)?,
            range: f(2)?,
            rate: f(3)?,
            spread: f(4)?,
            shots: u(5)?.clamp(1, SHOTS_MAX),
            budget: u(6)?,
            slots: u(7)?.clamp(SLOTS_MIN, SLOTS_MAX),
            grade: if w.len() > 9 { f(9)? } else { f(8)? },
        })
    }
}

/// What a squad carries between planets (the roguelike's shop): its gun
/// and the cards in it. A run holds one a squad.
#[derive(Clone, Debug, PartialEq)]
pub struct Loadout {
    pub gun: Gun,
    pub cards: Vec<Card>,
}

impl Loadout {
    /// The gun with no cards.
    pub fn bare(gun: Gun) -> Loadout {
        Loadout { gun, cards: Vec::new() }
    }

    /// The cards fit the gun (`rounds::fit_gun`).
    pub fn fits(&self) -> Result<(), Over> {
        fit_gun(self.cards.clone(), &self.gun).map(|_| ())
    }
}

fn rng(seed: u64, depth: u32) -> Rng {
    // Its own stream, apart from the planet's (`levelgen`), so a gun and
    // the map it was found on never share draws.
    Rng::new(seed ^ (depth as u64).wrapping_mul(0xA24B_AED4_963E_E407) ^ 0x6775_6e73)
}

/// A standard normal draw (Box-Muller); the bullets roll by it too.
pub(super) fn normal(r: &mut Rng) -> f32 {
    let u = 1.0 - r.f32();
    let v = r.f32();
    (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
}

fn roll_with(kind: GunKind, depth: u32, r: &mut Rng) -> Gun {
    let a = kind.average_at(depth);
    // Every stat its own normal draw (18:07Z), nothing else: +z is the
    // good side for range, rate and budget, -z for spread.
    let mut z = || normal(r);
    let (zr, zt, zs, zb, zn, zk) = (z(), z(), z(), z(), z(), z());
    let range = a.range * (SIGMA[0] * zr).exp();
    let rate = a.rate * (SIGMA[1] * zt).exp();
    let spread = a.spread * (-SIGMA[2] * zs).exp();
    let budget = ((a.budget as f32 * (SIGMA[3] * zb).exp()).round() as u32).max(2);
    let deeper = (DEPTH_SLOTS * (depth.max(1) - 1) as f32).min(DEPTH_SLOTS_MAX);
    let slots = (a.slots as f32 + deeper + SLOT_SIGMA * zn).round().clamp(SLOTS_MIN as f32, SLOTS_MAX as f32) as u32;
    let shots = if kind == GunKind::Scatter { (a.shots as f32 + SHOT_SIGMA * zk).round().clamp(2.0, SHOTS_MAX as f32) as u32 } else { 1 };
    let mut parts = vec![range / a.range, rate / a.rate, budget as f32 / a.budget as f32, slots as f32 / a.slots as f32];
    if a.spread > 0.0 {
        parts.push(a.spread / spread.max(1e-3));
    }
    if kind == GunKind::Scatter {
        parts.push(shots as f32 / a.shots as f32);
    }
    let grade = (parts.iter().map(|p| p.ln()).sum::<f32>() / parts.len() as f32).exp();
    Gun { kind, depth: a.depth, range, rate, spread, shots, budget, slots, grade }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn many(kind: Option<GunKind>, depth: u32, n: u64) -> Vec<Gun> {
        (0..n).map(|s| kind.map_or_else(|| Gun::roll(s, depth), |k| Gun::roll_kind(k, s, depth))).collect()
    }

    #[test]
    fn the_standard_gun_is_the_rifle_every_soldier_had() {
        let r = Rules::default();
        let g = Gun::standard(&r);
        assert_eq!(g.kind, GunKind::Rifle);
        assert_eq!((g.range, g.rate, g.spread, g.shots, g.budget, g.slots), (r.range, 1.0 / r.shot_every, r.spread, 1, 10, SLOTS as u32));
        let a = GunKind::Rifle.average();
        assert_eq!((a.range, a.rate, a.spread, a.budget, a.slots), (g.range, g.rate, g.spread, g.budget, g.slots), "the rifle's average is that gun");
    }

    #[test]
    fn a_roll_is_the_same_for_the_same_seed_and_depth() {
        assert_eq!(Gun::roll(7, 3), Gun::roll(7, 3));
        assert_ne!(Gun::roll(7, 3), Gun::roll(8, 3));
        assert_ne!(Gun::roll_kind(GunKind::Long, 7, 3), Gun::roll_kind(GunKind::Long, 7, 4));
    }

    #[test]
    fn rolls_scatter_round_each_kinds_average_and_stay_in_bounds() {
        for kind in GunKind::ALL {
            let a = kind.average();
            let gs = many(Some(kind), 1, 4000);
            let mean = |f: &dyn Fn(&Gun) -> f32| gs.iter().map(f).sum::<f32>() / gs.len() as f32;
            // exp(sigma z) has mean exp(sigma^2 / 2), a percent or two over.
            assert!((mean(&|g| g.range) / a.range - 1.0).abs() < 0.04, "{kind:?} range {}", mean(&|g| g.range));
            assert!((mean(&|g| g.rate) / a.rate - 1.0).abs() < 0.05, "{kind:?} rate {}", mean(&|g| g.rate));
            assert!((mean(&|g| g.budget as f32) / a.budget as f32 - 1.0).abs() < 0.06, "{kind:?} budget {}", mean(&|g| g.budget as f32));
            for g in &gs {
                assert!(g.range > 0.0 && g.rate > 0.0 && g.spread >= 0.0, "{g:?}");
                assert!((SLOTS_MIN..=SLOTS_MAX).contains(&g.slots) && (1..=SHOTS_MAX).contains(&g.shots) && g.budget >= 2, "{g:?}");
                assert_eq!(g.shots > 1, kind == GunKind::Scatter && g.shots > 1);
                assert_eq!(Gun::from_words(&g.to_words().split(' ').collect::<Vec<_>>()), Some(*g), "the text form round-trips");
            }
            // Some better, some worse, both ways by a fifth now and then.
            assert!(gs.iter().any(|g| g.range > a.range * 1.2) && gs.iter().any(|g| g.range < a.range * 0.8), "{kind:?} range never strays");
            if kind == GunKind::Scatter {
                // Owner 18:08Z: "2-6 bullets per shot". Rounded from a
                // normal draw round 4: 2 and 6 one gun in 14 each.
                assert!((2..=6).all(|n| gs.iter().any(|g| g.shots == n)), "the scattergun's fan is 2 to 6");
                let mid = gs.iter().filter(|g| (3..=5).contains(&g.shots)).count();
                assert!(mid * 10 > gs.len() * 8, "most fans are 3 to 5: {mid}");
            }
        }
    }

    /// Luck is the normal curve in each roll (owner 18:07Z), and no gun
    /// is labelled lucky (18:18Z).
    #[test]
    fn luck_is_the_normal_curve_in_each_roll_and_the_grade_sums_them() {
        let gs = many(None, 1, 6000);
        let mut grades: Vec<f32> = gs.iter().map(|g| g.grade).collect();
        grades.sort_by(f32::total_cmp);
        let at = |f: f32| grades[(grades.len() as f32 * f) as usize];
        // Over 6,000 rolls at depth 1 (18:07Z, the normal curve alone):
        // median 0.995, 90% 1.105, 92% 1.115, 99% 1.205, best 1.33. With
        // the 1-in-6 lifts (17:40Z) they were 0.99, 1.14, 1.15 and 1.28.
        assert!((at(0.5) - 1.0).abs() < 0.02, "the kind's numbers stay the average: median {}", at(0.5));
        assert!(at(0.9) > 1.07 && at(0.9) < 1.14 && at(0.1) < 0.93, "most guns near average: 10% {} 90% {}", at(0.1), at(0.9));
        assert!(at(0.99) > 1.15, "a great gun now and then: 99% {}", at(0.99));
        // Luck is a stat at a time: some guns have one great number and
        // others average, so a great range turns up on an ordinary gun.
        let long = GunKind::Long.average();
        assert!(gs.iter().any(|g| g.grade < 1.05 && g.kind == GunKind::Long && g.range > long.range * 1.25), "a great range on an ordinary gun");
        assert!(gs.iter().all(|g| !g.title().contains("lucky")));
        // The lines #283 wrote, with a lucky flag, still read.
        let g = gs[0];
        let old = format!("{} {} {} {} {} {} {} {} 1 {}", g.kind.key(), g.depth, g.range, g.rate, g.spread, g.shots, g.budget, g.slots, g.grade);
        assert_eq!(Gun::from_words(&old.split(' ').collect::<Vec<_>>()), Some(g));
    }

    #[test]
    fn guns_from_deeper_planets_have_bigger_budgets_and_better_stats() {
        let mean = |depth: u32, f: &dyn Fn(&Gun) -> f32| many(None, depth, 3000).iter().map(f).sum::<f32>() / 3000.0;
        let (b1, b5, b10) = (mean(1, &|g| g.budget as f32), mean(5, &|g| g.budget as f32), mean(10, &|g| g.budget as f32));
        assert!(b1 < b5 && b5 < b10, "budget by depth: {b1} {b5} {b10}");
        assert!(b10 > 1.8 * b1, "{b10} against {b1}");
        let (s1, s10) = (mean(1, &|g| g.slots as f32), mean(10, &|g| g.slots as f32));
        assert!(s10 > s1 + 0.2, "slots {s1} then {s10}");
        for kind in GunKind::ALL {
            let (a, d) = (kind.average_at(1), kind.average_at(10));
            assert!(d.range > a.range && d.rate > a.rate && d.spread <= a.spread && d.budget > a.budget, "{kind:?}");
        }
        assert_eq!(GunKind::Long.average_at(40).range, GunKind::Long.average().range * DEPTH_STAT_MAX, "capped");
    }

    #[test]
    fn a_loadout_fits_its_own_guns_budget_and_slots() {
        let mut l = Loadout::bare(GunKind::Carbine.average());
        assert_eq!((l.gun.budget, l.gun.slots), (8, 5));
        l.cards = vec![Card::Rate; 4];
        assert!(l.fits().is_ok(), "4 rate cards cost 8 in 5 slots");
        l.cards.push(Card::Quiet);
        assert!(l.fits().is_err(), "a ninth point is over");
        l.cards = vec![Card::Thrift, Card::Rate, Card::Thrift, Card::Rate, Card::Quiet, Card::Quiet];
        assert!(l.fits().is_err(), "six cards in five slots");
        l.gun.slots = 6;
        assert!(l.fits().is_ok(), "a sixth slot holds it: cost {}", super::super::rounds::cost(&l.cards));
        let rifle = Loadout { gun: Gun::standard(&Rules::default()), cards: vec![Card::Bomb, Card::Germ, Card::Quiet] };
        assert!(rifle.fits().is_ok());
    }

    #[test]
    fn each_kind_is_what_the_owner_named() {
        let (r, c, l, s) = (GunKind::Rifle.average(), GunKind::Carbine.average(), GunKind::Long.average(), GunKind::Scatter.average());
        assert!(c.rate > r.rate && c.range < r.range, "fast and short");
        assert!(l.rate < r.rate && l.range > r.range, "slow and far");
        assert!(s.rate < r.rate && s.shots >= 2 && s.spread > c.spread, "wide and slow");
        for k in GunKind::ALL {
            assert_eq!(GunKind::parse(k.key()), Some(k));
        }
    }
}
