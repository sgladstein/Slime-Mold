//! **A gun's damage a second, estimated** (the owner, 2026-11-22 04:20Z:
//! "If there is a way to see a calculated/estimated dps of you gun, that
//! would be cool"), for the shop: what one soldier's gun and cards clear
//! of the slime a second if every bullet lands in it.
//!
//! **Damage** is the bullets' own unit (`bullets::DAMAGE`): the slime a hit
//! clears, by area, where the standard rifle's plain strike on the default
//! rules is 10. So the starting rifle, one shot a second, is 10 a second,
//! and a card or gun reads against it. What is counted, all of it read from
//! the rules the game fires with:
//! - each bullet's strike (`Rules::strike` x the bullet's `Ammo::reach`,
//!   cut at `squad::R_MAX` fine cells, x its set's share), its pellets
//!   (buckshot) and its gun's bullets a shot (the scattergun);
//! - a strike past the cap fires faster instead (`Game::strike_over`);
//! - its kill bursts (`rounds::bursts` with `kill`: Blast, Bomb), Wide and
//!   Scatter included, at each slime hit, and the plain bullet's pop with
//!   a modifier on it (`rounds::plain_bursts`: Wide, Scatter, Delay,
//!   Thick), less the part of it the strike already cleared;
//! - the hits it flies on through (`Card::Pierce`, the needle's), the
//!   pieces a split leaves (`Card::Split`, `Rules::split_n` at `split_r`),
//!   the casts fired beside it (`Card::Multicast`) and carried by it
//!   (`Card::Trigger`, `Expire`, let out where it hits);
//! - the gun's rate (`Gun::rate`) and the `Rate` cards.
//!
//! **Not counted**: misses (it is every shot landing), the cards that work
//! on the slime over time or by its rules (gas, germ, the Mind bullets,
//! readers, flare, torch, the trails, shoves), the controlled squad's
//! edge, and walkers killed by a hit's alarm or a fire. So it ranks guns
//! and the damage cards; the others say what they do in words, and the
//! shop shows their estimate unchanged rather than a guess.
use super::bullets::DAMAGE;
use super::guns::Gun;
use super::larder_world::Rules;
use super::rounds::{bursts, casts, cycle, plain_bursts, volley, Card, Cast, RoundRules};
use super::squad::R_MAX;

/// The dish scale the game's planets run at (`larder-world`'s `scale=2`):
/// fine cells a dish cell, which sets where the strike cap bites.
pub const SCALE: f32 = 2.0;

/// Area of a circle of radius `r`, without the pi (it cancels).
fn area(r: f32) -> f32 {
    r * r
}

/// Shots a second as the game fires them: the gun's gap, cut by the `Rate`
/// cards and by a strike past the cap (`Game::strike_over`), then snapped
/// to whole ticks as `Game::shot_gap` does (`Game::secs` rounds, at least
/// one tick). So two Rate cards on the rifle are 15 / 8 = 1.875 a second,
/// not 2 (the review lane, 2026-11-22).
fn fire_rate(gun: &Gun, rate_cards: f32, rules: &Rules) -> f32 {
    let over = (rules.strike * SCALE / R_MAX).max(1.0).powi(2);
    let every = gun.every() / (1.0 + rules.rate_card * rate_cards).max(0.01) / over;
    let tps = rules.tps.max(1) as f32;
    tps / (every * tps).round().max(1.0)
}

