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
//! is one a wall or rock stopped (a crack, no impact). The shots come from
//! the left of the view and land in its middle. The game round under them
//! is a fresh one, never stepped, so nothing else is heard but the room.

use slime_mould::mould::larder_world::{race_world, Game, Rules, Shot};
use slime_mould::sound::synth::VoiceBank;
use slime_mould::sound::world::{kill_weight, Ear, WorldListener};

fn main() {
    let (mut out, mut kills, mut seed) = (String::from("ladder.wav"), vec![-1i32, 0, 1, 5, 15, 37, 56, 80], 1u64);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').unwrap_or_else(|| panic!("expected key=value, got {a}"));
        match k {
            "out" => out = v.to_string(),
            "kills" => kills = v.split(',').map(|s| s.parse().unwrap()).collect(),
            "seed" => seed = v.parse().unwrap(),
            _ => panic!("unknown argument {k} (out, kills, seed)"),
        }
    }
    let g = Game::new(race_world(seed, 1, 1_000), Rules::default(), seed);
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
                l.shots(&[Shot { from: (40.0, 100.0), to: (110.0, 100.0), hit, killed: k.max(0) as u32 }]);
                if i < kills.len() {
                    println!("{:>4.1} s: {}", samples.len() as f32 / 2.0 / 48_000.0, if k < 0 { "stopped by rock: crack only".to_string() } else { format!("{k} killed, weight {:.2}", kill_weight(k as u32)) });
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
