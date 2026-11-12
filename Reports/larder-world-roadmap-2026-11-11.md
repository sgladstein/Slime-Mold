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

## WM1: maps load into the world, 2026-11-11

**What loads.** `src/mould/map.rs` reads a map in the agreed files (`Map::load`): `<name>.png` at 1024 × 640, whose G byte is the ground word's bits 8-15 (wet 0x01, rock 0x08, soil kind in the high nibble), and `<name>.txt` with `crystal x y r charge`, `mould x y`, `landing x y` and `wall x0 y0 x1 y1 thick hp` lines in the same cells. `BigDish::with_map(p, &map)` builds the world from it, in place of the keyed piles and the noise:
- **Crystals** each have their own radius and charge a cell (`BigDish::pile_rs`, `pile_fill`). Without a map every pile has `pile_r` and `crystal_charge`, so older dishes are unchanged. The crystal nearest the mould's start is pile 0, the mould's home.
- The mould's blob centres on `mould` (`BigDish::homes`). A map may give several starts: the Passes has two rivals, and the walkers are shared equally among the starts (one species, for now). A blob that cannot fit widens by a quarter until every walker has a cell: the v2 Fork's start is 12 dish cells from the rim, and its blob placed 3,752 of 5,000. The first round draws as before, so a blob that fits is unchanged.
- The larder sits on `landing` (`World::landing`). A world without one keeps the farthest crystal.
- Rock, wet and soil come from G, sampled at each fine cell's centre. Walls are built with their own health (`build_wall_hp`) before the walkers are placed.
- `p` still switches each system on (`crystal_charge` > 0 for crystals). Height (R) is parked and not read.
- `Map::of_dish` saves a made world as a map. `examples/larderworld.rs` takes `map=NAME` for a census or a look on a map.

