# Handoff: roguelike run (2026-11-17)

The roguelike run is a separate mode in Larder World (the campaign is kept
and frozen): 8 generated planets, one slime per planet, a shop between
planets, dev mode. Full write-up: `Reports/roguelike-run-2026-11-14/README.md`
(sections "Run pacing", "Winning when the slime is gone", "The run's slime is
shape C"); wiki page `wiki/world/run.md`.

## State (all merged; no open branches or PRs from this area)

- Run, levels, shop, dev mode: #280. Shop with icons, two areas: #287, #294.
  Shop holds still when the bag fills: #316 (`src/world/start.rs` shop_plan).
- Pacing (#314): `run::clock(area)` scales the clock by planet area
  (900 s to 3330 s); `run::head_start(area)` lets the slime grow alone before
  landing on big planets (0 to 87 s); `Game::landed` makes the clock start on
  landing. Planet 6 is the breather planet.
- Win rule (#320): `run::WIN_SHARE` 0.02, the slime must fall to 2% of its
  peak (owner 03:02Z 11-15: "I keep winning before I've actually killed
  everything").
- The run's slime is shape C (25,000 walkers per unit, C's keys) (#336);
  each planet's size varies by seed (`run::size_roll`, 0.8 to 1.25) and grows
  with depth (owner 08:34Z: "It should vary and be more later on") (#339).
- 1070 frame p95 at 51f47b83: planet 1 4.6 ms, planet 5 5.9 ms, core world
  12.0 ms (365k walkers; 87 s head start takes 12 s to compute). All 60 fps.

## Shared with the slime-enemy thread

The slime-enemy work (`enemy_keys`, food-driven surges, PR #347 and after)
writes into `run.rs`, `args.rs`, `main.rs`, `menu.rs`. The enemy's
progression across planets is that area's; `level.walkers` already includes
the size roll, so anything scaled by it varies per seed.

## Pending owner picks / playtest

Nothing waits on a pick. Awaiting the owner's playtest of: pacing per
planet, the breather planet, the core world's landing pause (12 s), shop
prices, the 2% win rule.

## Next steps (ideas, not started)

Lost soldiers persisting between planets; mortar / artillery in the shop;
rescue planets; saving a run mid-way; precompute the core-world head start
during the shop so the landing pause goes away.
