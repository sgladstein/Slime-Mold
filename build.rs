//! Stamps the build with where it came from (the Engine and tools lane's
//! first job, 2026-11-17: "the build tells Scott what it holds"). The game
//! reads the three values through `slime_mould::build::build()`, shows them
//! on the menu and writes them in the play log, so a play note or a log can
//! be matched to the code that was played.
//!
//! - `LARDER_BUILD_NUMBER`: the commits on this checkout's history
//!   (`git rev-list --count HEAD`) when the game was last rebuilt, so a
//!   newer build always has a bigger number. 0 when it cannot be counted: no git, no `.git` (a source
//!   archive), or a shallow clone, where the count would be the clone's
//!   depth and not the build (cloud sessions clone shallow, so their films
//!   say "build d08583d" with no number).
//! - `LARDER_BUILD_COMMIT`: the short hash, "unknown" without git.
//! - `LARDER_BUILD_DATE`: the commit's date (UTC, YYYY-MM-DD), "" without.
//!
//! `LARDER_BUILD=NUMBER:COMMIT:DATE` in the environment wins over git, for a
//! build made where the history is not (a CI job on a shallow checkout).
//!
//! It never fails the build: every miss falls back to the values above.
//!
//! Why the commit count and not a number kept in a file: a file that every
//! pull request bumps conflicts in every pull request, and the merge queue
//! must land exactly the commit it tested, so it cannot add a bump commit.

use std::path::Path;
use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    // TZ: the date below is the commit's day in UTC on every machine.
    let out = Command::new("git").args(args).env("TZ", "UTC").output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string()).filter(|s| !s.is_empty())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LARDER_BUILD");
    // Run again when the game's own files change, which is when cargo
    // rebuilds the game anyway. Not when the checkout merely moves
    // (`.git/logs/HEAD`, the first cut on 11-17): cargo rebuilds the game
    // whenever this script reruns, even when it prints the same stamp
    // (measured that day: a rerun with an unchanged stamp recompiled the
    // library and the window, 1 min 21 s in a cloud session), and most pulls of main bring only reports and notes, so the
    // owner's `play.cmd` would have rebuilt for minutes with no code
    // changed. The cost of this way: after a docs-only pull the game still
    // names the commit it was last built at, an older commit with the same
    // code, which is what a play log needs to name.
    for p in ["src", "assets", "news", "Cargo.toml", "Cargo.lock"] {
        if Path::new(p).exists() {
            println!("cargo:rerun-if-changed={p}");
        }
    }
    let given = std::env::var("LARDER_BUILD").ok().filter(|v| !v.is_empty());
    let (number, commit, date) = match given.as_deref().map(|v| v.splitn(3, ':').collect::<Vec<_>>()).as_deref() {
        Some([n, c, d]) => (n.parse::<u32>().unwrap_or(0), c.to_string(), d.to_string()),
        _ => {
            let shallow = git(&["rev-parse", "--is-shallow-repository"]).is_none_or(|s| s != "false");
            let number = if shallow { 0 } else { git(&["rev-list", "--count", "HEAD"]).and_then(|n| n.parse().ok()).unwrap_or(0) };
            let commit = git(&["rev-parse", "--short=7", "HEAD"]).unwrap_or_else(|| "unknown".into());
            // The commit's date in UTC whatever the committer's zone.
            let date = git(&["-c", "log.showSignature=false", "log", "-1", "--date=format-local:%Y-%m-%d", "--format=%cd"]).unwrap_or_default();
            (number, commit, date)
        }
    };
    println!("cargo:rustc-env=LARDER_BUILD_NUMBER={number}");
    println!("cargo:rustc-env=LARDER_BUILD_COMMIT={commit}");
    println!("cargo:rustc-env=LARDER_BUILD_DATE={date}");
}
