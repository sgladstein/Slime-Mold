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
| added | Sporer | Fed walkers fly as spores to new crystals; struck, it scatters (`spore_rate`) | Starve it; don't hit it |
| added | Sleeper | A dormant crust that wakes in a wave and rushes a wound, then is deaf for 5 s (`sleep_rate`) | Bait it awake, then strike |
| added | Swarmer | Walks toward where it is hit, into the fire (`alarm_weight`) | A kill zone |
| 7 | Mixes | Several of the above | Adapt the build to the mix |

The three added tiers come from the tiers thread ([mould-tiers-2026-11-11.md](mould-tiers-2026-11-11.md), PR #70); each beat its control 8/8. Their place on the ladder is for play to settle; W2b's rules take a tier once its `Tier::apply` lands.

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

## Walls with a growing mould, 2026-11-11

Scratch build with the GPU thread's growth (raw: [road, random, ring at 60 s](larder-world-roadmap-2026-11-11/walls/run8-growth.txt), [leaky rings at 90 s](larder-world-roadmap-2026-11-11/walls/run9-growth-ring-leak.txt)). `worldwall`, 512 × 320, charge 512, 5,000 walkers to start in 50,000 slots, upkeep 0.001, wall health 400, one wall at 5 s, 8 seeds. Deciding measure named first: walkers alive at 60 s, paired by seed.

| Arm | Alive at 60 s, mean | Fewer than no wall | Sign test p |
|---|---|---|---|
| none | 16,075 | - | - |
| road (60 × 3, ahead of the front toward the nearest crystals) | 14,674 | 7 of 8 | 0.070 |
| random (same wall, keyed place) | 14,537 | 5 of 8 | 0.73 |
| ring round the nearest unreached crystal | 13,859 | 6 of 8 | 0.29 |

Road against random: fewer on 5 of 8 (p 0.73). None survive Holm over the four. So with growth, any wall costs the mould roughly a tenth of its size at 60 s, and a wall on its road is no better than one dropped at random. The growing mould does chew through: road walls were broken in places on 4 of 8 seeds (43-196 cells), random walls on 6 of 8. Picture, seed 1 at 60 s, none / road / random / ring: [growth-s1-60s](larder-world-roadmap-2026-11-11/walls/growth-s1-60s-none-road-random-ring.png).

**A ringed crystal is ignored while walls stop scent.** The mould's tubes pass right beside the ring and never press on it: 0 bites on 6 of 8 seeds in 60 s. That is the owner's worry (15:24Z) in another form: a sealed crystal is safe because it is silent. With walls leaking scent, measured at 90 s with the measure named first (bites on the ring, and when the crystal inside is first eaten):

| `wall_scent` | Ring bitten | Ring broken through in places | Crystal inside eaten |
|---|---|---|---|
| 0 (sealed) | 2 of 8 | 1 of 8 | 0 of 8 |
| 0.5 | 4 of 8 | 3 of 8 | 0 of 8 |
| 0.9 | 7 of 8 | 6 of 8 | 2 of 8 (86 s) |

Bites at 0.9 against sealed: more on 6 of 7 untied seeds (p 0.125). Not significant at 8 seeds, but the trend is steady and it is what the owner asked for, so **`wall_scent` now defaults to 0.9**: a walled crystal still calls faintly, the mould gathers and gnaws, and it gets in after about a minute and a half of pressure.

## Owner, 16:48Z (graphics thread)

"Gun fire should hit the first slime in its path" and "The crystals should have hexagonal outline, not circle." Crystals are now pointy-top hexagons of circumradius `pile_r` in the simulation itself (`BigDish::in_pile`): the crystal's cells, its scent and where it counts as reached, about 0.83 of the disc's area, so every crystal holds about 17% less charge than in the runs above. Piles with crystals off keep their disc. The graphics thread writes `world::look::first_hit` (march the shot's line, stop at the first mould); the Larder world's firing rule will use it.

## W1b: crystals, walls and terrain on the GPU engine, 2026-11-11

Branch `claude/project-thread-u9z06g` (7766f32; W0's later merges of main, with #70's mould tiers, came in cleanly at e1e321f and f8f4eb9, and all 16 mouldgpu tests still pass), stacked on the GPU thread's W0 (growth on `gpu::Engine`, 55778bc), which is stacked on #63 (the engine in the library). The merge order agreed with the graphics thread (18:58Z): #63, W0, W1b, then #69 rebased onto it (its shot hits and look move to bindings 42 and 43, and its `Engine::block` becomes a call to `Engine::build_wall`).

**What runs on the GPU now**, each the CPU big dish's rule from W1a (PR #64):

- **Crystals.** A charge a cell (`charge`), and the crystals' cells listed pile by pile (`pcell`, `piles`). The `eat` pass gives the n walkers on a crystal cell equal shares, `min(crystal_eat, c / n)` each, floored at 0, and keeps each cell's share (`share`), which the growth pass `life` eats directly (it replaces W0's per-walker `took` buffer and saves a pass). `pilesum` sums each pile in one workgroup (an f32 tree; the CPU sums in f64, and the two agree to about 3e-10 of charge a tick), `flow` moves charge toward its pile's mean (`crystal_flow`) and regrows it (`crystal_regrow`), and a crystal's scent is scaled by its share left. Footprints are the hexagons of `BigDish::in_pile`.
- **Terrain.** Rock is not agar; on wet ground the step is multiplied by `wet_step`.
- **Walls.** One `ground` word a cell holds the pile index, WET, WALL, SCENT_WALL and ROCK bits. A walker that a wall blocks bites the wall cell it aimed at (`bites`, an atomic count a cell); the `walls` pass takes `wall_bite` of health a bite, and a cell at 0 turns to wet rubble open to walkers and scent. Walls pass on `wall_scent` of the scent spread into them. `Engine::build_wall(ax, ay, bx, by, width)` lays a capsule wall on the GPU: it clears the trail and scent under it and moves each walker it covers to the nearest free cell, in the CPU's ring order and under the crowd limit, in rounds until no walker moves (four fixed rounds left 24 of 178 inside).
- **Order in a tick**, as the CPU's: walk, walls, eat (then pilesum, flow), pilesum, growth (life, breed, born), convert, then trail and scent.

**Measure, named first.** One GPU tick against one `BigDish::tick` from the same CPU dish after 60 ticks of warm-up, on nine world dishes chosen so that every rule acts in the tick, and a wall built on the GPU against `BigDish::build_wall` on three dishes. Then 17 planted bugs, one at a time, each of which the test must fail. On lavapipe (`MOULDGPU_TEST_GPU=1 cargo test --release --bin mouldgpu -- world wall`); raw output: [twin and bugs](larder-world-roadmap-2026-11-11/w1b/twin-and-bugs.txt).

| Dish (25,000 walkers unless said) | Walkers off | Trail L1 | Scent L1 | Charge L1 | Bites, broken (CPU / GPU) |
|---|---|---|---|---|---|
| crystals (charge 512, 23 piles) | 0 | 0 | 1.2e-9 | 2.3e-10 | - |
| crystals at 1, no crowd limit, regrowing | 0 | 0 | 1.9e-9 | 3.5e-10 | - |
| crystals at 1, no flow, no crowd limit | 0 | 0 | 2.6e-10 | 0 | - |
| rock and wet ground | 0 | 0 | 1.2e-10 | 2.0e-10 | - |
| walls that break (health 3), leaking scent | 0 | 0 | 8.8e-10 | 1.5e-10 | 11, 4 / 11, 4 |
| walls that never break, sealed | 0 | 0 | 1.6e-9 | 3.2e-10 | 0, 0 / 4, 0 |
| walls under a crowd (health 1,000), no limit | 0 | 0 | 2.7e-9 | 3.1e-10 | 11, 0 / 11, 0 |
| a cage round a packed start (5,000) | 0 | 0 | 0 | 0 | 53, 0 / 53, 0 |
| growth on crystals (15,000 slots) | 0 | 0 | 9.2e-10 | 3.1e-10 | 1, 0 / 1, 0 |

Health, energy, grass, each cell's take, and the agar and ground bits match exactly on every dish. The one difference is by design: walls that never break keep no health on the CPU, so it counts no bites; the GPU counts them on infinite health, which changes nothing. A wall built on the GPU covers the same 812 cells as the CPU's, moves all 178 covered walkers (250 with no crowd limit) out, loses none, and leaves no cell over the limit.

**Planted bugs**: all 17 caught. Two were missed at first and the test grew to catch them: charge that goes negative when eaten (caught once a no-flow dish at charge 1 drains cells to 0, and any negative charge fails), and one bite a cell however many walkers hit it (caught once the cage dish puts 53 bites into a tick).

| # | Bug | Caught by |
|---|---|---|
| 0 | eat share not divided by the walkers on a cell | crystals at 1, no limit, regrowing: 753 takes, 0 agar, 0 ground off |
| 1 | eaten charge not floored at 0 | crystals at 1, no flow, no limit: charge L1 inf, health L1 0.00e0 |
| 2 | flow away from the mean | crystals: charge L1 9.48e-6, health L1 0.00e0 |
| 3 | no regrowing | crystals at 1, no limit, regrowing: trail L1 0.00e0, scent L1 3.18e-5 |
| 4 | pile sums count cells a wall took | walls that break, leaking scent: trail L1 0.00e0, scent L1 3.08e-4 |
| 5 | crystal scent not scaled by the share left | crystals: trail L1 0.00e0, scent L1 2.83e-5 |
| 6 | wet ground not slowing | rock and wet ground: 6345 of 25000 walkers off |
| 7 | every blocked walker bites | walls that break, leaking scent: bites and cells broken |
| 8 | one bite a cell however many hit it | a cage round a packed start: charge L1 0.00e0, health L1 4.16e-6 |
| 9 | rubble not wet | walls that break, leaking scent: 0 takes, 0 agar, 4 ground off |
| 10 | walls keep all the scent spread into them | walls that break, leaking scent: trail L1 0.00e0, scent L1 4.00e-4 |
| 11 | scent does not spread through leaky walls | walls that break, leaking scent: trail L1 0.00e0, scent L1 4.08e-3 |
| 12 | a new wall keeps its trail | one a cell: 0 agar, 0 ground, 812 cells left on |
| 13 | evicted walkers ignore the crowd limit | one a cell: 0 counts off, 48 crowded |
| 14 | growth eats the take of the cell beside | growth on crystals: energy L1 2.51e-2, grass 0.00e0 |
| 15 | crystal footprint a disc, not a hexagon | crystals: trail L1 0.00e0, scent L1 5.85e-3 |
| 16 | eating after growth | growth on crystals: energy L1 2.99e-2, grass 0.00e0 |

**Frame cost.** On the cloud's software GPU (lavapipe on 4 cores, so relative only), 100,000 walkers, 23 crystals, 4 s, two runs each ([raw](larder-world-roadmap-2026-11-11/w1b/timing-lavapipe.txt)). The world's four passes (walls, eat, two pile sums, flow) take 0.73-0.82 ms a tick at 512 × 320 and 1.56-1.80 ms at 1024 × 640, against 6.0-6.9 and 14.0-17.8 ms for the base passes (sense, walk, trail, scent) in the same runs: **about 10-12% more GPU time a tick** at either size. The median frame (two ticks and the drawing) went from 14.1-14.5 to 15.2-16.0 ms at 512 × 320, and from 35.8-35.9 to 37.4-39.6 ms at 1024 × 640. The walls pass visits every cell and is the largest of the four at 1024 × 640 (0.68-0.82 ms). A list of wall cells would cut it, if the 1070 shows that it matters.

**On the GTX 1070** (Scott-Desktop, 4a1f42f, 10 s runs, three runs each, the same world; [raw](larder-world-roadmap-2026-11-11/w1b/timing-1070.txt)). At 1024 × 640 the median frame goes from 1.73 ms to 2.11 ms (all three runs within 0.01 ms), and the GPU time of the two ticks from 1.50-1.51 to 1.75-1.76 ms, **+0.25 ms a frame, about 16%**. The world's own passes take 0.042 ms a tick (walls 0.018, eat 0.007, the two pile sums 0.013, flow 0.004). The rest is in the base passes that now read the ground: sense +0.03, scent +0.035 and walk +0.015 ms a tick. At 512 × 320 the frame goes from 0.58-0.63 to 0.80-0.81 ms. The walls pass is 0.018 ms, so a list of wall cells is not worth building. With growth as well, the frame is 2.10-2.11 ms at 1024 × 640, with 54,873 walkers alive at 10 s. Everything is far under the 10 ms bar.

**Picture**, 1024 × 640 (the W2 world), 100,000 walkers, 23 crystals at charge 512, rock 0.2, wet 0.2, two walls laid at 1 s and 2 s, at 4 s, drawn by the engine's own draw pass: [world](larder-world-roadmap-2026-11-11/w1b/world-1024x640-4s.jpg), and the same seed without the world fields: [base](larder-world-roadmap-2026-11-11/w1b/base-1024x640-4s.jpg). The crystals are hexagons, cyan by charge; the walls are grey by health; dark patches are rock. By 4 s the walls had taken 6,857 bites and none had broken (health 400). The mould runs its tubes between the crystals and around the rock, and nothing in the picture is drawn on top: it is the engine's fields.

## W2 plan: Larder in the world, 2026-11-11

**Goal.** A native GPU build the owner plays on the GTX 1070 (owner, 00:30Z: the real game is a native GPU app): a squad against a growing mould on the 1024 × 640 world, with crystals that run out, rock and wet ground, and walls, seen through the camera and UI of #63 and drawn by the look of #69. The mould starts on one crystal and grows (the race; owner, 14:54Z: "if you do nothing, the slime should get much stronger than you and kill you").

**What carries over from M3** (`src/mould/larder.rs`): `Rules`, the orders (hold, move, clear, burn, attack), the stances, `Cmd` and its replay log, the squad's turn (walk, defend, clear, burn), buying soldiers, turrets and bait. The controls of #63 were written for these orders.

**What changes in the world:**
- **Food is crystal charge, for both sides.** A clear order on a crystal harvests its charge into the soldier's carry, which it walks home to the larder. Charge the squad takes is charge the mould can't eat: the race is over the same crystals.
- **Fire stops at the first mould** (owner, 16:48Z). The shot's line from the soldier toward its target stops at the first cell that looks like slime, trail thickness 26/255 or more on a fresh read (the graphics thread, 18:57Z: a stricter test let 98 of 120 shots fly through visible slime), and strikes a disc there: it kills the walkers and clears the trail (`BigDish::strike`, #70; with growth the slot is freed).
- **The mould attacks as in M3**: soldiers smell like a small crystal, walkers within `touch` drain health, and a dead soldier becomes a small crystal.
- **Walls are a squad tool**, priced by the cell and built with `Engine::build_wall`; the mould wears them down (W1a).
- **The core** is the mould's home crystal; holding it clear stops births.

**Engine hooks**, each a GPU pass with a CPU twin and a one-tick twin test with planted bugs, as in W1b:
1. `squad`: one workgroup a unit (64 at most). Walkers within `touch` (summing `occ`), the densest mould within range, and harvest from the crystal cells under it. Read back each frame: a few hundred bytes.
2. `smell`: scent added on each soldier's disc before the scent pass.
3. `shots`: each shot marches its line on the trail and strikes at the first slime cell, in the same tick, so there is no 2.6 MB readback of the world. The hit points come back for the look's tracers.
4. `strike`: kill walkers and clear trail in discs.
5. Bodies: spare crystal slots reserved at the start and filled when a soldier dies.

**Game layer**, `src/mould/larder_world.rs`, no window. The rules run over a small `World` trait with two sides: the CPU big dish, for tests, CI and small censuses, and `gpu::Engine`, for play and censuses on the 1070. A replay (seed plus command log) reaches the same hash on one machine.

**Window**, `src/bin/larderworld.rs`: `Camera`, `View`, `Controls` and `Marks` from #63, the look from #69, and a panel with the bank, the squad, the crystals left, the mould's size and the time.

| Step | What | Gate |
|---|---|---|
| W2a | Engine hooks (squad, smell, shots, strike, bodies) | GPU matches CPU in a one-tick twin test; every planted bug caught; frame cost on the 1070 |
| W2b | Game layer, rules over `World` | Idle and random hands lose to the mould; a plan (harvest and defend) beats both on paired seeds (sign test, Holm) |
| W2c | Window | The owner plays it; frame under 10 ms p95 on the 1070 |

W2b can start now on the CPU dish. W2a's GPU passes follow W1b's merge, and W2c follows #69.

## W2a: the squad's hooks into the world, 2026-11-11

**What runs.** `src/mould/squad.rs` holds the CPU twin and the buffer layout. Six GPU passes in `src/gpu/engine.wgsl` do the same work (shots, strike, cut, smell, units, harvest), driven by `Engine::set_squad` and `Engine::squad_out`. A frame takes up to 64 units and 256 shots.
- At the frame's start, each shot walks its line every half cell. It stops at the first cell that looks like slime: trail at or above `slime_trail(knee)`, the trail whose look is 26/255. A cell that is not agar (rock or wall) stops it with no hit.
- At the hit it strikes a disc of radius 7 or less. Living walkers in the disc die as starving ones do, counted for the first shot that covers them, and the trail there is cleared.
- Each unit lays smell on the agar cells of its disc.
- Then the ticks run.
- At the frame's end, each unit reads how many walkers touch it and finds the densest 4 × 4 block in its sight (a tie goes to the first block in row order). Then units harvest crystal charge in their discs, in unit order, so a second unit on a patch gets what the first left.
- The CPU computes the disc tests and the shot paths in integers, so both sides pick the same cells.
- Bodies (hook 5 of the plan) move to W2b. A dead soldier's crystal is a spare pile the game fills through the crystal fields W1b already has, so it needs no pass.

**Measure, named first.** One frame (shots, strike, smell, one tick, reads, harvest) on the GPU against `squad::frame` on the CPU from the same dish, after 60 ticks of growth with a wall. The squad's outputs must be equal exactly, and the world fields equal up to float sums. Then planted bugs, one at a time, each of which the test must fail. The test is `MOULDGPU_TEST_GPU=1 cargo test --release --bin mouldgpu -- squad`, run on lavapipe; it runs in CI.

**Twin.** 16 shots: 15 hit, 19 walkers killed, 1 stopped by a wall. The touch counts and aims of 15 units are equal. The harvest is equal: 3,000 and 1,606.99 for two units on one patch, then 100 for each of five others. Every slot's life is equal, no walker is off, and the trail is exact; scent L1 is 1.4e-9 and charge L1 is 2.3e-10.

**Planted bugs** (each alone; raw: [round 1](larder-world-roadmap-2026-11-11/w2a/bugs-round1-87ebf13.txt), [round 2](larder-world-roadmap-2026-11-11/w2a/bugs-round2-efe9337.txt), [round 3](larder-world-roadmap-2026-11-11/w2a/bugs-round3-3957496.txt), [script](larder-world-roadmap-2026-11-11/w2a/plant-bugs.py)).
- **Round 1** caught 10 of 16. It missed:
  - a walker counted for every shot that strikes it, because no two shot discs overlapped;
  - empty slots struck, because no struck disc held a dead slot;
  - smell laid on walls and rock, because no unit stood near the wall;
  - aim tested at a block's corner, because no unit's sight was small enough to matter.
  - Two of the round's bugs were faulty. "Touch counted over the sight" changed only the loop bound, which the disc test inside still filtered, so it changed nothing; it was replaced. "A strike one cell wider" failed only because the shader did not compile, so it tested nothing; it was fixed.
- **The test then gained** a second copy of one shot, a shot from the cell of a slot that starved inside the mould, a unit with smell radius 4 standing on the first wall cell a shot meets, and six units with sights from 2.5 to 7.5.
- **Round 3, at 3957496, caught all 16.** Fourteen fail on an output or field. "Strikes leave the trail" leaves 29 walkers off. "Smell on walls" gives scent L1 1.6e-5 against the 1e-6 bound. "A walker counted for every shot" makes the cell's count wrap below zero, and the test then hangs instead of failing (killed after 120 s; it normally takes half a second).

**Frame cost on the GTX 1070** (the same runs as W1b's, [raw](larder-world-roadmap-2026-11-11/w1b/timing-1070.txt)). With the world and growth, `fire=N` sends N units a frame that cycle over the crystals, each shooting at the densest mould it saw the frame before. It also reads the squad's outputs back every frame. This is a load test, not play: with 40 units at 512 × 320 only 57 walkers were left at 10 s. Median frame in ms, three runs each:

| | 512 × 320 | 1024 × 640 |
|---|---|---|
| World and growth, no squad | 0.97-0.98 | 2.10-2.11 |
| 8 units a frame | 2.01 | 3.14-3.18 |
| 40 units a frame | 2.35-2.36 | 3.56-3.58 |

The worst frame with 40 units at 1024 × 640 was 5.2-5.4 ms. The squad's frame-end passes (units, harvest) show in the GPU's drawing span. They take 0.73 ms with 8 units and 0.81-0.85 ms with 40, at either size. The cost hardly depends on the unit count because the units pass gives each unit one thread, which walks every 4 × 4 block in its sight one after another (sight 30 is about 250 blocks, 4,000 cells). The rest of the 1.0-1.5 ms is the frame-start passes and the read-back, which waits for the GPU every frame. For W2b: one workgroup a unit for the sight scan, and read the previous frame's outputs without waiting. Both are under the W2c bar (10 ms p95) as they stand.

## W2b: Larder's rules on the world, 2026-11-11

**What runs.** `src/mould/larder_world.rs` is the game: M3's squad over a `World` (the CPU big dish, and `gpu::larder_world::GpuWorld` on the engine). Each squad frame the game walks the soldiers, harvests, picks shots, and hands one `squad::Frame` to the world (W2a), then reads back touch, aim, the aimed block's walkers and harvest. `examples/larderworld.rs` plays fixed hands on paired seeds.
- **The world** (`race_world`): 512 × 320 fine cells (scale 2, M3's 256 × 160 dish), 23 crystals at charge 512 a cell that run dry, rock and wet ground at 0.2, and a mould of 5,000 walkers that starts as a blob on crystal 0 and grows (upkeep 0.001, room for 50,000). The squad of 4 starts on the crystal farthest from it, which is the larder.
- **Rules** (`Rules`, M3's where they carry over): 15 ticks a game second, a 900 s cap. Soldiers walk 10 dish cells a second, see and fire 15, strike 2.5, one shot a second at 10 food, 100 health, drained 1 a second by each touching walker. A clear order harvests 30 food a second into a carry of 300 (64 charge a food), walked home to the larder. A soldier costs 400; the bank starts at 2,000. Walls cost 20 food a dish cell, need a soldier in range of both ends, and wear down as in W1a. The larder smells of the bank, and walkers touching it steal 1 food a second each. Holding the core clear of mould for 5 s stops births.
- **Ends and score**: M3's ordered score. The squad wipes when no soldier is alive and the bank can't buy one; the mould wipes when no walker lives; at the cap the score is the squad's share of the crystals held, minus 0.5. The squad holds a crystal with an untouched soldier on it; the mould holds one it has eaten from with no soldier on it. (The first version gave the mould every crystal without a soldier, reached or not.)
- **Not ported yet**: turrets, bait, hunger, and bodies as crystals.

**Hands** (fixed, `examples/larderworld.rs`): *idle* never orders, so the four stand at the larder on Defend. *random* gives one soldier a random order and stance every 5 s, buys with chance 1/4 and lays a short wall with chance 1/8. *farm* sends soldier k to clear the k-th nearest crystal, on Defend, and buys up to 8 soldiers while the bank stays over 600 after the purchase. *rush* sends everyone at the core, buying with all but 600. *wall* farms and also builds a wall 26 dish cells long across the core-to-larder road, 0.6 of the way to the larder, as soon as a soldier can reach it.

**Measure, named first.** The score paired by seed, each plan (farm, rush, wall) against idle and against random, a two-sided sign test, and Holm over the six comparisons. Gate (ROADMAP): idle and random lose; a plan beats both.

**First look and a first census** (seeds 1-4, before the fixes below; [raw](larder-world-roadmap-2026-11-11/w2b/census-1-4-before-fixes.txt)). Random was wiped on 4 of 4 seeds and idle on 2 of 4, at 316-630 s. Farm and wall reached the cap on all four. Three faults showed:
- **Rush hardly killed.** 80 shots killed 13 to 46 walkers on three seeds. A trace of seed 1 ([raw](larder-world-roadmap-2026-11-11/w2b/rush-trace-seed1-2.txt), [script](larder-world-roadmap-2026-11-11/w2b/rush-trace.rs)) showed why. Three of four soldiers stood behind rock 50 cells short of the core, and the fourth never moved: walking only slid along an axis when blocked. 196 of their 200 shots went into the rock, which stops a shot with no hit, until the bank ran dry at 80 s.
- **The wall hand built no wall** on seeds 1 and 3. It tried once at 5 s, out of range, and the farm hand took back the soldier sent to build it.
- A shot fired at a block of lone walkers can fly through: they don't show as slime. The units pass now also returns the aimed block's walker count (`UnitOut::dense`, on the CPU and the GPU; the W2a twin test compares it and caught a planted bug in it), and `Rules::aim_min` holds fire below it.

**Fixes.** Soldiers walk round rock and walls on a distance map to their goal: Dijkstra over open cells, 5 a side step and 7 a diagonal, no corner cut, built when first needed, and rebuilt every 30 game seconds and after each wall. They fire only along a clear line to the aim. On rush seed 1 all four soldiers then reached the core by 20 s, none of 200 shots was wasted, and they killed 1,282 walkers instead of 13; on seed 2, 1,452. With the sight check, `aim_min` above 1 bought nothing: on seed 2, 1 wasted 1 shot of 200 and killed 1,600, 2 killed 1,486, and 4 killed 1,452. So it stays at 1, and the read stays for the window and the player AI. The wall hand now sends soldier 0 to stand by the wall's middle until it can build. A [test](../src/mould/larder_world.rs) checks that the walking map goes round a wall through its gap and that a sealed goal reads unreachable.

**Held-out census** (seeds 5-20, all five hands, to the 900 s cap; [raw](larder-world-roadmap-2026-11-11/w2b/census-5-20.txt)). **The gate passes.** Idle and random lose on all 16 seeds, and farm and wall each score higher than both on 16 of 16 (sign test p < 0.0001, all four comparisons hold under Holm over six).

| Hand | Mean score | Squad wiped | Reached the cap | Score above 0 | Food harvested (mean) | Walkers killed (mean) |
|---|---|---|---|---|---|---|
| idle | −0.779 | 5 (274-432 s) | 11, holding 6-17% of the contested crystals | 0 | 0 | 216 |
| random | −1.373 | 12 (98-894 s) | 4 | 0 | 956 | 856 |
| farm | −0.027 | 0 | 16, holding 27-64% | 5 (3 at exactly 0) | 22,725 | 3,072 |
| wall | −0.021 | 0 | 16 | 7 | 22,331 | 2,811 |
| rush | −1.874 | 16 (10 of them by 36 s) | 0 | 0 | 0 | 899 |

What it says, beyond the gate:
- **No plan wins outright.** No hand wipes the mould, and farm and wall end in the stalemate band. Their share of the contested crystals spreads from 27% to 64% by seed, which is the spread the owner asked an outcome to be.
- **The wall adds nothing here.** Wall against farm: 6 seeds higher, 7 lower, 3 tied (p 1). One 26-cell wall across the road is too small on this world for the mould to notice.
- **The rush is suicide.** Seven soldiers reach the core in about 25 s and kill about 900 walkers, just as the mould's first walkers starve (start energy runs out near 17 s on every hand). On several seeds the mould is down to 968-2,082 when the bank runs dry and they die. Farm first, buy an army, then rush (M1's reader plan) is the obvious next hand.
- **An idle squad is beaten but often not killed.** On 11 seeds the mould has not reached the larder by 15 minutes. The owner's race wants an idle player overrun (14:54Z), so a smaller world, a nearer start or a faster mould is worth trying when maps arrive (WM2).

The 80 games took 24.6 minutes on four threads.

**The game on the GPU engine.** `GpuWorld` implements `World` over `Engine`. The CPU race world takes `scale` fine steps a tick, and everything that moves, eats or grows runs once a fine step. The engine takes one fine step a tick, so the GPU world runs `scale` engine ticks for each game tick. `race_settings` gives the engine the settings whose params are the CPU race world's, bar `substeps`; a unit test checks the params are equal. A GPU test (`the_larder_game_runs_on_the_engine_as_on_the_cpu`, lavapipe in CI) plays seed 3 for 20 game seconds on both:
- the crystals, mould, larder and core are equal at the start;
- a soldier clearing the larder's crystal banks 300 food on both;
- the mould stays within 8% of the CPU's size every 5 s. The starting walkers starve at about 17 s on both, so at 20 s the count falls from 5,234 to 3,120 on the CPU and 3,220 on the GPU.

With one engine tick a game tick (planted), the GPU's mould still held 5,129 at 20 s and the test failed. The GPU world reads its squad results back each frame and waits for them; the 1070's frame cost of the game, and the units-pass speedup W2a named, come with the window (W2c).

**Cost.** On the CPU, the race world with the game layer costs 5.5-14 ms a tick on this 4-core box (the most when a farm squad of 8 is out), so a 900 s game takes 1.5-3 minutes and a 16-seed census of five hands about 45 minutes on four threads.

## Maps in the plan (owner, 19:29Z)

The owner saw the world drawn as a black background and asked: "Is it part of the roadmap to build an actual world?" Then: "We will want maps though. That should be part of the plan". Today's world has ground the simulation uses (rock, wet ground, rubble, crystals, walls), but noise and a pile layout place it. Open agar draws dark because the simulation stores nothing there except trail and scent. Under the rule that the picture is the data, a richer ground needs ground that does something. ROADMAP.md now has five map steps:

| Step | What | Owner | Gate |
|---|---|---|---|
| WM1 | A map file and loader: ground kind per cell, crystals (centre, size, charge), starts; into the CPU big dish and the GPU engine; noise worlds can be saved as maps | Larder thread, with W2b | The CPU and GPU load a map the same (twin test); a saved noise world plays as the world it came from |
| WM2 | Three to five designed maps, each made for a different plan ([the maps thread's plan](larder-maps-2026-11-11.md): five paper maps) | maps thread | Idle and random hands lose on each; the winning plan changes between maps |
| WM3 | New ground kinds the simulation uses, brainstormed from the model. Height is parked (owner, 20:06Z, below); sand, clay and barren ground, which act on the mould's strong terms, are tested first | maps thread; GPU side with the Larder thread | Each kind changes a measured outcome and can be seen |
| WM4 | The ground's look, drawn from those fields | graphics thread | #69, WM3 |
| WM5 | A map editor in the window | to be chosen | WM1, W2c |

Owner, 20:06Z, to the maps thread: "Your map generation is heavily focused on heights. How much does height actually influence our slime mold behaviors? I am skeptical how much fun/complexity/strategy this adds to the game. You should also be starting from the improve solider and slime graphics work which already has added terrain to their map (and is visually much better)". So height is parked until a test shows it changes play. The first test is the ground kinds that act on the mould's strong terms, and the maps are rebuilt on the graphics thread's terrain.

Agreed with the graphics thread (19:33Z): byte 3 (bits 24-31) of the engine's ground word is reserved for height. Nothing fills it while height is parked, and the view shades it only once the simulation uses it. After rebasing on W1b, the view reads the engine's ground word in place of its own ground buffer. Agreed with the maps thread (19:40Z): soil kind in bits 12-15 and mould-made ground in 16-23. The loader reads `maps/<name>.png` at 1024 × 640 (R height, G the word's bits 8-15) and `maps/<name>.txt` (crystal, mould, landing and wall lines; soft rock is walls, so the engine builds them as it builds player walls), both written from the maps thread's `maps.json`.
