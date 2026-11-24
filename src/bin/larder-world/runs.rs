//! **The runs that ended** (Direction 11-24: "A Runs page off the main
//! menu. It lists past runs: the date, planets won, time played, how the
//! run ended (the planet and what killed the squad), and the deck and
//! bodies held. It also shows his best time per planet. Save it beside
//! run.txt").
//!
//! Each run that ends (won, lost, or given up from the shop) is written as
//! a block of `key=value` lines to `runs.txt` beside the run's save
//! (`App::run_file`), oldest first, a blank line between. The menu's
//! "Past runs" page (`start::PastRun`) reads them back newest first. A
//! block it cannot read is skipped and the rest kept; a key it does not
//! know is skipped, so a later build's lines read in this one.

use std::path::{Path, PathBuf};

use slime_mould::gpu::larder_world::GpuWorld;
use slime_mould::mould::campaign::{Cause, Verdict};
use slime_mould::mould::larder_world::Game;
use slime_mould::mould::run::{self, Run};
use slime_mould::world::start::{PastPlanet, PastRun};

use super::draw::title_case;
use super::App;

/// The file's first line, so a stray file is never read as runs.
const HEAD: &str = "# Plasmodia's ended runs, oldest first: a block a run, a blank line between";

/// `runs.txt` beside the run's save.
pub(super) fn path_beside(run_file: &Path) -> PathBuf {
    run_file.with_file_name("runs.txt")
}

/// The run `r` as it ended: `end` is "won", "lost" or "given up", `why`
/// the sentence saying what ended it.
pub(super) fn record(r: &Run, end: &str, why: String) -> PastRun {
    let reached = (r.depth as usize).max(r.stars.len()).min(run::PLANETS as usize);
    let planets = (1..=run::PLANETS)
        .map(|d| {
            let mut at = r.clone();
            at.depth = d;
            let stars = r.stars.get(d as usize - 1).copied();
            let secs = (d as usize <= reached).then(|| r.planet_secs(d)).flatten();
            PastPlanet { name: at.planet().name, slime: title_case(at.level().tier.name()), stars, secs }
        })
        .collect();
    PastRun {
        date: now_utc(),
        seed: r.seed,
        end: end.to_string(),
        depth: r.depth,
        played: r.played,
        planets,
        why,
        guns: r.guns.iter().map(|g| g.title()).collect(),
        cards: r.cards.iter().map(|(c, _)| c.title()).collect(),
        bodies: r.bodies.iter().map(|b| b.kind.name().to_string()).collect(),
        relics: r.relics.iter().map(|x| x.name().to_string()).collect(),
    }
}

/// What ended a lost planet, a sentence: the cause, when, and for a squad
/// wiped out, what its soldiers fell to (`Game::fell_*`, the squad's own
/// blasts).
pub(super) fn why_lost(v: Verdict, g: &Game<GpuWorld>, slime: &str) -> String {
    let at = format!("{}:{:02}", v.at as u32 / 60, v.at as u32 % 60);
    match v.cause {
        Cause::SquadWiped => {
            let parts: Vec<String> = [(g.fell_bitten, "the slime's touch"), (g.fell_burned, "fire"), (g.fell_starfall, "a falling star"), (g.rounds.blast_kills, "their own blasts")]
                .into_iter()
                .filter(|(n, _)| *n > 0)
                .map(|(n, what)| format!("{n} to {what}"))
                .collect();
            if parts.is_empty() {
                format!("{slime} wiped out the squad at {at}.")
            } else {
                format!("{slime} wiped out the squad at {at}: {}.", parts.join(", "))
            }
        }
        Cause::Overrun => format!("{slime} took the base at {at}, with no food in the bank to fight it off."),
        Cause::Stuck => format!("Out of food at {at}: nothing left to buy a soldier or fire a shot against {}.", lower_the(slime)),
        Cause::Cap => format!("The clock ran out at {at} with {} still holding on.", lower_the(slime)),
        Cause::MouldWiped | Cause::Goal => format!("{slime} was gone at {at}, but the planet's goal fell short."),
    }
}

/// "The Trunk" in the middle of a sentence: "the Trunk".
fn lower_the(slime: &str) -> String {
    slime.strip_prefix("The ").map_or_else(|| slime.to_string(), |rest| format!("the {rest}"))
}

