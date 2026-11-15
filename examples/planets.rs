//! **Generated planets, written out and measured** (`mould::levelgen`, the
//! roguelike run's levels, 2026-11-14).
//!
//! `cargo run --release --example planets -- [seeds=1-3] [depths=1-8] [out=DIR] [attempt=N]`
//!
//! For each seed and depth: the planet's card (name, biome), the attempt
//! that passed its checks (and what the earlier ones failed on), its
//! numbers (`levelgen::stats`), and the build time. With `out=` the maps
//! are saved as the loader's files, `DIR/planet-DEPTH-SEED.png` and `.txt`,
//! for the editor (`larder-edit map=DIR/planet-3-1`) and the game
//! (`larder-world map=DIR/planet-3-1`); `larder-world map=planet:3:1` builds
//! the same planet without the files.

use std::path::PathBuf;
use std::time::Instant;

use slime_mould::mould::levelgen::{self, ATTEMPTS};

fn range(v: &str) -> Result<Vec<u64>, String> {
    let (a, b) = v.split_once('-').unwrap_or((v, v));
    let (a, b): (u64, u64) = (
        a.parse().map_err(|e| format!("{v}: {e}"))?,
        b.parse().map_err(|e| format!("{v}: {e}"))?,
    );
    Ok((a..=b).collect())
}

fn main() -> Result<(), String> {
    let (mut seeds, mut depths, mut out, mut attempt) = (
        vec![1, 2, 3],
        (1..=8).collect::<Vec<u64>>(),
        None::<PathBuf>,
        None::<u32>,
    );
    for a in std::env::args().skip(1) {
        let (k, v) = a.split_once('=').ok_or(format!("expected key=value, got {a}"))?;
        match k {
            "seeds" => seeds = range(v)?,
            "depths" => depths = range(v)?,
            "out" => out = Some(PathBuf::from(v)),
            // Save this attempt's build, checked or not, in place of the
            // planet (to look at why an attempt failed).
            "attempt" => attempt = Some(v.parse().map_err(|e| format!("attempt {v}: {e}"))?),
            _ => return Err(format!("unknown key {k} (seeds, depths, out, attempt)")),
        }
    }
    if let Some(d) = &out {
        std::fs::create_dir_all(d).map_err(|e| format!("{}: {e}", d.display()))?;
    }
    println!("depth seed name biome [openness water soil pattern] attempt ms | crystals rock sand clay barren wet walls | walk: nest first crystal | open charge | doors: direct/round per nest | failed attempts");
    let mut total = (0, 0.0f64);
    for &depth in &depths {
        for &seed in &seeds {
            let p = levelgen::planet(seed, depth as u32);
            let t = Instant::now();
            let m = levelgen::generate(&p);
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            total = (total.0 + 1, total.1 + ms);
            // Which attempt passed, and why the ones before it did not.
            let mut why = Vec::new();
            let mut passed = None;
            for a in 0..ATTEMPTS {
                let problems = levelgen::check(&p, &levelgen::build(&p, a));
                if problems.is_empty() {
                    passed = Some(a);
                    break;
                }
                why.push(problems.join("; "));
            }
            let s = levelgen::stats(&p, &m);
            let rc = levelgen::Recipe::of(&p);
            let pc = |v: f32| format!("{:.0}%", v * 100.0);
            println!(
                "{depth} {seed} {} {} [{:?} {:?} {:?} {}] {} {ms:.0} | {} {} {} {} {} {} {} | {} {} {} | {} | {} | {}",
                p.name,
                p.biome.name().replace(' ', "-"),
                rc.openness,
                rc.water,
                rc.soil,
                rc.pattern.name().replace(' ', "-"),
                passed.map_or("fallback".into(), |a| a.to_string()),
                s.crystals,
                pc(s.rock),
                pc(s.sand),
                pc(s.clay),
                pc(s.barren),
                pc(s.wet),
                s.walls,
                s.to_nest,
                s.to_first,
                s.to_crystal,
                pc(s.open_share),
                levelgen::doors(&p, &m)
                    .iter()
                    .map(|(d, r)| format!("{d}/{r}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                why.join(" / ")
            );
            if let Some(d) = &out {
                let m = attempt.map_or(m, |a| levelgen::build(&p, a));
                m.save(d, &format!("planet-{depth}-{seed}"))?;
            }
        }
    }
    println!(
        "{} planets, {:.0} ms each on average",
        total.0,
        total.1 / total.0.max(1) as f64
    );
    Ok(())
}
