//! **Sound for the game** (owner, 2026-11-10: "yes to sound";
//! `Reports/mould-sound-2026-11-10.md`). The window was silent; this is
//! Pixel_Physics's sound architecture (`src/audio/` there) with the mould's
//! own vocabulary.
//!
//! Three pieces, and the split is the point:
//!
//! - [`listen`] reads the dish and decides what should be heard. It takes
//!   `&Dish` and nothing else, so the borrow checker proves it cannot change
//!   a cell, a tally or a random draw: sound on and sound off play the same
//!   game, bit for bit (`Dish::state_hash`; the guard is
//!   `listen::tests::listening_never_changes_the_game`). It runs once per drawn
//!   frame and emits [`SoundCmd`]s.
//! - [`synth`] turns commands into samples. A pure function of its seed and
//!   command stream, so the offline render (`examples/mouldsound.rs`) is the
//!   same code the window plays.
//! - `device` is the only file that touches an audio API (`cpal`) and the
//!   only platform-gated one: macOS and Windows always, Linux only with
//!   `--features audio-alsa` (`Cargo.toml` has why).
//!
//! **Everything heard is the simulation** (the owner's rule for the
//! picture, held for sound too): every texture's rate and level is a count
//! read off the dish this frame, and every one-shot is a tally that moved.
//! Nothing is scored or scripted for the ear alone.
//! The one exception is [`score`] (2026-11-22), the music bed and the
//! planet's air, asked for by the owner because the game was gunfire over
//! silence; even it takes its tension from the game.
//!
//! **Never one voice per walker.** Two thousand walkers become a handful of
//! textures whose grain rate follows the count up to a ceiling and whose
//! level follows `count^0.4` (`listen::density`).

pub mod listen;
pub mod pieces;
pub mod score;
pub mod synth;
pub mod world;

#[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
mod device;

