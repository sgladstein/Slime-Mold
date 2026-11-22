//! **The strategy bot** (2026-11-19, the thread "Build a smarter bot that
//! explores strategies"; the owner, 11-18 23:58Z: "There are probably tons
//! of different strategies that I have not come up with yet", and 23:59Z:
//! "a variety of bots that play with different strategies or a bot that can
//! switch/test different strategies").
//!
//! One hand, `hand=strat:PLAN`, whose play is set by a plan of dials, so
//! each plan is its own bot and a search can walk the dials. A plan is a
//! preset name and/or `key=value` dials joined by commas:
//! `strat:turtle,go=40,lure=1`. `Plan::DIALS` lists every dial; `PRESETS`
//! the named families. With no dials it plays as the run bot does
//! (`hands::bot_hand`): drain crystals, buy soldiers, hunt.
//!
//! What it adds to the run bot, each a dial:
//! - **an economy**: pumps on the best quiet crystals near the landing
//!   (`pumps`, `walk`), upgraded (`up`), and more pumps on crystals the
//!   squad takes while out (`more`);
//! - **a base**: turrets round the first `guard` pumps on the slime's
//!   side (`turrets`), a wall arc in front (`walls`), a lure past the
//!   turrets so calm slime walks into their fire (`lure`);
//! - **timing**: soldiers bought before any building (`hold`), soldiers
//!   before the squad leaves home (`go`), the most it keeps (`cap`),
//!   whether it goes back to a raided pump (`defend`), when it steps back
//!   (`back`), and how thin the slime by its home must be before it walks
//!   in (`per`);
//! - **the shop** between planets (`shop`, `cards`), read by the league
//!   (`league.rs`), not by the hand;
//! - **switching** in the game (2026-11-20; the owner's "a bot that can
//!   switch/test different strategies", and planet 2 on today's game,
//!   where the plans that build first won 9 of 66 games and most squads
//!   were wiped by 3:04): all-out soldiers while the slime outgrows its
//!   start (`grow`), falling back home to rebuild a beaten squad
//!   (`regroup`), and a soldier's price kept back so a wiped squad is not
//!   the end of the planet (`spare`);
//! - **starving** the slime (`starve`, 11-20): with enough soldiers, clear
//!   and drain the crystal the most slime sits on, its home among them.
//!
//! It never changes the game's numbers: it plays through `Game::apply`
//! as the window does, and builds only what the run has unlocked (`own`,
//! which the league fills from the run; walls alone, as a run starts,
//! when not given).

use super::*;
use std::cell::RefCell;

/// A plan: the dials one strategy is played by.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Plan {
    /// Pumps built from home, on the best quiet crystals near it.
    pub pumps: u32,
    /// How far a pump's crystal may be for its charge, in gun ranges: a
    /// crystal's worth is its charge over 1 + walk / (`walk` ranges).
    pub walk: f32,
    /// Upgrades bought on each pump (0 to 2).
    pub up: u32,
    /// Pumps, of the first, given turrets, walls and a lure.
    pub guard: u32,
    /// Turrets round each guarded pump, on the slime's side.
    pub turrets: u32,
    /// A wall in front of each guarded pump: 0 none, 1 an arc, 2 a wider
    /// arc (two thirds round).
    pub walls: u32,
    /// A lure past each guarded pump's turrets.
    pub lure: bool,
    /// Defences of a pump before the next pump (`true`), or every pump
    /// first (`false`).
    pub guard_first: bool,
    /// Soldiers bought before anything is built.
    pub hold: u32,
    /// Soldiers before the squad leaves home to work and hunt.
    pub go: u32,
    /// The most soldiers it buys.
    pub cap: u32,
    /// Health (share of full) at which a bitten squad steps back.
    pub back: f32,
    /// The slime by its home crystal, walkers a soldier, under which the
    /// squad walks in (the run bot's 200).
    pub per: f32,
    /// Out of home, the squad goes back to a guarded pump with slime on it.
    pub defend: bool,
    /// Pumps put on crystals the squad takes while out (beyond `pumps`).
    pub more: u32,
    /// Out of home: 0 drain crystals by hand as the run bot does, 1 hunt
    /// the slime only.
    pub take: u32,
    /// The shop's items in the order bought, one level of each a pass
    /// (`runbot::WANTS` keys).
    pub shop: Vec<String>,
    /// Gun cards: 0 none, 1 the card the planet asks for (`run::bring`),
    /// 2 that and then any offer the points and the gun's budget allow.
    pub cards: u32,
    /// The unlocks the run owns (`Run::has` keys); None: walls only.
    pub own: Option<Vec<String>>,
    /// Volleys aimed a second (1 to 20): the league re-aims between the
    /// plan's once-a-second turns (`aim`). The owner's planet 3 (11-18
    /// 22:10Z) gave about 3 fire points a second, 1,469 in 7:21.
    pub aim: f32,
    /// Kiting: 0 off; else, between turns, the squad steers away from the
    /// slime while more than 30 walkers are within this share of its gun
    /// range, and lets go once clear (the owner steered half the time in
    /// his planet 3 push, 372 steers in all).
    pub kite: f32,
    /// Hold crystals (Scott's 11-19 playtest, "split my squad a lot and
    /// have each of them defend a single crystal ... I stole all the
    /// crystals"): up to this many groups, each of `group` soldiers taken
    /// from the main squad, drain a crystal of their own and keep the
    /// slime off it; a group whose crystal is spent moves to the next.
    /// The main squad (controlled, where bought soldiers join) plays the
    /// plan as before. 0: one squad.
    pub split: u32,
    /// Soldiers in each holding group.
    pub group: u32,
    /// Gun cards the league puts on the shop's shelf and buys when the
    /// points allow (`league::shop`), so a plan can be tried with a card
    /// the run happened to offer (Scott's push guns: `want=heave`).
    pub want: Vec<String>,
    /// Mop up: once the slime is down to this many walkers (0: never),
    /// the squad stops draining crystals and building pumps and hunts
    /// every square with any slime left. Without it the bot stalled on the
    /// last few walkers: the sensing skips squares under 8 (`Sense`), and a
    /// planet is won only at none (`check_end`). On Scott's planet 3,
    /// `econ,cards=0` brought 70k slime down to 2 and ran out the clock
    /// draining crystals (11-19, p3 gen1).
    pub mop: u32,
    /// Switch to soldiers: while the slime is over this many times its
    /// size at the landing (0: never), every point goes to soldiers, no
    /// building is placed, and the squad goes out, as `rush` plays; back
    /// to the plan once the slime is down to its landing size. On planet 2
    /// map 3 (11-20) the slime doubled in three minutes under every
    /// build-first plan while it waited for pumps.
    pub grow: f32,
    /// Fall back: once out, a squad down to fewer than this many soldiers
    /// (0: never) goes home and rebuilds, and goes out again at `go` or
    /// twice this, whichever is more. The owner's planet 2 (11-18) went
    /// down to one soldier at 2:00 and came back; the bots' squads fought
    /// to the last.
    pub regroup: u32,
    /// Soldiers' prices the bank keeps back from buildings and upgrades
    /// (0: none). A squad wiped with a soldier's price in the bank waits
    /// for its recruit; with less, the planet is lost (`check_end`).
    pub spare: u32,
    /// Where wanted cards come from in the league's shop: set on the
    /// shelf as if offered (true, the default since 11-19), or bought only
    /// when the run offers them (false), as a real run plays. With the
    /// shelf, Wide + Rate + Needle Blast won all 17 planet 3 maps (11-20,
    /// p3n generation 5); a run that has to be offered three cards is
    /// another game. Without the shelf a payload named bare (`torch`)
    /// takes the shop's bullet of that shell in whatever set it rolled.
    pub shelf: bool,
    /// Starve the slime (0: never): out of home with at least this many
    /// soldiers, the squad goes for the crystal with the most slime on it
    /// that it can take (under `per` walkers a soldier, the slime's home
    /// among them) and clears and drains it, in place of the crystal with
    /// the most charge (`deep`: whatever its size). On planet 2 map 3
    /// (11-20) the slime held about 13k for four minutes while the squad
    /// killed 50 walkers a second elsewhere: what it lost it grew back from
    /// the big crystal it sat on.
    pub starve: u32,
    /// Starving takes the crystal with the most slime whatever its size
    /// (`true`), not only one under `per` walkers a soldier. On planet 2
    /// map 3 in the cloud (11-20) `rush,aim=6,cards=0,starve=30` built
    /// this way won in 8:20 with 41 soldiers left, the first bot win there;
    /// at `starve=20` the squad fell from 20 to 8 in a minute in the home
    /// blob and lost.
    pub deep: bool,
    /// Points the league adds to the run before its first shop (0: none),
    /// so a measure can buy what a planet's own points would not reach
    /// (Direction's 11-20 job: Heavier shots 4 against Faster trigger 4,
    /// 480 points each, from planet 1).
    pub points: u32,
    /// Out of fire (11-20; the balance thread counted 2,387 of 2,598
    /// soldiers lost on planet 3 to fire, the bot's own Needle Blast
    /// lighting the Trunk's crust, 31 to the slime's touch): a squad with a
    /// member hurt this turn while no slime is near it, and under this
    /// share of full health, steps back as a bitten one does (0: never; 1:
    /// at the first burn). At 1 (job 14-b, 11-20) the Needle Blast deck on
    /// planet 3 lost 253 soldiers in 9 games against 1,033 but won 4 of 9
    /// against 8: four ran out the clock stepping back, so a lower share
    /// keeps the push and only saves the soldiers fire is about to kill.
    pub fire: f32,
    /// The volley takes the heaviest slime at least this share of the range
    /// away when there is any (0: the heaviest in range), so a bullet that
    /// lights the ground lights it away from the squad.
    pub standoff: f32,
    /// Mines (11-20, the owner: "Do they use bombs/mines?"; no plan had
    /// used a tool): at most this many laid and not gone off, round home
    /// while the squad is there and under the squad while slime bites it
    /// (`tools`). 0: none. Needs the run's Mines (`own`, the shop's `mine`).
    pub mines: u32,
    /// Thrown blasts: one a second at the heaviest slime in reach while a
    /// blast there would take at least this many walkers (0: never). Needs
    /// the run's Blasts (the shop's `blast`).
    pub blast: u32,
    /// A second gun body (11-20, the helper thread "Teach the bot tools
    /// and guns": no bot had ever bought a gun, `Pick::Gun`): the kind the
    /// league's shop buys once the points allow, after the wanted cards
    /// (`rifle`, `carbine`, `long`, `scatter`; `any`: the best-rolled gun
    /// on offer; None: never). With `shelf` a kind not on offer is rolled
    /// for the shop as if it were, as wanted cards are.
    pub gun: Option<String>,
    /// Who fires the bought gun: 0 the main squad (it goes in loadout 1,
    /// whose cards move to it, and the rifle stays in the armory), 1 the
    /// holding groups (`split`), from a loadout of their own (`GROUPS`)
    /// with the cards in `want2`, while the main squad keeps the rifle.
    pub arm: u32,
    /// Cards for the holding groups' loadout (`arm=1`), as `want` takes
    /// them, bought after the gun.
    pub want2: Vec<String>,
    /// Cards for the turrets' loadout (`Run::turret_loadout`), as `want`
    /// takes them, bought after the squads' and fitted to the turrets'
    /// rack (2 slots, a budget of 4 before "Turret+"). No bot had put a
    /// card on its turrets.
    pub tcards: Vec<String>,
    /// The squads' gun's cards in this firing order, as `want` names them
    /// (`blast.needle/wide`), after the shop (`Pick::Move`); cards not
    /// named keep their places after these. Empty: the shop's own order
    /// (flight cards, then modifiers and triggers, then bullets).
    pub order: Vec<String>,
    /// **Drops** (11-20; the coordinator for Direction, to price the
    /// win's base pay: no bot went for starfall ore or a boss's core, so
    /// the league's numbers showed what drops pay only by chance): the
    /// squad walks to the richest ore or core on the ground that it can
    /// reach before it is gone, while at most this many walkers a soldier
    /// lie within its range of the drop (a core: of where it slides to).
    /// The risk dial: 0 never; 5 only quiet ones; 60 most of them.
    pub drops: u32,
    /// **Out of its own fire** (11-20, Direction: on planet 2, 32 of 38
    /// of the run bot's dead and 25 of 31 on another map burned in its own
    /// Tinder fires): when a quarter of the squad (at least 2) stands in
    /// fire hotter than `HOT` (`Soldier::heat`), the squad moves toward and
    /// past its cool members (back from the slime when all are hot), and
    /// for this many seconds the hunt takes no stand near where it burned.
    /// 0: off. Unlike `fire`, it reads the heat itself, so it moves before
    /// the health is gone. Off by default: stepping out of any heat at all
    /// (0.02) lost planet 2 map 1 sooner in the cloud (210 s against 348,
    /// 4 burned against 31 but 389 harvested against 10,407), since the
    /// Torch and Tinder keep some member in fire about half the time.
    pub cool: f32,
    /// **Retake when broke** (11-20, Direction: on planet 1 run seed 3 the
    /// run bot held 16 soldiers and 228 in the bank for four minutes while
    /// the slime doubled, since it drains only crystals with under 30
    /// walkers on them until it is out): with less in the bank than a
    /// soldier and no harvest for this many seconds, it drains crystals
    /// with under `per` walkers a soldier on them, as when out. 0: off.
    pub broke: f32,
    /// **Siege the slime's home** (11-21, the second stall on planet 1 run
    /// seed 3: every other crystal dry, 43 soldiers and 56 in the bank, and
    /// the hunt shot only at the thin slime away from the home crystal,
    /// which held 11,600 walkers and grew back faster than the edges died,
    /// until the 1,800 s limit): with every crystal but the home dry or
    /// barren, less in the bank than a soldier and no harvest for `broke`
    /// seconds, the hunt takes its stands on the home's slime too, once the
    /// home holds under this many times `per` walkers a soldier (seed 3's
    /// stall: 1.35 times; one cloud game each, siege on with no limit: seed
    /// 3 won at 366 s against a loss at 678 s, but seed 2 went in at 2.1
    /// times with 30 soldiers and lost them all in ten seconds). Job 00-a
    /// on the PC (11-21, CI's plan on planet 1's 11 maps): 6 wins off, 7 at
    /// 1.5, 9 at 3 (seeds 3, 5 and 6 gained, none lost), so 3 is the
    /// default. 0: off.
    pub siege: f32,
}

