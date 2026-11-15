//! **The play log** (whole-game review W13, 2026-11-15): what a round in the
//! window did, a line a game second, so a thread can read the owner's play
//! instead of asking for it. The window writes it beside the replay as
//! `larder-world-log-MAP-SEED.txt` (gitignored); `examples/playlog.rs` reads
//! one back and prints the summary (`Summary`).
//!
//! The replay (`Game::replay_text`) already holds the commands, but it
//! replays only on the machine that made it (CPU and GPU maths differ
//! across machines) and says nothing of what happened. The log says what
//! happened in plain numbers: the mould's walkers and mass, soldiers,
//! squads, the bank, income, shots and kills, the buildings standing, and
//! the window's frame times (p50 and p95 over the second), with each
//! command at the second it was given.
//!
//! The format is one record a line, `key=value` words after a tag:
//!
//! ```text
//! log map=planet:1:7 seed=7 mode=squads tier=Drifter planet=1
//! t=12 mould=1834 mass=1834 soldiers=6 squads=1 bank=40 income=120 pumped=0 shots=55 killed=300 pumps=0 turrets=0 walls=0 other=0 crystals=0.991 stuck=0 p50=3.1 p95=6.0
//! cmd t=12.4 order 0 move 310 200
//! end t=301 verdict=lost
//! ```
//!
//! A reader skips any word it doesn't know, so new words can be added.

use super::larder_world::{Game, Kind, World};

/// The writer: gathers a second's frame times and the commands since the
/// last line, and gives the text to append.
#[derive(Clone, Debug, Default)]
pub struct Playlog {
    frames: Vec<f32>,
    cmds: usize,
}

/// Frame-time quantile `q` (0-1) in milliseconds of `ms` (sorted here).
fn quantile(ms: &mut [f32], q: f32) -> f32 {
    if ms.is_empty() {
        return 0.0;
    }
    ms.sort_by(f32::total_cmp);
    ms[((ms.len() - 1) as f32 * q).round() as usize]
}

impl Playlog {
    /// The header line.
    pub fn head(map: &str, seed: u64, extra: &[(&str, String)]) -> String {
        let mut s = format!("log map={} seed={seed}", map.replace(' ', "_"));
        for (k, v) in extra {
            s += &format!(" {k}={}", v.replace(' ', "_"));
        }
        s + "\n"
    }

    /// One window frame took `ms` milliseconds.
    pub fn frame(&mut self, ms: f32) {
        self.frames.push(ms);
    }

    /// The commands given since the last call (from `Game::log`), one line
    /// each at their game second.
    pub fn commands<W: World>(&mut self, g: &Game<W>) -> String {
        let mut s = String::new();
        // A new round's log starts again from 0.
        if g.log.len() < self.cmds {
            self.cmds = 0;
        }
        for (t, c) in &g.log[self.cmds..] {
            let secs = t.saturating_sub(g.landed) as f32 / g.rules.tps as f32;
            s += &format!("cmd t={secs:.1} {}\n", c.to_line());
        }
        self.cmds = g.log.len();
        s
    }

    /// The second's line (with the commands before it), and the frame
    /// times start again.
    pub fn second<W: World>(&mut self, g: &Game<W>) -> String {
        let mut s = self.commands(g);
        let up = |k: fn(&Kind) -> bool| g.buildings.iter().filter(|b| b.up() && k(&b.kind)).count();
        let pumps = up(|k| *k == Kind::Pump);
        let turrets = up(|k| *k == Kind::Turret);
        let walls = up(|k| matches!(k, Kind::Wall { .. }));
        let other = g.buildings.iter().filter(|b| b.up()).count() - pumps - turrets - walls;
        let mut squads: Vec<u8> = g.soldiers.iter().filter(|x| x.alive).map(|x| x.squad).collect();
        squads.sort_unstable();
        squads.dedup();
        let full: f32 = g.world.crystals().iter().map(|c| c.1).sum::<f32>();
        let p50 = quantile(&mut self.frames, 0.5);
        let p95 = quantile(&mut self.frames, 0.95);
        self.frames.clear();
        s += &format!(
            "t={:.0} mould={} mass={} soldiers={} squads={} bank={:.0} income={:.0} pumped={:.0} shots={} killed={} pumps={pumps} turrets={turrets} walls={walls} other={other} charge={full:.0} took={:.3} stuck={} p50={p50:.1} p95={p95:.1}\n",
            g.time(),
            g.mould,
            g.world.mass(),
            g.alive(),
            squads.len(),
            g.bank,
            g.income(),
            g.pumped,
            g.shots,
            g.killed,
            g.take_share(),
            g.stuck_run,
        );
        s
    }
}

