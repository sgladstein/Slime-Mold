// The soft-lock scan of 2026-11-20 (README.md, *Checks*). Paste at the end
// of src/mould/larder_world/tests.rs and run
// `cargo test --release --locked --lib zz_scan -- --nocapture` (about 100 s).
// It prints any planet where a slime start's walkers cannot reach a crystal
// or the landing over dry or shallow ground, or a living walker starts in
// deep water. On 2026-11-20: "67 maps with deep water, 0 with a problem"
// once the empty slots (parked, not alive) were left out.
#[test]
fn zz_scan_slime_reach_over_deep_water() {
    let mut bad = 0;
    let mut deep_maps = 0;
    for depth in 1..=8u32 {
        for seed in 1..=30u64 {
            let m = crate::mould::levelgen::level(seed, depth);
            if !m.ground.contains(&crate::mould::map::G_DEEP) {
                continue;
            }
            deep_maps += 1;
            let d = BigDish::with_map(race_params(1, 2, 500), &m);
            let (w, h) = (d.w, d.h);
            let open: Vec<bool> = (0..w * h).map(|i| d.agar[i] && !d.deep(i)).collect();
            let mut msgs = Vec::new();
            // Homes are dish cells; walkers fine cells (scale 2).
            for (hk, &hm0) in d.homes.iter().enumerate() {
                let hm = (hm0.0 * 2.0, hm0.1 * 2.0);
                let mut seen = vec![false; w * h];
                let mut q = std::collections::VecDeque::new();
                for &(x, y, _) in &d.walkers {
                    if (x - hm.0).hypot(y - hm.1) > 40.0 {
                        continue;
                    }
                    let i = y as usize * w + x as usize;
                    if open[i] && !seen[i] {
                        seen[i] = true;
                        q.push_back(i);
                    }
                }
                while let Some(i) = q.pop_front() {
                    let (x, y) = (i % w, i / w);
                    for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                            continue;
                        }
                        let j = ny as usize * w + nx as usize;
                        if open[j] && !seen[j] {
                            seen[j] = true;
                            q.push_back(j);
                        }
                    }
                }
                let near = |c: (f32, f32), r: f32| {
                    let m = r + 3.0;
                    ((c.1 - m).max(0.0) as usize..((c.1 + m) as usize).min(h)).any(|y| ((c.0 - m).max(0.0) as usize..((c.0 + m) as usize).min(w)).any(|x| seen[y * w + x] && (x as f32 + 0.5 - c.0).hypot(y as f32 + 0.5 - c.1) <= m))
                };
                let cs = World::crystals(&d);
                let miss: Vec<usize> = (0..cs.len()).filter(|&k| !near(cs[k].0, cs[k].1)).collect();
                let land = World::landing(&d).is_none_or(|l| near(l, 4.0));
                let wet_home = d.walkers.iter().filter(|&&(x, y, _)| (x - hm.0).hypot(y - hm.1) <= 40.0 && d.deep(y as usize * w + x as usize)).count();
                if !miss.is_empty() || !land || wet_home > 0 {
                    msgs.push(format!("home {hk}: {wet_home} walkers in deep, crystals cut off {miss:?} of {}, landing reached {land}", cs.len()));
                }
            }
            if !msgs.is_empty() {
                bad += 1;
                eprintln!("planet {depth} seed {seed}: {}", msgs.join("; "));
            }
        }
    }
    eprintln!("{deep_maps} maps with deep water, {bad} with a problem");
}