/// The run's loadout the holding groups fire with `arm=1` (`Run::loadouts`:
/// 0 the squads', 1 the turrets', so the first new one).
pub(super) const GROUPS: usize = 2;

impl Default for Plan {
    /// The run bot's play (`hands::bot_hand`).
    fn default() -> Self {
        Plan {
            pumps: 0,
            walk: 4.0,
            up: 0,
            guard: 0,
            turrets: 0,
            walls: 0,
            lure: false,
            guard_first: true,
            hold: 0,
            go: 30,
            cap: 80,
            back: 0.5,
            per: 200.0,
            defend: false,
            more: 0,
            take: 0,
            shop: ["strike", "rate", "hp", "squad", "harvest", "range", "bank"].map(String::from).to_vec(),
            cards: 1,
            own: None,
            aim: 1.0,
            kite: 0.0,
            split: 0,
            group: 4,
            want: Vec::new(),
            mop: 0,
            grow: 0.0,
            regroup: 0,
            spare: 0,
            shelf: true,
            starve: 0,
            deep: false,
            points: 0,
            fire: 0.0,
            standoff: 0.0,
            mines: 0,
            blast: 0,
            gun: None,
            arm: 0,
            want2: Vec::new(),
            tcards: Vec::new(),
            order: Vec::new(),
            drops: 0,
            cool: 0.0,
            broke: 20.0,
            siege: 3.0,
        }
    }
}

/// Named families, each a starting point the search moves from: the dials
/// after the name win.
pub(super) const PRESETS: [(&str, &str); 8] = [
    // The run bot as it is (drain, buy, hunt).
    ("bot", ""),
    // Everything into soldiers and straight at the slime.
    ("rush", "go=0,cap=200,shop=strike/rate/hp/squad/range"),
    // Pumps and their upgrades first, a small squad until the income is up.
    ("econ", "pumps=4,up=2,hold=4,go=40,more=4,shop=pump+/bank/squad/strike/rate/hp"),
    // Pumps behind turrets, walls and a lure; the squad stays home long.
    ("turtle", "pumps=2,guard=2,turrets=3,walls=1,lure=1,hold=4,go=60,defend=1,more=2,shop=turret/turret+/pump+/strike/rate/hp"),
    // The owner's planet 3 (11-18 22:10Z): a pump, four turrets round it,
    // walls, a second pump, soldiers from 2:10, out at about 40.
    ("scott", "pumps=3,guard=1,turrets=4,walls=1,hold=4,go=40,defend=1,more=2,shop=turret/strike/rate/hp/pump+"),
    // A kill zone: turrets and a lure, no walls.
    ("lure", "pumps=2,guard=1,turrets=4,lure=1,hold=4,go=50,defend=1,more=2,shop=turret/turret+/strike/rate/hp"),
    // Pumps, no defences, a big squad early that takes crystals.
    ("boom", "pumps=2,hold=2,go=20,more=6,cap=120,shop=pump+/squad/strike/rate/hp"),
    // The run bot with pumps on what it takes in place of draining.
    ("taker", "more=8,go=20,shop=strike/rate/hp/pump+/squad"),
];

