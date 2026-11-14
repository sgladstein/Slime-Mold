//! **The roguelike run** (owner's notes, 2026-11-14 15:06Z; answers
//! 15:21Z): a separate mode beside the campaign, which it does not touch.
//! A run is planet after planet on the way to the slime's home world (the
//! owner's story, 15:25Z). Each level is one map with one slime to beat;
//! the slime gets harder (the tier ladder, more walkers) the deeper the
//! run goes; between levels a shop spends the points a win earned on the
//! squad (armour, shots, range, a bigger landing), unlocks and upgrades
//! buildings and tools, and buys gun cards that get better with depth. A
//! lost level ends the run.
//!
//! **What is this module's and what is not.** The level's geometry (ground,
//! crystals, mould starts, landing, walls, the map's size) is the level
//! generator's (`levelgen::level(seed, depth) -> Map`, its own thread,
//! agreed 2026-11-14 15:25Z, #277): a level is planet `depth` of the run's
//! seed (`Ground::Planet`), its mission's map `planet:DEPTH:SEED` as the
//! window's `map=` reads it. Everything else about a level is here:
//! the tier, the walker count, the goal and the rules the upgrades set.
//! Like `campaign`, a level is a `Mission` over the shared game, so the
//! window plays it with the mission tracker and its end screen; nothing in
//! `larder_world` changes for a run.
//!
//! **Gun cards** are today's `rounds` cards by name, joined into a loadout
//! (`Run::gun`). The gun thread is reworking guns into a card budget; when
//! that lands the shop sells its cards instead (asked 15:24Z).
//!
//! Rules-free and window-free, like the HUD: the window turns `Run::shop`
//! rows into the start menu's shop page and a pick back into `Run::pick`.

use super::campaign::{Bonus, Goal, Mission};
use super::kit::KitRules;
use super::larder_world::Rules;
use super::levelgen::{self, Planet};
use super::map::Map;
use super::rounds::{self, Card, Kind};
use super::tiers::Tier;

/// Where a level's ground comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum Ground {
    /// A hand-made map by name (`Reports/larder-maps-2026-11-11/maps`).
    /// The first build cycled the five (Flats first) until the generator
    /// landed (#277, 2026-11-14); kept for designed planets.
    Named(String),
    /// A generated planet: `levelgen::level(seed, depth)`, built when the
    /// window starts the level (about 55 ms), not when the shop shows it.
    Planet { seed: u64, depth: u32 },
    /// A map already made.
    Made(Map),
}

impl Ground {
    /// The map, built or loaded (`dir`: the hand-made maps' folder).
    pub fn map(&self, dir: &std::path::Path) -> Result<Map, String> {
        match self {
            Ground::Named(n) => Map::load(dir, n),
            Ground::Planet { seed, depth } => Ok(levelgen::level(*seed, *depth)),
            Ground::Made(m) => Ok(m.clone()),
        }
    }
}

/// The slime by depth: the campaign's ladder, gentlest first. Past its
/// end the hardest three take turns. The Grudge joins once the GPU engine
/// takes its scar keys (`Tier::settings` refuses it today; the test below
/// keeps every rung playable).
pub const LADDER: [Tier; 6] = [Tier::Drifter, Tier::Sprawler, Tier::Trunk, Tier::Swarmer, Tier::Sporer, Tier::Sleeper];

/// One level: what the window needs to start it.
#[derive(Clone, Debug, PartialEq)]
pub struct Level {
    pub depth: u32,
    /// The world's seed (the run's, moved by depth).
    pub seed: u64,
    pub ground: Ground,
    pub tier: Tier,
    /// The walkers the slime starts with, as a factor on the window's own
    /// count (1 on the first planet).
    pub walkers: f32,
    /// The goal, the bonus and the rules, as a campaign mission.
    pub mission: Mission,
}

/// Planets in a run; the last is the slime's home world (the core world,
/// the level generator's biggest map). Told to the level-gen thread
/// 2026-11-14: the owner's guess is 20 minutes to over an hour a run.
pub const PLANETS: u32 = 8;

/// Walkers grow this much a planet (a fifth more each).
pub const WALKERS_PER_DEPTH: f32 = 0.2;

