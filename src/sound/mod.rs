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
//!
//! **Never one voice per walker.** Two thousand walkers become a handful of
//! textures whose grain rate follows the count up to a ceiling and whose
//! level follows `count^0.4` (`listen::density`).

pub mod listen;
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
}

impl Output {
    /// Open the default output, or say why not: no device compiled, no
    /// device present (a VM, a container), or the host refused the stream.
    /// The caller runs silent on any `Err`.
    pub fn open(seed: u64) -> Result<Output, String> {
        #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
        {
            device::Device::open(seed).map(|device| Output { device })
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa"))))]
        {
            let _ = seed;
            Err("this build has no sound device (a Linux build without --features audio-alsa)".to_string())
        }
    }

    pub fn send(&mut self, cmds: impl IntoIterator<Item = SoundCmd>) {
        #[cfg(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa")))]
        self.device.send(cmds);
        #[cfg(not(any(target_os = "macos", target_os = "windows", all(target_os = "linux", feature = "audio-alsa"))))]
        drop(cmds.into_iter());
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
}

impl TextureKind {
    pub const COUNT: usize = 3;
    pub const ALL: [TextureKind; 3] = [TextureKind::Creep, TextureKind::Feed, TextureKind::Starve];

    /// The grain one of this texture's events is made of.
    pub fn grain(self) -> GrainKind {
        match self {
            TextureKind::Creep => GrainKind::Squelch,
            TextureKind::Feed => GrainKind::Slurp,
            TextureKind::Starve => GrainKind::Crackle,
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
}

impl GrainKind {
    pub const ALL: [GrainKind; 14] = [
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
    ];
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
    Impact { weight: f32, pan: f32, gain: f32, delay: f32 },
    /// The lamp's hum, where the lamp stands.
    Lamp { on: bool, pan: f32 },
    /// Larder World: **the mould's own voice**, one breathing drone for the
    /// whole body (`world::WorldListener`; design review of #96, the
    /// owner's "not one cohesive organism"). Absolute targets, re-sent
    /// every frame like a texture: `level` 0 ..= 1, `pitch` Hz of its sub
    /// tone, `breath` swells a second, `rough` 0 ..= 1 (strain: hunger),
    /// `bright` 0 ..= 1 (its noise band opens while it eats).
    Body { level: f32, pitch: f32, breath: f32, rough: f32, bright: f32, pan: f32 },
    /// Fade every texture out (paused, a card up) or back in. One-shots
    /// still sound, so a tool used while paused still answers.
    Hush(bool),
    /// The master level, 0 ..= 1; the applied gain is its square.
    Master(f32),
}
