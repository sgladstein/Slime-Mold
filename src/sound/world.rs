//! **Larder World's listener**: reads the game, says what should be heard.
//!
//! Asked for after the owner's first playtest (2026-11-12 04:13Z: "shooting
//! the slime didn't feel satisfying. I know it was killing lots of
//! individual walkers but visually it didn't impact much"). The graphics
//! thread makes a hit read on screen (the wound, the hole the kills leave);
//! this makes it read to the ear, from the same counts:
//!
//! - **A shot** is a crack where the soldier stands, and where it struck,
//!   an impact whose body is the walkers it killed (`Shot::killed`): a shot
//!   into a thick vein is a heavy low slap, one that grazes the edge a light
//!   tap, one stopped by rock or wall no impact at all. Every kill is also a
//!   grain of the kill crackle, so a volley into the mould's heart is a
//!   roar of pops and a stray shot is one.
//! - **The mould closing in** is a pulse whose tempo and level follow how
//!   near the squad's nearest mould is (`Soldier::aim`, the densest block a
//!   soldier sees, against `Rules::range`), and a wet squelch for the
//!   walkers touching soldiers (`Soldier::touching`). A soldier losing
//!   health groans; one falling gets two low notes.
//! - **The economy** answers too: the mould at the larder slurps at the
//!   rate it steals, a carrier banking food chimes, a soldier bought or a
//!   wall laid thunks, a crystal running dry knocks, the core taken and the
//!   end of the round ring.
//!
//! Like [`super::listen`] for the dish, it takes `&Game` and draws no random
//! numbers, so listening cannot change a round (the guard is
//! `tests::listening_never_changes_the_round`). It is placed by an [`Ear`]:
//! the camera's view. What is on screen is panned across it at full level;
//! what is off it is quieter the farther it is, so a fight across the map
//! does not sound like the one in front of you.

use crate::mould::larder_world::{Game, Shot, World};
use crate::mould::larder::End;

use super::listen::density;
use super::{AlertKind, GrainKind, SoundCmd, TextureKind};

/// Walkers a shot must kill to sound at full weight: the 90th centile of a
/// rush on fork seed 1 (2026-11-12, 43 shots: 10th centile 1, quartiles 5,
/// 37 and 56, top 81; the farm hand's 95 defending shots: median 0, top 4).
/// At 30 (the first guess) 30 of the rush's 43 shots sounded at full
/// weight, so the heavy shots were not told apart. Re-measure if the
/// strike changes (`Rules::strike`, the graphics thread's per-walker wound).
///
/// It holds at that round's density, 5,000 walkers on the 256 x 160 dish
/// cells (`REF_DENSITY`). A denser world kills more a shot, so the full
/// weight grows with the round's starting walkers a dish cell
/// (`full_kill`), or every shot of a big world would sound full (tech lead
/// review of #96). Not in proportion: the same rush at 50,000 walkers
/// killed 66 a shot on average (18 shots) against 35 at 5,000 (43 shots),
/// 1.9 times for 10 times the walkers, so the full weight follows density
/// to the power `DENSITY_LAW` (0.3). In proportion, all 18 of the 50,000
/// rush's shots were light. The starting count, not the living one, so a
/// mould thinned by the squad does sound thinner.
pub const FULL_KILL: f32 = 80.0;
/// Walkers a dish cell where `FULL_KILL` was measured.
pub const REF_DENSITY: f32 = 5_000.0 / (256.0 * 160.0);
/// How a shot's kills grow with density (fork seed 1, 5,000 and 50,000
/// walkers: log 1.9 / log 10).
pub const DENSITY_LAW: f32 = 0.3;
/// Shots voiced a drawn frame; the rest still count in the kill crackle.
/// The voice bank keeps four verb grains of a kind at once
/// (`synth::PER_KIND_CAP`), so more would only steal from each other.
pub const SHOTS_VOICED: usize = 6;
/// Seconds the per-second rates (kills, stealing) are smoothed over.
const RATE_SMOOTH: f32 = 0.3;
/// Health lost before a soldier groans, and the least gap between groans.
const HURT_EVERY: f32 = 8.0;
const HURT_GAP: f32 = 0.35;
/// The pulse: beats a minute with mould just in sight, and with it on a
/// soldier; quieter than that, no pulse at all.
const PULSE_SLOW: f32 = 54.0;
const PULSE_FAST: f32 = 150.0;
const PULSE_FLOOR: f32 = 0.05;
/// The least gap between banking chimes, seconds. Carriers bank a few food
/// at a time: the farm hand banked 325 times in two minutes (1,770 food),
/// which chimed nearly three times a second, so banking is gathered.
const BANK_GAP: f32 = 1.0;
/// The least level of a where-to-look sound off screen (the larder robbed,
/// a crystal run dry), against 1 on screen.
const ALERT_FLOOR: f32 = 0.7;