impl Plan {
    /// Every dial's key, for the search and the docs.
    pub const DIALS: [&'static str; 46] = ["pumps", "walk", "up", "guard", "turrets", "walls", "lure", "first", "hold", "go", "cap", "back", "per", "defend", "more", "take", "shop", "cards", "own", "name", "aim", "kite", "split", "group", "want", "mop", "grow", "regroup", "spare", "shelf", "starve", "deep", "points", "fire", "standoff", "mines", "blast", "gun", "arm", "want2", "tcards", "order", "drops", "cool", "broke", "siege"];

    /// `PRESET,key=value,...` (either part may be missing).
    pub fn parse(spec: &str) -> Result<Plan, String> {
        let mut p = Plan::default();
        for w in spec.split(',').map(str::trim).filter(|w| !w.is_empty()) {
            match w.split_once('=') {
                None => {
                    let (_, dials) = PRESETS.iter().find(|x| x.0 == w).ok_or_else(|| format!("strat: no preset {w} ({})", PRESETS.map(|x| x.0).join(", ")))?;
                    let base = Plan::parse(dials)?;
                    p = Plan { own: p.own.take().or(base.own.clone()), ..base };
                }
                Some((k, v)) => p.set(k, v)?,
            }
        }
        Ok(p)
    }

    fn set(&mut self, k: &str, v: &str) -> Result<(), String> {
        let num = |v: &str| v.parse::<f32>().map_err(|e| format!("strat: {k}={v}: {e}"));
        let whole = |v: &str| num(v).map(|x| x.max(0.0).round() as u32);
        match k {
            "pumps" => self.pumps = whole(v)?,
            "walk" => self.walk = num(v)?.max(0.1),
            "up" => self.up = whole(v)?.min(2),
            "guard" => self.guard = whole(v)?,
            "turrets" => self.turrets = whole(v)?.min(8),
            "walls" => self.walls = whole(v)?.min(2),
            "lure" => self.lure = num(v)? > 0.0,
            "first" => self.guard_first = v == "guard",
            "hold" => self.hold = whole(v)?,
            "go" => self.go = whole(v)?,
            "cap" => self.cap = whole(v)?.max(1),
            "back" => self.back = num(v)?.clamp(0.0, 1.0),
            "per" => self.per = num(v)?.max(1.0),
            "defend" => self.defend = num(v)? > 0.0,
            "more" => self.more = whole(v)?,
            "take" => self.take = whole(v)?.min(1),
            "shop" => self.shop = v.split('/').filter(|s| !s.is_empty()).map(String::from).collect(),
            "cards" => self.cards = whole(v)?.min(2),
            "own" => self.own = Some(v.split('/').filter(|s| !s.is_empty()).map(String::from).collect()),
            "aim" => self.aim = num(v)?.clamp(1.0, 20.0),
            "kite" => self.kite = num(v)?.clamp(0.0, 1.0),
            "split" => self.split = whole(v)?.min(12),
            "group" => self.group = whole(v)?.clamp(1, 40),
            "want" => self.want = v.split('/').filter(|s| !s.is_empty()).map(String::from).collect(),
            "mop" => self.mop = whole(v)?,
            "grow" => self.grow = num(v)?.max(0.0),
            "regroup" => self.regroup = whole(v)?,
            "spare" => self.spare = whole(v)?.min(20),
            "shelf" => self.shelf = num(v)? > 0.0,
            "starve" => self.starve = whole(v)?,
            "deep" => self.deep = num(v)? > 0.0,
            "points" => self.points = whole(v)?,
            "fire" => self.fire = num(v)?.clamp(0.0, 1.0),
            "standoff" => self.standoff = num(v)?.clamp(0.0, 0.95),
            "mines" => self.mines = whole(v)?.min(12),
            "blast" => self.blast = whole(v)?,
            "gun" => {
                self.gun = match v {
                    "" | "0" | "none" => None,
                    "any" => Some(v.to_string()),
                    k => Some(slime_mould::mould::guns::GunKind::parse(k).ok_or_else(|| format!("strat: gun={k}: rifle, carbine, long, scatter, any or none"))?.key().to_string()),
                }
            }
            "arm" => self.arm = whole(v)?.min(1),
            "want2" => self.want2 = v.split('/').filter(|s| !s.is_empty()).map(String::from).collect(),
            "tcards" => self.tcards = v.split('/').filter(|s| !s.is_empty()).map(String::from).collect(),
            "order" => self.order = v.split('/').filter(|s| !s.is_empty()).map(String::from).collect(),
            "drops" => self.drops = whole(v)?,
            "cool" => self.cool = num(v)?.max(0.0),
            "broke" => self.broke = num(v)?.max(0.0),
            "siege" => self.siege = num(v)?.max(0.0),
            // A label for the league's rows; the hand ignores it.
            "name" => {}
            _ => return Err(format!("strat: no dial {k} ({})", Plan::DIALS.join(", "))),
        }
        Ok(())
    }

    /// The run owns unlock `key` (`Run::has`): walls alone when not given.
    pub fn owns(&self, key: &str) -> bool {
        match &self.own {
            Some(o) => o.iter().any(|x| x == key),
            None => key == "walls",
        }
    }
}

/// One building the plan wants, and the building that stands for it.
#[derive(Clone, Copy, Debug)]
struct Site {
    kind: Kind,
    at: (f32, f32),
    /// `Game::buildings` index once placed.
    b: Option<usize>,
    /// Times placed: a site lost three times is given up.
    tries: u8,
}

#[derive(Default)]
struct Memory {
    spec: String,
    plan: Plan,
    /// Home: the first pump's crystal, else the landing.
    home: Option<(f32, f32)>,
    /// Pumps placed, by crystal, in order; each one's defences.
    pumps: Vec<(usize, Vec<Site>)>,
    /// When the bank started saving for the next building.
    saving: Option<f32>,
    /// The run bot's crystal memory: the crystal worked, its harvest when
    /// picked and when; crystals left until when; crystals ever left.
    work: Option<(usize, f32, f32)>,
    left: Vec<(usize, f32)>,
    barren: Vec<usize>,
    /// Crystals a pump was refused on.
    refused: Vec<usize>,
    /// Crystals whose pump was eaten, until when they wait.
    eaten: Vec<(usize, f32)>,
    /// The ghost the squad is raising and since when: one not up in 40 s
    /// is called off (its price comes back).
    raising: Option<(usize, f32)>,
    /// Soldiers ever bought and the most alive, for the league's row.
    pub bought: u32,
    pub peak: usize,
    /// Seconds the squad spent home, out, building and stepping back.
    pub secs: [u32; 4],
    /// Holding groups (`Plan::split`): squad and its crystal.
    held: Vec<(usize, usize)>,
    /// The slime at the landing (`Plan::grow`), and whether the plan is
    /// switched to soldiers now.
    start: f32,
    rushing: bool,
    /// The squad has been out (`Plan::regroup`), and is home rebuilding.
    been_out: bool,
    regrouping: bool,
    /// Seconds switched to soldiers and falling back, for the league's row.
    pub switched: [u32; 2],
    /// Each member's health at the last turn (`Plan::fire`), and seconds
    /// stepped back from fire, for the league's row.
    hp: Vec<(usize, f32)>,
    pub burnt: u32,
    /// Blasts thrown and mines laid (`tools`).
    tools: [u32; 2],
    /// The drops the squad set out for, by the tick each landed
    /// (`Plan::drops`).
    chased: Vec<u64>,
    /// Where members stood in fire, and until when the hunt keeps off
    /// (`Plan::cool`); seconds spent stepping out of it.
    hot: Vec<((f32, f32), f32)>,
    pub cooled: u32,
    /// The harvest at its last rise, and when (`Plan::broke`).
    harvest_at: (f32, f32),
}

thread_local! {
    static MEM: RefCell<Memory> = RefCell::new(Memory::default());
    /// The holding groups' gun and cards (`Plan::arm`), from the run's
    /// `GROUPS` loadout: the hand sees only the game, so the league sets
    /// it before each planet (`arm_groups`).
    static GROUP_ARMS: RefCell<Option<(Gun, Vec<rounds::Card>)>> = const { RefCell::new(None) };
}

/// What the holding groups fire on the next planet: the run's `GROUPS`
/// loadout's gun (with the run's upgrades) and cards, or None (they fire
/// the main squad's, as a split keeps).
pub(super) fn arm_groups(r: &Run) {
    let a = r.gun_of(GROUPS).map(|_| (r.squad_gun(GROUPS), r.cards_of(GROUPS)));
    GROUP_ARMS.with_borrow_mut(|g| *g = a);
}

/// What the league reads back after a game: soldiers bought, the most
/// alive, and the seconds home, out, building and stepping back.
pub(super) fn tally() -> (u32, usize, [u32; 4]) {
    MEM.with_borrow(|m| (m.bought, m.peak, m.secs))
}

/// Seconds the plan spent switched to soldiers (`grow`) and falling back
/// home (`regroup`).
pub(super) fn switched() -> [u32; 2] {
    MEM.with_borrow(|m| m.switched)
}

/// Turns the squad stepped back from fire (`Plan::fire`).
pub(super) fn burnt() -> u32 {
    MEM.with_borrow(|m| m.burnt)
}

/// Slime squares (`mould_grid`) round a point and on a line, shared by
/// the volley, the hunt and the defences.
pub(super) struct Sense {
    grid: Vec<u32>,
    bin: usize,
    gw: usize,
    w: usize,
    set: Vec<u32>,
    /// The fewest walkers a square needs to count as slime to aim at or
    /// walk to: 8, or 1 when mopping up (`Plan::mop`).
    floor: u32,
}

impl Sense {
    pub(super) fn of(g: &Game<GpuWorld>) -> Sense {
        let sc = g.world.scale();
        let bin = (4.0 * sc).round().max(1.0) as usize;
        let (w, _) = g.world.size();
        Sense { grid: g.world.mould_grid(bin).unwrap_or_default(), bin, gw: w.div_ceil(bin), w, set: g.world.set_cells().unwrap_or_default(), floor: 8 }
    }

    /// Every walker the squares hold.
    pub(super) fn total(&self) -> u32 {
        self.grid.iter().sum()
    }

