//! **Two Fevers: a contagion war over one neutral mould** (2026-11-10).
//!
//! The direction the RTS brainstorm and its red team ranked strongest
//! (`Reports/rts-design-2026-11-10/brainstorm/clean-slate-player.md`,
//! *Concept 2*; `red-team/README.md` section 3). Nobody owns the mould. A
//! host of `Rules::walkers` Jones walkers fills the dish, fed by 23 piles
//! that are eaten down and grow back, so its tubes keep moving. Each player
//! is a fever in it:
//!
//! * **Seed** (`Fevers::seed`, one spore): the clear walkers in a small disc
//!   catch your strain.
//! * **Spread:** a carrier lays its strain's own field (`Fevers::strain`),
//!   spread and faded like trail. A clear walker standing in enough of it
//!   catches it. So a fever runs along the tubes where walkers stand and
//!   dies out across bare agar.
//! * **Recover:** after `Rules::sick_seconds` a carrier is clear again and
//!   immune to *that* strain for `Rules::immune_seconds`, but not to the
//!   other. So a wave passes as a ring, and the ground a fever has just held
//!   is open to its rival. That is the comeback rule: leading feeds the
//!   trailer (`Rules::cross_immune` turns it off, for the control).
//! * **Score:** each strain's share of the host, averaged over the match. A
//!   spread, not a knockout: the host never dies, and a strain that dies out
//!   can be seeded again.
//!
//! **Its own host, not `Dish`.** The game's `Dish` is a mould that owns
//! itself: a store, upkeep, births and deaths, tools, a clock, a storm.
//! None of that is wanted here, and per-walker strain state would have to
//! survive `Dish`'s `swap_remove` deaths and its births. So this module
//! keeps the same model (Jones 2010: three sensors, turn, step, deposit, a
//! 3x3 spread that fades, one walker a cell) with a fixed population, and
//! depends on nothing but `std`, so the browser build (`fevers-web/`) can
//! include this one file as it is.
//!
//! Everything drawn (`Fevers::paint`) is state: a walker's colour is what it
//! carries or is immune to, the strain fields are tinted where they are,
//! the tubes are the trail, a pile is as green as it is full.

use std::f32::consts::TAU;

pub const W: usize = 256;
pub const H: usize = 160;
pub const STEPS_PER_SECOND: u32 = 120;
/// Each side's home in version 3 (`Rules::home_r`): the field's two end piles.
pub const HOMES: [(f32, f32); 2] = [(40.0, 80.0), (216.0, 80.0)];
/// Steps between starvation checks (`Rules::starve_seconds`).
const STARVE_EVERY: u64 = 6;

/// The numbers of one match. `Default` is the demo.
#[derive(Clone, Debug)]
pub struct Rules {
    /// Host walkers, fixed for the match.
    pub walkers: usize,
    pub sensor_angle: f32,
    pub rotate_angle: f32,
    pub sensor_offset: f32,
    pub deposit: f32,
    /// Trail kept each step after the 3x3 spread.
    pub keep: f32,
    /// Scent a full pile lays on each of its cells a step.
    pub scent: f32,
    pub scent_keep: f32,
    /// A pile's food when full, what each walker on it eats a step, and what
    /// it grows back a step.
    pub pile_food: f32,
    pub eat: f32,
    pub regrow: f32,
    /// Strain field a carrier lays a step, and the fraction kept after the
    /// spread.
    pub strain_deposit: f32,
    pub strain_keep: f32,
    /// A clear walker catches a strain with chance `catch * (f - catch_floor)`
    /// a step, where `f` is that strain's field on its cell.
    pub catch: f32,
    pub catch_floor: f32,
    pub sick_seconds: f32,
    pub immune_seconds: f32,
    /// Off (the game): immunity is per strain. On: recovering from either
    /// strain makes a walker immune to both, the control for the comeback
    /// rule.
    pub cross_immune: bool,
    /// **Owned ground** (owner's pick, 2026-11-10 21:15Z, after "everything
    /// ends back at the initial state"): a walker keeps the colour of the
    /// last fever it caught after it recovers, and the score is the share of
    /// the host each strain owns rather than carries. Off: version 1, scored
    /// on carriers.
    pub owned: bool,
    /// With `owned`: a walker that recovers is immune to the *other* strain
    /// for this long too, so ground just taken is held for a while before
    /// the rival can take it back. Its own strain's immunity
    /// (`immune_seconds`) runs longer, so the owner cannot refresh it until
    /// that lapses.
    pub hold_seconds: f32,
    /// Trail a carrier lays while its strain is hushed (`Fevers::hush`), as
    /// a share of `deposit`: tubes under a hushed fever thin and break.
    pub hush_trail: f32,
    /// Seconds between a strain's hush switches.
    pub hush_cooldown: f32,
    /// **Version 3 (design: `Reports/two-fevers-2026-11-10/v3-design.md`).**
    /// Fever by tube: when above 0, each fever's field is multiplied, every
    /// step, by `min(1, trail / tube_fever)` on each cell, so it lives only
    /// on tube, thins on thin tube and stops at a gap. 0: version 2, where
    /// it spreads over bare agar too.
    pub tube_fever: f32,
    /// The most that factor can be: above 1, thick tube keeps a fever's
    /// field longer than `strain_keep`, so it runs faster there (keep times
    /// cap must stay below 1).
    pub tube_cap: f32,
    /// A call's pull on the walkers its strain owns: `call_weight` at the
    /// call, falling straight to 0 at `call_reach` cells, added to what they
    /// sense. 0 weight: no calls. Not a diffusing field like scent: a 3x3
    /// spread kept 0.97 a step carries a smell about 3 cells, and a call has
    /// to reach a body tens of cells away.
    pub call_weight: f32,
    pub call_reach: f32,
    /// Inside this many cells of a call its pull is flat (`call_weight`
    /// everywhere), so walkers that have arrived build tubes by trail and
    /// scent as usual instead of heaping on the call. Without it, a match
    /// with the pull at 30 piled 70% of the host onto Tide's call by 2:00.
    pub call_core: f32,
    /// A pile pays foraging (`forage`) for at most this many of a strain's
    /// walkers on it, so income counts piles reached rather than walkers
    /// heaped: uncapped, the side ahead earned a seed every 4 s and took
    /// the host (raider v trunk, seed 1: 44 seeds to 7).
    pub forage_cap: u32,
    /// A pinch: the strain's own walkers within `pinch_r` lay no trail for
    /// `pinch_seconds`.
    pub pinch_r: f32,
    pub pinch_seconds: f32,
    /// Foraging: each step a strain's spore clock moves 1 plus `forage` for
    /// every walker it owns standing on a pile. 0: the flat clock of
    /// versions 1 and 2.
    pub forage: f32,
    /// Version 3's start: as play begins each side owns the walkers within
    /// this of its end pile ((40, 80) Ember, (216, 80) Tide) and calls
    /// there. 0: nobody owns anything until the first seed.
    pub home_r: f32,
    /// Catch chance multiplier for a walker the other strain owns while it
    /// is fed (on a tube route to food): a fed body resists a raid, a
    /// starving or cut-off one does not. 1: no resistance.
    pub resist: f32,
    /// Spores a pinch costs, and the seconds a side waits between pinches.
    /// A pinch first cost one spore; the scripted Web then spent every
    /// spore on pinches, seeded once a game and kept 1-2% of the host, so
    /// version 3 makes it free and lets the cut's real price be the supply
    /// it cuts (`starve_seconds`).
    pub pinch_cost: u32,
    pub pinch_wait: f32,
    /// A walker a strain owns that stands on no tube route to food (tube:
    /// trail of at least `fed_trail`, joined to a pile) for this long goes
    /// back to nobody's. 0: never.
    pub starve_seconds: f32,
    pub fed_trail: f32,
    pub seed_r: f32,
    /// Spores in hand at the start, the most there can be, and seconds per
    /// spore.
    pub spores_start: u32,
    pub spores_max: u32,
    pub spore_seconds: f32,
    /// The host grows this long before the clock starts; nobody can seed.
    pub warm_seconds: f32,
    pub match_seconds: f32,
}

