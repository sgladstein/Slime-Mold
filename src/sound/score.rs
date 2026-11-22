//! **The score**: a low music bed and each planet's own air under the
//! fight (2026-11-22, the owner: "We need some like background, music or
//! more ambient sound or something but it sounds weird with just like
//! mostly gunshots").
//!
//! Until this the game's whole sound was its events: shots, hits, the
//! slime's breathing drone and the pulse, over a room tone at -40 dB. In a
//! quiet stretch that is near silence, and in a fight it is a wall of
//! gunfire with nothing under it. This is the floor under both, three
//! layers:
//!
//! - **The pad**: four slow, open-voiced chords a planet, each a mode and a
//!   key of its own (`MOODS`), a chord every 14-22 seconds crossfaded over
//!   several, through a soft low-pass. Never a beat or a melody line, so it
//!   cannot fight the guns' rhythm or the pulse's tempo.
//! - **Bells**: now and then a soft glassy note from the chord in play, a
//!   few seconds apart, mostly heard through the reverb.
//! - **The planet's air**: wind for open ground and highlands, a river's
//!   rush in a valley, insects in a jungle, drips and a low rumble in caves
//!   and fens (the drips are the dish's own `GrainKind::Drip`, spawned by
//!   the voice bank).
//!
//! **Bends a ruling, on purpose**: "everything heard is the simulation"
//! (`super` module docs; the owner's rule for the picture, held for sound).
//! A music bed is not a count read off the dish. It is kept as close to the
//! rule as it can be: its one input from the game is `tension`, how near
//! the slime is to the squad (the pulse's own danger, `world::WorldListener`),
//! and the slime's size against its start. Tension opens the pad's filter,
//! raises a low drone and a faint high beating pair, and thins the bells, so
//! the bed darkens as the fight closes in and settles when it eases.
//!
//! Like the rest of the bank it is a pure function of its seed and the
//! commands, with no allocation, so the offline render plays what the window
//! plays. Its cost is about 20 sine-and-cosine pairs a sample (eight pad
//! voices of two detuned oscillators, four bells), with the partials made
//! from them by the double-angle identities, not more calls.

use super::Mood;

/// Pad voice slots: two chords' worth, so one fades out while the next
/// fades in.
const PAD_VOICES: usize = 8;
const BELLS: usize = 4;

/// The score's level under the master, at the Options page's Music 100%.
/// Measured on planets 1 and 2, squads hand, 120 s (`hand=squads at=120
/// sound=F.wav`): the whole mix ran -20 and -18 dBFS, its quietest tenth of
/// 0.4 s windows -24 and -21. At the first cut, 0.055, the score was -37
/// dBFS (17.6 dB under the mix by the mix meter), 13 dB under even the
/// quiet windows: there on paper, gone under a fight. At 0.12 (+6.8 dB)
/// it sits about 6 dB under the quiet windows, 11 dB under the mix: under
/// every shot, heard between them.
pub const SCORE_LEVEL: f32 = 0.12;

/// One planet's music and air.
#[derive(Clone, Copy, Debug)]
pub struct MoodSpec {
    /// The pad's root, Hz (octave 2).
    pub root: f32,
    /// The scale, semitones above the root for degrees 0..7.
    pub mode: [u8; 7],
    /// Each chord's root degree; the voicing stacks `voicing` on it.
    pub chords: [u8; 4],
    /// Degrees over the chord's root: an open voicing.
    pub voicing: [u8; 4],
    /// Seconds a chord holds.
    pub chord_secs: f32,
    /// The pad's upper partials, 0 (pure) ..= 1 (reedy).
    pub bright: f32,
    /// The pad's low-pass, Hz, at no tension.
    pub cutoff: f32,
    /// Bells a second, on average, at no tension.
    pub bells: f32,
    /// The bells' octave over the root (4 = two octaves up).
    pub bell_oct: f32,
    /// The air: band centre Hz, Q, level, and how fast it gusts (Hz).
    pub wind: (f32, f32, f32, f32),
    /// Drips a second (fen, caves).
    pub drips: f32,
    /// Insect chirr level (jungle, meadow at a whisper).
    pub insects: f32,
    /// Low rumble level (caves, the core).
    pub rumble: f32,
}

