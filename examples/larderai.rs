//! **The Larder player AI's harness** (`slime_mould::mould::larder_ai`).
//!
//! ```text
//! cargo run --release --example larderai -- census seeds=1-16 [lean=1] [hands=adapt,rush] [set=price=20,hp=150]
//! cargo run --release --example larderai -- look=DIR seeds=1 hand=adapt [lean=1]
//! cargo run --release --example larderai -- replays=DIR seeds=1-4 [hands=...]
//! ```
//!
//! `census` plays every hand (idle, the random hand, and each preset
//! style) on every seed and prints, per hand, the score's spread, the
//! round's ends, and what it spent its food on; then the best style per
//! seed, and paired sign tests (each style against idle and random, and
//! each pair of styles) with Holm's correction over all of them.
//! `set=` changes rules by name (`larder_ai::set_rule`) for a balance test;
//! `lean=1` cuts every pile but the larder to a quarter (M2's lean field).
//! `look=` saves a frame a game minute (`trace=SECS` adds a line of what
//! the squad is doing every SECS game seconds); `replays=` writes each round's
//! command log, which `larder replay FILE` plays back to the same hash.
//!
//! Scores are `End::score`: a squad win is in (1, 3], a stalemate at the
//! cap in [-0.5, 0.5], a loss in [-2, -1).
use slime_mould::mould::larder::{field_game, Game, Rules, GAME_SECOND};
use slime_mould::mould::larder_ai::{set_rule, Hand, Style};

const LEAN: f32 = 0.25;

#[derive(Clone, Debug, Default)]
struct Out {
    score: f32,
    band: &'static str,
    end_t: f32,
    left: usize,
    most: usize,
    bought: u32,
    turrets: usize,
    baited: f32,
    shots: u32,
    harvested: f32,
    cmds: usize,
    core_down: Option<f32>,
    walkers: usize,
    think_us: f64,
}

fn arg<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|a| a.strip_prefix(k).and_then(|r| r.strip_prefix('=')))
}

fn seeds(s: Option<&str>) -> Vec<u64> {
    let s = s.unwrap_or("1-16");
    if let Some((a, b)) = s.split_once('-') {
        (a.parse().unwrap()..=b.parse().unwrap()).collect()
    } else {
        s.split(',').map(|v| v.parse().unwrap()).collect()
    }
}

fn game(seed: u64, rules: Rules, lean: bool) -> Game {
    let mut g = field_game(seed, rules);
    if lean {
        let li = g.pile_at(g.larder).expect("larder");
        for (k, p) in g.dish.piles.iter_mut().enumerate() {
            if k != li {
                p.food *= LEAN;
                p.full *= LEAN;
            }
        }
    }
    g
}

/// `trace=SECS` with `look=`: a line every SECS game seconds of what the
/// squad is doing.
static TRACE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn trace(g: &Game) {
    use slime_mould::mould::larder::Order;
    let (mut clear, mut burn, mut walk, mut attack, mut hold) = (0, 0, 0, 0, 0);
    let mut hp = Vec::new();
    for s in g.soldiers.iter().filter(|s| s.alive) {
        match s.order {
            Order::Clear(..) => clear += 1,
            Order::Burn(..) => burn += 1,
            Order::Move(..) => walk += 1,
            Order::Attack => attack += 1,
            Order::Hold => hold += 1,
        }
        hp.push(s.hp as i32);
    }
    let dead = g.soldiers.len() - g.alive();
    println!(
        "  {:4.0} s: walkers {:5}  alive {:2} dead {:2}  bank {:6.0}  clear {clear} burn {burn} walk {walk} attack {attack} hold {hold}  hp {hp:?}",
        g.time(),
        g.dish.walkers.len(),
        g.alive(),
        dead,
        g.bank
    );
}

