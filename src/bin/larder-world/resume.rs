//! **Save and resume A in the window** (Milestone 2, Direction 2026-11-24):
//! quitting mid-planet in a run writes the planet as it stands to
//! `planet.sav` beside the run's `run.txt` (`Game::save`: the game and
//! every GPU buffer, a few tens of MB), and the main menu's "Back to the
//! run" lands that planet and reads the save onto it instead of opening the
//! shop. The file only lives between a quit and the next resume: it is
//! deleted once read (loaded or not), when a planet lands fresh, and with
//! the run.
//!
//! A save from another build, or of another planet, or one that does not
//! read, is set aside and the planet relands fresh with the run intact
//! (step C, #748), with a word on the HUD. Written on the window's close,
//! the pause menu's Quit and Main menu; not every few seconds, since a save
//! reads back the whole GPU world (a crash relands the planet, as before).

use super::*;
use slime_mould::mould::larder_world::snap;

/// The window's own layer over `Game::save`: the mission's tracker and its
/// clock, which live in the window.
const WINDOW_HEAD: &str = "plasmodia window planet 1";

impl App {
    /// `planet.sav`, beside the run's save.
    fn planet_file(&self) -> Option<PathBuf> {
        self.run_file.as_ref().map(|p| p.with_file_name("planet.sav"))
    }

    /// The planet being played in a run, as the save names it: the map,
    /// the run's seed and planet.
    pub(super) fn planet_tag(&self) -> Option<String> {
        let r = self.args.run.as_ref().filter(|r| !r.over && !r.range && !r.field)?;
        Some(format!("{} run {} planet {}", self.name, r.seed, r.depth))
    }

    /// Whether quitting now would leave a planet to carry on: a run's
    /// planet under way, with no verdict yet.
    pub(super) fn planet_open(&self) -> bool {
        self.run.is_some()
            && self.planet_tag().is_some()
            && self.viewer.is_none()
            && self.game.as_ref().is_some_and(|g| g.end.is_none())
            && self.mission.as_ref().is_some_and(|t| t.verdict.is_none())
    }

    /// Writes the planet under way (`planet_open`) to `planet.sav`, or
    /// removes a stale one. A side file renamed over it, as the run's.
    pub(super) fn save_planet(&mut self) {
        let Some(p) = self.planet_file() else { return };
        if !self.planet_open() {
            self.drop_planet();
            return;
        }
        let Some(tag) = self.planet_tag() else { return };
        let t0 = Instant::now();
        let Some(g) = self.game.as_mut() else { return };
        let bytes = g.save(&tag).and_then(|game| bincode::serialize(&(WINDOW_HEAD, game, &self.mission, self.mission_sec)).map_err(|e| format!("save: {e}")));
        let wrote = bytes.and_then(|b| {
            let tmp = p.with_extension("sav.new");
            std::fs::write(&tmp, &b).map_err(|e| format!("{}: {e}", tmp.display()))?;
            std::fs::rename(&tmp, &p).map_err(|e| format!("{}: {e}", p.display()))?;
            Ok(b.len())
        });
        match wrote {
            Ok(n) => eprintln!("larder-world: the planet was saved ({:.1} MB in {:.0} ms)", n as f64 / 1e6, t0.elapsed().as_secs_f64() * 1e3),
            Err(e) => {
                eprintln!("larder-world: the planet was not saved ({e}); it relands with the run");
                self.drop_planet();
            }
        }
    }

    /// Removes `planet.sav`, if there is one.
    pub(super) fn drop_planet(&self) {
        if let Some(p) = self.planet_file() {
            let _ = std::fs::remove_file(p);
        }
    }

    /// The planet `planet.sav` holds, if it is this run's planet and this
    /// build's: (its tag, the bytes).
    pub(super) fn planet_saved(&self) -> Option<Vec<u8>> {
        let bytes = std::fs::read(self.planet_file()?).ok()?;
        let r = self.run.as_ref()?;
        let (_, game, _, _): (String, Vec<u8>, Option<Tracker>, u64) = bincode::deserialize(&bytes).ok()?;
        let (commit, tag) = snap::peek(&game).ok()?;
        let fits = commit == slime_mould::build::build().commit && tag.ends_with(&format!(" run {} planet {}", r.seed, r.depth));
        fits.then_some(bytes)
    }

    /// "Back to the run" with a planet saved mid-game: lands the run's
    /// planet and reads the save onto it. False when there is none to read
    /// (the shop opens, as before); a save that won't read leaves the
    /// planet landed fresh, with a word on the HUD.
    pub(super) fn resume_planet(&mut self) -> Result<bool, String> {
        let Some(bytes) = self.planet_saved() else {
            self.drop_planet();
            return Ok(false);
        };
        let Some((base, _)) = self.base.clone() else { return Ok(false) };
        let Some(mut r) = self.run.take() else { return Ok(false) };
        let Some(menu) = self.menu.take() else {
            self.run = Some(r);
            return Ok(false);
        };
        r.dev = menu.option("dev").is_some_and(|o| o.value() == "on");
        self.args = run_args(&base, &menu, &r);
        self.run = Some(r);
        self.round = self.args.round;
        self.parked = Some(menu);
        self.start()?;
        let read = self.read_planet(&bytes);
        self.drop_planet();
        match read {
            Ok(()) => {
                // Paused, so the player finds the planet before it moves.
                self.paused = true;
                self.hud.alert("Carried on where you left the planet: P to play", None, theme::ACCENT);
            }
            Err(e) => {
                eprintln!("larder-world: the saved planet did not load ({e}); it relands fresh");
                self.start()?;
                self.paused = false;
                self.hud.alert("The saved planet could not be read: it starts over, your run intact", None, theme::WARN);
            }
        }
        self.save_run();
        Ok(true)
    }

    /// The save read onto the game `start` just built for its planet.
    fn read_planet(&mut self, bytes: &[u8]) -> Result<(), String> {
        let (head, game, mission, mission_sec): (String, Vec<u8>, Option<Tracker>, u64) = bincode::deserialize(bytes).map_err(|e| format!("load: {e}"))?;
        if head != WINDOW_HEAD {
            return Err(format!("load: not a window save ({head:?})"));
        }
        let tag = self.planet_tag().ok_or("load: no run planet landed")?;
        let fresh = self.game.take().ok_or("load: no planet landed")?;
        let Game { world, .. } = fresh;
        self.game = Some(Game::from_save(&game, &tag, world)?);
        self.mission = mission;
        self.mission_sec = mission_sec;
        Ok(())
    }
}
