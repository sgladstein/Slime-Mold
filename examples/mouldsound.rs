//! **Hear a dish offline**: play a dish with a hand and write what the
//! window would have played to a WAV, with the counts beside it.
//!
//! ```text
//! cargo build --release --example mouldsound
//! target/release/examples/mouldsound seed=1 hand=plan out=plan-s1.wav
//! target/release/examples/mouldsound level=2 seed=3 hand=random from=20 secs=30 out=clip.wav
//! ```
//! Hands: `idle`, `random`, `plan`, and `tour`, which is the plan hand
//! plus every tool once on the mould itself (lamp at 8 s for 3 s, heard as its hum since `Params::burn` is 0 and it
//! burns nothing, salt at
//! 18 s, the knife at 28 s, water at 38 s, on a free dish given water), so
//! one recording has every sound in it.
//!
//! `level=K` plays challenge dish K (1-5) instead of the free dish; `from`
//! and `secs` cut the recording (seconds of game; the dish is still played
//! from the start, silently, up to `from`); `rate` is the sample rate
//! (default 44100, stereo); `fast=1` plays every frame at four times the
//! steps, as Space does.
//!
//! It runs the same `Listener` and `VoiceBank` the window does, one drawn
//! frame (1/60 s) of commands then 1/60 s of samples, so what it writes is
//! what the window plays, sample for sample but for the device's buffer
//! timing. It prints the listener's and the voice bank's counters, the
//! dish's hash at the end, and the listener's cost per frame. (That sound
//! cannot change the game is `listen::tests::listening_never_changes_the_game`.)

use std::time::Instant;

use slime_mould::mould::level::DISHES;
use slime_mould::mould::play::{Idle, Plan, Player, Random};
use slime_mould::mould::{Dish, Params, Tool, STEPS_PER_FRAME, STEPS_PER_SECOND};
use slime_mould::sound::listen::Listener;
use slime_mould::sound::synth::VoiceBank;

