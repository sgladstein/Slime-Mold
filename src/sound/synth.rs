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

use super::score::Score;
use super::{AlertKind, GrainKind, GunVoice, SoundCmd, TextureKind};

/// Grains that can sound at once, across every texture, burst and verb.
/// Over this the quietest playing grain is stolen. 64 until 2026-11-20,
/// when a gunshot became four to six grains.
pub const GRAIN_POOL: usize = 96;
/// The most grains of one kind fired by *verbs* that may sound together;
/// past it the oldest of that kind is replaced.
pub const PER_KIND_CAP: usize = 4;
const BURSTS: usize = 6;
/// 8 until 2026-11-21, when the coin ticks (`AlertKind::Coin`, a few at
/// once while a squad drains a drop) could take every slot from a
/// soldier's fall.
const TONES: usize = 12;
/// The first-run master level: well below full, so the first launch is
/// never loud. The Options page's Volume scales it (`master_at`).
pub const DEFAULT_MASTER: f32 = 0.5;

/// The master for the Options page's `volume` (1 = 100%, the first-run
/// level): `SoundCmd::Master` squares it, so the steps are even in dB.
pub fn master_at(volume: f32) -> f32 {
    (DEFAULT_MASTER * volume).clamp(0.0, 1.0)
}
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
    /// Extra pitch at the onset that falls away in `drop_tau`: an impact.
    pub drop: f32,
    /// Seconds the onset pitch takes to fall away; 0 is 20 ms.
    pub drop_tau: f32,
    /// Its send to the reverb (`VoiceBank::render`), 0 ..= 1. Until
    /// 2026-11-20 the whole mix went to the reverb at 0.35, the mould's
    /// drone and the pulse too, and the reverb was the loudest thing in a
    /// fight (the mix meter, `SynthStats::energy`). Now a gun's report
    /// rings and a texture stays dry.
    pub verb: f32,
    /// Its low-pass, Hz; 0 is `LP_HZ`.
    pub lp_hz: f32,
}

/// Every grain's low-pass when its recipe sets none: takes the edge off.
const LP_HZ: f32 = 12_000.0;
/// The least low-pass of a sound far off screen (`SoundCmd::Gun`'s
/// `far` 1): air takes the top off a distant sound before its body, so a
/// fight across the map is duller, not only quieter.
const FAR_LP_HZ: f32 = 1_800.0;
/// Samples a stolen grain takes to fade out (about 4 ms at 48 kHz), so a
/// grain cut short by a newer one is a fade, not a click.
const FADE_SAMPLES: f32 = 192.0;
/// A tone's reverb send (a chime, a bell, a siren).
const TONE_VERB: f32 = 0.4;
/// The reverb's input level: the old mix sent half of left plus right at
/// 0.35; a send of 1 here is about what that was.
const VERB_IN: f32 = 0.35;
/// The reverb's input high-pass, Hz: a room rings in its mids, so low
/// sound in the reverb is only mud.
const VERB_HP_HZ: f32 = 280.0;
/// The master high-pass, Hz: below this a speaker only moves air the ear
/// does not hear and the limiter does (it also takes off any offset).
const MASTER_HP_HZ: f32 = 28.0;