/// One run as its block of lines.
pub(super) fn to_text(p: &PastRun) -> String {
    let clean = |t: &str| t.replace(['|', '\n', '\r'], " ");
    let list = |v: &[String]| v.iter().map(|t| clean(t)).collect::<Vec<_>>().join("|");
    let mut s = String::new();
    let mut put = |k: &str, v: String| s.push_str(&format!("{k}={v}\n"));
    put("run", clean(&p.date));
    put("seed", p.seed.to_string());
    put("end", clean(&p.end));
    put("depth", p.depth.to_string());
    put("played", format!("{:.1}", p.played));
    for pl in &p.planets {
        let stars = pl.stars.map_or("-".to_string(), |n| n.to_string());
        let secs = pl.secs.map_or("-".to_string(), |t| format!("{t:.1}"));
        put("planet", format!("{}|{}|{stars}|{secs}", clean(&pl.name), clean(&pl.slime)));
    }
    put("why", clean(&p.why));
    put("guns", list(&p.guns));
    put("cards", list(&p.cards));
    put("bodies", list(&p.bodies));
    put("relics", list(&p.relics));
    s
}

/// The runs in a file's text, oldest first; a block with no `run=` date
/// or end is skipped.
pub(super) fn parse(text: &str) -> Vec<PastRun> {
    let mut lines = text.lines();
    if lines.next().map(str::trim) != Some(HEAD) {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut cur: Option<PastRun> = None;
    let list = |v: &str| v.split('|').map(str::trim).filter(|t| !t.is_empty()).map(String::from).collect::<Vec<_>>();
    for l in lines {
        let Some((k, v)) = l.split_once('=') else {
            out.extend(cur.take().filter(|r| !r.end.is_empty()));
            continue;
        };
        let (k, v) = (k.trim(), v.trim());
        if k == "run" {
            out.extend(cur.take().filter(|r| !r.end.is_empty()));
            cur = Some(PastRun { date: v.to_string(), ..PastRun::default() });
            continue;
        }
        let Some(r) = cur.as_mut() else { continue };
        match k {
            "seed" => r.seed = v.parse().unwrap_or(0),
            "end" => r.end = v.to_string(),
            "depth" => r.depth = v.parse().unwrap_or(1),
            "played" => r.played = v.parse::<f32>().ok().filter(|t| t.is_finite() && *t >= 0.0).unwrap_or(0.0),
            "planet" => {
                let f: Vec<&str> = v.split('|').map(str::trim).collect();
                if f.len() == 4 {
                    r.planets.push(PastPlanet { name: f[0].into(), slime: f[1].into(), stars: f[2].parse().ok(), secs: f[3].parse::<f32>().ok().filter(|t| t.is_finite() && *t >= 0.0) });
                }
            }
            "why" => r.why = v.to_string(),
            "guns" => r.guns = list(v),
            "cards" => r.cards = list(v),
            "bodies" => r.bodies = list(v),
            "relics" => r.relics = list(v),
            _ => {}
        }
    }
    out.extend(cur.take().filter(|r| !r.end.is_empty()));
    out
}

/// The file's runs, oldest first (none if there is no file).
pub(super) fn load(p: &Path) -> Vec<PastRun> {
    std::fs::read_to_string(p).map_or_else(|_| Vec::new(), |t| parse(&t))
}

/// `p` with `run` added at its end (a side file renamed over it, as the
/// run's save is written).
pub(super) fn append(p: &Path, run: &PastRun) -> Result<(), String> {
    let old = std::fs::read_to_string(p).unwrap_or_default();
    let mut text = if old.lines().next().map(str::trim) == Some(HEAD) { old } else { format!("{HEAD}\n") };
    if !text.ends_with("\n\n") {
        text.push('\n');
    }
    text += &to_text(run);
    super::menu::write_save(p, &text)
}

/// "2026-11-24 14:05", now, in UTC (the game has no time zone table).
fn now_utc() -> String {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    date_of(t)
}

/// A Unix time as "YYYY-MM-DD HH:MM" (UTC), by Howard Hinnant's
/// days-to-civil.
fn date_of(t: u64) -> String {
    let days = (t / 86_400) as i64;
    let (hh, mm) = (t % 86_400 / 3600, t % 3600 / 60);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}")
}