    /// Mopping up (`Plan::mop`): at most `mop` walkers left, every square
    /// with any counts. True if so.
    pub(super) fn mop(&mut self, mop: u32) -> bool {
        let on = mop > 0 && self.total() <= mop;
        if on {
            self.floor = 1;
        }
        on
    }

    pub(super) fn centre(&self, i: usize) -> (f32, f32) {
        (((i % self.gw) as f32 + 0.5) * self.bin as f32, ((i / self.gw) as f32 + 0.5) * self.bin as f32)
    }

    /// Walkers in the squares within `r` of `p`.
    pub(super) fn mass_near(&self, p: (f32, f32), r: f32) -> u32 {
        self.grid.iter().enumerate().filter(|&(i, _)| dist(self.centre(i), p) <= r).map(|(_, &c)| c).sum()
    }

    /// The slime's middle, by weight.
    pub(super) fn middle(&self) -> Option<(f32, f32)> {
        let (mut sx, mut sy, mut n) = (0.0f64, 0.0f64, 0.0f64);
        for (i, &c) in self.grid.iter().enumerate().filter(|&(_, &c)| c >= self.floor) {
            let p = self.centre(i);
            sx += f64::from(p.0) * f64::from(c);
            sy += f64::from(p.1) * f64::from(c);
            n += f64::from(c);
        }
        (n > 0.0).then(|| ((sx / n) as f32, (sy / n) as f32))
    }

    /// A line of fire over open ground with no set walker on it (the run
    /// bot's `clear`).
    pub(super) fn clear(&self, g: &Game<GpuWorld>, a: (f32, f32), b: (f32, f32)) -> bool {
        let n = dist(a, b).ceil().max(1.0) as usize;
        (0..=n).all(|i| {
            let f = i as f32 / n as f32;
            let (x, y) = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            g.ground(x, y) && (x < 0.0 || y < 0.0 || x as usize >= self.w || self.set.get(y as usize * self.w + x as usize).is_none_or(|&c| c == 0))
        })
    }

    /// The heaviest slime squares weighed by the walk from `from`, at most 40.
    pub(super) fn ranked(&self, from: (f32, f32), range: f32) -> Vec<((f32, f32), f32)> {
        let mut v: Vec<((f32, f32), f32, f32)> = self
            .grid
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c >= self.floor)
            .map(|(i, &c)| {
                let p = self.centre(i);
                let d = dist(p, from);
                (p, d, c as f32 / (1.0 + d / (2.0 * range.max(1.0))))
            })
            .collect();
        v.sort_by(|a, b| b.2.total_cmp(&a.2));
        v.truncate(40);
        v.into_iter().map(|(p, d, _)| (p, d)).collect()
    }
}

thread_local! {
    /// Whether `between` has the squad steering away (kiting).
    static KITING: RefCell<bool> = const { RefCell::new(false) };
}

/// Between the plan's once-a-second turns (the league calls it at the
/// plan's aim rate): with `refire`, re-aims the controlled squad's volley
/// at the heaviest slime in range with a clear line, as the turn does
/// (both bots' targeting); with `kite` above 0, steers the squad away from
/// slime closer than that share of its range and lets go once clear;
/// with `mop`, aims at the last scraps too (`Plan::mop`).
pub(super) fn between(g: &mut Game<GpuWorld>, refire: bool, kite: f32, mop: u32, standoff: f32) {
    let Some(q) = g.active else { return };
    let Some(m) = g.middle(q) else { return };
    let range = g.gun_range(q) * g.world.scale();
    let mut sense = Sense::of(g);
    sense.mop(mop);
    if refire {
        let near = aim_at(g, &sense, m, range, standoff);
        g.apply(Cmd::Fire(q as u8, near));
    }
    if kite > 0.0 {
        let r = kite * range;
        let (mut sx, mut sy, mut n) = (0.0f32, 0.0f32, 0u32);
        for (i, &c) in sense.grid.iter().enumerate().filter(|&(_, &c)| c > 0) {
            let p = sense.centre(i);
            if dist(p, m) <= r {
                sx += p.0 * c as f32;
                sy += p.1 * c as f32;
                n += c;
            }
        }
        let was = KITING.with_borrow(|k| *k);
        if n > 30 {
            let (cx, cy) = (sx / n as f32, sy / n as f32);
            let d = dist((cx, cy), m).max(0.01);
            g.apply(Cmd::Steer(q as u8, ((m.0 - cx) / d, (m.1 - cy) / d)));
            KITING.with_borrow_mut(|k| *k = true);
        } else if was {
            g.apply(Cmd::Steer(q as u8, (0.0, 0.0)));
            KITING.with_borrow_mut(|k| *k = false);
        }
    }
}

/// The volley's mark: the heaviest slime in range with a clear line, at
/// least `standoff` of the range away when there is any (`Plan::standoff`).
/// The squad's tools (`Plan::mines`, `Plan::blast`), once a second from
/// the league before the hand's turn (so a tool comes before the next
/// soldier: at 400 a soldier, a rush never kept a soldier's price and a
/// mine's 80 spare): the kit is the app's, not the game's, so the hand
/// cannot reach it. In the squad mode the squad sets each off from its
/// device slot (`Cmd::Device`, free, one at a time on the slot's
/// cooldown; owner 2026-11-21 06:55Z); elsewhere soldiers take one tool
/// each at the larder (`kit_between`) and a tool is paid from the bank
/// when used. Only a tool the run owns is used.
pub(super) fn tools(g: &mut Game<GpuWorld>, p: &Plan) {
    if p.blast == 0 && p.mines == 0 {
        return;
    }
    let Some(q) = g.active.or_else(|| g.live_squads().first().copied()) else { return };
    let Some(m) = g.middle(q) else { return };
    let slot = g.rules.squads;
    if slot && g.device_ready_in(q).0 > 0.0 {
        return;
    }
    let kr = g.kit.rules;
    let sense = Sense::of(g);
    let sc = g.world.scale();
    let reach = kr.reach * sc;
    let near = |g: &Game<GpuWorld>, a: (f32, f32)| if slot { g.kit.squad_thrower(g, q, a).is_some_and(|(_, n)| n) } else { g.kit.in_reach(g, a) };
    let paid = |g: &Game<GpuWorld>, price: f32| slot || g.bank >= price;
    let throw = |g: &mut Game<GpuWorld>, t: Tool| if slot { g.apply(Cmd::Device(q as u8, t)) } else { g.use_tool(t) };
    if p.blast > 0 && p.owns("blast") && paid(g, kr.blast_price) {
        let r = kr.blast_r * sc;
        let best = sense.ranked(m, reach).into_iter().filter(|&(_, d)| d <= reach).map(|(a, _)| (a, sense.mass_near(a, r))).filter(|&(a, n)| n >= p.blast && near(g, a)).max_by_key(|&(_, n)| n);
        if let Some((a, _)) = best {
            if throw(g, Tool::Blast(a)) {
                MEM.with_borrow_mut(|mm| mm.tools[0] += 1);
                // One slot: the mine waits for the next second.
                if slot {
                    return;
                }
            }
        }
    }
    // A mine waits for `mine_trigger` walkers, so it goes where slime will
    // come: round home (the first pump, else the landing) on the slime's
    // side while the squad is there, for the raids; out, under the squad
    // while slime bites it, since the slime follows where soldiers stood.
    // Two tries in the 11-20 cloud games laid 3 that nothing set off in
    // 2:30: half way to the heaviest slime, and under the squad whenever
    // thick slime was in reach (the rush walked on and the slime never
    // came back there).
    if p.mines > 0 && p.owns("mine") && (g.kit.mines.len() as u32) < p.mines && paid(g, kr.mine_price) {
        let home = MEM.with_borrow(|mm| mm.home).unwrap_or(g.larder);
        let apart = 2.0 * kr.mine_r * sc;
        let mines = g.kit.mines.clone();
        let free = |at: (f32, f32)| mines.iter().all(|&x| dist(x, at) > apart);
        let at = if dist(m, home) < reach {
            let to = sense.middle().unwrap_or(m);
            let face = (to.1 - home.1).atan2(to.0 - home.0);
            let r = 0.6 * reach;
            [0.0f32, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5].into_iter().map(|k| (home.0 + r * (face + k * 0.5).cos(), home.1 + r * (face + k * 0.5).sin())).find(|&a| free(a) && g.world.open(a.0, a.1) && near(g, a))
        } else {
            let bitten = g.members(q).iter().any(|&k| g.soldiers[k].touching > 0);
            (bitten && free(m)).then_some(m)
        };
        if let Some(at) = at {
            if throw(g, Tool::Mine(at)) {
                MEM.with_borrow_mut(|mm| mm.tools[1] += 1);
            }
        }
    }
}

/// Blasts thrown and mines laid this game (`tools`).
/// Drops the squad set out for this planet (`Plan::drops`).
pub(super) fn drops_chased() -> u32 {
    MEM.with_borrow(|mm| mm.chased.len() as u32)
}