/// Whether this build can open a sound device at all. `false` on a default
/// Linux build (CI, cloud sessions), where the device file is not compiled
/// and the window runs silent, exactly as before sound existed.
pub const DEVICE_COMPILED: bool = cfg!(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")));

/// **The speaker, or nothing.** What the window holds, so `bin/mould.rs`
/// needs no `cfg` of its own: without a device this is an empty shell whose
/// `send` drops the commands.
pub struct Output {
    #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
    device: device::Device,
    /// **Reopening** (2026-11-20, the owner: "the sound was working and
    /// partway through cut out"). A stream that errored or stopped asking
    /// for samples is replaced by a fresh one on the default device; the
    /// seed for its bank, the last master volume sent (so the new bank is
    /// at the player's volume), when to try next, and the wait after a
    /// failed try (2 s, doubling to 30 s, so a machine with no device left
    /// is not asked every frame).
    #[allow(dead_code)]
    seed: u64,
    #[allow(dead_code)]
    master: Option<SoundCmd>,
    #[allow(dead_code)]
    retry: (std::time::Instant, f32),
    /// Times the device was reopened.
    pub reopened: u32,
}

impl Output {
    /// Open the default output, or say why not: no device compiled, no
    /// device present (a VM, a container), or the host refused the stream.
    /// The caller runs silent on any `Err`.
    pub fn open(seed: u64) -> Result<Output, String> {
        #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
        {
            device::Device::open(seed).map(|device| Output { device, seed, master: None, retry: (std::time::Instant::now(), 2.0), reopened: 0 })
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa"))))]
        {
            let _ = seed;
            Err("this build has no sound device (a Linux build without --features audio-alsa)".to_string())
        }
    }

    pub fn send(&mut self, cmds: impl IntoIterator<Item = SoundCmd>) {
        #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
        {
            if !self.device.healthy() && self.retry.0 <= std::time::Instant::now() {
                match device::Device::open(self.seed.wrapping_add(self.reopened as u64 + 1)) {
                    Ok(mut d) => {
                        d.dropped = self.device.dropped;
                        d.send(self.master);
                        self.device = d;
                        self.reopened += 1;
                        self.retry = (std::time::Instant::now() + std::time::Duration::from_secs(2), 2.0);
                        eprintln!("sound: reopened the output device ({} time(s))", self.reopened);
                    }
                    Err(e) => {
                        eprintln!("sound: could not reopen the output device ({e}); trying again in {:.0} s", self.retry.1);
                        self.retry = (std::time::Instant::now() + std::time::Duration::from_secs_f32(self.retry.1), (self.retry.1 * 2.0).min(30.0));
                    }
                }
            }
            let master = &mut self.master;
            self.device.send(cmds.into_iter().inspect(|c| {
                if let SoundCmd::Master(_) = c {
                    *master = Some(*c);
                }
            }));
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa"))))]
        drop(cmds.into_iter());
    }

    /// **The play log's `sound` line** (2026-11-22, sound still cutting
    /// out): how the speaker has done so far, so a cut-out in his play can
    /// be told apart from a quiet stretch.
    ///
    /// ```text
    /// sound prio=proaudio calls=6012 late=0 gap=10.7 render=0.9 soft=0 dropped=0 reopened=0
    /// ```
    ///
    /// `prio` the callback thread's priority (`device::Health::priority`),
    /// `late` callbacks over 30 ms after the last (each one likely a gap
    /// heard), `gap` and `render` the most ms since the last line, `soft`
    /// stream errors played through, `dropped` commands lost to a full
    /// channel, `reopened` streams replaced. Empty without a device.
    pub fn health_line(&mut self) -> String {
        #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
        {
            use std::sync::atomic::Ordering::Relaxed;
            let h = &self.device.health;
            let (gap, render) = h.take_most();
            format!(
                "sound prio={} calls={} late={} gap={gap:.1} render={render:.1} soft={} dropped={} reopened={}\n",
                h.priority_name(),
                h.calls.load(Relaxed),
                h.late.load(Relaxed),
                h.soft_errors.load(Relaxed),
                self.device.dropped,
                self.reopened
            )
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa"))))]
        String::new()
    }

    /// Commands dropped because the device fell behind; should stay 0.
    pub fn dropped(&self) -> u64 {
        #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
        return self.device.dropped;
        #[cfg(not(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa"))))]
        0
    }
}

/// Kinds of continuous activity, one granular texture each.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureKind {
    /// The mould growing: walkers born per second.
    Creep = 0,
    /// The mould eating: walkers on food piles (its own and the wild
    /// mould's).
    Feed = 1,
    /// The mould dying of hunger: walkers starved per second.
    Starve = 2,
    /// Larder World: a cut-off piece withering, walkers it loses a second
    /// (`world::WorldListener`, from the mould grid).
    Wither = 3,
    /// Larder World: walkers the squad kills a second, a wet pop each
    /// (2026-11-20; until then the starving crackle, which made a fight
    /// sound like radio static).
    Kill = 4,
}

impl TextureKind {
    pub const COUNT: usize = 5;
    pub const ALL: [TextureKind; 5] = [TextureKind::Creep, TextureKind::Feed, TextureKind::Starve, TextureKind::Wither, TextureKind::Kill];

    /// The grain one of this texture's events is made of.
    pub fn grain(self) -> GrainKind {
        match self {
            TextureKind::Creep => GrainKind::Squelch,
            TextureKind::Feed => GrainKind::Slurp,
            TextureKind::Starve => GrainKind::Crackle,
            TextureKind::Wither => GrainKind::Wilt,
            TextureKind::Kill => GrainKind::Pop,
        }
    }
}

/// One short procedural sound, recipe in `synth::recipe`, jittered every
/// time so no two are identical.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrainKind {
    /// Growth: a low wet tick.
    Squelch,
    /// Feeding: a rough mid scrape.
    Slurp,
    /// Starving: a dry pop.
    Crackle,
    /// Oats set down.
    Thunk,
    /// Salt laid; a fight at the seam.
    Grit,
    /// The knife; a food pile turning up.
    Knock,
    /// Mould burned under the lamp.
    Sizzle,
    /// A drop of water.
    Drip,
    /// The storm being aimed.
    Whoosh,
    /// The storm striking.
    Thud,
    /// Larder World: a soldier's shot leaving the gun (`world`).
    Crack,
    /// Larder World: a shot striking slime, played by `SoundCmd::Impact`
    /// with the walkers it killed as its weight (heavier, lower, longer).
    Splat,
    /// Larder World: the squad's pulse, quickening as the mould closes in.
    Heart,
    /// Larder World: a soldier hurt by the walkers on it.
    Hurt,
    /// Larder World: a bullet flying past (`Game::bullets`).
    Zip,
    /// Larder World: a cut-off piece's tell, as it leans toward a soldier
    /// before it charges.
    Snarl,
    /// Larder World: the mould torn in two (a new cut-off piece).
    Tear,
    /// Larder World: a withering piece, a soft dry fizz.
    Wilt,
    /// Larder World: a gun's low body, under its crack (`SoundCmd::Gun`).
    Punch,
    /// Larder World: a gun's report, the tail that rolls off across the
    /// planet (`SoundCmd::Gun`).
    Report,
    /// Larder World: a gun's action, the bolt or pump after a heavy shot
    /// (`SoundCmd::Gun`).
    Action,
    /// Larder World: a walker killed, a small wet pop (`TextureKind::Kill`).
    Pop,
    /// Larder World: a gas round's cloud let out, a long falling hiss.
    Hiss,
    /// Larder World: the slime gathering itself (a raid or a surge
    /// starting, a mini-boss forming), a low rough groan that rises.
    Swell,
    /// Larder World: a building set down, a metal clank.
    Clank,
    /// The menus and the HUD answering a click: a small soft tick.
    Tick,
}

impl GrainKind {
    pub const ALL: [GrainKind; 26] = [
        GrainKind::Squelch,
        GrainKind::Slurp,
        GrainKind::Crackle,
        GrainKind::Thunk,
        GrainKind::Grit,
        GrainKind::Knock,
        GrainKind::Sizzle,
        GrainKind::Drip,
        GrainKind::Whoosh,
        GrainKind::Thud,
        GrainKind::Crack,
        GrainKind::Splat,
        GrainKind::Heart,
        GrainKind::Hurt,
        GrainKind::Zip,
        GrainKind::Snarl,
        GrainKind::Tear,
        GrainKind::Wilt,
        GrainKind::Punch,
        GrainKind::Report,
        GrainKind::Action,
        GrainKind::Pop,
        GrainKind::Hiss,
        GrainKind::Swell,
        GrainKind::Clank,
        GrainKind::Tick,
    ];
}

/// **A gun's voice** (`SoundCmd::Gun`, 2026-11-20): each kind of gun its
/// own report, so the squad's gun is heard for what it is and a turret is
/// told from a soldier. Until then every shot was the same 4 ms tick
/// (`GrainKind::Crack` alone).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GunVoice {
    /// The starting gun: a firm crack, a mid punch, a short report.
    Rifle,
    /// Fast and short: lighter, higher, a short tail, fired twice as often.
    Carbine,
    /// Slow and far: a deep boom, a long rolling report and a bolt after.
    Long,
    /// A fan of bullets: one wide blast a shot (not one a pellet) and a
    /// pump after.
    Scatter,
    /// A turret: a metallic, ringing, quieter shot, set under the squad's.
    Turret,
}

impl GunVoice {
    pub const ALL: [GunVoice; 5] = [GunVoice::Rifle, GunVoice::Carbine, GunVoice::Long, GunVoice::Scatter, GunVoice::Turret];
}

/// The moments that matter, as tuned tones.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertKind {
    /// Two pieces of mould joined on food (the join flash, `Dish::snaps`).
    /// `weight` climbs with how many joins the dish has had, and so does
    /// the note.
    Joined,
    /// The dish is over with mould alive. `weight` is the share of the
    /// dish's piles still joined after the storm.
    Bell,
    /// The last walker died.
    Died,
    /// Larder World: one soldier fell (two low notes, short of `Died`).
    Fallen,
    /// Larder World: a siren calling (`Game::sirens`), a two-tone wail.
    Siren,
    /// Larder World: a soldier taking shop points from ore or a core, a
    /// short bright coin tick (2026-11-21). `weight` is the share of the
    /// drop taken so far, and the note climbs with it.
    Coin,
    /// Larder World: a drop the squad emptied, its points in: a quick high
    /// arpeggio, to the octave at `weight` 1 (a core), stopping short below
    /// (ore). Higher and brighter than the planet's end `Bell`.
    Cashed,
}

