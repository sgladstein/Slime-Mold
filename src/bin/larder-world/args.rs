//! The shapes' keys and the default shape. Moved out of `larder-world.rs`
//! unchanged (C9, 2026-11-14).

use super::*;
// `shape_keys` moved to the library (`larder_world::shapes`, 2026-11-14,
// D1) so the census plays the same shapes.
pub(super) use slime_mould::mould::larder_world::shape_keys;

/// The mould a round plays when nothing picks one (owner, 2026-11-12
/// 22:19Z, after playing D and C: "C is better how it looks, the shape it
/// takes, how it networks the crystals").
pub(super) const DEFAULT_SHAPE: &str = "c";

/// A run level's start walkers: shape C's, times the level's factor
/// (`run::Level::walkers`: depth and area). The menu's Run and `run=` both
/// use it, so a film is the level the owner plays.
pub(super) fn run_walkers(factor: f32) -> usize {
    let (walkers, _) = shape_keys(DEFAULT_SHAPE).expect("the default shape is on the shapes page");
    ((walkers as f32) * factor).round().max(1.0) as usize
}

/// A run level's slime under its tier (`start`): shape C's keys, and C's
/// slots grown by the level's walker factor, so the slime has room to
/// grow on every planet (C holds 25,000 walkers in 50,000 slots on today's
/// map; at 50,000 the core world's 14.4 times the walkers had no room).
pub(super) fn run_shape(s: &mut slime_mould::gpu::Settings, factor: f32) -> Result<(), String> {
    let (_, keys) = shape_keys(DEFAULT_SHAPE).expect("the default shape is on the shapes page");
    for (k, v) in keys {
        slime_mould::gpu::set(s, k, v)?;
    }
    s.capacity = ((s.capacity as f32) * factor).round().max(s.walkers as f32) as usize;
    Ok(())
}

/// Shape C's walkers and keys, under any engine keys given (theirs come
/// after, so they win).
pub(super) fn default_shape(args: &mut Args) {
    let (walkers, keys) = shape_keys(DEFAULT_SHAPE).expect("the default shape is on the shapes page");
    args.walkers = walkers;
    let given = std::mem::take(&mut args.keys);
    args.keys = keys.iter().map(|(k, v)| (k.to_string(), v.to_string())).chain(given).collect();
}