const IONIAN: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const DORIAN: [u8; 7] = [0, 2, 3, 5, 7, 9, 10];
const PHRYGIAN: [u8; 7] = [0, 1, 3, 5, 7, 8, 10];
const LYDIAN: [u8; 7] = [0, 2, 4, 6, 7, 9, 11];
const MIXOLYDIAN: [u8; 7] = [0, 2, 4, 5, 7, 9, 10];
const AEOLIAN: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];

/// Each mood's music and air, in `Mood` order. Look choices, the thread's
/// to make and his to overrule in play: brighter modes for the early,
/// open planets, darker ones as the biomes close in.
pub const MOODS: [MoodSpec; Mood::COUNT] = [
    // Menu: D dorian, i VII IV i, warm and slow, no air.
    MoodSpec { root: 73.42, mode: DORIAN, chords: [0, 6, 3, 0], voicing: [0, 4, 7, 9], chord_secs: 18.0, bright: 0.5, cutoff: 1400.0, bells: 0.22, bell_oct: 4.0, wind: (0.0, 1.0, 0.0, 0.1), drips: 0.0, insects: 0.0, rumble: 0.0 },
    // Meadow: E major, I vi IV I, light wind, a whisper of insects.
    MoodSpec { root: 82.41, mode: IONIAN, chords: [0, 5, 3, 0], voicing: [0, 4, 7, 8], chord_secs: 16.0, bright: 0.45, cutoff: 1600.0, bells: 0.25, bell_oct: 4.0, wind: (900.0, 0.7, 0.5, 0.08), drips: 0.0, insects: 0.15, rumble: 0.0 },
    // Barrens: A aeolian, i VI VII i, dry hollow wind, sparse.
    MoodSpec { root: 55.0, mode: AEOLIAN, chords: [0, 5, 6, 0], voicing: [0, 4, 7, 11], chord_secs: 22.0, bright: 0.3, cutoff: 1000.0, bells: 0.1, bell_oct: 8.0, wind: (520.0, 2.2, 0.8, 0.05), drips: 0.0, insects: 0.0, rumble: 0.0 },
    // Fen: F aeolian, i iv VI v, still damp air, drips.
    MoodSpec { root: 87.31, mode: AEOLIAN, chords: [0, 3, 5, 4], voicing: [0, 4, 7, 9], chord_secs: 18.0, bright: 0.35, cutoff: 1100.0, bells: 0.18, bell_oct: 4.0, wind: (380.0, 0.9, 0.35, 0.04), drips: 0.6, insects: 0.08, rumble: 0.0 },
    // River valley: G mixolydian, I VII IV I, the river's steady rush.
    MoodSpec { root: 98.0, mode: MIXOLYDIAN, chords: [0, 6, 3, 0], voicing: [0, 4, 7, 9], chord_secs: 17.0, bright: 0.45, cutoff: 1500.0, bells: 0.2, bell_oct: 4.0, wind: (420.0, 0.5, 0.7, 0.02), drips: 0.1, insects: 0.0, rumble: 0.0 },
    // Jungle: C dorian, i IV i VII, thick insects, little wind.
    MoodSpec { root: 65.41, mode: DORIAN, chords: [0, 3, 0, 6], voicing: [0, 4, 8, 9], chord_secs: 16.0, bright: 0.5, cutoff: 1300.0, bells: 0.2, bell_oct: 8.0, wind: (700.0, 0.8, 0.2, 0.06), drips: 0.15, insects: 0.55, rumble: 0.0 },
    // Highlands: B-flat lydian, I II I V, a high howling wind.
    MoodSpec { root: 58.27, mode: LYDIAN, chords: [0, 1, 0, 4], voicing: [0, 4, 7, 9], chord_secs: 20.0, bright: 0.4, cutoff: 1400.0, bells: 0.16, bell_oct: 8.0, wind: (1100.0, 3.0, 0.75, 0.07), drips: 0.0, insects: 0.0, rumble: 0.0 },
    // Caves: C-sharp phrygian, i II i vii, rumble and drips, no wind.
    MoodSpec { root: 69.3, mode: PHRYGIAN, chords: [0, 1, 0, 6], voicing: [0, 4, 7, 11], chord_secs: 22.0, bright: 0.25, cutoff: 800.0, bells: 0.12, bell_oct: 4.0, wind: (0.0, 1.0, 0.0, 0.1), drips: 0.9, insects: 0.0, rumble: 0.6 },
    // The core world: B phrygian, i II v i, rumble and a low dry wind.
    MoodSpec { root: 61.74, mode: PHRYGIAN, chords: [0, 1, 4, 0], voicing: [0, 4, 7, 8], chord_secs: 20.0, bright: 0.35, cutoff: 900.0, bells: 0.1, bell_oct: 4.0, wind: (300.0, 1.4, 0.45, 0.05), drips: 0.2, insects: 0.0, rumble: 0.45 },
];