/// **Why these numbers** (`Reports/two-fevers-2026-11-10/README.md`).
/// `catch` 0.03 and `sick_seconds` 4 put a lone front at about 7 cells a
/// second (p90 of carriers' distance from the seed), crossing the dish in
/// about 15 s; at 0.05 and 8 s it crossed in 10 s and held 80% of the host
/// at once, one ring the size of the dish. Spores and immunity were swept
/// on plan against random (8 seeds, both colours, 120 s): plan won all 16
/// games in every arm, by a summed-share part of 0.515 (spore 5 s, immune
/// 20 s), 0.533 (10, 20), 0.563 (15, 20), 0.555 (15, 40) and 0.558 (10,
/// 40). With immunity at 20 s the side ahead at 1:30 still led at the bell
/// in 15 or 16 of 16; at 40 s in 7 or 8. So 10 s and 40 s: a spore every
/// 10 s is a decision, and the longer immunity is what lets a lead turn.
impl Default for Rules {
    fn default() -> Self {
        Rules {
            walkers: 5_000,
            sensor_angle: 22.5f32.to_radians(),
            rotate_angle: 45f32.to_radians(),
            sensor_offset: 9.0,
            deposit: 5.0,
            keep: 0.90,
            scent: 6.0,
            scent_keep: 0.95,
            pile_food: 6_000.0,
            eat: 0.05,
            regrow: 1.5,
            strain_deposit: 1.0,
            strain_keep: 0.80,
            catch: 0.03,
            catch_floor: 0.3,
            sick_seconds: 4.0,
            immune_seconds: 40.0,
            cross_immune: false,
            owned: true,
            hold_seconds: 15.0,
            hush_trail: 0.0,
            hush_cooldown: 3.0,
            tube_fever: 0.0,
            tube_cap: 1.0,
            call_weight: 0.0,
            call_reach: 60.0,
            call_core: 15.0,
            forage_cap: 12,
            pinch_r: 5.0,
            pinch_seconds: 6.0,
            forage: 0.0,
            home_r: 0.0,
            resist: 1.0,
            pinch_cost: 1,
            pinch_wait: 0.0,
            starve_seconds: 0.0,
            fed_trail: 2.0,
            seed_r: 5.0,
            spores_start: 1,
            spores_max: 3,
            spore_seconds: 10.0,
            warm_seconds: 15.0,
            match_seconds: 180.0,
        }
    }
}

impl Rules {
    /// Version 3 (`Reports/two-fevers-2026-11-10/v3-design.md`): calls,
    /// fever by tube, pinch, foraging income and starvation, on version 2's
    /// owned ground and hold.
    pub fn v3() -> Self {
        Rules {
            tube_fever: 6.0,
            call_weight: 10.0,
            call_reach: 60.0,
            // Tuned on seeds 1-2 of the three plans: forage 0.02 and 0.005
            // with a pinch at one spore and no resistance gave the raider
            // 0.96-0.99 of every game; resistance 0.1 brought trunk v raider
            // to a contest (0.39-0.94); a free pinch every 5 s brought web
            // v raider from 0.98 to 0.44-0.63.
            forage: 0.005,
            resist: 0.1,
            pinch_cost: 0,
            pinch_wait: 5.0,
            spore_seconds: 20.0,
            spores_max: 4,
            starve_seconds: 12.0,
            home_r: 12.0,
            ..Rules::default()
        }
    }
}

/// The two strains, by index into every `[_; 2]` here.
pub const EMBER: usize = 0;
pub const TIDE: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Warm,
    Play,
    Over,
}

#[derive(Clone, Copy, Debug)]
pub struct Host {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    /// 0 clear, 1 + strain while carrying.
    pub carry: u8,
    /// Steps of carrying left.
    pub sick: u16,
    /// Steps of immunity left, per strain.
    pub immune: [u16; 2],
    /// 0 nobody, 1 + strain: the last fever it caught (`Rules::owned`).
    pub own: u8,
    /// Steps it has stood off any tube route to food (`Rules::starve_seconds`).
    pub hunger: u16,
}

impl Host {
    pub fn carrying(&self) -> Option<usize> {
        (self.carry > 0).then(|| self.carry as usize - 1)
    }
    /// Clear, and open to strain `s`.
    pub fn open_to(&self, s: usize) -> bool {
        self.carry == 0 && self.immune[s] == 0
    }
    pub fn owner(&self) -> Option<usize> {
        (self.own > 0).then(|| self.own as usize - 1)
    }
}

#[derive(Clone, Debug)]
pub struct Pile {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub food: f32,
    pub eaters: u32,
}

/// Deterministic xorshift, the same one `mould::Rng` uses, kept here so this
/// file needs nothing outside `std`.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub fn f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
    pub fn below(&mut self, n: u32) -> u32 {
        (((self.next_u64() >> 32) * n as u64) >> 32) as u32
    }
}

/// The field's 23 piles: the RTS field retest's `scatter` layout
/// (`Reports/rts-design-2026-11-10/field/README.md`), 9 mirrored pairs, 3 on
/// the centre line and the two bigger end piles that were homes there. The
/// host is neutral, so here they are only food.
pub fn field_piles() -> Vec<(f32, f32, f32)> {
    let left = [
        (78.0, 20.0),
        (67.0, 61.0),
        (18.0, 71.0),
        (56.0, 126.0),
        (110.0, 17.0),
        (98.0, 51.0),
        (92.0, 108.0),
        (96.0, 142.0),
        (65.0, 96.0),
    ];
    let mut p = vec![(40.0, 80.0, 6.0), (216.0, 80.0, 6.0)];
    for (x, y) in left {
        p.push((x, y, 4.0));
        p.push((256.0 - x, y, 4.0));
    }
    for y in [40.0, 80.0, 120.0] {
        p.push((128.0, y, 4.0));
    }
    p
}