/// What the game thread tells the voice bank. **Textures carry absolute
/// targets**, re-sent every drawn frame, so a dropped command costs one
/// frame of staleness, never a stuck sound. `pan` is -1 (left of the dish)
/// ..= 1 (right); `delay` is seconds of wall clock.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SoundCmd {
    Texture { kind: TextureKind, rate: f32, level: f32, pan: f32, spread: f32 },
    Grain { kind: GrainKind, pan: f32, gain: f32, delay: f32 },
    /// A decaying run of grains: `rate` per second at the start, falling
    /// with time constant `secs`.
    Burst { kind: GrainKind, rate: f32, secs: f32, pan: f32, spread: f32, gain: f32 },
    Alert { kind: AlertKind, weight: f32, pan: f32 },
    /// A `GrainKind::Splat` shaped by `weight` (0 ..= 1): at 0 a light wet
    /// tap, at 1 a heavy low slap about twice as long. Larder World's shot
    /// sends the share of a big kill it made (`world::kill_weight`).
    /// `far` (0 on screen ..= 1 far off it) dulls it (`world::Ear::far`).
    Impact { weight: f32, pan: f32, gain: f32, delay: f32, far: f32 },
    /// Larder World: a gunshot, several layers at once (`synth::gun`):
    /// `far` dulls the crack and leaves the report, as distance does.
    Gun { voice: GunVoice, pan: f32, gain: f32, delay: f32, far: f32 },
    /// The lamp's hum, where the lamp stands.
    Lamp { on: bool, pan: f32 },
    /// Larder World: **the mould's own voice**, one breathing drone for the
    /// whole body (`world::WorldListener`; design review of #96, the
    /// owner's "not one cohesive organism"). Absolute targets, re-sent
    /// every frame like a texture: `level` 0 ..= 1, `pitch` Hz of its sub
    /// tone, `breath` swells a second, `rough` 0 ..= 1 (strain: hunger),
    /// `bright` 0 ..= 1 (its noise band opens while it eats).
    ///
    /// `slot` 0 is the main body; 1 ..= 3 are cut-off pieces, each a
    /// fainter, higher copy (`synth::BODY_VOICES`).
    Body { slot: u8, level: f32, pitch: f32, breath: f32, rough: f32, bright: f32, pan: f32 },
    /// Fade every texture out (paused, a card up) or back in. One-shots
    /// still sound, so a tool used while paused still answers.
    Hush(bool),
    /// The master level, 0 ..= 1; the applied gain is its square.
    Master(f32),
    /// **The score** (`score`, 2026-11-22): the music bed and the planet's
    /// air. Absolute targets, re-sent every drawn frame: the planet's
    /// `mood`, `tension` 0 ..= 1 (how near the slime is), `level` the
    /// Options page's Music (1 = 100%, 0 = off), and `hushed` (paused: it
    /// plays on, quieter).
    Score { mood: Mood, tension: f32, level: f32, hushed: bool },
}