impl MoodSpec {
    /// Hz of scale degree `d` (may pass 7: the next octave).
    pub fn degree(&self, d: u8) -> f32 {
        let semis = self.mode[(d % 7) as usize] as f32 + 12.0 * (d / 7) as f32;
        self.root * (semis / 12.0).exp2()
    }
}

/// The score's own noise, never the dish's or the grains'.
#[derive(Clone, Copy)]
struct Rng(u64);

impl Rng {
    #[inline]
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    #[inline]
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
    #[inline]
    fn bipolar(&mut self) -> f32 {
        self.unit() * 2.0 - 1.0
    }
}

/// A state-variable band-pass (Chamberlin), cheap and stable at these
/// centres.
#[derive(Clone, Copy, Default)]
struct Svf {
    low: f32,
    band: f32,
}

impl Svf {
    #[inline]
    fn band(&mut self, x: f32, f: f32, q: f32) -> f32 {
        let high = x - self.low - self.band / q;
        self.band += f * high;
        self.low += f * self.band;
        self.band
    }
}

#[derive(Clone, Copy, Default)]
struct PadVoice {
    freq: f32,
    phase: [f32; 2],
    env: f32,
    target: f32,
    /// Per-sample rates toward the target: up (the attack) and down.
    k_up: f32,
    k_down: f32,
    pan: (f32, f32),
    lfo: f32,
    lfo_rate: f32,
}

#[derive(Clone, Copy, Default)]
struct Bell {
    freq: f32,
    phase: [f32; 2],
    env: f32,
    decay: f32,
    t: u32,
    wait: u32,
    pan: (f32, f32),
}

/// What one sample of the score is: dry left and right, the reverb send,
/// and whether a drip should fall now (and where).
pub struct ScoreSample {
    pub l: f32,
    pub r: f32,
    pub send: f32,
    pub drip: Option<f32>,
}

pub struct Score {
    sr: f32,
    rng: Rng,
    mood: Mood,
    /// The level the window asks (the Options page's Music, 0 = off).
    level: f32,
    level_now: f32,
    tension: f32,
    tension_now: f32,
    hushed: bool,
    hush_now: f32,
    pad: [PadVoice; PAD_VOICES],
    chord: usize,
    chord_t: f32,
    bells: [Bell; BELLS],
    lp: [[f32; 2]; 2],
    wind: [Svf; 2],
    gust: f32,
    gust_to: f32,
    insect: Svf,
    insect_ph: f32,
    insect_gate: f32,
    insect_on: bool,
    rumble: [f32; 2],
    drone_ph: f32,
    drone_lfo: f32,
    shimmer_ph: [f32; 2],
    block: u32,
    started: bool,
}

