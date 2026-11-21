//! **The slime viewer** (2026-11-20, the thread "Build a slime viewer to
//! tune looks"). The owner, 23:17Z, after a playtest: "I want to review
//! some of the changes in the slime looks. Can we have a thread build a
//! slime viewer so I can play around with the features/looks that we have
//! build and modify them myself?"
//!
//! `viewer.cmd` (or `play.cmd viewer=1`, or the main menu's Slime viewer
//! row) opens the game on a slime's own planet, as a run lands there, with
//! a panel on the right:
//!
//! - **Slime**: each of the seven slimes on its planet (a restart), the
//!   body look drawn on it (live), a new map, restart.
//! - **Spawn**: each of the slime's events now, by the code play uses
//!   (`larder_world::force`): a mini-boss of each kind, a fruit, ore or a
//!   boss core where you click; a wave, the last-stand heart, a raid,
//!   strikers and the heartbeat at once.
//! - **Look**: every plain number and colour of the view shader
//!   (`view.wgsl`'s constants, `tweaks::shader_consts`), grouped, each a
//!   slider; the shader is compiled again as they move (`View::reshade`).
//! - **Rules**: this slime's walk (its engine keys, live) and the numbers
//!   behind each event (`Rules::set`).
//!
//! **Save** writes `slime-tweaks.txt` at the repo's root
//! (`tweaks::Tweaks`), which the game reads at every round's start, so what
//! he tuned is what he plays (the corner's "experiments on" names it), and
//! a copy into `play-logs/`, which `play.cmd` sends to the `play-logs`
//! branch, so a thread can make his values the default. Right click on a
//! row puts that one back; Reset all puts back everything.
//!
//! The round never ends on time or a wiped squad here (the gun range's
//! clock and bank), and a wiped slime starts again.

use super::*;
use slime_mould::mould::larder_world::{BossKind, Rules};
use slime_mould::mould::run::{Run, LADDER};
use slime_mould::mould::tiers::Tier;
use slime_mould::world::canvas::{Align, Canvas, Rect, Weight};
use slime_mould::world::tweaks::{self, ShaderConst, Tweaks};
use slime_mould::world::ui::theme;
use slime_mould::world::view::SlimeLook;

/// The panel's tabs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Tab {
    Slime,
    Spawn,
    Look,
    Rules,
}

const TABS: [(Tab, &str); 4] = [(Tab::Slime, "Slime"), (Tab::Spawn, "Spawn"), (Tab::Look, "Look"), (Tab::Rules, "Rules")];

/// Something the Spawn tab starts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Spawn {
    Boss(BossKind),
    Fruit,
    Ore,
    Core,
    Wave,
    Stand,
    Raid,
    Strikers,
    Heartbeat,
}

impl Spawn {
    /// Placed where the map is clicked (else at once).
    fn at_click(self) -> bool {
        matches!(self, Spawn::Boss(_) | Spawn::Fruit | Spawn::Ore | Spawn::Core)
    }

    fn label(self) -> String {
        match self {
            Spawn::Boss(k) => format!("Mini-boss: {}", title(k.name())),
            Spawn::Fruit => "Fruiting knot".into(),
            Spawn::Ore => "Starfall ore".into(),
            Spawn::Core => "Boss core".into(),
            Spawn::Wave => "Wave on the squad".into(),
            Spawn::Stand => "Last-stand heart".into(),
            Spawn::Raid => "Raid".into(),
            Spawn::Strikers => "Condensed strikers".into(),
            Spawn::Heartbeat => "Heartbeat".into(),
        }
    }

    fn about(self) -> &'static str {
        match self {
            Spawn::Boss(BossKind::Hunter) => "Hunts the nearest soldier, lunging (violet).",
            Spawn::Boss(BossKind::Wanderer) => "Goes for a far fresh crystal, roots there and grows (bone).",
            Spawn::Boss(BossKind::Sapper) => "Eats through your nearest wall or building (ember).",
            Spawn::Boss(BossKind::Leech) => "Sits on a pump and takes its food (ink).",
            Spawn::Boss(BossKind::Mortar) => "Sits and lobs slime at soldiers and buildings in reach (moss and acid green).",
            Spawn::Boss(BossKind::Pinwheel) => "A hunter whose body turns in arms (cobalt and ice blue).",
            Spawn::Fruit => "The slime streams into a knot that ripens and bursts into spores, as when the squad hurts it.",
            Spawn::Ore => "Ore falls after the planet's warning: soldiers who reach it earn shop points.",
            Spawn::Core => "A broken boss's core: soldiers drain it for points, or the slime takes it back and the boss forms again.",
            Spawn::Wave => "The slime pulls back to points round the squad, then comes in at once.",
            Spawn::Stand => "What is left of the slime gathers into one armoured heart on its crystal that lunges at the squad.",
            Spawn::Raid => "A raid sets out next second for what play's raids aim at.",
            Spawn::Strikers => "Forces the slime's edge facing the squad to condense into gold strikers that charge (on until clicked again).",
            Spawn::Heartbeat => "The pulse that runs out from the slime's heart crystal (on or off).",
        }
    }
}

const SPAWNS: [Spawn; 14] = [
    Spawn::Boss(BossKind::Hunter),
    Spawn::Boss(BossKind::Wanderer),
    Spawn::Boss(BossKind::Sapper),
    Spawn::Boss(BossKind::Leech),
    Spawn::Boss(BossKind::Mortar),
    Spawn::Boss(BossKind::Pinwheel),
    Spawn::Fruit,
    Spawn::Ore,
    Spawn::Core,
    Spawn::Wave,
    Spawn::Stand,
    Spawn::Raid,
    Spawn::Strikers,
    Spawn::Heartbeat,
];

/// What a slider changes.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Target {
    /// A view shader constant, by its index in `Viewer::consts`.
    Look(usize),
    /// A game rule (`Rules::set`).
    Rule(&'static str),
    /// An engine key of this slime (`gpu::set`).
    Engine(&'static str),
}

/// A button at the panel's foot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Act {
    Save,
    ResetAll,
    Hide,
    Restart,
    NewMap,
    Pause,
    Fast,
}

/// What a place on the panel does when clicked.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Hit {
    Tab(Tab),
    /// The slime of the ladder's rung (0 the Drifter).
    Slime(usize),
    Body(SlimeLook),
    Spawn(Spawn),
    Group(String),
    /// A slider's bar: its target and which of its numbers.
    Bar(Target, usize),
    /// A switch, flipped by a click.
    Switch(Target),
    Act(Act),
    /// The panel itself (takes the click, does nothing).
    Panel,
}