/// The drop to walk to (`Plan::drops`): the most points left, less for a
/// longer walk, of the ore and cores down that the squad reaches with a
/// second to spare and whose ground holds at most `drops` walkers a
/// soldier within `range`. A core is judged where it slides to (the
/// thickest slime near it, `cores.rs`), since that is where the squad
/// will stand by the time it is drained.
fn drop_to_take(g: &Game<GpuWorld>, p: &Plan, m: (f32, f32), range: f32, sense: &Sense, n: usize) -> Option<(f32, f32)> {
    let sc = g.world.scale();
    let now = g.world.ticks();
    let tps = g.rules.tps.max(1) as f32;
    let speed = (g.rules.speed * sc).max(1e-3);
    let (at, lands, _) = g
        .starfall
        .live
        .iter()
        .filter(|d| d.down && d.left >= 5.0)
        .filter_map(|d| {
            let walk = dist(d.at, m) / speed;
            let life = d.ends.saturating_sub(now) as f32 / tps;
            let lies = if d.kind == slime_mould::mould::larder_world::DropKind::Core { d.home.unwrap_or(d.at) } else { d.at };
            (walk + 1.0 < life && sense.mass_near(lies, range) <= p.drops * n as u32).then_some((d.at, d.lands, d.left / (1.0 + walk / 20.0)))
        })
        .max_by(|a, b| a.2.total_cmp(&b.2))?;
    MEM.with_borrow_mut(|mm| {
        if !mm.chased.contains(&lands) {
            mm.chased.push(lands);
        }
    });
    Some(at)
}

/// Seconds the squad stepped out of its own fire this planet (`Plan::cool`).
pub(super) fn cooled() -> u32 {
    MEM.with_borrow(|mm| mm.cooled)
}

/// Heat over which a member counts as in fire (`Plan::cool`): 2 health a
/// second at `fire_hurt` 10. On planet 2 with the Torch and Tinder, a
/// burning member's heat ran 0.2 to 0.8 (cloud, 11-21).
const HOT: f32 = 0.2;

/// A crystal holding less than this is dry to the hunt: ten points at
/// `charge` 64. A drained crystal never reaches 0 exactly (charge stays on
/// its cells), and on planet 1 run seed 2 the bot spent 30 s at a time on
/// one holding under a point, its soldiers sent home dry each time
/// (11-21).
const DRY: f32 = 640.0;

/// Whether `at` lies within `r` of where a member burned lately (`Plan::cool`).
fn hot_near(at: (f32, f32), r: f32, t: f32) -> bool {
    MEM.with_borrow(|mm| mm.hot.iter().any(|&(h, until)| until > t && dist(h, at) < r))
}

fn centroid(ps: &[(f32, f32)]) -> (f32, f32) {
    let n = ps.len().max(1) as f32;
    (ps.iter().map(|p| p.0).sum::<f32>() / n, ps.iter().map(|p| p.1).sum::<f32>() / n)
}

pub(super) fn tools_used() -> [u32; 2] {
    MEM.with_borrow(|mm| mm.tools)
}

fn aim_at(g: &Game<GpuWorld>, sense: &Sense, m: (f32, f32), range: f32, standoff: f32) -> Option<(f32, f32)> {
    let ranked = sense.ranked(m, range);
    let ok = |&(a, d): &((f32, f32), f32)| d <= range && sense.clear(g, m, a);
    ranked.iter().find(|x| x.1 >= standoff * range && ok(x)).or_else(|| ranked.iter().find(|x| ok(x))).map(|&(a, _)| a)
}

/// Where the last walkers are, for the league line once the slime is down
/// to a few (why `Plan::mop` misses them): a square each, `;` between, at
/// most 6, as `x:y:walkers:set:ground:dist:clear`. `set` counts set
/// (crust) cells in the square; `dist` and `clear` (a line of fire) are
/// from the controlled squad's middle, -1 and 0 with no squad.
pub(super) fn remnant(g: &Game<GpuWorld>) -> String {
    let sense = Sense::of(g);
    let m = g.active.and_then(|q| g.middle(q));
    let h = sense.set.len().checked_div(sense.w).unwrap_or(0);
    let v: Vec<String> = sense
        .grid
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c > 0)
        .take(6)
        .map(|(i, &c)| {
            let a = sense.centre(i);
            let (x0, y0) = ((i % sense.gw) * sense.bin, (i / sense.gw) * sense.bin);
            let set = (y0..(y0 + sense.bin).min(h)).flat_map(|y| (x0..(x0 + sense.bin).min(sense.w)).map(move |x| (x, y))).filter(|&(x, y)| sense.set[y * sense.w + x] > 0).count();
            let (d, clear) = m.map_or((-1.0, false), |m| (dist(a, m), sense.clear(g, m, a)));
            format!("{:.0}:{:.0}:{c}:{set}:{}:{d:.0}:{}", a.0, a.1, u8::from(g.ground(a.0, a.1)), u8::from(clear))
        })
        .collect();
    if v.is_empty() {
        "none".into()
    } else {
        v.join(";")
    }
}

pub(super) fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).hypot(a.1 - b.1)
}

/// The nearest point to `p` where a `kind` may go now (`build_blocked`),
/// searched outward in half-cell rings.
pub(super) fn spot(g: &Game<GpuWorld>, kind: Kind, p: (f32, f32)) -> Option<(f32, f32)> {
    let sc = g.world.scale();
    (0..=12).flat_map(|r| (0..12).map(move |k| (r, k))).map(|(r, k)| {
        let a = k as f32 / 12.0 * std::f32::consts::TAU;
        (p.0 + 0.5 * r as f32 * sc * a.cos(), p.1 + 0.5 * r as f32 * sc * a.sin())
    }).find(|q| g.build_blocked(kind, *q).is_none())
}

/// The defences of a pump on crystal `c`, facing `to` (the slime's middle):
/// turrets in an arc just off the crystal, a wall arc past them, a lure
/// past the wall within the turrets' reach.
fn defences(g: &Game<GpuWorld>, p: &Plan, c: usize, to: (f32, f32)) -> Vec<Site> {
    let sc = g.world.scale();
    let at = g.world.crystals()[c].0;
    let r = g.world.crystal_r(c);
    let face = (to.1 - at.1).atan2(to.0 - at.0);
    let on = |rad: f32, a: f32| (at.0 + rad * a.cos(), at.1 + rad * a.sin());
    let mut out = Vec::new();
    if p.owns("turret") && p.turrets > 0 {
        let n = p.turrets as f32;
        // Spread over up to 160 degrees, 40 apart.
        let step = (160.0f32 / n.max(1.0)).min(40.0).to_radians();
        for i in 0..p.turrets {
            let a = face + (i as f32 - (n - 1.0) / 2.0) * step;
            out.push(Site { kind: Kind::Turret, at: on(r + 4.0 * sc, a), b: None, tries: 0 });
        }
    }
    if p.lure {
        out.push(Site { kind: Kind::Lure, at: on(r + 11.0 * sc, face), b: None, tries: 0 });
    }
    if p.owns("walls") && p.walls > 0 {
        let half = if p.walls == 1 { 60.0f32 } else { 120.0 }.to_radians();
        let segs = if p.walls == 1 { 2 } else { 4 };
        let rad = r + 8.0 * sc;
        for s in 0..segs {
            let a0 = face - half + 2.0 * half * s as f32 / segs as f32;
            let a1 = face - half + 2.0 * half * (s + 1) as f32 / segs as f32;
            out.push(Site { kind: Kind::Wall { to: on(rad, a1) }, at: on(rad, a0), b: None, tries: 0 });
        }
    }
    out
}

/// What the bank goes to next.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// A pump on crystal `c`.
    Pump(usize),
    /// Site `s` of pump slot `i`.
    Site(usize, usize),
    /// Upgrade building `b`.
    Upgrade(usize),
}