impl Score {
    pub fn new(sample_rate: u32, seed: u64) -> Score {
        Score {
            sr: sample_rate as f32,
            rng: Rng((seed ^ 0x5C0E_A7B1_D3E2_9F41) | 1),
            mood: Mood::Menu,
            level: 0.0,
            level_now: 0.0,
            tension: 0.0,
            tension_now: 0.0,
            hushed: false,
            hush_now: 1.0,
            pad: [PadVoice::default(); PAD_VOICES],
            chord: 0,
            chord_t: 0.0,
            bells: [Bell::default(); BELLS],
            lp: [[0.0; 2]; 2],
            wind: [Svf::default(); 2],
            gust: 0.5,
            gust_to: 0.5,
            insect: Svf::default(),
            insect_ph: 0.0,
            insect_gate: 0.0,
            insect_on: false,
            rumble: [0.0; 2],
            drone_ph: 0.0,
            drone_lfo: 0.0,
            shimmer_ph: [0.0; 2],
            block: 0,
            started: false,
        }
    }

    /// The window's targets: the planet's mood, how tense the fight is
    /// (0 ..= 1), the music level (0 ..= 2, 1 = `SCORE_LEVEL`), and whether
    /// the game is paused (the score plays on, quieter).
    pub fn set(&mut self, mood: Mood, tension: f32, level: f32, hushed: bool) {
        self.tension = tension.clamp(0.0, 1.0);
        self.level = level.clamp(0.0, 2.0);
        self.hushed = hushed;
        if mood != self.mood || !self.started {
            let was = self.started;
            self.mood = mood;
            self.started = true;
            // A new planet: what plays fades out quickly, and the first
            // chord comes in a moment later from the top.
            for v in self.pad.iter_mut() {
                v.target = 0.0;
                v.k_down = 1.0 / (0.8 * self.sr);
            }
            self.chord = 0;
            let secs = MOODS[mood as usize].chord_secs;
            self.chord_t = if was { secs - 1.5 } else { secs };
        }
    }

    /// The level playing now, after its fade (0 ..= 2).
    pub fn level(&self) -> f32 {
        self.level_now * self.hush_now
    }

    pub fn mood(&self) -> Mood {
        self.mood
    }

    /// Clear every voice and filter, keeping the targets (the bank's
    /// `recover`, on a sample that was not finite).
    pub fn reset(&mut self) {
        let mood = self.mood;
        let (level, tension, hushed) = (self.level, self.tension, self.hushed);
        *self = Score::new(self.sr as u32, self.rng.0);
        self.set(mood, tension, level, hushed);
    }

    fn next_chord(&mut self) {
        let spec = MOODS[self.mood as usize];
        let root = spec.chords[self.chord % 4];
        self.chord = (self.chord + 1) % 4;
        // Release what plays.
        let release = 1.0 / (spec.chord_secs * 0.3 * self.sr);
        for v in self.pad.iter_mut() {
            if v.target > 0.0 {
                v.target = 0.0;
                v.k_down = release;
            }
        }
        // The four quietest slots take the new chord.
        let mut order: [usize; PAD_VOICES] = core::array::from_fn(|i| i);
        order.sort_by(|&a, &b| self.pad[a].env.total_cmp(&self.pad[b].env));
        const PANS: [f32; 4] = [-0.15, 0.45, -0.5, 0.2];
        const AMPS: [f32; 4] = [1.0, 0.75, 0.6, 0.45];
        for (j, &slot) in order.iter().take(4).enumerate() {
            let f = spec.degree(root + spec.voicing[j]);
            let th = (PANS[j] + 1.0) * std::f32::consts::FRAC_PI_4;
            let v = &mut self.pad[slot];
            // A slot still sounding keeps its phase; a silent one starts
            // fresh, at a random phase so the voices never line up.
            if v.env < 1e-4 {
                v.phase = [self.rng.unit() * std::f32::consts::TAU, self.rng.unit() * std::f32::consts::TAU];
            }
            v.freq = f;
            v.target = AMPS[j];
            v.k_up = 1.0 / (spec.chord_secs * 0.25 * self.sr);
            v.k_down = release;
            v.pan = (th.cos(), th.sin());
            v.lfo_rate = 0.05 + 0.1 * self.rng.unit();
        }
        self.chord_t = 0.0;
    }

