//! **Each of the cloud plan's C7 sounds alone** (2026-11-14), through the
//! game's voice bank, so the ear can judge them one at a time before
//! hearing them in a round: a bullet's zip, the siren's call, a kit blast,
//! a piece tearing off, a scared piece's charge (the snarl), a piece
//! withering out, a piece rejoining, the withering texture rising, and a
//! piece's own voice beside the body's (calm, withering, charging).
//! Written for the owner's listening page
//! (`Reports/larder-sound-2026-11-12.md`, section C7).
//!
//! ```text
//! cargo build --release --example c7sounds
//! target/release/examples/c7sounds out=c7.wav [seed=1]
//! ```
//!
//! The commands and their levels are the listener's own
//! (`sound::world::WorldListener`); the inputs are made up, one at a time.

use slime_mould::sound::synth::VoiceBank;
use slime_mould::sound::{AlertKind, GrainKind, SoundCmd, TextureKind};

fn main() {
    let (mut out, mut seed) = (String::from("c7.wav"), 1u64);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').unwrap_or_else(|| panic!("expected key=value, got {a}"));
        match k {
            "out" => out = v.to_string(),
            "seed" => seed = v.parse().unwrap(),
            _ => panic!("unknown argument {k} (out, seed)"),
        }
    }
    let mut bank = VoiceBank::new(48_000, seed);
    let frame = 1.0 / 60.0;
    let mut samples = Vec::new();
    let mut buf = vec![0.0f32; 800 * 2];
    let grain = |kind, gain, pan, delay| SoundCmd::Grain { kind, pan, gain, delay };
    let body = |slot, level, pitch, breath, rough, pan| SoundCmd::Body { slot, level, pitch, breath, rough, bright: 0.0, pan };
    // (seconds, what, the commands at its start; a closure for the ones
    // that run every frame).
    type Each = Box<dyn Fn(f32) -> Vec<SoundCmd>>;
    let parts: Vec<(f32, &str, Vec<SoundCmd>, Option<Each>)> = vec![
        (2.0, "three bullets whizz past (zip), left to right", vec![grain(GrainKind::Crack, 0.4, -0.8, 0.0), grain(GrainKind::Zip, 0.22, -0.2, 0.25), grain(GrainKind::Crack, 0.4, -0.8, 0.5), grain(GrainKind::Zip, 0.22, 0.0, 0.75), grain(GrainKind::Crack, 0.4, -0.8, 1.0), grain(GrainKind::Zip, 0.22, 0.2, 1.25)], None),
        (3.0, "a siren calls", vec![SoundCmd::Alert { kind: AlertKind::Siren, weight: 1.0, pan: 0.3 }], None),
        (2.5, "a kit blast that killed 40 walkers", vec![grain(GrainKind::Thud, 0.8, 0.0, 0.0), SoundCmd::Impact { weight: 0.7, pan: 0.0, gain: 0.61, delay: 0.02 }, SoundCmd::Burst { kind: GrainKind::Grit, rate: 120.0, secs: 0.25, pan: 0.0, spread: 0.3, gain: 0.3 }], None),
        (2.0, "a piece tears off the body", vec![grain(GrainKind::Tear, 0.5, -0.3, 0.0)], None),
        (2.0, "a scared piece charges (the snarl)", vec![grain(GrainKind::Snarl, 0.5, 0.3, 0.0)], None),
        (2.0, "a piece withers out", vec![SoundCmd::Burst { kind: GrainKind::Wilt, rate: 60.0, secs: 0.4, pan: 0.2, spread: 0.1, gain: 0.25 }], None),
        (2.0, "a piece rejoins the body", vec![grain(GrainKind::Squelch, 0.5, -0.2, 0.0), grain(GrainKind::Squelch, 0.35, -0.2, 0.08)], None),
        (
            5.0,
            "withering rises: 0 to 40 walkers a second lost, then stops",
            vec![],
            Some(Box::new(|t: f32| {
                let lost = if t < 4.0 { 40.0 * t / 4.0 } else { 0.0 };
                let (rate, level) = slime_mould::sound::listen::density(lost, 1.0, 40.0, 0.18);
                vec![SoundCmd::Texture { kind: TextureKind::Wither, rate, level, pan: 0.0, spread: 0.2 }]
            })),
        ),
        (
            12.0,
            "the body's voice, then a 250-walker piece's beside it: calm 0-4 s, withering 4-8 s, charging 8-12 s",
            vec![],
            Some(Box::new(move |t: f32| {
                let mut c = vec![body(0, 0.14, 48.0, 0.25, 0.25, -0.3)];
                let level = 0.6 * 0.18 * ((1.0 + 250.0f32 / 100.0).log10() / 51f32.log10());
                let (wither, charge) = if t < 4.0 { (0.0, 0.0) } else if t < 8.0 { ((t - 4.0) / 4.0, 0.0) } else { (0.0, ((t - 8.0) / 2.0).min(1.0)) };
                let pitch = 48.0 * 1.62 * (1.0 - 0.15 * wither + 0.5 * charge);
                let breath = 0.25 * 2.0 * (1.0 - 0.3 * wither + 3.0 * charge);
                let rough = (0.25 + 0.65 * wither + 0.3 * charge).min(1.0);
                c.push(body(1, level * (1.0 + 0.6 * charge), pitch, breath, rough, 0.4));
                c
            })),
        ),
    ];
    for (secs, what, cmds, each) in &parts {
        println!("{:>5.1} s: {what}", samples.len() as f32 / 2.0 / 48_000.0);
        for c in cmds {
            bank.apply(*c);
        }
        let n = (secs / frame) as usize;
        for f in 0..n {
            if let Some(e) = each {
                for c in e(f as f32 * frame) {
                    bank.apply(c);
                }
            }
            bank.render(&mut buf, 2);
            samples.extend_from_slice(&buf);
        }
        // Quiet the continuing voices before the next part.
        bank.apply(SoundCmd::Texture { kind: TextureKind::Wither, rate: 0.0, level: 0.0, pan: 0.0, spread: 0.0 });
        for slot in 0..2 {
            bank.apply(body(slot, 0.0, 48.0, 0.25, 0.25, 0.0));
        }
    }
    for _ in 0..60 {
        bank.render(&mut buf, 2);
        samples.extend_from_slice(&buf);
    }
    slime_mould::wav::write(std::path::Path::new(&out), &samples, 2, 48_000).expect("write the WAV");
    let peak = samples.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    println!("{out}: {:.1} s, peak {peak:.3}", samples.len() as f32 / 2.0 / 48_000.0);
}