pub(super) fn strat_hand(g: &mut Game<GpuWorld>, spec: &str) {
    let t = g.time();
    let fresh = MEM.with_borrow(|m| m.spec != spec) || t < 1.0;
    if fresh {
        let plan = match Plan::parse(spec) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        MEM.with_borrow_mut(|m| *m = Memory { spec: spec.to_string(), plan, start: g.mould as f32, ..Memory::default() });
    }
    let p = MEM.with_borrow(|m| m.plan.clone());
    let sc = g.world.scale();
    // One squad: every other squad joins the first (the controlled one
    // when crystals are held), bar the holding groups.
    let live = g.live_squads();
    let main = if p.split > 0 { g.active.filter(|a| live.contains(a)).or(live.first().copied()) } else { live.first().copied() };
    let Some(q) = main else { return };
    let holders: Vec<usize> = MEM.with_borrow(|mm| mm.held.iter().map(|h| h.0).collect());
    for &o in live.iter().filter(|&&o| o != q && (p.split == 0 || !holders.contains(&o))) {
        g.apply(Cmd::Merge(q as u8, o as u8));
    }
    if g.active != Some(q) {
        g.apply(Cmd::Active(q as u8));
    }
    if p.split > 0 {
        hold(g, &p, q);
    }
    let Some(m) = g.middle(q) else { return };
    let range = g.gun_range(q) * sc;
    let mut sense = Sense::of(g);
    let mopping = sense.mop(p.mop);
    let cs = g.world.crystals();
    let home = MEM.with_borrow(|mm| mm.home).unwrap_or(g.larder);
    let quiet = |g: &Game<GpuWorld>, c: usize| g.world.walkers_in(cs[c].0, g.world.crystal_r(c) + 0.7 * range) < 30;

    // ---- The bank: soldiers, pumps, defences, upgrades.
    // Forget buildings that fell, so their sites are wanted again.
    MEM.with_borrow_mut(|mm| {
        // A crystal whose pump the slime ate waits 90 s for another.
        for &(c, _) in mm.pumps.iter().filter(|(c, _)| g.pump_on(*c).is_none()) {
            mm.eaten.push((c, t + 90.0));
        }
        mm.eaten.retain(|&(_, until)| until > t);
        mm.pumps.retain(|(c, _)| g.pump_on(*c).is_some());
        for (_, sites) in &mut mm.pumps {
            for s in sites.iter_mut() {
                if s.b.is_some_and(|b| !g.buildings.get(b).is_some_and(|bd| bd.alive)) {
                    s.b = None;
                }
            }
        }
    });
    // Switching (`grow`, `regroup`): each with a way in and a way out, so
    // the plan does not flicker at the line.
    let alive = g.alive() as u32;
    let (rushing, regrouping) = MEM.with_borrow_mut(|mm| {
        let size = g.mould as f32;
        if p.grow > 0.0 && mm.start > 0.0 {
            if !mm.rushing && size > p.grow * mm.start {
                mm.rushing = true;
            } else if mm.rushing && size <= mm.start {
                mm.rushing = false;
            }
        }
        if p.regroup > 0 {
            if !mm.regrouping && mm.been_out && alive < p.regroup {
                mm.regrouping = true;
            } else if mm.regrouping && alive >= p.go.max(2 * p.regroup) {
                mm.regrouping = false;
                mm.been_out = false;
            }
        }
        if mm.rushing {
            mm.switched[0] += 1;
        }
        if mm.regrouping {
            mm.switched[1] += 1;
        }
        (mm.rushing, mm.regrouping)
    });
    let out = (alive >= p.go || mopping || rushing) && !regrouping;
    if out {
        MEM.with_borrow_mut(|mm| mm.been_out = true);
    }
    // What buildings and upgrades may spend: the bank less `spare`
    // soldiers' prices.
    let keep = p.spare as f32 * g.rules.soldier_cost;
    let want = |g: &Game<GpuWorld>| -> Option<Want> {
        if rushing {
            return None;
        }
        let (pumps, home, mut refused) = MEM.with_borrow(|mm| (mm.pumps.clone(), mm.home, mm.refused.clone()));
        refused.extend(MEM.with_borrow(|mm| mm.eaten.iter().map(|e| e.0).collect::<Vec<_>>()));
        let base = home.unwrap_or(g.larder);
        let next_pump = || {
            (1..cs.len())
                .filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none() && g.pump_flows(c) && quiet(g, c) && !refused.contains(&c))
                .max_by(|&a, &b| {
                    let v = |c: usize| cs[c].1.min(400_000.0) / (1.0 + dist(cs[c].0, base) / (p.walk * range).max(1.0));
                    v(a).total_cmp(&v(b))
                })
        };
        let site_want = |i: usize, sites: &[Site]| sites.iter().position(|s| s.b.is_none() && s.tries < 3).map(|s| Want::Site(i, s));
        // Pumps (and each one's defences first when `guard_first`).
        for i in 0..p.pumps as usize {
            match pumps.get(i) {
                Some((_, sites)) => {
                    if p.guard_first {
                        if let Some(w) = site_want(i, sites) {
                            return Some(w);
                        }
                    }
                }
                None => match next_pump() {
                    Some(c) => return Some(Want::Pump(c)),
                    None => break,
                },
            }
        }
        for (i, (_, sites)) in pumps.iter().enumerate() {
            if let Some(w) = site_want(i, sites) {
                return Some(w);
            }
        }
        // Upgrades.
        for (c, _) in &pumps {
            if let Some(b) = g.pump_on(*c).filter(|&b| g.buildings[b].up() && u32::from(g.buildings[b].level) <= p.up) {
                if g.upgrade_price(b).is_some() {
                    return Some(Want::Upgrade(b));
                }
            }
        }
        None
    };
    let price = |g: &Game<GpuWorld>, w: Want| match w {
        Want::Pump(c) => g.build_price(Kind::Pump, cs[c].0),
        Want::Site(i, s) => MEM.with_borrow(|mm| {
            let site = mm.pumps[i].1[s];
            g.build_price(site.kind, site.at)
        }),
        Want::Upgrade(b) => g.upgrade_price(b).unwrap_or(f32::INFINITY),
    };
    let mut bought = 0;
    for _ in 0..8 {
        let alive = g.alive() as u32;
        if alive < p.hold.min(p.cap) {
            if g.bank >= g.rules.soldier_cost && g.apply(Cmd::Buy) {
                bought += 1;
                continue;
            }
            break;
        }
        match want(g) {
            Some(w) => {
                let cost = price(g, w) + keep;
                if g.bank >= cost {
                    MEM.with_borrow_mut(|mm| mm.saving = None);
                    if !place(g, &p, w, &sense) {
                        // Refused: count it as tried so another is wanted.
                        MEM.with_borrow_mut(|mm| match w {
                            Want::Site(i, s) => mm.pumps[i].1[s].tries += 1,
                            Want::Pump(c) => mm.refused.push(c),
                            Want::Upgrade(_) => {}
                        });
                        break;
                    }
                    continue;
                }
                // Saving: no soldiers for a minute, then soldiers again.
                let since = MEM.with_borrow_mut(|mm| *mm.saving.get_or_insert(t));
                if t - since < 60.0 || alive >= p.cap {
                    break;
                }
                if g.bank >= g.rules.soldier_cost + cost.min(g.rules.soldier_cost) && g.apply(Cmd::Buy) {
                    bought += 1;
                    continue;
                }
                break;
            }
            None => {
                if alive < p.cap && g.bank >= g.rules.soldier_cost && g.apply(Cmd::Buy) {
                    bought += 1;
                    continue;
                }
                break;
            }
        }
    }
    MEM.with_borrow_mut(|mm| {
        mm.bought += bought;
        mm.peak = mm.peak.max(g.alive());
    });

    MEM.with_borrow_mut(|mm| {
        if g.harvested > mm.harvest_at.0 + 1.0 {
            mm.harvest_at = (g.harvested, t);
        }
    });

    // ---- The squad.
    let members = g.members(q);
    let n = members.len().max(1);
    let hp = members.iter().map(|&k| g.soldiers[k].hp).sum::<f32>() / n as f32 / g.rules.hp.max(1.0);
    let bitten = members.iter().filter(|&&k| g.soldiers[k].touching > 0).count();
    // In fire (`fire`): a member hurt since the last turn with no slime on
    // it or near it (the soldiers' own heat is not read back to the hand).
    let burning = MEM.with_borrow_mut(|mm| {
        let hurt = p.fire > 0.0
            && members.iter().any(|&k| {
                let s = &g.soldiers[k];
                s.touching == 0
                    && s.hp < p.fire * g.rules.hp.max(1.0) + 0.5
                    && sense.mass_near((s.x, s.y), 0.2 * range) < 5
                    && mm.hp.iter().any(|&(j, h)| j == k && s.hp < h - 0.5)
            });
        mm.hp = members.iter().map(|&k| (k, g.soldiers[k].hp)).collect();
        if hurt {
            mm.burnt += 1;
        }
        hurt
    });
    let eaten = (hp < p.back && bitten > 0) || burning;
    // The volley: the heaviest slime in range with a clear line.
    let near = aim_at(g, &sense, m, range, p.standoff);
    g.apply(Cmd::Fire(q as u8, near));
    let fresh_order = |g: &Game<GpuWorld>, o: Order| !members.iter().all(|&k| g.soldiers[k].order == o);
    let log = (t - t.round()).abs() < 0.02 && (t.round() as u64).is_multiple_of(10);
    let fin = |g: &Game<GpuWorld>, doing: usize| {
        MEM.with_borrow_mut(|mm| mm.secs[doing] += 1);
        if log {
            eprintln!(
                "STRAT t {t:.0} {} mould {} soldiers {} hp {hp:.2} bank {:.0} killed {} turret {} pumps {} buildings {}",
                ["home", "out", "build", "back"][doing],
                g.mould,
                g.alive(),
                g.bank,
                g.killed,
                g.turret_killed,
                g.buildings.iter().filter(|b| b.alive && b.kind == Kind::Pump).count(),
                g.buildings.iter().filter(|b| b.alive).count(),
            );
        }
    };
    if eaten {
        let away = near.unwrap_or(home);
        let d = dist(away, m).max(1.0);
        let back = (m.0 - (away.0 - m.0) / d * range, m.1 - (away.1 - m.1) / d * range);
        if near.is_some() {
            g.apply(Cmd::SquadOrder(q as u8, Order::Move(back.0, back.1)));
        } else if !g.squads[q].heal && g.bank > 0.0 {
            g.apply(Cmd::Heal(q as u8));
        }
        return fin(g, 3);
    }
    // Out of its own fire (`cool`): toward and past the cool members, or
    // back from the slime when every member is hot.
    if p.cool > 0.0 {
        let (hot, cold): (Vec<usize>, Vec<usize>) = members.iter().partition(|&&k| g.soldiers[k].heat > HOT);
        if hot.len() >= (members.len() / 4).max(2) {
            let at = |ks: &[usize]| centroid(&ks.iter().map(|&k| (g.soldiers[k].x, g.soldiers[k].y)).collect::<Vec<_>>());
            let h = at(&hot);
            MEM.with_borrow_mut(|mm| {
                mm.hot.retain(|x| x.1 > t);
                mm.hot.push((h, t + p.cool));
                mm.cooled += 1;
            });
            let to = if cold.is_empty() {
                let away = near.unwrap_or(home);
                let d = dist(away, m).max(1.0);
                (m.0 - (away.0 - m.0) / d * 0.5 * range, m.1 - (away.1 - m.1) / d * 0.5 * range)
            } else {
                let c = at(&cold);
                let d = dist(c, h).max(1.0);
                (c.0 + (c.0 - h.0) / d * 0.3 * range, c.1 + (c.1 - h.1) / d * 0.3 * range)
            };
            if g.ground(to.0, to.1) {
                g.apply(Cmd::SquadOrder(q as u8, Order::Move(to.0, to.1)));
                return fin(g, 3);
            }
        }
    }
    // A drop on the ground (`drops`): it lasts a minute (a core 15 s), so
    // it comes before raising, guarding and the hunt.
    if p.drops > 0 {
        if let Some(at) = drop_to_take(g, &p, m, range, &sense, n) {
            let o = Order::Move(at.0, at.1);
            if fresh_order(g, o) {
                g.apply(Cmd::SquadOrder(q as u8, o));
            }
            return fin(g, 1);
        }
    }
    // A ghost of ours to raise, with no thick slime on it: the squad
    // raises it (builders add up).
    let ghost = (0..g.buildings.len())
        .filter(|&b| g.buildings[b].alive && !g.buildings[b].up())
        .filter(|&b| sense.mass_near(g.buildings[b].at, range * 0.5) < 40 * n as u32)
        .min_by(|&a, &b| dist(g.buildings[a].at, m).total_cmp(&dist(g.buildings[b].at, m)));
    if let Some(b) = ghost {
        let since = MEM.with_borrow_mut(|mm| match mm.raising {
            Some((x, at)) if x == b => at,
            _ => {
                mm.raising = Some((b, t));
                t
            }
        });
        if t - since > 40.0 {
            g.apply(Cmd::Unbuild(b));
            MEM.with_borrow_mut(|mm| mm.raising = None);
            return fin(g, 2);
        }
        if fresh_order(g, Order::Build(b)) {
            g.apply(Cmd::SquadOrder(q as u8, Order::Build(b)));
        }
        return fin(g, 2);
    }
    // A guarded pump with slime on it: back to it (`defend`).
    let guarded: Vec<(f32, f32)> = MEM.with_borrow(|mm| mm.pumps.iter().take(p.guard as usize).map(|(c, _)| cs[*c].0).collect());
    if p.defend && out {
        let raided = guarded.iter().copied().filter(|&a| sense.mass_near(a, range) > 60).max_by(|&a, &b| sense.mass_near(a, range).cmp(&sense.mass_near(b, range)));
        if let Some(a) = raided.filter(|&a| dist(a, m) > 1.5 * range) {
            g.apply(Cmd::SquadOrder(q as u8, Order::Fight(a.0, a.1)));
            return fin(g, 0);
        }
    }
    // Home: guard the first pump (or the landing) until `go` soldiers;
    // with no pump to guard and no plan to build one, drain as the run bot.
    let has_home = MEM.with_borrow(|mm| mm.home.is_some()) || p.pumps > 0 || regrouping;
    if !out && has_home {
        let o = Order::Guard(home.0, home.1);
        if fresh_order(g, o) && dist(m, home) > 0.25 * range {
            g.apply(Cmd::SquadOrder(q as u8, o));
        }
        return fin(g, 0);
    }
    // Out: a crystal to pump (`more`), else to drain (`take` 0), else hunt.
    let pumps_now = MEM.with_borrow(|mm| mm.pumps.len()) as u32;
    if out && !mopping && p.more > 0 && pumps_now < p.pumps + p.more {
        let target = (1..cs.len())
            .filter(|&c| cs[c].1 > 0.0 && g.pump_on(c).is_none() && g.pump_flows(c))
            .filter(|&c| sense.mass_near(cs[c].0, g.world.crystal_r(c) + range) < 120 * n as u32)
            .min_by(|&a, &b| {
                let v = |c: usize| dist(cs[c].0, m) / (1.0 + cs[c].1.min(400_000.0) / 100_000.0);
                v(a).total_cmp(&v(b))
            });
        if let Some(c) = target {
            if g.bank >= g.build_price(Kind::Pump, cs[c].0) && dist(cs[c].0, m) < 2.0 * range && place(g, &p, Want::Pump(c), &sense) {
                return fin(g, 2);
            }
            let o = Order::Clear(cs[c].0 .0, cs[c].0 .1);
            if fresh_order(g, o) {
                g.apply(Cmd::SquadOrder(q as u8, o));
            }
            return fin(g, 1);
        }
    }
    hunt(g, &p, q, m, range, &sense, &members, out, mopping);
    fin(g, 1)
}

