//! **Your keys** (Direction 11-24: "Key rebinding in Options: every squad
//! key, with the defaults kept and a reset button, saved beside
//! settings").
//!
//! A rebinding is a swap of two keys, never a key taken from one thing and
//! left on nothing: the game keeps reading the keys it always read (its
//! "logical" keys: A steers left, R attack-moves, F guards), and this
//! module turns the key pressed into the one it stands for. Binding a
//! thing to a key that already does something moves that something to the
//! thing's old key, so every action keeps exactly one key and nothing can
//! clash. Every label that names a key (`ui::key_name`) shows the key now
//! pressed for it.
//!
//! Why a swap of keys and not a table of actions: the squad game's keys
//! mean different things by context (F guards on the main card and drops
//! sleep gas on the tools page; A steers and is a card letter in the RTS
//! round), spread over the input files of three lanes. Moving the key
//! moves all of its meanings together, so no context needs to learn about
//! rebinding, and a new key the game adds is rebindable at once.
//!
//! Only letters and the three punctuation keys the squad uses ([ ] `) can
//! move: the digits pick squads, and Tab, Space, Esc and the F keys are
//! the same in every game of the kind. Kept in the settings file as
//! `keys=PRESSED>MEANS,...` by the keys' names (`ui::key_name`).

use std::sync::RwLock;
use winit::keyboard::KeyCode;

/// The keys that can be moved.
pub const FREE: [KeyCode; 29] = {
    use KeyCode::*;
    [KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ, BracketLeft, BracketRight, Backquote]
};

/// The swaps in force: (pressed, means) for each key that does not mean
/// itself. None until first read from the settings file.
static MAP: RwLock<Option<Vec<(KeyCode, KeyCode)>>> = RwLock::new(None);

fn with<T>(f: impl FnOnce(&Vec<(KeyCode, KeyCode)>) -> T) -> T {
    if MAP.read().map(|m| m.is_none()).unwrap_or(false) {
        let loaded = super::settings::get("keys").map(|w| parse(&w)).unwrap_or_default();
        if let Ok(mut m) = MAP.write() {
            if m.is_none() {
                *m = Some(loaded);
            }
        }
    }
    let m = MAP.read().unwrap_or_else(|e| e.into_inner());
    f(m.as_ref().unwrap_or(&Vec::new()))
}

fn name(k: KeyCode) -> Option<String> {
    super::ui::raw_key_name(k)
}

fn by_name(n: &str) -> Option<KeyCode> {
    FREE.into_iter().find(|&k| name(k).as_deref() == Some(n))
}

/// The settings word read back; a pair naming a key that cannot move, or
/// that would leave the swaps not a one-to-one, is dropped whole.
fn parse(w: &str) -> Vec<(KeyCode, KeyCode)> {
    let pairs: Vec<(KeyCode, KeyCode)> = w.split(',').filter_map(|p| p.split_once('>')).filter_map(|(a, b)| Some((by_name(a.trim())?, by_name(b.trim())?))).filter(|(a, b)| a != b).collect();
    // A one-to-one: no key pressed twice, no key meant twice, and the keys
    // pressed are the keys meant (every meaning still has a key).
    let mut from: Vec<KeyCode> = pairs.iter().map(|p| p.0).collect();
    let mut to: Vec<KeyCode> = pairs.iter().map(|p| p.1).collect();
    let n = from.len();
    from.sort_by_key(|k| name(*k));
    to.sort_by_key(|k| name(*k));
    from.dedup();
    if from.len() != n || from != to {
        return Vec::new();
    }
    pairs
}

/// The settings word for `pairs`.
fn word(pairs: &[(KeyCode, KeyCode)]) -> String {
    pairs.iter().filter_map(|&(a, b)| Some(format!("{}>{}", name(a)?, name(b)?))).collect::<Vec<_>>().join(",")
}

/// What the key pressed stands for, in play.
pub fn means(pressed: KeyCode) -> KeyCode {
    with(|m| m.iter().find(|p| p.0 == pressed).map_or(pressed, |p| p.1))
}

/// The key pressed for `meant` (what a label shows).
pub fn pressed(meant: KeyCode) -> KeyCode {
    with(|m| m.iter().find(|p| p.1 == meant).map_or(meant, |p| p.0))
}

/// Any key moved off its default.
pub fn any_moved() -> bool {
    with(|m| !m.is_empty())
}

fn store(pairs: Vec<(KeyCode, KeyCode)>, save: bool) {
    if save {
        if let Err(e) = super::settings::set("keys", &word(&pairs)) {
            eprintln!("larder-world: the keys were not saved ({e})");
        }
    }
    if let Ok(mut m) = MAP.write() {
        *m = Some(pairs);
    }
}

/// `meant` is pressed with `key` from now on, and whatever `key` did moves
/// to `meant`'s old key; saved (when `save`). False when `key` cannot move.
pub fn bind(meant: KeyCode, key: KeyCode, save: bool) -> bool {
    if !FREE.contains(&key) || !FREE.contains(&meant) {
        return false;
    }
    let old = pressed(meant);
    let other = means(key);
    let mut pairs: Vec<(KeyCode, KeyCode)> = with(|m| m.clone());
    pairs.retain(|p| p.0 != key && p.0 != old);
    pairs.push((key, meant));
    pairs.push((old, other));
    pairs.retain(|p| p.0 != p.1);
    store(pairs, save);
    true
}

/// Every key back to its default; saved (when `save`).
pub fn reset(save: bool) {
    store(Vec::new(), save);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A bind swaps: the key taken gives its old meaning to the other key;
    /// a chain of binds stays one-to-one; the word reads back; reset
    /// clears. (One test, as the map is global.)
    #[test]
    fn a_bind_swaps_two_keys_and_reads_back() {
        use KeyCode::*;
        reset(false);
        assert_eq!((means(KeyQ), pressed(KeyR)), (KeyQ, KeyR));
        assert!(bind(KeyR, KeyQ, false));
        assert_eq!((means(KeyQ), means(KeyR), pressed(KeyR), pressed(KeyQ)), (KeyR, KeyQ, KeyQ, KeyR));
        // Again, with a third key: R now on Z; Z's old meaning to Q.
        assert!(bind(KeyR, KeyZ, false));
        assert_eq!((means(KeyZ), means(KeyQ), means(KeyR)), (KeyR, KeyZ, KeyQ));
        let all: Vec<KeyCode> = FREE.iter().map(|&k| means(k)).collect();
        let mut sorted = all.clone();
        sorted.sort_by_key(|k| name(*k));
        let mut free = FREE.to_vec();
        free.sort_by_key(|k| name(*k));
        assert_eq!(sorted, free, "every meaning keeps one key");
        let w = with(|m| word(m));
        assert_eq!(parse(&w).len(), with(|m| m.len()), "{w}");
        assert!(parse("Q>R").is_empty(), "not one-to-one: dropped");
        assert!(!bind(KeyR, Digit1, false), "a digit cannot take a meaning");
        // Back on itself: no pair kept.
        reset(false);
        assert!(bind(KeyR, KeyQ, false) && bind(KeyR, KeyR, false));
        assert!(!any_moved());
        reset(false);
    }
}
