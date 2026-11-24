//! **Device bodies** (gun step E3, 2026-11-24; the owner's 11-16 gun
//! ruling, "a body holding one to three cards in the gun's language",
//! and his 11-19 loadout ruling, one loadout per gun or body with a
//! default; Direction's call on the shop, 11-24 07:01Z).
//!
//! A body is what the squad mode's device slot sets off (`Game::device`):
//!
//! | Body | Goes off | The tool it is |
//! |---|---|---|
//! | Grenade | thrown, bursts where it lands | blast |
//! | Mine | placed, bursts when slime crowds it | mine |
//! | Line | laid along a drag, bursts along it | det cord |
//! | Canister | placed, sleep gas, then its cards every second for `KitRules::canister_secs` | sleep gas |
//!
//! Each is rolled as a gun is: how far a soldier throws it (`reach`, 16
//! to 32 dish cells), its cooldown against the tool's own (`cool`, 0.7 to
//! 1.4), and its card rack (`slots` 1 to 3, `budget` 2 to 6), which the
//! cards of its loadout fit (`Run::body_cards`). The four bodies replace
//! the Tools group's unlocks (Blasts, Mines, Det cord, Sleep gas): a run
//! owns at most one body of each kind, and a bought one takes the place of
//! the one it had. A saved run that owned an unlock gets the plain body
//! (`Body::plain`: today's reach and cooldown, 2 slots and a budget of 4,
//! the unlock's price).
//!
//! Charge (placed, all of a squad's go off on an order) comes later.
//! Prices and rolls are first guesses (Direction retunes them after play).

use super::guns::Gun;

/// What a body is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BodyKind {
    Grenade,
    Mine,
    Line,
    Canister,
}

impl BodyKind {
    pub const ALL: [BodyKind; 4] = [BodyKind::Grenade, BodyKind::Mine, BodyKind::Line, BodyKind::Canister];

    /// The run's unlock key and the toolkit's tool name (`kit::TOOLS`).
    pub fn key(self) -> &'static str {
        match self {
            BodyKind::Grenade => "blast",
            BodyKind::Mine => "mine",
            BodyKind::Line => "cord",
            BodyKind::Canister => "gas",
        }
    }

    /// Its tool's index (`kit::Tool::index`, `kit::TOOLS`).
    pub fn tool(self) -> usize {
        match self {
            BodyKind::Grenade => 0,
            BodyKind::Mine => 1,
            BodyKind::Line => 2,
            BodyKind::Canister => 4,
        }
    }

    /// The body that is tool `t` (`kit::Tool::index`), if any.
    pub fn of_tool(t: usize) -> Option<BodyKind> {
        BodyKind::ALL.into_iter().find(|k| k.tool() == t)
    }

    pub fn of_key(key: &str) -> Option<BodyKind> {
        BodyKind::ALL.into_iter().find(|k| k.key() == key)
    }

    pub fn name(self) -> &'static str {
        match self {
            BodyKind::Grenade => "Grenade",
            BodyKind::Mine => "Mine",
            BodyKind::Line => "Line",
            BodyKind::Canister => "Canister",
        }
    }

    /// How it goes off, for the shop's line.
    pub fn about(self) -> &'static str {
        match self {
            BodyKind::Grenade => "thrown; a blast and its cards where it lands",
            BodyKind::Mine => "placed; a blast and its cards when slime crowds it",
            BodyKind::Line => "laid along a drag; blasts along it, its cards at the ends and every 8 cells",
            BodyKind::Canister => "placed; sleep gas, then its cards every second for 4 s",
        }
    }

    /// The plain body's price: the unlock's it replaces.
    pub fn plain_price(self) -> u32 {
        match self {
            BodyKind::Grenade | BodyKind::Mine => 150,
            BodyKind::Line | BodyKind::Canister => 200,
        }
    }

    fn word(self) -> &'static str {
        match self {
            BodyKind::Grenade => "grenade",
            BodyKind::Mine => "mine",
            BodyKind::Line => "line",
            BodyKind::Canister => "canister",
        }
    }
}

/// A device body: its kind and its rolls.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body {
    pub kind: BodyKind,
    /// Dish cells from its nearest soldier it can be set off at
    /// (`KitRules::reach`, 24, before bodies).
    pub reach: f32,
    /// Its cooldown as a share of its tool's (`KitRules::cooldown`).
    pub cool: f32,
    /// Its card rack.
    pub slots: u32,
    pub budget: u32,
    /// The planet it was found for (1: the plain one).
    pub depth: u32,
    /// Whether it rolled (false: `plain`, priced as the old unlock).
    pub rolled: bool,
    /// Whether it goes off on its own (gun step E4, `Game::autocast`);
    /// off as bought.
    pub auto: bool,
}

/// The rolls' ranges.
pub const REACH: (f32, f32) = (16.0, 32.0);
pub const COOL: (f32, f32) = (0.7, 1.4);
pub const SLOTS: (u32, u32) = (1, 3);
pub const BUDGET: (u32, u32) = (2, 6);

impl Body {
    /// Today's tool as a body: reach 24, its own cooldown, 2 slots and a
    /// budget of 4 (E2's device rack).
    pub fn plain(kind: BodyKind) -> Body {
        Body { kind, reach: 24.0, cool: 1.0, slots: 2, budget: 4, depth: 1, rolled: false, auto: false }
    }

