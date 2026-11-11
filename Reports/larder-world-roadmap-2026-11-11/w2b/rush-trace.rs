// Scratch: where rush shots land (not committed).
use slime_mould::mould::larder_world::{race_world, Cmd, Game, Order, Rules};
fn main() {
    let seed: u64 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(1);
    let aim_min: u32 = std::env::args().nth(2).map(|s| s.parse().unwrap()).unwrap_or(1);
    let mut g = Game::new(race_world(seed, 2, 5000), Rules { aim_min, ..Rules::default() }, seed);
    for k in 0..g.soldiers.len() {
        g.apply(Cmd::Order(k, Order::Attack));
    }
    let mut d_hist = [0u32; 8];
    let mut k_hist = [0u32; 6];
    let (mut n, mut none) = (0, 0);
    while g.end.is_none() && g.time() < 200.0 {
        g.step();
        if g.world.ticks % (15 * 10) == 0 {
            let ss: Vec<String> = g.soldiers.iter().filter(|s| s.alive).map(|s| format!("({:.0},{:.0}) hp {:.0} t {} d {} aim {:?}", s.x, s.y, s.hp, s.touching, s.dense, s.aim.map(|a| (a.0 as i32, a.1 as i32)))).collect();
            println!("{:.0}s core ({:.0},{:.0}) down {:?} shots {} none {}: {}", g.time(), g.core.0, g.core.1, g.core_down, n, none, ss.join(" | "));
        }
        for s in &g.last_shots {
            n += 1;
            match s.hit {
                None => none += 1,
                Some(h) => {
                    let d = (h.0 - s.from.0).hypot(h.1 - s.from.1);
                    d_hist[((d / 4.0) as usize).min(7)] += 1;
                    k_hist[(s.killed as usize).min(5)] += 1;
                }
            }
        }
    }
    println!("seed {seed} aim_min {aim_min}: t {:.0} alive {} shots {n} none {none} killed {}", g.time(), g.alive(), g.killed);
    println!("hit distance from shooter (fine cells, bins of 4): {:?}", d_hist);
    println!("kills a shot (0..5+): {:?}", k_hist);
}