/// Spend on want `w`: place the pump or site, or buy the upgrade. True if
/// the game took it.
fn place(g: &mut Game<GpuWorld>, p: &Plan, w: Want, sense: &Sense) -> bool {
    let cs = g.world.crystals();
    match w {
        Want::Pump(c) => {
            // On its rim, toward the squad: the pump snaps to the middle.
            let (cx, cy) = cs[c].0;
            let from = g.live_squads().first().and_then(|&q| g.middle(q)).unwrap_or(g.larder);
            let d = dist(from, (cx, cy)).max(1.0);
            let r = g.world.crystal_r(c) * 0.9;
            if !g.apply(Cmd::Build(Kind::Pump, (cx + (from.0 - cx) / d * r, cy + (from.1 - cy) / d * r))) {
                return false;
            }
            let to = sense.middle().unwrap_or(cs[0].0);
            let sites = if MEM.with_borrow(|mm| mm.pumps.len()) < p.guard as usize { defences(g, p, c, to) } else { Vec::new() };
            MEM.with_borrow_mut(|mm| {
                mm.home.get_or_insert((cx, cy));
                mm.pumps.push((c, sites));
            });
            true
        }
        Want::Site(i, s) => {
            let site = MEM.with_borrow(|mm| mm.pumps[i].1[s]);
            let ok = match site.kind {
                Kind::Wall { to } => g.build_blocked(site.kind, site.at).is_none() && g.apply(Cmd::Build(Kind::Wall { to }, site.at)),
                k => spot(g, k, site.at).is_some_and(|at| g.apply(Cmd::Build(k, at))),
            };
            if ok {
                let b = g.buildings.len() - 1;
                MEM.with_borrow_mut(|mm| {
                    let st = &mut mm.pumps[i].1[s];
                    st.b = Some(b);
                    st.tries += 1;
                });
            }
            ok
        }
        Want::Upgrade(b) => g.apply(Cmd::Upgrade(b)),
    }
}

/// Out of home with nothing to build: the run bot's play (work the
/// crystal with the most charge, leave one that pays nothing, take the
/// home when the slime by it is thin, else hunt the heaviest slime).
/// The holding groups' turn (`Plan::split`): each drains its crystal and
/// keeps it, moves on when it is spent (back into the main squad when no
/// crystal is left), and one more group splits off the main squad while it
/// has `group` soldiers to spare beyond `group` (at least four) of its own.
fn hold(g: &mut Game<GpuWorld>, p: &Plan, q: usize) {
    let cs = g.world.crystals();
    let live = g.live_squads();
    let mut held = MEM.with_borrow(|mm| mm.held.clone());
    held.retain(|&(s, _)| s != q && live.contains(&s));
    let pumped: Vec<bool> = (0..cs.len()).map(|c| g.pump_on(c).is_some()).collect();
    let free = |held: &[(usize, usize)], from: (f32, f32)| {
        (1..cs.len())
            .filter(|&c| cs[c].1 > 0.0 && !pumped[c] && !held.iter().any(|h| h.1 == c))
            .max_by(|&a, &b| {
                let v = |c: usize| cs[c].1.min(200_000.0) / (1.0 + dist(cs[c].0, from) / 60.0);
                v(a).total_cmp(&v(b))
            })
    };
    let mut i = 0;
    while i < held.len() {
        let (s, c) = held[i];
        if cs[c].1 <= 0.0 {
            let from = g.middle(s).unwrap_or(cs[c].0);
            match free(&held, from) {
                Some(n) => held[i].1 = n,
                None => {
                    g.apply(Cmd::Merge(q as u8, s as u8));
                    held.remove(i);
                    continue;
                }
            }
        }
        let at = cs[held[i].1].0;
        let o = Order::Clear(at.0, at.1);
        if g.members(s).iter().any(|&k| g.soldiers[k].order != o) {
            g.apply(Cmd::SquadOrder(s as u8, o));
        }
        i += 1;
    }
    let ms = g.members(q);
    if held.len() < p.split as usize && ms.len() as u32 >= p.group + p.group.max(4) {
        let from = g.middle(q).unwrap_or(g.larder);
        let slot = (0..slime_mould::mould::larder_world::SQUADS_MAX).find(|s| !live.contains(s));
        if let (Some(c), Some(s)) = (free(&held, from), slot) {
            let mut by: Vec<usize> = ms;
            by.sort_by(|&a, &b| dist((g.soldiers[a].x, g.soldiers[a].y), cs[c].0).total_cmp(&dist((g.soldiers[b].x, g.soldiers[b].y), cs[c].0)));
            for &k in by.iter().take(p.group as usize) {
                g.apply(Cmd::Join(k, s as u8));
            }
            // With a gun of their own (`Plan::arm`), from their loadout, as
            // the window arms a squad given one in the field (`arm_squads`).
            if let Some((gun, cards)) = GROUP_ARMS.with_borrow(|a| a.clone()) {
                g.apply(Cmd::Arm(s as u8, Some(gun)));
                g.apply(Cmd::Load(s as u8, Some(rounds::Deck::new(&cards))));
                g.squads[s].loadout = GROUPS as u8;
            }
            let at = cs[c].0;
            g.apply(Cmd::SquadOrder(s as u8, Order::Clear(at.0, at.1)));
            held.push((s, c));
        }
    }
    MEM.with_borrow_mut(|mm| mm.held = held);
}