    /// A body of `kind` rolled from `z` for planet `depth`: each stat drawn
    /// evenly in its range (`REACH`, `COOL`, `SLOTS`, `BUDGET`).
    pub fn roll(kind: BodyKind, z: u64, depth: u32) -> Body {
        let mut z = z;
        let mut u = || {
            z = split(z);
            (z >> 11) as f32 / (1u64 << 53) as f32
        };
        let reach = (REACH.0 + (REACH.1 - REACH.0) * u()).round();
        let cool = ((COOL.0 + (COOL.1 - COOL.0) * u()) * 20.0).round() / 20.0;
        let slots = SLOTS.0 + ((SLOTS.1 - SLOTS.0 + 1) as f32 * u()).min((SLOTS.1 - SLOTS.0) as f32) as u32;
        let budget = BUDGET.0 + ((BUDGET.1 - BUDGET.0 + 1) as f32 * u()).min((BUDGET.1 - BUDGET.0) as f32) as u32;
        Body { kind, reach, cool, slots, budget, depth, rolled: true, auto: false }
    }

    /// How good its rolls are, 0 (the worst of each) to 1 (the best).
    pub fn grade(&self) -> f32 {
        let r = (self.reach - REACH.0) / (REACH.1 - REACH.0);
        let c = (COOL.1 - self.cool) / (COOL.1 - COOL.0);
        let s = (self.slots - SLOTS.0) as f32 / (SLOTS.1 - SLOTS.0) as f32;
        let b = (self.budget - BUDGET.0) as f32 / (BUDGET.1 - BUDGET.0) as f32;
        ((r + c + s + b) / 4.0).clamp(0.0, 1.0)
    }

    /// Its price: the plain one's is the unlock's, a rolled one's 120 to
    /// 260 by its grade, in tens.
    pub fn price(&self) -> u32 {
        if !self.rolled {
            return self.kind.plain_price();
        }
        ((120.0 + 140.0 * self.grade()) / 10.0).round() as u32 * 10
    }

    /// What its loadout's cards fit against (`rounds::fit_gun`).
    pub fn rack(&self) -> Gun {
        Gun { slots: self.slots, budget: self.budget, ..super::run::first_gun() }
    }

    /// "Grenade: 24 reach, 100% cooldown, 2 slots, budget 4".
    pub fn title(&self) -> String {
        format!("{}: {:.0} reach, {:.0}% cooldown, {} slot{}, budget {}", self.kind.name(), self.reach, self.cool * 100.0, self.slots, if self.slots == 1 { "" } else { "s" }, self.budget)
    }

    /// The run save's word: `grenade/24/1.00/2/4/1/0/0` (the last,
    /// autocast, since E4; a word of 7 reads as off).
    pub fn word(&self) -> String {
        format!("{}/{}/{:.2}/{}/{}/{}/{}/{}", self.kind.word(), self.reach, self.cool, self.slots, self.budget, self.depth, u8::from(self.rolled), u8::from(self.auto))
    }

    pub fn from_word(s: &str) -> Option<Body> {
        let w: Vec<&str> = s.split('/').collect();
        if !(7..=8).contains(&w.len()) {
            return None;
        }
        let kind = BodyKind::ALL.into_iter().find(|k| k.word() == w[0])?;
        let f = |i: usize| w[i].parse::<f32>().ok().filter(|v| v.is_finite());
        Some(Body { kind, reach: f(1)?, cool: f(2)?, slots: w[3].parse().ok()?, budget: w[4].parse().ok()?, depth: w[5].parse().ok()?, rolled: w[6] == "1", auto: w.get(7) == Some(&"1") })
    }
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

    /// Rolls stay in their ranges, a better roll costs more, the plain body
    /// costs the unlock's price, and a body survives the save's word.
    #[test]
    fn bodies_roll_in_range_and_price_by_grade() {
        let mut seen_slots = [false; 4];
        for z in 0..400u64 {
            let b = Body::roll(BodyKind::ALL[(z % 4) as usize], z * 7919, 2);
            assert!((REACH.0..=REACH.1).contains(&b.reach) && (COOL.0..=COOL.1).contains(&b.cool), "{b:?}");
            assert!((SLOTS.0..=SLOTS.1).contains(&b.slots) && (BUDGET.0..=BUDGET.1).contains(&b.budget), "{b:?}");
            assert!((120..=260).contains(&b.price()), "{b:?} {}", b.price());
            seen_slots[b.slots as usize] = true;
            assert_eq!(Body::from_word(&b.word()), Some(b));
        }
        assert!(seen_slots[1] && seen_slots[2] && seen_slots[3], "every slot count rolls");
        let best = Body { reach: REACH.1, cool: COOL.0, slots: SLOTS.1, budget: BUDGET.1, ..Body::roll(BodyKind::Grenade, 1, 2) };
        let worst = Body { reach: REACH.0, cool: COOL.1, slots: SLOTS.0, budget: BUDGET.0, ..best };
        assert_eq!((best.price(), worst.price()), (260, 120));
        assert_eq!(BodyKind::ALL.map(|k| Body::plain(k).price()), [150, 150, 200, 200]);
        assert_eq!(Body::from_word(&Body::plain(BodyKind::Line).word()), Some(Body::plain(BodyKind::Line)));
        let auto = Body { auto: true, ..Body::plain(BodyKind::Mine) };
        assert_eq!(Body::from_word(&auto.word()), Some(auto));
        assert_eq!(Body::from_word("mine/24/1.00/2/4/1/0"), Some(Body::plain(BodyKind::Mine)), "a word from before autocast");
    }
}
