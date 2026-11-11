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
