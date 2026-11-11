//! Named mould tiers for the Larder World ladder (Sporer, Sleeper,
//! Swarmer): each is a set of the big dish's life-cycle keys, so the game
//! asks for `Tier::Sporer` instead of copying a dozen numbers from a report.
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
//! The GPU engine has none of the life-cycle rules yet (the GPU thread is
//! porting them). [`Tier::settings`] passes each key through `gpu::set`, so
//! it refuses with the missing key's name until the port lands, and works
//! unchanged once the GPU takes the CPU names.

use super::big::BigParams;

/// A rung of the mould ladder that needs the life cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    /// Fed walkers fruit into spores that fly to new food; struck, it
    /// scatters spores (`spore_burst`). Counter: starve it, and don't
    /// strike it near untaken food.
    Sporer,
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
    pub const ALL: [Tier; 3] = [Tier::Sporer, Tier::Sleeper, Tier::Swarmer];

    pub fn name(self) -> &'static str {
        match self {
            Tier::Sporer => "sporer",
            Tier::Sleeper => "sleeper",
            Tier::Swarmer => "swarmer",
        }
    }

    pub fn from_name(name: &str) -> Option<Tier> {
        Tier::ALL.into_iter().find(|t| t.name() == name)
    }

    /// The tier as `key=value` pairs, with the CPU's key names (the ones
    /// `examples/bigdish.rs` takes).
    pub fn keys(self) -> Vec<(&'static str, &'static str)> {
        match self {
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
            ],
            Tier::Swarmer => vec![("alarm_keep", "0.98"), ("alarm_weight", "3")],
        }
    }

    /// Sets the tier's keys on CPU big-dish parameters (other fields are
    /// left as they are).
    pub fn apply(self, p: &mut BigParams) {
        for (k, v) in self.keys() {
            set_life(p, k, v).unwrap_or_else(|e| panic!("tier {}: {e}", self.name()));
        }
    }

    /// Sets the tier's keys on the GPU engine's settings, or names the first
    /// key the GPU does not take yet. Today that is every tier: spores,
    /// dormancy and the alarm field are CPU-only until the GPU life-cycle
    /// port, so a GPU game must refuse the tier rather than run without it.
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

/// The life-cycle keys a tier may set, by their CPU names.
fn set_life(p: &mut BigParams, key: &str, v: &str) -> Result<(), String> {
    let f = || v.parse::<f32>().map_err(|e| format!("{key}={v}: {e}"));
    let n = || v.parse::<u16>().map_err(|e| format!("{key}={v}: {e}"));
    match key {
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
        "alarm_keep" => p.alarm_keep = f()?,
        "alarm_weight" => p.alarm_weight = f()?,
        "alarm_relay" => p.alarm_relay = f()?,
        _ => return Err(format!("{key} is not a life-cycle key")),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip_and_every_key_sets() {
        for t in Tier::ALL {
            assert_eq!(Tier::from_name(t.name()), Some(t));
            let mut p = BigParams::today();
            t.apply(&mut p);
            assert_ne!(format!("{p:?}"), format!("{:?}", BigParams::today()), "{} changed nothing", t.name());
        }
        assert_eq!(Tier::from_name("trunk"), None);
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
        // Every tier with an alarm rule keeps its alarm (a field that decays
        // in one step does nothing).
        for t in Tier::ALL {
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_gpu_refuses_a_tier_it_cannot_run_and_keeps_its_settings() {
        let mut s = crate::gpu::Settings::big_dish();
        let before = format!("{s:?}");
        for t in Tier::ALL {
            match t.settings(&mut s) {
                Ok(()) => {}
                Err(e) => {
                    assert!(e.contains(t.name()), "{e}");
                    assert_eq!(format!("{s:?}"), before, "a refused tier changed the settings");
                }
            }
        }
    }
}