/// The game rules the Rules tab shows, by group: (name, help, whole number).
const RULE_GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "Mini-bosses",
        &[
            ("boss_every", "Seconds between chances of a mini-boss (mean)."),
            ("boss_first", "Seconds from landing to the first chance."),
            ("boss_grade_lo", "Weakest grade drawn, 0 to 1 (armour, bite, pace)."),
            ("boss_grade_hi", "Strongest grade drawn, 0 to 1."),
            ("boss_reach", "How far round its centre a mini-boss takes walkers (dish cells)."),
            ("boss_haste", "Pace of a mini-boss on the move, times a walker's."),
            ("boss_lunge", "Seconds a hunter's lunge lasts."),
            ("boss_lunge_every", "Seconds between a hunter's lunges."),
            ("boss_spin", "How hard a pinwheel's arms turn."),
            ("boss_lob", "How far a mortar throws (dish cells)."),
            ("boss_lob_every", "Seconds between a mortar's throws."),
            ("boss_life", "Seconds a mini-boss lives."),
        ],
    ),
    (
        "Fruiting",
        &[
            ("fruit", "On or off."),
            ("fruit_hurt", "Share of its mass the squad must kill in the window to make it fruit."),
            ("fruit_ripen", "Seconds a fruit takes to ripen and burst."),
            ("fruit_size", "Share of the slime a fruit takes in."),
            ("fruit_swell", "How much a fruit swells as it ripens."),
            ("fruit_armour", "Blows a fruit's member takes (whole number)."),
        ],
    ),
    (
        "Waves",
        &[
            ("wave", "On or off."),
            ("wave_loss", "Share of its mass lost in the window that starts a wave."),
            ("wave_gather", "Seconds the slime gathers before it strikes."),
            ("wave_strike", "Seconds a strike lasts."),
            ("wave_every", "Seconds between waves."),
            ("wave_sides", "Points it gathers at round the squad (whole number)."),
        ],
    ),
    ("Heartbeat", &[("heart", "On or off."), ("heart_lean", "How hard a beat pushes the body out."), ("heart_period", "Seconds a beat."), ("heart_len", "A beat's wavelength (dish cells).")]),
    (
        "Condensed strikers",
        &[
            ("strikers", "On or off."),
            ("strikers_below", "The slime condenses while under this share of its peak mass."),
            ("strikers_haste", "A striker's pace as it charges, times its own."),
            ("strikers_band", "How far past the guns' reach the condensing edge runs."),
            ("strikers_most", "Walkers one striker takes in (1 to 15)."),
            ("strikers_few", "A squad this small meets no strikers."),
            ("strikers_full", "A squad this big meets them all."),
        ],
    ),
    (
        "Last-stand heart",
        &[
            ("stand", "On or off."),
            ("stand_at", "It forms under this share of the slime's peak mass."),
            ("stand_army", "Soldiers needed before it forms."),
            ("stand_every", "Seconds between its lunges."),
            ("stand_lunge", "Seconds a lunge lasts."),
            ("stand_haste", "Pace in a lunge."),
            ("stand_armour", "Blows a member takes (whole number)."),
        ],
    ),
    ("Starfall", &[("star", "On or off."), ("star_worth", "Shop points an ore holds."), ("star_warn", "Seconds of warning before ore lands."), ("star_life", "Seconds ore lasts once down."), ("core_worth", "A boss core's base worth.")]),
    ("Raids", &[("raid_every", "Mean seconds between raids (0 turns them off)."), ("raid_first", "Mean seconds to the first raid."), ("raid_size", "Share of the slime a raid takes.")]),
];

/// The rules that are switches (shown as on/off).
const SWITCHES: [&str; 6] = ["fruit", "wave", "heart", "strikers", "stand", "star"];
/// The rules that are whole numbers.
const WHOLE: [&str; 4] = ["fruit_armour", "wave_sides", "strikers_most", "stand_armour"];

/// This slime's engine keys the Rules tab shows: (key, help, range).
pub(super) const ENGINE_KEYS: &[(&str, &str, (f32, f32))] = &[
    ("sensor_angle", "Degrees between a walker's sensors: wide makes blobs and sheets, narrow long cords.", (5.0, 135.0)),
    ("turn", "Degrees a walker turns a step.", (5.0, 135.0)),
    ("reach", "How far ahead a walker smells (cells).", (1.0, 40.0)),
    ("step", "How far a walker moves a step (cells).", (0.2, 4.0)),
    ("keep", "Share of the trail kept each step: higher, longer-lasting trails.", (0.5, 0.999)),
    ("laid", "Trail a walker lays a step.", (0.0, 20.0)),
    ("explore", "Steps a newborn walker walks away from trail first (the Sprawler's).", (0.0, 1000.0)),
    ("blocked_lay", "Trail a packed walker lays where it stands (the Drifter's droplets).", (0.0, 5.0)),
    ("hunger_scent", "How hard a hungry walker follows food scent.", (0.0, 40.0)),
    ("energy_flow", "How fast the body shares its food.", (0.0, 0.2)),
    ("wither", "How fast a part cut off from food thins.", (0.0, 0.05)),
    ("alarm_weight", "How a walker reads a strike's alarm: below 0 flees it, above 0 swarms to it.", (-5.0, 10.0)),
    ("alarm_keep", "Share of the alarm kept each step.", (0.8, 1.0)),
    ("flinch", "How hard the body flinches from gunfire.", (0.0, 4.0)),
    ("spore_rate", "Chance a fed walker fruits into a spore each step (the Sporer's).", (0.0, 0.0001)),
    ("spore_burst", "Share of the struck fed walkers that fruit at once (the Sporer's).", (0.0, 0.3)),
    ("sleep_rate", "Chance a walker falls dormant each step (the Sleeper's crust).", (0.0, 0.05)),
    ("sleep_deposit", "Trail a dormant walker lays.", (0.0, 2.0)),
    ("scar_weight", "How a walker reads old scars: the Grudge comes back to where it was shot.", (-2.0, 2.0)),
    ("germ_resist", "How well it resists the germ.", (0.0, 30.0)),
];

/// The Look tab's groups: (heading, the names in it, open at first). A
/// name is in the first group whose list holds it or a prefix of it (a
/// name ending `_`); the rest are "Everything else".
const LOOK_GROUPS: &[(&str, &[&str], bool)] = &[
    ("The slime's body", &["MOULD_HUE", "BODY_", "SHARP_", "PLAIN_AMP", "FILM_", "SLIME_ON_"], true),
    ("Each slime's body look", &["SHEET_VEIL", "THREAD_VEIL", "CORD_", "SWELL_"], true),
    ("Mini-bosses, fruit and heart", &["LUMP_", "FRUIT_RISE"], true),
    ("Alarm flush, bites and minds", &["FLUSH_", "BITE_SLIME", "BITE_HOT", "MIND_"], false),
    ("Trunk's amber walls", &["SET_"], false),
    ("Flung drops and strikers", &["DROP_"], false),
    ("Trails on bare ground", &["DUST", "SLICK_"], false),
    ("Crystals", &["GEM_"], false),
];
const OTHER: &str = "Everything else (ground, soldiers, marks)";

/// A row of the panel.
enum Row {
    Head { text: String, group: Option<String>, open: bool },
    Choice { label: String, note: String, hit: Hit, on: bool, about: String },
    Slider { label: String, about: String, target: Target, vals: Vec<f32>, ranges: Vec<(f32, f32)>, colour: bool, changed: bool },
    Switch { label: String, about: String, target: Target, on: bool, changed: bool },
    Text(String),
}