    fn ring_bell(&mut self, spec: &MoodSpec) {
        let Some(slot) = self.bells.iter().position(|b| b.env < 1e-3) else { return };
        // A tone of the chord in play, in the bells' octave.
        let root = spec.chords[(self.chord + 3) % 4];
        let pick = (self.rng.unit() * 4.0) as usize % 4;
        let f = spec.degree(root + spec.voicing[pick] % 7) * spec.bell_oct;
        let pan = self.rng.bipolar() * 0.7;
        let th = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
        let gain = 0.5 + 0.5 * self.rng.unit();
        self.bells[slot] = Bell { freq: f, phase: [0.0; 2], env: gain, decay: (-1.0 / (2.8 * self.sr)).exp(), t: 0, wait: 0, pan: (th.cos(), th.sin()) };
        // Sometimes a second, a fifth or an octave over, a beat later.
        if self.rng.unit() < 0.35 {
            if let Some(s2) = self.bells.iter().position(|b| b.env < 1e-3) {
                let up = if self.rng.unit() < 0.5 { 1.5 } else { 2.0 };
                self.bells[s2] = Bell { freq: f * up, phase: [0.0; 2], env: gain * 0.6, decay: (-1.0 / (2.2 * self.sr)).exp(), t: 0, wait: (0.35 * self.sr) as u32, pan: (th.sin(), th.cos()) };
            }
        }
    }

