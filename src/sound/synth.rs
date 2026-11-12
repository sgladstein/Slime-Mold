//! **The voice bank**: commands in, stereo samples out.
//!
//! Ported from Pixel_Physics's `src/audio/synth.rs` (2026-11-10, owner: "yes
//! to sound"; `Reports/mould-sound-2026-11-10.md`). The engine is the same:
//! a fixed pool of short procedural grains (filtered noise plus a sine, each
//! with its own decay), one granular texture per kind of activity spawning
//! grains Poisson-wise at a smoothed rate, a few bursts, a few tones for the
//! moments that matter, a room tone so a quiet dish never sounds broken, a
//! small reverb and a soft limiter. What changed is the vocabulary: the
//! grains and textures are the mould's (`super::GrainKind`), the three
//! Pixel_Physics styles are gone (one sound until the owner asks for a
//! choice), and the room tone follows the lamp.
//!
//! It runs on the audio thread in the window and on the main thread in the
//! offline render (`examples/mouldsound.rs`), and must sound identical in
//! both, so it is a pure function of `(seed, sample rate, command stream)`:
//! no clock, no allocation after [`VoiceBank::new`], no locks. It never
//! draws from the dish's random stream, so sound cannot change a game
//! (`Dish::state_hash` is the check).

use super::{AlertKind, GrainKind, SoundCmd, TextureKind};

/// Grains that can sound at once, across every texture, burst and verb.
/// Over this the quietest playing grain is stolen.
pub const GRAIN_POOL: usize = 64;
/// The most grains of one kind fired by *verbs* that may sound together;
/// past it the oldest of that kind is replaced.
pub const PER_KIND_CAP: usize = 4;
const BURSTS: usize = 6;
const TONES: usize = 8;
/// The first-run master level: well below full, so the first launch is
/// never loud. S in the window steps it.
pub const DEFAULT_MASTER: f32 = 0.5;
/// Room tone with the lamp off: a little low air, about -40 dB under a
/// full-scale grain. Enough that a still dish reads as quiet, not broken.
const ROOM_LEVEL: f32 = 0.010;
/// The lamp's hum on top of the room while it is lit, as a multiple of the
/// room level. The lamp is a lever on the model (it pushes trail and
/// walkers), so it is heard where it stands (`SoundCmd::Lamp`).
const LAMP_HUM: f32 = 4.0;
/// Make-up gain after the master, so `DEFAULT_MASTER` (whose square is the
/// applied gain, 0.25) plays the mix near unity. Pixel_Physics measured
/// -41 dB RMS for a busy colony without it: inaudible on a phone.
const MAKEUP: f32 = 4.0;
/// Highest grain rate any texture plays, per second. Past it grains overlap
/// and the ear stops hearing them as separate, so only level grows
/// (`super::listen::density`).
pub const TEXTURE_RATE_CAP: f32 = 220.0;

/// A small, fast, deterministic noise source: the voice bank's own, never
/// the dish's.
#[derive(Clone)]
struct Xs(u64);

impl Xs {
    #[inline]
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    /// Uniform in 0 ..= 1.
    #[inline]
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
    #[inline]
    fn bipolar(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }
    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.unit()
    }
    /// Roughly normal (sum of three uniforms), mean 0.
    fn gauss(&mut self) -> f32 {
        self.bipolar() + self.bipolar() + self.bipolar()
    }
}

/// RBJ constant-peak band-pass.
#[derive(Clone, Copy, Default)]
struct Biquad {
    b0: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    /// Divides out the filter's noise bandwidth so a recipe's `noise` level
    /// means the same loudness whatever its centre and Q.
    norm: f32,
}

impl Biquad {
    fn set(&mut self, f0: f32, q: f32, sr: f32) {
        let f0 = f0.clamp(40.0, sr * 0.45);
        let w0 = std::f32::consts::TAU * f0 / sr;
        let alpha = w0.sin() / (2.0 * q);
        let a0 = 1.0 + alpha;
        self.b0 = alpha / a0;
        self.b2 = -alpha / a0;
        self.a1 = -2.0 * w0.cos() / a0;
        self.a2 = (1.0 - alpha) / a0;
        self.norm = 1.0 / (std::f32::consts::PI * f0 / (q * sr)).sqrt().max(1e-3);
    }
    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b2 * self.x2 - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y * self.norm
    }
}

/// One grain recipe: a noise band and a sine, each with its own decay.
#[derive(Clone, Copy, Debug, Default)]
pub struct Spec {
    pub len: f32,
    pub attack: f32,
    pub noise: f32,
    pub noise_tau: f32,
    pub band: f32,
    pub q: f32,
    /// Band centre at the end of the grain, as a multiple of `band`.
    pub band_end: f32,
    /// Amplitude modulation depth by slow random noise: a scrape.
    pub rough: f32,
    pub sine: f32,
    pub sine_tau: f32,
    pub freq: f32,
    /// Frequency at the end of the grain as a multiple of `freq`.
    pub freq_end: f32,
    /// Extra pitch at the onset that falls away in ~20 ms: an impact.
    pub drop: f32,
}