/// One second's numbers, read back.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Second {
    pub t: f32,
    pub mould: u32,
    pub mass: u32,
    pub soldiers: u32,
    pub squads: u32,
    pub bank: f32,
    pub income: f32,
    pub shots: u32,
    pub killed: u32,
    pub pumps: u32,
    pub turrets: u32,
    pub walls: u32,
    pub other: u32,
    pub p50: f32,
    pub p95: f32,
}

/// A log read back: its header words, seconds and commands.
#[derive(Clone, Debug, Default)]
pub struct Read {
    pub head: Vec<(String, String)>,
    pub secs: Vec<Second>,
    /// (game second, the command's line)
    pub cmds: Vec<(f32, String)>,
    pub end: Option<String>,
}

fn words(line: &str) -> impl Iterator<Item = (&str, &str)> {
    line.split_whitespace().filter_map(|w| w.split_once('='))
}

/// Reads a log; lines it can't read are skipped (a log cut off by a crash
/// still reads).
pub fn read(text: &str) -> Read {
    let mut r = Read::default();
    for line in text.lines() {
        let (tag, rest) = line.split_once(' ').unwrap_or((line, ""));
        match tag {
            "log" => r.head = words(rest).map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            "cmd" => {
                let (t, c) = rest.split_once(' ').unwrap_or((rest, ""));
                if let Some(t) = t.strip_prefix("t=").and_then(|v| v.parse().ok()) {
                    r.cmds.push((t, c.to_string()));
                }
            }
            "end" => r.end = Some(rest.to_string()),
            _ if tag.starts_with("t=") => {
                let mut s = Second::default();
                for (k, v) in words(line) {
                    let f = v.parse::<f32>().unwrap_or(0.0);
                    let u = f.max(0.0) as u32;
                    match k {
                        "t" => s.t = f,
                        "mould" => s.mould = u,
                        "mass" => s.mass = u,
                        "soldiers" => s.soldiers = u,
                        "squads" => s.squads = u,
                        "bank" => s.bank = f,
                        "income" => s.income = f,
                        "shots" => s.shots = u,
                        "killed" => s.killed = u,
                        "pumps" => s.pumps = u,
                        "turrets" => s.turrets = u,
                        "walls" => s.walls = u,
                        "other" => s.other = u,
                        "p50" => s.p50 = f,
                        "p95" => s.p95 = f,
                        _ => {}
                    }
                }
                r.secs.push(s);
            }
            _ => {}
        }
    }
    r
}