/// How heavy a shot that killed `killed` walkers sounds, 0 ..= 1, when
/// `full` kills is a full vein (`full_kill`). Square root, so at the
/// reference density one kill is already heard (0.11), the rush's median
/// (37) is 0.68 and a full vein is 1.
pub fn kill_weight(killed: u32, full: f32) -> f32 {
    (killed as f32 / full.max(1.0)).sqrt().min(1.0)
}

/// A full vein's kills in this round: `FULL_KILL` scaled by the starting
/// walkers a dish cell against `REF_DENSITY`, to the power `DENSITY_LAW`.
pub fn full_kill<W: World>(g: &Game<W>) -> f32 {
    let (w, h) = g.world.size();
    let sc = g.world.scale().max(1.0);
    let dish_cells = (w as f32 / sc) * (h as f32 / sc);
    FULL_KILL * (g.start_mould as f32 / dish_cells.max(1.0) / REF_DENSITY).max(1e-3).powf(DENSITY_LAW)
}

/// Where the listener stands: the camera's view, fine cells (the world's
/// own units, which `Shot` and `Soldier` use).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ear {
    pub centre: (f32, f32),
    /// Half the view's width and height.
    pub half: (f32, f32),
}

impl Ear {
    /// Pan (-1 left of the view ..= 1 right) and gain (1 on screen, falling
    /// as 1 / (1 + 2 x views away) off it) of a sound at `p`.
    pub fn place(&self, p: (f32, f32)) -> (f32, f32) {
        let (hx, hy) = (self.half.0.max(1.0), self.half.1.max(1.0));
        let (dx, dy) = ((p.0 - self.centre.0) / hx, (p.1 - self.centre.1) / hy);
        let out = (dx.abs() - 1.0).max(0.0).hypot((dy.abs() - 1.0).max(0.0));
        (dx.clamp(-1.0, 1.0), 1.0 / (1.0 + 2.0 * out))
    }
}

/// Counters for "did it fire", printed beside every recording.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldStats {
    pub hears: u64,
    pub rebaselines: u64,
    pub shots: u64,
    pub voiced: u64,
    pub impacts: u64,
    pub kills: u64,
    /// Voiced impacts by weight: under a third, to two thirds, over.
    pub light: u64,
    pub mid: u64,
    pub heavy: u64,
    pub beats: u64,
    pub hurts: u64,
    pub fallen: u32,
    pub banked: u32,
    pub bought: u32,
    pub walls: u32,
    pub dry: u32,
    pub cores: u32,
    pub endings: u32,
    /// The last frame's inputs: the pulse's danger 0 ..= 1, walkers
    /// touching soldiers, kills and food stolen a second.
    pub danger: f32,
    pub touching: u32,
    pub kills_per_sec: f32,
    pub stolen_per_sec: f32,
}

pub struct WorldListener {
    ticks: u64,
    full: f32,
    pending: Vec<Shot>,
    hp: Vec<f32>,
    alive: Vec<bool>,
    hurt: Vec<f32>,
    hurt_wait: Vec<f32>,
    kill_rate: f32,
    kill_at: (f32, f32),
    steal_rate: f32,
    harvested: f32,
    bank_wait: f32,
    banked: f32,
    stolen: f32,
    walls: u32,
    dry: Vec<bool>,
    core_down: bool,
    ended: bool,
    beat: f32,
    pub stats: WorldStats,
}