/// **The recipes.** First guesses, to be tuned by ear against the offline
/// renders on the listening page; jitter is drawn here so two grains of one
/// kind are never identical (the machine-gun effect).
fn recipe(kind: GrainKind, rng: &mut Xs) -> Spec {
    let base = Spec { band_end: 1.0, freq_end: 1.0, q: 1.4, ..Spec::default() };
    match kind {
        // A wet tick as the mould pushes out: low, soft, a falling sine
        // under a dull noise band. Lower and longer than an ant's step.
        GrainKind::Squelch => {
            let f = rng.range(180.0, 420.0);
            Spec {
                len: rng.range(0.025, 0.05),
                attack: 0.002,
                noise: 0.25,
                noise_tau: 0.006,
                band: rng.range(500.0, 1100.0),
                q: 1.1,
                sine: 0.35,
                sine_tau: 0.012,
                freq: f,
                freq_end: rng.range(0.55, 0.8),
                ..base
            }
        }
        // Feeding on a pile: a rough, mid scrape, like Pixel_Physics's dig.
        GrainKind::Slurp => Spec {
            len: rng.range(0.03, 0.06),
            attack: 0.004,
            noise: 0.35,
            noise_tau: rng.range(0.01, 0.02),
            band: rng.range(700.0, 1400.0),
            q: 0.8,
            band_end: 0.7,
            rough: 0.8,
            ..base
        },
        // Starving mould: a dry pop, its loudness drawn skewed so a few
        // stand out.
        GrainKind::Crackle => Spec {
            len: 0.012,
            attack: 0.0002,
            noise: 0.35 * (1.0 + 2.0 * rng.unit() * rng.unit()),
            noise_tau: 0.0012,
            band: rng.range(1800.0, 4500.0),
            q: 1.1,
            ..base
        },
        // Oats set down: a soft low thunk.
        GrainKind::Thunk => Spec {
            len: 0.12,
            attack: 0.001,
            noise: 0.25,
            noise_tau: 0.01,
            band: 600.0,
            q: 0.9,
            sine: 0.8,
            sine_tau: 0.035,
            freq: rng.range(160.0, 185.0),
            drop: 0.6,
            ..base
        },
        // Salt laid: a gritty pour.
        GrainKind::Grit => Spec {
            len: rng.range(0.02, 0.045),
            attack: 0.002,
            noise: 0.4,
            noise_tau: rng.range(0.008, 0.016),
            band: rng.range(2500.0, 4200.0),
            q: 0.7,
            rough: 0.9,
            ..base
        },
        // The knife: a small hard knock.
        GrainKind::Knock => {
            let f = rng.range(250.0, 700.0);
            Spec { len: 0.07, attack: 0.0005, noise: 0.45, noise_tau: 0.008, band: f * 2.0, q: 1.0, sine: 0.5, sine_tau: 0.015, freq: f, ..base }
        }
        // Mould burned under the lamp: a bright short hiss.
        GrainKind::Sizzle => Spec {
            len: rng.range(0.008, 0.02),
            attack: 0.001,
            noise: 0.3,
            noise_tau: rng.range(0.003, 0.008),
            band: rng.range(5000.0, 7500.0),
            q: 0.8,
            ..base
        },
        // A drop of water: a short rising sine.
        GrainKind::Drip => {
            let len = rng.range(0.02, 0.05);
            Spec { len, attack: 0.0008, sine: 0.5, sine_tau: len / 3.5, freq: rng.range(500.0, 1300.0), freq_end: rng.range(2.2, 4.0), ..base }
        }
        // Air moving: a rising band of noise (the storm's warning).
        GrainKind::Whoosh => Spec { len: 0.6, attack: 0.25, noise: 0.35, noise_tau: 0.16, band: 400.0, q: 0.9, band_end: 5.0, ..base },
        // The storm's cut: a heavy low impact.
        GrainKind::Thud => {
            let f = rng.range(55.0, 95.0);
            Spec { len: 0.25, attack: 0.001, noise: 0.35, noise_tau: 0.03, band: 300.0, q: 0.7, sine: 0.9, sine_tau: 0.08, freq: f, drop: 1.5, ..base }
        }
        // A shot: a bright, very short crack over a small low punch, so it
        // reads as a gun and not as the mould's own wet sounds.
        GrainKind::Crack => Spec {
            len: 0.09,
            attack: 0.0003,
            noise: 0.7,
            noise_tau: rng.range(0.003, 0.005),
            band: rng.range(2500.0, 3800.0),
            q: 0.7,
            band_end: 0.5,
            sine: 0.45,
            sine_tau: 0.018,
            freq: rng.range(140.0, 180.0),
            freq_end: 0.7,
            drop: 2.0,
            ..base
        },
        // A shot striking slime: a wet, rough, low slap (weight in
        // `weighted`).
        GrainKind::Splat => Spec {
            len: 0.12,
            attack: 0.001,
            noise: 0.45,
            noise_tau: rng.range(0.018, 0.03),
            band: rng.range(500.0, 900.0),
            q: 0.9,
            band_end: 0.6,
            rough: 0.5,
            sine: 0.6,
            sine_tau: 0.04,
            freq: rng.range(115.0, 145.0),
            freq_end: 0.6,
            drop: 1.0,
        },
        // A heartbeat: a low soft sine thump.
        GrainKind::Heart => Spec {
            len: 0.22,
            attack: 0.004,
            noise: 0.12,
            noise_tau: 0.01,
            band: 220.0,
            q: 0.8,
            sine: 0.9,
            sine_tau: 0.06,
            freq: rng.range(52.0, 58.0),
            freq_end: 0.8,
            ..base
        },
        // A soldier hurt: a short, rough, falling mid groan.
        GrainKind::Hurt => Spec {
            len: 0.16,
            attack: 0.003,
            noise: 0.35,
            noise_tau: 0.05,
            band: rng.range(500.0, 800.0),
            q: 2.0,
            band_end: 0.6,
            rough: 0.6,
            sine: 0.3,
            sine_tau: 0.06,
            freq: rng.range(220.0, 260.0),
            freq_end: 0.7,
            ..base
        },
    }
}

/// A recipe shaped by `weight` (0 ..= 1), for `SoundCmd::Impact`: lower,
/// longer and fuller as it grows, so the walkers a shot killed are heard
/// in its body, not only its loudness (loudness alone the ear reads as
/// nearer, not bigger).
fn weighted(mut s: Spec, weight: f32) -> Spec {
    let w = weight.clamp(0.0, 1.0);
    s.freq *= 1.0 - 0.45 * w;
    s.band *= 1.0 - 0.4 * w;
    s.len *= 1.0 + 1.2 * w;
    s.noise_tau *= 1.0 + 1.0 * w;
    s.sine_tau *= 1.0 + 1.5 * w;
    s.sine *= 1.0 + 0.6 * w;
    s
}

