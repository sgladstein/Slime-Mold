//! **What is new, and things to try**: the two lists the game shows about
//! itself (the plan of 2026-11-17, rule 4: "Every change Scott can see or
//! feel adds one line to the game's 'what is new' list: what he will see
//! and how to try it"), and what this PC has already seen.
//!
//! The lists are two text files at the repo's root, compiled into the
//! game so a build always carries its own:
//!
//! - `news/whats-new.txt`, one line a change, the newest at the bottom:
//!   `DATE | LANE | what you will see | how to try it`. Every lane's pull
//!   requests add lines; the Direction lane owns the wording. `wiki/lint.py`
//!   fails a pull request that changes the game and adds no line.
//! - `news/things-to-try.txt`, at most five lines, the Direction lane's.
//!
//! **What this PC has seen** ([`Memory`]) is kept in the settings file
//! beside the UI size (`world::settings`): the last build played here, the
//! lines seen when it was played, and the things ticked as tried. A line
//! is new until a round has been played on a build that holds it.
//!
//! Why lines are remembered by their text and not by a build number: a
//! pull request cannot know the build it will land in (the merge queue
//! decides the order), and the commit count is 0 on a shallow clone. A
//! hash of the line needs neither, and survives lines being reordered or
//! pruned. The cost: a line that is reworded shows as new once more.

use super::switches::Lane;
use crate::world::settings;
use std::path::{Path, PathBuf};

const WHATS_NEW: &str = include_str!("../../news/whats-new.txt");
const TO_TRY: &str = include_str!("../../news/things-to-try.txt");

/// The most things to try the game shows (the plan: "up to five").
pub const MOST_TO_TRY: usize = 5;

/// One "what is new" line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct News {
    /// The line's own number (a hash of its text), what `Memory` keeps.
    pub id: u32,
    /// The day it was written, YYYY-MM-DD.
    pub date: &'static str,
    pub lane: Lane,
    /// What the player will see.
    pub sees: &'static str,
    /// How to try it; may be empty.
    pub try_it: &'static str,
}

/// One thing to try.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Try {
    pub id: u32,
    pub text: &'static str,
}

/// FNV-1a over the line's words (runs of spaces count once, so
/// re-aligning a line does not make it new).
fn id_of(line: &str) -> u32 {
    let mut h: u32 = 0x811C_9DC5;
    for (i, w) in line.split_whitespace().enumerate() {
        for b in (i > 0).then_some(b' ').into_iter().chain(w.bytes()) {
            h = (h ^ u32::from(b)).wrapping_mul(0x0100_0193);
        }
    }
    h
}

/// The lines of a list that are not blank or a `#` comment.
fn lines(text: &'static str) -> impl Iterator<Item = &'static str> {
    text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#'))
}

/// One line of `whats-new.txt` read, or what is wrong with it.
pub fn parse_line(line: &'static str) -> Result<News, String> {
    let p: Vec<&str> = line.split('|').map(str::trim).collect();
    let [date, lane, sees, rest @ ..] = p.as_slice() else { return Err(format!("`{line}`: DATE | LANE | what you will see | how to try it")) };
    let d: Vec<&str> = date.split('-').collect();
    if d.len() != 3 || [4, 2, 2] != [d[0].len(), d[1].len(), d[2].len()] || d.iter().any(|x| x.parse::<u32>().is_err()) {
        return Err(format!("`{line}`: the date is YYYY-MM-DD"));
    }
    let lane = Lane::from_word(lane).ok_or_else(|| format!("`{line}`: the lane is one of direction, enemy, squad, look, engine, lab"))?;
    if sees.is_empty() {
        return Err(format!("`{line}`: say what the player will see"));
    }
    if rest.len() > 1 {
        return Err(format!("`{line}`: four parts at most (write a `|` in the text as a comma)"));
    }
    Ok(News { id: id_of(line), date, lane, sees, try_it: rest.first().copied().unwrap_or("") })
}

/// What is new, the newest first. A line that does not read is left out
/// (`the_lists_read` fails on one, so none reaches main).
pub fn news() -> Vec<News> {
    let mut v: Vec<News> = lines(WHATS_NEW).filter_map(|l| parse_line(l).ok()).collect();
    v.reverse();
    v
}

/// The things to try, at most `MOST_TO_TRY`, in the file's order.
pub fn to_try() -> Vec<Try> {
    lines(TO_TRY).take(MOST_TO_TRY).map(|l| Try { id: id_of(l), text: l }).collect()
}

/// **What this PC has seen**: read once when the window opens, and saved
/// when a round is played or a thing is ticked.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Memory {
    /// The last build a round was played on here: its number (0 when that
    /// build had none), commit and date. None on a PC that has not played
    /// since the game began remembering (2026-11-17).
    pub last_build: Option<(u32, String, String)>,
    seen: Vec<u32>,
    tried: Vec<u32>,
    /// Where it is saved; None keeps it in memory only (tests, films).
    path: Option<PathBuf>,
}

fn ids(v: Option<String>) -> Vec<u32> {
    v.unwrap_or_default().split(',').filter_map(|x| u32::from_str_radix(x.trim(), 16).ok()).collect()
}

fn word(v: &[u32]) -> String {
    v.iter().map(|x| format!("{x:08x}")).collect::<Vec<_>>().join(",")
}

impl Memory {
    /// This PC's, from the settings file (empty with none).
    pub fn load() -> Memory {
        settings::path().map_or_else(Memory::default, |p| Memory::load_from(&p))
    }

    /// From the settings file at `p`, where it is saved back.
    pub fn load_from(p: &Path) -> Memory {
        let get = |k: &str| settings::get_at(p, k);
        Memory { last_build: get("last_build").as_deref().and_then(super::parse), seen: ids(get("news_seen")), tried: ids(get("tried")), path: Some(p.to_path_buf()) }
    }