#[allow(clippy::too_many_arguments)]
fn hunt(g: &mut Game<GpuWorld>, p: &Plan, q: usize, m: (f32, f32), range: f32, sense: &Sense, members: &[usize], out: bool, mopping: bool) {
    let t = g.time();
    let cs = g.world.crystals();
    let fresh = |g: &Game<GpuWorld>, o: Order| !members.iter().all(|&k| g.soldiers[k].order == o);
    let left = MEM.with_borrow_mut(|b| {
        b.left.retain(|&(_, until)| until > t);
        b.left.iter().map(|l| l.0).collect::<Vec<_>>()
    });
    let home_mass = sense.mass_near(cs[0].0, g.world.crystal_r(0) + range);
    let home_ok = (home_mass as f32) < p.per * g.alive() as f32;
    let barren = MEM.with_borrow(|b| b.barren.clone());
    let held: Vec<usize> = MEM.with_borrow(|b| b.held.iter().map(|h| h.1).collect());
    let strong = out || (1..cs.len()).all(|c| cs[c].1 <= DRY || barren.contains(&c));
    // Broke (`broke`): no soldier to buy and no harvest lately, so a
    // crystal with some slime on it is worth the fight, up to `per` walkers
    // a soldier, as `home_ok` judges the home crystal.
    let broke = p.broke > 0.0 && g.bank < g.rules.soldier_cost && MEM.with_borrow(|mm| t - mm.harvest_at.1 > p.broke);
    // Siege (`siege`): the home crystal is the only food left.
    let siege = p.siege > 0.0
        && g.bank < g.rules.soldier_cost
        && (home_mass as f32) < p.siege * p.per * g.alive() as f32
        && (1..cs.len()).all(|c| cs[c].1 <= DRY || barren.contains(&c))
        && MEM.with_borrow(|mm| t - mm.harvest_at.1 > p.broke.max(10.0));
    let takeable = |c: usize| (sense.mass_near(cs[c].0, g.world.crystal_r(c) + range) as f32) < p.per * g.alive() as f32;
    // Starving (`starve`): the crystal with the most slime on it.
    if p.starve > 0 && out && !mopping && g.alive() as u32 >= p.starve {
        let feed = (0..cs.len())
            .filter(|&c| cs[c].1 > DRY && g.pump_on(c).is_none() && !held.contains(&c))
            .map(|c| (c, sense.mass_near(cs[c].0, g.world.crystal_r(c) + range)))
            .filter(|&(_, n)| n > 0 && (p.deep || (n as f32) < p.per * g.alive() as f32))
            .max_by_key(|&(_, n)| n);
        if let Some((c, _)) = feed {
            let at = cs[c].0;
            let o = Order::Clear(at.0, at.1);
            if fresh(g, o) {
                g.apply(Cmd::SquadOrder(q as u8, o));
            }
            return;
        }
    }
    let drains = p.take == 0 && !mopping;
    let quiet = |c: usize| g.world.walkers_in(cs[c].0, g.world.crystal_r(c) + range) < 30;
    let pick = drains
        .then(|| {
            (0..cs.len())
                .filter(|&c| cs[c].1 > DRY && !left.contains(&c) && !held.contains(&c) && (c > 0 || home_ok) && (strong || (c > 0 && quiet(c)) || (broke && c > 0 && takeable(c))) && g.pump_on(c).is_none())
                .max_by(|&a, &b| {
                    let walk = if strong || broke { 4.0 * range } else { 0.5 * range };
                    let v = |c: usize| cs[c].1.min(200_000.0) / (1.0 + dist(cs[c].0, m) / walk.max(1.0));
                    v(a).total_cmp(&v(b))
                })
        })
        .flatten();
    let work = MEM.with_borrow_mut(|b| {
        match (b.work, pick) {
            (Some((c, h, since)), _) if drains && cs.get(c).is_some_and(|x| x.1 > DRY) && !b.left.iter().any(|l| l.0 == c) && (c > 0 || home_ok) && g.pump_on(c).is_none() => {
                if g.harvested > h + 50.0 {
                    b.work = Some((c, g.harvested, t));
                } else if t - since > 30.0 && c > 0 {
                    b.left.push((c, t + 60.0));
                    if !b.barren.contains(&c) {
                        b.barren.push(c);
                    }
                    b.work = pick.filter(|&x| x != c).map(|x| (x, g.harvested, t));
                }
            }
            (_, Some(x)) => b.work = Some((x, g.harvested, t)),
            _ => b.work = None,
        }
        b.work.map(|w| w.0)
    });
    // `STRAT_DBG=1`: the hunt's choice and every crystal each 10 s, to
    // trace a stall (how the 11-21 planet 1 stalls were found).
    static DBG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *DBG.get_or_init(|| std::env::var_os("STRAT_DBG").is_some()) && (t - t.round()).abs() < 0.02 && (t.round() as u64).is_multiple_of(10) {
        let k0 = members.first().copied();
        let carry: f32 = members.iter().map(|&k| g.soldiers[k].carry).sum();
        let homing = members.iter().filter(|&&k| g.soldiers[k].homing).count();
        let dry = members.iter().filter(|&&k| g.soldiers[k].dry).count();
        eprintln!("DBG t {t:.0} m ({:.0},{:.0}) pick {pick:?} work {work:?} left {left:?} barren {barren:?} held {held:?} broke {broke} siege {siege} strong {strong} home_ok {home_ok} home_mass {home_mass} order {:?} carry {carry:.1} homing {homing} dry {dry}", m.0, m.1, k0.map(|k| g.soldiers[k].order));
        for (c, &(at, val)) in cs.iter().enumerate() {
            let r = g.world.crystal_r(c) + range;
            eprintln!("DBG   c{c} at ({:.0},{:.0}) val {val:.0} mass {} walkers {} pump {:?} d {:.0}", at.0, at.1, sense.mass_near(at, r), g.world.walkers_in(at, r), g.pump_on(c), dist(at, m));
        }
    }
    if let Some(c) = work.filter(|&c| c > 0 || home_ok) {
        let at = cs[c].0;
        let o = Order::Clear(at.0, at.1);
        if fresh(g, o) {
            g.apply(Cmd::SquadOrder(q as u8, o));
        }
        return;
    }
    // Hunt: from a stand with a clear line to one of the heaviest squares;
    // the home's slime only once it is thin enough to walk into.
    let stand_for = |a: (f32, f32)| {
        let d = dist(a, m).max(1.0);
        (a.0 - (a.0 - m.0) / d * 0.7 * range, a.1 - (a.1 - m.1) / d * 0.7 * range)
    };
    let by_home = |a: (f32, f32)| dist(a, cs[0].0) <= g.world.crystal_r(0) + range;
    let ranked = sense.ranked(m, range);
    // Not back into its own fire (`cool`).
    let cool_r = if p.cool > 0.0 { 0.35 * range } else { 0.0 };
    let shot = ranked.iter().map(|&(a, _)| (a, stand_for(a))).filter(|&(a, _)| home_ok || siege || !by_home(a)).find(|&(a, st)| g.ground(st.0, st.1) && sense.clear(g, st, a) && !hot_near(st, cool_r, t));
    let shot = shot.or_else(|| ranked.iter().map(|&(a, _)| (a, stand_for(a))).find(|&(a, _)| home_ok || siege || !by_home(a)));
    let Some((a, stand)) = shot.or_else(|| ranked.first().map(|&(a, _)| (a, stand_for(a)))) else { return };
    let d = dist(a, m).max(1.0);
    if (d > 0.8 * range || !sense.clear(g, m, a)) && !hot_near(stand, cool_r, t) {
        g.apply(Cmd::SquadOrder(q as u8, Order::Fight(stand.0, stand.1)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_parses_and_dials_win_over_it() {
        for (name, _) in PRESETS {
            Plan::parse(name).unwrap_or_else(|e| panic!("{name}: {e}"));
        }
        let p = Plan::parse("turtle,go=12,own=walls/turret").unwrap();
        assert_eq!(p.go, 12);
        assert_eq!(p.turrets, 3);
        assert!(p.owns("turret") && !p.owns("mortar"));
        assert!(Plan::parse("nonsense").is_err());
        assert!(Plan::parse("pumps=x").is_err());
        assert_eq!(Plan::parse("").unwrap(), Plan::default());
    }
}