#[derive(Clone, Copy, Default)]
struct Grain {
    active: bool,
    kind_tag: u8,
    from_verb: bool,
    wait: u32,
    t: u32,
    len: u32,
    attack: u32,
    noise: f32,
    noise_env: f32,
    noise_decay: f32,
    band: Biquad,
    band0: f32,
    band_end: f32,
    q: f32,
    rough: f32,
    rough_lp: f32,
    sine: f32,
    sine_env: f32,
    sine_decay: f32,
    freq: f32,
    freq_end: f32,
    drop: f32,
    phase: f32,
    lp: [f32; 2],
    lp_c: f32,
    gl: f32,
    gr: f32,
}

#[derive(Clone, Copy, Default)]
struct Smoothed {
    value: f32,
    target: f32,
}

impl Smoothed {
    #[inline]
    fn step(&mut self, up: f32, down: f32) -> f32 {
        let k = if self.target > self.value { up } else { down };
        self.value += (self.target - self.value) * k;
        self.value
    }
}

#[derive(Clone, Copy, Default)]
struct Texture {
    rate: Smoothed,
    level: Smoothed,
    pan: Smoothed,
    spread: Smoothed,
    /// **Habituation**: eases a steady texture down a few dB after half a
    /// minute and snaps back on change, so the sound reports change and a
    /// constant murmur does not become wallpaper.
    habit: f32,
    steady: f32,
    last_rate: f32,
    last_level: f32,
}

/// The mould's voice (`SoundCmd::Body`).
#[derive(Clone, Copy)]
struct BodyVoice {
    level: Smoothed,
    pitch: Smoothed,
    breath: Smoothed,
    rough: Smoothed,
    bright: Smoothed,
    pan: Smoothed,
    phase: f32,
    swell: f32,
    duck: f32,
    band: Biquad,
    rough_lp: f32,
    t: u32,
}

impl Default for BodyVoice {
    fn default() -> Self {
        let s = |v: f32| Smoothed { value: v, target: v };
        BodyVoice { level: s(0.0), pitch: s(48.0), breath: s(0.25), rough: s(0.0), bright: s(0.0), pan: s(0.0), phase: 0.0, swell: 0.0, duck: 1.0, band: Biquad::default(), rough_lp: 0.5, t: 0 }
    }
}

#[derive(Clone, Copy)]
struct Burst {
    active: bool,
    kind: GrainKind,
    rate0: f32,
    secs: f32,
    t: f32,
    wait: f32,
    pan: f32,
    spread: f32,
    gain: f32,
}

impl Default for Burst {
    fn default() -> Self {
        Burst { active: false, kind: GrainKind::Crackle, rate0: 0.0, secs: 0.0, t: 0.0, wait: 0.0, pan: 0.0, spread: 0.0, gain: 0.0 }
    }
}

#[derive(Clone, Copy, Default)]
struct Tone {
    active: bool,
    wait: u32,
    t: u32,
    len: u32,
    f0: f32,
    partials: [(f32, f32, f32); 5],
    env: [f32; 5],
    phase: [f32; 5],
    gl: f32,
    gr: f32,
}

/// Freeverb-lite: four damped combs and two all-passes per side. Short on
/// purpose: a dish on a bench, not a hall.
struct Reverb {
    combs: [[Vec<f32>; 4]; 2],
    comb_pos: [[usize; 4]; 2],
    comb_lp: [[f32; 4]; 2],
    aps: [[Vec<f32>; 2]; 2],
    ap_pos: [[usize; 2]; 2],
}

impl Reverb {
    fn new(sr: f32) -> Self {
        let scale = sr / 44_100.0;
        let len = |n: usize| ((n as f32 * scale) as usize).max(8);
        let comb_n = [1116, 1188, 1277, 1356];
        let ap_n = [556, 441];
        let mk_c = |spread: usize| [0, 1, 2, 3].map(|i| vec![0.0f32; len(comb_n[i] + spread)]);
        let mk_a = |spread: usize| [0, 1].map(|i| vec![0.0f32; len(ap_n[i] + spread)]);
        Reverb { combs: [mk_c(0), mk_c(23)], comb_pos: [[0; 4]; 2], comb_lp: [[0.0; 4]; 2], aps: [mk_a(0), mk_a(23)], ap_pos: [[0; 2]; 2] }
    }

    #[inline]
    fn run(&mut self, input: f32) -> [f32; 2] {
        const FEEDBACK: f32 = 0.78;
        const DAMP: f32 = 0.3;
        let mut out = [0.0f32; 2];
        for (ch, o) in out.iter_mut().enumerate() {
            let mut acc = 0.0;
            for i in 0..4 {
                let buf = &mut self.combs[ch][i];
                let p = self.comb_pos[ch][i];
                let y = buf[p];
                self.comb_lp[ch][i] = y * (1.0 - DAMP) + self.comb_lp[ch][i] * DAMP;
                buf[p] = input + self.comb_lp[ch][i] * FEEDBACK;
                self.comb_pos[ch][i] = (p + 1) % buf.len();
                acc += y;
            }
            for i in 0..2 {
                let buf = &mut self.aps[ch][i];
                let p = self.ap_pos[ch][i];
                let b = buf[p];
                buf[p] = acc + b * 0.5;
                acc = b - acc;
                self.ap_pos[ch][i] = (p + 1) % buf.len();
            }
            *o = acc * 0.25;
        }
        out
    }
}

/// What the voice bank did: the counters a listening page puts beside each
/// recording, because "did it fire" needs a number, not a recording.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SynthStats {
    pub grains: u64,
    pub grains_by_texture: [u64; TextureKind::COUNT],
    pub verb_grains: u64,
    pub bursts: u64,
    pub alerts: u64,
    pub steals: u64,
    pub samples: u64,
}