#[derive(Clone)]
pub struct Fevers {
    pub rules: Rules,
    pub seed: u64,
    /// Steps since the host was put down, warm-up included.
    pub t: u64,
    pub agar: Vec<bool>,
    /// 1 / (agar cells in the 3x3 round each cell), 0 off agar.
    inv: Vec<f32>,
    pub trail: Vec<f32>,
    pub scent: Vec<f32>,
    /// Each strain's field.
    pub strain: [Vec<f32>; 2],
    scratch: Vec<f32>,
    occ: Vec<bool>,
    /// The pile each cell is on, `u8::MAX` none.
    pile_at: Vec<u8>,
    pub host: Vec<Host>,
    order: Vec<u32>,
    pub piles: Vec<Pile>,
    pub spores: [u32; 2],
    /// Steps towards the next spore (more than one a step with
    /// `Rules::forage`).
    spore_clock: [f32; 2],
    /// Walkers each strain owns on piles at the last step.
    pub foragers: [u32; 2],
    /// Cells on a tube route to a pile (`Rules::starve_seconds`).
    pub fed: Vec<bool>,
    /// Carrier-steps summed over play, per strain: the score is this over
    /// (walkers x play steps).
    pub held: [f64; 2],
    /// Seeds each side has planted, and walkers they caught.
    pub seeds: [u32; 2],
    pub seeded: [u32; 2],
    /// The last few seeds, (strain, x, y, step), for the HUD's ring.
    pub marks: Vec<(usize, f32, f32, u64)>,
    /// Each strain's carriers lay `Rules::hush_trail` of the usual trail.
    pub hush: [bool; 2],
    /// Step each strain may next switch hush.
    hush_at: [u64; 2],
    /// Each strain's calls (x, y), felt only by the walkers that strain owns
    /// (`Rules::call_weight`).
    pub calls: [Vec<(f32, f32)>; 2],
    /// Pinches: strain, centre, step it ends.
    pub pinches: Vec<(usize, f32, f32, u64)>,
    /// The step each side last paid for a pinch (`Rules::pinch_wait`).
    pub pinch_paid_at: [u64; 2],
    rng: Rng,
}

fn idx(x: usize, y: usize) -> usize {
    y * W + x
}

fn cell(x: f32, y: f32) -> Option<usize> {
    if x < 0.0 || y < 0.0 {
        return None;
    }
    let (cx, cy) = (x as usize, y as usize);
    (cx < W && cy < H).then(|| idx(cx, cy))
}

/// Integer offsets inside a disc of radius `r`.
pub fn disc(r: f32) -> impl Iterator<Item = (f32, f32)> {
    let n = r.ceil() as i32;
    (-n..=n)
        .flat_map(move |dy| (-n..=n).map(move |dx| (dx, dy)))
        .filter(move |(dx, dy)| ((dx * dx + dy * dy) as f32) <= r * r)
        .map(|(dx, dy)| (dx as f32, dy as f32))
}

/// The 3x3 mean over agar, times `keep`: the trail rule (`mould::spread`).
/// Off-agar cells always hold 0 (nothing is ever laid there and this writes
/// 0), so the sum can take all nine and multiply by 1 / the agar count,
/// which `inv` holds per cell (0 off agar): no branch in the loop. Measured
/// on one match (plan v random, seed 1, 30 s): the four spreads took 0.48
/// ms a step with `mould::spread`'s per-neighbour agar test, 0.10 without.
fn spread(inv: &[f32], from: &[f32], to: &mut [f32], keep: f32) {
    for y in 1..H - 1 {
        let row = y * W;
        for i in row + 1..row + W - 1 {
            let sum = from[i - W - 1]
                + from[i - W]
                + from[i - W + 1]
                + from[i - 1]
                + from[i]
                + from[i + 1]
                + from[i + W - 1]
                + from[i + W]
                + from[i + W + 1];
            to[i] = sum * inv[i] * keep;
        }
    }
}

impl Fevers {
    pub fn new(seed: u64, rules: Rules) -> Self {
        let mut agar = vec![false; W * H];
        let (cx, cy, rx, ry) = (
            W as f32 / 2.0,
            H as f32 / 2.0,
            W as f32 / 2.0 - 3.0,
            H as f32 / 2.0 - 3.0,
        );
        for y in 0..H {
            for x in 0..W {
                let dx = (x as f32 + 0.5 - cx) / rx;
                let dy = (y as f32 + 0.5 - cy) / ry;
                agar[idx(x, y)] = dx * dx + dy * dy <= 1.0;
            }
        }
        let mut inv = vec![0.0; W * H];
        for y in 1..H - 1 {
            for x in 1..W - 1 {
                let i = idx(x, y);
                if agar[i] {
                    let n = [
                        i - W - 1,
                        i - W,
                        i - W + 1,
                        i - 1,
                        i,
                        i + 1,
                        i + W - 1,
                        i + W,
                        i + W + 1,
                    ]
                    .iter()
                    .filter(|&&j| agar[j])
                    .count();
                    inv[i] = 1.0 / n as f32;
                }
            }
        }
        let mut pile_at = vec![u8::MAX; W * H];
        let piles: Vec<Pile> = field_piles()
            .into_iter()
            .map(|(x, y, r)| Pile {
                x,
                y,
                r,
                food: rules.pile_food,
                eaters: 0,
            })
            .collect();
        for (k, p) in piles.iter().enumerate() {
            for (ox, oy) in disc(p.r) {
                if let Some(i) = cell(p.x + ox, p.y + oy) {
                    pile_at[i] = k as u8;
                }
            }
        }
        let mut rng = Rng::new(seed ^ 0xF3_7E_25);
        let mut occ = vec![false; W * H];
        let mut host = Vec::with_capacity(rules.walkers);
        let mut tries = 0;
        while host.len() < rules.walkers && tries < 1_000_000 {
            tries += 1;
            let (x, y) = (rng.f32() * W as f32, rng.f32() * H as f32);
            if let Some(i) = cell(x, y) {
                if agar[i] && !occ[i] {
                    occ[i] = true;
                    host.push(Host {
                        x,
                        y,
                        heading: rng.f32() * TAU,
                        carry: 0,
                        sick: 0,
                        immune: [0; 2],
                        own: 0,
                        hunger: 0,
                    });
                }
            }
        }
        let spores = [rules.spores_start; 2];
        Fevers {
            seed,
            t: 0,
            trail: vec![0.0; W * H],
            scent: vec![0.0; W * H],
            strain: [vec![0.0; W * H], vec![0.0; W * H]],
            scratch: vec![0.0; W * H],
            occ,
            pile_at,
            host,
            order: Vec::new(),
            piles,
            spores,
            spore_clock: [0.0; 2],
            foragers: [0; 2],
            fed: vec![true; W * H],
            held: [0.0; 2],
            seeds: [0; 2],
            seeded: [0; 2],
            marks: Vec::new(),
            hush: [false; 2],
            hush_at: [0; 2],
            calls: [Vec::new(), Vec::new()],
            pinches: Vec::new(),
            pinch_paid_at: [0; 2],
            rng,
            agar,
            inv,
            rules,
        }
    }

    pub fn warm_steps(&self) -> u64 {
        (self.rules.warm_seconds * STEPS_PER_SECOND as f32) as u64
    }

    pub fn end_step(&self) -> u64 {
        self.warm_steps() + (self.rules.match_seconds * STEPS_PER_SECOND as f32) as u64
    }

    pub fn phase(&self) -> Phase {
        if self.t < self.warm_steps() {
            Phase::Warm
        } else if self.t < self.end_step() {
            Phase::Play
        } else {
            Phase::Over
        }
    }

    /// Seconds of play so far (0 in the warm-up).
    pub fn play_seconds(&self) -> f32 {
        self.t
            .saturating_sub(self.warm_steps())
            .min(self.end_step() - self.warm_steps()) as f32
            / STEPS_PER_SECOND as f32
    }

    /// Steps of play so far.
    fn play_steps(&self) -> u64 {
        self.t
            .saturating_sub(self.warm_steps())
            .min(self.end_step() - self.warm_steps())
    }