    /// One stereo sample.
    #[inline]
    pub fn sample(&mut self) -> ScoreSample {
        let sr = self.sr;
        let spec = MOODS[self.mood as usize];
        // Slow controls, every 64 samples.
        self.block += 1;
        if self.block >= 64 {
            self.block = 0;
            let dt = 64.0 / sr;
            let k_t = if self.tension > self.tension_now { dt / 2.0 } else { dt / 6.0 };
            self.tension_now += (self.tension - self.tension_now) * k_t.min(1.0);
            self.level_now += (self.level - self.level_now) * (dt / 1.5).min(1.0);
            // Off is off: the last of the fade snaps to silence.
            if self.level == 0.0 && self.level_now < 0.01 {
                self.level_now = 0.0;
            }
            let hush_to = if self.hushed { 0.6 } else { 1.0 };
            self.hush_now += (hush_to - self.hush_now) * (dt / 0.5).min(1.0);
            self.gust += (self.gust_to - self.gust) * (dt * spec.wind.3 * 4.0).min(1.0);
            if self.rng.unit() < dt * spec.wind.3 * 2.0 {
                self.gust_to = 0.15 + 0.85 * self.rng.unit();
            }
            if self.started {
                self.chord_t += dt;
                if self.chord_t >= spec.chord_secs {
                    self.next_chord();
                }
            }
            // Bells thin out as the fight closes in.
            let rate = spec.bells * (1.0 - 0.7 * self.tension_now);
            if self.rng.unit() < rate * dt {
                self.ring_bell(&spec);
            }
            // Insects chirr in spells.
            if self.rng.unit() < dt * 0.4 {
                self.insect_on = self.rng.unit() < 0.7;
            }
        }
        let level = self.level_now * self.hush_now * SCORE_LEVEL;
        if level == 0.0 {
            return ScoreSample { l: 0.0, r: 0.0, send: 0.0, drip: None };
        }
        let tension = self.tension_now;
        let tau = std::f32::consts::TAU;

        // --- the pad ---
        let (mut pl, mut pr) = (0.0f32, 0.0f32);
        let b = spec.bright;
        for v in self.pad.iter_mut() {
            let k = if v.target > v.env { v.k_up } else { v.k_down };
            v.env += (v.target - v.env) * k;
            if v.env < 1e-5 && v.target == 0.0 {
                continue;
            }
            v.lfo = (v.lfo + tau * v.lfo_rate / sr) % tau;
            let mut y = 0.0;
            // Two oscillators 0.3% apart: the slow beating is the pad's life.
            for (o, det) in [0.9985f32, 1.0015].into_iter().enumerate() {
                let p = v.phase[o];
                let (s, c) = p.sin_cos();
                let s2 = 2.0 * s * c;
                let c2 = 1.0 - 2.0 * s * s;
                let s3 = s * (3.0 - 4.0 * s * s);
                let s4 = 2.0 * s2 * c2;
                y += s + b * (0.45 * s2 + 0.25 * s3 + 0.12 * s4);
                v.phase[o] = (p + tau * v.freq * det / sr) % tau;
            }
            y *= v.env * (0.8 + 0.2 * v.lfo.sin()) * 0.5;
            pl += y * v.pan.0;
            pr += y * v.pan.1;
        }
        // Two one-pole low-passes in a row; tension opens them.
        let cutoff = spec.cutoff * (1.0 + 1.6 * tension);
        let k = 1.0 - (-tau * cutoff / sr).exp();
        self.lp[0][0] += (pl - self.lp[0][0]) * k;
        self.lp[0][1] += (self.lp[0][0] - self.lp[0][1]) * k;
        self.lp[1][0] += (pr - self.lp[1][0]) * k;
        self.lp[1][1] += (self.lp[1][0] - self.lp[1][1]) * k;
        let (mut l, mut r) = (self.lp[0][1] * 0.55, self.lp[1][1] * 0.55);

        // --- tension: a low drone on the root and a faint beating pair ---
        if tension > 0.01 {
            self.drone_lfo = (self.drone_lfo + tau * 0.22 / sr) % tau;
            self.drone_ph = (self.drone_ph + tau * spec.root / sr) % tau;
            let d = self.drone_ph.sin() + 0.3 * (2.0 * self.drone_ph).sin();
            let drone = d * tension * (0.7 + 0.3 * self.drone_lfo.sin()) * 0.35;
            let f = spec.root * 16.0;
            self.shimmer_ph[0] = (self.shimmer_ph[0] + tau * f / sr) % tau;
            self.shimmer_ph[1] = (self.shimmer_ph[1] + tau * f * 1.0595 / sr) % tau;
            let sh = tension * tension * 0.035;
            l += drone + self.shimmer_ph[0].sin() * sh;
            r += drone + self.shimmer_ph[1].sin() * sh;
        }

        // --- bells ---
        let mut bell = 0.0f32;
        let (mut bl, mut br) = (0.0f32, 0.0f32);
        for bv in self.bells.iter_mut() {
            if bv.env < 1e-3 {
                continue;
            }
            if bv.wait > 0 {
                bv.wait -= 1;
                continue;
            }
            let att = (bv.t as f32 / (0.006 * sr)).min(1.0);
            let y = (bv.phase[0].sin() + 0.25 * bv.phase[1].sin() * bv.env) * bv.env * att * 0.35;
            bv.phase[0] = (bv.phase[0] + tau * bv.freq / sr) % tau;
            bv.phase[1] = (bv.phase[1] + tau * bv.freq * 2.756 / sr) % tau;
            bv.env *= bv.decay;
            bv.t += 1;
            bl += y * bv.pan.0;
            br += y * bv.pan.1;
            bell += y;
        }
        l += bl * 0.6;
        r += br * 0.6;

        // --- the planet's air ---
        let (wf, wq, wl, _) = spec.wind;
        if wl > 0.0 {
            let f = 2.0 * (std::f32::consts::PI * wf * (0.7 + 0.6 * self.gust) / sr).sin();
            let g = wl * (0.25 + 0.75 * self.gust * self.gust) * 0.5;
            l += self.wind[0].band(self.rng.bipolar(), f, wq) * g;
            r += self.wind[1].band(self.rng.bipolar(), f, wq) * g;
        }
        if spec.insects > 0.0 {
            self.insect_ph = (self.insect_ph + tau * 23.0 / sr) % tau;
            let to = if self.insect_on { 1.0 } else { 0.0 };
            self.insect_gate += (to - self.insect_gate) * (1.0 / (0.6 * sr));
            let am = (self.insect_ph.sin() * 0.5 + 0.5).powi(3);
            let f = 2.0 * (std::f32::consts::PI * 4600.0 / sr).sin();
            let y = self.insect.band(self.rng.bipolar(), f, 9.0) * am * self.insect_gate * spec.insects * 0.5;
            l += y * 0.7;
            r += y;
        }
        if spec.rumble > 0.0 {
            let k = 1.0 - (-tau * 70.0 / sr).exp();
            self.rumble[0] += (self.rng.bipolar() - self.rumble[0]) * k;
            self.rumble[1] += (self.rumble[0] - self.rumble[1]) * k;
            let y = self.rumble[1] * spec.rumble * (0.6 + 0.4 * self.gust) * 6.0;
            l += y;
            r += y;
        }
        let drip = (spec.drips > 0.0 && self.rng.unit() < spec.drips / sr).then(|| self.rng.bipolar() * 0.8);

        ScoreSample { l: l * level, r: r * level, send: ((l + r) * 0.25 + bell * 0.8) * level, drip }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(s: &mut Score, secs: f32) -> (f32, usize) {
        let n = (secs * s.sr) as usize;
        let (mut e, mut drips) = (0.0f64, 0);
        for _ in 0..n {
            let x = s.sample();
            assert!(x.l.is_finite() && x.r.is_finite() && x.send.is_finite());
            e += (x.l * x.l + x.r * x.r) as f64;
            drips += x.drip.is_some() as usize;
        }
        (((e / n as f64) as f32).sqrt(), drips)
    }

    #[test]
    fn every_mood_plays_and_music_zero_is_silent() {
        for m in Mood::ALL {
            let mut s = Score::new(48_000, 7);
            s.set(m, 0.3, 1.0, false);
            // The first chord comes in at once.
            let (rms, _) = play(&mut s, 8.0);
            assert!(rms > 1e-4 && rms < 0.2, "{m:?} rms {rms}");
            s.set(m, 0.3, 0.0, false);
            play(&mut s, 8.0);
            let (rms, _) = play(&mut s, 1.0);
            assert!(rms == 0.0, "{m:?} at music 0: {rms}");
        }
    }

    #[test]
    fn tension_brings_the_drone_and_caves_drip() {
        let mut calm = Score::new(48_000, 3);
        let mut tense = Score::new(48_000, 3);
        calm.set(Mood::Caves, 0.0, 1.0, false);
        tense.set(Mood::Caves, 1.0, 1.0, false);
        let (a, drips) = play(&mut calm, 10.0);
        let (b, _) = play(&mut tense, 10.0);
        assert!(b > a * 1.2, "tension adds weight: {a} vs {b}");
        assert!(drips > 2, "caves drip: {drips}");
    }

    /// Its cost on the audio thread, printed (`--nocapture`): a second of
    /// the busiest mood, with tension, against a second of real time.
    #[test]
    fn a_second_of_score_is_cheap() {
        let mut s = Score::new(48_000, 5);
        s.set(Mood::Jungle, 0.8, 1.0, false);
        play(&mut s, 2.0);
        let t0 = std::time::Instant::now();
        play(&mut s, 1.0);
        let ms = t0.elapsed().as_secs_f32() * 1e3;
        println!("score: {ms:.2} ms for a second of sound");
        assert!(ms < 200.0, "{ms} ms");
    }

    #[test]
    fn the_same_seed_plays_the_same() {
        let run = || {
            let mut s = Score::new(48_000, 11);
            s.set(Mood::Jungle, 0.5, 1.0, false);
            (0..48_000).map(|_| s.sample().l).collect::<Vec<f32>>()
        };
        assert_eq!(run(), run());
    }
}
