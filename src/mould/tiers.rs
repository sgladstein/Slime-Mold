//! Named mould tiers for the Larder World ladder (Drifter, Sprawler, Trunk,
//! Sporer, Sleeper, Swarmer): each is a set of the big dish's keys, so the
//! game asks for `Tier::Sporer` instead of copying a dozen numbers from a
//! report.
//!
//! The Drifter, Sprawler and Trunk (2026-11-12, for the campaign's missions
//! 1-4; `Reports/mould-tiers-campaign-2026-11-12.md`) are shapes, not life
//! cycles: each sets the walk (sensor angle and reach, step, trail kept,
//! explorers, what a packed walker lays) on rules both engines already run,
//! so no new GPU row. Angles are in degrees, as `examples/bigdish.rs` and
//! `gpu::set` take them; `Tier::apply` turns them into the CPU's radians.
//!
//! The tiers and their counters come from `Reports/mould-tiers-2026-11-11.md`
//! (each beat its control on 8 of 8 paired seeds on the 20,000-walker dish)
//! and its world re-test (same report, "In the Larder world"). The rules
//! themselves, and their doc comments, are `BigParams`' `spore_*`,
//! `sleep_*`, `wake_*` and `alarm_*` fields; `wiki/model/life-cycle.md` is
//! the page.
//!
//! The values are the report's. A tier is data, not a rule: changing a
//! number here changes the creature, not the model, so a playtest can tune
//! it without touching `big.rs`.
//!
//! The GPU engine runs every life-cycle rule under the CPU's names (the
//! port, `Reports/life-gpu-2026-11-11.md`, 2026-11-12, with `sleep_death`
//! and the shot alarm). [`Tier::settings`] passes each key through
//! `gpu::set`; before the port it refused with the missing key's name, and
//! it still would for a key a later tier adds before the GPU takes it.

use super::big::BigParams;

/// A rung of the mould ladder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    /// Droplets: round bodies that sit on crystals and crawl between them
    /// on faint threads (sensors 90 degrees apart, trail kept 0.93, a packed
    /// walker lays its trail where it stands). Slow to spread. Counter: any
    /// squad; shoot a body off its crystal. Mission 1-2's mould.
    Drifter,
    /// Explorers: each walker first walks away from trail for up to 800
    /// fine steps, then joins the network, so the mould throws a fast, thin
    /// mesh over the whole map with few walkers in it (step 1.5, reach 18,
    /// trail kept 0.8). Counter: walls; chasing it loses. Mission 3's.
    Sprawler,
    /// Long straight cords (sensors 22.5 degrees apart reading 18 fine
    /// cells out, trail kept 0.95) that hold a few big bodies together and
    /// re-form round a hole, since the trail outlasts it. Counter: starve
    /// it (cut a cord; the one body thread's shared food, `energy_flow`, is
    /// what makes a cut piece starve, and joins this tier once the GPU runs
    /// it). Mission 4's.
    Trunk,
    /// Fed walkers fruit into spores that fly to new food; struck, it
    /// scatters spores (`spore_burst`). Counter: starve it, and don't
    /// strike it near untaken food.
    Sporer,
    /// Habituation, the avoiding answer: a scar field remembers where the
    /// mould was shot (the alarm, kept about 40 s) and it keeps away from
    /// a place hurt again and again, long after the alarm has faded; shots
    /// laid there move it a little (alarm weight -1). Counter: burn a road
    /// clear once and it stays clear; but it will not come where you wait.
    Wary,
    /// Habituation, the swarming answer: the same scar, read the other way,
    /// with the Swarmer's alarm: each shot at one place brings more of it
    /// back there (kills a strike rose 16 to 306 over 16 strikes, seed 1).
    /// Counter: move; don't fire twice from one spot, or make it a kill
    /// zone.
    Grudge,
    /// A dormant crust that wakes in a wave and rushes a wound, then is deaf
    /// for 5 s (`sleep_rest`). Counter: bait it, then strike.
    Sleeper,
    /// Walks toward the alarm a strike lays (`alarm_weight` +3), so it
    /// floods the place it was hit. Counter: a kill zone.
    Swarmer,
}

/// The shared keys of the spore flight: spores follow the food scent,
/// home only on their second half, land on food, and lay a little trail.
const SPORE: [(&str, &str); 4] = [("spore_scent", "1"), ("spore_reach", "36"), ("spore_land", "1"), ("spore_deposit", "0.2")];

impl Tier {
    pub const ALL: [Tier; 8] = [Tier::Drifter, Tier::Sprawler, Tier::Trunk, Tier::Sporer, Tier::Sleeper, Tier::Swarmer, Tier::Wary, Tier::Grudge];