/// The estimate of one soldier firing `gun` (as armed: the run's upgrades
/// on it) with `cards` (the gun's, in firing order), under `rules` and
/// `rr` (the run's), in damage a second.
pub fn estimate(gun: &Gun, cards: &[Card], rules: &Rules, rr: &RoundRules) -> f32 {
    let sc = SCALE;
    // The unit: the default rules' plain strike, capped as the game caps it.
    let unit = area((Rules::default().strike * sc).min(R_MAX));
    let want = rules.strike * sc;
    let strike = want.min(R_MAX);
    let n = |k: Card| cards.iter().take(super::rounds::MAX_SLOTS).filter(|&&c| c == k).count() as f32;
    // Past the cap the area comes back as shots (`Game::strike_over`).
    let rate = fire_rate(gun, n(Card::Rate), rules);
    let pierce = n(Card::Pierce);
    let split = n(Card::Split) as i32;
    let all = casts(cards);
    // One bullet of cast `i`, landing: its strikes and kill bursts, and
    // what it carries out where it hits (`depth` stops a loop).
    #[allow(clippy::too_many_arguments)]
    fn bullet(i: usize, all: &[Cast], strike: f32, pierce: f32, split: i32, rules: &Rules, rr: &RoundRules, sc: f32, depth: usize) -> f32 {
        let Some(c) = all.get(i) else { return 0.0 };
        let shape = c.ammo.set.shape();
        let r = (strike * c.ammo.reach()).min(R_MAX) * shape.r;
        let hits = 1.0 + pierce + shape.pierce as f32;
        let kill: f32 = if c.payload == Card::Plain {
            // The plain bullet's pop (`rounds::plain_bursts`, 2026-11-22):
            // on the hit, it is the strike's own disc again, so only what
            // a Wide pop reaches past the strike counts; scattered, its
            // bursts sit off the hit; delayed (Delay, Thick), the slime
            // has flowed back, so all of it counts.
            let pop: f32 = plain_bursts(c, (0.0, 0.0), r, rr, sc).iter().filter(|b| b.kill).map(|b| area(b.r * sc)).sum();
            if c.scatter || c.trigger.is_some() { pop } else { (pop - area(r)).max(0.0) }
        } else {
            // A payload's kill burst on the hit covers the strike's own
            // disc, so only the ring past it is new slime (the review
            // lane, 2026-11-22: Blast and Bomb read about a strike too
            // high a hit); a scattered burst sits off the hit and counts
            // whole.
            bursts(c, (0.0, 0.0), rr, sc, 30.0)
                .iter()
                .filter(|b| b.kill)
                .map(|b| if b.at.0.hypot(b.at.1) < 1e-3 { (area(b.r * sc) - area(r)).max(0.0) } else { area(b.r * sc) })
                .sum()
        };
        // A needle lets a payload out at its first hit only and flies on
        // plain (`shots.rs`, `one_burst`); Pierce cards on another set
        // burst at every hit.
        let kill_hits = if c.ammo.set == super::bullets::Set::Needle && c.ammo.shell != super::bullets::Shell::Plain { 1.0 } else { hits };
        // The pieces a split bullet leaves at its last hit, a generation a
        // card, each `split_n` pieces at `split_r` of the radius.
        let mut pieces = 0.0;
        let (mut k, mut pr) = (1.0, r);
        for _ in 0..split {
            k *= rules.split_n.max(1) as f32;
            pr *= rules.split_r;
            pieces += k * area(pr);
        }
        let carried = if depth < 4 { c.child.map_or(0.0, |j| bullet(j as usize, all, strike, pierce, split, rules, rr, sc, depth + 1)) } else { 0.0 };
        shape.pellets.max(1) as f32 * (hits * area(r) + kill_hits * kill + pieces + carried)
    }
    let turn = cycle(&all);
    let per_shot = if turn.is_empty() {
        // No cast: the plain bullet.
        let plain = casts(&[Card::Plain]);
        bullet(0, &plain, strike, pierce, split, rules, rr, sc, 0)
    } else {
        turn.iter().map(|&id| volley(&all, id as usize - 1).into_iter().map(|j| bullet(j, &all, strike, pierce, split, rules, rr, sc, 0)).sum::<f32>()).sum::<f32>() / turn.len() as f32
    };
    rate * gun.shots.max(1) as f32 * per_shot / unit * DAMAGE
}

