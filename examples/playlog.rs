//! **Reads a play log** (`mould::playlog`, the whole-game review's W13,
//! 2026-11-15): the window writes `larder-world-log-MAP-SEED.txt` beside
//! wherever the game ran from (on the owner's PC, the repo folder that
//! `play.cmd` runs in), a line a game second.
//!
//! `cargo run --release --example playlog -- FILE [FILE...]`
//!
//! For each file: time to 10, 20, 30 and 40 soldiers, squads at once and the
//! first split, pumps, turrets and walls standing, shots and kills, the
//! slime's peak mass, a minute-by-minute table of kills a second against the
//! slime's mass, frame times, and the commands given by kind.

fn main() -> Result<(), String> {
    let files: Vec<String> = std::env::args().skip(1).collect();
    if files.is_empty() {
        return Err("usage: playlog FILE [FILE...] (the window's larder-world-log-MAP-SEED.txt)".into());
    }
    for f in files {
        let text = std::fs::read_to_string(&f).map_err(|e| format!("{f}: {e}"))?;
        println!("== {f}");
        print!(
            "{}",
            slime_mould::mould::playlog::summary(&slime_mould::mould::playlog::read(&text))
        );
    }
    Ok(())
}