    pub fn name(self) -> &'static str {
        match self {
            Tier::Drifter => "drifter",
            Tier::Sprawler => "sprawler",
            Tier::Trunk => "trunk",
            Tier::Wary => "wary",
            Tier::Grudge => "grudge",
            Tier::Sporer => "sporer",
            Tier::Sleeper => "sleeper",
            Tier::Swarmer => "swarmer",
        }
    }

    pub fn from_name(name: &str) -> Option<Tier> {
        Tier::ALL.into_iter().find(|t| t.name() == name)
    }

    /// The tier as `key=value` pairs, with the CPU's key names (the ones
    /// `examples/bigdish.rs` takes; angles in degrees).
    pub fn keys(self) -> Vec<(&'static str, &'static str)> {
        match self {
            // Chosen by eye on the race world, seed 1, from 15 candidates
            // (`Reports/mould-tiers-campaign-2026-11-12/`, look.sh). Grazing
            // (graze_weight 10) was the first Drifter: grass fed it to the
            // 50,000-slot cap in 15 s, a flood, not a crawl. Step 0.6 held
            // 3 crystals in 60 s, too still to reach a squad.
            Tier::Drifter => vec![("sensor_angle", "90"), ("keep", "0.93"), ("blocked_deposit", "1")],
            // Explorers alone (keep 0.9, step 1) starved to 250 walkers by
            // 60 s and let go of every crystal; the faster step and longer
            // reach keep a mesh on 14 of 23. Explore 400 steps held about
            // 1,100 walkers, and in the census (seeds 201-216) every plan
            // and random wiped it, idle survived: newborns start as
            // explorers (age 0) and leave food before they eat. At 200 it
            // holds 17 crystals on seeds 1 and 2, and on dev seeds 1-4
            // random lost the squad 2 of 4 while wall and strike won 4 of 4.
            Tier::Sprawler => vec![("explore_steps", "200"), ("explore_spread", "1"), ("keep", "0.8"), ("step", "1.5"), ("sensor_offset", "18")],
            // Reach 27 with keep 0.97 drew the widest cords but stopped
            // growing at about 1,900 walkers; crowd 0 starved (1,000); two
            // trail passes thicken them further but the GPU has no such key.
            Tier::Trunk => vec![("sensor_angle", "22.5"), ("sensor_offset", "18"), ("keep", "0.95")],
            // 1e-5: at 1e-4 it took all 7 piles by 10 s on 6 of 8 seeds,
            // too fast for the owner's rule (14:59Z, no mould takes the map
            // in about 10 s). Struck, 5% of the fed walkers in the alarm
            // fruit at once (spore_burst with spore_alarm).
            Tier::Sporer => {
                let mut k = SPORE.to_vec();
                k.extend([("spore_rate", "0.00001"), ("spore_burst", "0.05"), ("spore_alarm", "1"), ("alarm_keep", "0.98")]);
                k
            }
            Tier::Sleeper => vec![
                ("sleep_rate", "0.01"),
                ("sleep_deposit", "0.3"),
                ("wake_scent", "1"),
                ("wake_alarm", "1"),
                ("alarm_keep", "0.98"),
                ("alarm_relay", "20"),
                ("alarm_weight", "3"),
                ("sleep_rest", "600"),
                // A dormant walker dies with chance 1e-4 a fine step (a mean
                // of 10,000 steps, 42 s at the world's 2 substeps). A free
                // crust never dies, so a world whose crystals ran dry froze
                // (review 1, 2026-11-11); paying upkeep while asleep lost
                // the bait counter in the world (`Reports/mould-tiers-2026-11-11.md`,
                // *In the Larder world*).
                ("sleep_death", "0.0001"),
            ],
            Tier::Swarmer => vec![("alarm_keep", "0.98"), ("alarm_weight", "3")],
            // The scar: keep 0.9999 a fine step, gain 0.01 of the alarm, so
            // 16 strikes 2 s apart leave a peak of about 250 that halves in
            // about 40 s; read at -0.5 / +0.5 beside trail and scent
            // (`Reports/mould-tiers-campaign-2026-11-12.md`, *Habituation*).
            Tier::Wary => vec![("alarm_keep", "0.98"), ("alarm_weight", "-1"), ("scar_keep", "0.9999"), ("scar_gain", "0.01"), ("scar_weight", "-0.5")],
            Tier::Grudge => vec![("alarm_keep", "0.98"), ("alarm_weight", "3"), ("scar_keep", "0.9999"), ("scar_gain", "0.01"), ("scar_weight", "0.5")],
        }
    }

    /// The tier's core (`larder_world::Core`), or None for the plain
    /// mould's (`Core::of(&rules)`: fixed, `Rules::core_secs`,
    /// `Rules::core_pull`). Every tier is None for now; the tiers thread
    /// fills in its own (the Sprawler headless, the Grudge's pull) once the
    /// core-defence PR is in. Read once, by `Game::with_tier`.
    pub fn core(self) -> Option<super::larder_world::Core> {
        None
    }

    /// Sets the tier's keys on CPU big-dish parameters (other fields are
    /// left as they are).
    pub fn apply(self, p: &mut BigParams) {
        for (k, v) in self.keys() {
            set_key(p, k, v).unwrap_or_else(|e| panic!("tier {}: {e}", self.name()));
        }
    }

    /// Sets the tier's keys on the GPU engine's settings, or names the first
    /// key the GPU does not take. Since the GPU life-cycle port it takes
    /// every key of the three tiers; a GPU game must still refuse a tier
    /// that names a key it lacks rather than run without it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn settings(self, s: &mut crate::gpu::Settings) -> Result<(), String> {
        // Set on a copy, so a refusal leaves `s` as it was.
        let mut t = s.clone();
        for (k, v) in self.keys() {
            crate::gpu::set(&mut t, k, v).map_err(|e| format!("tier {} needs {k}, which the GPU engine does not take yet ({e})", self.name()))?;
        }
        *s = t;
        Ok(())
    }
}