**On the GPU.** `Engine::with_map` and `GpuWorld::with_map` start the engine from the same dish. Each pile's full charge rides in its pile record: the shader's `Pile::full`, a spare word until now, feeds the glow, the share left and regrowth. The world bits for terrain and wet now also come from the dish's ground (`ground_bits`), so a map's rock and wet run with the noise settings at 0. Soil is not on the GPU yet. The engine says so when a world has any, and that port (WM3's GPU side) is next (*done 11-12: Soil on the GPU, below*).

**Gate (ROADMAP).** Both parts pass, with lavapipe standing in for the GPU here and in CI.
- **A map loads the same on the CPU and the GPU.** `a_designed_map_plays_on_the_engine_as_on_the_cpu` plays the v2 Fork (rock, wet, ten crystals of five sizes) and the Nest (34 walls) for 20 game seconds on both, with soil taken off both. Crystals, radii, start and landing are equal. A soldier clearing the crystal nearest the landing banks 300 food on both. The moulds stay within 1% at every 5 s sample (Fork at 20 s: 2,639 on the CPU, 2,619 on the GPU; Nest: 4,109 and 4,128), and the charge left within 2%. The v1 Fork and Nest (smaller crystals) also passed.
- **A saved noise world plays the same.** `a_saved_world_loads_and_plays_the_same` saves the race world as a map, through the text file, and loads it back. The rock, wet, agar, scent ground, crystals and the dish's hash after 30 ticks are all equal, at scales 1 and 2.
- `a_designed_map_makes_its_world` checks the rest on the designed maps. The Nest's crystals come in order, each crystal's cells hold its own charge, and the blob sits within 3 dish cells of the start. All 34 walls stand at 400 health with no walker inside. The rock, wet and soil counts equal the map's on the Fork, Nest and Passes. Every Fork walker is placed, and the Passes' two blobs hold 2,500 each.

**What the maps showed.**
- **The v1 maps held 3-6% of the race world's food.** Their crystal radius r 12 is 3 dish cells, half the race world's `pile_r` of 6, and the race world's 4.44M charge compares with 146k-262k. With 5,000 walkers, the v1 Fork's mould fell to 1,709 by 20 s. The maps thread drew v2 (PR #83) with three times the radii: 56-97% of the race world's crystal cells.
- **A soldier waits at a dry crystal (not fixed).** On the v1 Fork one soldier emptied the landing's crystal (12,032 charge, 188 food) in 7 s, then stood there for a full 300 load that never came. On the v1 Nest the take dwindled toward 0 as charge seeped in from the crystal's edge, and it sat at 128. M3 and W2b have the same rule (a carrier goes home only full); the race world's crystals never ran dry under one soldier. Four rules that send the carrier home were tried, each on race seeds 5-20 paired with W2b's census (score at the cap; one-sided sign test), and each cost the planned hands:

| Rule: home with what it carries once the frame gave under half its ask, and | farm, lower / higher (mean) | wall, lower / higher (mean) | raw |
| --- | --- | --- | --- |
| nothing else | 2 / 0 of seeds 5-6 (seed 5: −0.136 against −0.029) | 2 / 0 | [partial](larder-world-roadmap-2026-11-11/wm1/census-half-ask-partial.txt) |
| the whole crystal holds less than the rest of a load | 15 / 0 of 16, p 0.0001 (−0.204 against −0.027); kills lower on 13, down a fifth | 15 / 1, p 0.0005 (−0.203 against −0.022) | [raw](larder-world-roadmap-2026-11-11/wm1/census-5-20-cantfill.txt) |
| that, and it carries a quarter load or more | 1 / 1, 14 the same (−0.029 against −0.027) | 8 / 0, p 0.008 (−0.053 against −0.022) | [raw](larder-world-roadmap-2026-11-11/wm1/census-5-20-floor.txt) |
| the whole crystal holds less than one frame's ask | 10 / 0 of 10, p 0.002 (−0.188 against −0.034) | 8 / 1 of 9, p 0.04 (−0.210 against −0.033) | [partial](larder-world-roadmap-2026-11-11/wm1/census-5-20-dry-partial.txt) |

Every one banked a little more food (farm higher on 10 of 10 under the last) but sent soldiers off crystals with crumbs left, and back for more crumbs, and the score's share counts a crystal with any charge left as held while a soldier stands on it. Idle, random and rush were unchanged, apart from seed 13, whose blob now places differently (the widening above). So the rule stays W2b's, and the window's "crystal ran dry" alert tells the player to order the soldier on. A rule that keeps the hands' score needs the share to weigh a crystal by its charge left, or hands that re-order a soldier as a player would. (Dated note, 2026-11-12: under score v2 the last rule costs nothing and is on, with the soldier holding at the larder once it banks; see *Score v2 and the dry crystal*.)

**Cost.** Per step, the new work is a pile lookup per crystal cell: in regrowth and the picture on the CPU, and in the GPU's draw, pile sum and flow passes. None of this is timed on the 1070 yet; the window (W2c) times the whole game there.

## W2c: the game in the window, first cut, 2026-11-12

**Why now.** The owner played `cargo run --release --bin mouldgpu` (relayed 2026-11-11 23:53Z): "I could get the soliders to appear but they didn't interact with the slime at all. There were also enemy soliders but if they moved near each other everyone health dropped but there was not actual shooting, it just seemed like proximity." That was the sandbox's stand-in squad (Q, `world::demo`): the camera and UI thread's test of the controls, with no game rules, whose rivals trade damage by range. The game the W2b census measured had no window.

**What is built.** `cargo run --release --bin larder-world [-- map=fork seed=1]` (`src/bin/larder-world.rs`) plays `larder_world::Game` on `GpuWorld`, on a v2 map (`map=` fork, nest, rim, flats, passes, or `noise` for the race world), with the world's camera, controls, look and UI. Every command goes through `Game::apply`, so a round saves as a replay. The game steps at its own 15 frames a game second through an accumulator, whatever the window's frame rate; Tab runs it four times faster.
- **Orders.** Right click moves, or on a crystal harvests it. The command card has attack the core (A), stop (S), harvest (G), burn a line (F), hold fire, defend and free fire (Z, X, C), buy a soldier (B, 400) and lay a wall (V, 20 food a dish cell, both ends in a soldier's range). The ghost says why a wall or a harvest cannot go where the cursor is.
- **What is drawn is the engine's.** The look shades the mould from the engine's own look field. The ground comes from the world's rock, wet, walls and crystals, with each crystal's cells (`pile_of`, so a map crystal's size and shape are the ones the mould eats) dimming by its charge left and each wall by its health, read back once a game second. A wall the GPU has worn through draws as rubble. A shot draws as its ray for a tenth of a second, from the soldier to the slime it stopped at, with a muzzle flash.
- **Marks.** Soldiers are the look's sprite (A), drawn as wide as their touch (1.5 dish cells: walkers that close drain them), and at least 8 pixels across, so they read at the whole map's zoom. A health bar shows under each and a carry bar while it carries. The larder and the core's ring are marks.
- **The HUD.** Food with its rate over the last 10 s, soldiers and losses, walkers alive, and the share of held crystals (the score at the cap). The clock against 15:00, the objective, a card per selected soldier and the hover card for a crystal (charge left) or the larder. Alerts fire when a soldier falls, a crystal runs dry, the core is taken, the mould reaches the larder, or the food runs out. The end screen gives the band, the share, the counts and the score.
- **Two engine and view fixes it needed.** The look drew a speck for every walker slot. With growth the empty slots sit stacked at the dish's centre and the dead stay where they died, so it drew a bright dot and the ghost of every walker a shot killed. `View::set_alive` now takes the energy buffer, and a slot below 0 draws no speck. And the game's shots cut in the squad's `cut` pass, which did not write the look's wound byte (only the stand-in hits of `set_hits` did): the cut now writes it too, so a shot leaves the wound of the thickness it destroyed. `the_window_plays_the_real_game` plays the Fork with all four soldiers sent at the core for 12 s. It checks shots and kills, a ray drawn for every shot, and a wound in the look; with the wound write removed it fails ("no wound in the look after 27 shots").

**What it looks like** (no window: `-- look=DIR hand=rush|farm`, software Vulkan, 1280 × 800).
- [The Fork, rush, 11 s](larder-world-roadmap-2026-11-11/w2c/fork-rush-011s-near.jpg) (zoom 6; [whole map](larder-world-roadmap-2026-11-11/w2c/fork-rush-011s-all.jpg)). Four soldiers on the home crystal at 62% health on average; 23 shots, 612 walkers killed, mould 4,445. The dark red holes are the wounds the strikes cut.
- [The Nest, farm, 120 s](larder-world-roadmap-2026-11-11/w2c/nest-farm-120s-all.jpg) ([30 s, near](larder-world-roadmap-2026-11-11/w2c/nest-farm-030s-near.jpg)). The farm hand drained the five outer crystals in two minutes (the dark hexagons, 3,392 food harvested), bought up to 15 soldiers, then walked into the ring and lost 6 of them on the rich crystals. Mould 4,165, held 0%, food 2.

**Cost.** The window's own CPU work (the cloud's CPU): the model and HUD layout 0.03-0.05 ms a frame. The ground field is 3.9-6.1 ms once a game second, a hitch to cut (most of it is the crystals' depth). Each game step makes four small blocking read-backs (the squad's out, its kills, the pile sums, the wall counts), as the census does, plus the energy once a game second. The window prints its frame time every 10 s (p50, p95, max, steps a second). The gate's number, under 10 ms p95 on the GTX 1070, is not measured yet.

**Not yet.** Soil plays as plain ground on the GPU, so the v2 maps' sand and clay do nothing in the window (the engine says so at start); that port is next (*done 11-12, see Soil on the GPU below*). The view does not draw soil, and maps have no look of their own yet (WM4). The second rival on the Passes is the same species. Turrets, bait and the tiers are not in the world game. A soldier whose crystal runs dry waits there until it is ordered on (WM1's open bug; the alert says when).

## Score v2 and the dry crystal, 2026-11-12

**Why.** Two findings said the stalemate score measured the wrong thing. In WM1, every rule that brought a carrier home from a dry crystal cost the planned hands, because `Game::share` counts a crystal with 0.8 charge left as held while a soldier stands on it. The player-AI thread's census ([report](larder-world-ai-2026-11-12.md), seeds 21-36) found two more problems, both from the share being read only at the cap. A style that grabs crystals in the last 90 s (late) tied one that holds them from 10:00 (claim), 7-7. A strike that took the core and left 7 to 574 walkers still scored as a stalemate, the same as a farm that left 9,000.

**The rule** (`Game::score`). Wins and losses keep `End::score`'s bands. A stalemate at the cap scores `0.5 (take - 0.5) + 0.25 (1 - m) / (1 + m)`, which stays inside (-0.5, 0.5):
- `take` (`Game::take_share`) is the squad's share of all the charge gone from the crystals over the game. The squad's part is what it took, less the food the mould stole from the larder. The mould's part is what it ate, plus what it stole.
- `m` is the walkers at the cap over the walkers at the start. A mould cut to nothing adds +0.25, one at its starting size adds 0, and a growing one tends toward -0.25.
- W2b's score is kept as `Game::score_w2b`, and the census prints it as `score1`. It reproduces W2b's census exactly on every seed except 13, where WM1's widening changed where the blob is placed.
- The HUD's fourth number and the end screen now show the charge share. The objective reads "Take the mould's core, or out-harvest it and cut it down by 15:00".

**Tried first: the held share weighted by charge left.** On race seed 5 it put farm below idle (−0.265 against −0.232), because harvesting empties the crystals you hold ([dead end](dead-ends.md)).

**Deciding measures, named before the runs.** The census is race seeds 5-20 at the 900 s cap, paired by seed, sign test, Holm within each run.
1. The gate still holds under v2: farm and wall beat idle and random.
2. The AI's question: does v2 credit a strike that cuts the mould down?
3. For the dry-crystal rule: no lower score than with it off.

**Results.** Raw output is in [score2/](larder-world-roadmap-2026-11-11/score2/).
- **The gate holds.** Farm and wall beat idle and random 16-0 on every pairing. Rush loses to idle 0-16 and to random 2-14. This is the same as W2b. The planned hands' stalemates now score +0.02 to +0.36 (farm mean +0.132, wall +0.128), and idle's stalemates score −0.38 to −0.20. Under W2b's score, farm and wall stalemates were centred near −0.03.
- **The AI styles** (#88's defaults, seeds 5-20; [raw](larder-world-roadmap-2026-11-11/score2/census-5-20-v2-styles.txt)). Strike beats farm 14-2 (p 0.004) under v2, against 8-8 under W2b's score. Strike against claim is 9-7 (p 0.80), against 3-13 (p 0.02) under W2b's. So v2 credits the walkers the strike killed: at the cap it left 2, 2, 65, 230 and 839 walkers on five seeds. Claim against late is 8-8 (9-5 under W2b's). The last-minute grab no longer earns anything, and the two styles took about the same charge, so v2 does not separate them.
- **The dry crystal.** `Rules::home_dry`: the frame gave under half its ask, and the whole crystal holds less than one frame's ask, so the carrier goes home with what it has.
  - **Under v2 it costs nothing.** With W2b's farm hand, farm was higher on 5, lower on 8 and the same on 3 (p 0.58; mean +0.132 to +0.133). Wall was 7 higher and 9 lower (p 0.80; +0.128 to +0.116). Farm's kills were lower on 12 of 16 (p 0.08).
  - **The farm hand never left a dry crystal.** It re-ordered a soldier only from a crystal at exactly 0 charge, and crystals keep crumbs. So W2b's farm and wall stood on emptied crystals for much of the game. The hand now skips crystals under one frame's full ask. The soldier the game sends home now holds at the larder once it banks (`Soldier::dry`), rather than walking back for crumbs. The hand's fix alone (rule off) doubled the food (farm 22,875 to 47,269 harvested a game, 16 of 16) and nearly quintupled the kills (3,167 to 15,143). Farm scored higher on 15 of 16 (p 0.0005). Wall scored higher on 12 of 16 (p 0.08), but its mean fell from +0.128 to −0.103: the squad was wiped on 4 seeds, between 695 and 832 s, with 0 to 6 food in the bank.
  - **The rule with the hold, against the hand's fix alone:** farm is even, 7 higher and 9 lower (p 0.80). Wall is higher on 13 of 16 (p 0.02, which holds under Holm over the two). Wipes: farm 2 (seeds 13 and 20, at 644 and 886 s) and wall 1, against 0 and 4. The rule is on by default, since no score is lower with it.
- **What the census hands now do.** With the fixed hand and the rule, farm harvests 46,364 food and kills 15,240 walkers a game on average, against W2b's 22,875 and 3,167. It now loses its whole squad late on 2 of 16 seeds, when the bank runs out. That is the expansion the run-out crystals were meant to force (owner, 14:06Z). These are the new baselines, and W2b's farm numbers come from a hand stuck on dry crystals.

**The window's ground hitch.** The ground field now takes the crystals' depth worked out once a world (`look::crystal_depth`, from the graphics thread's #90, held in the window's `depth`). The once-a-game-second rebuild is down from 3.8-5.3 ms to 1.7-1.8 ms on the cloud CPU (headless, `hand=farm at=5`): Fork 3.79-3.88 to 1.67-1.68, Nest 5.28 to 1.82, Passes 3.88 to 1.76.

**Cost.** `take_share` sums the crystal list. It runs once at the cap, and in the window once a frame for the HUD (23 crystals). The rule reads the crystal list once a squad frame, only while it is on.

## Soil on the GPU (WM3's GPU side), 2026-11-12

**Why.** The v2 maps' sand and clay played as plain ground in the window, because the engine read no soil (it said so at start). The maps thread measured each kind on the CPU big dish ([report](larder-maps-2026-11-11.md), *Which ground the mould feels*), and the census (`larderworld map=NAME`) already ran with them, so the window played different maps from the ones measured.

**What is built.**
- The soil kind (big.rs `SOIL_SAND` 1, `SOIL_CLAY` 2, `SOIL_BARREN` 3) is packed into bits 12-15 of the engine's ground word (`GROUND_SOIL`). That is where the map's G byte already puts it, and the slot was agreed with the graphics and maps threads on 11-11.
- `WORLD_SOIL` (16) is set by `ground_bits` when any cell has soil. Without it, the passes read no soil.
- One new row at the end of `Params` (832 to 848 bytes, after the strike row) carries `sand_keep`, `clay_keep` and `barren_upkeep`, taken from the dish's own `BigParams` as the CPU reads them.
- The trail pass multiplies the trail by the cell's keep after the spread. With species, each species' field is multiplied too, and the trail is their sum times the keep, in the CPU's order.
- The life pass charges `upkeep x barren_upkeep` on barren cells, and the breed pass skips a barren target.
- The start-up warning is gone. The window passes the map's soil to the look (`Fields::soil`), which has drawn it since #84.

**Deciding measure, named before the run.** One GPU tick from a warmed CPU dish matches one CPU tick within the world twin test's bounds, with soil on. The 20-second game twin stays within its 8% bound on maps with soil. A planted bug in each soil rule fails the test.

**Results** (software Vulkan; raw in [soil/twin.txt](larder-world-roadmap-2026-11-11/soil/twin.txt) and [soil/runs.txt](larder-world-roadmap-2026-11-11/soil/runs.txt)).
- **One tick.** `the_world_matches_the_cpu_for_one_tick` gains two dishes: sand, clay, barren and plain bands across the dish, with growth on crystals, run with one species and with two. Every living walker is fed to breed before the tick, so children aim at barren cells too; 1,925 and 2,076 were born in the tick. In both dishes 0 of 15,000 walkers are off, and the trail and energy L1 are 0.
- **Planted bugs.** Each soil rule was broken one at a time in the shader, and each fails: sand's keep skipped (trail L1 1.5e-2, bound 1e-4), barren upkeep x 1 (energy L1 4.0e-3, bound 1e-5), births on barren (528 walkers off, energy L1 0.13). The test was first written without feeding the walkers; no child was born in the tick, so the third bug passed.
- **The game twin** (`a_designed_map_plays_on_the_engine_as_on_the_cpu`) now plays the maps with their soil (it used to clear it), and adds the Passes and the Rim (sand and clay) to the Fork and the Nest (clay). Both sides bank 300 food on every map. At 20 s the moulds are Fork 2,642 on the CPU and 2,644 on the GPU, Nest 4,025 and 4,022, Passes 3,174 and 3,162, and Rim 4,756 and 4,658 (2.1%, the widest).

**What it does in play** (one game, seed 1, farm hand, soil as drawn against the same map with soil cleared; [page](https://claude.ai/artifact/1ifC4Qrw6fXe5Aq7vVSRzZ), pictures in [soil/](larder-world-roadmap-2026-11-11/soil/)).
- **The Rim.** The sand ring holds the mould in. With soil, the clay core carries a broad sheet, and only beaded strands cross the ring at the gates. By 60 s no tube runs to an outer crystal, and the mould is 5,477 walkers. Without soil, tubes cross the ring to the top and bottom crystals by 30 s, and the mould is 5,928 at 60 s.
- **The Passes.** The clay lane's tube is smooth and wide. The sand lane's tube breaks into beads with a gap. Without soil, that lane forms a loop of two tubes.
- The harvest and the take are the same in each pair (the farm hand works the outer crystals). This is a look, not a census: the maps' outcome counts are WM2's.

**Cost.** On software Vulkan, a game step costs the same with soil as without, within the 0.3-0.9 ms noise between rounds: Rim 14.52 against 14.82 ms, Passes 14.82 against 14.90 (scale 2, 5,000 walkers, three rounds). Soil adds a uniform branch and one read of a ground word each pass already binds. The 1070's figure is open, with the window's p95. The window's ground rebuild with soil drawn is 1.64-1.77 ms once a game second.

**A fix that rode along.** The headless run's first "all" picture was zoomed past the world's edges. The camera's `fit` was read before the HUD had set its insets. It now prepares the frame first.

## Playtest fixes: free shots, the soldiers' scent, and a hole under fire, 2026-11-12

**Why.** The owner's first playtest of the window (11-12 04:13Z, relayed by the coordinator): "1) the slime didn't feel like one cohesive organism. 2) shooting the slime didn't feel satisfying. I know it was killing lots of individual walkers but visually it didn't impact much 3) the slime mostly ignored my soldiers unless they walked into the slime 4) energy required for shooting isn't fun. I ran out of energy and I had a bunch of soldiers but I couldn't fight the slime and I couldn't harvest crystal because the slime would just kill me." Point 1 went to the one body thread. On point 2 the owner turned down both of the graphics thread's drawn hits (04:38Z, "Both of those look like stamps... Maybe part of the issue is the slime/walkers move to fast") and set the bar (04:39Z): "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at."

**What is built.**
- **Free shots** (`Rules::price` 0, was 10): a soldier's fire rate (`shot_every`, 1 s) is the only limit. The window's texts drop the cost when it is 0. A price still comes out of the bank shot by shot (test).
- **The soldiers' scent as rules** (`Rules::smell` 0.5 of a full pile's scent a step, `Rules::smell_r` 2 dish cells), with census switches `smell=` and `smell_r=`.
- **The mould's turn from fresh gunfire, as an opt-in** (`larder_world::FLEE_KEEP` 0.98, `FLEE_WEIGHT` -1; the census's `flee=W` and `flee_keep=K`, the hole test's `flee:W[:K]`). Each hit already laid the alarm field (`squad::SHOT_ALARM` 50 out to twice the strike's radius) when a tier turned the field on; with the opt-in the plain mould has the field and its sensors read it at -W. It was the race world's default for a few hours and **failed the census gate** (below), so `race_params` does not set it.
- **The hole test** (`examples/larderhole.rs`): the race world grows 60 s with the squad held at the larder (no drain), then a squad of 4 is set down in a line 8 dish cells past the body's edge on the ray from the core to the larder (turning 20 degrees at a time if a spot is off the open agar or in the mould), on Hold. It fires freely 20 s, then holds fire 10 s. The same seed is run holding fire throughout as the control (an alarm-only arm's control is base's, run once: no shot, no alarm, the same game to the bit, checked on seed 1). `look=DIR every=S` saves crops.
- **For other threads**, all off by default so no census moves: `Game::density` (start walkers per open fine cell; integration thread), `Game::stuck` and `stuck_run` (the census's stuck test, counted by the game for the campaign's mission layer), `Rules::overrun` (`End::Overrun`, a loss: the mould holds the larder with the bank empty that many seconds) and `Rules::reserve` (a free soldier every that many seconds while under 2 live and the bank can't buy one, unless the mould holds the larder). Tests for each.

**Deciding measures, named before the runs.** For the hole: the dish cells within the squad's range (15) of its centre that held a living walker when it was set down and hold none after 20 s of fire, each arm against the race world without the field, paired by seed, sign test, Holm over the arms. For the defaults: score v2 (`Game::score`), each plan against idle and random, paired by seed, sign test, Holm; the median paired difference beside each count (technical lead's review: 32 seeds and two designed maps before a default).

