//! **The Larder campaign's mission layer** (ROADMAP W5; the design is
//! `Reports/larder-campaign-2026-11-12.md`, approved by the owner
//! 2026-11-12 04:44Z). A mission is data over the shared game: a map, a
//! tier, a goal with its numbers, the verbs allowed, a bonus. This module
//! reads a mission file and follows a running `larder_world::Game` once a
//! game second, giving the player one goal bar, stars, and a lose cause,
//! and the census its dead time.
//!
//! **It never changes the game.** `End` and `Game::score` stay the Larder
//! thread's (the integration thread's ruling, 04:40Z): a mission's verdict
//! (`Verdict`) is this layer's own, read from what the game already counts
//! (soldiers, food, kills, walkers, the mould grid). The window stops the
//! round on a verdict; the census reads both. The reserve recruit and free
//! shots are game rules and belong to the Larder thread; a mission may set
//! `Rules` keys it names (`rule price 0`).
//!
//! **Why a separate mission file** and not lines in the map's `.txt`:
//! `map::Items::parse` refuses unknown lines (on purpose: a typo in a map
//! should fail), and one map serves several missions (the Fork is missions 2
//! and 4), so a mission names its map instead.
//!
//! **Positional goals need the mould grid.** Clearing a crystal or keeping
//! the mould in a region reads `World::mould_grid`, which the CPU world
//! gives and the GPU world does not yet (it needs an occupancy read-back).
//! Until then such a goal's bar is `None` on the GPU, and the stuck and
//! overrun checks fall back to what the game counts.

use super::larder::End;
use super::larder_world::{Cmd, Game, Order, Rules, World};
use super::map::MAP_SCALE;

/// Grid bin for the positional checks, fine cells (as the player AI's eyes).
pub const BIN: usize = 4;

/// What fills the bar.
#[derive(Clone, Debug, PartialEq)]
pub enum Goal {
    /// Soldiers alive reach `n` while soldiers harvest at least `crystals`
    /// different crystals (mission 1: harvest and hire). The crystals were
    /// added after the design review of the first census (2026-11-12): a
    /// squad size alone was met by 1:28 at 8 soldiers, and raising it to 24
    /// only lengthened the game; harvesting two crystals at once takes the
    /// squad out to where the mould is.
    Squad { n: usize, crystals: usize },
    /// At least `need` of these crystals (map order) have no walker on them
    /// for `secs` game seconds in a row (missions 2, 5, 7).
    ///
    /// With `retake`, a crystal counts only once the mould has been on it:
    /// the first census (2026-11-12) found the Fork's west crystal "clear"
    /// from the start, so every hand, idle included (best 0.22-0.87), filled
    /// the bar before the mould arrived, and the mission taught nothing.
    ///
    /// With `guard` (`goal hold`, which is also `retake`) a crystal counts
    /// only while a living soldier stands on it (within `GUARD` of its
    /// edge): the Drifter drifts since tiers #133, and on the plain retake
    /// random fire met First contact on 2 of 4 seeds by 2:49 (census v10),
    /// chasing the mould off a crystal and walking away; holding means
    /// standing there.
    Clear { crystals: Vec<usize>, need: usize, secs: f32, retake: bool, guard: bool },
    /// At most `leak` of the walkers outside this rectangle (map cells) for
    /// `secs` game seconds in a row (mission 3: keep the mould in the Rim).
    /// With no leak allowed (the first census) no hand ever held the Rim for
    /// a second: a few walkers always stray.
    ///
    /// With `body` above 0 a leak is what the player sees instead: no 4 x 4
    /// bin outside holds `body` walkers or more (thick mould; the drawn body
    /// is trail times `1 - exp(-walkers near / 6)`, so a thin trail of 1-3 a
    /// bin barely shows). The share must still be under `leak` (1 turns that
    /// off). On the Sprawler (census v6) every hand had 46% outside; the
    /// wall-the-gaps hand cut it to 17% with no thick bin (design review,
    /// 07:22Z: pick between the two rules on the Sprawler's numbers).
    Contain { rect: (f32, f32, f32, f32), secs: f32, leak: f32, body: u32 },
    /// Walkers fall to `frac` of their peak (mission 4: starve it; `shrink
    /// 0` is a wipe, mission 2). The peak counts from `SHRINK_FROM`: a new
    /// mould halves by itself in its first 40 s as walkers starve, and
    /// counted from the start `shrink 0.5` was met by every hand, idle
    /// included, at 0:20 (census v11).
    Shrink(f32),
    /// The squad's share of the charge taken reaches `frac` (mission 8).
    Take(f32),
}