/// The viewer's state.
pub(super) struct Viewer {
    pub open: bool,
    tab: Tab,
    scroll: [f32; 4],
    /// The view shader as built, its constants, and their values now.
    base: String,
    consts: Vec<ShaderConst>,
    look: Vec<Vec<f32>>,
    /// Look groups the owner opened or closed (by heading), against their
    /// first state.
    flipped: Vec<String>,
    /// This frame's click places, the cursor, the bar being dragged.
    hits: Vec<(Rect, Hit)>,
    panel: Rect,
    cursor: Option<(f32, f32)>,
    drag: Option<(Target, usize, Rect, (f32, f32))>,
    /// What the next click on the map places.
    pub armed: Option<Spawn>,
    /// The look's numbers moved since the shader was last compiled, and
    /// when it was.
    dirty: bool,
    shaded: Instant,
    /// Changes not saved yet; the line at the foot, and its seconds left.
    pub unsaved: bool,
    status: String,
    status_secs: f32,
    /// The rules and this slime's engine keys as the round started, under
    /// the tweaks (what Reset puts back), and the sliders' ranges.
    pub rules0: Option<Rules>,
    pub engine0: Vec<(&'static str, f32)>,
    ranges: Vec<(String, (f32, f32))>,
    /// The strikers' rules while they are forced on.
    strikers_was: Option<slime_mould::mould::larder_world::StrikerRules>,
    /// `LARDER_VIEWER_SPAWN` (films and checks, no window to click):
    /// spawns at game seconds, `T:NAME[@X,Y];..` (world units), each
    /// started once.
    staged: Vec<Staged>,
}

/// A staged spawn: its game second, what, and where (world units; None
/// for the slime's middle or the squad).
type Staged = (f32, Spawn, Option<(f32, f32)>);

/// A spawn by the name `LARDER_VIEWER_SPAWN` gives it.
fn spawn_named(n: &str) -> Option<Spawn> {
    if let Some(k) = BossKind::ALL.iter().find(|k| k.name() == n) {
        return Some(Spawn::Boss(*k));
    }
    Some(match n {
        "fruit" => Spawn::Fruit,
        "ore" => Spawn::Ore,
        "core" => Spawn::Core,
        "wave" => Spawn::Wave,
        "stand" => Spawn::Stand,
        "raid" => Spawn::Raid,
        "strikers" => Spawn::Strikers,
        "heartbeat" => Spawn::Heartbeat,
        _ => return None,
    })
}

/// `T:NAME[@X,Y];..`, the entries it cannot read skipped and named.
fn staged_of(text: &str) -> Vec<Staged> {
    let mut out = Vec::new();
    for e in text.split(';').map(str::trim).filter(|e| !e.is_empty()) {
        let read = (|| {
            let (t, rest) = e.split_once(':')?;
            let (name, at) = match rest.split_once('@') {
                Some((n, xy)) => {
                    let (x, y) = xy.split_once(',')?;
                    (n, Some((x.trim().parse().ok()?, y.trim().parse().ok()?)))
                }
                None => (rest, None),
            };
            Some((t.trim().parse().ok()?, spawn_named(name.trim())?, at))
        })();
        match read {
            Some(x) => out.push(x),
            None => eprintln!("larder-world: LARDER_VIEWER_SPAWN: skipped {e} (T:NAME[@X,Y])"),
        }
    }
    out
}

impl Viewer {
    pub(super) fn new(tweaks: &Tweaks) -> Viewer {
        let base = std::env::var("LARDER_VIEW_WGSL").ok().and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_else(|| slime_mould::world::view::SHADER.to_string());
        // Worked-out values past 10,000 (a flag's 1e9) are no look number.
        let consts: Vec<ShaderConst> = tweaks::shader_consts(&base).into_iter().filter(|c| c.value.iter().all(|v| v.abs() <= 10_000.0)).collect();
        let look = consts.iter().map(|c| tweaks.look_of(&c.name).filter(|v| v.len() == c.value.len()).map_or_else(|| c.value.clone(), <[f32]>::to_vec)).collect();
        // `LARDER_VIEWER_TAB=look` opens on that tab (the films' pictures).
        let tab = std::env::var("LARDER_VIEWER_TAB").ok().and_then(|t| TABS.iter().find(|(_, n)| n.eq_ignore_ascii_case(&t)).map(|(t, _)| *t)).unwrap_or(Tab::Slime);
        Viewer {
            open: true,
            tab,
            scroll: [0.0; 4],
            base,
            consts,
            look,
            flipped: Vec::new(),
            hits: Vec::new(),
            panel: Rect::new(0.0, 0.0, 0.0, 0.0),
            cursor: None,
            drag: None,
            armed: None,
            dirty: false,
            shaded: Instant::now(),
            unsaved: false,
            status: String::new(),
            status_secs: 0.0,
            rules0: None,
            engine0: Vec::new(),
            ranges: Vec::new(),
            strikers_was: None,
            staged: std::env::var("LARDER_VIEWER_SPAWN").map(|t| staged_of(&t)).unwrap_or_default(),
        }
    }

    pub(super) fn say(&mut self, text: impl Into<String>) {
        self.status = text.into();
        self.status_secs = 6.0;
    }

    /// The panel holds screen point `p` (while it is open).
    pub(super) fn over(&self, p: (f32, f32)) -> bool {
        self.open && self.panel.contains(p)
    }

    /// The shader source with the look's values now (unchanged ones left
    /// as they are).
    pub(super) fn source(&self) -> String {
        tweaks::apply(&self.base, &self.changed_looks())
    }

    fn changed_looks(&self) -> Vec<(String, Vec<f32>)> {
        self.consts.iter().zip(&self.look).filter(|(c, v)| c.value != **v).map(|(c, v)| (c.name.clone(), v.clone())).collect()
    }

    fn group_of(name: &str) -> usize {
        LOOK_GROUPS.iter().position(|(_, names, _)| names.iter().any(|n| if n.ends_with('_') { name.starts_with(n) } else { name == *n || (name.starts_with(n) && !n.contains('_')) })).unwrap_or(LOOK_GROUPS.len())
    }

    fn group_open(&self, head: &str, first: bool) -> bool {
        first != self.flipped.iter().any(|f| f == head)
    }

    /// The slider range for `key` starting from `v` (kept once made, so it
    /// does not move under a drag).
    fn range(&mut self, key: &str, v: f32, given: Option<(f32, f32)>) -> (f32, f32) {
        if let Some((_, r)) = self.ranges.iter().find(|(k, _)| k == key) {
            return *r;
        }
        let r = given.unwrap_or_else(|| default_range(v));
        let r = (r.0.min(v), r.1.max(v));
        self.ranges.push((key.to_string(), r));
        r
    }
}

/// A number's slider range: 0 to twice it (at least 1), or twice it to 0
/// below 0.
fn default_range(v: f32) -> (f32, f32) {
    if v < 0.0 {
        ((2.0 * v).min(-1.0), 0.0)
    } else {
        (0.0, (2.0 * v).max(1.0))
    }
}

fn title(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().collect::<String>() + c.as_str())
}

/// A value as the panel shows it: three significant figures.
fn show(v: f32) -> String {
    let a = v.abs();
    if a == 0.0 {
        "0".into()
    } else if a >= 100.0 {
        format!("{v:.0}")
    } else if a >= 10.0 {
        format!("{v:.1}")
    } else if a >= 1.0 {
        format!("{v:.2}")
    } else if a >= 0.001 {
        format!("{v:.3}")
    } else {
        format!("{v:.1e}")
    }
}

/// This slime's engine key `key` as the world holds it now.
pub(super) fn engine_get(s: &slime_mould::gpu::Settings, key: &str) -> Option<f32> {
    Some(match key {
        "sensor_angle" => s.sensor_angle,
        "turn" => s.turn,
        "reach" => s.reach,
        "step" => s.step,
        "keep" => s.keep,
        "laid" => s.laid,
        "explore" => s.explore as f32,
        "blocked_lay" => s.blocked_lay,
        "hunger_scent" => s.hunger_scent,
        "energy_flow" => s.energy_flow,
        "wither" => s.wither,
        "alarm_weight" => s.alarm_weight,
        "alarm_keep" => s.alarm_keep,
        "flinch" => s.flinch,
        "spore_rate" => s.spore_rate,
        "spore_burst" => s.spore_burst,
        "sleep_rate" => s.sleep_rate,
        "sleep_deposit" => s.sleep_deposit,
        "scar_weight" => s.scar_weight,
        "germ_resist" => s.germ_resist,
        _ => return None,
    })
}

/// The slime of a viewer run's planet.
pub(super) fn tier_of(run: &Run) -> Tier {
    run.level().tier
}

impl App {
    /// The viewer's slime now.
    fn viewer_tier(&self) -> Option<Tier> {
        self.args.run.as_ref().map(tier_of)
    }

    /// Once a frame: the shader compiled again when the look moved (at
    /// most every 0.3 s while a bar is dragged), the foot's line aged, a
    /// wiped slime started again.
    pub(super) fn viewer_frame(&mut self, dt: f32) {
        let Some(vw) = self.viewer.as_mut() else { return };
        vw.status_secs = (vw.status_secs - dt).max(0.0);
        if vw.dirty && vw.shaded.elapsed().as_secs_f32() > 0.3 {
            vw.dirty = false;
            vw.shaded = Instant::now();
            let src = vw.source();
            if let Some(v) = self.view.as_mut() {
                if let Err(e) = v.reshade(&src) {
                    let line = e.lines().find(|l| !l.trim().is_empty()).unwrap_or("").to_string();
                    if let Some(vw) = self.viewer.as_mut() {
                        vw.say(format!("The look did not compile: {line}"));
                    }
                }
            }
        }
        self.viewer_staged();
        if self.game.as_ref().is_some_and(|g| g.end.is_some()) {
            if let Err(e) = self.viewer_restart(None, false) {
                eprintln!("larder-world: viewer restart: {e}");
            }
            if let Some(vw) = self.viewer.as_mut() {
                vw.say("The round ended, so a fresh one started");
            }
        }
    }

    /// The staged spawns whose time has come (`LARDER_VIEWER_SPAWN`), once
    /// a step in a film and once a frame in the window.
    pub(super) fn viewer_staged(&mut self) {
        let Some(t) = self.game.as_ref().map(|g| g.time()) else { return };
        let Some(vw) = self.viewer.as_mut() else { return };
        if vw.staged.is_empty() {
            return;
        }
        let due: Vec<_> = vw.staged.iter().filter(|e| e.0 <= t).copied().collect();
        vw.staged.retain(|e| e.0 > t);
        for (_, sp, at) in due {
            self.viewer_spawn(sp, at);
            let line = self.viewer.as_ref().map_or(String::new(), |v| v.status.clone());
            println!("  viewer t {t:.1} s: {line}");
        }
    }

