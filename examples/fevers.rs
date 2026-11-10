//! **Two Fevers probe** (`slime_mould::mould::fever`).
//!
//! ```text
//! cargo run --release --example fevers -- look=DIR [seed=N] [a=plan b=random]
//! cargo run --release --example fevers -- census a=plan b=random seeds=1-16 [cross=1] [set=key:val,...]
//! cargo run --release --example fevers -- front seeds=1-8
//! ```
//!
//! `look` writes a PNG every 10 s of one match. `census` plays every seed
//! with the hands both ways round (a as Ember then as Tide) and prints one
//! line a game: the score each side, a's part of the summed score, and who
//! led at 1:30. `front` seeds once at the start and prints how far from the
//! seed the farthest carrier and the median carrier are each second.
use slime_mould::mould::fever::{play_match, Fevers, Hand, Phase, Player, Rules, EMBER, H, STEPS_PER_SECOND, TIDE, W};
use std::time::Instant;

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

fn rules(args: &[String]) -> Rules {
    let mut r = if arg(args, "v3") == Some("1") {
        Rules::v3()
    } else {
        Rules::default()
    };
    if arg(args, "cross") == Some("1") {
        r.cross_immune = true;
    }
    if let Some(set) = arg(args, "set") {
        for kv in set.split(',') {
            let (k, v) = kv.split_once(':').unwrap();
            let f: f32 = v.parse().unwrap();
            match k {
                "walkers" => r.walkers = f as usize,
                "catch" => r.catch = f,
                "floor" => r.catch_floor = f,
                "keep" => r.strain_keep = f,
                "dep" => r.strain_deposit = f,
                "sick" => r.sick_seconds = f,
                "immune" => r.immune_seconds = f,
                "spore" => r.spore_seconds = f,
                "seed_r" => r.seed_r = f,
                "match" => r.match_seconds = f,
                "angle" => r.sensor_angle = f.to_radians(),
                "regrow" => r.regrow = f,
                "eat" => r.eat = f,
                "owned" => r.owned = f != 0.0,
                "hold" => r.hold_seconds = f,
                "hush_trail" => r.hush_trail = f,
                "tube" => r.tube_fever = f,
                "call_w" => r.call_weight = f,
                "reach" => r.call_reach = f,
                "forage" => r.forage = f,
                "starve" => r.starve_seconds = f,
                "home" => r.home_r = f,
                "resist" => r.resist = f,
                "pinch_cost" => r.pinch_cost = f as u32,
                "pinch_wait" => r.pinch_wait = f,
                "spores_max" => r.spores_max = f as u32,
                _ => panic!("unknown rule {k}"),
            }
        }
    }
    r
}