pub struct VoiceBank {
    sr: f32,
    rng: Xs,
    grains: [Grain; GRAIN_POOL],
    textures: [Texture; TextureKind::COUNT],
    bursts: [Burst; BURSTS],
    tones: [Tone; TONES],
    reverb: Reverb,
    room_phase: f32,
    room_lp: f32,
    room_am: f32,
    lamp: Smoothed,
    lamp_pan: f32,
    body: BodyVoice,
    hush: Smoothed,
    master: Smoothed,
    k_fast: f32,
    k_slow: f32,
    k_mid: f32,
    k_hush: f32,
    block: u32,
    pub stats: SynthStats,
}

fn coef(seconds: f32, sr: f32) -> f32 {
    1.0 - (-1.0 / (seconds * sr)).exp()
}

/// Equal-power pan gains.
fn pan_gains(pan: f32) -> (f32, f32) {
    let th = (pan.clamp(-1.0, 1.0) + 1.0) * std::f32::consts::FRAC_PI_4;
    (th.cos(), th.sin())
}

/// Soft limiter: linear to 0.6, then a tanh knee that never passes 1.
#[inline]
fn limit(x: f32) -> f32 {
    let a = x.abs();
    if a <= 0.6 {
        x
    } else {
        x.signum() * (0.6 + 0.4 * ((a - 0.6) / 0.4).tanh())
    }
}

