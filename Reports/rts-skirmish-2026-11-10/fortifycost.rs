// The frame cost of the Fortify check in fight_losses (2026-11-11). To run:
// copy to examples/ and `cargo run --release --example fortifycost`.
// Every pile ringed on both moulds at 1x, so play (and the walker counts
// printed) is the same with and without the rings.
use slime_mould::mould::skirmish::{Plan, Skirmish};
use slime_mould::mould::STEPS_PER_SECOND;
const SPS: u64 = STEPS_PER_SECOND as u64;
use std::time::Instant;

fn at50() -> Skirmish {
    let mut sk = Skirmish::new(1, [Some(Plan::Soft), Some(Plan::Keep)]);
    for (k, v) in [("fight_fed", "2"), ("fight_unfed", "0.5"), ("spore_ramp", "0.5")] {
        sk.set(k, v);
    }
    while sk.dish.t < 50 * SPS {
        sk.tick();
        sk.dish.step();
    }
    sk
}

fn main() {
    let n = 10 * SPS;
    let mut base = 0.0;
    let mut dug = 0.0;
    for rep in 0..4 {
        for with in [false, true] {
            let mut s = at50();
            if with {
                // Every pile ringed for both moulds, at 1x so play is unchanged.
                let discs: Vec<_> = s.dish.piles.iter().map(|p| (p.x, p.y, p.r + 6.0, 1.0)).collect();
                s.dish.fortify = discs.clone();
                s.dish.rival.as_mut().unwrap().fortify = discs;
            }
            let t0 = Instant::now();
            for _ in 0..n {
                s.tick();
                s.dish.step();
            }
            let ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
            let w = (s.dish.walkers.len(), s.dish.rival.as_ref().unwrap().walkers.len());
            println!("rep {rep} rings={with} {ms:.3} ms/step walkers={w:?}");
            if rep > 0 {
                if with { dug += ms } else { base += ms }
            }
        }
    }
    println!("mean without {:.3}, with every pile ringed {:.3} ms/step", base / 3.0, dug / 3.0);
}
