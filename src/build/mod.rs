//! **What this build holds** (the Engine and tools lane's first job,
//! 2026-11-17, from the plan the owner approved that day: "the build tells
//! Scott what it holds"). Three things the game can say about itself, so
//! nobody has to remember a list of flags or ask what changed:
//!
//! - [`build()`]: which build this is (a number, the commit, its date),
//!   stamped by the crate's `build.rs`. The window's title and the play
//!   log carry it.
//! - [`switches`]: every feature that is built but not on in the default
//!   game, declared once, with a test that fails on one that is not.
//! - [`news`]: the "what is new" and "things to try" lists the game shows,
//!   and what this PC has already seen (the last build played here).
//!
//! The Look and interface lane draws the pages from these; nothing here
//! draws. The data shape was agreed with that lane on 11-17.

pub mod news;
pub mod switches;

/// **The folder the game's files are in** (the designed maps and the
/// campaign's missions, under `Reports/`): the repo's root.
///
/// Until 2026-11-17 the game used the path it was compiled at
/// (`CARGO_MANIFEST_DIR`), which is right for a build made where it runs
/// and wrong for one made elsewhere: the Windows build CI makes for
/// `play.cmd` would have looked for the maps in the runner's folder. So,
/// in order: `LARDER_ROOT` if it names a folder with the maps (`play.ps1`
/// sets it), the compiled path if the maps are there (every local build
/// and test), a folder above the program that has them (CI's build
/// started by hand from `target\ci`), the current folder. With none, the
/// compiled path, so the error names it as before.
pub fn root() -> std::path::PathBuf {
    use std::path::PathBuf;
    let has = |p: &PathBuf| p.join("Reports/larder-maps-2026-11-11/maps").is_dir();
    let built = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let given = std::env::var_os("LARDER_ROOT").filter(|v| !v.is_empty()).map(PathBuf::from);
    let above = std::env::current_exe().ok().and_then(|exe| exe.ancestors().skip(1).map(PathBuf::from).find(|p| has(p)));
    let here = std::env::current_dir().ok();
    given.into_iter().chain([built.clone()]).chain(above).chain(here).find(has).unwrap_or(built)
}

/// Which build this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Build {
    /// The commits in this build's history, so a newer main has a bigger
    /// number. 0 when it could not be counted (no git at build time, or a
    /// shallow clone, as cloud sessions make): show the commit alone then.
    pub number: u32,
    /// The commit's short hash ("unknown" without git).
    pub commit: &'static str,
    /// The commit's date, UTC, YYYY-MM-DD ("" without git).
    pub date: &'static str,
}

/// This build (see `build.rs` at the crate's root for where it comes from).
pub fn build() -> Build {
    Build { number: env!("LARDER_BUILD_NUMBER").parse().unwrap_or(0), commit: env!("LARDER_BUILD_COMMIT"), date: env!("LARDER_BUILD_DATE") }
}

impl Build {
    /// As the menu's corner and the window's title show it: "build 3127"
    /// or, with no number, "build d08583d".
    pub fn short(&self) -> String {
        if self.number > 0 {
            format!("build {}", self.number)
        } else {
            format!("build {}", self.commit)
        }
    }

    /// The whole of it: "build 3127 (d08583d, 2026-11-17)".
    pub fn long(&self) -> String {
        let mut more: Vec<&str> = Vec::new();
        if self.number > 0 {
            more.push(self.commit);
        }
        if !self.date.is_empty() {
            more.push(self.date);
        }
        if more.is_empty() {
            self.short()
        } else {
            format!("{} ({})", self.short(), more.join(", "))
        }
    }

    /// As the play log and the settings file write it, one word:
    /// "3127:d08583d:2026-11-17" (`parse` reads it back).
    pub fn word(&self) -> String {
        format!("{}:{}:{}", self.number, self.commit, self.date)
    }
}

/// A build read back from `Build::word` (a play log's `build=`, the
/// settings' last build played): its number, commit and date.
pub fn parse(word: &str) -> Option<(u32, String, String)> {
    let mut p = word.splitn(3, ':');
    let n = p.next()?.parse().ok()?;
    Some((n, p.next()?.to_string(), p.next().unwrap_or("").to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_games_files_are_found() {
        // In a test the compiled path is the repo, whatever LARDER_ROOT
        // says when it names a folder without the maps.
        assert!(root().join("Reports/larder-maps-2026-11-11/maps").is_dir());
        assert!(root().join("Reports/larder-campaign-2026-11-12/missions").is_dir());
    }

    #[test]
    fn a_build_says_its_commit_and_reads_back() {
        let b = build();
        assert!(!b.commit.is_empty());
        assert!(b.short().starts_with("build "));
        assert_eq!(parse(&b.word()), Some((b.number, b.commit.to_string(), b.date.to_string())));
        let b = Build { number: 3127, commit: "d08583d", date: "2026-11-17" };
        assert_eq!(b.short(), "build 3127");
        assert_eq!(b.long(), "build 3127 (d08583d, 2026-11-17)");
        let b = Build { number: 0, commit: "d08583d", date: "2026-11-17" };
        assert_eq!(b.short(), "build d08583d");
        assert_eq!(b.long(), "build d08583d (2026-11-17)");
    }
}