/// **The gun's stats as it fires with its cards** (the owner, 2026-11-22
/// 08:53Z: the shop was "supposed to make visual for speed range, etc",
/// not only DPS): what each card does to them is the shop's bars, a card
/// against the gun without it. The bullets' numbers are the mean over the
/// bullets the gun fires in turn (a card goes into every shot's average).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Profile {
    /// Damage a shot (all its bullets, pellets, pierces, pieces and kill
    /// bursts), in `estimate`'s unit: the rifle's plain shot is 10.
    pub damage: f32,
    /// Shots a second, a soldier (`Rate` cards and past-the-cap strike in).
    pub rate: f32,
    /// How far a shot flies, dish cells (`Range` cards and the bullets').
    pub range: f32,
    /// How fast a bullet flies, dish cells a second.
    pub speed: f32,
    /// The spread at the target, dish cells (smaller is truer).
    pub spread: f32,
    /// `estimate`: damage times rate.
    pub dps: f32,
}

/// `Profile` of `gun` with `cards`, as `estimate` reads them.
pub fn profile(gun: &Gun, cards: &[Card], rules: &Rules, rr: &RoundRules) -> Profile {
    let dps = estimate(gun, cards, rules, rr);
    let n = |k: Card| cards.iter().take(super::rounds::MAX_SLOTS).filter(|&&c| c == k).count() as f32;
    let rate = fire_rate(gun, n(Card::Rate), rules);
    // The bullets fired in turn, and those beside them (`volley`).
    let all = casts(cards);
    let fired: Vec<usize> = cycle(&all).iter().flat_map(|&id| volley(&all, id as usize - 1)).collect();
    let mean = |f: &dyn Fn(&Cast) -> f32| if fired.is_empty() { 1.0 } else { fired.iter().map(|&j| f(&all[j])).sum::<f32>() / fired.len() as f32 };
    Profile {
        damage: dps / rate,
        rate,
        range: gun.range * (1.0 + rules.range_card * n(Card::Range)) * mean(&|c| c.ammo.range()),
        speed: rules.bullet_speed * mean(&|c| c.ammo.speed()),
        spread: gun.spread * mean(&|c| c.ammo.spread()),
        dps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::bullets::{Ammo, Set, Shell};
    use crate::mould::guns::GunKind;

    #[test]
    #[ignore]
    fn print_numbers() {
        let rifle = GunKind::Rifle.average();
        for (name, cs) in [("rifle", vec![]), ("Rate", vec![Card::Rate]), ("Blast", vec![Card::Blast]), ("Bomb", vec![Card::Bomb]), ("Wide Blast", vec![Card::Wide, Card::Blast]), ("Pierce", vec![Card::Pierce]), ("Split", vec![Card::Split])] {
            println!("{name}: {:.1}", est(&rifle, &cs));
        }
        for k in [GunKind::Carbine, GunKind::Long, GunKind::Scatter] {
            let g = k.average();
            println!("{k:?}: {:.1} / Blast {:.1}", est(&g, &[]), est(&g, &[Card::Blast]));
        }
        let needle = |sh| Card::Ammo(Ammo { set: Set::Needle, ..Ammo::standard(sh) });
        println!("needle plain {:.1}, needle blast {:.1}", est(&rifle, &[needle(Shell::Plain)]), est(&rifle, &[needle(Shell::Blast)]));
    }

    #[test]
    fn a_needle_lets_its_payload_out_once() {
        // `shots.rs`, `one_burst`: three strikes but one blast; Pierce on
        // a standard blast bursts at both hits.
        let g = GunKind::Rifle.average();
        let blast = est(&g, &[Card::Blast]) - est(&g, &[]);
        let needle = |sh| Card::Ammo(Ammo { set: Set::Needle, ..Ammo::standard(sh) });
        let extra = est(&g, &[needle(Shell::Blast)]) - est(&g, &[needle(Shell::Plain)]);
        assert!((extra - blast).abs() < 0.05 * blast.max(1.0), "needle blast adds {extra}, a blast {blast}");
        let pierced = est(&g, &[Card::Pierce, Card::Blast]) - est(&g, &[Card::Pierce]);
        assert!((pierced - 2.0 * blast).abs() < 0.05 * blast.max(1.0), "pierced blast adds {pierced}");
    }

    fn est(gun: &Gun, cards: &[Card]) -> f32 {
        estimate(gun, cards, &Rules::default(), &RoundRules::default())
    }

    #[test]
    fn the_starting_rifle_is_ten() {
        let g = GunKind::Rifle.average();
        assert!((est(&g, &[]) - 10.0).abs() < 1e-3, "{}", est(&g, &[]));
        assert!((est(&g, &[Card::Plain]) - 10.0).abs() < 1e-3);
    }

    #[test]
    fn damage_cards_raise_it_and_others_leave_it() {
        let g = GunKind::Rifle.average();
        let base = est(&g, &[]);
        for c in [Card::Rate, Card::Pierce, Card::Split, Card::Blast] {
            assert!(est(&g, &[c]) > base * 1.2, "{c:?}: {}", est(&g, &[c]));
        }
        // A Bomb's kill (2.5 dish cells) sits inside the rifle's strike;
        // what it does is throw the slime round it, which is not counted
        // (the review lane, 2026-11-22). It never reads lower.
        assert!(est(&g, &[Card::Bomb]) >= base - 1e-3);
        for c in [Card::Quiet, Card::Range, Card::Seek, Card::Gas, Card::Flare] {
            assert!((est(&g, &[c]) - base).abs() < 1e-3, "{c:?}: {}", est(&g, &[c]));
        }
        // Wide works on a kill burst: a wide blast beats a blast.
        assert!(est(&g, &[Card::Wide, Card::Blast]) > est(&g, &[Card::Blast]));
        // And on the rifle's plain bullet (its pop, 2026-11-22), as do
        // Scatter, Delay and Thick.
        for c in [Card::Wide, Card::Scatter, Card::Delay, Card::Thick] {
            assert!(est(&g, &[c]) > base * 1.05, "{c:?} on the rifle: {}", est(&g, &[c]));
        }
    }

    #[test]
    fn faster_guns_and_more_bullets_read_higher() {
        assert!(est(&GunKind::Carbine.average(), &[]) > est(&GunKind::Rifle.average(), &[]));
        let mut one = GunKind::Scatter.average();
        let four = one;
        one.shots = 1;
        assert!((est(&four, &[]) - 4.0 * est(&one, &[])).abs() < 1e-3);
    }

    #[test]
    fn a_profile_moves_with_its_cards() {
        let g = GunKind::Rifle.average();
        let (r, rr) = (Rules::default(), RoundRules::default());
        let base = profile(&g, &[], &r, &rr);
        assert!((base.damage - 10.0).abs() < 1e-3 && (base.rate - 1.0).abs() < 1e-3 && (base.range - 24.0).abs() < 1e-3);
        // 15 ticks a second: a half-second gap is 7.5 ticks, rounded to 8.
        assert!((profile(&g, &[Card::Rate], &r, &rr).rate - 1.875).abs() < 1e-3);
        assert!(profile(&g, &[Card::Range], &r, &rr).range > base.range * 1.5);
        assert!(profile(&g, &[Card::Blast], &r, &rr).damage > base.damage * 2.0);
        assert!((profile(&g, &[Card::Rate], &r, &rr).damage - base.damage).abs() < 1e-3, "Rate fires faster, not harder");
    }

    #[test]
    fn heavier_shots_past_the_cap_still_count() {
        let g = GunKind::Rifle.average();
        let mut r = Rules::default();
        let base = estimate(&g, &[], &r, &RoundRules::default());
        r.strike *= 1.2f32.powi(4);
        assert!(estimate(&g, &[], &r, &RoundRules::default()) > base * 1.5);
    }
}