/// A bonus that earns the third star (never a time: the design review,
/// 04:42Z, so the climax stays open).
#[derive(Clone, Debug, PartialEq)]
pub enum Bonus {
    /// Soldiers lost over the mission at most this many.
    LoseAtMost(usize),
    /// Food stolen from the larder at most this much.
    StolenAtMost(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mission {
    pub name: String,
    /// The goal sentence on screen.
    pub says: String,
    pub map: String,
    /// The mould's tier by name (`mould::tiers::Tier::from_name`, or a ladder
    /// name not yet built, which the window refuses).
    pub tier: Option<String>,
    pub goal: Goal,
    /// Verbs the command card offers (the window's command names).
    pub verbs: Vec<String>,
    pub bonus: Option<Bonus>,
    /// `Rules` keys this mission sets (`rule price 0`).
    pub rules: Vec<(String, f32)>,
}

impl Mission {
    /// One item a line; `#` starts a comment. Lines: `name WORDS`, `says
    /// WORDS`, `map NAME`, `tier NAME`, `verbs NAME...`, `rule KEY VALUE`,
    /// `bonus lose N` | `bonus stolen F`, and one goal: `goal squad N [K]`,
    /// `goal clear NEED SECS C...` (or `retake`: only crystals the mould has
    /// been on; `hold`: retake, and a soldier stands on it), `goal contain X0 Y0 X1 Y1 SECS LEAK`, `goal
    /// shrink F`, `goal take F`.
    pub fn parse(text: &str) -> Result<Mission, String> {
        let (mut name, mut says, mut map, mut tier, mut goal, mut verbs, mut bonus, mut rules) = (None, None, None, None, None, vec![], None, vec![]);
        for (n, line) in text.lines().enumerate() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let err = |m: String| format!("line {}: {m}", n + 1);
            let (kind, rest) = line.split_once(char::is_whitespace).map(|(k, r)| (k, r.trim())).unwrap_or((line, ""));
            let nums = |s: &str| s.split_whitespace().map(|w| w.parse::<f32>().map_err(|e| err(format!("{w:?}: {e}")))).collect::<Result<Vec<f32>, String>>();
            match kind {
                "name" => name = Some(rest.to_string()),
                "says" => says = Some(rest.to_string()),
                "map" => map = Some(rest.to_string()),
                "tier" => tier = Some(rest.to_string()),
                "verbs" => verbs = rest.split_whitespace().map(str::to_string).collect(),
                "rule" => {
                    let (k, v) = rest.split_once(char::is_whitespace).ok_or_else(|| err("`rule` takes KEY VALUE".into()))?;
                    let v = v.trim().parse::<f32>().map_err(|e| err(format!("{v:?}: {e}")))?;
                    if !RULE_KEYS.contains(&k) {
                        return Err(err(format!("unknown rule `{k}`")));
                    }
                    rules.push((k.to_string(), v));
                }
                "bonus" => {
                    let (k, v) = rest.split_once(char::is_whitespace).ok_or_else(|| err("`bonus` takes KIND N".into()))?;
                    let v = nums(v)?;
                    bonus = Some(match (k, v.as_slice()) {
                        ("lose", [x]) => Bonus::LoseAtMost(*x as usize),
                        ("stolen", [x]) => Bonus::StolenAtMost(*x),
                        _ => return Err(err(format!("bad bonus `{rest}`"))),
                    });
                }
                "goal" => {
                    if goal.is_some() {
                        return Err(err("one goal a mission".into()));
                    }
                    let (k, v) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
                    let v = nums(v)?;
                    goal = Some(match (k, v.as_slice()) {
                        ("squad", [x]) => Goal::Squad { n: *x as usize, crystals: 0 },
                        ("squad", [x, k]) => Goal::Squad { n: *x as usize, crystals: *k as usize },
                        ("clear" | "retake" | "hold", [need, secs, cs @ ..]) if !cs.is_empty() && (*need as usize) <= cs.len() => Goal::Clear { crystals: cs.iter().map(|&c| c as usize).collect(), need: *need as usize, secs: *secs, retake: k != "clear", guard: k == "hold" },
                        ("contain", [x0, y0, x1, y1, secs, leak]) => Goal::Contain { rect: (*x0, *y0, *x1, *y1), secs: *secs, leak: *leak, body: 0 },
                        ("contain", [x0, y0, x1, y1, secs, leak, body]) => Goal::Contain { rect: (*x0, *y0, *x1, *y1), secs: *secs, leak: *leak, body: *body as u32 },
                        ("shrink", [f]) => Goal::Shrink(*f),
                        ("take", [f]) => Goal::Take(*f),
                        _ => return Err(err(format!("bad goal `{rest}`"))),
                    });
                }
                _ => return Err(err(format!("unknown item `{kind}`"))),
            }
        }
        Ok(Mission {
            name: name.ok_or("no `name`")?,
            says: says.ok_or("no `says`")?,
            map: map.ok_or("no `map`")?,
            tier,
            goal: goal.ok_or("no `goal`")?,
            verbs,
            bonus,
            rules,
        })
    }

    /// The game's rules with this mission's keys set.
    pub fn rules(&self, mut r: Rules) -> Rules {
        // No soft lock (design, owner's pick 04:44Z), now the Game's own
        // rules (Larder PR #106): an overrun larder ends the mission, and a
        // wiped or broke squad gets a free recruit. A mission file can
        // still set either (0 turns it off).
        r.overrun = OVERRUN_END;
        r.reserve = RESERVE;
        for (k, v) in &self.rules {
            match k.as_str() {
                "price" => r.price = *v,
                "start_bank" => r.start_bank = *v,
                "squad" => r.squad = *v as usize,
                "soldier_cost" => r.soldier_cost = *v,
                "cap" => r.cap = *v,
                "smell" => r.smell = *v,
                "overrun" => r.overrun = *v,
                "reserve" => r.reserve = *v,
                _ => unreachable!("parse checks the keys"),
            }
        }
        r
    }
}

const RULE_KEYS: [&str; 8] = ["price", "start_bank", "squad", "soldier_cost", "cap", "smell", "overrun", "reserve"];

/// Seconds from the design: a stuck or overrun run this long ends the
/// mission; two stars need the goal before `TWO_STARS`.
pub const STUCK_END: f32 = 30.0;
pub const OVERRUN_END: f32 = 30.0;
/// `hold`: a soldier within this many dish cells of a crystal's edge holds
/// it. Soldiers stand at its rim, not on it, since crystals turned solid
/// (#153, `Rules::crystal_solid`), and spread round a goal by slot.
pub const GUARD: f32 = 5.0;
/// `Rules::reserve` for a mission: a free recruit every 20 game seconds
/// while fewer than two soldiers live and the bank can't buy one.
pub const RESERVE: f32 = 20.0;
/// `shrink`: game seconds before the peak starts counting (`Goal::Shrink`).
pub const SHRINK_FROM: f32 = 60.0;
pub const TWO_STARS: f32 = 720.0;
/// The arc's phase edges, game seconds (landing, contact, pressure, climax).
pub const PHASES: [f32; 3] = [120.0, 360.0, 660.0];

/// Why a mission ended.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cause {
    /// The bar filled.
    Goal,
    /// The 15:00 cap (graded by the bar).
    Cap,
    SquadWiped,
    /// The Game's `End::Overrun`: walkers on the larder with the bank empty
    /// for `Rules::overrun` (`OVERRUN_END` in a mission).
    Overrun,
    /// `Game::stuck_run` reached `STUCK_END`.
    Stuck,
    /// The mould was wiped; the goal is met if the bar can still fill
    /// (every goal but `Squad`), else graded like the cap.
    MouldWiped,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Verdict {
    pub cause: Cause,
    /// Game seconds.
    pub at: f32,
    /// 0 is a loss; 1 to 3 stars.
    pub stars: u8,
    /// The bar's best value, 0 to 1.
    pub best: f32,
}

/// A dead second's kind (the design's *Dead time*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dead {
    /// No affordable move: the bank can't buy a soldier, a wall cell, or a
    /// shot. The owner's playtest (04:13Z): "I ran out of energy and I had a
    /// bunch of soldiers but I couldn't fight the slime and I couldn't
    /// harvest crystal". An approximation of "no move could raise the bar",
    /// shared with the Larder thread's stuck count once that lands.
    Stuck,
    /// A move exists but nothing useful happened.
    Waiting,
}