/// Three ended runs for a film (`menu=runs`, `menu=main-runs`), newest
/// first, from `r`'s seed: given up on planet 2, lost on planet 4, and
/// won, at about 18 minutes a planet, with a few cards held.
pub(super) fn film_runs(r: &Run) -> Vec<PastRun> {
    use slime_mould::mould::rounds::Card;
    let mut base = r.clone();
    (base.depth, base.stars, base.splits, base.played, base.over) = (1, Vec::new(), Vec::new(), 0.0, false);
    for c in [Card::Pierce, Card::Split, Card::Gas, Card::Wide] {
        if !base.cards.iter().any(|(h, _)| *h == c) {
            base.cards.push((c, true));
        }
    }
    let play = |r: &mut Run, wins: &[u8], secs: &[f32]| {
        for (k, &w) in wins.iter().enumerate() {
            r.played += secs[k % secs.len()];
            r.won(w);
        }
    };
    let mut won = base.clone();
    play(&mut won, &[3, 2, 3, 1, 2, 3, 2, 3], &[905.0, 1210.0, 1033.0, 1388.0]);
    let mut lost = base.clone();
    play(&mut lost, &[2, 3, 1], &[1012.0, 1104.0, 1290.0]);
    lost.played += 731.0;
    lost.lost();
    let mut gave = base;
    play(&mut gave, &[1], &[1302.0]);
    let slime = title_case(lost.level().tier.name());
    let mut out = vec![
        record(&gave, "given up", format!("Given up in the shop before planet 2, with {}.", gave.line())),
        record(&lost, "lost", format!("{slime} wiped out the squad at 12:11: 5 to the slime's touch, 1 to fire.")),
        record(&won, "won", "All 8 planets freed, 19 of 24 stars.".into()),
    ];
    for (p, d) in out.iter_mut().zip(["2026-11-24 21:40", "2026-11-23 19:05", "2026-11-22 02:17"]) {
        p.date = d.into();
    }
    out
}

impl App {
    /// The runs file, beside the run's save; None when runs are not saved
    /// (a film, a test with no save).
    fn runs_file(&self) -> Option<PathBuf> {
        self.run_file.as_deref().map(path_beside)
    }

    /// The menu's list of ended runs from the file, newest first.
    pub(super) fn load_runs(&mut self) {
        let Some(p) = self.runs_file() else { return };
        let mut all = load(&p);
        all.reverse();
        if let Some(m) = self.menu.as_mut().or(self.parked.as_mut()) {
            m.runs = all;
        }
    }

    /// A run that ended (won, lost or given up): written to the file and
    /// put at the top of the menu's list.
    pub(super) fn keep_run(&mut self, r: &Run, end: &str, why: String) {
        if r.range {
            return;
        }
        let Some(p) = self.runs_file() else { return };
        let rec = record(r, end, why);
        if let Err(e) = append(&p, &rec) {
            eprintln!("larder-world: the ended run was not added to {} ({e})", p.display());
        }
        for m in [self.menu.as_mut(), self.parked.as_mut()].into_iter().flatten() {
            m.runs.insert(0, rec.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> PastRun {
        PastRun {
            date: "2026-11-24 14:05".into(),
            seed: 482_913,
            end: "lost".into(),
            depth: 2,
            played: 812.4,
            planets: vec![
                PastPlanet { name: "Dreskor".into(), slime: "The Drifter".into(), stars: Some(3), secs: Some(402.0) },
                PastPlanet { name: "Vell|a".into(), slime: "The Trunk".into(), stars: None, secs: Some(410.4) },
                PastPlanet { name: "Omm".into(), slime: "The Mire".into(), stars: None, secs: None },
            ],
            why: "The Trunk wiped out the squad at 6:50: 5 to the slime's touch, 1 to fire.".into(),
            guns: vec!["rifle".into(), "long gun".into()],
            cards: vec!["Pierce".into(), "Needle gas".into()],
            bodies: vec!["Grenade".into()],
            relics: Vec::new(),
        }
    }

    /// A run reads back as it was written (a `|` in a name becomes a
    /// space), two blocks read as two runs, a broken block is skipped,
    /// a later build's key is skipped, and a file that is not ours reads
    /// as none.
    #[test]
    fn ended_runs_read_back() {
        let a = sample();
        let mut b = sample();
        (b.end, b.date, b.why) = ("won".into(), "2026-11-25 09:00".into(), String::new());
        let text = format!("{HEAD}\n{}\n{}later_key=1\n\nrun=2026-11-26 00:00\nseed=4\n", to_text(&a), to_text(&b));
        let back = parse(&text);
        let (mut a2, mut b2) = (a.clone(), b.clone());
        a2.planets[1].name = "Vell a".into();
        b2.planets[1].name = "Vell a".into();
        assert_eq!(back, vec![a2, b2]);
        assert!(parse("seed=4\nend=lost\n").is_empty());
    }

    /// Appended to the file, oldest first; the file made if it is not
    /// there.
    #[test]
    fn ended_runs_append() {
        let dir = std::env::temp_dir().join(format!("larder-runs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let p = dir.join("runs.txt");
        let (a, mut b) = (sample(), sample());
        b.seed = 9;
        append(&p, &a).unwrap();
        append(&p, &b).unwrap();
        let back = load(&p);
        assert_eq!(back.iter().map(|r| r.seed).collect::<Vec<_>>(), vec![482_913, 9]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn dates_are_civil() {
        assert_eq!(date_of(0), "1970-01-01 00:00");
        // 2026-11-24 14:05 UTC.
        assert_eq!(date_of(1_795_529_100), "2026-11-24 14:05");
        assert_eq!(date_of(951_782_400), "2000-02-29 00:00");
    }
}
