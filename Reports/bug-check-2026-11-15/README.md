# Bug check of the 11-14 merges (2026-11-15)

**Ask.** Owner, 11-15 00:20Z: "Start a deep bug check code review thread. We
have made many changes today." Scope: everything merged 2026-11-14 after
#275 (#276-#307, 62 code files, about 13,000 lines), main 3f6b6fbc. The full
suite was green on it (455 lib tests and the binaries'), so every finding
below slipped past the tests.

## Reproduced, fixed here

| # | Bug | Reproduction | Fix |
| --- | --- | --- | --- |
| 1 | Linked squads soft-lock: when the controlled squad of a link dies, `Cmd::Active` refuses every live squad of its link (the dead squad keeps its link id), so `squad_frame` never hands control on | `a_link_whose_controlled_squad_dies_hands_control_to_the_rest` failed on main | `Cmd::Active` checks the link only while the controlled squad has members |
| 2 | A `Thrift` card takes a run's gun over its budget: `Run::fits` costs a card at the gun's end, `put_in` and `move_to` place it elsewhere, and a Thrift cuts the card after it | Thrift, Bomb, Germ (6) on a budget of 7, buy Quiet: the gun became Thrift, Quiet, Bomb, Germ costing 8 (`a_thrift_cannot_take_the_gun_over_its_budget`) | `put_in` and `move_to` check the gun where the card lands (bag, or the move refused); taking a card out runs `refit` |
| 3 | Hard seams half a cell to a cell and a half off every shore and bridge at close zoom (#305's spline skip tested the 2 x 2 cells round a pixel, but the spline reads 4 x 4, and the bank, shallow margin, bridge shadow and depth read its shares as a distance) | River map at zoom 24 beside the bridge: one pixel row jumped from 56 to 89 (RGB sum); with the skip off it ramps 56, 59, 62 ... 89. 8,013 pixels differ by more than 6 at zoom 24, 2,995 at 12 | Skip only when all 16 cells are one ground; renders now equal the no-skip path at zoom 12, 16 and 24 (0 pixels differ) |

## Read in the code, fixed here

| # | Bug | Fix |
| --- | --- | --- |
| 4 | Run shop "Faster trigger" and "Longer barrels" did nothing: `Run::rules` puts them on `Rules::shot_every`/`range`, which only `Gun::standard` reads, and a run arms squad 0 with `Run::carried()` | `Run::armed`: the carried gun with those levels; the window arms and shows it (`barrels_and_trigger_reach_the_carried_gun`) |

## Reproduced, fixed by the planets thread (PR #309)

- Dune clay islands overwrote deep water and bridges (`(g & 0x0F) | G_CLAY`):
  26 of 32 dune planets over seeds 1-40, depths 1-8 (all Valley, depth 4)
  had deep cells turned to standable wet clay, up to 9,027 cells (seed 9).
- Read: bridge planks left on dry land near nests; stale walking maps starved
  by a steady stream of asks; `map_for` preferring a failed neighbour's map
  over the goal's own; no cache trim at the old pace.

## Read in the code, left open

- **Spread on bullet cards does nothing on the rifle and long gun** (their
  spread is 0; `turn.rs` multiplies it), yet `Ammo::grade` prices it.
  Owner question: a small base spread, or an additive bullet spread.
  **Answered 11-15 08:10Z: "Small base spread on both"**: rifle 0.5 cells
  (`guns::RIFLE_SPREAD`, now also `Rules::spread`'s default, so the
  standard rifle every unarmed squad fires), long gun 0.3. Test
  `a_spread_bullet_widens_the_rifles_aim`: a x4 spread bullet turns the
  same frame's shots four times as far. Default shots in every mode now
  scatter up to 0.5 cells at the target (they were dead on).
- **Fire kills counted unevenly**: the GPU's `firekill` adds to the strike
  counter (`wstat[3]`), so the next kill burst claims them; the CPU counts
  `burned()` only on burn-trail frames. Stats and score only.
  *Fixed 11-15, see* Leftovers fixed *below.*
- **Designed maps get plants (`plants::stand_in`) only in the window**; the
  CPU census and campaign tools load them bare. *Fixed 11-15, below.*
- Healing runs while a soldier works (still, unbitten, not firing).
- Dev only: N on a `planet:D:SEED` map reloads the same planet (*fixed
  11-15, below*); a run with `menu=0` does not scale walkers by
  `Level::walkers`.
- Cost: `Game::mods` rebuilds `rounds::flight` on every call; `heal` clones
  the crystals per guarded crystal; `work_boost` scans the soldiers per
  worker.

## Checked and clean

Upkeep is gone (no rations, hunger or lure burn; the slime's own upkeep is
its metabolism). HUD bar and minimap hit rects match the drawing (#300).
Order lines read each squad's queue every frame; WASD pans while paused
(#301). Map files keep size, plants and moisture. The view's buffer layouts
and the engine's params, plant header and gas read-back offsets (#307) agree
between Rust and WGSL. Drag, Q and E keep every card once.

## Cost of fix 3

llvmpipe, `map=river seed=1 bench=8 hand=idle screen=1280x800
centre=256,140,ZOOM`, GPU tail p50, three alternating rounds:

| Zoom | Main | This |
| --- | --- | --- |
| 1 | 166.9, 178.0, 201.5 | 180.7, 199.5, 206.9 |
| 12 | 88.8, 90.5, 103.0 | 99.1, 105.8, 106.7 |

About +12-17% on llvmpipe, giving back part of #305's saving: a uniform
pixel now reads 16 ground words instead of 4 (no spline weights). A bit in
the ground word marking "one ground for 2 cells round" would make it one
read again; byte 0 has none spare (kind, soil, `FAR`).

**GTX 1070** (PC runner, 11-15; main 66dd92fe against this PR's 36e2f84b,
the same River look, three alternating rounds a zoom; raw output on branch
`claude/project-thread-26drse` at 58bcadf9,
`runner-out/edge-skip-pr311-66dd92fe-vs-36e2f84b/`). Frame p95 / GPU tail
p50, ms:

| Zoom | Main | This |
| --- | --- | --- |
| 1 | 6.08-6.53 / 5.23-5.28 | 6.29-6.33 / 5.46-5.50 |
| 12 | 3.22-3.24 / 2.39-2.42 | 3.44-3.47 / 2.62-2.65 |
| 24 | 3.04-3.32 / 2.22-2.28 | 3.23-3.61 / 2.42-2.45 |

About +0.2 ms of GPU a frame at every zoom (some main runs started with the
card near 1000 MHz). Kept: the seams broke the crisp-at-24-px rule, and
every frame stays under 3.7 ms p95 at play zoom.

## Leftovers fixed (2026-11-15, later)

Three of the open items above, in one change (thread "Fix leftover
bug-check items"). No shader change.

- **Fire kills.** `Game::finish_step` now asks `World::burned` once a
  frame and counts the plants' fire in `Game::killed` and the new
  `Game::fire_killed`; before, only a Scorch trail's frame asked, so a CPU
  game without one never counted a fire kill. On the GPU the strike
  counter (`wstat[3]`) is shared with `firekill`, so `GpuWorld` now reads
  it from every frame's readout (already copied there): what moved over a
  frame is fire's, as bursts run only between frames. A kill burst after a
  burn reads the counter first, so the burn's kills stay fire's and the
  blast counts only its own. No new read-back a frame (one before a kill
  burst only when a burn came first in that gap). A Scorch trail's burns
  still count in `Rounds::burned` on both. Test
  `the_plants_fire_kills_count_without_a_scorch_trail` (flats, full plant
  cover, a burn by the slime's home at 10 s): 5 fire kills by 40 s, all 5
  in `Game::fire_killed` (0 before: the field did not exist and nothing
  asked).
- **Plants in the tools.** `plants::planted(map, seed)` is the window's
  `stand_in` on a copy; the `campaign`, `larderworld` and `kitplans`
  examples plant each seed's map with it (`plants=0` plays bare, as the
  window's). `cutprobe` stays bare (hunk probes light no fire). The
  library's tests keep their bare maps.
- **N on a seeded planet.** N on `planet:D:SEED[:AREA]` goes on to
  `planet:D:SEED+1` (the name's seed wins over `seed=`, so the old fresh
  seed changed nothing); a seedless planet still rolls a fresh seed. Test
  `n_on_a_seeded_planet_goes_on_to_the_next_seed`.