/// Follows one mission over one game.
#[derive(Clone, Debug)]
pub struct Tracker {
    pub mission: Mission,
    /// The bar this second (None: the world can't show it yet).
    pub bar: Option<f32>,
    pub best: f32,
    /// The bar once a game second, for the census's "decided at".
    pub bars: Vec<Option<f32>>,
    pub verdict: Option<Verdict>,
    /// Dead seconds by phase, and the run lengths.
    pub dead: [u32; 4],
    pub stuck: [u32; 4],
    pub secs: [u32; 4],
    pub dead_run: u32,
    pub longest_dead: u32,
    pub stuck_run: u32,
    pub longest_stuck: u32,
    /// Stalled seconds: the bar flat, the mould no smaller than 30 s ago,
    /// nothing banked (the design review, 05:38Z: free shots make "stuck"
    /// unreachable, but a squad can fire forever at a mould that regrows
    /// while nothing moves; that is the owner's soft lock in a new form).
    pub stall_run: u32,
    pub longest_stall: u32,
    /// Walkers each game second, for "shrinking over 30 s".
    moulds: Vec<usize>,
    held: f32,
    /// Crystals the mould has been on (for `retake`).
    touched: Vec<bool>,
    /// The goal's count this second, for the HUD and the census: crystals
    /// clear, or walkers outside the region.
    pub count: Option<u32>,
    /// Contain goals: the share of the walkers outside the region now.
    pub outside: Option<f32>,
    /// Contain goals with `body`: bins outside with thick mould now.
    pub thick: Option<u32>,
    peak: usize,
    lost: usize,
    last: Snapshot,
}

#[derive(Clone, Debug, Default)]
struct Snapshot {
    harvested: f32,
    killed: u32,
    spent: f32,
    walls: u32,
    bar: f32,
    alive: usize,
    at: Vec<(f32, f32)>,
}

fn snap<W: World>(g: &Game<W>, bar: f32) -> Snapshot {
    Snapshot {
        harvested: g.harvested,
        killed: g.killed,
        spent: g.spent,
        walls: g.walls,
        bar,
        alive: g.alive(),
        at: g.soldiers.iter().map(|s| (s.x, s.y)).collect(),
    }
}

/// Walkers in the grid's bins whose centre falls in `f` (fine cells).
fn walkers_where(grid: &[u32], size: (usize, usize), f: impl Fn(f32, f32) -> bool) -> u32 {
    let gw = size.0.div_ceil(BIN);
    grid.iter()
        .enumerate()
        .filter(|&(i, &n)| n > 0 && f(((i % gw) * BIN) as f32 + BIN as f32 / 2.0, ((i / gw) * BIN) as f32 + BIN as f32 / 2.0))
        .map(|(_, &n)| n)
        .sum()
}

impl Tracker {
    pub fn new<W: World>(mission: Mission, g: &Game<W>) -> Tracker {
        Tracker {
            mission,
            bar: None,
            best: 0.0,
            bars: vec![],
            verdict: None,
            dead: [0; 4],
            stuck: [0; 4],
            secs: [0; 4],
            dead_run: 0,
            longest_dead: 0,
            stuck_run: 0,
            longest_stuck: 0,
            stall_run: 0,
            longest_stall: 0,
            moulds: vec![],
            held: 0.0,
            touched: vec![],
            count: None,
            outside: None,
            thick: None,
            peak: 0,
            lost: 0,
            last: snap(g, 0.0),
        }
    }

    /// The bar from the game and, for positional goals, the mould grid
    /// (`World::mould_grid(BIN)`); `dt` is game seconds since the last call.
    fn bar_now<W: World>(&mut self, g: &Game<W>, grid: Option<&[u32]>, dt: f32) -> Option<f32> {
        let size = g.world.size();
        // Map cells to fine cells: a map cell is 1/MAP_SCALE dish cells.
        let fine = g.world.scale() / MAP_SCALE;
        match &self.mission.goal {
            // From the squad the mission starts with, so an idle squad's bar is
            // 0, not the half a start of 4 of 8 would show (and grade a star).
            Goal::Squad { n, crystals } => {
                let s0 = g.rules.squad.min(n.saturating_sub(1)) as f32;
                let squad = ((g.alive() as f32 - s0) / (*n as f32 - s0)).clamp(0.0, 1.0);
                if *crystals == 0 {
                    return Some(squad);
                }
                let mut at: Vec<(i32, i32)> = g.soldiers.iter().filter(|s| s.alive).filter_map(|s| if let Order::Clear(x, y) = s.order { Some((x as i32, y as i32)) } else { None }).collect();
                at.sort();
                at.dedup();
                self.count = Some(at.len() as u32);
                Some(squad.min(at.len() as f32 / *crystals as f32).min(1.0))
            }
            Goal::Shrink(f) => {
                if self.peak == 0 {
                    return Some(0.0);
                }
                let peak = self.peak as f32;
                Some(((peak - g.mould as f32) / (peak * (1.0 - f))).clamp(0.0, 1.0))
            }
            // From an even share: `take_share` is 0.5 before any charge is
            // gone, which as share / f showed 83% of a 60% goal at 0:00.
            Goal::Take(f) => Some(((g.take_share() - 0.5) / (f - 0.5).max(1e-3)).clamp(0.0, 1.0)),
            Goal::Clear { crystals, need, secs, retake, guard } => {
                let grid = grid?;
                let cs = g.world.crystals();
                self.touched.resize(cs.len(), false);
                let mut clear = 0;
                for &c in crystals.iter().filter(|&&c| c < cs.len()) {
                    let ((x, y), r) = (cs[c].0, g.world.crystal_r(c));
                    let on = walkers_where(grid, size, |px, py| (px - x).hypot(py - y) <= r);
                    if on > 0 {
                        self.touched[c] = true;
                    } else if (!retake || self.touched[c]) && (!guard || g.soldiers.iter().any(|s| s.alive && (s.x - x).hypot(s.y - y) <= r + GUARD * g.world.scale())) {
                        clear += 1;
                    }
                }
                self.count = Some(clear as u32);
                self.held = if clear >= *need { self.held + dt } else { 0.0 };
                Some((self.held / secs).min(1.0))
            }
            Goal::Contain { rect, secs, leak, body } => {
                let grid = grid?;
                let (x0, y0, x1, y1) = (rect.0 * fine, rect.1 * fine, rect.2 * fine, rect.3 * fine);
                let outside = |px: f32, py: f32| px < x0 || px > x1 || py < y0 || py > y1;
                let out = walkers_where(grid, size, outside);
                let gw = size.0.div_ceil(BIN);
                let thick = if *body > 0 {
                    grid.iter().enumerate().filter(|&(i, &n)| n >= *body && outside(((i % gw) * BIN) as f32 + BIN as f32 / 2.0, ((i / gw) * BIN) as f32 + BIN as f32 / 2.0)).count() as u32
                } else {
                    0
                };
                self.count = Some(out);
                self.outside = Some(out as f32 / g.mould.max(1) as f32);
                self.thick = Some(thick);
                self.held = if out as f32 <= leak * g.mould.max(1) as f32 && thick == 0 { self.held + dt } else { 0.0 };
                Some((self.held / secs).min(1.0))
            }
        }
    }