/// The goal: the slime down to this share of its peak. Gentler early
/// (a quarter), down to a tenth from planet 6.
pub fn shrink_goal(depth: u32) -> f32 {
    (0.25 - 0.03 * (depth.saturating_sub(1)) as f32).max(0.10)
}

/// Points a won level pays: more the deeper it was and the more stars.
pub fn reward(depth: u32, stars: u8) -> u32 {
    150 + 50 * depth + 50 * u32::from(stars.saturating_sub(1))
}

/// What a shop row is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Squad,
    Build,
    Tools,
    Gun,
    Dev,
}

impl Group {
    pub fn name(self) -> &'static str {
        match self {
            Group::Squad => "Squad",
            Group::Build => "Buildings",
            Group::Tools => "Tools",
            Group::Gun => "Gun",
            Group::Dev => "Dev mode",
        }
    }
}

/// Something the shop sells by levels (an unlock is one level).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item {
    pub key: &'static str,
    pub name: &'static str,
    pub group: Group,
    /// What one level does.
    pub about: &'static str,
    pub max: u8,
    /// The first level's price; each next level costs this much more.
    pub cost: u32,
    /// Another item that must be owned first.
    pub needs: Option<&'static str>,
}

/// Everything the shop sells but cards. Unlock keys (`max` 1) are what
/// `Run::has` answers for the window's command card.
pub const ITEMS: [Item; 21] = [
    Item { key: "hp", name: "Armour", group: Group::Squad, about: "Soldiers take a quarter more biting before they fall", max: 4, cost: 120, needs: None },
    Item { key: "strike", name: "Heavier shots", group: Group::Squad, about: "Each shot kills a fifth more slime", max: 4, cost: 120, needs: None },
    Item { key: "rate", name: "Faster trigger", group: Group::Squad, about: "Soldiers fire 15% sooner", max: 4, cost: 120, needs: None },
    Item { key: "range", name: "Longer barrels", group: Group::Squad, about: "3 more cells of sight and reach", max: 3, cost: 100, needs: None },
    Item { key: "squad", name: "Bigger landing", group: Group::Squad, about: "Two more soldiers land with you", max: 4, cost: 150, needs: None },
    Item { key: "bank", name: "Supply drop", group: Group::Squad, about: "Land with 500 more food in the bank", max: 4, cost: 100, needs: None },
    Item { key: "harvest", name: "Quick hands", group: Group::Squad, about: "Soldiers take a quarter more from a crystal each second", max: 3, cost: 100, needs: None },
    Item { key: "walls", name: "Walls", group: Group::Build, about: "Unlocks laying walls", max: 1, cost: 100, needs: None },
    Item { key: "turret", name: "Turrets", group: Group::Build, about: "Unlocks the turret, a gun that does not move", max: 1, cost: 200, needs: None },
    Item { key: "turret+", name: "Turret power", group: Group::Build, about: "Turrets: 30% tougher, 3 cells more reach, fire 15% sooner", max: 3, cost: 150, needs: Some("turret") },
    Item { key: "lure", name: "Lures", group: Group::Build, about: "Unlocks the lure, food scent the slime follows", max: 1, cost: 150, needs: None },
    Item { key: "pump", name: "Pumps", group: Group::Build, about: "Unlocks the pump, which draws a crystal into the bank", max: 1, cost: 200, needs: None },
    Item { key: "outpost", name: "Outposts", group: Group::Build, about: "Unlocks the outpost, a second drop-off", max: 1, cost: 150, needs: None },
    Item { key: "siren", name: "Sirens", group: Group::Build, about: "Unlocks the siren, which sets off the slime's kinds", max: 1, cost: 150, needs: None },
    Item { key: "blast", name: "Blasts", group: Group::Tools, about: "Unlocks the thrown blast and its crater", max: 1, cost: 150, needs: None },
    Item { key: "mine", name: "Mines", group: Group::Tools, about: "Unlocks the mine, a blast the slime sets off", max: 1, cost: 150, needs: None },
    Item { key: "cord", name: "Det cord", group: Group::Tools, about: "Unlocks det cord, a line of blasts", max: 1, cost: 200, needs: None },
    Item { key: "gas", name: "Sleep gas", group: Group::Tools, about: "Unlocks sleep gas", max: 1, cost: 200, needs: None },
    Item { key: "tools+", name: "Cheaper tools", group: Group::Tools, about: "Every tool costs a fifth less", max: 3, cost: 120, needs: None },
    Item { key: "budget", name: "Bigger gun", group: Group::Gun, about: "2 more card budget in the gun (still at most 4 cards)", max: 3, cost: 150, needs: None },
    Item { key: "draw", name: "More cards on offer", group: Group::Gun, about: "One more gun card on offer in every shop", max: 2, cost: 100, needs: None },
];