    /// Carriers of each strain now.
    pub fn carriers(&self) -> [u32; 2] {
        let mut c = [0; 2];
        for h in &self.host {
            if let Some(s) = h.carrying() {
                c[s] += 1;
            }
        }
        c
    }

    /// Walkers each strain owns now (`Rules::owned`).
    pub fn owned(&self) -> [u32; 2] {
        let mut c = [0; 2];
        for h in &self.host {
            if let Some(s) = h.owner() {
                c[s] += 1;
            }
        }
        c
    }

    /// What the score counts now, per strain: owned walkers with
    /// `Rules::owned`, carriers without.
    pub fn counted(&self) -> [u32; 2] {
        if self.rules.owned {
            self.owned()
        } else {
            self.carriers()
        }
    }

    /// Switch strain `s`'s hush; false while its cooldown runs or outside
    /// play.
    pub fn set_hush(&mut self, s: usize, on: bool) -> bool {
        if self.phase() != Phase::Play || self.t < self.hush_at[s] || self.hush[s] == on {
            return false;
        }
        self.hush[s] = on;
        self.hush_at[s] = self.t + (self.rules.hush_cooldown * STEPS_PER_SECOND as f32) as u64;
        true
    }

    /// Seconds until strain `s` may switch hush again.
    pub fn hush_wait(&self, s: usize) -> f32 {
        self.hush_at[s].saturating_sub(self.t) as f32 / STEPS_PER_SECOND as f32
    }

    /// The share of the host the score counts now (`counted`).
    pub fn share_now(&self) -> [f32; 2] {
        let c = self.counted();
        let n = self.host.len().max(1) as f32;
        [c[0] as f32 / n, c[1] as f32 / n]
    }

    /// The score so far: each strain's share of the host (owned, or
    /// carrying without `Rules::owned`) averaged over the play steps.
    pub fn score(&self) -> [f64; 2] {
        let d = (self.play_steps() as f64 * self.host.len() as f64).max(1.0);
        [self.held[0] / d, self.held[1] / d]
    }

    /// Of the two strains' summed scores, the part strain 0 holds (0.5 with
    /// neither).
    pub fn split(&self) -> f64 {
        let s = self.score();
        if s[0] + s[1] <= 0.0 {
            0.5
        } else {
            s[0] / (s[0] + s[1])
        }
    }

    /// How far the next spore has come, 0..1.
    pub fn spore_progress(&self, s: usize) -> f32 {
        if self.spores[s] >= self.rules.spores_max {
            return 1.0;
        }
        self.spore_clock[s] / (self.rules.spore_seconds * STEPS_PER_SECOND as f32)
    }

    /// Plant strain `s` at (x, y): every clear walker in the seed disc that
    /// is not immune to `s` catches it. Costs a spore only if one caught;
    /// returns how many did. Nothing during the warm-up or after the bell.
    pub fn seed(&mut self, s: usize, x: f32, y: f32) -> u32 {
        if self.phase() != Phase::Play || self.spores[s] == 0 {
            return 0;
        }
        let sick = (self.rules.sick_seconds * STEPS_PER_SECOND as f32) as u16;
        let r2 = self.rules.seed_r * self.rules.seed_r;
        let mut n = 0;
        for h in self.host.iter_mut() {
            if (h.x - x).powi(2) + (h.y - y).powi(2) <= r2 && h.open_to(s) {
                h.carry = 1 + s as u8;
                h.sick = sick;
                h.own = 1 + s as u8;
                n += 1;
            }
        }
        if n > 0 {
            self.spores[s] -= 1;
            self.seeds[s] += 1;
            self.seeded[s] += n;
            self.marks.push((s, x, y, self.t));
            let t = self.t;
            self.marks.retain(|m| t - m.3 < 2 * STEPS_PER_SECOND as u64);
        }
        n
    }

    /// How many walkers a seed at (x, y) would catch for strain `s`.
    pub fn would_catch(&self, s: usize, x: f32, y: f32) -> u32 {
        let r2 = self.rules.seed_r * self.rules.seed_r;
        self.host
            .iter()
            .filter(|h| (h.x - x).powi(2) + (h.y - y).powi(2) <= r2 && h.open_to(s))
            .count() as u32
    }

    pub fn step(&mut self) {
        if self.rules.home_r > 0.0 && self.t == self.warm_steps() {
            let r2 = self.rules.home_r * self.rules.home_r;
            for (s, (hx, hy)) in HOMES.into_iter().enumerate() {
                for h in self.host.iter_mut() {
                    if (h.x - hx).powi(2) + (h.y - hy).powi(2) <= r2 {
                        h.own = 1 + s as u8;
                    }
                }
                self.calls[s] = vec![(hx, hy)];
            }
        }
        self.lay_scent();
        self.move_host();
        self.diffuse();
        self.infect();
        self.eat();
        if self.phase() == Phase::Play {
            let c = self.counted();
            self.held[0] += c[0] as f64;
            self.held[1] += c[1] as f64;
            let every = self.rules.spore_seconds * STEPS_PER_SECOND as f32;
            for s in 0..2 {
                if self.spores[s] < self.rules.spores_max {
                    self.spore_clock[s] += 1.0 + self.rules.forage * self.foragers[s] as f32;
                    if self.spore_clock[s] >= every {
                        self.spore_clock[s] -= every;
                        self.spores[s] += 1;
                    }
                } else {
                    self.spore_clock[s] = 0.0;
                }
            }
        }
        if self.rules.starve_seconds > 0.0 && self.t.is_multiple_of(STARVE_EVERY) {
            self.starve();
        }
        self.t += 1;
    }

    fn lay_scent(&mut self) {
        let full = self.rules.pile_food;
        for p in &self.piles {
            let strength = self.rules.scent * (0.1 + 0.9 * (p.food / full).clamp(0.0, 1.0));
            for (ox, oy) in disc(p.r) {
                if let Some(i) = cell(p.x + ox, p.y + oy) {
                    if self.agar[i] {
                        self.scent[i] += strength;
                    }
                }
            }
        }
    }

    /// What a walker senses at (x, y): trail and food scent, and, for a
    /// walker a strain owns, that strain's calls (`own` is `Host::own`).
    #[inline]
    fn sense(&self, x: f32, y: f32, own: u8) -> f32 {
        match cell(x, y) {
            Some(i) if self.agar[i] => {
                let mut call = 0.0;
                if own > 0 && self.rules.call_weight > 0.0 {
                    let (core, reach) = (self.rules.call_core, self.rules.call_reach);
                    let k = 1.0 / (reach - core).max(1.0);
                    for &(cx, cy) in &self.calls[own as usize - 1] {
                        call += (1.0 - ((cx - x).hypot(cy - y) - core).max(0.0) * k).max(0.0);
                    }
                    call *= self.rules.call_weight;
                }
                self.trail[i] + self.scent[i] + call
            }
            _ => -1.0e6,
        }
    }

    /// Strain `s`'s walkers in a disc lay no trail for a while; the tube
    /// there thins and breaks (`Rules::pinch_seconds`).
    pub fn pinch(&mut self, s: usize, x: f32, y: f32) {
        let until = self.t + (self.rules.pinch_seconds * STEPS_PER_SECOND as f32) as u64;
        self.pinches.push((s, x, y, until));
    }