    /// Follow the game for one more game second (or whatever `dt` passed).
    /// Returns the verdict once it is reached; later calls change nothing.
    pub fn second<W: World>(&mut self, g: &Game<W>, grid: Option<&[u32]>, dt: f32) -> Option<Verdict> {
        if self.verdict.is_some() {
            return self.verdict;
        }
        let t = g.time();
        if g.time() >= SHRINK_FROM {
            self.peak = self.peak.max(g.mould);
        }
        let alive = g.alive();
        self.lost += self.last.alive.saturating_sub(alive);
        let bar = self.bar_now(g, grid, dt);
        self.bar = bar;
        self.bars.push(bar);
        let b = bar.unwrap_or(self.last.bar);
        self.best = self.best.max(b);

        // Dead time: did anything the player owns do something useful?
        let r = &g.rules;
        let walked = g.soldiers.iter().enumerate().any(|(i, s)| {
            s.alive && !matches!(s.order, Order::Hold) && self.last.at.get(i).is_some_and(|&(x, y)| (s.x - x).hypot(s.y - y) > 0.1)
        });
        // Kills count only while the mould is smaller than 30 s ago: firing
        // at a mould that regrows as fast is not progress (design review,
        // 05:38Z).
        self.moulds.push(g.mould);
        let ago = self.moulds.len().saturating_sub(31);
        let shrinking = g.mould < self.moulds[ago];
        let banked = g.harvested > self.last.harvested + 1e-3;
        let rising = b > self.last.bar + 1e-4;
        self.stall_run = if rising || shrinking || banked { 0 } else { self.stall_run + 1 };
        self.longest_stall = self.longest_stall.max(self.stall_run);
        let useful = banked
            || (g.killed > self.last.killed && shrinking)
            || g.spent > self.last.spent + 1e-3
            || g.walls > self.last.walls
            || rising
            || walked;
        let phase = PHASES.iter().filter(|&&p| t >= p).count();
        self.secs[phase] += 1;
        if useful {
            self.dead_run = 0;
        } else {
            self.dead[phase] += 1;
            self.dead_run += 1;
            self.longest_dead = self.longest_dead.max(self.dead_run);
        }
        // Stuck is the Game's own count since Larder PR #106 (`Game::stuck`:
        // a soldier alive, no shot, no food taken, the bank below a
        // soldier's price). Until then the tracker counted it here, as a
        // dead second with the bank below both prices.
        if g.stuck_run > 0 {
            self.stuck[phase] += 1;
        }
        self.stuck_run = g.stuck_run;
        self.longest_stuck = self.longest_stuck.max(self.stuck_run);
        self.last = snap(g, b);

        let met = b >= 1.0 && !matches!(self.mission.goal, Goal::Squad { .. } if alive == 0);
        let bonus = match self.mission.bonus {
            Some(Bonus::LoseAtMost(n)) => self.lost <= n,
            Some(Bonus::StolenAtMost(f)) => g.stolen <= f,
            None => false,
        };
        let graded = |best: f32| if best >= 0.5 { 1 } else { 0 };
        let v = |cause, stars| Some(Verdict { cause, at: t, stars, best: self.best });
        self.verdict = if met {
            v(Cause::Goal, 1 + u8::from(t < TWO_STARS) + u8::from(bonus))
        } else if matches!(g.end, Some(End::SquadWiped(_))) {
            // With the reserve on, an empty squad is no loss while a recruit
            // can still come; the Game ends the round when none can.
            v(Cause::SquadWiped, 0)
        } else if matches!(g.end, Some(End::Overrun(_))) {
            v(Cause::Overrun, 0)
        } else if self.stuck_run as f32 >= STUCK_END {
            v(Cause::Stuck, graded(self.best))
        } else if matches!(g.end, Some(End::MouldWiped(_))) {
            // Only a squad goal can't be met by wiping the mould.
            match self.mission.goal {
                Goal::Squad { .. } => v(Cause::MouldWiped, graded(self.best)),
                _ => v(Cause::Goal, 1 + u8::from(t < TWO_STARS) + u8::from(bonus)),
            }
        } else if t >= r.cap || matches!(g.end, Some(End::Cap(_))) {
            v(Cause::Cap, graded(self.best))
        } else {
            None
        };
        self.verdict
    }

    /// Dead seconds over all seconds followed.
    pub fn dead_share(&self) -> f32 {
        self.dead.iter().sum::<u32>() as f32 / self.secs.iter().sum::<u32>().max(1) as f32
    }

    /// The design's census targets on the intended plan's hand: dead share
    /// under 15%, no phase over 25%, longest dead run under 45 s, no stuck
    /// run over 30 s.
    pub fn pacing_ok(&self) -> bool {
        self.dead_share() < 0.15
            && (0..4).all(|p| self.secs[p] == 0 || (self.dead[p] as f32) < 0.25 * self.secs[p] as f32)
            && self.longest_dead < 45
            && (self.longest_stuck as f32) < STUCK_END
    }