    /// The round again: on the ladder's rung `rung` (its planet), on a new
    /// map, or as it was.
    pub(super) fn viewer_restart(&mut self, rung: Option<usize>, new_map: bool) -> Result<(), String> {
        let Some(r) = self.args.run.as_mut() else { return Ok(()) };
        if new_map {
            let depth = r.depth;
            *r = Run::new(fresh_seed());
            r.depth = depth;
            r.dev = true;
            r.restock();
        }
        if let Some(k) = rung {
            r.depth = (k as u32 + 1).clamp(1, slime_mould::mould::run::PLANETS);
        }
        let l = r.level();
        self.args.seed = l.seed;
        self.args.walkers = run_walkers(l.walkers);
        if let Some(vw) = self.viewer.as_mut() {
            vw.armed = None;
            vw.strikers_was = None;
        }
        self.start()
    }

    /// The view's shader as the tweaks file has it (or the game's own),
    /// whatever it was (out of the viewer).
    pub(super) fn reset_look(&mut self) {
        let src = tweaks::apply(slime_mould::world::view::SHADER, &self.tweaks.look);
        if let Some(v) = self.view.as_mut() {
            if let Err(e) = v.reshade(&src) {
                eprintln!("larder-world: the look tweaks did not compile: {e}");
                let _ = v.reshade(slime_mould::world::view::SHADER);
            }
        }
    }

    /// The window's look tweaks into the view (at its first round).
    pub(super) fn apply_look_tweaks(&mut self) {
        let src = match &self.viewer {
            Some(vw) => vw.source(),
            None if !self.tweaks.look.is_empty() => tweaks::apply(slime_mould::world::view::SHADER, &self.tweaks.look),
            None => return,
        };
        if src == slime_mould::world::view::SHADER {
            return;
        }
        if let Some(v) = self.view.as_mut() {
            if let Err(e) = v.reshade(&src) {
                eprintln!("larder-world: the look tweaks in {} did not compile, so the game's own look plays: {e}", tweaks::path().display());
            }
        }
    }

    /// A click on the panel or, with a spawn armed, on the map. True if
    /// the viewer took it.
    pub(super) fn viewer_button(&mut self, b: Button, down: bool) -> bool {
        let Some(vw) = self.viewer.as_mut() else { return false };
        let at = vw.cursor;
        if !down {
            if vw.drag.take().is_some() {
                vw.dirty = true;
                return true;
            }
            return at.is_some_and(|p| vw.over(p));
        }
        let Some(p) = at else { return false };
        if vw.over(p) {
            let hit = vw.hits.iter().rev().find(|(r, _)| r.contains(p)).map(|(r, h)| (*r, h.clone()));
            match (b, hit) {
                (Button::Right, Some((_, Hit::Bar(t, _) | Hit::Switch(t)))) => self.viewer_reset(&t),
                (Button::Left, Some((r, hit))) => self.viewer_click(hit, r, p),
                _ => {}
            }
            return true;
        }
        // An armed spawn takes the next click on the map; right click
        // puts it down.
        if let Some(sp) = vw.armed {
            if b == Button::Right {
                vw.armed = None;
                vw.say("Put down");
                return true;
            }
            if b == Button::Left && !self.hud.over(p) {
                if let Some(w) = self.camera.world_at(p) {
                    self.viewer_spawn(sp, Some(w));
                }
                return true;
            }
        }
        false
    }

    /// The cursor moved: a dragged bar follows it. True over the panel.
    pub(super) fn viewer_moved(&mut self, at: Option<(f32, f32)>) -> bool {
        let Some(vw) = self.viewer.as_mut() else { return false };
        vw.cursor = at;
        if let (Some((t, k, r, range)), Some(p)) = (vw.drag.clone(), at) {
            let v = range.0 + (range.1 - range.0) * ((p.0 - r.x) / r.w.max(1.0)).clamp(0.0, 1.0);
            self.viewer_set(&t, k, v);
            return true;
        }
        at.is_some_and(|p| self.viewer.as_ref().is_some_and(|vw| vw.over(p)))
    }

    /// The wheel over the panel scrolls it.
    pub(super) fn viewer_wheel(&mut self, notches: i32) -> bool {
        let Some(vw) = self.viewer.as_mut() else { return false };
        if !vw.cursor.is_some_and(|p| vw.over(p)) {
            return false;
        }
        let t = TABS.iter().position(|(t, _)| *t == vw.tab).unwrap_or(0);
        vw.scroll[t] = (vw.scroll[t] - notches as f32 * 90.0).max(0.0);
        true
    }

    /// F1 shows and hides the panel; Esc puts down an armed spawn. True if
    /// the viewer took the key.
    pub(super) fn viewer_key(&mut self, code: KeyCode) -> bool {
        let Some(vw) = self.viewer.as_mut() else { return false };
        match code {
            KeyCode::F1 => {
                vw.open = !vw.open;
                true
            }
            KeyCode::Escape if vw.armed.is_some() => {
                vw.armed = None;
                vw.say("Put down");
                true
            }
            _ => false,
        }
    }

    fn viewer_click(&mut self, hit: Hit, r: Rect, p: (f32, f32)) {
        match hit {
            Hit::Tab(t) => {
                if let Some(vw) = self.viewer.as_mut() {
                    vw.tab = t;
                }
            }
            Hit::Slime(k) => {
                if let Err(e) = self.viewer_restart(Some(k), false) {
                    eprintln!("larder-world: viewer: {e}");
                }
                let name = LADDER.get(k).map_or("", |t| t.name());
                if let Some(vw) = self.viewer.as_mut() {
                    vw.say(format!("The {} on planet {}", title(name), k + 1));
                }
            }
            Hit::Body(look) => self.viewer_body(look),
            Hit::Spawn(sp) if sp.at_click() => {
                if let Some(vw) = self.viewer.as_mut() {
                    vw.armed = Some(sp);
                    vw.say(format!("Click the map to place: {} (right click puts it down)", sp.label()));
                }
            }
            Hit::Spawn(sp) => self.viewer_spawn(sp, None),
            Hit::Group(g) => {
                if let Some(vw) = self.viewer.as_mut() {
                    match vw.flipped.iter().position(|f| *f == g) {
                        Some(i) => {
                            vw.flipped.remove(i);
                        }
                        None => vw.flipped.push(g),
                    }
                }
            }
            Hit::Bar(_, usize::MAX) => {}
            Hit::Bar(t, k) => {
                let range = self.viewer_range(&t, k);
                let v = range.0 + (range.1 - range.0) * ((p.0 - r.x) / r.w.max(1.0)).clamp(0.0, 1.0);
                self.viewer_set(&t, k, v);
                if let Some(vw) = self.viewer.as_mut() {
                    vw.drag = Some((t, k, r, range));
                }
            }
            Hit::Switch(t) => {
                let on = self.viewer_value(&t).first().is_some_and(|&v| v > 0.0);
                self.viewer_set(&t, 0, if on { 0.0 } else { 1.0 });
            }
            Hit::Act(a) => self.viewer_act(a),
            Hit::Panel => {}
        }
    }

    fn viewer_act(&mut self, a: Act) {
        match a {
            Act::Save => self.viewer_save(),
            Act::ResetAll => self.viewer_reset_all(),
            Act::Hide => {
                if let Some(vw) = self.viewer.as_mut() {
                    vw.open = false;
                    vw.say("F1 shows the panel again");
                }
                self.hud.alert("Slime viewer hidden: F1 shows it again", None, theme::ACCENT);
            }
            Act::Restart => {
                if let Err(e) = self.viewer_restart(None, false) {
                    eprintln!("larder-world: viewer: {e}");
                }
            }
            Act::NewMap => {
                if let Err(e) = self.viewer_restart(None, true) {
                    eprintln!("larder-world: viewer: {e}");
                }
            }
            Act::Pause => self.paused = !self.paused,
            Act::Fast => self.fast = !self.fast,
        }
    }