fn save(g: &Fevers, path: &str) {
    let mut buf = vec![0u8; W * H * 4];
    g.paint(&mut buf);
    let img = image::RgbaImage::from_raw(W as u32, H as u32, buf).unwrap();
    let img = image::imageops::resize(&img, W as u32 * 3, H as u32 * 3, image::imageops::FilterType::Nearest);
    img.save(path).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let r = rules(&args);
    let a = Hand::parse(arg(&args, "a").unwrap_or("plan")).unwrap();
    let b = Hand::parse(arg(&args, "b").unwrap_or("random")).unwrap();
    if let Some(dir) = arg(&args, "look") {
        std::fs::create_dir_all(dir).unwrap();
        let seed: u64 = arg(&args, "seed").unwrap_or("1").parse().unwrap();
        let mut g = Fevers::new(seed, r);
        let mut p = [Player::new(a, EMBER, seed), Player::new(b, TIDE, seed)];
        let every = arg(&args, "every").map_or(10.0, |v| v.parse::<f32>().unwrap());
        let mut next = 0.0;
        let t0 = Instant::now();
        while g.phase() != Phase::Over {
            let first = (g.t & 1) as usize;
            p[first].act(&mut g);
            p[1 - first].act(&mut g);
            g.step();
            let secs = g.t as f32 / STEPS_PER_SECOND as f32;
            if secs >= next {
                let sh = g.share_now();
                println!(
                    "{secs:6.1}s  ember {:.3}  tide {:.3}  spores {:?}",
                    sh[0], sh[1], g.spores
                );
                save(&g, &format!("{dir}/t{:03}.png", secs as u32));
                next += every;
            }
        }
        let s = g.score();
        println!(
            "score ember {:.4} tide {:.4}  seeds {:?}  caught {:?}",
            s[0], s[1], g.seeds, g.seeded
        );
        println!("ms/step {:.3}", t0.elapsed().as_secs_f64() * 1e3 / g.t as f64);
        return;
    }
    if args.iter().any(|a| a == "front") {
        for seed in seeds(arg(&args, "seeds")) {
            let mut g = Fevers::new(seed, r.clone());
            while g.phase() == Phase::Warm {
                g.step();
            }
            // Seed on the walker nearest the dish centre.
            let w = *g
                .host
                .iter()
                .min_by(|p, q| ((p.x - 128.0).hypot(p.y - 80.0)).total_cmp(&(q.x - 128.0).hypot(q.y - 80.0)))
                .unwrap();
            let n = g.seed(EMBER, w.x, w.y);
            let mut line = format!("seed {seed} caught {n}:");
            for sec in 1..=20 {
                for _ in 0..STEPS_PER_SECOND {
                    g.step();
                }
                let mut d: Vec<f32> = g
                    .host
                    .iter()
                    .filter(|h| h.carrying() == Some(EMBER))
                    .map(|h| (h.x - w.x).hypot(h.y - w.y))
                    .collect();
                d.sort_by(f32::total_cmp);
                let far = d.last().copied().unwrap_or(0.0);
                let p90 = if d.is_empty() { 0.0 } else { d[d.len() * 9 / 10] };
                line += &format!(" {sec}s:{}/{:.0}/{:.0}", d.len(), p90, far);
            }
            println!("{line}");
        }
        return;
    }
    if args.iter().any(|a| a == "census") {
        println!(
            "# census a={} b={} cross={} rules={:?}",
            a.name(),
            b.name(),
            r.cross_immune,
            arg(&args, "set")
        );
        println!("seed a_as score_a score_b a_part lead90 a_seeds b_seeds");
        let t0 = Instant::now();
        let list = seeds(arg(&args, "seeds"));
        let jobs: Vec<(u64, usize)> = list.iter().flat_map(|&s| [(s, EMBER), (s, TIDE)]).collect();
        let out = std::sync::Mutex::new(Vec::new());
        let next = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|sc| {
            for _ in 0..std::thread::available_parallelism().map_or(4, |n| n.get()) {
                sc.spawn(|| loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(&(seed, a_as)) = jobs.get(k) else { break };
                    let (ha, hb) = if a_as == EMBER { (a, b) } else { (b, a) };
                    // Who leads the running score at 1:30 of play.
                    let mut g = Fevers::new(seed, r.clone());
                    let mut p = [Player::new(ha, EMBER, seed), Player::new(hb, TIDE, seed)];
                    let mut lead90 = None;
                    while g.phase() != Phase::Over {
                        let first = (g.t & 1) as usize;
                        p[first].act(&mut g);
                        p[1 - first].act(&mut g);
                        g.step();
                        if lead90.is_none() && g.play_seconds() >= 90.0 {
                            let s = g.score();
                            lead90 = Some(if s[a_as] >= s[1 - a_as] { "a" } else { "b" });
                        }
                    }
                    let s = g.score();
                    let line = format!(
                        "{seed} {} {:.4} {:.4} {:.4} {} {} {}",
                        if a_as == EMBER { "ember" } else { "tide" },
                        s[a_as],
                        s[1 - a_as],
                        s[a_as] / (s[0] + s[1]).max(1e-9),
                        lead90.unwrap_or("-"),
                        g.seeds[a_as],
                        g.seeds[1 - a_as]
                    );
                    out.lock().unwrap().push((seed, a_as, line));
                });
            }
        });
        let mut v = out.into_inner().unwrap();
        v.sort();
        for (_, _, l) in &v {
            println!("{l}");
        }
        eprintln!("{} games in {:.1}s", v.len(), t0.elapsed().as_secs_f64());
        return;
    }
    let g = play_match(1, r, a, b);
    println!("{:?}", g.score());
}