    /// The goal line the player reads: the mission's sentence and its bar,
    /// and for a contain goal the share outside now against the allowance.
    /// The window's picture shows the mould's trail outside the Rim long
    /// before the share passes the allowance (a winning hand keeps 2-5% of
    /// its walkers outside, thinly, over 100-240 4 x 4 bins with 1-3 walkers
    /// each, census v5): said on screen, the player sees why the bar still
    /// holds (design review, 06:26Z: the bar and the picture must agree).
    pub fn progress_line(&self) -> String {
        let says = &self.mission.says;
        match (self.bar, &self.mission.goal, self.outside) {
            (Some(b), Goal::Contain { body, .. }, _) if *body > 0 => match self.thick {
                Some(0) | None => format!("{says} ({:.0}%)", 100.0 * b),
                Some(n) => format!("{says} ({:.0}%; thick mould outside in {n} places)", 100.0 * b),
            },
            (Some(b), Goal::Contain { leak, .. }, Some(o)) => format!("{says} ({:.0}%; {:.1}% outside, {:.0}% allowed)", 100.0 * b, 100.0 * o, 100.0 * leak),
            (Some(b), _, _) => format!("{says} ({:.0}%)", 100.0 * b),
            // Said, not hidden (design review, 05:38Z): a world with no
            // mould grid gives a goal on where the mould is no bar.
            (None, _, _) if self.secs.iter().sum::<u32>() > 0 => format!("{says} (progress not shown in this build yet)"),
            (None, _, _) => says.clone(),
        }
    }

    /// The line a lost or capped mission ends on.
    pub fn cause_line(&self) -> String {
        let Some(v) = self.verdict else { return String::new() };
        let at = format!("{}:{:02}", v.at as u32 / 60, v.at as u32 % 60);
        let best = format!("best {:.0}% of the goal", 100.0 * v.best);
        match v.cause {
            Cause::Goal => format!("Goal met at {at}: {} star{}", v.stars, if v.stars == 1 { "" } else { "s" }),
            Cause::Cap => format!("Time: {} ({best})", if v.stars > 0 { "held on, 1 star" } else { "lost" }),
            Cause::SquadWiped => format!("Your last soldier fell at {at} ({best})"),
            Cause::Overrun => format!("The mould took the larder at {at} with the bank empty ({best})"),
            Cause::Stuck => format!("No food to buy or fire with since {} ({best})", {
                let s = (v.at - STUCK_END).max(0.0) as u32;
                format!("{}:{:02}", s / 60, s % 60)
            }),
            Cause::MouldWiped => format!("The mould is gone at {at}, but the goal was not met ({best})"),
        }
    }
}

/// The earliest game second after which the stars no longer change, given
/// each second's bar and the final stars (the design's "decided at"): the
/// first second from which the bar's star band (under half, half to full,
/// full) stays the final one.
pub fn decided_at(bars: &[Option<f32>], final_stars: u8) -> usize {
    let band = |b: f32| {
        if b >= 1.0 {
            2
        } else if b >= 0.5 {
            1
        } else {
            0
        }
    };
    let last = match final_stars {
        0 => 0,
        1 => 1,
        _ => 2,
    };
    let mut at = bars.len();
    for (i, b) in bars.iter().enumerate().rev() {
        match b {
            Some(b) if band(*b) != last => break,
            _ => at = i,
        }
    }
    at
}

