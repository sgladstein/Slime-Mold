# Larder World: long-term roadmap (2026-11-11)

Copy of the roadmap doc, [Larder World: long-term roadmap](https://claude.ai/artifact/1N2NHKefAzKxUMZcr3ktjJ), which is the live version and is open for the owner's edits. It was written after the owner's rulings of 2026-11-11 14:01-14:09Z ([wiki/decisions.md](../wiki/decisions.md)). It is a plan only: no code and no runs.

**The game.** You land a squad in a large world. Energy crystals are spread across it, each holding a finite charge that it gives off as scent, and each dims as it drains. You and several mould species live on that charge, and every crystal runs dry, so you have to expand. The moulds also fight each other. The world persists, and a campaign may chain worlds together. Everything on screen is the simulation.

**Two openings to test.** In the *landing*, the world starts mostly mould. In the *race*, both sides start small and expand at once. The risk with the race: on the Bloomfront field skirmish, the 1:00 leader kept the lead 80-85% of the time ([rts-skirmish](rts-skirmish-2026-11-10.md)). A start becomes the default only when the early leader wins well under 80%, as measured by the player AI.

**The gap.** The GPU big dish (`src/mould/big.rs`, `src/bin/mouldgpu.rs`) already has species and a grazing field, and grazing with regrowth at 0 is already a crystal that runs out. What it lacks is growth and starvation: its walker count is fixed. That is the GPU plan's phase 4.

| Phase | What | Owner | Gate |
| --- | --- | --- | --- |
| W0 | Births, upkeep and starving per walker on the GPU | GPU thread | GPU matches CPU in CI; a fed mould grows and a starved one shrinks; cost quoted at 100k walkers |
| W1 | World with terrain fields and crystals; both starts | Larder thread | Look first; the mould follows crystals and thins as they run dry; crystal lifetimes as a spread |
| W2 | Larder's soldiers, fire, buying and bait in the world; extractors; camera and UI | Larder, Camera and UI threads | Replays reach the same hash; frame under 10 ms p95 on the 1070; plans beat idle and random |
| W3 | Two or three mould species fighting over crystals | Larder, creature threads | Plans that counter each other across species mixes (sign test, Holm); owner playtest |
| W4 | Persistent world: save and load, base, extractors and upgrades across sessions | Larder thread | The owner plays an hour and wants more |
| W5 | Campaign: a chain of worlds | to be chosen | Owner's call |

**Interfaces.**
- The game takes `Cmd`s and logs them for replay.
- The Camera and UI thread turns input into commands: a `world` module with `Camera`, `View` and `Controls`. Units carry a team id. Marks include squares for buildings. The View has a hook for an optional visibility mask.
- The Player AI thread plays only through `Cmd`s, sees what a player sees, and reports paired-seed censuses.
- Two soldier teams, from W2 on, are two command sources in the same state.

**Open questions:**
- World size. 1024 × 640 at 1.5 ms against 2048 × 1280 at 8-10 ms of GPU. Recommend starting at 1024 × 640, viewed zoomed in.
- How a world ends. Recommend: when its crystals are gone.
- Whether the two teams are allies or rivals.
- Fog of war, after W2's playtest.
- What carries between campaign worlds.

## Mould progression, easy to hard (owner agreed 14:25Z; it grows as the research threads find new behaviours; walls or weapons in either order)

| Tier | Mould | What it does (rule) | What beats it |
| --- | --- | --- | --- |
| 1 | Drifter | Slow grazing bodies that crawl and graze crystals (`graze_weight`) | Any squad; teaches aiming and extractors |
| 2 | Sprawler | Fast feathery fronts grab crystals early, thin tubes hold badly (`explorer_spread`) | Let it overreach, cut thin tubes; walls |
| 3 | Trunk | Thick looped cords that heal around a cut (dense start, slow respawn, W0 growth) | Starve it; heavy weapons |
| 4 | Hunter | Hungry mould smells and drains soldiers, eats bodies (Larder hunger) | Turrets, walls, bait elsewhere |
| 5 | Rival pair | Two species with own trails fighting over borders (`species_avoid`) | Lure one into the other |
| 6 | Converter | Converts rivals' walkers where it outnumbers them (`species_convert`, `species_cycle`) | Kill early, keep rivals apart |
| 7 | Mixes | Several of the above | Adapt the build to the mix |

Player side: walls (a terrain field the mould can't enter), upgrades, new weapons (M4's burner and long gun); each tool answers a tier. The early leader's win rate (80-85% on the field skirmish) is a per-mould difficulty stat, not a gate (owner 14:14Z).

## Independent reviews (14:40Z)

Three agents reviewed this plan at the owner's request: a skeptic, an expander and a technical feasibility check. Their prompts and reports are in [reviews/](larder-world-roadmap-2026-11-11/reviews/).
- The skeptic says the plan scales up a game that isn't fun yet. Finite crystals make a lean world, and a lean mould starved on its own in M2. Its kill test: finite crystals, idle against the plans, and whether the mould survives untouched.
- The expander says walls come first, since they are the only tool with a measured effect (6 of 12 lost games turned). It proposes Sporer and Sleeper tiers, which meet the re-test conditions of M3's rejected spores and crust, and a player-owned mould species.
- Feasibility: Larder runs only on the small dish, the world size is tied to the dish's shape, births are the hard part on the GPU, and a full-scale CPU census would take about 90 hours. The 8-10 ms figure was 1M walkers on 1024 × 640, not 2048 × 1280; its estimate for 2048 × 1280 is about 3.7 ms a frame.

Proposed order, on the owner's card: build the whole game on a 512 × 320 CPU world first, then port it to the GPU.

## W1a: crystals and terrain on the big dish (CPU), 2026-11-11

`BigParams` gained the following, all off by default:
- `crystal_charge` and `crystal_eat`: finite charge in every fine cell of a pile. The n walkers on a cell take equal shares. The pile's scent is multiplied by the share of its charge left, and the picture draws the charge as a cyan glow.
- `terrain_rock`, `terrain_wet`, `wet_step` and `terrain_blob`: keyed value noise on the dish grid, the same map at every scale. Rock is not agar. A step on wet ground is × 0.5.
- `rect`: a rectangular world.

Test `crystals_run_dry_and_terrain_pairs`. The walker count is still fixed (growth is W0), so the mould can't thin yet.

First look, 512 × 320, 20k walkers, 23 crystals, seed 1 ([5 s](larder-world-roadmap-2026-11-11/w1a/big-w1-s1-005s.png), [60 s](larder-world-roadmap-2026-11-11/w1a/big-w1-s1-060s.png)). Crystal charge left at 20 s was 0.566 on seed 1 and 0.497 on seed 2, with eat 0.02 against a charge of 1 (too fast; to tune). Cost: 8.00 and 8.20 ms a tick with crystals and terrain on, against 7.59 and 8.34 off (2 seeds, 4 threads), which is within noise.

**Owner, 14:54-14:55Z:** don't scale back. Test crystals that run out against crystals that refill. As a first thought, to be settled by playtest, run-out crystals go with the race and refilling ones with the landing. Walls first, the Sporer and Sleeper tiers and a player species are agreed. Use a CPU world for small tests and the 1070 for GPU work, in parallel. Added `crystal_regrow`: each fine step a crystal cell regains this share of what it is missing (0 = runs dry).

## First wall test (W1a, fixed walker count), 2026-11-11

`examples/worldwall.rs`: 512 × 320, 20k walkers, 23 finite crystals (charge 1, eat 0.005), the mould starting as a blob on the home crystal. At 5 s one 60 × 3 dish-cell wall goes in. The arms are `none`, `road` (across the line to the three nearest crystals, 6 cells ahead of the mould's front there) and `random` (keyed position and angle). The deciding measure, named before the run, was the charge drained from the non-home crystals by 30 s, paired by seed.

- Run 1 ([raw](larder-world-roadmap-2026-11-11/walls/run1-wall-inside-body.txt)) put the road wall 12 cells from home, which is inside the 20k-walker blob. That was a placement error and is not counted.
- Run 2 ([raw](larder-world-roadmap-2026-11-11/walls/run2-wall-at-front.txt)): the road wall drained less than none on 5 of 8 seeds (two-sided sign test p 0.73) and less than random on 6 of 8 (p 0.29). The differences run from -0.060 to +0.041 of the charge. **No effect.**

Why: the walker count is fixed, so the mould's total eating rate is set by how many walkers stand on crystals, not by which crystals it reaches. A 60-cell wall is also simply walked around ([seed 1](larder-world-roadmap-2026-11-11/walls/run2-s1-road.png)). This says nothing yet about walls in play. It says the test needs W0's growth, where cutting the mould off from crystals should shrink it. Rerun when W0 merges, with the mould's walker count at 30 and 60 s as the measure.

## Walls the mould can wear down, and crystals that drain as a whole, 2026-11-11

Page: [Larder Walls and Crystals](https://claude.ai/artifact/ChBBfKkuMYaggBvXCKrxtN) (source `larder-world-roadmap-2026-11-11/page-walls.html`).

**Owner, 15:24Z:** "There should be a way for slime to damage walls or grow over it (but it has to be a significant slow down), otherwise walls can just fully surround crystals and the slime can do nothing."

**Mechanism** (`BigParams::wall_hp`, `wall_bite`): each wall cell has health. Each fine step, every walker that a wall cell blocks takes `wall_bite` from the cell it tried to enter (summed per cell and applied after the pass, so walker order does not matter). At 0 the cell is rubble: agar again, wet (step × `wet_step`), open to scent. The picture draws a wall's brightness from its health. A unit test checks that a weak wall across the home blob breaks within 60 ticks and that a wall without health never does.

All runs below: `examples/worldwall.rs`, 512 × 320 (scale 2), 20k walkers, 23 crystals that run dry (charge 1, eat 0.005), 40 s.

- **Ring round one crystal** (at 5 s, round the nearest crystal the mould has not reached; picked the same way in every arm). Measure named first: the second the mould first eats from it. With walls that stop scent ([raw](larder-world-roadmap-2026-11-11/walls/run3-ring-sealed-scent.txt)) and with walls that leak 0.3 of their scent ([raw](larder-world-roadmap-2026-11-11/walls/run4-ring-leaky-scent.txt)), the mould never ate a ringed crystal and broke no cell at health 30 or 300 (5 valid seeds; seed 2 had no unreached crystal). Without the ring it first ate there at 5.8-8.6 s on 3 of the 5. Counting bites with health set too high to break ([raw](larder-world-roadmap-2026-11-11/walls/run5-ring-bites.txt)): 0 to 76 bites on the whole ring in 35 s. Today's mould has a fixed walker count, forms a network and does not press anywhere.
- **Cage round the whole home blob** at the start (its farthest walker plus 6 dish cells), the hardest the mould can press ([raw](larder-world-roadmap-2026-11-11/walls/run7-cage.txt)). A first try also built a random wall at 5 s in this arm, a harness bug, and is not counted ([raw](larder-world-roadmap-2026-11-11/walls/run6-cage-bug-extra-wall.txt)).

| Wall health | First cell broken (seeds 1, 2, 3) | Walkers outside at 20 s | at 40 s |
|---|---|---|---|
| 0 | never | 0, 0, 0 | 0, 0, 0 |
| 400 | 4.7, 4.3, 3.4 s | 6, 217, 660 | 2,023, 2,927, 2,319 |
| 1,600 | 15.9, 14.8, 13.6 s | 0, 0, 0 | 0, 0, 0 |
| 6,400 | never | 0, 0, 0 | 0, 0, 0 |

**Default `wall_hp` 400**, a first number for play: the whole mould pressing breaks out in 20-30 s, while a tube passing a wall never breaks it. Recalibrate when the mould grows (W0): a growing mould presses wherever it is crowded. **`wall_scent`** (scent leaking through a wall) is built and off: it changed nothing measurable here (seed 1: 12 bites against 9 sealed), and on the fine scent grid a wall 4 scent cells thick passes very little.

**Crystal flow** (`BigParams::crystal_flow`). The GPU thread's first growth run on crystals that run dry (one seed, 15:27Z) saw the mould starve with 61% of the charge left, because it ate only the cells its tubes crossed. Now each fine step every cell of a crystal moves `crystal_flow` of the way to the crystal's mean charge: the total is kept, a tube touching one edge drains the whole disc, and the whole disc dims together. Measure named first: the share of the non-home crystals' charge eaten by 40 s, paired by seed ([raw](larder-world-roadmap-2026-11-11/crystals/flow-runs.txt)).

| Flow | Share eaten, mean | Seeds | Higher than no flow | Sign test p |
|---|---|---|---|---|
| 0 | 0.476 | 12 | - | - |
| 0.01 | 0.829 | 12 | 12 of 12 | 0.0005 (survives Holm over 3) |
| 0.05 | 0.749 | 6 | 5 of 6 | 0.22 |
| 0.2 | 0.800 | 6 | 6 of 6 | 0.031 (not after Holm) |

**Default `crystal_flow` 0.01.** Cost within noise: 7.8 ms a tick with and without (bigdish, 4 threads, 10 s). Pictures, seed 3 at 40 s: [no flow](larder-world-roadmap-2026-11-11/crystals/flow-off-s3-40s.png), [flow 0.01](larder-world-roadmap-2026-11-11/crystals/flow-on-s3-40s.png). Without growth this shows the mould can now reach the food; whether it then keeps growing is the GPU thread's rerun.

**With growth** (the GPU thread's CPU growth on top of f292952, one seed, 16:03Z; raw in its W0 PR, `Reports/growth-2026-11-11/raw/crystals.txt`): same world, 5,000 walkers to start in 50,000 slots, upkeep 0.001, crystals that run dry. Flow 0: alive 1,454 / 934 / 106 / 1 at 5 / 10 / 20 / 60 s, 61.2% of the charge left. Flow 0.01: alive 2,297 / 2,400 / 515 / 20 at 5 / 10 / 20 / 60 s, 0.8% left. The mould now eats 99% of the crystals instead of 39% and peaks about 1.65 times higher, then starves at about 20 s because the food is gone. This world's whole charge feeds about 2,000 walkers for 15-20 s, so for "enough resources for a long game" the charge (or regrow) has to be sized against upkeep times the mould size wanted. That sizing is the first W1 job once growth merges.

## Crystal sizing with growth, 2026-11-11

The owner, 14:54Z: with crystals that run out the mould starts in one spot, there should be enough for a long game, and "if you do nothing, the slime should get much stronger than you and kill you". So: how much charge keeps an idle player's mould alive and growing for minutes? Scratch build: the GPU thread's CPU growth (65d914f) plus this branch's pile-total fix below. 512 × 320, 23 crystals that run dry, flow 0.01, 5,000 walkers to start in 50,000 slots, upkeep 0.001, no player. Raw: [after the fix](larder-world-roadmap-2026-11-11/growth-sizing/runs-after-fix.txt), [before](larder-world-roadmap-2026-11-11/growth-sizing/runs-before-fix.txt).

| Charge a cell | Alive at 60 / 150 / 300 s | Charge left at 300 s | Seeds |
|---|---|---|---|
| 1 | 39 / 0 / 0 (dead by 120 s) | - | 1 |
| 8 | 3,906 / 0 / 0 (dead by 150 s) | 20% (piles it never reached again) | 1 |
| 64 | 13,599 / 12,539 / 141 (peak 19,327 at 90 s) | 0% by 210 s | 1 |
| 512 | 13,981 / 18,907 / 21,015 and 18,221 / 22,075 / 24,106 | 65%, 55% | 2 |
| 4,096 | 15,475 / 17,329 / 18,200 | 96% | 1 |

- **Charge sets how long the mould lasts, not how big it gets.** On this world the mould reaches all 23 crystals by 30-90 s and levels off at 18,000-24,000 walkers whatever the charge. At 64 it booms and dies within 5 minutes. At 512 it is still growing slowly at 5 minutes with half the charge left, so about 10-12 minutes of food. Pictures: [64 at 150 s](larder-world-roadmap-2026-11-11/growth-sizing/c64-s1-150s.png) (crystals drained, thin net, shrinking), [512 at 300 s](larder-world-roadmap-2026-11-11/growth-sizing/c512-s1-300s.png).
- **The mould hoards.** Mean walker energy climbs without limit at high charge (30 at 300 s at charge 512 and 4,096). About 40% of what it eats sits in walkers that cannot divide, because no cell beside them is free, while walkers born at the edges get half of a small surplus and starve within 2 s (about a million births and a million starvings in 5 minutes). The stored energy is a reserve: a walker holding 30 lives about 2 minutes without food. Whether that reserve should turn into growth (an energy cap, or sharing along the tubes) is a growth-rule question for the GPU thread.
- **First number for the race: charge 512** on a world this size (a 10-12 minute mould if nobody fights it). The size of the world, not the charge, sets how strong the idle mould gets.

**Bug found and fixed (a3aaccb):** pile totals were a running f32 sum. At charge 512 a pile holds about 180,000, and a 0.005 bite is below that number's f32 step (0.016), so the totals stood still: `crystal_left` read 1.000 at 180 s and the piles kept calling at full scent while the mould ate. They are now summed from the cells in f64 each step. Runs at charge 1, all the earlier measurements, were not affected (a pile of about 350 has a step of 3e-5). The charge 512 run before the fix was stopped and is not counted.