    /// A pinch the player pays for: one spore, and only where strain `s`
    /// owns walkers. Returns whether it was made.
    pub fn pinch_paid(&mut self, s: usize, x: f32, y: f32) -> bool {
        let wait = (self.rules.pinch_wait * STEPS_PER_SECOND as f32) as u64;
        if self.phase() != Phase::Play
            || self.spores[s] < self.rules.pinch_cost
            || (self.pinch_paid_at[s] > 0 && self.t < self.pinch_paid_at[s] + wait)
        {
            return false;
        }
        let r2 = self.rules.pinch_r * self.rules.pinch_r;
        if !self
            .host
            .iter()
            .any(|h| h.owner() == Some(s) && (h.x - x).powi(2) + (h.y - y).powi(2) <= r2)
        {
            return false;
        }
        self.spores[s] -= self.rules.pinch_cost;
        self.pinch_paid_at[s] = self.t;
        self.pinch(s, x, y);
        true
    }

    /// Mark the cells on a tube route to a pile (breadth first from every
    /// pile cell through cells of trail `fed_trail` or more), then let each
    /// owned walker off them grow hungrier; past `starve_seconds` it is
    /// nobody's again. A carrier keeps its fever meanwhile.
    fn starve(&mut self) {
        let thr = self.rules.fed_trail;
        self.fed.fill(false);
        let mut queue: Vec<usize> = Vec::new();
        for (i, &k) in self.pile_at.iter().enumerate() {
            if k != u8::MAX && self.agar[i] {
                self.fed[i] = true;
                queue.push(i);
            }
        }
        let mut head = 0;
        while head < queue.len() {
            let i = queue[head];
            head += 1;
            for j in [i - 1, i + 1, i - W, i + W] {
                if !self.fed[j] && self.agar[j] && self.trail[j] >= thr {
                    self.fed[j] = true;
                    queue.push(j);
                }
            }
        }
        let limit = (self.rules.starve_seconds * STEPS_PER_SECOND as f32) as u16;
        for h in self.host.iter_mut() {
            if h.own == 0 {
                continue;
            }
            let i = cell(h.x, h.y).expect("walker inside the dish");
            if self.fed[i] {
                h.hunger = 0;
            } else {
                h.hunger = h.hunger.saturating_add(STARVE_EVERY as u16);
                if h.hunger >= limit && h.carry == 0 {
                    h.own = 0;
                    h.hunger = 0;
                }
            }
        }
    }

    /// Set strain `s`'s calls.
    pub fn set_calls(&mut self, s: usize, calls: &[(f32, f32)]) {
        self.calls[s] = calls.to_vec();
    }

    fn move_host(&mut self) {
        let n = self.host.len();
        self.order.clear();
        self.order.extend(0..n as u32);
        for k in (1..n).rev() {
            let j = self.rng.below(k as u32 + 1) as usize;
            self.order.swap(k, j);
        }
        let (sa, ra, so, dep, sdep) = (
            self.rules.sensor_angle,
            self.rules.rotate_angle,
            self.rules.sensor_offset,
            self.rules.deposit,
            self.rules.strain_deposit,
        );
        let hushed = [0, 1].map(|s| if self.hush[s] { self.rules.hush_trail } else { 1.0 });
        let t = self.t;
        self.pinches.retain(|p| p.3 > t);
        let pr2 = self.rules.pinch_r * self.rules.pinch_r;
        let pinches = std::mem::take(&mut self.pinches);
        let (ssa, csa) = sa.sin_cos();
        let (sra, cra) = ra.sin_cos();
        for k in 0..n {
            let wi = self.order[k] as usize;
            let mut w = self.host[wi];
            // One sin_cos a walker: the side sensors and the turned heading
            // are rotations of it by fixed angles (move was 0.40 of 0.59 ms
            // a step with six trig calls).
            let (sh, ch) = w.heading.sin_cos();
            let (sl, cl) = (sh * csa - ch * ssa, ch * csa + sh * ssa);
            let (sr, cr) = (sh * csa + ch * ssa, ch * csa - sh * ssa);
            let f = self.sense(w.x + ch * so, w.y + sh * so, w.own);
            let fl = self.sense(w.x + cl * so, w.y + sl * so, w.own);
            let fr = self.sense(w.x + cr * so, w.y + sr * so, w.own);
            let mut h = w.heading;
            let (mut dx, mut dy) = (ch, sh);
            let left = (sh * cra - ch * sra, ch * cra + sh * sra);
            let right = (sh * cra + ch * sra, ch * cra - sh * sra);
            if f > fl && f > fr {
            } else if f < fl && f < fr {
                if self.rng.next_u64() & 1 == 0 {
                    h += ra;
                    (dy, dx) = right;
                } else {
                    h -= ra;
                    (dy, dx) = left;
                }
            } else if fl < fr {
                h += ra;
                (dy, dx) = right;
            } else if fr < fl {
                h -= ra;
                (dy, dx) = left;
            }
            if h >= TAU {
                h -= TAU;
            } else if h < 0.0 {
                h += TAU;
            }
            let (nx, ny) = (w.x + dx, w.y + dy);
            let here = cell(w.x, w.y).expect("walker inside the dish");
            let moved_to = match cell(nx, ny) {
                Some(there) if there == here => Some(there),
                Some(there) if self.agar[there] && !self.occ[there] => {
                    self.occ[here] = false;
                    self.occ[there] = true;
                    Some(there)
                }
                _ => None,
            };
            let at = match moved_to {
                Some(there) => {
                    w.x = nx;
                    w.y = ny;
                    w.heading = h;
                    let pinched = w.owner().is_some_and(|s| {
                        pinches
                            .iter()
                            .any(|p| p.0 == s && (p.1 - nx).powi(2) + (p.2 - ny).powi(2) <= pr2)
                    });
                    if !pinched {
                        self.trail[there] += w.carrying().map_or(dep, |s| dep * hushed[s]);
                    }
                    there
                }
                None => {
                    w.heading = self.rng.f32() * TAU;
                    here
                }
            };
            if let Some(s) = w.carrying() {
                self.strain[s][at] += sdep;
            }
            self.host[wi] = w;
        }
        self.pinches = pinches;
    }

    fn diffuse(&mut self) {
        spread(&self.inv, &self.trail, &mut self.scratch, self.rules.keep);
        std::mem::swap(&mut self.trail, &mut self.scratch);
        spread(&self.inv, &self.scent, &mut self.scratch, self.rules.scent_keep);
        std::mem::swap(&mut self.scent, &mut self.scratch);
        for s in 0..2 {
            spread(&self.inv, &self.strain[s], &mut self.scratch, self.rules.strain_keep);
            std::mem::swap(&mut self.strain[s], &mut self.scratch);
            if self.rules.tube_fever > 0.0 {
                let k = 1.0 / self.rules.tube_fever;
                let cap = self.rules.tube_cap;
                for (f, &t) in self.strain[s].iter_mut().zip(&self.trail) {
                    *f *= (t * k).min(cap);
                }
            }
        }
    }