/// What the run owns at the start: walls (the plainest defence).
pub const START: [&str; 1] = ["walls"];
/// The gun's card budget at the start (`rounds::fit`; the gun thread's
/// cards cost 1-4, so 5 holds a payload and a tweak). Free play's gun
/// has `RoundRules::budget` (10); `budget` upgrades reach 11.
pub const START_BUDGET: u32 = 5;
pub const BUDGET_STEP: u32 = 2;
/// Cards on offer in a shop before `draw`.
pub const OFFER: usize = 3;

/// A gun card's power, 1-3: the gun thread's `Card::tier` (common,
/// uncommon, rare), which depth first offers it and its price. None:
/// never sold (the germ: the GPU world casts none yet).
pub fn card_power(c: Card) -> Option<u8> {
    (c != Card::Germ).then(|| c.tier())
}

/// The first depth whose shop offers a card of this power.
pub fn power_from(power: u8) -> u32 {
    match power {
        1 => 1,
        2 => 2,
        _ => 4,
    }
}

pub fn card_cost(power: u8) -> u32 {
    [0, 80, 150, 250][power.min(3) as usize]
}

/// A row of the shop, for the menu to draw.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub group: Group,
    pub name: String,
    pub about: String,
    /// "150", or "" when it can't be bought again.
    pub price: String,
    /// "2/4", "owned", "in the gun", "in the bag".
    pub state: String,
    /// A pick does something now.
    pub ready: bool,
    pub pick: Pick,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pick {
    /// `ITEMS[i]`'s next level.
    Item(usize),
    /// Card on offer `i`.
    Offer(usize),
    /// Owned card `i`: into the gun or out of it, free.
    Card(usize),
    /// Dev mode (owner, 11-14 15:33Z: "a dev mode to control what level
    /// planet i go to", unlimited money, auto reroll): the next planet one
    /// deeper or shallower.
    DevDepth(i32),
    /// Dev mode: 1,000 more points.
    DevPoints,
    /// Dev mode: a fresh draw of cards.
    DevReroll,
}

/// One run.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub seed: u64,
    /// The level being played or next to play, from 1.
    pub depth: u32,
    pub points: u32,
    /// Levels owned of each of `ITEMS`.
    pub owned: [u8; ITEMS.len()],
    /// Cards owned, and whether each is in the gun.
    pub cards: Vec<(Card, bool)>,
    /// This shop's cards on offer (a bought one is taken off).
    pub offers: Vec<Card>,
    /// Stars won on each level so far.
    pub stars: Vec<u8>,
    /// The run is over (a level was lost) at this depth.
    pub over: bool,
    /// Dev mode: the shop has rows to jump planets, add points and redraw.
    pub dev: bool,
    /// Redraws so far (dev mode), so each draws anew.
    rerolls: u64,
    /// The run ended on a lost level.
    beaten: bool,
}

fn level_of(owned: &[u8; ITEMS.len()], key: &str) -> u8 {
    ITEMS.iter().position(|i| i.key == key).map_or(0, |k| owned[k])
}

