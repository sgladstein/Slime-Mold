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
//!   Scatter included, at each slime hit;
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
use super::rounds::{bursts, casts, cycle, volley, Card, Cast, RoundRules};
use super::squad::R_MAX;

/// The dish scale the game's planets run at (`larder-world`'s `scale=2`):
/// fine cells a dish cell, which sets where the strike cap bites.
pub const SCALE: f32 = 2.0;

/// Area of a circle of radius `r`, without the pi (it cancels).
fn area(r: f32) -> f32 {
    r * r
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
    // Past the cap the area comes back as shots (`Game::strike_over`).
    let over = (want / R_MAX).max(1.0).powi(2);
    let n = |k: Card| cards.iter().take(super::rounds::MAX_SLOTS).filter(|&&c| c == k).count() as f32;
    let rate = gun.rate * (1.0 + rules.rate_card * n(Card::Rate)).max(0.01) * over;
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
        let kill: f32 = bursts(c, (0.0, 0.0), rr, sc, 30.0).iter().filter(|b| b.kill).map(|b| area(b.r * sc)).sum();
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
        shape.pellets.max(1) as f32 * (hits * (area(r) + kill) + pieces + carried)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::guns::GunKind;

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
        for c in [Card::Rate, Card::Pierce, Card::Split, Card::Blast, Card::Bomb] {
            assert!(est(&g, &[c]) > base * 1.2, "{c:?}: {}", est(&g, &[c]));
        }
        for c in [Card::Quiet, Card::Range, Card::Seek, Card::Gas, Card::Flare] {
            assert!((est(&g, &[c]) - base).abs() < 1e-3, "{c:?}: {}", est(&g, &[c]));
        }
        // Wide works on a kill burst: a wide blast beats a blast.
        assert!(est(&g, &[Card::Wide, Card::Blast]) > est(&g, &[Card::Blast]));
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
    fn heavier_shots_past_the_cap_still_count() {
        let g = GunKind::Rifle.average();
        let mut r = Rules::default();
        let base = estimate(&g, &[], &r, &RoundRules::default());
        r.strike *= 1.2f32.powi(4);
        assert!(estimate(&g, &[], &r, &RoundRules::default()) > base * 1.5);
    }
}