/// **A planet's music and air** (`score::MOODS`): one per biome of the
/// run's planets (`levelgen::Biome`), and one for the menus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mood {
    Menu = 0,
    Meadow = 1,
    Barrens = 2,
    Fen = 3,
    Valley = 4,
    Jungle = 5,
    Highlands = 6,
    Caves = 7,
    Core = 8,
}

impl Mood {
    pub const COUNT: usize = 9;
    pub const ALL: [Mood; 9] = [Mood::Menu, Mood::Meadow, Mood::Barrens, Mood::Fen, Mood::Valley, Mood::Jungle, Mood::Highlands, Mood::Caves, Mood::Core];

    /// A planet's mood by its biome.
    pub fn of(b: crate::mould::levelgen::Biome) -> Mood {
        use crate::mould::levelgen::Biome;
        match b {
            Biome::Meadow => Mood::Meadow,
            Biome::Barrens => Mood::Barrens,
            Biome::Fen => Mood::Fen,
            Biome::Valley => Mood::Valley,
            Biome::Jungle => Mood::Jungle,
            Biome::Highlands => Mood::Highlands,
            Biome::Caves => Mood::Caves,
            Biome::Core => Mood::Core,
        }
    }

    /// By name, for `mood=` on the command line: `menu` or a biome's name
    /// (`meadow`, `barrens`, `fen`, `valley`, `jungle`, `highlands`,
    /// `caves`, `core`).
    pub fn from_name(s: &str) -> Option<Mood> {
        Mood::ALL.into_iter().find(|m| format!("{m:?}").eq_ignore_ascii_case(s))
    }
}