/// The Rim's gaps (rock ring radius 200 map cells round (512, 320), 16
/// thick, gaps at 0, 90, 180, 270 degrees, half widths 14, 10, 14, 10:
/// `sketch_maps.py`): soldier k walks to gap k's middle and walls its chord
/// at the ring's centre line, 3 degrees past each edge, and stays to raise
/// it. `walled[i]` turns true once gap i's wall is placed. Mission 3's own
/// plan, for the census (`examples/campaign.rs` hand `gaps+STYLE`) and the
/// smoke test that it still wins.
pub fn wall_rim_gaps<W: World>(g: &mut Game<W>, walled: &mut [bool; 4]) {
    const GAPS: [(f32, f32); 4] = [(0.0, 14.0), (90.0, 10.0), (180.0, 14.0), (270.0, 10.0)];
    let f = g.world.scale() / MAP_SCALE;
    let at = |deg: f32| {
        let a = deg.to_radians();
        ((512.0 + 200.0 * a.cos()) * f, (320.0 + 200.0 * a.sin()) * f)
    };
    let live: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect();
    for (i, &(deg, half)) in GAPS.iter().enumerate() {
        if walled[i] {
            continue;
        }
        let Some(&k) = live.get(i) else { return };
        let mid = at(deg);
        let s = g.soldiers[k];
        if (s.x - mid.0).hypot(s.y - mid.1) < 3.0 * g.world.scale() {
            // Placed walls rise only while a builder raises them (#122,
            // `BuildRules::raise` 8 s a 10 dish cells): this soldier stays.
            walled[i] = g.apply(Cmd::Wall(at(deg - half - 3.0), at(deg + half + 3.0)));
            if walled[i] {
                let b = g.buildings.len() - 1;
                g.apply(Cmd::Order(k, Order::Build(b)));
            }
        } else if s.order != Order::Move(mid.0, mid.1) {
            g.apply(Cmd::Order(k, Order::Move(mid.0, mid.1)));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::larder_world::race_world;

    const LANDFALL: &str = "name Landfall\nsays Grow your squad to 8 soldiers\nmap flats\ntier drifter\nverbs harvest buy\nrule price 0\nbonus lose 2\ngoal squad 8\n";

    #[test]
    fn a_mission_file_parses_and_refuses_typos() {
        let m = Mission::parse(LANDFALL).unwrap();
        assert_eq!(m.goal, Goal::Squad { n: 8, crystals: 0 });
        assert_eq!(Mission::parse(&LANDFALL.replace("goal squad 8", "goal squad 8 2")).unwrap().goal, Goal::Squad { n: 8, crystals: 2 });
        assert_eq!(m.bonus, Some(Bonus::LoseAtMost(2)));
        assert_eq!(m.rules(Rules::default()).price, 0.0);
        assert!(Mission::parse(&LANDFALL.replace("goal squad 8", "goal sqaud 8")).is_err());
        assert!(Mission::parse(&LANDFALL.replace("rule price 0", "rule prise 0")).is_err());
        assert!(Mission::parse(&format!("{LANDFALL}goal take 0.6\n")).is_err());
        let c = Mission::parse(&LANDFALL.replace("goal squad 8", "goal clear 6 60 1 2 3 4 5 6 7 8")).unwrap();
        assert_eq!(c.goal, Goal::Clear { crystals: (1..=8).collect(), need: 6, secs: 60.0, retake: false, guard: false });
        let c = Mission::parse(&LANDFALL.replace("goal squad 8", "goal hold 1 60 1 2")).unwrap();
        assert!(matches!(c.goal, Goal::Clear { retake: true, guard: true, .. }));
        let c = Mission::parse(&LANDFALL.replace("goal squad 8", "goal retake 1 60 1")).unwrap();
        assert!(matches!(c.goal, Goal::Clear { retake: true, .. }));
        let c = Mission::parse(&LANDFALL.replace("goal squad 8", "goal contain 0 0 10 10 60 1 16")).unwrap();
        assert!(matches!(c.goal, Goal::Contain { body: 16, .. }));
        assert!(Mission::parse(&LANDFALL.replace("goal squad 8", "goal contain 0 0 10 10 60")).is_err(), "contain needs its leak");
        assert!(Mission::parse(&LANDFALL.replace("goal squad 8", "goal clear 3 60 1 2")).is_err());
    }

    #[test]
    fn the_campaign_files_parse() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-campaign-2026-11-12/missions");
        let mut n = 0;
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|x| x == "txt") {
                Mission::parse(&std::fs::read_to_string(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
                n += 1;
            }
        }
        assert!(n >= 3);
    }

    /// An idle squad on the race world: the squad goal never fills, the
    /// mission ends (wiped, overrun or the cap) with no stars, and an idle
    /// squad's seconds are dead.
    #[test]
    fn an_idle_squad_does_not_meet_a_squad_goal() {
        let mut g = Game::new(race_world(1, 2, 2000), Rules { cap: 120.0, ..Rules::default() }, 1);
        let mut tr = Tracker::new(Mission::parse(LANDFALL).unwrap(), &g);
        let mut v = None;
        while v.is_none() {
            let s = g.world.ticks() / g.rules.tps as u64;
            while g.world.ticks() / g.rules.tps as u64 == s && g.end.is_none() {
                g.step();
            }
            let grid = g.world.mould_grid(BIN);
            v = tr.second(&g, grid.as_deref(), 1.0);
        }
        let v = v.unwrap();
        assert_eq!(v.stars, 0, "{v:?}");
        assert!(v.best < 1.0);
        assert!(tr.dead_share() > 0.9, "idle is dead time: {}", tr.dead_share());
        assert!(!tr.pacing_ok());
    }

    /// Buying soldiers fills the squad bar, and the goal ends the mission
    /// with stars before the cap.
    #[test]
    fn hiring_fills_the_squad_goal() {
        let mut g = Game::new(race_world(1, 2, 2000), Rules { start_bank: 10_000.0, ..Rules::default() }, 1);
        let mut tr = Tracker::new(Mission::parse(LANDFALL).unwrap(), &g);
        let mut v = None;
        while v.is_none() {
            if g.alive() < 8 {
                g.buy();
            }
            let s = g.world.ticks() / g.rules.tps as u64;
            while g.world.ticks() / g.rules.tps as u64 == s && g.end.is_none() {
                g.step();
            }
            v = tr.second(&g, None, 1.0);
        }
        let v = v.unwrap();
        assert_eq!(v.cause, Cause::Goal, "{v:?}");
        assert!(v.stars >= 2 && v.at < 60.0, "{v:?}");
        assert!(tr.cause_line().starts_with("Goal met"));
    }

    /// Plays `secs` game seconds (or to the verdict) with `hand` called once
    /// a second, the tracker following with the CPU world's mould grid.
    fn follow(g: &mut Game<crate::mould::big::BigDish>, tr: &mut Tracker, secs: f32, mut hand: impl FnMut(&mut Game<crate::mould::big::BigDish>)) -> Option<Verdict> {
        let tps = g.rules.tps as u64;
        while g.time() < secs {
            hand(g);
            let s = g.world.ticks() / tps;
            while g.world.ticks() / tps == s && g.end.is_none() {
                g.step();
            }
            let grid = g.world.mould_grid(BIN);
            if let Some(v) = tr.second(g, grid.as_deref(), 1.0) {
                return Some(v);
            }
        }
        None
    }

    fn mission(goal: &str) -> Mission {
        Mission::parse(&LANDFALL.replace("goal squad 8", goal).replace("rule price 0\n", "")).unwrap()
    }

    fn small(rules: Rules) -> Game<crate::mould::big::BigDish> {
        Game::new(race_world(1, 2, 2000), rules, 1)
    }

    /// The crystal nearest the larder (the mould starts on crystal 0, far
    /// away) and the mould's home crystal.
    fn near_and_home(g: &Game<crate::mould::big::BigDish>) -> (usize, usize) {
        let cs = g.world.crystals();
        let d = |c: &((f32, f32), f32)| (c.0 .0 - g.larder.0).hypot(c.0 .1 - g.larder.1);
        let near = (0..cs.len()).min_by(|&a, &b| d(&cs[a]).total_cmp(&d(&cs[b]))).unwrap();
        (near, 0)
    }

    /// `clear` fills on a crystal the mould has not reached; `retake` does
    /// not count it until the mould has been on it, and the mould's own home
    /// crystal is never clear for an idle squad.
    #[test]
    fn clear_and_retake() {
        let mut g = small(Rules::default());
        let (near, home) = near_and_home(&g);
        let mut tr = Tracker::new(mission(&format!("goal clear 1 10 {near}")), &g);
        let v = follow(&mut g, &mut tr, 30.0, |_| {}).expect("an untouched crystal is clear");
        assert_eq!(v.cause, Cause::Goal);
        assert_eq!(tr.count, Some(1));

        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission(&format!("goal retake 1 10 {near}")), &g);
        assert!(follow(&mut g, &mut tr, 30.0, |_| {}).is_none(), "retake: the mould was never there");
        assert_eq!(tr.best, 0.0);

        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission(&format!("goal retake 1 10 {home}")), &g);
        assert!(follow(&mut g, &mut tr, 30.0, |_| {}).is_none());
        assert_eq!(tr.best, 0.0, "the home crystal stays the mould's");

        // `hold`: a crystal counts only with a soldier on it. On a crystal
        // the mould never reached (guard alone, retake off) the squad
        // walked onto it fills the bar; walked the other way it does not.
        let held = |g: &Game<crate::mould::big::BigDish>, near: usize| {
            let mut m = mission(&format!("goal hold 1 20 {near}"));
            if let Goal::Clear { retake, .. } = &mut m.goal {
                *retake = false;
            }
            Tracker::new(m, g)
        };
        let mut g = small(Rules::default());
        let (near, _) = near_and_home(&g);
        let at = g.world.crystals()[near].0;
        let mut tr = held(&g, near);
        let v = follow(&mut g, &mut tr, 40.0, |g| {
            for s in g.soldiers.iter_mut() {
                s.order = Order::Move(at.0, at.1);
            }
        });
        assert_eq!(v.map(|v| v.cause), Some(Cause::Goal), "a soldier on the crystal holds it");
        let mut g = small(Rules::default());
        let mut tr = held(&g, near);
        // The larder is on that crystal: walk well clear of it.
        let away = (at.0 - 120.0, at.1);
        assert!(follow(&mut g, &mut tr, 40.0, |g| {
            for s in g.soldiers.iter_mut() {
                s.order = Order::Move(away.0, away.1);
            }
        }).is_none(), "nobody on it, not held: best {}", tr.best);
    }

    /// `contain` fills over the whole world and never in a corner; the leak
    /// is a share of the walkers.
    #[test]
    fn contain_whole_world_and_corner() {
        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission("goal contain 0 0 1024 640 10 0"), &g);
        let v = follow(&mut g, &mut tr, 30.0, |_| {}).expect("all inside the world");
        assert_eq!(v.cause, Cause::Goal);
        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission("goal contain 0 0 8 8 10 0.02"), &g);
        assert!(follow(&mut g, &mut tr, 30.0, |_| {}).is_none());
        assert_eq!(tr.best, 0.0);
        assert!(tr.count.unwrap() > 1000, "walkers outside: {:?}", tr.count);
        // The goal line says how much is outside against the allowance.
        let line = tr.progress_line();
        assert!(line.contains("% outside, 2% allowed"), "{line}");
        // The thick-mould rule: share off (1), no bin outside with 16+.
        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission("goal contain 0 0 8 8 10 1 16"), &g);
        assert!(follow(&mut g, &mut tr, 30.0, |_| {}).is_none());
        assert!(tr.thick.unwrap() > 0, "thick bins outside: {:?}", tr.thick);
        assert!(tr.progress_line().contains("thick mould outside in"), "{}", tr.progress_line());
    }

    /// An idle squad takes no charge: `take` stays at 0 (its bar starts from
    /// an even share). `shrink` counts its peak from `SHRINK_FROM`, after the
    /// new mould has halved by itself (best 0.50 at 40 s when it counted
    /// from the start), so an idle squad's bar is still empty at 40 s.
    #[test]
    fn take_stays_empty_for_idle() {
        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission("goal take 0.6"), &g);
        assert!(follow(&mut g, &mut tr, 40.0, |_| {}).is_none());
        assert!(tr.best < 0.05, "best {}", tr.best);
        let mut g = small(Rules::default());
        let mut tr = Tracker::new(mission("goal shrink 0.5"), &g);
        assert!(follow(&mut g, &mut tr, 40.0, |_| {}).is_none());
        assert_eq!(tr.best, 0.0);
    }

    /// A squad that has no food and pays for shots is stuck: after 30 s the
    /// mission ends on Stuck (the owner's playtest, 04:13Z).
    #[test]
    fn a_broke_squad_is_stuck() {
        let mut g = small(Rules { start_bank: 0.0, price: 10.0, ..Rules::default() });
        let mut tr = Tracker::new(mission("goal take 0.6"), &g);
        let v = follow(&mut g, &mut tr, 60.0, |_| {}).expect("stuck by 0:31");
        assert_eq!(v.cause, Cause::Stuck, "{v:?}");
        assert!(v.at <= 32.0, "{v:?}");
        assert!(tr.cause_line().starts_with("No food"));
    }

    /// The squad goal with crystals: hired soldiers alone fill only the
    /// squad half; sent to harvest two crystals, the bar fills.
    #[test]
    fn squad_and_crystals() {
        let mut g = small(Rules { start_bank: 10_000.0, ..Rules::default() });
        let mut tr = Tracker::new(mission("goal squad 6 2"), &g);
        assert!(follow(&mut g, &mut tr, 10.0, |g| {
            if g.alive() < 6 {
                g.buy();
            }
        })
        .is_none());
        assert_eq!(tr.bar, Some(0.0), "no crystal harvested yet");
        let cs: Vec<(f32, f32)> = g.world.crystals().iter().skip(1).take(2).map(|c| c.0).collect();
        g.apply(crate::mould::larder_world::Cmd::Order(0, Order::Clear(cs[0].0, cs[0].1)));
        g.apply(crate::mould::larder_world::Cmd::Order(1, Order::Clear(cs[1].0, cs[1].1)));
        let v = follow(&mut g, &mut tr, 20.0, |_| {}).expect("six soldiers on two crystals");
        assert_eq!(v.cause, Cause::Goal);
    }

    #[test]
    fn decided_at_finds_the_last_band_change() {
        let bars = [Some(0.1), Some(0.6), Some(0.4), Some(0.7), Some(1.0)];
        assert_eq!(decided_at(&bars, 2), 4);
        assert_eq!(decided_at(&bars[..4], 1), 3);
        assert_eq!(decided_at(&[Some(0.2), None, Some(0.3)], 0), 0);
    }

    /// **Each mission's plan still wins** (tech lead, 14:13Z): missions 1-3
    /// rest on rules other threads own (walls, raise time, craters, the
    /// core), so these play each one's best census hand on seed 1 at the
    /// census's size. The full census is `examples/campaign.rs`; these are
    /// its v8 rows (seeds 1-8: Landfall claim 8/8 by 2:48, First contact
    /// strike 8/8 by 6:19, The gaps walling 24/24 at 10:00).
    ///
    /// CI runs the quick one (`the_gaps_needs_walls_and_landfall_wins`,
    /// about 12 s: the tech lead's cap is 30 s with ten threads merging
    /// through one queue); this full one, with First contact's 7:00 round,
    /// took 105 s and is `#[ignore]`d: `cargo test --release --
    /// --ignored each_missions_plan`.
    #[test]
    #[ignore]
    fn each_missions_plan_still_wins() {
        use crate::mould::big::BigDish;
        use crate::mould::larder_world::race_params;
        use crate::mould::larder_world_ai::Hand;
        use crate::mould::map::Map;
        use crate::mould::tiers::Tier;
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let play = |name: &str, hand: &str, walls: bool, secs: f32| -> (Option<Verdict>, f32) {
            let m = Mission::parse(&std::fs::read_to_string(root.join(format!("Reports/larder-campaign-2026-11-12/missions/{name}.txt"))).unwrap()).unwrap();
            let map = Map::load(&root.join("Reports/larder-maps-2026-11-11/maps"), &m.map).unwrap();
            let mut p = race_params(1, 2, 5000);
            if let Some(t) = m.tier.as_deref().and_then(Tier::from_name) {
                t.apply(&mut p);
            }
            let mut g = Game::new(BigDish::with_map(p, &map), m.rules(Rules::default()), 1);
            let mut tr = Tracker::new(m, &g);
            let mut ai = Hand::named(hand, &g).unwrap();
            let mut walled = [!walls; 4];
            let v = follow(&mut g, &mut tr, secs, |g| {
                if walled.iter().any(|w| !w) && g.time() < 120.0 {
                    wall_rim_gaps(g, &mut walled);
                } else {
                    ai.play(g);
                }
            });
            (v, tr.best)
        };
        // The four rounds side by side: 105 s on a 4-core box (140 one
        // after another); First contact's wipe (7:17-8:01 on headless
        // moulds, census v11) is the long pole.
        let [m1, m2, walls, open] = std::thread::scope(|sc| {
            let play = &play;
            [
                sc.spawn(move || play("1-landfall", "claim", false, 200.0)),
                sc.spawn(move || play("2-first-contact", "strike", false, 540.0)),
                sc.spawn(move || play("3-the-gaps", "claim", true, 180.0)),
                sc.spawn(move || play("3-the-gaps", "claim", false, 180.0)),
            ]
            .map(|h| h.join().unwrap())
        });
        assert!(m1.0.is_some_and(|v| v.cause == Cause::Goal), "Landfall, claim: {:?}", m1.0);
        assert!(m2.0.is_some_and(|v| v.cause == Cause::Goal), "First contact, strike: {:?}", m2.0);
        assert!(walls.0.is_none() && walls.1 >= 0.25, "The gaps, walls: {:?} best {}", walls.0, walls.1);
        assert!(open.1 < 0.2, "The gaps without walls held: best {}", open.1);
    }


    /// The quick CI check (11.8 s on 4 cores; seed 1, measured
    /// 2026-11-12: walls standing by 0:50, bar 0.23 at 2:20 with no thick
    /// mould outside; without walls thick mould is outside by 2:00 and the
    /// bar is back to 0). The fragile mission is The gaps, which rests on
    /// wall and raise rules; Landfall is cheap enough to keep.
    #[test]
    fn the_gaps_needs_walls_and_landfall_wins() {
        use crate::mould::big::BigDish;
        use crate::mould::larder_world::race_params;
        use crate::mould::larder_world_ai::Hand;
        use crate::mould::map::Map;
        use crate::mould::tiers::Tier;
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // A round to `secs`, or until `done` says it is decided.
        let play = |name: &str, walls: bool, secs: f32, done: fn(&Tracker) -> bool| -> Tracker {
            let m = Mission::parse(&std::fs::read_to_string(root.join(format!("Reports/larder-campaign-2026-11-12/missions/{name}.txt"))).unwrap()).unwrap();
            let map = Map::load(&root.join("Reports/larder-maps-2026-11-11/maps"), &m.map).unwrap();
            let mut p = race_params(1, 2, 5000);
            if let Some(t) = m.tier.as_deref().and_then(Tier::from_name) {
                t.apply(&mut p);
            }
            let mut g = Game::new(BigDish::with_map(p, &map), m.rules(Rules::default()), 1);
            let mut tr = Tracker::new(m, &g);
            let mut ai = Hand::named("claim", &g).unwrap();
            let mut walled = [!walls; 4];
            let tps = g.rules.tps as u64;
            while g.time() < secs && tr.verdict.is_none() && !done(&tr) {
                if walled.iter().any(|w| !w) && g.time() < 120.0 {
                    wall_rim_gaps(&mut g, &mut walled);
                } else {
                    ai.play(&mut g);
                }
                let s = g.world.ticks() / tps;
                while g.world.ticks() / tps == s && g.end.is_none() {
                    g.step();
                }
                let grid = g.world.mould_grid(BIN);
                tr.second(&g, grid.as_deref(), 1.0);
            }
            tr
        };
        let [landfall, walls, open] = std::thread::scope(|sc| {
            let play = &play;
            [
                sc.spawn(move || play("1-landfall", false, 200.0, |_| false)),
                sc.spawn(move || play("3-the-gaps", true, 140.0, |_| false)),
                sc.spawn(move || play("3-the-gaps", false, 140.0, |tr| tr.thick.is_some_and(|n| n > 0))),
            ]
            .map(|h| h.join().unwrap())
        });
        assert!(landfall.verdict.is_some_and(|v| v.cause == Cause::Goal), "Landfall, claim: {:?}", landfall.verdict);
        assert!(walls.verdict.is_none() && walls.thick == Some(0) && walls.bar.unwrap_or(0.0) >= 0.2, "The gaps with walls: thick {:?} bar {:?}", walls.thick, walls.bar);
        assert!(open.thick.is_some_and(|n| n > 0), "The gaps without walls kept thick mould in for 2:20: bar {:?}", open.bar);
    }
}