/// **The recipes.** First guesses, to be tuned by ear against the offline
/// renders on the listening page; jitter is drawn here so two grains of one
/// kind are never identical (the machine-gun effect).
fn recipe(kind: GrainKind, rng: &mut Xs) -> Spec {
    let base = Spec { band_end: 1.0, freq_end: 1.0, q: 1.4, verb: 0.35, ..Spec::default() };
    // The quiet beds stay dry: a texture spawns hundreds of grains a
    // second and in the reverb they only wash.
    let dry = Spec { verb: 0.1, ..base };
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
                ..dry
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
            ..dry
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
            ..dry
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
        // A shot's crack heard alone (a `Grain` command; the game's guns
        // send `SoundCmd::Gun`, which builds its crack in `gun_layers`).
        // Until 2026-11-20 this was the whole gunshot: a 3-5 ms noise snap
        // at 2.5-3.8 kHz over a 160 Hz blip, the same for every gun.
        GrainKind::Crack => Spec {
            len: 0.09,
            attack: 0.0003,
            noise: 0.7,
            noise_tau: rng.range(0.004, 0.006),
            band: rng.range(2600.0, 3600.0),
            q: 0.6,
            band_end: 0.55,
            sine: 0.5,
            sine_tau: 0.04,
            freq: rng.range(90.0, 105.0),
            freq_end: 0.75,
            drop: 2.5,
            drop_tau: 0.012,
            verb: 0.3,
            ..base
        },
        // A gun's punch, its report and its action heard alone; the guns
        // build their own (`gun_layers`).
        GrainKind::Punch => Spec { len: 0.2, attack: 0.0005, sine: 0.9, sine_tau: 0.045, freq: rng.range(85.0, 100.0), freq_end: 0.8, drop: 2.5, drop_tau: 0.012, noise: 0.3, noise_tau: 0.015, band: 320.0, q: 0.8, verb: 0.05, ..base },
        GrainKind::Report => Spec { len: 0.5, attack: 0.006, noise: 0.3, noise_tau: 0.15, band: rng.range(800.0, 1000.0), q: 0.7, band_end: 0.5, rough: 0.3, verb: 0.6, ..base },
        GrainKind::Action => Spec { len: 0.02, attack: 0.0002, noise: 0.25, noise_tau: 0.0015, band: rng.range(4000.0, 5000.0), q: 4.0, verb: 0.1, ..base },
        // Gas let out (sound pass two, 2026-11-20): a soft puff into a
        // long hiss that falls as the cloud spreads.
        GrainKind::Hiss => Spec { len: 0.9, attack: 0.02, noise: 0.3, noise_tau: 0.28, band: rng.range(4200.0, 5200.0), q: 0.7, band_end: 0.55, rough: 0.25, verb: 0.3, ..base },
        // The slime gathering itself: a low rough groan that rises over a
        // second, under a sine that climbs with it.
        GrainKind::Swell => Spec {
            len: 1.7,
            attack: 0.9,
            noise: 0.45,
            noise_tau: 0.7,
            band: rng.range(170.0, 220.0),
            q: 1.3,
            band_end: 2.6,
            rough: 0.85,
            sine: 0.35,
            sine_tau: 0.8,
            freq: rng.range(70.0, 80.0),
            freq_end: 1.6,
            verb: 0.3,
            ..base
        },
        // A building set down: a short metal clank, a ringing band over a
        // falling mid tone.
        GrainKind::Clank => Spec {
            len: 0.3,
            attack: 0.0005,
            noise: 0.35,
            noise_tau: 0.05,
            band: rng.range(2200.0, 2900.0),
            q: 6.0,
            sine: 0.3,
            sine_tau: 0.09,
            freq: rng.range(420.0, 520.0),
            freq_end: 0.97,
            drop: 0.3,
            verb: 0.35,
            ..base
        },
        // A click on a menu or the HUD: a small soft tick, dry.
        GrainKind::Tick => Spec { len: 0.025, attack: 0.0003, noise: 0.25, noise_tau: 0.0015, band: rng.range(3200.0, 3800.0), q: 1.5, sine: 0.2, sine_tau: 0.004, freq: rng.range(1400.0, 1600.0), verb: 0.05, ..base },
        // A walker killed (`TextureKind::Kill`, 2026-11-20): a tiny wet
        // pop, a sine blip whose pitch jumps up as it bursts (a bubble's
        // way) with a breath of noise. Replaced the starving crackle in a
        // fight: its 2-5 kHz noise ticks, hundreds a second, read as static.
        GrainKind::Pop => {
            let len = rng.range(0.012, 0.024);
            Spec {
                len,
                attack: 0.0006,
                sine: 0.5,
                sine_tau: len / 3.0,
                freq: rng.range(380.0, 900.0),
                freq_end: rng.range(1.5, 2.3),
                noise: 0.08,
                noise_tau: 0.002,
                band: rng.range(1500.0, 2500.0),
                q: 1.0,
                verb: 0.08,
                lp_hz: 7_000.0,
                ..base
            }
        }
        // A shot striking slime (weight in `weighted`): a wet slap with a
        // falling squelch in it, set in the low mids (220-300 Hz) where a
        // small speaker still plays it. Until 2026-11-20 its sine sat at
        // 115-145 Hz, under a laptop's speakers and over the gun's body on
        // big ones; a heavy hit's extra low weight is now `weighted`'s.
        GrainKind::Splat => Spec {
            len: 0.12,
            attack: 0.0008,
            noise: 0.5,
            noise_tau: rng.range(0.012, 0.02),
            band: rng.range(700.0, 1100.0),
            q: 1.0,
            band_end: 0.5,
            rough: 0.6,
            sine: 0.55,
            sine_tau: 0.03,
            freq: rng.range(220.0, 300.0),
            freq_end: 0.5,
            drop: 0.6,
            drop_tau: 0.008,
            verb: 0.15,
            ..base
        },
        // A heartbeat: a soft thump. Its sine sits at 60-66 Hz with a
        // noise band at 160 Hz so a small speaker still hears the beat,
        // and it is dry: in the reverb (until 2026-11-20) its boom rang
        // on under the whole fight.
        GrainKind::Heart => Spec {
            len: 0.2,
            attack: 0.004,
            noise: 0.18,
            noise_tau: 0.012,
            band: 160.0,
            q: 1.0,
            sine: 0.7,
            sine_tau: 0.05,
            freq: rng.range(60.0, 66.0),
            freq_end: 0.8,
            verb: 0.0,
            ..base
        },
        // A bullet flying past: a thin band of noise sweeping down, the
        // flyby.
        GrainKind::Zip => Spec { len: 0.11, attack: 0.03, noise: 0.35, noise_tau: 0.05, band: rng.range(3000.0, 4200.0), q: 2.5, band_end: 0.45, ..base },
        // A piece's tell: a low rough growl that rises, the lean before
        // the charge.
        GrainKind::Snarl => Spec {
            len: 0.7,
            attack: 0.25,
            noise: 0.4,
            noise_tau: 0.35,
            band: rng.range(260.0, 340.0),
            q: 1.6,
            band_end: 2.2,
            rough: 0.9,
            sine: 0.35,
            sine_tau: 0.3,
            freq: rng.range(70.0, 85.0),
            freq_end: 1.8,
            ..base
        },
        // The mould torn in two: a ripping, rough noise that tears upward.
        GrainKind::Tear => Spec {
            len: 0.35,
            attack: 0.01,
            noise: 0.55,
            noise_tau: 0.12,
            band: rng.range(500.0, 800.0),
            q: 0.9,
            band_end: 3.0,
            rough: 1.0,
            sine: 0.3,
            sine_tau: 0.05,
            freq: rng.range(90.0, 110.0),
            freq_end: 0.6,
            drop: 1.0,
            ..base
        },
        // Withering: a soft dry fizz, duller than starving's crackle.
        GrainKind::Wilt => Spec {
            len: rng.range(0.03, 0.06),
            attack: 0.004,
            noise: 0.3,
            noise_tau: rng.range(0.01, 0.02),
            band: rng.range(1200.0, 2200.0),
            q: 1.2,
            band_end: 0.6,
            rough: 0.6,
            ..dry
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
    s.freq *= 1.0 - 0.5 * w;
    s.band *= 1.0 - 0.4 * w;
    s.len *= 1.0 + 1.2 * w;
    s.noise_tau *= 1.0 + 1.0 * w;
    s.sine_tau *= 1.0 + 1.5 * w;
    s.sine *= 1.0 + 0.6 * w;
    s
}

/// **A gun's numbers** (`gun_layers`): its crack (a noise band), its punch
/// (a falling sine), its report (a long noise band falling in pitch) and
/// its action (two small metal clicks after, 0 for none). Levels are
/// against the rifle's.
struct GunSound {
    /// The crack's band, Hz, its Q, its decay, s, and level.
    snap: (f32, f32, f32, f32),
    /// The first millisecond's click, level.
    click: f32,
    /// The punch's pitch, Hz, its onset drop, its decay, s, and level.
    punch: (f32, f32, f32, f32),
    /// The report's band, Hz, its end as a multiple, its decay, s, its
    /// length, s, and level.
    report: (f32, f32, f32, f32, f32),
    /// The action's delay after the shot, s, and level.
    action: (f32, f32),
}

/// **The guns' voices** (2026-11-20). One of each layer, set apart by kind
/// the way the shop tells them apart (`guns::GunKind::about`): the rifle
/// firm and middle, the carbine light and high and short (it fires twice
/// as often), the long gun deep with a long rolling report and a bolt, the
/// scattergun one wide blast with a pump, a turret ringing and metallic
/// (a narrow crack band) so it is told from the squad by ear.
fn gun_sound(v: GunVoice) -> GunSound {
    // First cut (2026-11-20) had the punches 4 dB up and the cracks 2 dB
    // down from these: in planets 1-3's recordings the punch was the mix's
    // biggest source and the 100-300 Hz band its loudest.
    match v {
        GunVoice::Rifle => GunSound { snap: (3200.0, 0.6, 0.005, 0.9), click: 0.55, punch: (92.0, 2.5, 0.045, 0.55), report: (900.0, 0.5, 0.14, 0.5, 0.3), action: (0.0, 0.0) },
        GunVoice::Carbine => GunSound { snap: (4300.0, 0.7, 0.0035, 0.75), click: 0.5, punch: (135.0, 2.0, 0.025, 0.35), report: (1300.0, 0.5, 0.07, 0.3, 0.18), action: (0.0, 0.0) },
        GunVoice::Long => GunSound { snap: (2600.0, 0.5, 0.007, 1.0), click: 0.65, punch: (62.0, 3.5, 0.09, 0.7), report: (650.0, 0.4, 0.33, 1.1, 0.4), action: (0.42, 0.35) },
        GunVoice::Scatter => GunSound { snap: (2000.0, 0.35, 0.009, 1.0), click: 0.55, punch: (75.0, 3.0, 0.07, 0.6), report: (800.0, 0.45, 0.22, 0.8, 0.35), action: (0.34, 0.3) },
        GunVoice::Turret => GunSound { snap: (2400.0, 2.5, 0.006, 0.6), click: 0.35, punch: (115.0, 1.5, 0.03, 0.35), report: (1100.0, 0.6, 0.09, 0.4, 0.15), action: (0.0, 0.0) },
    }
}

/// The layers of one shot (`SoundCmd::Gun`): each a grain kind, its
/// recipe, and its delay after the shot, s. A fixed array, so the audio
/// thread allocates nothing; an unused layer has `len` 0. Every number is
/// jittered a few per cent, so a volley is never one sound repeated.
fn gun_layers(v: GunVoice, rng: &mut Xs) -> [(GrainKind, Spec, f32); 6] {
    let g = gun_sound(v);
    let base = Spec { band_end: 1.0, freq_end: 1.0, q: 1.0, ..Spec::default() };
    let j = |rng: &mut Xs, x: f32, by: f32| x * (1.0 + by * rng.bipolar());
    let none = (GrainKind::Action, base, 0.0);
    let mut out = [none; 6];
    // The click: the first millisecond, wide and bright.
    out[0] = (GrainKind::Crack, Spec { len: 0.008, attack: 0.0001, noise: g.click, noise_tau: 0.0007, band: j(rng, 5500.0, 0.1), q: 0.5, verb: 0.1, lp_hz: 15_000.0, ..base }, 0.0);
    // The crack: the gun's own band, falling a little.
    let (hz, q, tau, lvl) = g.snap;
    out[1] = (GrainKind::Crack, Spec { len: (tau * 10.0).max(0.04), attack: 0.0002, noise: j(rng, lvl, 0.1), noise_tau: j(rng, tau, 0.15), band: j(rng, hz, 0.08), q, band_end: 0.6, verb: 0.25, ..base }, 0.0);
    // The punch: a falling sine with a low noise thump under it.
    let (hz, drop, tau, lvl) = g.punch;
    out[2] = (
        GrainKind::Punch,
        Spec { len: tau * 5.0, attack: 0.0004, sine: j(rng, lvl, 0.1), sine_tau: tau, freq: j(rng, hz, 0.06), freq_end: 0.75, drop, drop_tau: 0.012, noise: 0.35 * lvl, noise_tau: 0.012, band: j(rng, 300.0, 0.1), q: 0.8, verb: 0.05, ..base },
        0.0,
    );
    // The report: the tail that rolls off, rough, wet, a breath after.
    let (hz, end, tau, len, lvl) = g.report;
    out[3] = (GrainKind::Report, Spec { len, attack: 0.008, noise: j(rng, lvl, 0.15), noise_tau: j(rng, tau, 0.15), band: j(rng, hz, 0.1), q: 0.7, band_end: end, rough: 0.35, verb: 0.6, lp_hz: 8_000.0, ..base }, 0.004);
    // The action: two small clicks, the second lower.
    let (at, lvl) = g.action;
    if lvl > 0.0 {
        let click = |band: f32| Spec { len: 0.02, attack: 0.0002, noise: lvl, noise_tau: 0.0018, band, q: 3.5, verb: 0.1, ..base };
        out[4] = (GrainKind::Action, click(j(rng, 4800.0, 0.05)), j(rng, at, 0.05));
        out[5] = (GrainKind::Action, click(j(rng, 3400.0, 0.05)), at + j(rng, 0.09, 0.1));
    }
    out
}

/// The most grains of each kind fired by verbs that may sound together
/// (`PER_KIND_CAP` but for the guns' layers: a volley's cracks are short
/// and its reports long, so both need room).
fn kind_cap(kind: GrainKind) -> usize {
    match kind {
        GrainKind::Crack => 10,
        GrainKind::Punch | GrainKind::Report => 6,
        _ => PER_KIND_CAP,
    }
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
    /// The onset pitch's envelope and its decay a sample.
    drop_env: f32,
    drop_k: f32,
    /// Its reverb send.
    verb: f32,
    /// Fading out (stolen): samples left of `FADE_SAMPLES`, 0 not fading.
    fade: f32,
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

/// The mould's voices: the main body and up to three cut-off pieces
/// (`SoundCmd::Body`'s slot).
pub const BODY_VOICES: usize = 4;

/// One of the mould's voices (`SoundCmd::Body`).
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SynthStats {
    pub grains: u64,
    pub grains_by_texture: [u64; TextureKind::COUNT],
    pub verb_grains: u64,
    pub bursts: u64,
    pub alerts: u64,
    pub steals: u64,
    pub samples: u64,
    /// **The mix meter**: the energy (sum of squares, both channels, before
    /// the master) each source put in, so a recording says what it was
    /// mostly made of: one per `GrainKind`, then `MIX_TONES`, `MIX_BODY`,
    /// `MIX_ROOM` and `MIX_VERB`.
    pub energy: [f64; MIX_SOURCES],
    /// Samples the limiter pulled down, and the deepest it went (a gain).
    pub limited: u64,
    pub limit_min: f32,
    /// Commands thrown away because a number in them was not finite
    /// (`VoiceBank::apply`), and times the bank found its own output not
    /// finite and cleared itself (`VoiceBank::recover`). Both should stay 0.
    pub refused: u64,
    pub recovers: u64,
}

/// The mix meter's sources past the grains (`SynthStats::energy`).
pub const MIX_TONES: usize = GrainKind::ALL.len();
pub const MIX_BODY: usize = MIX_TONES + 1;
pub const MIX_ROOM: usize = MIX_TONES + 2;
pub const MIX_VERB: usize = MIX_TONES + 3;
/// The music bed and the planet's air (`super::score`), dry.
pub const MIX_SCORE: usize = MIX_TONES + 4;
pub const MIX_SOURCES: usize = MIX_TONES + 5;

/// A name for each mix meter source, for printing.
pub fn mix_name(i: usize) -> String {
    match i {
        MIX_TONES => "tones".into(),
        MIX_BODY => "voice".into(),
        MIX_ROOM => "room".into(),
        MIX_VERB => "reverb".into(),
        MIX_SCORE => "score".into(),
        _ => GrainKind::ALL.get(i).map_or("?".into(), |k| format!("{k:?}").to_lowercase()),
    }
}

impl SynthStats {
    /// The sources, loudest first, each in dB against the whole mix's
    /// energy, down to `floor_db`.
    pub fn mix(&self, floor_db: f32) -> Vec<(String, f32)> {
        let total: f64 = self.energy.iter().sum::<f64>().max(1e-12);
        let mut v: Vec<(String, f32)> = (0..MIX_SOURCES).map(|i| (mix_name(i), (10.0 * (self.energy[i].max(1e-30) / total).log10()) as f32)).filter(|x| x.1 > floor_db).collect();
        v.sort_by(|a, b| b.1.total_cmp(&a.1));
        v
    }
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
    body: [BodyVoice; BODY_VOICES],
    hush: Smoothed,
    master: Smoothed,
    k_fast: f32,
    k_slow: f32,
    k_mid: f32,
    k_hush: f32,
    /// The reverb input's and the master's high-pass states and rates.
    verb_lp: f32,
    k_verb_hp: f32,
    master_lp: [f32; 2],
    k_master_hp: f32,
    block: u32,
    /// The music bed and the planet's air (`SoundCmd::Score`).
    score: Score,
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

/// Whether every number in `cmd` is finite (`VoiceBank::apply`).
fn cmd_finite(cmd: &SoundCmd) -> bool {
    let ok = |xs: &[f32]| xs.iter().all(|x| x.is_finite());
    match *cmd {
        SoundCmd::Texture { rate, level, pan, spread, .. } => ok(&[rate, level, pan, spread]),
        SoundCmd::Grain { pan, gain, delay, .. } => ok(&[pan, gain, delay]),
        SoundCmd::Burst { rate, secs, pan, spread, gain, .. } => ok(&[rate, secs, pan, spread, gain]),
        SoundCmd::Alert { weight, pan, .. } => ok(&[weight, pan]),
        SoundCmd::Impact { weight, pan, gain, delay, far } => ok(&[weight, pan, gain, delay, far]),
        SoundCmd::Gun { pan, gain, delay, far, .. } => ok(&[pan, gain, delay, far]),
        SoundCmd::Body { level, pitch, breath, rough, bright, pan, .. } => ok(&[level, pitch, breath, rough, bright, pan]),
        SoundCmd::Lamp { pan, .. } => ok(&[pan]),
        SoundCmd::Hush(_) => true,
        SoundCmd::Master(m) => m.is_finite(),
        SoundCmd::Score { tension, level, .. } => ok(&[tension, level]),
    }
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
            body: [BodyVoice::default(); BODY_VOICES],
            hush: Smoothed { value: 1.0, target: 1.0 },
            master: Smoothed { value: DEFAULT_MASTER * DEFAULT_MASTER, target: DEFAULT_MASTER * DEFAULT_MASTER },
            k_fast: coef(0.05, sr),
            k_slow: coef(0.4, sr),
            k_mid: coef(0.2, sr),
            k_hush: coef(0.1, sr),
            verb_lp: 0.0,
            k_verb_hp: 1.0 - (-std::f32::consts::TAU * VERB_HP_HZ / sr).exp(),
            master_lp: [0.0; 2],
            k_master_hp: 1.0 - (-std::f32::consts::TAU * MASTER_HP_HZ / sr).exp(),
            block: 0,
            score: Score::new(sample_rate, seed),
            stats: SynthStats { limit_min: 1.0, ..SynthStats::default() },
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sr as u32
    }

    pub fn apply(&mut self, cmd: SoundCmd) {
        // **Refuse a command with a NaN or infinite number in it**
        // (2026-11-20, the owner's play: "the sound was working and partway
        // through cut out"). A NaN pan or gain went through every clamp
        // here (`NaN.clamp` is NaN, `NaN <= 1e-5` is false) into a grain,
        // from the grain into the reverb's feedback and the master
        // high-pass, and those never let it go: every sample after was NaN,
        // which the device plays as silence, for the rest of the session.
        if !cmd_finite(&cmd) {
            self.stats.refused += 1;
            return;
        }
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
            SoundCmd::Impact { weight, pan, gain, delay, far } => {
                self.spawn_weighted(GrainKind::Splat, pan, gain, (delay.max(0.0) * self.sr) as u32, true, 255, Some(weight), far);
                // A heavy hit ducks the mould's voices so it still punches.
                for b in self.body.iter_mut() {
                    b.duck = b.duck.min(1.0 - 0.6 * weight.clamp(0.0, 1.0));
                }
            }
            SoundCmd::Gun { voice, pan, gain, delay, far } => self.gun(voice, pan, gain, (delay.max(0.0) * self.sr) as u32, far),
            SoundCmd::Body { slot, level, pitch, breath, rough, bright, pan } => {
                let Some(b) = self.body.get_mut(slot as usize) else { return };
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
            SoundCmd::Score { mood, tension, level, hushed } => self.score.set(mood, tension, level, hushed),
        }
    }

    /// Start a grain, stealing the quietest if the pool is full. `tag` is
    /// the texture index (255 = not a texture), kept for its counter.
    fn spawn(&mut self, kind: GrainKind, pan: f32, gain: f32, wait: u32, from_verb: bool, tag: u8) {
        self.spawn_weighted(kind, pan, gain, wait, from_verb, tag, None, 0.0);
    }

    /// `spawn`, with the recipe shaped by `weight` when given (`weighted`)
    /// and dulled by `far`.
    #[allow(clippy::too_many_arguments)]
    fn spawn_weighted(&mut self, kind: GrainKind, pan: f32, gain: f32, wait: u32, from_verb: bool, tag: u8, weight: Option<f32>, far: f32) {
        if gain <= 1e-5 {
            return;
        }
        let s = recipe(kind, &mut self.rng);
        let s = match weight {
            Some(w) => weighted(s, w),
            None => s,
        };
        self.start(kind, s, pan, gain, wait, from_verb, tag, far);
    }

    /// One shot of a gun: its layers (`gun_layers`), `far` taking the
    /// crack and click down and leaving the report, as distance does.
    fn gun(&mut self, voice: GunVoice, pan: f32, gain: f32, wait: u32, far: f32) {
        if gain <= 1e-5 {
            return;
        }
        let far = far.clamp(0.0, 1.0);
        for (kind, spec, delay) in gun_layers(voice, &mut self.rng) {
            if spec.len <= 0.0 {
                continue;
            }
            let near = match kind {
                GrainKind::Crack | GrainKind::Action => 1.0 - 0.7 * far,
                GrainKind::Punch => 1.0 - 0.4 * far,
                _ => 1.0,
            };
            self.start(kind, spec, pan, gain * near, wait + (delay * self.sr) as u32, true, 255, far);
        }
    }

    /// Start a grain of `spec`. Over its kind's cap of verbs, the oldest
    /// of the kind fades out (`FADE_SAMPLES`) rather than being cut; with
    /// no free slot the quietest grain is taken, a fading one first.
    #[allow(clippy::too_many_arguments)]
    fn start(&mut self, kind: GrainKind, s: Spec, pan: f32, gain: f32, wait: u32, from_verb: bool, tag: u8, far: f32) {
        let kind_tag = kind as u8;
        if from_verb && self.grains.iter().filter(|g| g.active && g.from_verb && g.kind_tag == kind_tag && g.fade == 0.0).count() >= kind_cap(kind) {
            if let Some(old) = self.grains.iter_mut().filter(|g| g.active && g.from_verb && g.kind_tag == kind_tag && g.fade == 0.0).max_by_key(|g| g.t) {
                old.fade = FADE_SAMPLES;
            }
        }
        let i = match self.grains.iter().position(|g| !g.active) {
            Some(i) => i,
            None => {
                self.stats.steals += 1;
                let loud = |g: &Grain| (g.noise * g.noise_env + g.sine * g.sine_env) * if g.fade > 0.0 { 0.01 } else { 1.0 };
                self.grains.iter().enumerate().min_by(|a, b| loud(a.1).total_cmp(&loud(b.1))).map(|(i, _)| i).unwrap_or(0)
            }
        };
        let sr = self.sr;
        let (gl, gr) = pan_gains(pan);
        let mut band = Biquad::default();
        band.set(s.band.max(40.0), s.q, sr);
        // A gentle fixed low-pass (`LP_HZ`, or the recipe's) takes the edge
        // off every grain; a far one is duller (`FAR_LP_HZ`).
        let far = far.clamp(0.0, 1.0);
        let lp_hz = if s.lp_hz > 0.0 { s.lp_hz } else { LP_HZ };
        let lp_hz = lp_hz * (FAR_LP_HZ / lp_hz).min(1.0).powf(far);
        let lp_c = 1.0 - (-std::f32::consts::TAU * lp_hz / sr).exp();
        let decay = |tau: f32| if tau > 0.0 { (-1.0 / (tau * sr)).exp() } else { 0.0 };
        let drop_tau = if s.drop_tau > 0.0 { s.drop_tau } else { 0.02 };
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
            drop_env: 1.0,
            drop_k: decay(drop_tau),
            // A far sound is more room than source.
            verb: (s.verb * (1.0 + 0.8 * far)).min(1.0),
            fade: 0.0,
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
        // A coin: bright, glassy and short (a struck small bar's inharmonic
        // partials, fast decays), so a run of ticks stays a run and not a
        // wash (`AlertKind::Coin`, `Cashed`).
        let coin = [(1.0, 1.0, 0.09), (2.4, 0.45, 0.05), (4.1, 0.3, 0.03), (6.3, 0.12, 0.02), (0.0, 0.0, 0.05)];
        // (frequency, delay seconds, gain, voice: 0 marimba, 1 bell, 2 coin)
        let mut notes: [(f32, f32, f32, u8); 4] = [(0.0, 0.0, 0.0, 0); 4];
        // C major pentatonic from C5, climbing with how much food is joined.
        const PENTA: [f32; 6] = [523.25, 587.33, 659.25, 783.99, 880.0, 1046.5];
        match kind {
            AlertKind::Joined => {
                let k = ((w * (PENTA.len() - 1) as f32).round() as usize).min(PENTA.len() - 1);
                notes[0] = (PENTA[k], 0.0, 0.10 + 0.08 * w, 0);
            }
            AlertKind::Bell => {
                // The end of a dish: a rising triad if it scored, more notes
                // for more stars (`weight` = stars / 3).
                let n = (w * 3.0).round() as usize;
                for (j, f) in [392.0, 493.88, 587.33].iter().enumerate().take(n.max(1)) {
                    notes[j] = (*f, 0.18 * j as f32, if n == 0 { 0.08 } else { 0.18 }, 1);
                }
                // A whole win (sound pass two, 2026-11-20) climbs on to
                // the octave, held: the arpeggio resolves.
                if n >= 3 {
                    notes[3] = (783.99, 0.54, 0.22, 1);
                }
            }
            // A siren: a two-tone wail, high-low-high-low, plain sines
            // (marimba partials with long decays would ring; these are
            // short, so the call reads as a call).
            AlertKind::Siren => {
                for (j, f) in [740.0, 554.37, 740.0, 554.37].iter().enumerate() {
                    notes[j] = (*f, 0.28 * j as f32, 0.10, 0);
                }
            }
            AlertKind::Fallen => {
                notes[0] = (146.8, 0.0, 0.16, 1);
                notes[1] = (110.0, 0.22, 0.16, 1);
            }
            // C major pentatonic from E5, two octaves up as the drop drains
            // (the top's highest partial, 6.3 x E7, stays under 17 kHz).
            AlertKind::Coin => {
                const UP: [f32; 11] = [659.25, 783.99, 880.0, 1046.5, 1174.7, 1318.5, 1568.0, 1760.0, 2093.0, 2349.3, 2637.0];
                let k = ((w * (UP.len() - 1) as f32).round() as usize).min(UP.len() - 1);
                notes[0] = (UP[k], 0.0, 0.07, 2);
            }
            // C6 E6 G6, quick, then a held C7 for a core.
            AlertKind::Cashed => {
                for (j, f) in [1046.5, 1318.5, 1568.0].iter().enumerate() {
                    notes[j] = (*f, 0.06 * j as f32, 0.12, 2);
                }
                if w >= 0.9 {
                    notes[3] = (2093.0, 0.18, 0.11, 1);
                }
            }
            AlertKind::Died => {
                notes[0] = (196.0, 0.0, 0.25, 1);
                notes[1] = (146.8, 0.5, 0.25, 1);
                notes[2] = (98.0, 1.0, 0.3, 1);
            }
        }
        let (gl, gr) = pan_gains(pan * 0.6);
        for &(f, delay, g, voice) in notes.iter().filter(|n| n.2 > 0.0) {
            let partials = match voice {
                1 => bell,
                2 => coin,
                _ => marimba,
            };
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

    /// Clear everything that carries a sample from one frame to the next
    /// (grains, bursts, tones, the mould's voices, the reverb, the room and
    /// the high-passes), keeping the textures' and the master's settings
    /// where they are finite. Called by `render` on a sample that is not
    /// finite.
    fn recover(&mut self) {
        self.stats.recovers += 1;
        let fresh = VoiceBank::new(self.sr as u32, 1);
        self.grains = fresh.grains;
        self.bursts = fresh.bursts;
        self.tones = fresh.tones;
        self.reverb = fresh.reverb;
        (self.room_phase, self.room_lp, self.room_am) = (0.0, 0.0, 0.0);
        self.body = fresh.body;
        self.score.reset();
        (self.verb_lp, self.master_lp) = (0.0, [0.0; 2]);
        let fix = |s: &mut Smoothed, d: Smoothed| {
            if !(s.value.is_finite() && s.target.is_finite()) {
                *s = d;
            }
        };
        for (t, d) in self.textures.iter_mut().zip(fresh.textures) {
            fix(&mut t.rate, d.rate);
            fix(&mut t.level, d.level);
            fix(&mut t.pan, d.pan);
            fix(&mut t.spread, d.spread);
            if !t.habit.is_finite() {
                t.habit = 1.0;
            }
        }
        fix(&mut self.lamp, fresh.lamp);
        fix(&mut self.hush, fresh.hush);
        fix(&mut self.master, fresh.master);
        if !self.lamp_pan.is_finite() {
            self.lamp_pan = 0.0;
        }
    }

    /// The score's level now (0 ..= 2), for the drips it asks for.
    fn score_level(&self) -> f32 {
        self.score.level()
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

            // Grains. `send` is what goes to the reverb (each source's own
            // share, `Spec::verb`).
            let (mut l, mut r, mut send) = (0.0f32, 0.0f32, 0.0f32);
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
                    let f = g.freq * (1.0 + (g.freq_end - 1.0) * life) * (1.0 + g.drop * g.drop_env);
                    g.drop_env *= g.drop_k;
                    y += g.phase.sin() * g.sine * g.sine_env;
                    g.phase += std::f32::consts::TAU * f / sr;
                    if g.phase > std::f32::consts::TAU {
                        g.phase -= std::f32::consts::TAU;
                    }
                    g.sine_env *= g.sine_decay;
                }
                y *= att;
                if g.fade > 0.0 {
                    y *= g.fade / FADE_SAMPLES;
                    g.fade -= 1.0;
                    if g.fade <= 0.0 {
                        g.active = false;
                    }
                }
                g.lp[0] += (y - g.lp[0]) * g.lp_c;
                g.lp[1] += (g.lp[0] - g.lp[1]) * g.lp_c;
                l += g.lp[1] * g.gl;
                r += g.lp[1] * g.gr;
                send += g.lp[1] * g.verb;
                self.stats.energy[g.kind_tag as usize] += (g.lp[1] * g.lp[1] * (g.gl * g.gl + g.gr * g.gr)) as f64;
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
                send += y * TONE_VERB;
                self.stats.energy[MIX_TONES] += (y * y * (t.gl * t.gl + t.gr * t.gr)) as f64;
                t.t += 1;
                if t.t >= t.len {
                    t.active = false;
                }
            }

            // The mould's voice: a sub tone and its two partials under a band
            // of noise, swelling with its breath, roughened by strain.
            for bi in 0..BODY_VOICES {
                let (k_slow, k_mid) = (self.k_slow, self.k_mid);
                let b = &mut self.body[bi];
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
                    let (gl, gr) = pan_gains(pan);
                    l += y * gl;
                    r += y * gr;
                    self.stats.energy[MIX_BODY] += (y * y) as f64;
                }
            }

            // The score: the music bed and the planet's air, under all of
            // it, into the reverb like the rest. A drip it asks for is the
            // dish's own grain.
            let sc = self.score.sample();
            l += sc.l;
            r += sc.r;
            send += sc.send;
            self.stats.energy[MIX_SCORE] += (sc.l * sc.l + sc.r * sc.r) as f64;
            if let Some(pan) = sc.drip {
                let g = 0.05 * self.score_level();
                self.spawn(GrainKind::Drip, pan, g, 0, false, 255);
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

            // The reverb hears each source's send, its lows taken off
            // (`VERB_HP_HZ`). The mould's voice and the room stay dry.
            self.verb_lp += (send - self.verb_lp) * self.k_verb_hp;
            let wet = self.reverb.run((send - self.verb_lp) * VERB_IN);
            let master = self.master.step(self.k_mid, self.k_mid) * MAKEUP;
            self.stats.energy[MIX_ROOM] += (2.0 * air * air + hum * hum) as f64;
            self.stats.energy[MIX_VERB] += (wet[0] * wet[0] + wet[1] * wet[1]) as f64;
            let (mut pre_l, mut pre_r) = ((l + wet[0] + air + hum * hl) * master, (r + wet[1] + air + hum * hr) * master);
            // The master high-pass (`MASTER_HP_HZ`).
            self.master_lp[0] += (pre_l - self.master_lp[0]) * self.k_master_hp;
            self.master_lp[1] += (pre_r - self.master_lp[1]) * self.k_master_hp;
            (pre_l, pre_r) = (pre_l - self.master_lp[0], pre_r - self.master_lp[1]);
            // The second guard (`apply` is the first): a sample that is not
            // finite, from anything the bank itself did, clears every
            // voice and filter and plays a zero, so one bad sample costs a
            // click, never the rest of the session.
            if !(pre_l.is_finite() && pre_r.is_finite()) {
                self.recover();
                (pre_l, pre_r) = (0.0, 0.0);
            }
            let lo = limit(pre_l);
            let ro = limit(pre_r);
            let peak = pre_l.abs().max(pre_r.abs());
            if peak > 0.6 {
                self.stats.limited += 1;
                self.stats.limit_min = self.stats.limit_min.min(lo.abs().max(ro.abs()) / peak);
            }
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

    /// The owner's cut-out (2026-11-20): one NaN must not silence the
    /// session. A command carrying one is refused, and a bank whose state
    /// went NaN anyway clears itself and plays on.
    #[test]
    fn a_nan_never_silences_the_bank_for_good() {
        let shot = SoundCmd::Gun { voice: GunVoice::Rifle, pan: 0.0, gain: 0.8, delay: 0.0, far: 0.0 };
        let mut b = VoiceBank::new(48_000, 3);
        b.apply(SoundCmd::Grain { kind: GrainKind::Hurt, pan: f32::NAN, gain: 0.5, delay: 0.0 });
        b.apply(SoundCmd::Gun { voice: GunVoice::Rifle, pan: 0.0, gain: f32::NAN, delay: 0.0, far: 0.0 });
        b.apply(SoundCmd::Body { slot: 0, level: f32::NAN, pitch: 48.0, breath: 0.25, rough: 0.2, bright: 0.0, pan: 0.0 });
        assert_eq!(b.stats.refused, 3);
        b.apply(shot);
        let x = render(&mut b, 0.5);
        assert!(x.iter().all(|v| v.is_finite()) && rms(&x) > 1e-3);
        // Poison the reverb's feedback, as a NaN that got in would.
        b.reverb.combs[0][0].iter_mut().for_each(|v| *v = f32::NAN);
        b.apply(shot);
        let x = render(&mut b, 0.5);
        assert!(b.stats.recovers >= 1);
        assert!(x.iter().all(|v| v.is_finite()));
        b.apply(shot);
        let x = render(&mut b, 0.5);
        assert!(x.iter().all(|v| v.is_finite()) && rms(&x) > 1e-3, "the bank plays on after it recovers");
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
        // Past the cap the oldest fade out rather than being cut (a cut
        // is a click), and are gone once the fade is done.
        let live = |b: &VoiceBank| b.grains.iter().filter(|g| g.active && g.kind_tag == GrainKind::Thunk as u8 && g.fade == 0.0).count();
        assert_eq!(live(&b), PER_KIND_CAP);
        assert!(b.grains.iter().filter(|g| g.active && g.fade > 0.0).count() == 10 - PER_KIND_CAP);
        render(&mut b, 2.0 * FADE_SAMPLES / 48_000.0);
        assert_eq!(b.grains.iter().filter(|g| g.active && g.kind_tag == GrainKind::Thunk as u8).count(), PER_KIND_CAP);
    }

    /// A grain taken over its kind's cap fades: no sample jumps by more
    /// than the loudest grain could move in one sample's fade step.
    #[test]
    fn a_stolen_grain_fades_instead_of_clicking() {
        let mut b = VoiceBank::new(48_000, 5);
        b.apply(SoundCmd::Master(1.0));
        let mut x = Vec::new();
        for _ in 0..12 {
            b.apply(SoundCmd::Grain { kind: GrainKind::Thud, pan: 0.0, gain: 1.0, delay: 0.0 });
            x.extend(render(&mut b, 0.01));
        }
        let jump = x.windows(4).map(|w| (w[2] - w[0]).abs()).fold(0.0f32, f32::max);
        assert!(jump < 0.5, "a sample jumped by {jump}");
    }

    /// Every gun makes a bounded sound; the long gun rings on longest and
    /// the carbine stops soonest; a far shot is duller than a near one.
    #[test]
    fn each_gun_has_its_own_report_and_far_is_duller() {
        let shot = |voice: GunVoice, far: f32| {
            let mut b = VoiceBank::new(48_000, 17);
            b.apply(SoundCmd::Gun { voice, pan: 0.0, gain: 0.5, delay: 0.0, far });
            let x = render(&mut b, 1.2);
            assert!(x.iter().all(|v| v.is_finite() && v.abs() <= 1.0), "{voice:?}");
            let left: Vec<f32> = x.iter().step_by(2).copied().collect();
            let peak = left.iter().fold(0.0f32, |m, v| m.max(v.abs()));
            let tail = left.iter().rposition(|v| v.abs() > peak * 0.1).unwrap_or(0) as f32 / 48_000.0;
            // Brightness: the share of the first 50 ms's change a sample
            // (a rough high-frequency measure).
            let head = &left[..2_400];
            let bright = head.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>() / head.iter().map(|v| v.abs()).sum::<f32>().max(1e-9);
            (tail, bright, rms(&x))
        };
        for v in GunVoice::ALL {
            assert!(shot(v, 0.0).2 > 0.005, "{v:?} made no sound");
        }
        let (long, _, _) = shot(GunVoice::Long, 0.0);
        let (carbine, _, _) = shot(GunVoice::Carbine, 0.0);
        assert!(long > carbine * 2.0, "long gun {long} s vs carbine {carbine} s");
        let (_, near, _) = shot(GunVoice::Rifle, 0.0);
        let (_, far, _) = shot(GunVoice::Rifle, 1.0);
        assert!(far < near * 0.7, "far {far} vs near {near}");
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
            b.apply(SoundCmd::Impact { weight, pan: 0.0, gain: 0.5, delay: 0.0, far: 0.0 });
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
        let body = SoundCmd::Body { slot: 0, level: 0.3, pitch: 50.0, breath: 0.5, rough: 0.2, bright: 0.0, pan: 0.0 };
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
        b.apply(SoundCmd::Impact { weight: 1.0, pan: 0.0, gain: 0.0, delay: 0.0, far: 0.0 });
        assert!(b.body[0].duck < 0.5, "a heavy hit did not duck the voice");
        render(&mut b, 1.5);
        assert!(b.body[0].duck > 0.95, "the duck did not recover: {}", b.body[0].duck);
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
        for kind in [AlertKind::Joined, AlertKind::Bell, AlertKind::Died, AlertKind::Fallen, AlertKind::Siren, AlertKind::Coin, AlertKind::Cashed] {
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