    /// The body look drawn on the slime now, kept for this slime in the
    /// tweaks.
    fn viewer_body(&mut self, look: SlimeLook) {
        let Some(g) = self.game.as_ref() else { return };
        let fine_tps = g.rules.tps as f32 * g.world.fine_steps() as f32;
        let pace = g.world.s.step / g.world.scale() * fine_tps;
        if let Some(v) = self.view.as_mut() {
            v.set_slime_look(look, pace * slime_mould::world::view::STREAK_SECS);
        }
        let tier = self.viewer_tier();
        if let Some(t) = tier {
            if SlimeLook::of(Some(t)) == look {
                self.tweaks.body.retain(|(n, _)| n != t.name());
            } else {
                self.tweaks.set_body(t.name(), look.name());
            }
        }
        if let Some(vw) = self.viewer.as_mut() {
            vw.unsaved = true;
        }
    }

    fn viewer_spawn(&mut self, sp: Spawn, at: Option<(f32, f32)>) {
        let Some(g) = self.game.as_mut() else { return };
        if g.in_flight() {
            g.finish_step();
        }
        let Some(g) = self.game.as_mut() else { return };
        let done = match (sp, at) {
            (Spawn::Boss(k), Some(p)) => g.force_boss(k, p),
            (Spawn::Fruit, Some(p)) => g.force_fruit(p),
            (Spawn::Ore, Some(p)) => g.force_ore(p),
            (Spawn::Core, Some(p)) => g.force_core(p),
            (Spawn::Wave, _) => g.force_wave(),
            (Spawn::Stand, _) => g.force_stand(),
            (Spawn::Raid, _) => g.force_raid(),
            (Spawn::Strikers, _) => {
                let vw = self.viewer.as_mut();
                match vw.and_then(|vw| vw.strikers_was.take()) {
                    Some(was) => {
                        g.rules.strikers = was;
                        if let Some(vw) = self.viewer.as_mut() {
                            vw.say("Strikers back to the planet's rules");
                        }
                        return;
                    }
                    None => {
                        let was = g.rules.strikers;
                        g.rules.strikers = slime_mould::mould::larder_world::StrikerRules { on: true, below: 2.0, few: -1.0, full: 0.0, ..was };
                        if let Some(vw) = self.viewer.as_mut() {
                            vw.strikers_was = Some(was);
                        }
                        true
                    }
                }
            }
            (Spawn::Heartbeat, _) => {
                g.rules.heart.on = !g.rules.heart.on;
                let on = g.rules.heart.on;
                if let Some(vw) = self.viewer.as_mut() {
                    vw.say(if on { "Heartbeat on" } else { "Heartbeat off" });
                }
                return;
            }
            _ => false,
        };
        let why = match sp {
            Spawn::Boss(_) | Spawn::Fruit => "too few walkers there, or every lump is in use (each event holds one; wait for one to end)",
            Spawn::Ore | Spawn::Core => "not on open ground",
            Spawn::Wave => "one is under way, or too little slime round the squad to gather: bring the squad near the slime and try again",
            Spawn::Stand => "a heart lives, or no slime holds a crystal",
            Spawn::Raid => "raids are off on this planet",
            _ => "",
        };
        let line = if done { format!("{}: started", sp.label()) } else { format!("{}: not now ({why})", sp.label()) };
        if let Some(vw) = self.viewer.as_mut() {
            vw.say(line);
            // A placed spawn stays armed, so several go down in a row.
        }
    }

    /// A target's numbers now.
    fn viewer_value(&self, t: &Target) -> Vec<f32> {
        match t {
            Target::Look(i) => self.viewer.as_ref().and_then(|vw| vw.look.get(*i).cloned()).unwrap_or_default(),
            Target::Rule(n) => self.game.as_ref().and_then(|g| g.rules.get(n)).into_iter().collect(),
            Target::Engine(k) => self.game.as_ref().and_then(|g| engine_get(&g.world.s, k)).into_iter().collect(),
        }
    }

    /// A target's number `k` from the round's start (Reset's value).
    fn viewer_first(&self, t: &Target) -> Vec<f32> {
        let Some(vw) = self.viewer.as_ref() else { return Vec::new() };
        match t {
            Target::Look(i) => vw.consts.get(*i).map(|c| c.value.clone()).unwrap_or_default(),
            Target::Rule(n) => vw.rules0.as_ref().and_then(|r| r.get(n)).into_iter().collect(),
            Target::Engine(k) => vw.engine0.iter().find(|(n, _)| n == k).map(|e| e.1).into_iter().collect(),
        }
    }

    fn viewer_range(&mut self, t: &Target, k: usize) -> (f32, f32) {
        let first = self.viewer_first(t);
        let v0 = first.get(k).copied().unwrap_or(0.0);
        let Some(vw) = self.viewer.as_mut() else { return (0.0, 1.0) };
        match t {
            Target::Look(i) => {
                let colour = vw.consts.get(*i).is_some_and(|c| c.colour);
                let key = format!("look:{i}:{k}");
                vw.range(&key, v0, colour.then_some((0.0, 255.0)))
            }
            Target::Rule(n) => vw.range(&format!("rule:{n}"), v0, None),
            Target::Engine(n) => {
                let given = ENGINE_KEYS.iter().find(|e| e.0 == *n).map(|e| e.2);
                vw.range(&format!("engine:{n}"), v0, given)
            }
        }
    }

    /// Sets number `k` of a target to `v`: live in the game, and in the
    /// tweaks (dropped there when it is back at its first value).
    fn viewer_set(&mut self, t: &Target, k: usize, v: f32) {
        let first = self.viewer_first(t);
        let tier = self.viewer_tier();
        match t {
            Target::Look(i) => {
                let Some(vw) = self.viewer.as_mut() else { return };
                let Some(vals) = vw.look.get_mut(*i) else { return };
                if let Some(x) = vals.get_mut(k) {
                    *x = if vw.consts[*i].colour { v.round() } else { v };
                }
                let (name, now) = (vw.consts[*i].name.clone(), vals.clone());
                vw.dirty = true;
                vw.unsaved = true;
                if now == first {
                    self.tweaks.look.retain(|(n, _)| *n != name);
                } else {
                    self.tweaks.set_look(&name, &now);
                }
            }
            Target::Rule(n) => {
                let v = if SWITCHES.contains(n) || WHOLE.contains(n) { v.round() } else { v };
                let Some(g) = self.game.as_mut() else { return };
                if !g.rules.set(n, v) {
                    return;
                }
                let now = g.rules.get(n).unwrap_or(v);
                if first.first() == Some(&now) {
                    self.tweaks.rules.retain(|(r, _)| r != n);
                } else {
                    self.tweaks.set_rule(n, now);
                }
                if let Some(vw) = self.viewer.as_mut() {
                    vw.unsaved = true;
                }
            }
            Target::Engine(key) => {
                let v = if *key == "explore" { v.round() } else { v };
                let Some(g) = self.game.as_mut() else { return };
                if slime_mould::gpu::set(&mut g.world.s, key, &format!("{v}")).is_err() {
                    return;
                }
                let now = engine_get(&g.world.s, key).unwrap_or(v);
                if let Some(t) = tier {
                    if first.first() == Some(&now) {
                        self.tweaks.slime.retain(|(n, k, _)| !(n == t.name() && k == key));
                    } else {
                        self.tweaks.set_slime(t.name(), key, now);
                    }
                }
                if let Some(vw) = self.viewer.as_mut() {
                    vw.unsaved = true;
                }
            }
        }
    }

    /// A target back at its first value (right click).
    fn viewer_reset(&mut self, t: &Target) {
        let first = self.viewer_first(t);
        for (k, v) in first.into_iter().enumerate() {
            self.viewer_set(t, k, v);
        }
    }

    fn viewer_reset_all(&mut self) {
        let Some(vw) = self.viewer.as_mut() else { return };
        for (c, v) in vw.consts.iter().zip(vw.look.iter_mut()) {
            *v = c.value.clone();
        }
        vw.dirty = true;
        vw.unsaved = true;
        let (rules0, engine0) = (vw.rules0, vw.engine0.clone());
        let strikers = vw.strikers_was.take();
        if let Some(g) = self.game.as_mut() {
            if let Some(r) = rules0 {
                g.rules = r;
            }
            if let Some(s) = strikers {
                g.rules.strikers = s;
            }
            for (k, v) in engine0 {
                let _ = slime_mould::gpu::set(&mut g.world.s, k, &format!("{v}"));
            }
        }
        self.tweaks = Tweaks::default();
        let look = SlimeLook::of(self.viewer_tier());
        self.viewer_body(look);
        if let Some(vw) = self.viewer.as_mut() {
            vw.say("Everything is back to the game's own; Save to keep it so");
        }
    }