impl WorldListener {
    pub fn new<W: World>(g: &Game<W>) -> Self {
        let mut l = WorldListener {
            ticks: 0,
            full: FULL_KILL,
            pending: Vec::new(),
            hp: Vec::new(),
            alive: Vec::new(),
            hurt: Vec::new(),
            hurt_wait: Vec::new(),
            kill_rate: 0.0,
            kill_at: g.larder,
            steal_rate: 0.0,
            harvested: 0.0,
            bank_wait: 0.0,
            banked: 0.0,
            stolen: 0.0,
            walls: 0,
            dry: Vec::new(),
            core_down: false,
            ended: false,
            beat: 0.0,
            stats: WorldStats::default(),
        };
        l.rebaseline(g);
        l.stats.rebaselines = 0;
        l
    }

    /// Take the round as it is now, so a new round (N, a restart) is not
    /// heard as one burst of every tally.
    fn rebaseline<W: World>(&mut self, g: &Game<W>) {
        self.ticks = g.world.ticks();
        self.full = full_kill(g);
        self.pending.clear();
        self.hp = g.soldiers.iter().map(|s| s.hp).collect();
        self.alive = g.soldiers.iter().map(|s| s.alive).collect();
        self.hurt = vec![0.0; g.soldiers.len()];
        self.hurt_wait = vec![0.0; g.soldiers.len()];
        (self.kill_rate, self.steal_rate, self.kill_at) = (0.0, 0.0, g.larder);
        (self.harvested, self.stolen, self.walls) = (g.harvested, g.stolen, g.walls);
        (self.bank_wait, self.banked) = (0.0, 0.0);
        self.dry = g.world.crystals().iter().map(|c| c.1 <= 0.0).collect();
        (self.core_down, self.ended, self.beat) = (g.core_down.is_some(), g.end.is_some(), 0.0);
        self.stats.rebaselines += 1;
    }

    /// The shots of one game step (`Game::last_shots`), called after every
    /// step: the window runs several a drawn frame and each overwrites the
    /// last's shots.
    pub fn shots(&mut self, shots: &[Shot]) {
        self.pending.extend_from_slice(shots);
    }

