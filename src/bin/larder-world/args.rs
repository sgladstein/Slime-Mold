//! The shapes' keys and the default shape. Moved out of `larder-world.rs`
//! unchanged (C9, 2026-11-14).

use super::*;

/// The engine keys of a shape on the shapes page (`shape=`), with its start
/// walkers. The films held one walker a cell with `capacity=50000`.
pub(super) fn shape_keys(name: &str) -> Option<(usize, &'static [(&'static str, &'static str)])> {
    const DENSE: [(&str, &str); 2] = [("upkeep", "0.0002"), ("capacity", "50000")];
    // C shares its crystals' food through the body (`energy_flow`, the one
    // body thread's 0.05; 2026-11-13, the slime-enemy thread). Why: the
    // owner, 22:22Z, playing C: "I cannot tell if it is getting stronger by
    // holding the crystals." It was: with crystals that feed nothing C dies
    // by 2:00 (CPU, 4 seeds). But what it ate stayed with the walkers on the
    // crystals, whose cells were full, so it bred few walkers and the
    // screen showed nothing (`examples/fedprobe.rs`). With the food shared,
    // walkers along the network breed where there is room, and the body
    // swells round the crystals it holds: on Fork seed 1 (farm hand, the
    // window on software Vulkan) 45,237 walkers at 5:00 against 13,956, and
    // mould on 17.1% of the map's screen against 7.0% (1:45 8.9% against
    // 4.6%). Tried first: fed walkers packing two a cell
    // (`BigParams::fed_crowd` 1): 21,053 walkers but 7.6% of the screen, as
    // the look's thickness is full at one walker a cell.
    // A part cut off from every crystal withers (`BigParams::wither` 0.01,
    // 2026-11-13). Why: with the food shared, a cut-off part pooled its
    // energy, stayed whole for 75 s or more and then died at once, so a cut
    // read as nothing happening. With withering a part that has heard no
    // news of food for 5 s thins steadily while the body joined to its
    // crystals keeps every walker (uncut C the same to the walker on 4 of
    // 4 CPU seeds). The owner, 11-13 15:51Z, on the blind A/B/C: "B is the
    // best but still too fast" (B was 0.02, half the piece gone at about
    // 25 s), then 15:53Z: "Lets just double it and call it good. I will
    // have to playtest." At 0.01 a held ring cut on the CPU leaves 74% of
    // the piece at 20 s, 41% at 45 s and 15% at 90 s
    // (Reports/fed-mould-2026-11-13/README.md, *A cut-off part withers*).
    // A cut-off part also panics (`BigParams::panic` 2, 2026-11-13). The
    // owner, 15:57Z: "It shouldn't have a stronger attack but it should act
    // more like a scared animal. Either run away back to the main body or
    // go right for the soliders. It should be more reckless." A walker with
    // no news for 5 s heads straight for the nearest unit within 20 cells,
    // or else the nearest crystal with food, creeping at half its pace for
    // 1.5 s (the tell; `panic_creep` 0.5, the BigParams default since the
    // game designer's review of the last-stand film, 11-13) and then twice
    // its pace, for 0.5 s at a unit before it creeps again (`panic_dash`
    // 15, so a squad that pulls back in time gets clear); the bite is
    // unchanged. On the CPU race world
    // the scores are unchanged (smoke, 4 seeds x 3 hands); in the window
    // (Fork seed 1, farm) the squad has 30 soldiers at 2:30 against 33, as
    // bits its own shots knock off charge it and are shot (#198). Before
    // #198 soldiers could not shoot the bits on them and it fell to 1.
    // `panic=0` turns it off. The report's *A cut-off part panics* has
    // the films.
    const C: [(&str, &str); 5] = [DENSE[0], DENSE[1], ("energy_flow", "0.05"), ("wither", "0.01"), ("panic", "2")];
    Some(match name {
        "a" => (5_000, &[]),
        "b" => (25_000, &[("start", "spread"), ("food", "0"), ("upkeep", "0.00001"), ("capacity", "50000")]),
        "c" => (25_000, &C),
        "d" => (25_000, &[DENSE[0], DENSE[1], ("sensor_angle", "22.5"), ("sensor_offset", "5")]),
        "e" => (25_000, &[DENSE[0], DENSE[1], ("step", "0.15"), ("trail_scale", "0.02"), ("step_by", "0.85:2")]),
        _ => return None,
    })
}

/// The mould a round plays when nothing picks one (owner, 2026-11-12
/// 22:19Z, after playing D and C: "C is better how it looks, the shape it
/// takes, how it networks the crystals").
pub(super) const DEFAULT_SHAPE: &str = "c";

/// Shape C's walkers and keys, under any engine keys given (theirs come
/// after, so they win).
pub(super) fn default_shape(args: &mut Args) {
    let (walkers, keys) = shape_keys(DEFAULT_SHAPE).expect("the default shape is on the shapes page");
    args.walkers = walkers;
    let given = std::mem::take(&mut args.keys);
    args.keys = keys.iter().map(|(k, v)| (k.to_string(), v.to_string())).chain(given).collect();
}