    /// Writes the tweaks file, and a copy to the play logs.
    fn viewer_save(&mut self) {
        let line = match self.tweaks.save() {
            Ok(p) => {
                let copy = std::path::Path::new(slime_mould::mould::playlog::DIR);
                let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
                let sent = std::fs::create_dir_all(copy).and_then(|_| std::fs::write(copy.join(format!("slime-tweaks-{stamp}.txt")), self.tweaks.text())).is_ok();
                if self.tweaks.is_empty() {
                    format!("Saved: no tweaks, so {} is gone and the game plays its own", p.display())
                } else {
                    format!("Saved {} values to {}{}", self.tweaks.count(), p.display(), if sent { "; a copy goes with your play logs" } else { "" })
                }
            }
            Err(e) => format!("Could not save: {e}"),
        };
        if let Some(vw) = self.viewer.as_mut() {
            vw.unsaved = false;
            vw.say(line);
        }
    }

    /// The panel, drawn over the HUD on `c` (its rows built from the game
    /// as it is this frame).
    pub(super) fn viewer_draw(&mut self, c: &mut Canvas) {
        let tier = self.viewer_tier();
        let rows = self.viewer_rows(tier);
        let (top, bottom) = (self.hud.top, self.hud.bottom);
        let tweaks = self.tweaks.count();
        let Some(vw) = self.viewer.as_mut() else { return };
        vw.hits.clear();
        if !vw.open {
            vw.panel = Rect::new(0.0, 0.0, 0.0, 0.0);
            return;
        }
        let u = c.ui.px(c.screen);
        let (sw, sh) = (c.screen.0 as f32, c.screen.1 as f32);
        let w = (380.0 * u).min(sw * 0.45);
        let pad = 10.0 * u;
        let x = sw - w - pad;
        let y = top + pad;
        // `Hud::bottom` is where the bottom bar's top edge is.
        let h = (bottom.min(sh) - pad - y).max(200.0 * u);
        let panel = Rect::new(x, y, w, h);
        vw.panel = panel;
        c.shadow(panel, 10.0 * u, 14.0 * u, 4.0 * u, theme::SHADOW);
        c.panel(panel, 10.0 * u, theme::PANEL_TOP, theme::PANEL_BOTTOM, 1.0, theme::EDGE);
        vw.hits.push((panel, Hit::Panel));
        let inner = x + 12.0 * u;
        let iw = w - 24.0 * u;
        // Title.
        let mut yy = y + 24.0 * u;
        c.text(Weight::SemiBold, 17.0 * u, (inner, yy), Align::Left, "Slime viewer", theme::TEXT);
        let what = tier.map_or(String::new(), |t| format!("{} on planet {}", title(t.name()), LADDER.iter().position(|&l| l == t).map_or(0, |k| k + 1)));
        c.text_fit(Weight::Medium, 12.0 * u, (x + w - 12.0 * u, yy), Align::Right, &what, iw * 0.55, theme::MUTED);
        // Tabs.
        yy += 12.0 * u;
        let tw = iw / TABS.len() as f32;
        for (k, (t, name)) in TABS.iter().enumerate() {
            let r = Rect::new(inner + k as f32 * tw, yy, tw - 4.0 * u, 26.0 * u);
            let on = *t == vw.tab;
            let hover = vw.cursor.is_some_and(|p| r.contains(p));
            let (a, b) = if on { ([60, 96, 132, 255], [40, 66, 92, 255]) } else if hover { (theme::BUTTON_HOVER_TOP, theme::BUTTON_HOVER_BOTTOM) } else { (theme::BUTTON_TOP, theme::BUTTON_BOTTOM) };
            c.panel(r, 6.0 * u, a, b, 1.0, theme::EDGE);
            c.text(Weight::SemiBold, 13.0 * u, (r.x + r.w / 2.0, r.y + 17.5 * u), Align::Centre, name, if on { theme::TEXT } else { theme::MUTED });
            vw.hits.push((r, Hit::Tab(*t)));
        }
        yy += 34.0 * u;
        // The foot: help, the status line and the buttons.
        let foot_h = 104.0 * u;
        let list = Rect::new(inner, yy, iw, (y + h - foot_h - yy).max(40.0 * u));
        let tab_i = TABS.iter().position(|(t, _)| *t == vw.tab).unwrap_or(0);
        // Rows, scrolled; only whole rows inside the list are drawn.
        let row_h = |r: &Row| match r {
            Row::Head { .. } => 30.0 * u,
            Row::Text(_) => 20.0 * u,
            _ => 27.0 * u,
        };
        let total: f32 = rows.iter().map(row_h).sum();
        let most = (total - list.h).max(0.0);
        vw.scroll[tab_i] = vw.scroll[tab_i].min(most);
        let mut ry = list.y - vw.scroll[tab_i];
        let mut help = String::new();
        for row in &rows {
            let rh = row_h(row);
            let r = Rect::new(list.x, ry, list.w, rh);
            ry += rh;
            if r.y < list.y - 0.5 || r.bottom() > list.bottom() + 0.5 {
                continue;
            }
            let hover = vw.cursor.is_some_and(|p| r.contains(p));
            match row {
                Row::Head { text, group, open } => {
                    let mark = match group {
                        Some(_) if *open => "- ",
                        Some(_) => "+ ",
                        None => "",
                    };
                    c.text_fit(Weight::SemiBold, 13.0 * u, (r.x, r.y + 21.0 * u), Align::Left, &format!("{mark}{text}"), r.w, if hover && group.is_some() { theme::TEXT } else { theme::ACCENT });
                    if let Some(g) = group {
                        vw.hits.push((r, Hit::Group(g.clone())));
                    }
                }
                Row::Text(t) => {
                    c.text_fit(Weight::Medium, 11.5 * u, (r.x, r.y + 14.0 * u), Align::Left, t, r.w, theme::FAINT);
                }
                Row::Choice { label, note, hit, on, about } => {
                    let b = Rect::new(r.x, r.y + 1.5 * u, r.w, rh - 3.0 * u);
                    let (a, bb) = if *on { ([52, 84, 116, 255], [36, 60, 84, 255]) } else if hover { (theme::BUTTON_HOVER_TOP, theme::BUTTON_HOVER_BOTTOM) } else { (theme::BUTTON_TOP, theme::BUTTON_BOTTOM) };
                    c.panel(b, 5.0 * u, a, bb, 1.0, theme::EDGE);
                    c.text_fit(Weight::SemiBold, 12.5 * u, (b.x + 8.0 * u, b.y + 16.5 * u), Align::Left, label, b.w * 0.62, theme::TEXT);
                    c.text_fit(Weight::Medium, 11.5 * u, (b.right() - 8.0 * u, b.y + 16.5 * u), Align::Right, note, b.w * 0.36, if *on { theme::WARN } else { theme::MUTED });
                    vw.hits.push((b, hit.clone()));
                    if hover {
                        help = about.clone();
                    }
                }
                Row::Switch { label, about, target, on, changed } => {
                    c.text_fit(Weight::Medium, 12.0 * u, (r.x, r.y + 18.0 * u), Align::Left, label, r.w * 0.5, if *changed { theme::WARN } else { theme::TEXT });
                    let b = Rect::new(r.right() - 64.0 * u, r.y + 4.0 * u, 64.0 * u, rh - 8.0 * u);
                    c.panel(b, b.h / 2.0, if *on { [70, 150, 96, 255] } else { theme::BUTTON_TOP }, if *on { [50, 110, 70, 255] } else { theme::BUTTON_BOTTOM }, 1.0, theme::EDGE);
                    c.text(Weight::SemiBold, 11.5 * u, (b.x + b.w / 2.0, b.y + 13.5 * u), Align::Centre, if *on { "on" } else { "off" }, theme::TEXT);
                    vw.hits.push((r, Hit::Switch(target.clone())));
                    if hover {
                        help = about.clone();
                    }
                }
                Row::Slider { label, about, target, vals, ranges, colour, changed } => {
                    let lw = r.w * 0.44;
                    c.text_fit(Weight::Medium, 11.5 * u, (r.x, r.y + 18.0 * u), Align::Left, label, lw - 4.0 * u, if *changed { theme::WARN } else { theme::TEXT });
                    let mut bx = r.x + lw;
                    let mut bw_all = r.w - lw;
                    if *colour {
                        let sw = Rect::new(bx, r.y + 5.0 * u, 17.0 * u, 17.0 * u);
                        let rgb = |k: usize| vals.get(k).copied().unwrap_or(0.0).clamp(0.0, 255.0) as u8;
                        c.rrect(sw, 3.0 * u, [rgb(0), rgb(1), rgb(2), 255]);
                        c.outline(sw, 3.0 * u, 1.0, theme::EDGE);
                        bx += 22.0 * u;
                        bw_all -= 22.0 * u;
                    }
                    // The whole row puts it back on a right click; its bars,
                    // pushed after, take the clicks on them.
                    vw.hits.push((r, Hit::Bar(target.clone(), usize::MAX)));
                    let n = vals.len().max(1);
                    let gap = 4.0 * u;
                    let bw = (bw_all - gap * (n - 1) as f32) / n as f32;
                    for (k, (&v, &(lo, hi))) in vals.iter().zip(ranges.iter()).enumerate() {
                        let b = Rect::new(bx + k as f32 * (bw + gap), r.y + 6.0 * u, bw, rh - 12.0 * u);
                        let fill = ((v - lo) / (hi - lo).max(1e-9)).clamp(0.0, 1.0);
                        let tint = if *colour { [[230, 90, 90, 255], [100, 200, 110, 255], [100, 150, 240, 255]][k.min(2)] } else if *changed { [200, 160, 70, 255] } else { [80, 130, 180, 255] };
                        c.bar(b, fill, tint, theme::TRACK);
                        c.text_fit(Weight::SemiBold, 10.5 * u, (b.x + b.w / 2.0, b.y + 11.0 * u), Align::Centre, &show(v), b.w, theme::TEXT);
                        vw.hits.push((b, Hit::Bar(target.clone(), k)));
                    }
                    if hover {
                        help = if about.is_empty() { label.clone() } else { format!("{label}: {about}") };
                    }
                }
            }
        }
        // A scroll bar when the list is longer than its room.
        if most > 0.0 {
            let track = Rect::new(x + w - 6.0 * u, list.y, 3.0 * u, list.h);
            let knob_h = (list.h * list.h / total).max(20.0 * u);
            let knob = Rect::new(track.x, list.y + (list.h - knob_h) * vw.scroll[tab_i] / most, track.w, knob_h);
            c.rrect(track, 1.5 * u, theme::TRACK);
            c.rrect(knob, 1.5 * u, theme::MUTED);
        }
        // The foot.
        let fy = y + h - foot_h;
        c.rrect(Rect::new(inner, fy, iw, 1.0), 0.0, theme::EDGE);
        if help.is_empty() {
            help = match vw.tab {
                Tab::Look => "Drag a bar to change the picture; right click a row to put it back. Changed rows are yellow.".into(),
                Tab::Rules => "This slime's walk applies to it only; the rest are the planet's rules. Right click a row to put it back.".into(),
                Tab::Spawn => "Placed ones go where you click the map, again and again; right click puts it down.".into(),
                Tab::Slime => "Each slime on its own planet, as a run lands there. The squad cannot lose here; B buys soldiers.".into(),
            };
        }
        c.paragraph(Weight::Medium, 11.5 * u, (inner, fy + 17.0 * u), iw, 15.0 * u, &help, theme::MUTED);
        let line = if vw.status_secs > 0.0 {
            vw.status.clone()
        } else if vw.unsaved {
            format!("{tweaks} changed, not saved")
        } else if tweaks > 0 {
            format!("{tweaks} saved tweaks in play")
        } else {
            "The game's own look and rules".into()
        };
        c.text_fit(Weight::SemiBold, 11.5 * u, (inner, fy + 56.0 * u), Align::Left, &line, iw, if vw.unsaved && vw.status_secs <= 0.0 { theme::WARN } else { theme::TEXT });
        let bh = 26.0 * u;
        let by = y + h - bh - 10.0 * u;
        let buttons: [(&str, Act); 3] = [("Save", Act::Save), ("Reset all", Act::ResetAll), ("Hide (F1)", Act::Hide)];
        let bw = (iw - 8.0 * u) / 3.0;
        for (k, (name, a)) in buttons.iter().enumerate() {
            let r = Rect::new(inner + k as f32 * (bw + 4.0 * u), by, bw, bh);
            let hover = vw.cursor.is_some_and(|p| r.contains(p));
            let hot = *a == Act::Save && vw.unsaved;
            let (t, b) = if hot { ([70, 150, 96, 255], [50, 110, 70, 255]) } else if hover { (theme::BUTTON_HOVER_TOP, theme::BUTTON_HOVER_BOTTOM) } else { (theme::BUTTON_TOP, theme::BUTTON_BOTTOM) };
            c.panel(r, 6.0 * u, t, b, 1.0, theme::EDGE);
            c.text(Weight::SemiBold, 12.5 * u, (r.x + r.w / 2.0, r.y + 17.0 * u), Align::Centre, name, theme::TEXT);
            vw.hits.push((r, Hit::Act(*a)));
        }
        // A placed spawn waiting: a note by the cursor.
        if let (Some(sp), Some(p)) = (vw.armed, vw.cursor) {
            if !panel.contains(p) {
                c.text(Weight::SemiBold, 12.5 * u, (p.0 + 16.0 * u, p.1 + 4.0 * u), Align::Left, &sp.label(), theme::WARN);
            }
        }
    }