    fn infect(&mut self) {
        let sick = (self.rules.sick_seconds * STEPS_PER_SECOND as f32) as u16;
        let immune = (self.rules.immune_seconds * STEPS_PER_SECOND as f32) as u16;
        let hold = (self.rules.hold_seconds * STEPS_PER_SECOND as f32) as u16;
        let resist = self.rules.resist;
        let (catch, floor, cross, owned) = (
            self.rules.catch,
            self.rules.catch_floor,
            self.rules.cross_immune,
            self.rules.owned,
        );
        for k in 0..self.host.len() {
            let mut h = self.host[k];
            for m in h.immune.iter_mut() {
                *m = m.saturating_sub(1);
            }
            if let Some(s) = h.carrying() {
                h.sick = h.sick.saturating_sub(1);
                if h.sick == 0 {
                    h.carry = 0;
                    if cross {
                        h.immune = [immune; 2];
                    } else {
                        h.immune[s] = immune;
                        if owned {
                            h.immune[1 - s] = h.immune[1 - s].max(hold);
                        }
                    }
                }
            } else {
                let i = cell(h.x, h.y).expect("walker inside the dish");
                let p = [0, 1].map(|s| {
                    if h.immune[s] == 0 {
                        let k = if h.own == 2 - s as u8 && h.hunger == 0 {
                            resist
                        } else {
                            1.0
                        };
                        (k * catch * (self.strain[s][i] - floor)).max(0.0)
                    } else {
                        0.0
                    }
                });
                let all = p[0] + p[1];
                if all > 0.0 {
                    let r = self.rng.f32();
                    if r < all.min(1.0) {
                        // Both fields here: the stronger is likelier.
                        let s = if r * all.max(1.0) < p[0] { 0 } else { 1 };
                        h.carry = 1 + s as u8;
                        h.sick = sick;
                        h.own = 1 + s as u8;
                    }
                }
            }
            self.host[k] = h;
        }
    }

    fn eat(&mut self) {
        for p in self.piles.iter_mut() {
            p.eaters = 0;
        }
        self.foragers = [0; 2];
        let mut per = vec![[0u32; 2]; self.piles.len()];
        for h in &self.host {
            if let Some(i) = cell(h.x, h.y) {
                let k = self.pile_at[i];
                if k != u8::MAX {
                    self.piles[k as usize].eaters += 1;
                    if let Some(s) = h.owner() {
                        per[k as usize][s] += 1;
                    }
                }
            }
        }
        for p in &per {
            for (f, n) in self.foragers.iter_mut().zip(p) {
                *f += n.min(&self.rules.forage_cap);
            }
        }
        let (eat, grow, full) = (self.rules.eat, self.rules.regrow, self.rules.pile_food);
        for p in self.piles.iter_mut() {
            p.food = (p.food - eat * p.eaters as f32 + grow).clamp(0.0, full);
        }
    }

    /// Bit-for-bit identity of the match state (tests, replay checks).
    pub fn state_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |v: u64| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
        };
        eat(self.t);
        for w in &self.host {
            eat(w.x.to_bits() as u64 | (w.y.to_bits() as u64) << 32);
            eat(w.carry as u64
                | (w.own as u64) << 4
                | (w.sick as u64) << 8
                | (w.immune[0] as u64) << 24
                | (w.immune[1] as u64) << 40);
        }
        for f in self.trail.iter().chain(&self.strain[0]).chain(&self.strain[1]) {
            eat(f.to_bits() as u64);
        }
        h
    }

    /// Draw the dish into `rgba` (W x H x 4). Every colour is state: the
    /// trail as the host's pale tubes, each strain's field as a tint, a
    /// carrier in its strain's colour, a recovered walker pale in the colour
    /// it is immune to, a pile as green as it is full.
    pub fn paint(&self, rgba: &mut [u8]) {
        const C: [[f32; 3]; 2] = [[255.0, 120.0, 30.0], [40.0, 150.0, 255.0]];
        let full = self.rules.pile_food;
        for i in 0..W * H {
            let o = i * 4;
            if !self.agar[i] {
                rgba[o..o + 4].copy_from_slice(&[6, 6, 8, 255]);
                continue;
            }
            let t = self.trail[i];
            let l = t / (t + 12.0);
            let mut c = [14.0 + 150.0 * l, 16.0 + 140.0 * l, 20.0 + 110.0 * l];
            for (field, tint) in self.strain.iter().zip(C) {
                let f = field[i];
                let a = 0.55 * f / (f + 1.5);
                for (ch, k) in c.iter_mut().zip(tint) {
                    *ch = *ch * (1.0 - a) + k * a;
                }
            }
            let k = self.pile_at[i];
            if k != u8::MAX {
                let g = (self.piles[k as usize].food / full).clamp(0.0, 1.0);
                c = [
                    c[0] * 0.5 + 30.0 * g,
                    c[1] * 0.5 + 40.0 + 120.0 * g,
                    c[2] * 0.5 + 30.0 * g,
                ];
            }
            rgba[o] = c[0].min(255.0) as u8;
            rgba[o + 1] = c[1].min(255.0) as u8;
            rgba[o + 2] = c[2].min(255.0) as u8;
            rgba[o + 3] = 255;
        }
        let imm = self.rules.immune_seconds * STEPS_PER_SECOND as f32;
        let starve = (self.rules.starve_seconds * STEPS_PER_SECOND as f32).max(1.0);
        for h in &self.host {
            let Some(i) = cell(h.x, h.y) else { continue };
            let o = i * 4;
            let c: [f32; 3] = match (h.carrying(), h.owner()) {
                (Some(s), _) => C[s],
                // Owned ground: deep where it is held (immune to the rival),
                // pale where the rival can take it.
                (None, Some(s)) if self.rules.owned => {
                    let held = h.immune[1 - s] > 0;
                    let (k, mut m) = if held { (0.45, 0.85) } else { (1.0, 0.45) };
                    // Starving (`Rules::starve_seconds`): the colour drains
                    // towards nobody's as its hunger grows.
                    if self.rules.starve_seconds > 0.0 && h.hunger > 0 {
                        m *= 1.0 - 0.8 * h.hunger as f32 / starve;
                    }
                    let base = [215.0, 205.0, 175.0];
                    [0, 1, 2].map(|j| base[j] * (1.0 - m) + C[s][j] * k * m)
                }
                _ => {
                    let a = h.immune[0] as f32 / imm;
                    let b = h.immune[1] as f32 / imm;
                    let base = [215.0, 205.0, 175.0];
                    let mut c = base;
                    for (s, v) in [(0, a), (1, b)] {
                        let m = 0.55 * v;
                        for (ch, k) in c.iter_mut().zip(C[s]) {
                            *ch = *ch * (1.0 - m) + (k * 0.6 + 255.0 * 0.4) * m;
                        }
                    }
                    c
                }
            };
            rgba[o] = c[0] as u8;
            rgba[o + 1] = c[1] as u8;
            rgba[o + 2] = c[2] as u8;
        }
    }
}

// ------------------------------------------------------------------ hands

/// A scripted player, for the census and the demo's opponent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hand {
    /// One seed at the start, on a random walker, then nothing.
    Idle,
    /// Each spore spent at a random moment within a spore's time, on a random
    /// walker: the same spending as Plan, with no aim and no timing.
    Random,
    /// Seeds on the patch with the most walkers open to it and none of its
    /// own carriers near. Version 1 counts ground the rival has just held
    /// (immune to the rival, open to it) extra; with `Rules::owned` it
    /// counts the rival's ground double and its own half.
    Plan,
    /// Plan, and hushes its fever while the rival has more carriers than it
    /// does: the plainest use of the firebreak, to see whether it moves the
    /// score at all.
    PlanHush,
    /// Version 3. **Trunk:** one call, on the fullest pile near its body,
    /// moved when that pile runs low; seeds (as Plan) with every spore.
    /// Income first.
    Trunk,
    /// **Raider:** keeps its call at home and spends every spore at once,
    /// seeding where the rival owns the thickest tube (a trunk is a highway
    /// into the rival's body).
    Raider,
    /// **Web:** two calls on piles near its body, far apart, so it grows
    /// many routes; pinches its own tube where a rival fever is closest to
    /// its ground, and seeds (as Plan) only with a spore to spare.
    Web,
}

