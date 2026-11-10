// The faint-smell trace (GPU plan, phase 1, 2026-11-10). Paste into the
// `tests` module of src/mould/mod.rs and run
//   cargo test --release --lib -- --ignored faint_smell --nocapture
// It steps Behind the Wall (DISHES[0], seed 8) for 200 steps with floats and
// with `Params::fixed_point`, and every 40 steps prints the walkers past the
// wall (x > 125), the walkers on unfed cells, and how many agar cells hold
// trail or scent above zero but below one fixed-point step (1/65,536).
// Output from the run that found it is at the end of this file.
#[test]
#[ignore]
fn faint_smell() {
    for fx in [false, true] {
        let k = 0;
        let params = Params {
            sensor_angle: level::DISHES[k].sensor_deg.to_radians(),
            stray_reach: level::STRAY_REACH,
            fixed_point: fx,
            ..Params::default()
        };
        let mut d = Dish::from_level_with(&level::DISHES[k], 8, params);
        println!("fixed point {fx}");
        for t in 0..200 {
            d.step();
            if t % 40 == 0 || t == 199 {
                let past = d.walkers.iter().filter(|w| w.x > 125.0).count();
                let unfed = d.walkers.iter().filter(|w| cell(w.x, w.y).is_some_and(|i| d.fed[i] == u16::MAX)).count();
                let agar = d.ground.iter().filter(|g| **g == Ground::Agar).count();
                let faint = |f: &Vec<f32>| f.iter().zip(&d.ground).filter(|(v, g)| **g == Ground::Agar && **v > 0.0 && **v < 1.0 / FIX).count();
                let zero = |f: &Vec<f32>| f.iter().zip(&d.ground).filter(|(v, g)| **g == Ground::Agar && **v == 0.0).count();
                println!(
                    "  t {t}: past the wall {past}, unfed {unfed}; agar {agar}: trail faint {} zero {}, scent faint {} zero {}",
                    faint(&d.trail), zero(&d.trail), faint(&d.scent), zero(&d.scent)
                );
            }
        }
    }
}
// fixed point false
//   t 0: past the wall 0, unfed 216; agar 29899: trail faint 0 zero 29483, scent faint 0 zero 29517
//   t 40: past the wall 0, unfed 13; agar 29899: trail faint 4146 zero 19147, scent faint 12485 zero 12020
//   t 80: past the wall 0, unfed 5; agar 29899: trail faint 7182 zero 13421, scent faint 20849 zero 0
//   t 120: past the wall 0, unfed 0; agar 29899: trail faint 10612 zero 10588, scent faint 17917 zero 0
//   t 160: past the wall 0, unfed 0; agar 29899: trail faint 13400 zero 8119, scent faint 15673 zero 0
//   t 199: past the wall 0, unfed 0; agar 29899: trail faint 15247 zero 6052, scent faint 14021 zero 0
// fixed point true
//   t 0: past the wall 0, unfed 216; agar 29899: trail faint 0 zero 29483, scent faint 0 zero 29517
//   t 40: past the wall 0, unfed 43; agar 29899: trail faint 0 zero 21666, scent faint 0 zero 24921
//   t 80: past the wall 1, unfed 28; agar 29899: trail faint 0 zero 15199, scent faint 0 zero 21913
//   t 120: past the wall 3, unfed 34; agar 29899: trail faint 0 zero 14121, scent faint 0 zero 19741
//   t 160: past the wall 3, unfed 47; agar 29899: trail faint 0 zero 14109, scent faint 0 zero 18221
//   t 199: past the wall 7, unfed 53; agar 29899: trail faint 0 zero 14176, scent faint 0 zero 17297
// Rounding only one part (an env-var switch in the same run): trail alone
// left 9-14 walkers unfed from t 120 on, scent alone 2-8, positions and
// headings alone 0-1, against 0 with floats and 34-53 with all of them.