/// The reader's summary: what the review asked of the owner's play (time
/// to N soldiers, splits, pumps, shots per kill, buildings, frame times).
pub fn summary(r: &Read) -> String {
    let mut out = String::new();
    let head: Vec<String> = r.head.iter().map(|(k, v)| format!("{k} {v}")).collect();
    out += &format!("{}\n", head.join(", "));
    let Some(last) = r.secs.last() else {
        return out + "no seconds logged\n";
    };
    let clock = |t: f32| format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
    out += &format!(
        "played {} ({} seconds logged), {} commands{}\n",
        clock(last.t),
        r.secs.len(),
        r.cmds.len(),
        r.end.as_ref().map_or(String::new(), |e| format!(", {e}"))
    );
    let to = |n: u32| {
        r.secs
            .iter()
            .find(|s| s.soldiers >= n)
            .map_or("never".to_string(), |s| clock(s.t))
    };
    out += &format!(
        "soldiers: most {}; 10 at {}, 20 at {}, 30 at {}, 40 at {}\n",
        r.secs.iter().map(|s| s.soldiers).max().unwrap_or(0),
        to(10),
        to(20),
        to(30),
        to(40)
    );
    let most = |f: fn(&Second) -> u32| r.secs.iter().map(f).max().unwrap_or(0);
    let first = |f: fn(&Second) -> u32| {
        r.secs
            .iter()
            .find(|s| f(s) > 0)
            .map_or("never".to_string(), |s| clock(s.t))
    };
    out += &format!(
        "squads at once: most {} (first split {})\n",
        most(|s| s.squads),
        r.secs
            .iter()
            .find(|s| s.squads > 1)
            .map_or("never".to_string(), |s| clock(s.t))
    );
    out += &format!(
        "pumps: most {} (first {}); turrets most {} (first {}); walls most {}; other buildings most {}\n",
        most(|s| s.pumps),
        first(|s| s.pumps),
        most(|s| s.turrets),
        first(|s| s.turrets),
        most(|s| s.walls),
        most(|s| s.other)
    );
    out += &format!(
        "shots {}, killed {} ({:.1} a shot)\n",
        last.shots,
        last.killed,
        last.killed as f32 / last.shots.max(1) as f32
    );
    let peak = r.secs.iter().max_by_key(|s| s.mass).cloned().unwrap_or_default();
    out += &format!(
        "slime mass: peak {} at {}, last {}\n",
        peak.mass,
        clock(peak.t),
        last.mass
    );
    // A minute at a time: kills a second against the slime's change.
    out += "minute  kills/s  mass at end  income/s  soldiers\n";
    let mut prev = Second::default();
    for m in 0..=(last.t / 60.0).ceil() as u32 {
        let Some(s) = r.secs.iter().rfind(|s| s.t <= (m + 1) as f32 * 60.0) else {
            continue;
        };
        if s.t <= prev.t {
            continue;
        }
        let dt = (s.t - prev.t).max(1.0);
        out += &format!(
            "{:>6}  {:>7.1}  {:>11}  {:>8.1}  {:>8}\n",
            m + 1,
            (s.killed.saturating_sub(prev.killed)) as f32 / dt,
            s.mass,
            (s.income - prev.income) / dt,
            s.soldiers
        );
        prev = s.clone();
    }
    let mut p50: Vec<f32> = r.secs.iter().map(|s| s.p50).collect();
    let mut p95: Vec<f32> = r.secs.iter().map(|s| s.p95).collect();
    out += &format!(
        "frame ms: median of the seconds' p50 {:.1}, of their p95 {:.1}, worst p95 {:.1}\n",
        quantile(&mut p50, 0.5),
        quantile(&mut p95.clone(), 0.5),
        quantile(&mut p95, 1.0)
    );
    let mut kinds: Vec<(String, u32)> = Vec::new();
    for (_, c) in &r.cmds {
        let k = c.split_whitespace().next().unwrap_or("").to_string();
        match kinds.iter_mut().find(|(n, _)| *n == k) {
            Some(e) => e.1 += 1,
            None => kinds.push((k, 1)),
        }
    }
    kinds.sort_by_key(|a| std::cmp::Reverse(a.1));
    out += &format!(
        "commands: {}\n",
        kinds
            .iter()
            .map(|(k, n)| format!("{k} {n}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::larder_world::{race_world, Cmd, Order, Rules};

    #[test]
    fn a_log_reads_back_what_it_wrote() {
        let mut g = Game::new(race_world(1, 1, 1_000), Rules::default(), 1);
        let mut log = Playlog::default();
        let mut text = Playlog::head("planet:1:7", 7, &[("mode", "squads".into())]);
        let tps = g.rules.tps;
        for sec in 0..3 {
            if sec == 1 {
                let s = g.soldiers[0];
                g.apply(Cmd::Order(0, Order::Move(s.x + 20.0, s.y)));
                g.apply(Cmd::Buy);
            }
            for _ in 0..tps {
                g.step();
                log.frame(4.0);
            }
            log.frame(9.0);
            text += &log.second(&g);
        }
        text += "end t=3 verdict=test\n";
        let r = read(&text);
        assert_eq!(r.head[0], ("map".to_string(), "planet:1:7".to_string()));
        assert_eq!(r.secs.len(), 3);
        assert_eq!(r.secs[2].t, 3.0);
        assert_eq!(r.secs[2].soldiers as usize, g.alive());
        assert_eq!(r.secs[2].mass as usize, g.world.mass());
        assert_eq!(r.secs[0].p50, 4.0);
        assert_eq!(r.cmds.len(), g.log.len());
        assert!(r.cmds[0].1.starts_with("order 0"), "{:?}", r.cmds);
        let s = summary(&r);
        assert!(s.contains("played 0:03") && s.contains("commands: "), "{s}");
        // A cut-off line is skipped, not an error.
        let cut = read(&format!("{text}t=4 mould=1"));
        assert_eq!(cut.secs.len(), 4);
    }
}
