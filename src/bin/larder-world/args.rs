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

/// Shape C's walkers and keys, under any engine keys given (theirs come
/// after, so they win).
pub(super) fn default_shape(args: &mut Args) {
    let (walkers, keys) = shape_keys(DEFAULT_SHAPE).expect("the default shape is on the shapes page");
    args.walkers = walkers;
    let given = std::mem::take(&mut args.keys);
    args.keys = keys.iter().map(|(k, v)| (k.to_string(), v.to_string())).chain(given).collect();
}