impl VoiceBank {
    pub fn new(sample_rate: u32, seed: u64) -> Self {
        let sr = sample_rate as f32;
        let t = Texture { habit: 1.0, ..Texture::default() };
        VoiceBank {
            sr,
            rng: Xs(seed | 1),
            grains: [Grain::default(); GRAIN_POOL],
            textures: [t; TextureKind::COUNT],
            bursts: [Burst::default(); BURSTS],
            tones: [Tone::default(); TONES],
            reverb: Reverb::new(sr),
            room_phase: 0.0,
            room_lp: 0.0,
            room_am: 0.0,
            lamp: Smoothed::default(),
            lamp_pan: 0.0,
            body: BodyVoice::default(),
            hush: Smoothed { value: 1.0, target: 1.0 },
            master: Smoothed { value: DEFAULT_MASTER * DEFAULT_MASTER, target: DEFAULT_MASTER * DEFAULT_MASTER },
            k_fast: coef(0.05, sr),
            k_slow: coef(0.4, sr),
            k_mid: coef(0.2, sr),
            k_hush: coef(0.1, sr),
            block: 0,
            stats: SynthStats::default(),
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sr as u32
    }

    pub fn apply(&mut self, cmd: SoundCmd) {
        match cmd {
            SoundCmd::Texture { kind, rate, level, pan, spread } => {
                let t = &mut self.textures[kind as usize];
                t.rate.target = rate.max(0.0);
                t.level.target = level.max(0.0);
                t.pan.target = pan;
                t.spread.target = spread;
            }
            SoundCmd::Grain { kind, pan, gain, delay } => {
                self.spawn(kind, pan, gain, (delay.max(0.0) * self.sr) as u32, true, 255);
            }
            SoundCmd::Burst { kind, rate, secs, pan, spread, gain } => {
                let slot = self.bursts.iter().position(|b| !b.active).or_else(|| {
                    // Full: replace the one nearest its end.
                    self.bursts.iter().enumerate().min_by(|a, b| (a.1.secs * 4.0 - a.1.t).total_cmp(&(b.1.secs * 4.0 - b.1.t))).map(|(i, _)| i)
                });
                if let Some(i) = slot {
                    self.bursts[i] = Burst { active: true, kind, rate0: rate, secs: secs.max(0.05), t: 0.0, wait: 0.0, pan, spread, gain };
                    self.stats.bursts += 1;
                }
            }
            SoundCmd::Alert { kind, weight, pan } => self.alert(kind, weight, pan),
            SoundCmd::Impact { weight, pan, gain, delay } => {
                self.spawn_weighted(GrainKind::Splat, pan, gain, (delay.max(0.0) * self.sr) as u32, true, 255, Some(weight));
                // A heavy hit ducks the mould's voice so it still punches.
                self.body.duck = self.body.duck.min(1.0 - 0.6 * weight.clamp(0.0, 1.0));
            }
            SoundCmd::Body { level, pitch, breath, rough, bright, pan } => {
                let b = &mut self.body;
                b.level.target = level.clamp(0.0, 1.0);
                b.pitch.target = pitch.clamp(20.0, 400.0);
                b.breath.target = breath.clamp(0.02, 3.0);
                b.rough.target = rough.clamp(0.0, 1.0);
                b.bright.target = bright.clamp(0.0, 1.0);
                b.pan.target = pan.clamp(-1.0, 1.0);
            }
            SoundCmd::Lamp { on, pan } => {
                self.lamp.target = if on { 1.0 } else { 0.0 };
                self.lamp_pan = pan;
            }
            SoundCmd::Hush(on) => self.hush.target = if on { 0.0 } else { 1.0 },
            SoundCmd::Master(m) => {
                let m = m.clamp(0.0, 1.0);
                self.master.target = m * m;
            }
        }
    }

    /// Start a grain, stealing the quietest if the pool is full. `tag` is
    /// the texture index (255 = not a texture), kept for its counter.
    fn spawn(&mut self, kind: GrainKind, pan: f32, gain: f32, wait: u32, from_verb: bool, tag: u8) {
        self.spawn_weighted(kind, pan, gain, wait, from_verb, tag, None);
    }

    /// `spawn`, with the recipe shaped by `weight` when given (`weighted`).
    #[allow(clippy::too_many_arguments)]
    fn spawn_weighted(&mut self, kind: GrainKind, pan: f32, gain: f32, wait: u32, from_verb: bool, tag: u8, weight: Option<f32>) {
        if gain <= 1e-5 {
            return;
        }
        let kind_tag = kind as u8;
        let slot = if from_verb && self.grains.iter().filter(|g| g.active && g.from_verb && g.kind_tag == kind_tag).count() >= PER_KIND_CAP {
            self.grains.iter().enumerate().filter(|(_, g)| g.active && g.from_verb && g.kind_tag == kind_tag).max_by_key(|(_, g)| g.t).map(|(i, _)| i)
        } else {
            self.grains.iter().position(|g| !g.active)
        };
        let i = match slot {
            Some(i) => i,
            None => {
                self.stats.steals += 1;
                let loud = |g: &Grain| g.noise * g.noise_env + g.sine * g.sine_env;
                self.grains.iter().enumerate().min_by(|a, b| loud(a.1).total_cmp(&loud(b.1))).map(|(i, _)| i).unwrap_or(0)
            }
        };
        let s = recipe(kind, &mut self.rng);
        let s = match weight {
            Some(w) => weighted(s, w),
            None => s,
        };
        let sr = self.sr;
        let (gl, gr) = pan_gains(pan);
        let mut band = Biquad::default();
        band.set(s.band.max(40.0), s.q, sr);
        // A gentle fixed low-pass at ~12 kHz takes the edge off every grain.
        let lp_c = 1.0 - (-std::f32::consts::TAU * 12_000.0 / sr).exp();
        let decay = |tau: f32| if tau > 0.0 { (-1.0 / (tau * sr)).exp() } else { 0.0 };
        self.grains[i] = Grain {
            active: true,
            kind_tag,
            from_verb,
            wait,
            t: 0,
            len: (s.len * sr).max(1.0) as u32,
            attack: (s.attack * sr).max(1.0) as u32,
            noise: s.noise * gain,
            noise_env: 1.0,
            noise_decay: decay(s.noise_tau),
            band,
            band0: s.band,
            band_end: s.band_end,
            q: s.q,
            rough: s.rough,
            rough_lp: 0.5,
            sine: s.sine * gain,
            sine_env: 1.0,
            sine_decay: decay(s.sine_tau),
            freq: s.freq,
            freq_end: s.freq_end,
            drop: s.drop,
            phase: 0.0,
            lp: [0.0; 2],
            lp_c,
            gl,
            gr,
        };
        self.stats.grains += 1;
        if from_verb {
            self.stats.verb_grains += 1;
        }
        if (tag as usize) < TextureKind::COUNT {
            self.stats.grains_by_texture[tag as usize] += 1;
        }
    }

    fn alert(&mut self, kind: AlertKind, weight: f32, pan: f32) {
        let w = weight.clamp(0.0, 1.0);
        let marimba = [(1.0, 1.0, 0.35), (3.93, 0.25, 0.08), (9.2, 0.08, 0.03), (0.0, 0.0, 0.1), (0.0, 0.0, 0.1)];
        let bell = [(1.0, 1.0, 0.9), (2.0, 0.5, 0.6), (2.76, 0.35, 0.4), (5.4, 0.15, 0.2), (0.5, 0.3, 1.2)];
        // (frequency, delay seconds, gain, bell?)
        let mut notes: [(f32, f32, f32, bool); 3] = [(0.0, 0.0, 0.0, false); 3];
        // C major pentatonic from C5, climbing with how much food is joined.
        const PENTA: [f32; 6] = [523.25, 587.33, 659.25, 783.99, 880.0, 1046.5];
        match kind {
            AlertKind::Joined => {
                let k = ((w * (PENTA.len() - 1) as f32).round() as usize).min(PENTA.len() - 1);
                notes[0] = (PENTA[k], 0.0, 0.10 + 0.08 * w, false);
            }
            AlertKind::Bell => {
                // The end of a dish: a rising triad if it scored, more notes
                // for more stars (`weight` = stars / 3).
                let n = (w * 3.0).round() as usize;
                for (j, f) in [392.0, 493.88, 587.33].iter().enumerate().take(n.max(1)) {
                    notes[j] = (*f, 0.18 * j as f32, if n == 0 { 0.08 } else { 0.18 }, true);
                }
            }
            AlertKind::Fallen => {
                notes[0] = (146.8, 0.0, 0.16, true);
                notes[1] = (110.0, 0.22, 0.16, true);
            }
            AlertKind::Died => {
                notes[0] = (196.0, 0.0, 0.25, true);
                notes[1] = (146.8, 0.5, 0.25, true);
                notes[2] = (98.0, 1.0, 0.3, true);
            }
        }
        let (gl, gr) = pan_gains(pan * 0.6);
        for &(f, delay, g, is_bell) in notes.iter().filter(|n| n.2 > 0.0) {
            let partials = if is_bell { bell } else { marimba };
            let longest = partials.iter().map(|p| p.2).fold(0.0f32, f32::max);
            if let Some(slot) = self.tones.iter_mut().find(|t| !t.active) {
                *slot = Tone {
                    active: true,
                    wait: (delay * self.sr) as u32,
                    t: 0,
                    len: (longest * 6.0 * self.sr) as u32,
                    f0: f,
                    partials: partials.map(|(r, a, tau)| (r, a * g, (-1.0 / (tau.max(1e-3) * self.sr)).exp())),
                    env: [1.0; 5],
                    phase: [0.0; 5],
                    gl,
                    gr,
                };
            }
        }
        self.stats.alerts += 1;
    }

    /// Per-block bookkeeping: habituation. Every 512 samples.
    fn block_update(&mut self) {
        let dt = 512.0 / self.sr;
        for t in self.textures.iter_mut() {
            let (r, l) = (t.rate.target, t.level.target);
            let rel = |a: f32, b: f32| (a - b).abs() / b.abs().max(1e-3);
            if rel(r, t.last_rate) < 0.15 && rel(l, t.last_level) < 0.15 {
                t.steady += dt;
            } else {
                t.steady = 0.0;
            }
            t.last_rate = r;
            t.last_level = l;
            let goal = if t.steady > 30.0 { 0.63 } else { 1.0 };
            let k = if goal < t.habit { dt / 6.0 } else { dt / 0.25 };
            t.habit += (goal - t.habit) * k.min(1.0);
        }
    }

    /// Fill `out` with interleaved samples for `channels` channels (1 or 2
    /// are mixed properly; more get the stereo pair and silence after).
    pub fn render(&mut self, out: &mut [f32], channels: usize) {
        let channels = channels.max(1);
        let sr = self.sr;
        for frame in out.chunks_mut(channels) {
            self.block += 1;
            if self.block >= 512 {
                self.block = 0;
                self.block_update();
            }
            let hush = self.hush.step(self.k_hush, self.k_hush * 0.33);

            // Textures: smooth the targets, then spawn grains Poisson-wise.
            for (ti, kind) in TextureKind::ALL.iter().enumerate() {
                let (rate, level, pan, spread, habit) = {
                    let t = &mut self.textures[ti];
                    (t.rate.step(self.k_fast, self.k_slow), t.level.step(self.k_fast, self.k_slow), t.pan.step(self.k_mid, self.k_mid), t.spread.step(self.k_mid, self.k_mid), t.habit)
                };
                let rate = rate.min(TEXTURE_RATE_CAP);
                if rate > 0.01 && hush > 0.05 && level > 1e-4 && self.rng.unit() < rate / sr {
                    let p = pan + spread * self.rng.gauss() * 0.5;
                    let g = level * hush * habit * self.rng.range(0.6, 1.0);
                    self.spawn(kind.grain(), p, g, 0, false, ti as u8);
                }
            }

            // Bursts.
            for bi in 0..BURSTS {
                let b = self.bursts[bi];
                if !b.active {
                    continue;
                }
                let dt = 1.0 / sr;
                if b.wait > 0.0 {
                    self.bursts[bi].wait -= dt;
                    continue;
                }
                let rate = b.rate0 * (-b.t / b.secs).exp();
                self.bursts[bi].t += dt;
                if b.t > b.secs * 4.0 {
                    self.bursts[bi].active = false;
                    continue;
                }
                if self.rng.unit() < rate / sr {
                    let p = b.pan + b.spread * self.rng.gauss() * 0.5;
                    let g = b.gain * self.rng.range(0.6, 1.0);
                    self.spawn(b.kind, p, g, 0, false, 255);
                }
            }

            // Grains.
            let (mut l, mut r) = (0.0f32, 0.0f32);
            for g in self.grains.iter_mut() {
                if !g.active {
                    continue;
                }
                if g.wait > 0 {
                    g.wait -= 1;
                    continue;
                }
                let tf = g.t as f32;
                let life = tf / g.len as f32;
                let att = (tf / g.attack as f32).min(1.0);
                let mut y = 0.0;
                if g.noise > 0.0 {
                    if g.band_end != 1.0 && g.t % 64 == 0 {
                        let c = g.band0 * (1.0 + (g.band_end - 1.0) * life);
                        let q = g.q;
                        g.band.set(c, q, sr);
                    }
                    let mut n = g.band.run(self.rng.bipolar() * 1.73);
                    if g.rough > 0.0 {
                        g.rough_lp += (self.rng.unit() - g.rough_lp) * 0.02;
                        n *= 1.0 - g.rough + g.rough * 2.0 * g.rough_lp;
                    }
                    y += n * g.noise * g.noise_env;
                    g.noise_env *= g.noise_decay;
                }
                if g.sine > 0.0 {
                    let f = g.freq * (1.0 + (g.freq_end - 1.0) * life) * (1.0 + g.drop * (-tf / (0.02 * sr)).exp());
                    y += g.phase.sin() * g.sine * g.sine_env;
                    g.phase += std::f32::consts::TAU * f / sr;
                    if g.phase > std::f32::consts::TAU {
                        g.phase -= std::f32::consts::TAU;
                    }
                    g.sine_env *= g.sine_decay;
                }
                y *= att;
                g.lp[0] += (y - g.lp[0]) * g.lp_c;
                g.lp[1] += (g.lp[0] - g.lp[1]) * g.lp_c;
                l += g.lp[1] * g.gl;
                r += g.lp[1] * g.gr;
                g.t += 1;
                if g.t >= g.len || (g.noise_env + g.sine_env < 1e-4 && g.t > g.attack) {
                    g.active = false;
                }
            }

            // Tones.
            for t in self.tones.iter_mut() {
                if !t.active {
                    continue;
                }
                if t.wait > 0 {
                    t.wait -= 1;
                    continue;
                }
                let att = (t.t as f32 / (0.003 * sr)).min(1.0);
                let mut y = 0.0;
                for k in 0..5 {
                    let (ratio, amp, decay) = t.partials[k];
                    if amp <= 0.0 || ratio <= 0.0 {
                        continue;
                    }
                    y += t.phase[k].sin() * amp * t.env[k];
                    t.phase[k] = (t.phase[k] + std::f32::consts::TAU * t.f0 * ratio / sr) % std::f32::consts::TAU;
                    t.env[k] *= decay;
                }
                y *= att;
                l += y * t.gl;
                r += y * t.gr;
                t.t += 1;
                if t.t >= t.len {
                    t.active = false;
                }
            }

            // The mould's voice: a sub tone and its two partials under a band
            // of noise, swelling with its breath, roughened by strain.
            {
                let (k_slow, k_mid) = (self.k_slow, self.k_mid);
                let b = &mut self.body;
                let level = b.level.step(k_slow, k_slow);
                if level > 1e-4 || b.level.target > 0.0 {
                    let pitch = b.pitch.step(k_slow, k_slow);
                    let breath = b.breath.step(k_slow, k_slow);
                    let rough = b.rough.step(k_slow, k_slow);
                    let bright = b.bright.step(k_mid, k_slow);
                    let pan = b.pan.step(k_slow, k_slow);
                    b.duck += (1.0 - b.duck) * (1.0 / (0.3 * sr));
                    b.swell = (b.swell + breath / sr) % 1.0;
                    // In-breath shorter than out-breath: a skewed swell.
                    let ph = b.swell;
                    let sw = if ph < 0.4 { (ph / 0.4 * std::f32::consts::FRAC_PI_2).sin() } else { ((1.0 - ph) / 0.6 * std::f32::consts::FRAC_PI_2).sin() };
                    let swell = 0.35 + 0.65 * sw * sw;
                    b.phase = (b.phase + std::f32::consts::TAU * pitch / sr) % std::f32::consts::TAU;
                    let tone = b.phase.sin() + 0.45 * (2.0 * b.phase + 0.3).sin() + 0.2 * (3.0 * b.phase + 0.7).sin();
                    if b.t.is_multiple_of(64) {
                        b.band.set(pitch * (5.0 + 10.0 * bright), 0.8, sr);
                    }
                    b.t = b.t.wrapping_add(1);
                    let mut n = b.band.run(self.rng.bipolar() * 1.73);
                    b.rough_lp += (self.rng.unit() - b.rough_lp) * 0.004;
                    n *= 1.0 - rough + rough * 2.0 * b.rough_lp;
                    let y = (0.55 * tone * (1.0 - 0.5 * rough) + (0.25 + 0.35 * bright + 0.3 * rough) * n * 0.25) * swell * level * b.duck * hush;
                    let (gl, gr) = pan_gains(pan * 0.5);
                    l += y * gl;
                    r += y * gr;
                }
            }

            // Room tone: low air always, and the lamp's 120 Hz hum where the
            // lamp stands while it is lit.
            let lamp = self.lamp.step(self.k_mid, self.k_mid);
            self.room_phase = (self.room_phase + std::f32::consts::TAU * 120.0 / sr) % std::f32::consts::TAU;
            self.room_am = (self.room_am + std::f32::consts::TAU * 0.3 / sr) % std::f32::consts::TAU;
            let hum = (self.room_phase.sin() + 0.4 * (2.0 * self.room_phase).sin() + 0.15 * (3.0 * self.room_phase).sin()) * (1.0 + 0.1 * self.room_am.sin());
            self.room_lp += (self.rng.bipolar() - self.room_lp) * 0.02;
            let air = self.room_lp * 2.0 * ROOM_LEVEL;
            let hum = hum * 0.6 * ROOM_LEVEL * LAMP_HUM * lamp;
            let (hl, hr) = pan_gains(self.lamp_pan);

            let wet = self.reverb.run((l + r) * 0.5 * 0.35);
            let master = self.master.step(self.k_mid, self.k_mid) * MAKEUP;
            let lo = limit((l + wet[0] + air + hum * hl) * master);
            let ro = limit((r + wet[1] + air + hum * hr) * master);
            match channels {
                1 => frame[0] = (lo + ro) * 0.5,
                _ => {
                    frame[0] = lo;
                    frame[1] = ro;
                    for s in frame.iter_mut().skip(2) {
                        *s = 0.0;
                    }
                }
            }
            self.stats.samples += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(bank: &mut VoiceBank, secs: f32) -> Vec<f32> {
        let mut out = vec![0.0; (secs * bank.sr) as usize * 2];
        bank.render(&mut out, 2);
        out
    }

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|v| v * v).sum::<f32>() / x.len().max(1) as f32).sqrt()
    }