    /// What should sound after this drawn frame. `dt` is wall-clock seconds
    /// since the last call; `hushed` is paused or the menu up.
    pub fn hear<W: World>(&mut self, g: &Game<W>, ear: Ear, dt: f32, hushed: bool) -> Vec<SoundCmd> {
        let mut out = Vec::with_capacity(16);
        self.stats.hears += 1;
        let ticks = g.world.ticks();
        if ticks < self.ticks || g.soldiers.len() < self.hp.len() || g.harvested < self.harvested || g.walls < self.walls {
            self.rebaseline(g);
        }
        out.push(SoundCmd::Hush(hushed));
        let dt = dt.max(1e-3);
        let k = 1.0 - (-dt / RATE_SMOOTH).exp();

        // --- shots -----------------------------------------------------
        let shots = std::mem::take(&mut self.pending);
        let (mut killed, mut kx, mut ky) = (0u32, 0.0f32, 0.0f32);
        for s in &shots {
            if let (Some(h), true) = (s.hit, s.killed > 0) {
                killed += s.killed;
                kx += h.0 * s.killed as f32;
                ky += h.1 * s.killed as f32;
            }
        }
        self.stats.shots += shots.len() as u64;
        self.stats.kills += killed as u64;
        // The heaviest shots are the ones heard; a stable sort keeps the
        // order the game fired them among equals.
        let mut order: Vec<usize> = (0..shots.len()).collect();
        order.sort_by_key(|&i| std::cmp::Reverse(shots[i].killed));
        let voiced = order.len().min(SHOTS_VOICED);
        // Spread over the frame so a volley is a rattle, not one flam.
        let gap = if voiced > 1 { (dt / voiced as f32).min(0.03) } else { 0.0 };
        for (j, &i) in order.iter().take(voiced).enumerate() {
            let s = shots[i];
            let delay = gap * j as f32;
            let (pan, gain) = ear.place(s.from);
            out.push(SoundCmd::Grain { kind: GrainKind::Crack, pan, gain: 0.4 * gain, delay });
            self.stats.voiced += 1;
            if let Some(h) = s.hit {
                let w = kill_weight(s.killed, self.full);
                let (pan, gain) = ear.place(h);
                // The flight: the farther it struck, the later it lands
                // (a few hundredths of a second at most; the ear hears
                // the crack and the slap as two sounds, from and to).
                let flight = ((h.0 - s.from.0).hypot(h.1 - s.from.1) / ear.half.0.max(1.0) * 0.04).min(0.05);
                let level = if s.killed > 0 { 0.28 + 0.37 * w } else { 0.12 };
                out.push(SoundCmd::Impact { weight: w, pan, gain: level * gain, delay: delay + 0.012 + flight });
                self.stats.impacts += 1;
                match w {
                    w if w < 1.0 / 3.0 => self.stats.light += 1,
                    w if w < 2.0 / 3.0 => self.stats.mid += 1,
                    _ => self.stats.heavy += 1,
                }
            }
        }
        // The kill crackle: a pop a walker killed, where they died.
        self.kill_rate += (killed as f32 / dt - self.kill_rate) * k;
        if killed > 0 {
            self.kill_at = (kx / killed as f32, ky / killed as f32);
        }
        let (kill_pan, kill_gain) = ear.place(self.kill_at);
        let (rate, level) = density(self.kill_rate, 1.0, 60.0, 0.22);
        out.push(SoundCmd::Texture { kind: TextureKind::Starve, rate, level: level * kill_gain, pan: kill_pan, spread: 0.25 });
        self.stats.kills_per_sec = self.kill_rate;

        // --- the mould closing in --------------------------------------
        let reach = g.rules.range * g.world.scale();
        let (mut touching, mut tx, mut ty) = (0u32, 0.0f32, 0.0f32);
        let mut danger = 0.0f32;
        for (i, s) in g.soldiers.iter().enumerate() {
            if !s.alive {
                continue;
            }
            touching += s.touching;
            tx += s.x * s.touching as f32;
            ty += s.y * s.touching as f32;
            let near = if s.touching > 0 {
                1.0
            } else {
                s.aim.map_or(0.0, |a| (1.0 - (a.0 - s.x).hypot(a.1 - s.y) / reach.max(1.0)).clamp(0.0, 1.0))
            };
            danger = danger.max(near * ear.place((s.x, s.y)).1);
            // Hurt: health lost, in chunks.
            if let (Some(hp), Some(acc), Some(wait)) = (self.hp.get_mut(i), self.hurt.get_mut(i), self.hurt_wait.get_mut(i)) {
                *acc += (*hp - s.hp).max(0.0);
                *hp = s.hp;
                *wait -= dt;
                if *acc >= HURT_EVERY && *wait <= 0.0 {
                    let (pan, gain) = ear.place((s.x, s.y));
                    out.push(SoundCmd::Grain { kind: GrainKind::Hurt, pan, gain: 0.35 * gain, delay: 0.0 });
                    self.stats.hurts += 1;
                    (*acc, *wait) = (0.0, HURT_GAP);
                }
            }
        }
        let touch_at = if touching > 0 { (tx / touching as f32, ty / touching as f32) } else { ear.centre };
        let (touch_pan, touch_gain) = ear.place(touch_at);
        let (rate, level) = density(touching as f32, 2.0, 20.0, 0.25);
        out.push(SoundCmd::Texture { kind: TextureKind::Creep, rate, level: level * touch_gain, pan: touch_pan, spread: 0.3 });
        self.stats.touching = touching;
        self.stats.danger = danger;
        // The pulse is the squad's, so it sits in the middle; the beat's
        // second thump ("dub") is quieter and a fifth of a second after.
        if danger > PULSE_FLOOR && !hushed && g.end.is_none() {
            self.beat += dt * (PULSE_SLOW + (PULSE_FAST - PULSE_SLOW) * danger) / 60.0;
            if self.beat >= 1.0 {
                self.beat -= self.beat.floor();
                let gain = 0.15 + 0.4 * danger;
                out.push(SoundCmd::Grain { kind: GrainKind::Heart, pan: 0.0, gain, delay: 0.0 });
                out.push(SoundCmd::Grain { kind: GrainKind::Heart, pan: 0.0, gain: gain * 0.6, delay: 0.19 });
                self.stats.beats += 1;
            }
        } else {
            // Ready to beat at once when the mould next comes into sight.
            self.beat = 0.999;
        }

        // --- soldiers fallen and bought --------------------------------
        for (i, s) in g.soldiers.iter().enumerate() {
            if i >= self.alive.len() {
                self.alive.push(s.alive);
                self.hp.push(s.hp);
                self.hurt.push(0.0);
                self.hurt_wait.push(0.0);
                let (pan, gain) = ear.place((s.x, s.y));
                out.push(SoundCmd::Grain { kind: GrainKind::Thunk, pan, gain: 0.5 * gain, delay: 0.0 });
                self.stats.bought += 1;
                continue;
            }
            if self.alive[i] && !s.alive {
                let (pan, _) = ear.place((s.x, s.y));
                out.push(SoundCmd::Alert { kind: AlertKind::Fallen, weight: 1.0, pan });
                self.stats.fallen += 1;
            }
            self.alive[i] = s.alive;
        }

        // --- the economy -----------------------------------------------
        let stolen = (g.stolen - self.stolen).max(0.0);
        self.stolen = g.stolen;
        self.steal_rate += (stolen / dt - self.steal_rate) * k;
        // Alerts that tell the player where to look keep most of their level
        // off screen (design review of #96): the larder robbed, a crystal
        // dry. A soldier falling and the core are at full level already.
        let (larder_pan, larder_gain) = ear.place(g.larder);
        let larder_gain = larder_gain.max(ALERT_FLOOR);
        let (rate, level) = density(self.steal_rate, 0.6, 30.0, 0.2);
        out.push(SoundCmd::Texture { kind: TextureKind::Feed, rate, level: level * larder_gain, pan: larder_pan, spread: 0.15 });
        self.stats.stolen_per_sec = self.steal_rate;
        // Food banked: a chime, gathered to one a second, that climbs as the
        // bank fills toward a soldier's price several times over.
        self.banked += (g.harvested - self.harvested).max(0.0);
        self.harvested = g.harvested;
        self.bank_wait -= dt;
        if self.banked > 0.0 && self.bank_wait <= 0.0 {
            let weight = (g.bank / (g.rules.soldier_cost * 4.0)).clamp(0.0, 1.0);
            out.push(SoundCmd::Alert { kind: AlertKind::Joined, weight, pan: larder_pan });
            self.stats.banked += 1;
            (self.banked, self.bank_wait) = (0.0, BANK_GAP);
        }
        if g.walls > self.walls {
            out.push(SoundCmd::Grain { kind: GrainKind::Thunk, pan: 0.0, gain: 0.45, delay: 0.0 });
            out.push(SoundCmd::Burst { kind: GrainKind::Grit, rate: 90.0, secs: 0.15, pan: 0.0, spread: 0.3, gain: 0.25 });
            self.stats.walls += g.walls - self.walls;
        }
        self.walls = g.walls;
        for (i, (c, charge)) in g.world.crystals().into_iter().enumerate() {
            if i < self.dry.len() && !self.dry[i] && charge <= 0.0 {
                let (pan, gain) = ear.place(c);
                out.push(SoundCmd::Grain { kind: GrainKind::Knock, pan, gain: 0.4 * gain.max(ALERT_FLOOR), delay: 0.0 });
                self.stats.dry += 1;
                self.dry[i] = true;
            }
        }
        if g.core_down.is_some() && !self.core_down {
            let (pan, _) = ear.place(g.core);
            out.push(SoundCmd::Grain { kind: GrainKind::Thud, pan, gain: 0.8, delay: 0.0 });
            out.push(SoundCmd::Alert { kind: AlertKind::Bell, weight: 2.0 / 3.0, pan });
            self.stats.cores += 1;
        }
        self.core_down = g.core_down.is_some();
        if let (Some(end), false) = (g.end, self.ended) {
            self.stats.endings += 1;
            out.push(match end {
                End::MouldWiped(_) => SoundCmd::Alert { kind: AlertKind::Bell, weight: 1.0, pan: 0.0 },
                End::Cap(share) => SoundCmd::Alert { kind: AlertKind::Bell, weight: share.clamp(0.0, 1.0), pan: 0.0 },
                End::SquadWiped(_) => SoundCmd::Alert { kind: AlertKind::Died, weight: 1.0, pan: 0.0 },
            });
        }
        self.ended = g.end.is_some();
        self.ticks = ticks;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::larder_world::{race_world, Cmd, Order, Rules};

    fn kill_weight_ref(killed: u32) -> f32 {
        kill_weight(killed, FULL_KILL)
    }

    #[test]
    fn a_denser_world_needs_more_kills_for_a_full_shot() {
        let g = small(1);
        let base = full_kill(&g);
        let mut dense = small(1);
        dense.start_mould *= 10;
        let ratio = full_kill(&dense) / base;
        assert!((ratio - 10f32.powf(DENSITY_LAW)).abs() < 1e-3, "{base} vs {}", full_kill(&dense));
    }

    fn small(seed: u64) -> Game<crate::mould::big::BigDish> {
        Game::new(race_world(seed, 1, 1_000), Rules::default(), seed)
    }

    fn whole(g: &Game<crate::mould::big::BigDish>) -> Ear {
        let (w, h) = g.world.size();
        Ear { centre: (w as f32 / 2.0, h as f32 / 2.0), half: (w as f32 / 2.0, h as f32 / 2.0) }
    }

    #[test]
    fn a_heavier_kill_weighs_more_and_a_full_vein_is_full() {
        assert_eq!(kill_weight_ref(0), 0.0);
        assert!(kill_weight_ref(1) > 0.1 && kill_weight_ref(1) < kill_weight_ref(8));
        assert!(kill_weight_ref(37) > 0.6 && kill_weight_ref(37) < 0.75, "the rush's median shot sits in the upper middle");
        assert!(kill_weight_ref(8) < kill_weight_ref(20));
        assert_eq!(kill_weight_ref(FULL_KILL as u32), 1.0);
        assert_eq!(kill_weight_ref(500), 1.0);
    }

    #[test]
    fn the_ear_pans_on_screen_and_fades_off_it() {
        let ear = Ear { centre: (100.0, 100.0), half: (50.0, 30.0) };
        assert_eq!(ear.place((100.0, 100.0)), (0.0, 1.0));
        assert_eq!(ear.place((150.0, 129.0)), (1.0, 1.0));
        assert_eq!(ear.place((50.0, 100.0)).0, -1.0);
        let (_, one) = ear.place((200.0, 100.0));
        let (_, two) = ear.place((250.0, 100.0));
        assert!((one - 1.0 / 3.0).abs() < 1e-6, "a view off: {one}");
        assert!(two < one, "farther is quieter");
    }

    /// The point of the read-only listener: a round heard every frame ends
    /// exactly where an unheard one does, and the listener did hear it.
    #[test]
    fn listening_never_changes_the_round() {
        let play = |listen: bool| {
            let mut g = small(3);
            let mut l = WorldListener::new(&g);
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Order(k, Order::Attack));
            }
            for _ in 0..(60 * g.rules.tps) {
                g.step();
                if listen {
                    l.shots(&g.last_shots);
                    l.hear(&g, whole(&g), 1.0 / 15.0, false);
                }
                if g.end.is_some() {
                    break;
                }
            }
            (g.world.hash(), g.shots, g.killed, l.stats)
        };
        let (heard, shots, killed, stats) = play(true);
        let (unheard, _, _, _) = play(false);
        assert_eq!(heard, unheard);
        // The positive controls: shots were fired and heard, and the
        // listener counted every one the game counted.
        assert!(shots > 0 && killed > 0, "shots {shots} killed {killed}");
        assert_eq!(stats.shots, shots as u64, "{stats:?}");
        assert_eq!(stats.kills, killed as u64, "{stats:?}");
        assert!(stats.impacts > 0 && stats.beats > 0, "{stats:?}");
    }

    #[test]
    fn a_shot_that_kills_more_sounds_heavier() {
        let g = small(1);
        let mut l = WorldListener::new(&g);
        let ear = whole(&g);
        let shot = |killed| Shot { from: (10.0, 10.0), to: (20.0, 10.0), hit: Some((18.0, 10.0)), killed };
        let impact = |cmds: Vec<SoundCmd>| {
            cmds.into_iter().find_map(|c| match c {
                SoundCmd::Impact { weight, gain, .. } => Some((weight, gain)),
                _ => None,
            })
        };
        l.shots(&[shot(1)]);
        let (w1, g1) = impact(l.hear(&g, ear, 1.0 / 60.0, false)).unwrap();
        l.shots(&[shot(25)]);
        let (w2, g2) = impact(l.hear(&g, ear, 1.0 / 60.0, false)).unwrap();
        assert!(w2 > w1 * 2.0 && g2 > g1, "1 kill {w1} {g1}, 25 kills {w2} {g2}");
        // A shot a wall stopped cracks but strikes nothing.
        l.shots(&[Shot { hit: None, killed: 0, ..shot(0) }]);
        let cmds = l.hear(&g, ear, 1.0 / 60.0, false);
        assert!(impact(cmds.clone()).is_none());
        assert!(cmds.iter().any(|c| matches!(c, SoundCmd::Grain { kind: GrainKind::Crack, .. })));
    }

    #[test]
    fn a_volley_is_capped_and_the_heaviest_shots_are_the_ones_heard() {
        let g = small(1);
        let mut l = WorldListener::new(&g);
        let ear = whole(&g);
        let shots: Vec<Shot> = (0..20).map(|k| Shot { from: (10.0, 10.0), to: (20.0, 10.0), hit: Some((18.0, 10.0)), killed: k }).collect();
        l.shots(&shots);
        let cmds = l.hear(&g, ear, 1.0 / 60.0, false);
        let cracks = cmds.iter().filter(|c| matches!(c, SoundCmd::Grain { kind: GrainKind::Crack, .. })).count();
        assert_eq!(cracks, SHOTS_VOICED);
        let least = cmds.iter().filter_map(|c| if let SoundCmd::Impact { weight, .. } = c { Some(*weight) } else { None }).fold(1.0f32, f32::min);
        assert!(least >= kill_weight_ref(20 - SHOTS_VOICED as u32), "a light shot was voiced over a heavy one: {least}");
        assert_eq!(l.stats.kills, (0..20).sum::<u64>(), "every kill counts in the crackle");
    }

    #[test]
    fn a_new_round_is_a_rebaseline_not_a_burst() {
        let mut g = small(2);
        for k in 0..g.soldiers.len() {
            g.apply(Cmd::Order(k, Order::Attack));
        }
        for _ in 0..(40 * g.rules.tps) {
            g.step();
        }
        let mut l = WorldListener::new(&g);
        let fresh = small(4);
        let cmds = l.hear(&fresh, whole(&fresh), 1.0 / 60.0, false);
        assert_eq!(l.stats.rebaselines, 1);
        assert!(!cmds.iter().any(|c| matches!(c, SoundCmd::Alert { .. } | SoundCmd::Burst { .. } | SoundCmd::Impact { .. })), "{cmds:?}");
    }
}