/// The keys a tier may set, by their CPU names (angles in degrees).
fn set_key(p: &mut BigParams, key: &str, v: &str) -> Result<(), String> {
    let f = || v.parse::<f32>().map_err(|e| format!("{key}={v}: {e}"));
    let n = || v.parse::<u16>().map_err(|e| format!("{key}={v}: {e}"));
    match key {
        // The walk.
        "sensor_angle" => p.sensor_angle = f()?.to_radians(),
        "rotate_angle" => p.rotate_angle = f()?.to_radians(),
        "sensor_offset" => p.sensor_offset = f()?,
        "step" => p.step = f()?,
        "keep" => p.keep = f()?,
        "blocked_deposit" => p.blocked_deposit = f()?,
        "explore_steps" => p.explore_steps = n()?,
        "explore_spread" => p.explore_spread = f()?,
        "explore_deposit" => p.explore_deposit = f()?,
        // The life cycle.
        "spore_rate" => p.spore_rate = f()?,
        "spore_scent" => p.spore_scent = f()?,
        "spore_alarm" => p.spore_alarm = f()?,
        "spore_burst" => p.spore_burst = f()?,
        "spore_steps" => p.spore_steps = n()?,
        "spore_speed" => p.spore_speed = f()?,
        "spore_reach" => p.spore_reach = f()?,
        "spore_deposit" => p.spore_deposit = f()?,
        "spore_land" => p.spore_land = f()?,
        "spore_rest" => p.spore_rest = n()?,
        "sleep_rate" => p.sleep_rate = f()?,
        "sleep_deposit" => p.sleep_deposit = f()?,
        "wake_scent" => p.wake_scent = f()?,
        "wake_alarm" => p.wake_alarm = f()?,
        "wake_steps" => p.wake_steps = n()?,
        "sleep_rest" => p.sleep_rest = n()?,
        "sleep_death" => p.sleep_death = f()?,
        "alarm_keep" => p.alarm_keep = f()?,
        "alarm_weight" => p.alarm_weight = f()?,
        "alarm_relay" => p.alarm_relay = f()?,
        "scar_keep" => p.scar_keep = f()?,
        "scar_gain" => p.scar_gain = f()?,
        "scar_weight" => p.scar_weight = f()?,
        _ => return Err(format!("{key} is not a tier key")),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether the GPU engine runs the tier yet: every tier but the two
    /// habituation tiers, whose scar field waits for its GPU port (a new
    /// binding, through the integration thread's Params queue).
    fn on_gpu(t: Tier) -> bool {
        !t.keys().iter().any(|(k, _)| k.starts_with("scar_"))
    }

    #[test]
    fn names_round_trip_and_every_key_sets() {
        for t in Tier::ALL {
            assert_eq!(Tier::from_name(t.name()), Some(t));
            let mut p = BigParams::today();
            t.apply(&mut p);
            assert_ne!(format!("{p:?}"), format!("{:?}", BigParams::today()), "{} changed nothing", t.name());
        }
        assert_eq!(Tier::from_name("hunter"), None);
    }

    #[test]
    fn each_tier_turns_on_its_rule() {
        let on = |t: Tier| {
            let mut p = BigParams::today();
            t.apply(&mut p);
            p
        };
        assert!(on(Tier::Sporer).spore_rate > 0.0 && on(Tier::Sporer).spore_burst > 0.0);
        assert!(on(Tier::Sleeper).sleep_rate > 0.0 && on(Tier::Sleeper).sleep_rest > 0);
        assert!(on(Tier::Swarmer).alarm_weight > 0.0 && on(Tier::Swarmer).sleep_rate == 0.0);
        assert!((on(Tier::Drifter).sensor_angle - std::f32::consts::FRAC_PI_2).abs() < 1e-6, "degrees become radians");
        assert!(on(Tier::Sprawler).explore_steps > 0 && on(Tier::Sprawler).explore_spread > 0.0);
        assert!(on(Tier::Wary).scar_weight < 0.0 && on(Tier::Grudge).scar_weight > 0.0 && on(Tier::Grudge).alarm_weight > 0.0);
        assert!(on(Tier::Trunk).sensor_angle < BigParams::today().sensor_angle && on(Tier::Trunk).keep > BigParams::today().keep);
        // Every tier with an alarm rule keeps its alarm (a field that decays
        // in one step does nothing).
        for t in [Tier::Sporer, Tier::Sleeper, Tier::Swarmer, Tier::Wary, Tier::Grudge] {
            let p = on(t);
            assert!(p.alarm_keep > 0.5, "{}", t.name());
        }
    }

    #[test]
    fn a_tier_runs_and_leaves_the_default_off() {
        let mut p = BigParams { walkers: 600, piles: 3, seed: 3, ..BigParams::today() };
        Tier::Sleeper.apply(&mut p);
        let mut d = crate::mould::big::BigDish::new(p);
        for _ in 0..240 {
            d.tick();
        }
        assert!(d.life_counts()[2] > 0, "no walker fell asleep: {:?}", d.life_counts());
        assert_eq!(BigParams::today().sleep_rate, 0.0);
    }

    #[test]
    fn a_crust_that_can_die_cannot_freeze_the_world() {
        // Growth on, every walker off food falls asleep at once; no food
        // wakes them (`wake_scent` out of reach).
        let dormant_after = |sleep_death: f32| {
            let p = BigParams { walkers: 400, capacity: 800, upkeep: 0.01, piles: 1, seed: 5, sleep_rate: 1.0, wake_scent: 1e9, sleep_death, ..BigParams::today() };
            let mut d = crate::mould::big::BigDish::new(p);
            for _ in 0..600 {
                d.tick();
            }
            d.life_counts()[2]
        };
        let (free, dying) = (dormant_after(0.0), dormant_after(0.01));
        assert!(free >= 20, "free sleepers live on: {free}");
        assert!(dying * 4 < free, "sleepers that can die fade: {dying} against {free}");
    }

    /// Every tier runs on the GPU engine, and its settings make the same
    /// dish parameters as `Tier::apply` on the CPU's (each key reaches its
    /// `BigParams` field through `gpu::big_params`). Before the GPU port this
    /// test checked that the GPU refused each tier, naming it, and left its
    /// settings alone.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_gpu_takes_every_tier_as_the_cpu_does() {
        for t in Tier::ALL {
            let mut s = crate::gpu::Settings::big_dish();
            let before = format!("{s:?}");
            if !on_gpu(t) {
                let e = t.settings(&mut s).unwrap_err();
                assert!(e.contains("scar_keep"), "{e}");
                assert_eq!(format!("{s:?}"), before, "a refusal leaves the settings alone");
                continue;
            }
            t.settings(&mut s).unwrap_or_else(|e| panic!("{e}"));
            assert_ne!(format!("{s:?}"), before, "{} changed nothing", t.name());
            let mut want = crate::gpu::big_params(&crate::gpu::Settings::big_dish(), 1, false);
            t.apply(&mut want);
            assert_eq!(format!("{:?}", crate::gpu::big_params(&s, 1, false)), format!("{want:?}"), "{}", t.name());
            // The sandbox's `tier=` key sets the same.
            let mut by_key = crate::gpu::Settings::big_dish();
            crate::gpu::set(&mut by_key, "tier", t.name()).unwrap();
            assert_eq!(format!("{by_key:?}"), format!("{s:?}"), "tier={}", t.name());
        }
        assert!(crate::gpu::set(&mut crate::gpu::Settings::big_dish(), "tier", "hunter").is_err());
    }

    /// The race world's GPU settings with a tier make its CPU params with
    /// the same tier (bar the fine steps a tick), so a campaign mission's
    /// mould is the same creature in the window and in the census.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn each_tier_on_the_gpu_race_world_is_the_cpus() {
        use crate::gpu::larder_world::{engine_params, race_settings};
        for t in Tier::ALL.into_iter().filter(|&t| on_gpu(t)) {
            for (seed, scale, walkers) in [(1, 2, 5_000), (3, 4, 20_000)] {
                let mut cpu = crate::mould::larder_world::race_params(seed, scale, walkers);
                t.apply(&mut cpu);
                let mut s = race_settings(seed, scale, walkers);
                t.settings(&mut s).unwrap_or_else(|e| panic!("{e}"));
                let gpu = BigParams { substeps: cpu.substeps, ..engine_params(&s, scale) };
                assert_eq!(format!("{gpu:?}"), format!("{cpu:?}"), "{} seed {seed} scale {scale}", t.name());
            }
        }
    }
}