    fn creep(rate: f32, level: f32) -> SoundCmd {
        SoundCmd::Texture { kind: TextureKind::Creep, rate, level, pan: 0.0, spread: 0.3 }
    }

    #[test]
    fn the_same_commands_and_seed_render_the_same_samples() {
        let run = || {
            let mut b = VoiceBank::new(48_000, 7);
            b.apply(creep(150.0, 0.5));
            b.apply(SoundCmd::Alert { kind: AlertKind::Joined, weight: 0.8, pan: 0.0 });
            render(&mut b, 1.0)
        };
        assert_eq!(run(), run());
    }

    #[test]
    fn a_silent_bank_plays_only_the_room_tone() {
        let mut b = VoiceBank::new(48_000, 1);
        let level = rms(&render(&mut b, 1.0));
        assert!(level > 0.0, "the room tone must be there: a dead-silent dish reads as broken");
        assert!(level < 0.02, "room tone {level} is louder than a whisper");
    }

    /// The positive control for the room test: a busy texture must be much
    /// louder than the room, or "quiet" and "busy" would be the same.
    #[test]
    fn a_texture_is_heard_over_the_room_and_stays_under_full_scale() {
        let mut quiet = VoiceBank::new(48_000, 1);
        let room = rms(&render(&mut quiet, 1.0));
        let mut busy = VoiceBank::new(48_000, 1);
        busy.apply(creep(200.0, 0.8));
        let x = render(&mut busy, 1.0);
        assert!(rms(&x) > room * 3.0, "texture {} vs room {room}", rms(&x));
        assert!(x.iter().all(|v| v.abs() <= 1.0), "the limiter let a sample past full scale");
        assert!(busy.stats.grains_by_texture[0] > 100, "200/s for a second spawned {}", busy.stats.grains_by_texture[0]);
    }