    /// One that is never saved (the no-window films and tests, which must
    /// not mark the owner's news as read).
    pub fn unsaved() -> Memory {
        Memory::default()
    }

    /// `n` has not been on a build played here.
    pub fn is_new(&self, n: &News) -> bool {
        !self.seen.contains(&n.id)
    }

    /// What is new since the last build played here, the newest first
    /// (everything, on a PC that has not played).
    pub fn fresh(&self) -> Vec<News> {
        news().into_iter().filter(|n| self.is_new(n)).collect()
    }

    pub fn is_tried(&self, t: &Try) -> bool {
        self.tried.contains(&t.id)
    }

    /// Ticks or unticks a thing to try, and saves.
    pub fn set_tried(&mut self, t: &Try, tried: bool) {
        self.tried.retain(|&x| x != t.id);
        if tried {
            self.tried.push(t.id);
        }
        // Ticks of things no longer on the list are dropped.
        let now: Vec<u32> = to_try().iter().map(|t| t.id).collect();
        self.tried.retain(|x| now.contains(x));
        self.save();
    }

    /// **A round was played on this build**: from the next time the game
    /// opens, today's lines are no longer new. This copy keeps what it
    /// read, so the menu still shows them as new until the game closes.
    /// Returns the copy as the next opening will read it.
    pub fn played(&self) -> Memory {
        let b = super::build();
        let next = Memory { last_build: Some((b.number, b.commit.to_string(), b.date.to_string())), seen: news().iter().map(|n| n.id).collect(), tried: self.tried.clone(), path: self.path.clone() };
        next.save();
        next
    }

    fn save(&self) {
        let Some(p) = &self.path else { return };
        let build = self.last_build.as_ref().map(|(n, c, d)| format!("{n}:{c}:{d}"));
        let all = [("last_build", build), ("news_seen", Some(word(&self.seen))), ("tried", Some(word(&self.tried)))];
        for (k, v) in all {
            if let Some(v) = v {
                if let Err(e) = settings::set_at(p, k, &v) {
                    // The game goes on: the cost is seeing a line twice.
                    eprintln!("larder-world: what this PC has seen was not saved ({e})");
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_lists_read() {
        for l in lines(WHATS_NEW) {
            if let Err(e) = parse_line(l) {
                panic!("news/whats-new.txt: {e}");
            }
        }
        let all = news();
        assert!(!all.is_empty(), "news/whats-new.txt has no lines");
        // Two lines with one text would share an id, and one tick or one
        // play would cover both.
        for (i, a) in all.iter().enumerate() {
            assert!(all[..i].iter().all(|b| b.id != a.id), "news/whats-new.txt: `{}` is there twice", a.sees);
        }
        let n = lines(TO_TRY).count();
        assert!(n <= MOST_TO_TRY, "news/things-to-try.txt has {n} lines; the game shows {MOST_TO_TRY}, so take the oldest out");
    }

    #[test]
    fn a_line_reads_and_says_what_is_wrong() {
        let n = parse_line("2026-11-17 | engine | The menu shows a build number | Look at the window's title").unwrap();
        assert_eq!((n.date, n.lane, n.sees, n.try_it), ("2026-11-17", Lane::Engine, "The menu shows a build number", "Look at the window's title"));
        assert_eq!(parse_line("2026-11-17 | look | Brighter muzzle flashes").unwrap().try_it, "");
        // Re-aligned, it is the same line.
        assert_eq!(parse_line("2026-11-17 |  look |   Brighter muzzle flashes").unwrap().id, parse_line("2026-11-17 | look | Brighter muzzle flashes").unwrap().id);
        assert!(parse_line("11-17 | look | x").unwrap_err().contains("YYYY-MM-DD"));
        assert!(parse_line("2026-11-17 | guns | x").unwrap_err().contains("lane"));
        assert!(parse_line("2026-11-17 | look |").unwrap_err().contains("see"));
        assert!(parse_line("2026-11-17 | look | a | b | c").is_err());
    }

    #[test]
    fn a_pc_remembers_what_it_played() {
        let p = std::env::temp_dir().join(format!("larder-news-{}/settings.txt", std::process::id()));
        let _ = std::fs::remove_file(&p);
        settings::set_at(&p, "ui", "125%").unwrap();
        // A PC that has not played: everything is new, nothing is tried.
        let m = Memory::load_from(&p);
        assert_eq!(m.last_build, None);
        assert_eq!(m.fresh(), news());
        // Playing a round: this opening still shows the lines as new...
        let next = m.played();
        assert_eq!(m.fresh(), news());
        // ...and the next one shows none, and knows the build.
        let again = Memory::load_from(&p);
        assert_eq!(again, next);
        assert!(again.fresh().is_empty());
        let b = crate::build::build();
        assert_eq!(again.last_build, Some((b.number, b.commit.to_string(), b.date.to_string())));
        // A line from a later build is new again.
        let later = News { id: 7, date: "2099-01-01", lane: Lane::Lab, sees: "x", try_it: "" };
        assert!(again.is_new(&later));
        // Ticks are kept, and the other settings are untouched.
        if let Some(t) = to_try().first() {
            let mut m = again.clone();
            m.set_tried(t, true);
            assert!(Memory::load_from(&p).is_tried(t));
            m.set_tried(t, false);
            assert!(!Memory::load_from(&p).is_tried(t));
        }
        assert_eq!(settings::get_at(&p, "ui").as_deref(), Some("125%"));
        // One that is not saved leaves no file.
        let q = p.with_file_name("none.txt");
        let _ = Memory::unsaved().played();
        assert!(!q.exists());
        let _ = std::fs::remove_dir_all(p.parent().unwrap());
    }
}