fn main() {
    let (mut seed, mut level, mut hand, mut out) = (1u64, 0usize, String::from("plan"), String::from("mould.wav"));
    let (mut from, mut secs, mut rate, mut fast) = (0.0f32, f32::INFINITY, 44_100u32, false);
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').unwrap_or_else(|| panic!("expected key=value, got {a}"));
        match k {
            "seed" => seed = v.parse().unwrap(),
            "level" => level = v.parse().unwrap(),
            "hand" => hand = v.to_string(),
            "out" => out = v.to_string(),
            "from" => from = v.parse().unwrap(),
            "secs" => secs = v.parse().unwrap(),
            "rate" => rate = v.parse().unwrap(),
            "fast" => fast = v == "1",
            _ => panic!("unknown argument {k}"),
        }
    }
    let mut d = match (level, hand.as_str()) {
        (0, "tour") => {
            let mut d = Dish::with_params(seed, Params { wet: true, ..Params::default() });
            d.salt_left = d.salt_left.max(1);
            d.snips_left = d.snips_left.max(1);
            d.water_left = d.water_left.max(1);
            d
        }
        (0, _) => Dish::new(seed),
        _ => Dish::from_level(&DISHES[level - 1], seed),
    };
    let mut p: Box<dyn Player> = match hand.as_str() {
        "idle" => Box::new(Idle),
        "random" => Box::new(Random::new(seed)),
        "plan" => Box::new(Plan::new()),
        "tour" => Box::new(Tour { plan: Plan::new() }),
        _ => panic!("hand: idle, random, plan or tour"),
    };
    let mut listener = Listener::new(&d);
    let mut bank = VoiceBank::new(rate, seed);
    // The whole dish, the storm, and the score's bell.
    let end = Dish::dish_steps() + Dish::storm_steps() + 4 * STEPS_PER_SECOND as u64;
    let steps = STEPS_PER_FRAME * if fast { 4 } else { 1 };
    let per_frame = (rate / 60) as usize;
    let mut samples: Vec<f32> = Vec::new();
    let mut frame = vec![0.0f32; per_frame * 2];
    let (mut listen_ns, mut worst_ns, mut frames) = (0u128, 0u128, 0u64);
    while d.t < end {
        p.act(&mut d);
        for _ in 0..steps {
            d.step();
        }
        let t0 = Instant::now();
        let cmds = listener.hear(&d, 1.0 / 60.0, false);
        let ns = t0.elapsed().as_nanos();
        listen_ns += ns;
        worst_ns = worst_ns.max(ns);
        frames += 1;
        for c in cmds {
            bank.apply(c);
        }
        bank.render(&mut frame, 2);
        let now = frames as f32 / 60.0 * if fast { 4.0 } else { 1.0 };
        if now >= from && now < from + secs * if fast { 4.0 } else { 1.0 } {
            samples.extend_from_slice(&frame);
        }
    }
    slime_mould::wav::write(std::path::Path::new(&out), &samples, 2, rate).expect("write the WAV");
    let peak = samples.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    let rms = (samples.iter().map(|v| v * v).sum::<f32>() / samples.len().max(1) as f32).sqrt();
    let s = listener.stats;
    println!(
        "{out}: {:.1} s, peak {:.3}, rms {:.4} ({:.1} dBFS); walkers left {}; hash={:016x}",
        samples.len() as f32 / 2.0 / rate as f32,
        peak,
        rms,
        20.0 * rms.max(1e-9).log10(),
        d.walkers.len(),
        d.state_hash()
    );
    println!(
        "heard: oats {} salt cells {} snips {} drops {} burned {} joins {} arrivals {} fights {} warnings {} storms {} bells {} deaths {} rebaselines {}",
        s.oats, s.salt, s.snips, s.drops, s.burned, s.joins, s.arrivals, s.fights, s.warnings, s.storms, s.bells, s.deaths, s.rebaselines
    );
    let b = bank.stats;
    println!(
        "played: grains {} (creep {} feed {} starve {}, verbs {}), bursts {}, alerts {}, steals {}",
        b.grains, b.grains_by_texture[0], b.grains_by_texture[1], b.grains_by_texture[2], b.verb_grains, b.bursts, b.alerts, b.steals
    );
    println!("listener: mean {:.1} us/frame, worst {:.1} us over {frames} frames", listen_ns as f64 / frames as f64 / 1000.0, worst_ns as f64 / 1000.0);
}

/// The plan hand, plus each tool once where the mould is thickest.
struct Tour {
    plan: Plan,
}

impl Player for Tour {
    fn name(&self) -> &'static str {
        "tour"
    }

    fn act(&mut self, d: &mut Dish) {
        let sps = STEPS_PER_SECOND as u64;
        let n = d.walkers.len().max(1) as f32;
        let (cx, cy) = d.walkers.iter().fold((0.0, 0.0), |(a, b), w| (a + w.x / n, b + w.y / n));
        match d.t {
            t if t == 8 * sps => d.set_lamp(Some((cx, cy))),
            t if t == 11 * sps => d.set_lamp(None),
            t if t == 18 * sps || t == 28 * sps => {
                let tool = if t == 18 * sps { Tool::Salt } else { Tool::Snip };
                if d.begin_stroke(tool, cx - 14.0, cy - 10.0) {
                    d.drag_stroke(cx + 14.0, cy + 10.0);
                    d.end_stroke();
                }
            }
            t if t == 38 * sps => {
                // On a walker, not the centroid, which can be on a pile.
                let near = d.walkers.iter().min_by(|a, b| (a.x - cx).hypot(a.y - cy).total_cmp(&(b.x - cx).hypot(b.y - cy)));
                if let Some(&w) = near {
                    d.place_water(w.x, w.y);
                }
            }
            _ => self.plan.act(d),
        }
    }
}