    #[test]
    fn the_lamp_hums_while_lit_and_stops_after() {
        let mut off = VoiceBank::new(48_000, 2);
        let room = rms(&render(&mut off, 1.0));
        let mut on = VoiceBank::new(48_000, 2);
        on.apply(SoundCmd::Lamp { on: true, pan: 0.0 });
        let lit = rms(&render(&mut on, 1.0)[48_000..]);
        assert!(lit > room * 1.5, "lamp hum {lit} vs room {room}");
        on.apply(SoundCmd::Lamp { on: false, pan: 0.0 });
        render(&mut on, 2.0);
        let after = rms(&render(&mut on, 0.5));
        assert!(after < lit * 0.7, "the hum stayed after the lamp went out: {after} vs {lit}");
    }

    #[test]
    fn hush_fades_the_textures_but_not_verbs() {
        let mut b = VoiceBank::new(48_000, 3);
        b.apply(creep(200.0, 0.8));
        render(&mut b, 0.3);
        b.apply(SoundCmd::Hush(true));
        render(&mut b, 1.0);
        let before = b.stats.grains_by_texture[0];
        render(&mut b, 0.5);
        assert!(b.stats.grains_by_texture[0] - before < 5, "a hushed texture kept spawning");
        b.apply(SoundCmd::Grain { kind: GrainKind::Thunk, pan: 0.0, gain: 0.5, delay: 0.0 });
        render(&mut b, 0.1);
        assert_eq!(b.stats.verb_grains, 1, "a verb must still answer while paused");
    }