**The hole under fire** (seeds 1-16; raw in [playtest/hole-1-16.txt](larder-world-roadmap-2026-11-11/playtest/hole-1-16.txt), run before the default changed, and [playtest/hole-1-16-default-sporer.txt](larder-world-roadmap-2026-11-11/playtest/hole-1-16-default-sporer.txt), after; [page](https://claude.ai/artifact/5wQ3xSk8pBN6uig1rxKd4R), clips in [holes/](larder-world-roadmap-2026-11-11/holes/)). Medians:

| Mould | hole at 5 / 10 / 20 s of fire | share of the mould in range | hits land | hole 2 / 5 / 10 s after the last shot | killed | vs no field at 20 s |
| --- | --- | --- | --- | --- | --- | --- |
| holding fire (control) | 26 / 28 / 27 | | | 30 / 27 / 29 | 0 | |
| no alarm field (before) | 33 / 33 / 29 | 26% | 3.7 dish cells out | 30 / 31 / 27 | 175 | |
| flee 1, keep 0.98 (A) | 86 / 101 / 102 | 77% | 9.3 | 79 / 42 / 28 | 251 | 16 of 16 larger, +69 median, p < 0.0001, holds |
| flee 3 (B) | 100 / 124 / 126 | 93% | 10.7 | 103 / 70 / 42 | 293 | 16 of 16; 15 of 16 larger than flee 1 (+16, p 0.0005) |
| flee 1, keep 0.99 | 101 / 122 / 127 | 92% | 10.5 | 110 / 96 / 53 | 272 | 16 of 16 |
| flee 0.3 | 64 / 67 / 72 | 55% | 7.7 | 58 / 33 / 31 | 212 | 15 of 16 |
| flee 1, keep 0.95 | 67 / 74 / 67 | 50% | 7.6 | 41 / 34 / 26 | 228 | 16 of 16 |
| the Swarmer's pull (+3) | 56 / 58 / 66 | 39% | 3.0 | 57 / 50 / 39 | 991 | 16 of 16, but 4 of 64 soldiers alive |
| half the mould clock (no field) | 41 / 45 / 44 (control 37) | 39% | 3.7 | 42 / 40 / 43 | 186 | 12 of 16, +12, p 0.08 |

Without the field, shooting leaves no more hole than holding fire: the hits land 3.7 dish cells out every second, at a rim that closes at once. With it, the hole opens within 5 s, the hits move out to 9-11 cells as it deepens, and it stops growing at about 10 s only because it has taken most of what the squad can reach. It refills in about 10 s once the shooting stops. On one seed (5) the squad stood behind rock and fired 7 shots in 20 s, so no arm made a hole there; it is in every median. Every soldier lived through every test except under the Swarmer's pull. The page put flee 1 against flee 3 to the owner as a blind A/B (key: A is flee 1, B is flee 3); the card was withdrawn when the flee failed the gate.

**The tiers.** The Sleeper and the Swarmer set their own weight (+3), so the flee never changes them. The Sporer sets only the keep, so with the flee on it flees too: its hole at 20 s is 128 against 81 without the field (11 of 16 larger, p 0.21), and 87% of it is still open 5 s after the last shot (91% without the field: the Sporer's scatter already left a hole).

**Rejected for the hole.** A slower mould clock: 3x slower moved the graphics thread's refill timing only from 0.53 to 0.73 s, and half the clock gained 12 of 16 here (p 0.08). The Swarmer's pull as the default: a slightly larger hole, but the mould overran 60 of 64 soldiers. The flee as the default: below. All three are in `Reports/dead-ends.md`.

**The flee fails the census gate** (race seeds 5-20, 15-minute games, free shots; the flee on at ddd81c6 against off at a9eb09d, paired by seed; runner 1's raw files [census-flee-race-5-20.txt](larder-world-roadmap-2026-11-11/playtest/census-flee-race-5-20.txt) and [census-5-20-free.txt](larder-world-roadmap-2026-11-11/playtest/census-5-20-free.txt), fort and strike local: [census-flee-race-5-36-fort-strike.txt](larder-world-roadmap-2026-11-11/playtest/census-flee-race-5-36-fort-strike.txt), [census-noflee-race-5-20-fort-strike.txt](larder-world-roadmap-2026-11-11/playtest/census-noflee-race-5-20-fort-strike.txt)). Measures named before the run: each plan against idle and random (the gate), and for the game designer's review, first touch on the squad and walkers within 10 dish cells of it, flee on against off. Off → on:

| hand | mean score | mould wiped / squad wiped | walkers near the squad (mean) | first touch (median) | killed (mean) |
| --- | --- | --- | --- | --- | --- |
| idle | -0.330 → -0.332 | 0 / 0 → 0 / 0 | 7 → 5 | 81 → 286 s | 2,440 → 833 |
| random | -0.759 → **+1.348** | 2 / 10 → **12** / 1 | 125 → 107 | 20 → 24 s | 3,793 → 3,959 |
| farm | +0.214 → +0.238 | 0 / 0 | 406 → 193 | 27 → 66 s | 19,014 → 10,147 |
| wall | +0.225 → +0.245 | 0 / 0 | 348 → 212 | 39 → 43 s | 17,317 → 10,354 |
| rush | -0.134 → -0.117 | 0 / 1 → 0 / 0 | 27 → 10 | 16 → 16 s | 3,797 → 4,365 |
| fort | +0.206 → +0.155 | 0 / 0 | 22 → 20 | 26 → 39 s | 4,489 → 2,778 |
| strike | +0.697 → +0.349 | 3 / 0 → 0 / 0 | 84 → 41 | 26 → 39 s | 22,776 → 12,213 |

With the flee on, random beats farm, wall and rush 13-3 each (p 0.021); farm and wall still beat idle 16-0. Random's Attack orders send soldiers onto the core; a mould that turns from their fire no longer overruns them, they hold the core, and a mould without births starves (12 of its games end with the mould wiped by 900 s, most between 360 and 490 s, with only 1,000-2,000 walkers shot). The first touch comes later: farm 12 later and 2 earlier (median +36 s, p 0.013), idle +47 s, wall +15.5 s, rush 12-1 (p 0.003, survives Holm), fort and strike +16.5 s each (13-2, p 0.007, both survive); walkers near farm and wall fall on 14 of 16 (survive Holm), and farm and wall kill less on 16 of 16. Toggling the flee alone at the current code reproduces both censuses row for row (random, seeds 6-7). A stronger soldier scent does not save it: with scent 4 over 4 dish cells and the flee on, random wiped the mould on 13 of 16 ([census-flee-smell4-race-5-20.txt](larder-world-roadmap-2026-11-11/playtest/census-flee-smell4-race-5-20.txt), runner 2). On the Rim with the flee on, random still lost (its squad wiped on 11 of 16, farm and wall over it 16-0, [census-flee-rim-5-20.txt](larder-world-roadmap-2026-11-11/playtest/census-flee-rim-5-20.txt), runner 4), so the soft core depends on how open the map leaves it. So the flee answers the owner's point 2 by undoing point 3 and handing the game to random. It stays an opt-in. The game designer's review, on the core: any rule that keeps the mould off soldiers turns the core into a free win, so a later flee, or the Wary tier's habituation, must keep the mould defending its core (soldiers on the core smelling far stronger, or the alarm at the core pulling for every tier), tested against random first.

**The gate on this PR's head** (f92175e: free shots, scent 0.5 over 2 dish cells, #102's seek, no flee; 15-minute games; raw files from runners 1-4 through git, fort and strike local: [census-gate-race-5-20.txt](larder-world-roadmap-2026-11-11/playtest/census-gate-race-5-20.txt), [census-gate-race-21-36.txt](larder-world-roadmap-2026-11-11/playtest/census-gate-race-21-36.txt), [census-gate-race-5-36-fort-strike.txt](larder-world-roadmap-2026-11-11/playtest/census-gate-race-5-36-fort-strike.txt), [census-gate-fork-5-20.txt](larder-world-roadmap-2026-11-11/playtest/census-gate-fork-5-20.txt), [census-gate-rim-5-20.txt](larder-world-roadmap-2026-11-11/playtest/census-gate-rim-5-20.txt)). Measure named before the run: each plan against idle and random, paired by seed, sign test, Holm within each world; the median paired difference beside each count.

| world, seeds | hand | mean score | ends | against idle | against random |
| --- | --- | --- | --- | --- | --- |
| race, 5-36 | farm | +0.223 | 32 at the cap | 32-0, +0.565 | 30-2, +1.749 |
| | wall | +0.212 | 32 at the cap | 32-0, +0.552 | 30-2, +1.742 |
| | fort | +0.202 | 32 at the cap | 32-0, +0.546 | 30-2, +1.734 |
| | strike | +0.826 | 9 mould wiped | 32-0, +0.784 | 31-1, +1.920 |
| | rush | -0.344 | 7 squad wiped | 24-8, +0.272 (p 0.007) | 28-4, +1.192 |
| | idle | -0.342 | 32 at the cap | | |
| | random | -1.123 | 25 squad wiped, 2 mould wiped | | |
| Fork, 5-20 | farm / wall | +0.352 / +0.353 | 16 at the cap | 16-0 each, +1.955 / +1.957 | 16-0 each, +1.876 / +1.881 |
| | rush | -1.756 | 15 squad wiped | 1-15 | 1-15 |
| | idle / random | -1.498 / -1.423 | 15 squad wiped each | | |
| Rim, 5-20 | farm / wall | +0.047 / +0.061 | 16 at the cap | 16-0 each, +0.347 / +0.358 | 16-0 each, +1.684 / +1.678 |
| | rush | -1.530 | 16 squad wiped | 0-16 | 9-7 (p 0.80) |
| | idle / random | -0.295 / -1.176 | 16 at the cap / 10 squad wiped | | |

Every plan beats idle and random on all three worlds and survives Holm; the rush is the one hand that loses on the maps, as before. Two findings for the next PRs. Random wiped the mould on 2 of 32 race seeds (the tiers thread saw 2 of 4 on held-out seeds 201-204 at the same code, so about 4 in 36; with shots at 10 food it was 0 of 16). And strike dominates: it beats farm 28-4, wall 29-3 and fort 31-1 (each p < 0.0001), while every plan ends with 20,000-50,000 food in the bank, so food does not bind (the buildings census found the same with turrets: more guns at the crystals wins however they are bought). The economy is PR 3, running costs first (the game designer: everything alive eats), with a spend-everything hand as the fair comparison (technical lead).

**Next for the hole (PR 2, now #113: *The hole under fire*, below).** Shots lay the squad toolkit's crater (`BigDish::scorch_disc`, the one crater writer): the cells a hit clears stay closed to the mould for a few seconds, so sustained fire digs in and the hole closes once firing stops, with no change to where the mould goes elsewhere. Its gate's random and idle rows run beside the hole and first-touch measures (technical lead's review), not after them.

**Free shots and the scent** (seeds 5-20, 15-minute games, before the flee default; raw in [playtest/](larder-world-roadmap-2026-11-11/playtest/): `census-5-20-price10.txt` local, `census-5-20-free.txt` and `census-5-20-free-smell4.txt` from runners 1 and 2 at a9eb09d).

| mean score v2 | shots at 10 food | free | free, scent 4 over 4 dish cells |
| --- | --- | --- | --- |
| farm | +0.048 | +0.214 | +0.191 |
| wall | +0.152 | +0.225 | +0.191 |
| rush | -1.874 (16 squad wiped) | -0.134 (15 at the cap, 1 wiped) | -0.231 |
| idle | -0.729 | -0.330 | -0.327 |
| random | -1.325 | -0.759 | -0.443 (3 mould wiped) |
| farm / wall beat idle | 14 / 15 of 16 | 16 / 16 of 16 | 16 / 16 of 16 |
| farm / wall beat random | 16 / 16 of 16 | 14 / 14 of 16 | 12 / 12 of 16 (p 0.077, not holding) |
| farm: walkers within 10 dish cells, first touch (means) | 285, 49 s | 406, 49 s | 458, 27 s |

Free shots take the rush from 16 losses to 15 games at the cap with the mould cut from 5,000 walkers to under 600 on 12 of them; it never wipes the mould, probably because the last stragglers lay too little trail to stop a shot (not yet checked). The gate holds. Scent 4 brings the mould to the squad sooner but makes random play stronger.

**Cost** (of the opt-in). The alarm field is one 3 x 3 mean over the agar a fine step, and one more read at each sensor. On the CPU it first added 2.0 ms to a 5.4 ms game tick (farm seed 1, 120 s, one thread, three rounds each: 5.37/5.42/5.34 against 7.33/7.77/7.39 ms). The mean is now branchless (`spread_rows`: each neighbour adds its value times its agar mask, in the old order, so the result is bit for bit the old one; test `the_branchless_spread_matches_the_first_one_bit_for_bit`), which takes one pass on the race world's 512 x 320 grid from 0.65 to 0.24 ms on one thread (best of 30, interleaved); the germ's spread gets the same. On the GPU the field has its own switch (`alarm_on`), apart from the life states (`cyc`): the flee runs no `fate`, `fly` or `alight`, only the mean inside the scent pass, the sensor read and the one-workgroup `shout` on frames with shots, and the Larder World already runs the outside pipelines for growth. The life-gpu report's software-Vulkan run had the Swarmer (the alarm alone) at 38-39 ms a frame against 40 for the outside pipelines with every rule off; the GPU thread's one pair on the Larder World at 5,000 walkers was 15.16 ms a step without it and 15.64 with it. The GTX 1070's figure is open.

## The hole under fire: shots lay the crater, 2026-11-12

**Why.** The owner's bar (04:39Z, above): "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at". The flee made that hole and failed the gate, so this try closes the ground instead of moving the mould: no change to where the mould goes anywhere else.

**What is built** (PR #113).
- **Each hit lays the squad toolkit's crater** (`BigDish::scorch`, the one crater field, [squad toolkit](squad-toolkit-2026-11-12.md)) on the agar cells of its strike disc, for `Rules::crater` game seconds: 10 by default, and 0 is off.
  - `squad::Frame::crater` carries it in fine steps (`World::fine_steps`).
  - A cell keeps `scorch_keep` of its trail each fine step while its countdown is fresh, so the mould cannot refill it.
  - A shot stops at the first cell that reads as slime, so the next shots fly over the crater into the body behind it, and steady fire digs in.
  - When firing stops, the countdowns run out and the mould grows back.
  - Nothing is drawn: the hole is the trail that isn't there.
- **Shared defaults** `scorch_keep` 0.1 and `scorch_full` 30 (were 0.5 and 300).
  - The toolkit thread agreed (06:57Z).
  - Their probes found blasts and the det cord no worse: a blast's disc is the same at 20 s, and the cord's trail is 67 against 137.
- **GPU**:
  - Binding 47 is a 2-channel f32 damage memory. Block 0 is the crater countdown; block 1 is kept for the tiers thread's scar.
  - The Params row at `CRATER_AT` 864 holds [scorch_keep, scorch_full, crater steps, scorched], so `PARAMS_BYTES` is 880 and `BINDINGS` 48, as the integration thread assigned.
  - The `cut` pass lays the crater on the struck agar cells. The `trail` pass cuts the trail and each species' field under it, and counts down.
- **Tools**:
  - `larder-world crater=S`
  - `larderworld crater= scorch_full= scorch_keep=`
  - `larderhole crater:S[:F[:K]]`

**Deciding measures, named before the runs.**
- For the hole: the hole test's hole after 20 s of fire, each arm paired against no crater by seed, sign test, Holm over the arms.
- For the default: the census gate, each plan against idle and random, with random's mould wipes beside it. This is the test the flee failed.

**The hole** (seeds 1-8, the hole test of *Playtest fixes*; raw files [crater/hole-1-8-keep05-full300.txt](larder-world-roadmap-2026-11-11/crater/hole-1-8-keep05-full300.txt) at the toolkit's first keep and full, and [crater/hole-1-8-full30-keep01.txt](larder-world-roadmap-2026-11-11/crater/hole-1-8-full30-keep01.txt)). Medians:

| crater (keep, full) | hole at 5 / 10 / 20 s of fire | hits land from the squad | left 5 s after the last shot | killed in 20 s | against none at 20 s |
| --- | --- | --- | --- | --- | --- |
| none | 42 / 33 / 29 | 3.8 dish cells out | 98% | 163 | |
| 1 s (0.5, 300) | 36 / 38 / 31 | 4.3 | 90% | 154 | 5 larger, 2 smaller, +2, p 0.45 |
| 3 s (0.5, 300) | 41 / 42 / 41 | 4.6 | 88% | 269 | 6-2, +5, p 0.29 |
| 10 s (0.5, 300) | 50 / 55 / 54 | 6.4 | 70% | 670 | 8-0, +15, p 0.008, holds |
| 10 s (0.5, 30) | 50 / 49 / 72 | 6.4 | 75% | 786 | 8-0, +19, p 0.008, holds |
| **10 s (0.1, 30), the default** | **47 / 55 / 67** | **9.7** | **77%** | **791** | **8-0, +28, p 0.008, holds** |
| 30 s (0.1, 30) | 47 / 55 / 50 | 10.0 | 98% | 431 | 6-2, +10, p 0.29 |

- With the crater the hole grows under fire, and the hits land 10 dish cells from the squad instead of 4: without it the mould closes up to the squad as fast as it is shot. (Before the walker stop, below; with it the hits land 5.3 dish cells out against 3.8, and the hole is 60 against 29.)
- The hole closes over about 10 s once firing stops: at 30 s it is back to 30 cells, against 26 without.
- Craters of 30 s make a smaller hole. Once the near rim is empty, more shots fly long, and kills halve.
- Of 32 soldiers, 31 lived through the default arm (32 without craters).
- Seed 1 is the weak case: 43 against 40 cells at 20 s. There the craters let the hits land 13.5 dish cells from the squad, against 4.1, and kill 633 walkers against 163, most of them past the hole the count measures ([crater/clips-s1.txt](larder-world-roadmap-2026-11-11/crater/clips-s1.txt), [crater/clips-s3.txt](larder-world-roadmap-2026-11-11/crater/clips-s3.txt)).

**In the window** ([page](https://claude.ai/artifact/AcRm3aeB97YLpWobtmABhs), `page-crater.html`; [crater/window-counts.txt](larder-world-roadmap-2026-11-11/crater/window-counts.txt)).
- On the Fork, the rush hand at 13 s killed 1,098 walkers with craters against 974 without, in 31 shots each.
- The window shows each crater's hole as the strike's round disc. The red-brown inside a fresh hole is bare ground: the Fork map has a clay disc round the mould's start, and the view draws clay in that colour (`CLAY_RGB`, 104, 54, 38). The graphics thread checked it by drawing every cell with drawn thickness under 0.02 in debug green; the crater insides and the older, grey holes outside the clay both came out green. (This report first put the colour down to the crystal's scent, which was wrong.)
- **A ragged edge was tried and dropped.** Each cell got fewer steps toward the disc's rim, and a hashed share of them (the same whole numbers on both engines; the GPU took the longest crater a cell was given with an atomic max). In the window the holes came out smaller but no less round, because the trail's spread smooths cell-scale noise away. An edge irregular at the disc's own scale (a strike shape that isn't a disc) was not tried.

**Three looks for the owner, blind** (2026-11-12, after the game designer and the graphics thread read the round holes as stamps). The graphics thread's render named three causes, all from the rule's shape: every hole is the strike's same disc; `scorch_keep` 0.1 at full count drops the trail across one cell at the disc's edge, which the lighting draws as a dark rim; and the inside reads as a third flat colour. Two switches were added, both off by default, so the owner can pick from pictures:
- **`Rules::spread`** (dish cells at the target): each shot's line turns by a uniform angle that moves its aim up to that far either side. It draws a random number only when it is above 0, so default games are unchanged (the replay test passes). With a strike of 1.5 instead of 2.5, the fire rate was matched to the shipped crater's kills: one shot every 0.36 s killed 966 in 20 s against 741, 0.5 s killed 583 and 0.44 s killed 723 ([crater/hole-1-8-aim-rate.txt](larder-world-roadmap-2026-11-11/crater/hole-1-8-aim-rate.txt)). The hole is a union of offset bites, irregular at the disc's own scale, which is the re-test condition of the ragged-edge dead end.
- **`BigParams::scorch_cone`**: a shot lays `steps - steps * d² / (r² + 1)` on a cell at squared distance d² from the hit, in whole numbers on both engines (the GPU frame twin matches it to the bit). With `scorch_full` 150 the bite falls from full in the middle to little at the rim, and with `scorch_keep` 0 the inside holds no trail. Blasts keep their flat disc. The GPU reads it from bit 1 of the crater row's flag word, so no binding or Params row moved.

The hole test on seeds 1-8 with the walker stop ([crater/hole-1-8-looks.txt](larder-world-roadmap-2026-11-11/crater/hole-1-8-looks.txt)), medians:

| look | hole at 5 / 10 / 20 s of fire | hits land | 5 s after the last shot | killed in 20 s | at 20 s against the shipped crater |
| --- | --- | --- | --- | --- | --- |
| shipped crater (strike 2.5, a shot a second) | 48 / 50 / 60 | 5.3 | 79% | 741 | |
| scattered (strike 1.5, a shot every 0.44 s, spread 2.5) | 45 / 50 / 56 | 5.8 | 69% | 723 | 4 larger, 3 smaller, +4, p 1 |
| cone (`scorch_full` 150, `scorch_keep` 0) | 48 / 52 / 54 | 6.1 | 74% | 702 | 2 larger, 5 smaller, -3, p 0.45 |
| cone (`scorch_full` 300, keep 0) | 41 / 47 / 51 | 5.8 | 85% | 692 | 3-5, -3, p 0.73 |
| cone (`scorch_full` 300, keep 0.1) | 42 / 45 / 50 | 5.9 | 73% | 700 | 0-7, -4, p 0.016 (not under Holm over 6) |
| no crater | 42 / 33 / 29 | 3.8 | 98% | 163 | 0-8, -9, p 0.008, holds |

None of the looks changes the fight beyond chance, so the choice is the owner's eye. In the window (the Fork, seed 1, rush hand) the scattered holes come out as chains of small bites and the cone's as smaller holes with a soft shadow in place of the dark rim. In all three the fresh hole's inside is red-brown because the fight is on the Fork's clay disc; holes off the clay are grey-olive like a natural hole. The graphics thread rendered all three up close at zoom 6 and 16 (`/mnt/project-files/crater-ab/`). The page shows them blind as looks 1-3; the key is [crater/look-key.md](larder-world-roadmap-2026-11-11/crater/look-key.md), with the window's counts in [crater/look-window-counts.txt](larder-world-roadmap-2026-11-11/crater/look-window-counts.txt).

**The looks in motion** (2026-11-12 14:40Z; the owner: "I need to see videos/gifs of the craters to tell"). The same three looks and the game without craters, rendered as clips: the Fork, seed 1, rush hand, every game tick from 7 to 19 s, from a still camera (the look mode's new `centre=X,Y`, so the view no longer pans with the squad or jumps to the larder when it dies). The four play in step on [Crater Looks in Motion](https://claude.ai/artifact/HzgDT3XPLX59Ba621tVH2H), with each frame's counts under its clip, at 1×, ½× or ¼×. These runs include the core's pull, so the squad dies at about 16 s (17-19 s with `core_pull=0`), and the last 3 s show each hole after the fire stops. Clips, posters and the runs' t lines are in [crater/clips/](larder-world-roadmap-2026-11-11/crater/clips/L2.txt); same key as the stills. The owner (14:41Z): "Why do the craters happen under the players feet. I think 3 is best, but I was more expecting soliders standing shooting into slime. Not running through slime leaving craters under their feet." The rush walks the squad into the body, and a shot stops at the first slime on its line, which inside the body is beside the soldier. So the page now leads with a `stand` hand (attack-move on the core: the squad stops where the mould comes in range and fires from there), 22-40 s from `centre=254,264`. Killed by 40 s: 449, 436 and 569 for looks 1-3, against 242 with no craters (one seed). Look 3, the cone, is the owner's pick.

**The gate, first look** (race seeds 5-12, 15-minute games, crater 10; raw [crater/census-race-5-12.txt](larder-world-roadmap-2026-11-11/crater/census-race-5-12.txt)):
- farm, wall and strike each beat idle and random 8-0, and all hold under Holm.
- Random wiped no mould, and its squad was wiped on 4.
- Strike no longer beats every plan: it loses to farm 3-5, against 28-4 on 32 seeds without craters, because farm and wall now dig into the mould around their crystals too.
- The 32-seed gate is below.

**The first full gate, and the walker stop** (race 5-36, the Fork and the Rim 5-20, at 189ab71; raw `crater/*-first.txt`).
- Every plan (farm, wall, fort, strike) beat idle 32-0 and random 24-8 to 26-6, all under Holm, and strike no longer beat farm (12-20, against 28-4 without craters).
- But the rush hand flipped to losing: 8-24 against idle, its squad wiped on 22 of 32 seeds (7 without craters), and idle's squad was wiped on 7 (0 without).
- The cause: a crater holds no trail, so a shot flew over the walkers standing in it and the mould closed on the squad from inside its own hole.
- **The fix (db5640f)**: a cell under a crater that holds a living walker stops a shot, as slime does (`squad::shots`, the CPU's occupancy; the GPU's `shots` pass reads the occupancy atomically, gated by the crater flag). With it the rush hand's squad was wiped on 3 of seeds 5-20 ([crater/rush-5-20-walker-stop.txt](larder-world-roadmap-2026-11-11/crater/rush-5-20-walker-stop.txt)).
- The hole test with the stop ([crater/hole-1-8-looks.txt](larder-world-roadmap-2026-11-11/crater/hole-1-8-looks.txt), arms `base` and `crater:0`): a median hole of 60 dish cells at 20 s of fire against 29 without craters (8 of 8 larger, median paired difference +9, p 0.008, holds), hits landing 5.3 dish cells out against 3.8, 741 killed against 163, all 32 soldiers alive. The stop costs depth (hits landed 9.7 out before it) but not the hole.

**The gate at the walker stop** (db5640f, 15-minute games; raw [crater/census-stop-race-5-20.txt](larder-world-roadmap-2026-11-11/crater/census-stop-race-5-20.txt), [-21-36](larder-world-roadmap-2026-11-11/crater/census-stop-race-21-36.txt), [fork](larder-world-roadmap-2026-11-11/crater/census-stop-fork-5-20.txt), [rim](larder-world-roadmap-2026-11-11/crater/census-stop-rim-5-20.txt); runners 1-4, sha256 checked):

| race 5-36 (32 seeds) | against idle | against random | ends |
| --- | --- | --- | --- |
| farm | 32-0, holds | 24-8, p 0.007, holds | 32 at the cap |
| wall | 32-0, holds | 24-8, p 0.007, holds | 32 at the cap |
| fort | 32-0, holds | 24-8, p 0.007, holds | 32 at the cap |
| strike | 32-0, holds | 26-6, p 0.0005, holds | 28 at the cap, 4 moulds wiped |
| rush | 20-12, p 0.22 | 17-15, p 0.86 | 9 squads wiped (22 at the first gate, 7 without craters), 1 mould wiped |
| idle | | | 2 squads wiped (7 first, 0 without) |
| random | | | 20 squads wiped, **7 moulds wiped** (3 first, 2 without) |

- Plan against plan (Holm over 6): strike against farm 20-12 (p 0.22) and against wall 18-14 (p 0.60), so strike no longer beats every plan (28-4 against farm without craters); strike, farm and wall each beat fort 29-3 or more; farm and wall tie 15-16.
- Random wiped the mould on 7 of 32 race seeds, against 2 without craters: by seed 5-0 (14, 17, 24, 28, 29 only with craters), p 0.06, not significant but the trend the designer warned of (a rule that weakens the mould's defence makes the core a free win). The plans still beat random 24-8 or better. The integration thread's standing gate reports random wiping the plain mould on 5 of 16 of seeds 201-216 without craters, so the race block 5-36 was the easy one for the mould.
- **The Fork** (5-20): farm and wall beat idle and random 16-0 (hold); random wiped no mould; the rush and idle squads were wiped on 16 of 16 (15 each without craters).
- **The Rim** (5-20): farm and wall beat idle and random 16-0 (hold); rush's squad was wiped on 16 of 16 (16 without); random wiped no mould and lost its squad on 14 (10 without).
- So the crater ships on by default (`Rules::crater` 10), with the second block below as its watch item.

**The second seed block: with craters, no plan beats random after Holm** (race 201-232, at 3849a6a, `BuildRules::raise` 0, before #122; raw [crater/census-crater-race-201-208.txt](larder-world-roadmap-2026-11-11/crater/census-crater-race-201-208.txt), [-209-216](larder-world-roadmap-2026-11-11/crater/census-crater-race-209-216.txt), [-217-224](larder-world-roadmap-2026-11-11/crater/census-crater-race-217-224.txt), [-225-232](larder-world-roadmap-2026-11-11/crater/census-crater-race-225-232.txt), and the no-crater control `crater=0` for idle, random and farm, [crater/census-nocrater-race-201-208.txt](larder-world-roadmap-2026-11-11/crater/census-nocrater-race-201-208.txt), [-209-216](larder-world-roadmap-2026-11-11/crater/census-nocrater-race-209-216.txt), [-217-224](larder-world-roadmap-2026-11-11/crater/census-nocrater-race-217-224.txt), [-225-232](larder-world-roadmap-2026-11-11/crater/census-nocrater-race-225-232.txt); runners 1-4, sha256 checked):

| race 201-232 (32 seeds) | against idle | against random | ends |
| --- | --- | --- | --- |
| farm | 32-0, holds | 21-11, p 0.11 | 32 at the cap |
| wall | 32-0, holds | 21-11, p 0.11 | 32 at the cap |
| fort | 32-0, holds | 20-12, p 0.22 | 31 at the cap, 1 mould wiped |
| strike | 32-0, holds | 23-9, p 0.02 (not under Holm over 10) | 28 at the cap, 4 moulds wiped |
| rush | 22-10, p 0.05 | 12-20, p 0.22 | 8 squads wiped, 2 moulds wiped |
| random | | | 14 squads wiped, **11 moulds wiped**, 7 at the cap |
| farm, no craters | 32-0, holds | **25-7, p 0.002, holds** | 32 at the cap |
| random, no craters | | | 20 squads wiped, 5 moulds wiped, 7 at the cap |

- On 5-36 every plan beat random 24-8 or better under Holm. On 201-232 every plan still beats random on count, but none survives the correction. Without craters farm beats random 25-7 and holds, so the craters cost the gate's margin over random on this block.
- Random plays a coin flip: it wipes the mould (between 248 and 846 s, scoring 1.3 to 1.9) or loses its squad (scoring about -1.5). Every seed on which random beats a plan is one of its mould wipes; the plans never go for the core (strike wipes it on 4).
- Craters cause most of those wipes: 7 seeds are wiped only with craters (201, 203, 204, 205, 208, 214, 223) and 1 only without (218), p 0.07. Paired on and off, the craters raise random's score on 27 of 32 seeds (p 0.0001, holds) and farm's on 26 (p 0.0005, holds). Holes that don't refill let a squad clear the core and hold it for `core_secs`, so births stop.
- Both blocks' wipe counts: 5-36, 7 of 32 with craters and 2 without; 201-232, 11 with and 5 without.
- Ruling (integration thread with both reviewers, 11-12 11:16-11:17Z): merge with craters on, since the owner asked for this hole and the condition (random beating a plan on count) was not met; core defence is the next PR. Its bar (the game designer): a deliberate core-attack hand beats random on win rate and mean score; random's core wipes fall to about 1 in 10; a failed core attack costs the squad, so it loses to farm and wall where it misses; and the core depends on the tier. The technical lead's gate for it: every plan over random after Holm on both blocks.

**The building hands at #122's build time** (99c5e29, main through #123, `BuildRules::raise` 8; raw [crater/census-raise8-race-5-20.txt](larder-world-roadmap-2026-11-11/crater/census-raise8-race-5-20.txt), [-21-28](larder-world-roadmap-2026-11-11/crater/census-raise8-race-21-28.txt), [-29-36](larder-world-roadmap-2026-11-11/crater/census-raise8-race-29-36.txt), [fork](larder-world-roadmap-2026-11-11/crater/census-raise8-fork-5-20.txt), [rim](larder-world-roadmap-2026-11-11/crater/census-raise8-rim-5-20.txt); this thread and runners 1-4, sha256 checked). #122 is a census reset, and the gate above ran at raise 0, so the hands that build (wall, fort, and random, which builds at random) were re-run beside farm and idle:
- Race 5-36: farm, wall and fort each beat idle 32-0 and random 25-7 (p 0.002), all under Holm over 6. Random wiped the mould on 7 of 32 seeds, as at raise 0.
- Farm and idle build nothing and play the same games to the digit. Wall scores a little lower than at raise 0 (lower on 22 of 32 seeds, median -0.016, p 0.016, not under Holm over 5) and fort a little higher (22-10, p 0.05). Wall no longer beats fort by as much (21-11, p 0.11, against 29-3); farm beats fort 27-4.
- The Fork and the Rim (5-20): farm and wall beat idle and random 16-0 (hold). Random wiped no mould on either and lost its squad on all 16 on both.

**Cost.** On software Vulkan the crater adds 0.03 ms a step (13.54/13.56 against 13.60/13.57 ms). The GTX 1070's figure is open.


## Core defence: the body comes for a squad at its core, 2026-11-12

**Why.** With craters on (#113) the random hand wiped the plain mould on 11 of race seeds 201-232 and no plan beat it after Holm. The integration thread and both reviewers ruled (11:16-11:22Z) that the craters stay on and core defence comes next, ahead of the economy. The game designer's bar: a deliberate core attack beats random; random's core wipes fall to about 1 in 10; a failed attack costs the squad; the core depends on the tier. The technical lead's gate: every plan beats random after Holm on both seed blocks.

**Looked first** (race seed 204, the random hand, frames every 5-10 s; [core/hub-s204-counts.txt](larder-world-roadmap-2026-11-11/core/hub-s204-counts.txt)). The race world's starting disc of 5,000 walkers settles to about 2,000 by 30 s and then grows (idle reaches 8,582 by 330 s). Random's soldiers, sent one at a time on Attack, walk into the hub in the first minute and fire into its densest walkers: 1,187 killed by 60 s, while the holes round them don't refill and the body never comes for them. The mould falls to 80 walkers by 240 s, the core falls at 244 s and the last walker dies at 302 s. Farm and strike fight at the network's edge near the larder and leave the hub alone, so their moulds grow (to 4,664 and 5,038 by 330 s).

**What.** `larder_world::Core { kind, secs, pull }`, the tiers thread's shape, read once by `Game::with_tier` from `Tier::core` (None for every tier for now; the tiers thread fills its own). The plain mould's is `Core::of(&rules)`: fixed, `Rules::core_secs`, and the new `Rules::core_pull`, default 8.
- **The pull**: a soldier within twice `core_r` of the core lays `pull` times its own scent over a lure's reach (`BuildRules::lure_r`), so the body is drawn onto a squad at its core through the scent it already follows. Nothing is drawn, and no new GPU row: the soldier's scent goes through the squad frame on both engines. 8 is a lure's own strength (`lure_smell`).
- **`CoreKind::Headless`**: nothing to hold, so only a wipe ends the mould (for the Sprawler). The Trunk's regrowing core and the Drifter's migrating one need births to restart or move on both engines, and a migrating core needs to know where the body is on the GPU, so they come later.
- Census rows now say when the core fell (`core fell at T s`).

**In play** (for the owner to judge as a behaviour, not only as a gate fix). The mould now rallies to its core when it is touched there. The hub stays quiet while soldiers work the network's edge. Once one walks within two core radii (10 dish cells) of the core, the walkers there turn and pour onto it, and the fight moves to the soldier. On seed 204, random's seven soldiers walk into the hub at about 40 s:
- **With the pull**, the body closes on them. Three die between 45 and 50 s, the killing stops at about 1,290 walkers, and the mould holds at about 820, then grows again.
- **Without it**, all seven keep firing (1,392 killed by 75 s). The mould dwindles until the core falls at 244 s.

A lone soldier, or a trickle of them, now dies at the core; only a squad that brings enough fire at once (strike) takes it. This is the owner's playtest point 3 ("the slime mostly ignored my soldiers") answered in one place, the core, not everywhere. Answering it everywhere was the flee rule's failure, which handed random the core.

A note on the constants. Tonight's pattern was a feel rule that helped random (the flee, then the crater), followed by a counter-rule, and each counter adds a number every later census leans on. This one is a new key, `Rules::core_pull`, but its value is a lure's own strength over a lure's own reach (`lure_smell` 8, `lure_r` 6), so it adds no new magnitude. `core_pull=0` turns it off.

**The strength** (seeds 201-208, random, strike and rush; raw [core/sweep-pull0-201-208.txt](larder-world-roadmap-2026-11-11/core/sweep-pull0-201-208.txt) and the `sweep-pull4`, `-pull8`, `-pull16` files):

| pull | random: core taken, mould wiped | strike: core taken, mean score | rush: core taken |
| --- | --- | --- | --- |
| 0 | 4, 4 | 3, +0.755 | 6 |
| 4 | 2, 1 | 3, +0.961 | 6 |
| **8** | **0, 0** | **3, +0.744** | 6 |
| 16 | 1, 1 | 2, +0.578 | 6 |

**The census** (both seed blocks, seven hands, 15-minute games; the baseline is the same code at pull 0, 089167b, main through #124 with craters and #122's build time; the pull at 85b56fb with `core_pull=8`; raw [core/census-core0-race-*.txt](larder-world-roadmap-2026-11-11/core/census-core0-race-5-20.txt) and [core/census-pull8-race-*.txt](larder-world-roadmap-2026-11-11/core/census-pull8-race-5-20.txt), each block in two files; runners 1-4, sha256 checked):

| | race 5-36, pull 0 | race 5-36, pull 8 | race 201-232, pull 0 | race 201-232, pull 8 |
| --- | --- | --- | --- | --- |
| farm, wall, fort against random | 25-7, hold | 30-2, hold | 19-13, 19-13, 20-12 | 29-3, hold |
| strike against random | 27-5, holds | 31-1, holds | 22-10, p 0.05 | 29-3, holds |
| rush against random | 19-13 | 24-8, holds | 15-17 | 25-7, holds |
| every plan against idle | 32-0, hold | 32-0, hold | 32-0, hold | 32-0, hold |
| random takes the core | 7 | 2 | 11 | 3 |
| **random wipes the mould** | **7** | **2** | **11** | **3** |
| random's squad wiped | 23 | 29 | 17 | 26 |
| strike takes the core / wipes the mould | 7 / 4 | 5 / 4 | 6 / 4 | 7 / 5 |
| rush takes the core / squad wiped | 18 / 9 | 12 / 13 | 19 / 8 | 22 / 8 |

- **Every plan beats random after Holm on both blocks** (Holm over 10), the technical lead's gate; on 201-232 none did without the pull.
- **Random's core wipes: 5 of 64** (18 without), 1 in 13. Paired on and off (Holm over 7), random scores lower on 27 and 28 of 32 (both hold), and more walkers stand near its soldiers (201-232: 24 up, 7 down, p 0.003, holds): the mould comes for them.
- **Farm, wall, fort and idle play the same within chance** (idle identically: its soldiers never near the core).
- **The planned attack still lands**: strike takes the core on 12 of 64 (13 without) and scores higher on 45 of 64, 17 lower (201-232: 24-6, p 0.001, holds).
- **A failed attack costs the squad**: on 201-232 the rush failed to take the core on 10 seeds and lost its squad on 8 of them; on 5-36 it failed on 20 and lost the squad on 12.
- **Not met**: strike doesn't pay for a failed attack, since it farms behind it and rebuys to 30 soldiers; it beats farm 22-10 and 21-10 (p 0.05 and 0.07) and wall 26-6 on both blocks (holds; 21-11 and 20-10 without the pull). Food still doesn't bind, which is the economy PR's job (PR 4: running costs).
- **The core depends on the tier**: `Tier::core` is in; the tiers thread fills its values (the Sprawler headless, the Grudge's pull) after this merges.

**The maps** (the technical lead's ask: the Fork and the Rim, seeds 5-20, the same seven hands at ecaa5d2 with `core_pull=8` and `0`; raw [core/census-pull8-fork-5-20.txt](larder-world-roadmap-2026-11-11/core/census-pull8-fork-5-20.txt) and the `core0-fork`, `pull8-rim` and `core0-rim` files; runners 1-4, sha256 checked):
- **Random never takes the core or wipes the mould on either map**, with or without the pull (0 of 32 both ways); its squad is wiped on all 32.
- **Rim**: every plan beats random after Holm (over 10) either way: farm, wall, fort and strike 16-0, rush 15-1 with the pull and 16-0 without.
- **Fork**: farm, wall, fort and strike beat random 16-0 either way. **The rush loses to random 16-0 with the pull and 15-1 without** (both hold): on the Fork the rush's squad dies at the core on all 16 seeds whatever the pull, so the gate's "every plan beats random" fails there for the rush, and the map, not the pull, is why. Strike wipes the mould on 16 of 16 (15 without), takes the core on 7 (10), and scores higher with the pull on 12 seeds, lower on 1 (p 0.003, holds, median +0.003).
- Paired on and off (Holm over 7), nothing else changes on either map: farm, wall, fort and idle play identically, since their soldiers never stand at the core.

So the pull does no harm off the race world. Strike's dominance on the Fork is the economy PR's to answer, with the tiers' cores.

**Cost.** No new pass: a pulling soldier lays its scent over a 12-cell disc instead of 4 (about 450 fine cells a frame each, a handful of soldiers at most), on both engines. The GPU twin (`a_squad_at_the_core_draws_the_mould_on_the_engine_as_on_the_cpu`, seed 3, 30 s of attack at pull 8) kills 1,509 on the CPU and 1,527 on the GPU (1,224 without the pull). Its moulds at 30 s are 11% apart (730 and 815 walkers), so the bound is 8% or 100 walkers. The technical lead asked whether that gap is chaos or a rule the engines play differently, so `twin_gap_over_seeds_is_chaos` (ignored, about 200 s) plays seeds 3-10, each on the CPU, on the GPU, and on the CPU again with one walker more (5,001, same map):
- **At pull 8**: the mean gap from the CPU at 30 s is 176 walkers on the GPU and 213 with one walker more. The GPU is above the CPU on 4 seeds and below on 4. Mean kills are 1,469 on the CPU, 1,514 on the GPU and 1,554 with one walker more.
- **At pull 0**: the gaps are 107 and 164, again 4 above and 4 below. Seed 7 is 35% apart (1,044 walkers on the CPU, 1,408 on the GPU), so seed 3 passing the 8% bound at pull 0 is luck.

At 15 s, before the fire, all are within 70 walkers. The gap grows as the squad shoots the mould down, and it is no bigger than the CPU's own spread.

**Pictures**: the hub on seed 204 at 40, 45, 50 and 60 s without and with the pull ([core/hub-s204-off-40s.jpg](larder-world-roadmap-2026-11-11/core/hub-s204-off-40s.jpg) to `-on-60s.jpg`), on the crater page.

## Wading, and the cone crater by default (PR 3b), 2026-11-12

**Why.** The owner, after the clips (14:41-14:43Z): look 3 is the pick, and "soldiers should be heavily slowed down if they try to run into the thick slime". Also "shouldn't be able to stand on top of the crystal", which the controls thread builds. Then (15:18Z): "Is there an option between 3 and no craters? 3 fills in too slow still".

**What.**
- **The cone crater is the race world's default** (`race_params`: `scorch_cone` on, `scorch_full` 150, `scorch_keep` 0; `race_settings` carries them to the GPU). The toolkit's blasts share `scorch_full` and `scorch_keep` and stay flat discs. `scorch_cone=0 scorch_full=30 scorch_keep=0.1` plays #113's flat disc.
- **Wading** (`Rules::wade` 6): a soldier's step is `speed / (1 + (touching / wade)²)`. Two walkers touching leave it 90% of its speed, 6 leave half, and 12 a fifth (a rush meets 0 to 26 on race seeds 1-4). The count is the frame's touch read-back, the same on both engines, so the GPU needs nothing new. 0 is off. Test: `thick_slime_slows_a_soldier`.
- **In play**: the page's *Wading* clips (the Fork, seed 1, rush) and its crater-time clips were rendered by a window build from before the cone reached the GPU's `race_settings`, so they show #113's flat crater, not the cone (found 11-12 17:03Z: that build's t lines match today's build with `scorch_cone=0 scorch_full=30 scorch_keep=0.1` to the digit, and differ from it without). The census above ran the CPU's `race_params`, which had the cone. The clips are being rendered again with the cone (17:05Z).

**The crater's time** (the owner's 15:18Z ask): crater times of 10, 6 and 3 s are on the page, but as flat craters (see *In play*), so their kills are not look 3's. The cone at 10, 6, 3 and 1 s, and a scene where the squad fires into the thick ring on the core crystal, follow in the next PR. Both shorter times were being gated (runners 1-4, `crater=6` and `crater=3` at 90ca407) until the owner's testing rule stopped those runs (17:26Z: a census is not a gate for a look).

**The gate** (90ca407, both seed blocks, seven hands, 15-minute games; raw [wade/](larder-world-roadmap-2026-11-11/wade/census-wade6-race-5-20.txt), runners 1-4, sha256 checked). Three arms: the new defaults (wade 6, cone), `wade=0` (cone only), and `wade=0` with the flat crater (#126's rules; its random wipes match #126's 2 and 3 exactly).

| | race 5-36: wade 6, cone | wade 0, cone | wade 0, flat (#126) | race 201-232: wade 6, cone | wade 0, cone | wade 0, flat (#126) |
| --- | --- | --- | --- | --- | --- | --- |
| farm, wall, fort against random | 31-1, hold | 29-3, hold | 30-2, hold | 29-3, hold | 27-5, 27-5, 26-6, hold | 29-3, hold |
| strike against random | 32-0, holds | 32-0, holds | 32-0, holds | 30-2, holds | 30-2, holds | 31-1, holds |
| rush against random | 28-4, holds | 22-10, p 0.05 | 24-8, holds | 26-6, holds | 24-8, holds | 25-7, holds |
| farm, wall, fort, strike against idle | 32-0, hold | 32-0, hold | 32-0, hold | 32-0, hold | 32-0, hold | 32-0, hold |
| **random wipes the mould** | **1** | 3 | 2 | **3** | 5 | 3 |
| random takes the core | 1 | 3 | 2 | 3 | 5 | 3 |
| rush's squad wiped | 11 | 14 | 13 | 10 | 10 | 8 |

- **Every plan beats random after Holm on both blocks** with the new defaults (Holm over 10), the rush included, which it did not on 5-36 with the cone alone.
- **Random wipes the mould on 4 of 64** (5 at #126, 8 with the cone and no wading).
- **The cone changes nothing beyond chance** (cone against flat, both at wade 0, Holm over 7): no hand's score moves on either block (every p 0.07 or more). It is a look, as the hole test said.
- **Wading mostly slows random** (wade 6 against 0, Holm over 7): random kills fewer walkers on 5-36 (27 lower, 5 higher, median -524, p 0.0001, holds; 21-11 on 201-232, p 0.11) and scores lower there (22 lower, 9 higher, p 0.03, not under Holm). The rush kills more on 201-232 (24 higher, 7 lower, p 0.003, holds). No plan's score moves.
- **Against #126** (wade 6 and cone against wade 0 and flat): random scores lower on both blocks (23 lower and 8 higher, p 0.01, not under Holm; 24 and 7, p 0.003, holds) and kills fewer on both (25-7 and 28-4 lower, hold). Farm leaves more mould on 5-36 (24 more, 8 less, p 0.007, holds), and no plan's score moves.
- **Not met, as before**: strike beats farm 26-6 and 28-4 and every other plan (all hold), since food does not bind. That is PR 4's.

**Cost.** Wading is one division a soldier a tick, on the CPU, from a count both engines already read back. The cone is the toolkit's crater mode, already timed: 0.03 ms a step on software Vulkan.

## A core that grows back, and the crater's time in the cone, 2026-11-12

**Why.** The tiers thread asked for the Trunk's core before the Drifter's migrating one (11-12): the game designer's "regrows unless the body is starved", the lesson of campaign mission 4 (cut, then starve). With `CoreKind::Fixed`, a squad that holds the core for 5 s ends the mould's births for good, and on the race world that is the only way any plan ends a game (integration, 17:10Z, from the one-body thread's review). The owner, on the crater clips (16:55Z): "show me 1 second. also in these videos they were mostly shooting at individuals so it was hard to evaluate until the end when they shot at the core of slime".

**What.**
- **`CoreKind::Regrow { starve_secs }`**: the core falls as a fixed one does, and births stop. If the mould eats anything from any crystal in the `starve_secs` after the fall, the core grows back when they are up: births restart and the hold clock starts again. If it eats nothing all that time it stays down for good. So taking the core alone doesn't stop a Trunk; taking it while the squad holds the crystals, or has cut the body off them, does.
- **"Eats" is the whole body's** (`Game::mould_ate`: the crystals' charge gone less the squad's take), which both engines already read back. The core's own piece starving would need a per-region read-back on the GPU; the tiers thread chose the whole-body rule. With `BigParams::crystal_regrow` on, regrowth hides eating (off on every map so far).
- **`World::stop_births` returns what it stopped** (the birth energy, or the Garden's cap) and `World::start_births` puts it back, on the CPU, the GPU world and the player's Garden. `CoreKind` is no longer `Eq` (it holds an `f32`).
- **Off unless a tier sets it** (`Tier::core`); the tiers thread sets each tier's `starve_secs`. Census keys `regrow=S` and `core=fixed|headless` set the core over the tier's; a row adds `core grew back N`. The window's HUD says when the core is taken ("it grows back in N s unless the mould eats nothing till then") and when it grows back.
- Test: `a_regrowing_core_grows_back_unless_the_mould_starves` (a fed mould's core grows back and births restart; a starved one's stays down).

**Dev census** (b0538ae with the change, race seeds 1-6, five hands, 15-minute games, `regrow=30` against the fixed core; raw [regrow/](larder-world-roadmap-2026-11-11/regrow/census-regrow30-race-1-6.txt)). Only the rush takes the core on these seeds, on 4 of 6. Every other hand's scores are identical.

| rush, seed | 1 | 2 | 3 | 4 | 5 | 6 |
| --- | --- | --- | --- | --- | --- | --- |
| fixed core: end, mould left | core fell 90 s, 12 left | fell 69 s, 2 left | never fell, 9,991 | fell 88 s, 19 left | squad wiped 288 s | fell 56 s, 5 left |
| `regrow=30`: end, mould left | grew back 22 times, 2,745 | grew back 22 times, 2,378 | the same | grew back once, 18 | the same | grew back once, squad wiped 558 s |

- Under the fixed core, a rush that takes the core has won in all but name: the mould dwindles to 2-19 walkers by the cap with no births. Under `regrow=30` the rush on seeds 1 and 2 sits on the core and takes it again about every 38 s while the mould keeps eating, and the mould ends at 2,400-2,700 walkers.
- Rush scores lower on 3 of 6 (none higher, median -0.08, p 0.25) and kills more on 4 (p 0.13). Too few seeds to say more; the tiers thread tunes `starve_secs` per tier on its own census.

**Does the core rule cause the stalemates?** (integration, 17:10Z, from the one-body thread's review: "a squad holding the core for 5 s ends the whole mould, and that is the only way any plan ends a game"; the owner has the question open.) `core=headless` against the fixed core, race seeds 5-20, seven hands, 15-minute games, d5323dc (which plays the PR 3b gate's seed-5 rows to the digit); raw [headless/](larder-world-roadmap-2026-11-11/headless/census-headless-race-5-20.txt), paired with the gate's [wade6 rows](larder-world-roadmap-2026-11-11/wade/census-wade6-race-5-20.txt). Measure named before the run: games each hand ends before the cap, paired by seed. Win rate and stalemate median given apart.

| hand | mould wiped, fixed / headless | squad wiped | at the cap | mould left at the cap (median), fixed / headless | score, headless against fixed |
| --- | --- | --- | --- | --- | --- |
| idle | 0 / 0 | 0 / 0 | 16 / 16 | 10,457 / 10,457 | identical |
| random | 1 / 1 | 14 / 13 | 1 / 2 | | 12 higher, 3 lower, p 0.04 |
| farm | 0 / 0 | 0 / 0 | 16 / 16 | 3,620 / 3,870 | 1 higher, 9 lower, p 0.02 |
| wall | 0 / 0 | 0 / 0 | 16 / 16 | 4,278 / 4,257 | 3 and 3 |
| fort | 0 / 0 | 0 / 0 | 16 / 16 | 6,289 / 4,622 | 3 and 3 |
| rush | 1 / 0 | 5 / 5 | 10 / 11 | 3,351 / 9,568 | 5 higher, 11 lower, p 0.21 |
| strike | 4 / 1 | 0 / 0 | 12 / 15 | 1,736 / 1,866 | 3 higher, 13 lower, p 0.02 |

- **The core rule is not why farm, wall and fort stalemate**: they reach the cap in 16 of 16 games with the core and without it. Idle does too.
- **Without the core the player wins less**: the mould is wiped in 2 of 112 games against 6, and strike's 4 wins fall to 1. Strike and farm score lower, random higher (the core pull is off on a headless mould, so random's strays are not drawn in). None survives Holm over 7 (smallest p 0.02 against 0.007).
- So dropping the rule takes away the one decisive end the plans have and gives none back. The stalemates come from no plan wiping 3,500-6,000 walkers and from idle never being punished, which is PR 4's (overrun, and soldiers that cost food). `Regrow` is the middle option if taking the core should matter less.

**The crater's time in the cone.** A **volley** hand (`larder-world look=DIR hand=volley`): the squad of 9 stands off from the thick ring on the core crystal, burns a line across it from 22 to 34 s and stops, so the hole can be watched filling. Clips at 10, 6, 3 and 1 s and none (`crater/clips/V*`), the stand and rush clips re-rendered with the cone (T, R, W), on the page's first three sections.

| crater | 10 s | 6 s | 4.5 s | 3 s | 1 s | none |
| --- | --- | --- | --- | --- | --- | --- |
| volley: walkers killed by 46 s | 923 | 768 | 604 | 541 | 418 | 309 |
| stand at the edge: killed by 40 s | 486 | 525 | | 401 | 272 | 237 |
| rush: killed by 19 s (squad wiped in each) | 1,289 | 1,335 | | 1,368 | 1,352 | 1,409 |

- What the volley clips show: at 10 s the ring's front is cleared and pocked holes are still there 4 s after firing stops; at 6 s fewer holes are left; at 3 s the notch is small and closes in about 2 s; at 1 s it looks almost like no craters, even under fire.
- The rush's kills don't follow the crater time: its squad is wiped inside the body in every clip, and llvmpipe runs in parallel are not bit-repeatable (two clips with the same settings killed 1,289 and 1,300).
- **The owner picked 4.5 s** (17:29Z: "Can we do 4.5 s? I am thinking between the 3 and 6"), now `Rules::crater`'s default; its volley clip (`V45`) is on the page between 6 and 3 s. It lands on the tests and the clip: the 6 and 3 s gates were stopped under the owner's testing rule (17:26Z: build first, balance later; a census is not a gate for a look). The countdown is in fine steps, so any time works, not only whole seconds.

**Cost.** One comparison a tick while a core is down, on the CPU; the eating sum is over the crystals the game already reads. Nothing on the GPU.

## The economy: rations, and the overrun on (PR 4), 2026-11-12

**Why.** Integration's findings from the one-body thread's slime-enemy review (17:10Z):
- Idle survived 46 of 48 games, but the owner ruled on 11-11 that an idle player should lose ("if you do nothing, the slime should get much stronger than you and kill you").
- Food didn't bind: farm, wall and fort ended with 83-85% of their harvest unspent.
- Strike won by banking behind a 30-soldier army it never paid to keep.

Plan agreed with integration at 19:59Z, built under the owner's 17:26Z testing rule (build first; the census answers a question and gates nothing).

**What.**
- **Rations** (`Rules::ration` 1): every live soldier eats 1 food a game second from the bank (`Game::eaten`). Farm's 8 soldiers eat 8 a second of the about 55 they bank; strike's 30 eat 30.
- **Hunger** (`Rules::hunger` 1): while the bank can't pay, every soldier loses 1 hit point a second (a share for a part-paid step; `Game::hungry`), so a full-health soldier lasts 100 s of an empty bank.
  - Hunger costs only hit points. A starving soldier walks, fires and harvests at full speed, so a farm that runs dry recovers when a carrier banks, and the need falls as the squad shrinks. Slowing hungry soldiers would spiral, since slower carriers bank less and so starve more (integration's check, 19:59Z).
- **The overrun is on** (`Rules::overrun` 20 s; it had been a campaign switch): the round is lost once the mould has held the larder with the bank empty for 20 s.
- The window tells the player once, each time the bank runs dry: "Out of food: the squad is starving (N food a second to feed it)". Soldiers' health falls where the player can see it.
- Census keys `ration=`, `hunger=` and `overrun=`; each row ends with the food eaten.
- Test: `rations_feed_the_squad_and_an_empty_bank_starves_it_until_food_comes` (paid, half paid, recovery when food comes, death, off at 0). The books, stuck and pump tests now count rations.
- **The game designer's three checks** (21:51Z: a squad that slowly dies on an empty bank and can do nothing is the first playtest's soft lock again):
  - *The drain shows.* The HUD's food rate was already the bank's net change, so it counts rations. Its tooltip now names what the squad eats a second, and when the bank is falling and would run dry within a minute the rate says when ("-6/s, dry in 40 s").
  - *Starving soldiers can fix it.* `a_starving_squad_harvests_as_a_fed_one_and_feeds_itself`: two games on one seed, with the same rations and the hunger on or off, harvest step for step the same until a soldier dies, and the starving squad banks a load and eats again.
  - *The reserve recruit isn't fed into starvation.* In the same test, a wiped squad on an empty bank gets its recruit at full health. The recruit walks to a crystal, banks food and eats within 60 s. A recruit on an empty bank has 100 s of health to make that trip.

**Smoke runs** (race seeds 5-8, raw [economy/](larder-world-roadmap-2026-11-11/economy/smoke-ration1-race-5-8.txt)), choosing the ration:
- **At 2**: idle starves by 340-350 s on all 4. But strike starves its 30 soldiers on 3 of 4 and fort on 1 of 4: an army would cost 4.5 times its price over a game.
- **At 1**: idle starves by 570-600 s on all 4, and farm and fort hold to the cap. Strike wins 2, loses 1 and stalemates 1 (before rations it stalemated all 4). 1 is the default.
- **The rush** dies in 43-110 s on these seeds with or without rations: the hand walks its squad into the body. Without rations, 2 of its 4 games then sat at 0 soldiers to the cap. With rations, the bank drops under a soldier's price and the game ends.

**The census** (cd56a52, race seeds 5-20, seven hands, 15-minute games, about 30 min on this machine; raw [economy/](larder-world-roadmap-2026-11-11/economy/census-ration1-race-5-20.txt)). The question, named before the run: does idle lose, does food bind, and does strike stop dominating? The comparison column is the PR 3b gate (90ca407), which also had the 10 s crater, not 4.5 s, so it is a guide, not a pairing.

| hand | ends, rations (mould wiped / squad wiped / cap) | PR 3b gate | harvest unspent at the end (median), rations / PR 3b |
| --- | --- | --- | --- |
| idle | 0 / 16 / 0, median 586 s | 0 / 0 / 16 | |
| random | 0 / 14 / 0 (2 overrun) | 1 / 14 / 1 | 0% / 2% |
| farm | 0 / 0 / 16 | 0 / 0 / 16 | 62% / 74% |
| wall | 0 / 0 / 16 | 0 / 0 / 16 | 58% / 74% |
| fort | 0 / 0 / 16 | 0 / 0 / 16 | 43% / 80% |
| rush | 0 / 11 / 4, plus 1 overrun | 1 / 5 / 10 | |
| strike | 6 / 3 / 7 | 4 / 0 / 12 | 17% / 64% |

- **Idle loses, 16 of 16**: its squad starves at a median 586 s.
- **Food binds more**: fort keeps 43% of its harvest unspent against 80%, and strike 17% against 64%. Farm and wall still bank over half, since 8 soldiers cost little; buildings and tools are their sinks.
- **Strike no longer dominates**: it beats farm 12-4 (p 0.08, not under Holm; 26-6 at PR 3b, held). It is decisive in 9 of 16 games, 6 wins and 3 losses, where it was 4 wins and 12 stalemates. Its three losses are 30-soldier armies that starved on seeds with a thin harvest. The player AI buys to 30 regardless of what it can feed, which is a tuning note for the player AI thread.
- Every plan but the rush beats idle and random 16-0 (all hold under Holm over 12). The rush loses to idle 4-12 and ties random 7-9.
- **Smoke checks**:
  - No game ends in under a minute; the shortest is a rush at 83 s.
  - Farm and wall end every game with all 8 soldiers, so they never starved.
  - The rush hand's 4 cap games sit at 0 soldiers with 411-491 food, which is over a soldier's price. The hand keeps a reserve and won't buy; a player could.

**The regrowing core is taken out.** The owner ruled at 20:21Z: "remove the king rule. As long as the slime has food/crystal energy it can make walkers. Headless." The tiers thread confirmed at 21:00Z that no tier will use `CoreKind::Regrow`, so PR 4 removes it, along with `Game::mould_ate`, `World::start_births`, the census key `regrow=` and the window's "grew back" alert. It worked as built (the section above), so it is recorded in `dead-ends.md` as dropped, not rejected, with what to restore if a core comes back. `core=fixed|headless` stays for tests.

**Cost.** Per step on the CPU: one multiply and a subtraction, and a loop over the soldiers while hungry. Nothing on the GPU.