fn run(seed: u64, hand: &str, rules: Rules, lean: bool, look: Option<&str>, replays: Option<&str>) -> Out {
    let mut g = game(seed, rules, lean);
    let mut h = Hand::named(hand, &g, seed).unwrap_or_else(|| panic!("no hand {hand}"));
    let mut o = Out::default();
    let mut think = 0.0f64;
    let mut thinks = 0u32;
    let tag = if lean { "lean" } else { "fed" };
    while g.end.is_none() {
        if g.dish.t.is_multiple_of(GAME_SECOND) {
            let t0 = std::time::Instant::now();
            o.cmds += h.play(&mut g);
            think += t0.elapsed().as_secs_f64();
            thinks += 1;
            o.most = o.most.max(g.alive());
            o.turrets = o.turrets.max(g.turrets.len());
            let every = TRACE.load(std::sync::atomic::Ordering::Relaxed);
            if every > 0 && g.dish.t.is_multiple_of(every * GAME_SECOND) {
                trace(&g);
            }
            if let Some(dir) = look {
                if g.dish.t.is_multiple_of(60 * GAME_SECOND) {
                    println!(
                        "  {:3} s: walkers {:5}  soldiers {:2}  turrets {}  bank {:6.0}",
                        g.dish.t / GAME_SECOND,
                        g.dish.walkers.len(),
                        g.alive(),
                        g.turrets.len(),
                        g.bank
                    );
                    save(
                        &g,
                        &format!("{dir}/ai-s{seed}-{tag}-{hand}-{:03}.png", g.dish.t / GAME_SECOND),
                    );
                }
            }
        }
        g.step();
    }
    if let Some(dir) = look {
        save(&g, &format!("{dir}/ai-s{seed}-{tag}-{hand}-end.png"));
    }
    if let Some(dir) = replays {
        std::fs::write(format!("{dir}/ai-s{seed}-{tag}-{hand}.txt"), g.replay_text(seed)).expect("write replay");
    }
    let end = g.end.expect("ended");
    o.score = g.score().expect("ended");
    o.band = end.band();
    o.end_t = g.time();
    o.left = g.alive();
    o.bought = g.bought;
    o.baited = g.baited;
    o.shots = g.shots;
    o.harvested = g.harvested;
    o.core_down = g.core_down.map(|t| t as f32 / GAME_SECOND as f32);
    o.walkers = g.dish.walkers.len();
    o.think_us = think / thinks.max(1) as f64 * 1e6;
    o
}

fn save(g: &Game, path: &str) {
    use slime_mould::mould::render::{self, View};
    let scale = 3;
    let (w, h) = render::frame_size(scale);
    let mut buf = vec![0u8; (w * h * 4) as usize];
    let view = View {
        keys: false,
        bare: true,
        style: slime_mould::mould::look::Style::GAME,
        ..View::default()
    };
    render::draw(&g.dish, &view, scale, &mut buf);
    let mut stamp = |x: f32, y: f32, r: i32, col: [u8; 4]| {
        let (cx, cy) = ((x * scale as f32) as i32, (y * scale as f32) as i32);
        for dy in -r..=r {
            for dx in -r..=r {
                let (px, py) = (cx + dx, cy + dy);
                if px >= 0 && py >= 0 && (px as u32) < w && (py as u32) < h {
                    let i = ((py as u32 * w + px as u32) * 4) as usize;
                    buf[i..i + 4].copy_from_slice(&col);
                }
            }
        }
    };
    for t in &g.turrets {
        stamp(t.x, t.y, 3, [90, 170, 255, 255]);
    }
    for s in &g.soldiers {
        stamp(
            s.x,
            s.y,
            2,
            if s.alive {
                [255, 255, 255, 255]
            } else {
                [140, 20, 20, 255]
            },
        );
    }
    image::RgbaImage::from_raw(w, h, buf)
        .expect("frame")
        .save(path)
        .expect("save png");
}