    #[test]
    fn the_pool_steals_instead_of_growing() {
        let mut b = VoiceBank::new(48_000, 5);
        for i in 0..(GRAIN_POOL + 10) {
            let kind = if i % 2 == 0 { GrainKind::Thud } else { GrainKind::Knock };
            b.spawn(kind, 0.0, 0.5, 0, false, 255);
        }
        assert_eq!(b.stats.steals, 10);
        assert!(b.grains.iter().all(|g| g.active));
    }

    #[test]
    fn verb_grains_of_one_kind_are_capped() {
        let mut b = VoiceBank::new(48_000, 5);
        for _ in 0..10 {
            b.apply(SoundCmd::Grain { kind: GrainKind::Thunk, pan: 0.0, gain: 0.5, delay: 0.0 });
        }
        let live = b.grains.iter().filter(|g| g.active && g.kind_tag == GrainKind::Thunk as u8).count();
        assert_eq!(live, PER_KIND_CAP);
    }

    #[test]
    fn a_steady_texture_habituates_and_a_change_wakes_it() {
        let mut b = VoiceBank::new(8_000, 9);
        b.apply(creep(100.0, 0.5));
        render(&mut b, 40.0);
        let settled = b.textures[0].habit;
        assert!(settled < 0.8, "after 40 s of no change habit is {settled}");
        b.apply(creep(300.0, 0.9));
        render(&mut b, 1.0);
        assert!(b.textures[0].habit > 0.95, "a change should wake it, habit {}", b.textures[0].habit);
    }

    /// Larder World's hit: a shot that killed many is a longer, fuller
    /// sound than one that killed few, not only a louder one.
    #[test]
    fn a_heavier_impact_rings_longer_and_fuller() {
        let hit = |weight: f32| {
            let mut b = VoiceBank::new(48_000, 13);
            b.apply(SoundCmd::Impact { weight, pan: 0.0, gain: 0.5, delay: 0.0 });
            let x = render(&mut b, 0.6);
            let peak = x.iter().fold(0.0f32, |m, v| m.max(v.abs()));
            let tail = x.iter().rposition(|v| v.abs() > peak * 0.1).unwrap_or(0);
            (tail as f32 / 2.0 / 48_000.0, rms(&x[..9_600]))
        };
        let (light_len, light_rms) = hit(0.0);
        let (heavy_len, heavy_rms) = hit(1.0);
        assert!(heavy_len > light_len * 1.3, "heavy {heavy_len} s vs light {light_len} s");
        assert!(heavy_rms > light_rms, "heavy {heavy_rms} vs light {light_rms}");
    }

    /// The mould's voice is heard, breathes (its loudness swells at the
    /// breath rate), and a heavy hit ducks it.
    #[test]
    fn the_mould_voice_breathes_and_ducks_under_a_heavy_hit() {
        let body = SoundCmd::Body { level: 0.3, pitch: 50.0, breath: 0.5, rough: 0.2, bright: 0.0, pan: 0.0 };
        let mut b = VoiceBank::new(8_000, 21);
        b.apply(body);
        let x = render(&mut b, 6.0);
        let window = |i: usize| rms(&x[i * 2 * 800..(i + 1) * 2 * 800]);
        let quarters: Vec<f32> = (10..60).map(window).collect();
        let (lo, hi) = quarters.iter().fold((f32::MAX, 0.0f32), |(a, c), &v| (a.min(v), c.max(v)));
        assert!(hi > lo * 1.6, "no breath: tenth-second RMS {lo}..{hi}");
        let mut quiet = VoiceBank::new(8_000, 21);
        let room = rms(&render(&mut quiet, 1.0));
        assert!(rms(&x) > room * 3.0, "voice {} vs room {room}", rms(&x));
        b.apply(SoundCmd::Impact { weight: 1.0, pan: 0.0, gain: 0.0, delay: 0.0 });
        assert!(b.body.duck < 0.5, "a heavy hit did not duck the voice");
        render(&mut b, 1.5);
        assert!(b.body.duck > 0.95, "the duck did not recover: {}", b.body.duck);
    }

    #[test]
    fn every_recipe_and_alert_makes_a_finite_bounded_sound() {
        for kind in GrainKind::ALL {
            let mut b = VoiceBank::new(48_000, 11);
            b.apply(SoundCmd::Grain { kind, pan: 0.0, gain: 1.0, delay: 0.0 });
            let x = render(&mut b, 0.8);
            assert!(x.iter().all(|v| v.is_finite() && v.abs() <= 1.0), "{kind:?}");
            assert!(rms(&x) > 0.0005, "{kind:?} made no sound");
        }
        for kind in [AlertKind::Joined, AlertKind::Bell, AlertKind::Died, AlertKind::Fallen] {
            for w in [0.0, 0.5, 1.0] {
                let mut b = VoiceBank::new(48_000, 12);
                b.apply(SoundCmd::Alert { kind, weight: w, pan: 0.0 });
                let x = render(&mut b, 1.5);
                assert!(x.iter().all(|v| v.is_finite() && v.abs() <= 1.0), "{kind:?} {w}");
                assert!(rms(&x) > 0.001, "{kind:?} {w} made no sound");
            }
        }
    }
}
