//! Crystal sizes on generated planets: for each planet depth and many run
//! seeds, every crystal's radius (map cells) and charge (a fine cell and
//! total at scale 2). Prints CSV.
use slime_mould::mould::run::Run;

fn main() {
    let seeds: u64 = std::env::args().nth(1).and_then(|v| v.parse().ok()).unwrap_or(40);
    // A second argument: that one run seed alone.
    let only: Option<u64> = std::env::args().nth(2).and_then(|v| v.parse().ok());
    println!("depth,seed,crystal,r,per_cell,total");
    for depth in 1..=slime_mould::mould::run::PLANETS {
        for s in only.map_or(1..=seeds, |o| o..=o) {
            let mut r = Run::new(s);
            r.depth = depth;
            let p = r.planet();
            let m = slime_mould::mould::levelgen::generate(&p);
            for (i, c) in m.items.crystals.iter().enumerate() {
                let fine_r = c.r * 2.0;
                let total = std::f32::consts::PI * fine_r * fine_r * c.charge;
                println!("{depth},{s},{i},{:.2},{:.1},{:.0}", c.r, c.charge, total);
            }
        }
    }
}
