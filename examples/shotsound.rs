//! **Hear Larder World's shot weights side by side**: one shot a second
//! through the game's own listener (`sound::world`) and voice bank, killing
//! more walkers each time, so the ear can judge whether a big kill sounds
//! bigger. Written for the owner's listening page
//! (`Reports/larder-sound-2026-11-12.md`).
//!
//! ```text
//! cargo build --release --example shotsound
//! target/release/examples/shotsound out=ladder.wav [kills=0,1,5,15,37,56,80] [seed=1]
//! ```
//!
//! A kill count of 0 is a shot that struck slime and killed nothing; `-1`
//! is one a wall or rock stopped (a crack, no impact).
//!
//! `guns=1` plays the guns' voices instead (`SoundCmd::Gun`, 2026-11-20):
//! each kind three times on screen, then three times a view off it, then a
//! squad of four firing it together twice. The shots come from
//! the left of the view and land in its middle. The game round under them
//! is a fresh one, never stepped, so nothing else is heard but the room.

use slime_mould::mould::larder_world::{race_world, Game, Rules, Shot};
use slime_mould::sound::synth::VoiceBank;
use slime_mould::sound::world::{full_kill, kill_weight, Ear, WorldListener, REF_DENSITY};

fn main() {
    let (mut out, mut kills, mut seed, mut guns) = (String::from("ladder.wav"), vec![-1i32, 0, 1, 5, 15, 37, 56, 80], 1u64, false);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').unwrap_or_else(|| panic!("expected key=value, got {a}"));
        match k {
            "out" => out = v.to_string(),
            "kills" => kills = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "seed" => seed = v.parse().unwrap(),
            "guns" => guns = v == "1",
            _ => panic!("unknown argument {k} (out, kills, seed, guns)"),
        }
    }
    if guns {
        return gun_ladder(&out, seed);
    }
    let mut g = Game::new(race_world(seed, 1, 1_000), Rules::default(), seed);
    // The ladder is heard at the reference density (`REF_DENSITY`, where
    // `FULL_KILL` was measured), whatever this small world's.
    let (w, h) = slime_mould::mould::larder_world::World::size(&g.world);
    let sc = slime_mould::mould::larder_world::World::scale(&g.world);
    g.start_mould = (REF_DENSITY * (w as f32 / sc) * (h as f32 / sc)).round() as usize;
    let full = full_kill(&g);
    let mut l = WorldListener::new(&g);
    let mut bank = VoiceBank::new(48_000, seed);
    let ear = Ear { centre: (100.0, 100.0), half: (80.0, 50.0) };
    let frame = 1.0 / 60.0;
    let mut samples = Vec::new();
    let mut buf = vec![0.0f32; 800 * 2];
    for (i, &k) in kills.iter().chain([&i32::MIN]).enumerate() {
        for f in 0..60 {
            if f == 0 && k != i32::MIN {
                let hit = (k >= 0).then_some((100.0, 100.0));
                l.shots(&[Shot { from: (40.0, 100.0), to: (110.0, 100.0), hit, killed: k.max(0) as u32, by: Some(0), turret: None, fired: true, cast: 0, r: 2.0 }]);
                if i < kills.len() {
                    println!("{:>4.1} s: {}", samples.len() as f32 / 2.0 / 48_000.0, if k < 0 { "stopped by rock: crack only".to_string() } else { format!("{k} killed, weight {:.2}", kill_weight(k as u32, full)) });
                }
            }
            for c in l.hear(&g, ear, frame, false) {
                bank.apply(c);
            }
            bank.render(&mut buf, 2);
            samples.extend_from_slice(&buf);
        }
    }
    slime_mould::wav::write(std::path::Path::new(&out), &samples, 2, 48_000).expect("write the WAV");
    let peak = samples.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    println!("{out}: {:.1} s, peak {peak:.3}; impacts {} (light {} mid {} heavy {})", samples.len() as f32 / 2.0 / 48_000.0, l.stats.impacts, l.stats.light, l.stats.mid, l.stats.heavy);
}

/// `guns=1`: each gun's voice near, far, and as a squad's volley.
fn gun_ladder(out: &str, seed: u64) {
    use slime_mould::sound::{GunVoice, SoundCmd};
    let sr = 48_000u32;
    let mut bank = VoiceBank::new(sr, seed);
    let mut samples = Vec::new();
    let mut play = |bank: &mut VoiceBank, cmds: &[SoundCmd], secs: f32| {
        for &c in cmds {
            bank.apply(c);
        }
        let mut buf = vec![0.0f32; (secs * sr as f32) as usize * 2];
        bank.render(&mut buf, 2);
        samples.extend_from_slice(&buf);
    };
    let mut t = 0.0f32;
    for voice in GunVoice::ALL {
        let gain = if voice == GunVoice::Turret { 0.42 * 0.5 } else { 0.42 };
        println!("{t:>5.1} s: {voice:?} near x3, far x3, a squad of four x2");
        for far in [0.0f32, 0.55] {
            for _ in 0..3 {
                play(&mut bank, &[SoundCmd::Gun { voice, pan: -0.3, gain: gain * if far > 0.0 { 0.45 } else { 1.0 }, delay: 0.0, far }], 0.7);
                t += 0.7;
            }
        }
        for _ in 0..2 {
            let volley: Vec<SoundCmd> = (0..4).map(|k| SoundCmd::Gun { voice, pan: -0.4 + 0.25 * k as f32, gain, delay: 0.004 * k as f32, far: 0.0 }).collect();
            play(&mut bank, &volley, 1.0);
            t += 1.0;
        }
        play(&mut bank, &[], 0.6);
        t += 0.6;
    }
    slime_mould::wav::write(std::path::Path::new(out), &samples, 2, sr).expect("write the WAV");
    let peak = samples.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    println!("{out}: {:.1} s, peak {peak:.3}; mix {:?}", samples.len() as f32 / 2.0 / sr as f32, bank.stats.mix(-30.0));
}