    /// The rows of the tab open now.
    fn viewer_rows(&mut self, tier: Option<Tier>) -> Vec<Row> {
        let Some(tab) = self.viewer.as_ref().map(|v| v.tab) else { return Vec::new() };
        let mut rows = Vec::new();
        match tab {
            Tab::Slime => {
                rows.push(Row::Head { text: "Slime (starts it on its own planet)".into(), group: None, open: true });
                for (k, t) in LADDER.iter().enumerate() {
                    let look = SlimeLook::of(Some(*t));
                    rows.push(Row::Choice {
                        label: title(t.name()),
                        note: format!("planet {}, {}", k + 1, look.name()),
                        hit: Hit::Slime(k),
                        on: tier == Some(*t),
                        about: slime_mould::mould::run::does_to_you(*t).says.to_string(),
                    });
                }
                rows.push(Row::Head { text: "Body look on this slime (live)".into(), group: None, open: true });
                let now = self.view.as_ref().map(|v| v.slime_look());
                for look in SlimeLook::ALL {
                    let own = SlimeLook::of(tier) == look;
                    let about = match look {
                        SlimeLook::Plain => "Each walker's film a soft disc round it.",
                        SlimeLook::Threads => "The Sprawler's: each walker a dash along the way it came, joining into threads.",
                        SlimeLook::Sheet => "The Drifter's: its own fast-fading trail as a thin gel round and behind it.",
                        SlimeLook::Cords => "The Trunk's: its trail off the body a raised translucent tube.",
                        SlimeLook::Swell => "The Swarmer's: where its alarm is, the body stands up taller and rounder.",
                        SlimeLook::Dormant => "The Sleeper's: its sleeping walkers a dull dark crust, the waking ones gold.",
                    };
                    rows.push(Row::Choice { label: title(look.name()), note: if own { "its own".into() } else { String::new() }, hit: Hit::Body(look), on: now == Some(look), about: about.into() });
                }
                rows.push(Row::Head { text: "The round".into(), group: None, open: true });
                rows.push(Row::Choice { label: "Restart".into(), note: String::new(), hit: Hit::Act(Act::Restart), on: false, about: "The same map from its landing.".into() });
                rows.push(Row::Choice { label: "New map".into(), note: String::new(), hit: Hit::Act(Act::NewMap), on: false, about: "Another map of this planet.".into() });
                rows.push(Row::Choice { label: "Pause".into(), note: "P".into(), hit: Hit::Act(Act::Pause), on: self.paused, about: "P pauses too.".into() });
                rows.push(Row::Choice { label: "Four times faster".into(), note: "Tab".into(), hit: Hit::Act(Act::Fast), on: self.fast, about: "Tab does it too.".into() });
                if let Some(g) = self.game.as_ref() {
                    rows.push(Row::Text(format!("{} walkers, {} mini-bosses, {} soldiers, {:.0} food", g.mould, g.bosses.live.len(), g.alive(), g.bank)));
                }
            }
            Tab::Spawn => {
                let armed = self.viewer.as_ref().and_then(|v| v.armed);
                rows.push(Row::Head { text: "Where you click the map".into(), group: None, open: true });
                for sp in SPAWNS.iter().filter(|s| s.at_click()) {
                    rows.push(Row::Choice { label: sp.label(), note: if armed == Some(*sp) { "click the map".into() } else { String::new() }, hit: Hit::Spawn(*sp), on: armed == Some(*sp), about: sp.about().into() });
                }
                rows.push(Row::Head { text: "Now".into(), group: None, open: true });
                let g = self.game.as_ref();
                let forced = self.viewer.as_ref().is_some_and(|v| v.strikers_was.is_some());
                for sp in SPAWNS.iter().filter(|s| !s.at_click()) {
                    let (on, note) = match sp {
                        Spawn::Strikers => (forced, if forced { "forced on".to_string() } else { String::new() }),
                        Spawn::Heartbeat => {
                            let on = g.is_some_and(|g| g.rules.heart.on);
                            (on, if on { "on".into() } else { "off".into() })
                        }
                        Spawn::Wave => (false, g.and_then(|g| g.waves.live.as_ref()).map_or(String::new(), |w| format!("{:?}", w.phase).to_lowercase())),
                        Spawn::Stand => (false, if g.is_some_and(|g| g.stand.live.is_some()) { "alive".into() } else { String::new() }),
                        _ => (false, String::new()),
                    };
                    rows.push(Row::Choice { label: sp.label(), note, hit: Hit::Spawn(*sp), on, about: sp.about().into() });
                }
                rows.push(Row::Text("Each event holds one of the slime's few lumps while it lives.".into()));
            }
            Tab::Look => {
                let Some(vw) = self.viewer.as_ref() else { return rows };
                let mut groups: Vec<Vec<usize>> = vec![Vec::new(); LOOK_GROUPS.len() + 1];
                for (i, c) in vw.consts.iter().enumerate() {
                    groups[Viewer::group_of(&c.name)].push(i);
                }
                let mut want: Vec<(usize, usize)> = Vec::new();
                let mut heads: Vec<(usize, String, bool, usize)> = Vec::new();
                for (gi, members) in groups.iter().enumerate() {
                    if members.is_empty() {
                        continue;
                    }
                    let (head, first) = LOOK_GROUPS.get(gi).map_or((OTHER, false), |g| (g.0, g.2));
                    let open = vw.group_open(head, first);
                    heads.push((gi, head.to_string(), open, members.len()));
                    if open {
                        want.extend(members.iter().map(|&i| (gi, i)));
                    }
                }
                let mut out = Vec::new();
                for (gi, head, open, n) in heads {
                    out.push(Row::Head { text: format!("{head} ({n})"), group: Some(head.clone()), open });
                    for &(_, i) in want.iter().filter(|(g, _)| *g == gi) {
                        out.push(self.look_row(i));
                    }
                }
                return out;
            }
            Tab::Rules => {
                let name = tier.map_or("This slime".to_string(), |t| title(t.name()));
                rows.push(Row::Head { text: format!("{name}'s walk (this slime only, live)"), group: None, open: true });
                for (key, about, _) in ENGINE_KEYS {
                    if let Some(r) = self.slider(Target::Engine(key), key, about) {
                        rows.push(r);
                    }
                }
                for (head, keys) in RULE_GROUPS {
                    rows.push(Row::Head { text: (*head).into(), group: None, open: true });
                    for (key, about) in keys.iter() {
                        if let Some(r) = self.slider(Target::Rule(key), key, about) {
                            rows.push(r);
                        }
                    }
                }
            }
        }
        rows
    }