fn sign_p(wins: usize, n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let k = wins.min(n - wins);
    let mut c = 1.0f64;
    let mut tail = 0.0f64;
    for i in 0..=k {
        if i > 0 {
            c = c * (n - i + 1) as f64 / i as f64;
        }
        tail += c;
    }
    (2.0 * tail / 2f64.powi(n as i32)).min(1.0)
}

/// Every (seed, hand) round on all cores.
fn par(jobs: &[(u64, String)], f: impl Fn(u64, &str) -> Out + Sync) -> Vec<Out> {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out = std::sync::Mutex::new(vec![Out::default(); jobs.len()]);
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if i >= jobs.len() {
                    break;
                }
                let o = f(jobs[i].0, &jobs[i].1);
                out.lock().unwrap()[i] = o;
            });
        }
    });
    out.into_inner().unwrap()
}

fn median(mut v: Vec<f32>) -> f32 {
    v.sort_by(|a, b| a.total_cmp(b));
    if v.is_empty() {
        f32::NAN
    } else {
        v[v.len() / 2]
    }
}

fn quart(mut v: Vec<f32>) -> (f32, f32) {
    v.sort_by(|a, b| a.total_cmp(b));
    (v[v.len() / 4], v[(v.len() * 3) / 4])
}

fn census(seeds: &[u64], hands: &[String], rules: Rules, lean: bool) {
    let jobs: Vec<(u64, String)> = hands
        .iter()
        .flat_map(|h| seeds.iter().map(move |&s| (s, h.clone())))
        .collect();
    let flat = par(&jobs, |s, h| run(s, h, rules, lean, None, None));
    let res: Vec<(&String, &[Out])> = hands.iter().zip(flat.chunks(seeds.len())).collect();
    println!(
        "{} field, seeds {:?}..{:?} ({} rounds)",
        if lean { "lean" } else { "fed" },
        seeds.first().unwrap(),
        seeds.last().unwrap(),
        seeds.len()
    );
    println!(
        "{:8} {:>7} {:>15}  {:>4} {:>4} {:>4}  {:>5} {:>4} {:>4} {:>6} {:>6} {:>7} {:>5} {:>5}  {:>6}",
        "hand",
        "median",
        "quartiles",
        "win",
        "cap",
        "loss",
        "end s",
        "most",
        "buys",
        "turret",
        "bait",
        "harvest",
        "shots",
        "cmds",
        "think"
    );
    for (h, o) in &res {
        let sc: Vec<f32> = o.iter().map(|x| x.score).collect();
        let (q1, q3) = quart(sc.clone());
        let n = |b: &str| o.iter().filter(|x| x.band == b).count();
        let m = |f: &dyn Fn(&Out) -> f32| median(o.iter().map(f).collect());
        println!(
            "{:8} {:7.3} {:7.3}..{:<7.3}  {:4} {:4} {:4}  {:5.0} {:4.0} {:4.0} {:6.0} {:6.0} {:7.0} {:5.0} {:5.0}  {:5.0}us",
            h,
            median(sc),
            q1,
            q3,
            n("mould wiped"),
            n("cap"),
            n("squad wiped"),
            m(&|x| x.end_t),
            m(&|x| x.most as f32),
            m(&|x| x.bought as f32),
            m(&|x| x.turrets as f32),
            m(&|x| x.baited),
            m(&|x| x.harvested),
            m(&|x| x.shots as f32),
            m(&|x| x.cmds as f32),
            o.iter().map(|x| x.think_us).sum::<f64>() / o.len() as f64,
        );
    }
    println!("\nPer seed (score):");
    print!("{:5}", "seed");
    for (h, _) in &res {
        print!(" {:>8}", h);
    }
    println!("  best style");
    let styles: Vec<usize> = (0..res.len())
        .filter(|&i| res[i].0 != "idle" && res[i].0 != "random")
        .collect();
    let mut best = vec![0usize; res.len()];
    for (i, s) in seeds.iter().enumerate() {
        print!("{s:5}");
        for (_, o) in &res {
            print!(" {:8.3}", o[i].score);
        }
        let b = styles
            .iter()
            .copied()
            .max_by(|&a, &b| res[a].1[i].score.total_cmp(&res[b].1[i].score));
        if let Some(b) = b {
            best[b] += 1;
            print!("  {}", res[b].0);
        }
        println!();
    }
    print!("Best style per seed:");
    for &i in &styles {
        print!(" {} {}", res[i].0, best[i]);
    }
    println!();
    // Paired tests, Holm over all of them.
    let mut tests: Vec<(String, usize, usize, f64)> = Vec::new();
    for (ai, &a) in styles.iter().enumerate() {
        let others: Vec<usize> = (0..res.len())
            .filter(|&b| res[b].0 == "idle" || res[b].0 == "random")
            .chain(styles[ai + 1..].iter().copied())
            .collect();
        for b in others {
            let (mut w, mut n) = (0, 0);
            for (x, y) in res[a].1.iter().zip(res[b].1) {
                if (x.score - y.score).abs() < 1e-4 {
                    continue;
                }
                n += 1;
                if x.score > y.score {
                    w += 1;
                }
            }
            tests.push((format!("{} > {}", res[a].0, res[b].0), w, n, sign_p(w, n)));
        }
    }
    let mut order: Vec<usize> = (0..tests.len()).collect();
    order.sort_by(|&a, &b| tests[a].3.total_cmp(&tests[b].3));
    let m = tests.len();
    let mut holm = vec![false; m];
    for (rank, &i) in order.iter().enumerate() {
        if tests[i].3 * (m - rank) as f64 <= 0.05 {
            holm[i] = true;
        } else {
            break;
        }
    }
    println!("\nPaired sign tests (ties dropped), Holm over {m}:");
    for (i, (name, w, n, p)) in tests.iter().enumerate() {
        println!(
            "  {name:22} {w:2} of {n:2}  p {p:.5}  {}",
            if holm[i] { "survives" } else { "-" }
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds = seeds(arg(&args, "seeds"));
    let lean = arg(&args, "lean") == Some("1");
    let mut rules = Rules::default();
    if let Some(set) = arg(&args, "set") {
        for kv in set.split(',') {
            let (k, v) = kv.split_once('=').expect("set=name=value,...");
            assert!(set_rule(&mut rules, k, v.parse().expect("number")), "no rule {k}");
        }
        println!("rules: {set}");
    }
    let hands: Vec<String> = match arg(&args, "hands") {
        Some(h) => h.split(',').map(String::from).collect(),
        None => ["idle", "random"]
            .into_iter()
            .map(String::from)
            .chain(Style::presets().iter().map(|s| s.name.to_string()))
            .collect(),
    };
    if let Some(t) = arg(&args, "trace") {
        TRACE.store(t.parse().expect("trace=SECS"), std::sync::atomic::Ordering::Relaxed);
    }
    let t0 = std::time::Instant::now();
    if let Some(dir) = arg(&args, "look") {
        std::fs::create_dir_all(dir).expect("look dir");
        let hand = arg(&args, "hand").unwrap_or("adapt");
        for &s in &seeds {
            let o = run(s, hand, rules, lean, Some(dir), None);
            println!("seed {s} {hand}: {o:?}");
        }
    } else if let Some(dir) = arg(&args, "replays") {
        std::fs::create_dir_all(dir).expect("replay dir");
        for &s in &seeds {
            for h in &hands {
                let o = run(s, h, rules, lean, None, Some(dir));
                println!(
                    "seed {s} {h}: score {:.3} {} at {:.0} s, {} commands",
                    o.score, o.band, o.end_t, o.cmds
                );
            }
        }
    } else if args.first().map(String::as_str) == Some("census") {
        census(&seeds, &hands, rules, lean);
    } else {
        panic!("census | look=DIR | replays=DIR");
    }
    println!("({:.1} s)", t0.elapsed().as_secs_f32());
}