impl Hand {
    pub fn parse(s: &str) -> Option<Hand> {
        match s {
            "idle" => Some(Hand::Idle),
            "random" => Some(Hand::Random),
            "plan" => Some(Hand::Plan),
            "planhush" => Some(Hand::PlanHush),
            "trunk" => Some(Hand::Trunk),
            "raider" => Some(Hand::Raider),
            "web" => Some(Hand::Web),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Hand::Idle => "idle",
            Hand::Random => "random",
            Hand::Plan => "plan",
            Hand::PlanHush => "planhush",
            Hand::Trunk => "trunk",
            Hand::Raider => "raider",
            Hand::Web => "web",
        }
    }
}

/// A hand playing one strain.
#[derive(Clone, Debug)]
pub struct Player {
    pub hand: Hand,
    pub strain: usize,
    rng: Rng,
    /// Random: the step it next spends at.
    next: u64,
    done: bool,
    /// Web: the step its last pinch was paid at.
    pinched_at: u64,
}

/// Coarse bins the planner reads the host through.
const BIN: usize = 8;
const BW: usize = W / BIN;
const BH: usize = H / BIN;

impl Player {
    pub fn new(hand: Hand, strain: usize, seed: u64) -> Self {
        Player {
            hand,
            strain,
            rng: Rng::new(seed ^ (0xA11CE + strain as u64 * 0x1_0000)),
            next: 0,
            done: false,
            pinched_at: 0,
        }
    }

    /// Act on this step, if the hand wants to.
    pub fn act(&mut self, g: &mut Fevers) {
        if g.phase() != Phase::Play {
            return;
        }
        let s = self.strain;
        match self.hand {
            Hand::Idle => {
                if !self.done {
                    if let Some((x, y)) = self.random_spot(g) {
                        if g.seed(s, x, y) > 0 {
                            self.done = true;
                        }
                    }
                }
            }
            Hand::Random => {
                // Version 3: a random pile as its call every 20 s.
                if g.rules.call_weight > 0.0 && g.t.is_multiple_of(20 * STEPS_PER_SECOND as u64) {
                    let p = &g.piles[self.rng.below(g.piles.len() as u32) as usize];
                    let c = (p.x, p.y);
                    g.set_calls(s, &[c]);
                }
                if g.spores[s] == 0 {
                    return;
                }
                if self.next == 0 {
                    let span = (g.rules.spore_seconds * STEPS_PER_SECOND as f32) as u32;
                    self.next = g.t + self.rng.below(span) as u64;
                }
                if g.t >= self.next {
                    // With version 3's pinch, a quarter of the spores go on
                    // pinches at a random walker it owns.
                    let pinch = g.rules.home_r > 0.0 && self.rng.below(4) == 0;
                    let own: Vec<(f32, f32)> = if pinch {
                        g.host
                            .iter()
                            .filter(|h| h.owner() == Some(s))
                            .map(|h| (h.x, h.y))
                            .collect()
                    } else {
                        Vec::new()
                    };
                    if pinch && !own.is_empty() {
                        let (x, y) = own[self.rng.below(own.len() as u32) as usize];
                        g.pinch_paid(s, x, y);
                    } else if let Some((x, y)) = self.random_spot(g) {
                        g.seed(s, x, y);
                    }
                    self.next = 0;
                }
            }
            Hand::Plan | Hand::PlanHush => {
                // A look every quarter second is plenty and keeps it cheap.
                if !g.t.is_multiple_of(30) {
                    return;
                }
                if self.hand == Hand::PlanHush {
                    let c = g.carriers();
                    g.set_hush(s, c[1 - s] > c[s]);
                }
                if g.spores[s] == 0 {
                    return;
                }
                if let Some((x, y)) = self.best_spot(g) {
                    g.seed(s, x, y);
                }
            }
            Hand::Trunk | Hand::Raider | Hand::Web => {
                if !g.t.is_multiple_of(30) {
                    return;
                }
                self.v3(g);
            }
        }
    }

    /// The three version 3 plans (`Hand::Trunk`, `Raider`, `Web`).
    fn v3(&mut self, g: &mut Fevers) {
        let (s, o) = (self.strain, 1 - self.strain);
        let home = HOMES[s];
        // Where its body is: the mean of the walkers it owns (home if none).
        let (mut cx, mut cy, mut n) = (0.0, 0.0, 0.0);
        for h in g.host.iter().filter(|h| h.owner() == Some(s)) {
            cx += h.x;
            cy += h.y;
            n += 1.0;
        }
        let body = if n > 0.0 { (cx / n, cy / n) } else { home };
        let full = g.rules.pile_food;
        // Piles by how good a call they are: food left, and near the body.
        let mut piles: Vec<(f32, f32, f32)> = g
            .piles
            .iter()
            .map(|p| {
                (
                    (p.food / full) / (1.0 + (p.x - body.0).hypot(p.y - body.1) / 40.0),
                    p.x,
                    p.y,
                )
            })
            .collect();
        piles.sort_by(|a, b| b.0.total_cmp(&a.0));
        let every_5s = g.t.is_multiple_of(5 * STEPS_PER_SECOND as u64);
        match self.hand {
            Hand::Trunk => {
                let stale = g.calls[s].first().is_none_or(|&(x, y)| {
                    g.piles
                        .iter()
                        .any(|p| (p.x - x).abs() < 0.5 && (p.y - y).abs() < 0.5 && p.food < 0.3 * full)
                });
                if every_5s && (stale || g.calls[s].first() == Some(&home)) {
                    g.set_calls(s, &[(piles[0].1, piles[0].2)]);
                }
                // It first saved to a full hand; that lost every game to a
                // hand that spent at once (the neutral mould goes to whoever
                // seeds it first), so it spends as Plan does.
                if g.spores[s] > 0 {
                    if let Some((x, y)) = self.best_spot(g) {
                        g.seed(s, x, y);
                    }
                }
            }
            Hand::Raider => {
                if g.spores[s] == 0 {
                    return;
                }
                // The rival's walker, open to it, on the thickest trail.
                let pick = g
                    .host
                    .iter()
                    .filter(|h| h.owner() == Some(o) && h.open_to(s))
                    .max_by(|a, b| {
                        let ta = g.trail[a.y as usize * W + a.x as usize];
                        let tb = g.trail[b.y as usize * W + b.x as usize];
                        ta.total_cmp(&tb)
                    })
                    .map(|h| (h.x, h.y));
                if let Some((x, y)) = pick.or_else(|| self.best_spot(g)) {
                    g.seed(s, x, y);
                }
            }
            Hand::Web => {
                if every_5s {
                    // The best pile, then the best one at least 50 cells from it.
                    let a = (piles[0].1, piles[0].2);
                    let b = piles[1..]
                        .iter()
                        .find(|p| (p.1 - a.0).hypot(p.2 - a.1) >= 50.0)
                        .map(|p| (p.1, p.2));
                    let calls: Vec<(f32, f32)> = std::iter::once(a).chain(b).collect();
                    g.set_calls(s, &calls);
                }
                // A rival carrier within 12 cells of a walker it owns: pinch
                // its own walker nearest that carrier. At first it pinched
                // whenever it had a spore and no pinch running; once the
                // rival's fever was everywhere that spent every spore on
                // pinches (1 seed to the raider's 48 in a game), so it now
                // pinches at most every 15 s and keeps one spore for it.
                let pinching = g.pinches.iter().any(|p| p.0 == s);
                let rested = self.pinched_at == 0 || g.t >= self.pinched_at + 15 * STEPS_PER_SECOND as u64;
                if !pinching && rested {
                    let mut best: Option<(f32, f32, f32)> = None;
                    for c in g.host.iter().filter(|h| h.carrying() == Some(o)) {
                        for m in g.host.iter().filter(|h| h.owner() == Some(s) && h.carry == 0) {
                            let d = (c.x - m.x).hypot(c.y - m.y);
                            if d <= 12.0 && best.is_none_or(|b| d < b.0) {
                                best = Some((d, m.x, m.y));
                            }
                        }
                        if best.is_some_and(|b| b.0 < 3.0) {
                            break;
                        }
                    }
                    if let Some((_, x, y)) = best {
                        if g.pinch_paid(s, x, y) {
                            self.pinched_at = g.t;
                            return;
                        }
                    }
                }
                if g.spores[s] > g.rules.pinch_cost {
                    if let Some((x, y)) = self.best_spot(g) {
                        g.seed(s, x, y);
                    }
                }
            }
            _ => {}
        }
    }