    fn look_row(&mut self, i: usize) -> Row {
        let (name, about, colour, vals, first) = {
            let vw = self.viewer.as_ref().expect("the viewer");
            let c = &vw.consts[i];
            (c.name.clone(), c.about.clone(), c.colour, vw.look[i].clone(), c.value.clone())
        };
        let t = Target::Look(i);
        let ranges = (0..vals.len()).map(|k| self.viewer_range(&t, k)).collect();
        Row::Slider { label: name, about, target: t, changed: vals != first, vals, ranges, colour }
    }

    /// A rule's or engine key's row, or None where the game has no such
    /// number.
    fn slider(&mut self, t: Target, key: &str, about: &str) -> Option<Row> {
        let vals = self.viewer_value(&t);
        let v = *vals.first()?;
        let first = self.viewer_first(&t);
        let changed = first.first().is_some_and(|f| (f - v).abs() > 1e-6);
        if let Target::Rule(n) = t {
            if SWITCHES.contains(&n) {
                return Some(Row::Switch { label: key.into(), about: about.into(), target: t, on: v > 0.0, changed });
            }
        }
        let range = self.viewer_range(&t, 0);
        Some(Row::Slider { label: key.into(), about: about.into(), target: t, vals, ranges: vec![range], colour: false, changed })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rule_and_engine_key_the_panel_shows_is_read_and_set() {
        let mut rules = Rules::default();
        for (_, keys) in RULE_GROUPS {
            for (k, _) in keys.iter() {
                let v = rules.get(k).unwrap_or_else(|| panic!("Rules::get({k})"));
                let want = if SWITCHES.contains(k) { 1.0 - v.min(1.0) } else { (v + 1.0).round() };
                assert!(rules.set(k, want), "Rules::set({k})");
                assert_eq!(rules.get(k), Some(want), "{k} reads back what was set");
            }
        }
        let mut s = race_settings(1, 2, 5000);
        for (k, _, (lo, hi)) in ENGINE_KEYS {
            assert!(engine_get(&s, k).is_some(), "{k}");
            let want = if *k == "explore" { 300.0 } else { (lo + hi) / 2.0 };
            slime_mould::gpu::set(&mut s, k, &format!("{want}")).unwrap_or_else(|e| panic!("{k}: {e}"));
            assert!((engine_get(&s, k).unwrap() - want).abs() < 1e-6, "{k} reads back what was set");
        }
    }

    #[test]
    fn the_look_groups_hold_the_slimes_numbers() {
        let vw = Viewer::new(&Tweaks::default());
        let group = |n: &str| LOOK_GROUPS.get(Viewer::group_of(n)).map_or(OTHER, |g| g.0);
        assert_eq!(group("SHEET_VEIL"), "Each slime's body look");
        assert_eq!(group("CORD_RISE"), "Each slime's body look");
        assert_eq!(group("LUMP_CORE_MORTAR"), "Mini-bosses, fruit and heart");
        assert_eq!(group("MOULD_HUE"), "The slime's body");
        assert_eq!(group("DUST"), "Trails on bare ground");
        assert_eq!(group("AGAR_RGB"), OTHER);
        assert!(vw.consts.iter().all(|c| c.value.iter().all(|v| v.abs() <= 10_000.0)), "no flag-sized numbers");
        assert_eq!(vw.source(), vw.base, "nothing changed, nothing rewritten");
    }

    #[test]
    fn staged_spawns_read_their_times_names_and_places() {
        let s = staged_of("10:mortar@200,150; 12:wave;bad;14:fruit@1,2;3:nothing");
        assert_eq!(s, vec![(10.0, Spawn::Boss(BossKind::Mortar), Some((200.0, 150.0))), (12.0, Spawn::Wave, None), (14.0, Spawn::Fruit, Some((1.0, 2.0)))]);
        for sp in SPAWNS {
            let name = match sp {
                Spawn::Boss(k) => k.name().to_string(),
                other => format!("{other:?}").to_lowercase(),
            };
            assert_eq!(spawn_named(&name), Some(sp), "{name}");
        }
    }

    #[test]
    fn values_show_in_three_figures() {
        assert_eq!(show(0.0), "0");
        assert_eq!(show(240.0), "240");
        assert_eq!(show(0.4), "0.400");
        assert_eq!(show(22.5), "22.5");
        assert_eq!(show(0.00001), "1.0e-5");
        assert_eq!(default_range(0.4), (0.0, 1.0));
        assert_eq!(default_range(3.0), (0.0, 6.0));
        assert_eq!(default_range(-0.5), (-1.0, 0.0));
    }
}