/// A small seeded generator (splitmix64): the shop's draw must be the same
/// on every machine, so no floats.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Run {
    pub fn new(seed: u64) -> Run {
        let mut owned = [0; ITEMS.len()];
        for k in START {
            if let Some(i) = ITEMS.iter().position(|i| i.key == k) {
                owned[i] = 1;
            }
        }
        Run { seed, depth: 1, points: 0, owned, cards: Vec::new(), offers: Vec::new(), stars: Vec::new(), over: false, dev: false, rerolls: 0, beaten: false }
    }

    /// Levels owned of the item `key` (0 for an unknown key).
    pub fn level_of(&self, key: &str) -> u8 {
        level_of(&self.owned, key)
    }

    /// The unlock `key` is owned (the window's lock on the command card).
    pub fn has(&self, key: &str) -> bool {
        self.level_of(key) > 0
    }

    /// The planet card for the current depth (name, biome, size), with no
    /// map built.
    pub fn planet(&self) -> Planet {
        levelgen::planet(self.seed, self.depth)
    }

    /// The level at the current depth.
    pub fn level(&self) -> Level {
        let d = self.depth;
        let seed = mix(self.seed ^ u64::from(d).wrapping_mul(0x5851_F42D));
        // The planet the level generator makes for this run and depth
        // (its name and biome come with it); `map` as `map=` names it, so
        // the window's title and alert read its name.
        let planet = self.planet();
        let map = format!("planet:{d}:{}", self.seed);
        let tier = match LADDER.get(d as usize - 1) {
            Some(&t) => t,
            None => LADDER[LADDER.len() - 3 + (mix(seed) % 3) as usize],
        };
        let shrink = shrink_goal(d);
        let mission = Mission {
            name: format!("Planet {d} of {PLANETS}, {}", planet.name),
            says: format!("Beat the slime: cut it to {:.0}% of its peak", 100.0 * shrink),
            map: map.clone(),
            tier: Some(tier.name().to_string()),
            goal: Goal::Shrink(shrink),
            verbs: Vec::new(),
            bonus: Some(Bonus::LoseAtMost(2)),
            // Free play's bite (2), not the missions' older 1: walking into
            // the slime is very deadly (owner, 11-12).
            rules: vec![("drain".into(), Rules::default().drain)],
            own: false,
        };
        Level { depth: d, seed, ground: Ground::Planet { seed: self.seed, depth: d }, tier, walkers: 1.0 + WALKERS_PER_DEPTH * (d - 1) as f32, mission }
    }

    /// The game's rules with the squad's and the buildings' upgrades.
    pub fn rules(&self, r: &mut Rules) {
        let l = |k| f32::from(self.level_of(k));
        r.hp *= 1.0 + 0.25 * l("hp");
        r.strike *= 1.0 + 0.2 * l("strike");
        r.shot_every *= 0.85f32.powf(l("rate"));
        r.range += 3.0 * l("range");
        r.squad += 2 * self.level_of("squad") as usize;
        r.start_bank += 500.0 * l("bank");
        r.harvest *= 1.0 + 0.25 * l("harvest");
        let t = l("turret+");
        let b = &mut r.build;
        b.turret_hp *= 1.0 + 0.3 * t;
        b.turret_range += 3.0 * t;
        b.turret_every *= 0.85f32.powf(t);
    }

    /// The tools' prices with `tools+`.
    pub fn kit(&self, k: &mut KitRules) {
        let f = 0.8f32.powf(f32::from(self.level_of("tools+")));
        k.blast_price *= f;
        k.mine_price *= f;
        k.cord_price *= f;
        k.gas_price *= f;
        k.germ_price *= f;
    }

    /// The gun's card budget (`rounds::fit`).
    pub fn budget(&self) -> u32 {
        START_BUDGET + BUDGET_STEP * u32::from(self.level_of("budget"))
    }

    /// Card `c` would fit in the gun beside what it holds.
    pub fn fits(&self, c: Card) -> bool {
        let mut g = self.gun();
        g.push(c);
        rounds::fit(g, self.budget()).is_ok()
    }

    /// The gun: the cards in it, flight cards first, then the modifiers and
    /// triggers, then the payloads, each in the order bought (as the
    /// window's loadouts read: `pierce+split+blast`, `quiet+delay+wide+blast`).
    pub fn gun(&self) -> Vec<Card> {
        let rank = |c: Card| match c.kind() {
            Kind::Flight => 0,
            Kind::Modifier | Kind::Trigger => 1,
            Kind::Payload => 2,
        };
        let mut g: Vec<Card> = self.cards.iter().filter(|c| c.1).map(|c| c.0).collect();
        g.sort_by_key(|&c| rank(c));
        g
    }

    /// The gun as the window's loadout text (`rounds::parse` reads it).
    pub fn gun_text(&self) -> String {
        rounds::name(&self.gun())
    }

    /// Fills the shop's offer for the current depth: `OFFER` cards plus
    /// `draw`'s, of the powers this depth has reached, not already owned
    /// (a card may be offered again once owned only if every card is).
    pub fn restock(&mut self) {
        let n = OFFER + self.level_of("draw") as usize;
        let pool: Vec<Card> = Card::ALL.into_iter().filter(|&c| card_power(c).is_some_and(|p| power_from(p) <= self.depth)).collect();
        let fresh: Vec<Card> = pool.iter().copied().filter(|c| !self.cards.iter().any(|o| o.0 == *c)).collect();
        let mut from = if fresh.is_empty() { pool } else { fresh };
        let mut z = mix(mix(mix(self.seed ^ 0xC0FF_EE00) ^ u64::from(self.depth)) ^ self.rerolls);
        self.offers.clear();
        while self.offers.len() < n && !from.is_empty() {
            z = mix(z);
            self.offers.push(from.remove((z % from.len() as u64) as usize));
        }
    }

    /// The current level was won with `stars` (1-3): its points, the next
    /// depth, and a fresh shop. Winning the last planet wins the run.
    pub fn won(&mut self, stars: u8) {
        self.points += reward(self.depth, stars);
        self.stars.push(stars);
        if self.depth >= PLANETS {
            self.over = true;
            return;
        }
        self.depth += 1;
        self.restock();
    }

    /// The run ended on the home world's defeat.
    pub fn victory(&self) -> bool {
        self.over && !self.beaten
    }

    /// The current level was lost: the run is over.
    pub fn lost(&mut self) {
        self.over = true;
        self.beaten = true;
    }

    fn price(&self, i: usize) -> Option<u32> {
        let it = ITEMS[i];
        let l = self.owned[i];
        (l < it.max).then_some(it.cost * (u32::from(l) + 1))
    }

    /// The shop's rows: the items, the cards on offer, the cards owned.
    pub fn shop(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for (i, it) in ITEMS.iter().enumerate() {
            let l = self.owned[i];
            let price = self.price(i);
            let needs = it.needs.is_some_and(|k| !self.has(k));
            let state = if it.max == 1 {
                if l > 0 { "owned".to_string() } else { "locked".to_string() }
            } else {
                format!("{l}/{}", it.max)
            };
            let about = if needs { format!("{} (needs {})", it.about, it.needs.and_then(|k| ITEMS.iter().find(|x| x.key == k)).map_or("", |x| x.name)) } else { it.about.to_string() };
            rows.push(Row { group: it.group, name: it.name.into(), about, price: price.map_or(String::new(), |p| p.to_string()), state, ready: !needs && price.is_some_and(|p| p <= self.points), pick: Pick::Item(i) });
        }
        for (i, &c) in self.offers.iter().enumerate() {
            let p = card_cost(card_power(c).unwrap_or(1));
            rows.push(Row { group: Group::Gun, name: format!("Card: {} (budget {})", title(c.name()), c.cost()), about: c.about().into(), price: p.to_string(), state: "on offer".into(), ready: p <= self.points, pick: Pick::Offer(i) });
        }
        for (i, &(c, on)) in self.cards.iter().enumerate() {
            let fits = self.fits(c);
            rows.push(Row {
                group: Group::Gun,
                name: format!("{} (yours, budget {})", title(c.name()), c.cost()),
                about: if on { "In the gun: pick to take it out".into() } else if fits { "In the bag: pick to put it in the gun".into() } else { "In the bag: over the gun's budget, take a card out first".into() },
                price: String::new(),
                state: if on { "in the gun".into() } else { "in the bag".into() },
                ready: on || fits,
                pick: Pick::Card(i),
            });
        }
        if self.dev {
            let dev = |name: &str, about: String, pick| Row { group: Group::Dev, name: name.into(), about, price: String::new(), state: "dev".into(), ready: true, pick };
            rows.push(dev("Next planet deeper", format!("Land on planet {} instead", (self.depth + 1).min(PLANETS)), Pick::DevDepth(1)));
            rows.push(dev("Next planet shallower", format!("Land on planet {} instead", self.depth.saturating_sub(1).max(1)), Pick::DevDepth(-1)));
            rows.push(dev("1,000 points", "Free points".into(), Pick::DevPoints));
            rows.push(dev("Redraw the cards", "A fresh offer of gun cards".into(), Pick::DevReroll));
        }
        rows
    }

    /// A shop pick; false when it can't be done (too dear, maxed, locked).
    pub fn pick(&mut self, p: Pick) -> bool {
        match p {
            Pick::Item(i) => {
                let Some(price) = self.price(i).filter(|&c| c <= self.points) else { return false };
                if ITEMS[i].needs.is_some_and(|k| !self.has(k)) {
                    return false;
                }
                self.points -= price;
                self.owned[i] += 1;
                true
            }
            Pick::Offer(i) => {
                let Some(&c) = self.offers.get(i) else { return false };
                let price = card_cost(card_power(c).unwrap_or(1));
                if price > self.points {
                    return false;
                }
                self.points -= price;
                self.offers.remove(i);
                // Straight into the gun when it fits the budget.
                let room = self.fits(c);
                self.cards.push((c, room));
                true
            }
            Pick::DevDepth(by) if self.dev => {
                self.depth = (self.depth as i32 + by).clamp(1, PLANETS as i32) as u32;
                self.restock();
                true
            }
            Pick::DevPoints if self.dev => {
                self.points += 1000;
                true
            }
            Pick::DevReroll if self.dev => {
                self.rerolls += 1;
                self.restock();
                true
            }
            Pick::DevDepth(_) | Pick::DevPoints | Pick::DevReroll => false,
            Pick::Card(i) => {
                let room = self.cards.get(i).is_some_and(|c| self.fits(c.0));
                match self.cards.get_mut(i) {
                    Some(c) if c.1 || room => {
                        c.1 = !c.1;
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    /// One line on the run so far, for the shop and the end.
    pub fn line(&self) -> String {
        let stars: u32 = self.stars.iter().map(|&s| u32::from(s)).sum();
        format!("{} planet{} freed, {stars} star{}", self.stars.len(), if self.stars.len() == 1 { "" } else { "s" }, if stars == 1 { "" } else { "s" })
    }
}

fn title(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_get_harder_and_the_same_seed_gives_the_same_run() {
        let mut r = Run::new(7);
        let mut walkers = 0.0;
        let mut goals = 1.0;
        for d in 1..=10 {
            r.depth = d;
            let l = r.level();
            assert!(l.walkers > walkers, "more walkers deeper");
            assert!(shrink_goal(d) <= goals);
            (walkers, goals) = (l.walkers, shrink_goal(d));
            assert_eq!(l, Run { depth: d, ..Run::new(7) }.level());
            assert_eq!(l.ground, Ground::Planet { seed: 7, depth: d });
            assert_eq!(levelgen::spec_planet(&l.mission.map, 0), Some(Ok(levelgen::planet(7, d))), "the window reads the planet back from the mission's map");
        }
        r.depth = 1;
        assert_eq!(r.level().tier, Tier::Drifter);
        // The level generator's core world is the run's last planet.
        assert_eq!(levelgen::CORE_DEPTH, PLANETS);
    }

    #[test]
    fn a_level_is_a_mission_the_campaign_would_take() {
        let l = Run::new(1).level();
        let text = format!("name {}\nsays {}\nmap {}\ntier {}\ngoal shrink {}\nbonus lose 2\nrule drain {}\n", l.mission.name, l.mission.says, l.mission.map, l.tier.name(), shrink_goal(1), Rules::default().drain);
        assert_eq!(Mission::parse(&text).unwrap(), l.mission);
    }

    #[test]
    fn the_shop_sells_what_the_points_buy_and_no_more() {
        let mut r = Run::new(1);
        assert!(r.has("walls") && !r.has("turret"));
        assert!(r.shop().iter().all(|row| !row.ready || matches!(row.pick, Pick::Card(_))), "no points, nothing to buy");
        r.won(1);
        assert_eq!((r.depth, r.points), (2, reward(1, 1)));
        assert_eq!(r.offers.len(), OFFER);
        let turret = ITEMS.iter().position(|i| i.key == "turret").unwrap();
        let power = ITEMS.iter().position(|i| i.key == "turret+").unwrap();
        assert!(!r.pick(Pick::Item(power)), "turret power needs turrets");
        assert!(r.pick(Pick::Item(turret)));
        assert!(r.has("turret") && !r.pick(Pick::Item(turret)), "an unlock is bought once");
        assert_eq!(r.points, reward(1, 1) - 200);
        assert_eq!(reward(1, 1), 200, "the first win buys one unlock");
        assert!(!r.pick(Pick::Item(power)), "too dear now");
        // Levels rise in price.
        let hp = ITEMS.iter().position(|i| i.key == "hp").unwrap();
        r.points = 1000;
        assert!(r.pick(Pick::Item(hp)) && r.pick(Pick::Item(hp)));
        assert_eq!(r.points, 1000 - 120 - 240);
        let mut rules = Rules::default();
        r.rules(&mut rules);
        assert_eq!(rules.hp, 1.5 * Rules::default().hp);
    }

    #[test]
    fn cards_fill_the_gun_to_its_budget_and_get_better_deeper() {
        let mut r = Run::new(5);
        r.restock();
        assert!(r.offers.iter().all(|&c| card_power(c) == Some(1)), "planet 1 offers only the plainest cards");
        r.depth = 4;
        r.restock();
        assert!(r.offers.iter().all(|&c| c != Card::Germ));
        // Blast (3) and Wide (2) fill a budget of 5; Thick (1) waits in the bag.
        r.points = 10_000;
        r.offers = vec![Card::Blast, Card::Wide, Card::Thick];
        for _ in 0..3 {
            assert!(r.pick(Pick::Offer(0)));
        }
        assert_eq!(r.gun_text(), "wide+blast");
        assert!(!r.cards[2].1 && !r.pick(Pick::Card(2)), "over the budget");
        assert!(r.pick(Pick::Card(1)) && r.pick(Pick::Card(2)), "wide out, thick in");
        assert_eq!(r.gun_text(), "thick+blast");
        let big = ITEMS.iter().position(|i| i.key == "budget").unwrap();
        assert!(r.pick(Pick::Item(big)) && r.budget() == START_BUDGET + BUDGET_STEP);
        assert!(r.pick(Pick::Card(1)), "a bigger gun holds all three");
        assert!(rounds::fit(r.gun(), r.budget()).is_ok() && rounds::parse(&r.gun_text()).is_some(), "{}", r.gun_text());
        // Payloads go last.
        r.cards = vec![(Card::Blast, true), (Card::Wide, true), (Card::Pierce, true)];
        assert_eq!(r.gun_text(), "pierce+wide+blast");
        // The deep shops reach the bomb.
        let deep = (4..40).any(|d| {
            let mut r = Run::new(d);
            r.depth = d as u32;
            r.restock();
            r.offers.contains(&Card::Bomb)
        });
        assert!(deep);
    }

    #[test]
    fn dev_mode_jumps_planets_and_the_home_world_ends_the_run() {
        let mut r = Run::new(2);
        assert!(!r.pick(Pick::DevPoints), "not without dev mode");
        r.dev = true;
        let before = r.offers.clone();
        r.restock();
        assert!(r.pick(Pick::DevReroll) && r.pick(Pick::DevPoints) && r.points == 1000);
        assert!(r.offers != before || r.offers.is_empty());
        for _ in 0..20 {
            r.pick(Pick::DevDepth(1));
        }
        assert_eq!(r.depth, PLANETS);
        r.won(3);
        assert!(r.over && r.victory());
        let mut r = Run::new(2);
        r.lost();
        assert!(r.over && !r.victory());
    }

    #[test]
    fn every_ladder_tier_sets_on_the_gpu_engine() {
        for t in LADDER {
            let mut s = crate::gpu::larder_world::race_settings(1, 2, 5000);
            assert!(t.settings(&mut s).is_ok(), "{}", t.name());
        }
    }
}