    /// A random walker that would catch, tried a few times.
    fn random_spot(&mut self, g: &Fevers) -> Option<(f32, f32)> {
        for _ in 0..20 {
            let w = g.host[self.rng.below(g.host.len() as u32) as usize];
            if w.open_to(self.strain) {
                return Some((w.x, w.y));
            }
        }
        None
    }

    fn best_spot(&mut self, g: &Fevers) -> Option<(f32, f32)> {
        let (s, o) = (self.strain, 1 - self.strain);
        // Per bin: walkers open to me, of those the ones immune to the
        // rival, my carriers.
        let mut open = vec![0f32; BW * BH];
        let mut mine = vec![0f32; BW * BH];
        for h in &g.host {
            let b = (h.y as usize / BIN).min(BH - 1) * BW + (h.x as usize / BIN).min(BW - 1);
            if h.open_to(s) {
                open[b] += if g.rules.owned {
                    // Taking the rival's ground swings the score twice;
                    // refreshing my own only renews its hold.
                    match h.owner() {
                        Some(x) if x == o => 2.0,
                        Some(_) => 0.5,
                        None => 1.0,
                    }
                } else if h.immune[o] > 0 {
                    1.5
                } else {
                    1.0
                };
            } else if h.carrying() == Some(s) {
                mine[b] += 1.0;
            }
        }
        let mut best = (0.0f32, None);
        for by in 0..BH {
            for bx in 0..BW {
                if open[by * BW + bx] < 4.0 {
                    continue;
                }
                let mut v = 0.0;
                for dy in -2i32..=2 {
                    for dx in -2i32..=2 {
                        let (x, y) = (bx as i32 + dx, by as i32 + dy);
                        if x < 0 || y < 0 || x >= BW as i32 || y >= BH as i32 {
                            continue;
                        }
                        let b = y as usize * BW + x as usize;
                        v += open[b] - 3.0 * mine[b];
                    }
                }
                if v > best.0 {
                    best = (v, Some((bx, by)));
                }
            }
        }
        let (bx, by) = best.1?;
        // Seed on the walker in that bin with the most open walkers round it.
        let (x0, y0) = ((bx * BIN) as f32, (by * BIN) as f32);
        let mut pick = (0, None);
        for h in &g.host {
            if h.x >= x0 && h.x < x0 + BIN as f32 && h.y >= y0 && h.y < y0 + BIN as f32 && h.open_to(s) {
                let n = g.would_catch(s, h.x, h.y);
                if n > pick.0 {
                    pick = (n, Some((h.x, h.y)));
                }
            }
        }
        pick.1
    }
}

/// One whole match between two hands, Ember first. Returns the game at the
/// bell.
pub fn play_match(seed: u64, rules: Rules, a: Hand, b: Hand) -> Fevers {
    let mut g = Fevers::new(seed, rules);
    let mut p = [Player::new(a, EMBER, seed), Player::new(b, TIDE, seed)];
    while g.phase() != Phase::Over {
        // Alternate who acts first, so neither strain always seeds first.
        let first = (g.t & 1) as usize;
        p[first].act(&mut g);
        p[1 - first].act(&mut g);
        g.step();
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    fn short() -> Rules {
        Rules {
            walkers: 1500,
            warm_seconds: 2.0,
            match_seconds: 6.0,
            ..Rules::default()
        }
    }

    #[test]
    fn a_match_replays_bit_for_bit() {
        let a = play_match(3, short(), Hand::Plan, Hand::Random);
        let b = play_match(3, short(), Hand::Plan, Hand::Random);
        assert_eq!(a.state_hash(), b.state_hash());
        assert_eq!(a.score(), b.score());
    }

    #[test]
    fn walkers_never_share_a_cell() {
        let g = play_match(1, short(), Hand::Plan, Hand::Plan);
        let mut seen = vec![false; W * H];
        for h in &g.host {
            let i = cell(h.x, h.y).unwrap();
            assert!(g.agar[i]);
            assert!(!seen[i]);
            seen[i] = true;
        }
    }

    #[test]
    fn a_seed_catches_and_costs_a_spore_and_recovery_makes_immune() {
        let mut g = Fevers::new(2, short());
        while g.phase() == Phase::Warm {
            g.step();
        }
        let w = g.host[0];
        let n = g.seed(EMBER, w.x, w.y);
        assert!(n > 0);
        assert_eq!(g.spores[EMBER], 0);
        assert_eq!(g.host[0].carrying(), Some(EMBER));
        // Seeding the same walkers again catches nothing and costs nothing.
        g.spores[EMBER] = 1;
        assert_eq!(g.seed(EMBER, w.x, w.y), 0);
        assert_eq!(g.spores[EMBER], 1);
        for _ in 0..(g.rules.sick_seconds as u32 * STEPS_PER_SECOND + 2) {
            g.step();
        }
        let h = g.host[0];
        assert_eq!(h.carry, 0, "recovered");
        assert_eq!(h.owner(), Some(EMBER), "and still ember's ground");
        assert!(h.immune[TIDE] > 0, "held: tide cannot take it yet");
        assert!(h.immune[EMBER] > h.immune[TIDE], "ember cannot refresh it for longer");
        for _ in 0..(g.rules.hold_seconds as u32 * STEPS_PER_SECOND) {
            g.step();
        }
        let h = g.host[0];
        assert_eq!(h.immune[TIDE], 0, "the hold is over: open to tide");
        assert!(h.immune[EMBER] > 0);
    }

    #[test]
    fn no_seeding_in_the_warm_up() {
        let mut g = Fevers::new(2, short());
        let w = g.host[0];
        assert_eq!(g.seed(EMBER, w.x, w.y), 0);
        assert_eq!(g.spores[EMBER], g.rules.spores_start);
    }
}
