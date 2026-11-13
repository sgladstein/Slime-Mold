# A fed mould you can see (page, 2026-11-13)

The owner, 2026-11-12 22:22Z, after playing shape C: "honestly what is
going on with the slime is unclear to the player so that makes this type
of strategy [cutting the mould off its crystals] not attractive at the
moment. I cannot tell if it is getting stronger by holding the crystals.
Right now it is mostly moving around and I kill it."

Page: https://claude.ai/artifact/HwcxYxZWdzPdttXgLXn2q1 (source
`page/index.html`; films and posters in `page/`). It is a blind A/B, and
the key is below, not on the page.

## Is it getting stronger? Yes, but you can't see it

`examples/fedprobe.rs` runs shape C on the CPU race world (scale 2, 25,000
walkers, upkeep 0.0002, capacity 50,000) with an idle squad, on seeds 1-4.
The deciding measure is walkers at 5:00, paired by seed.

- **With crystals that feed nothing** (`crystal_eat` 0), C is gone by
  2:00. It has 1,631 walkers on average at 5:00, against 36,053 with
  today's food, and is smaller on 4 of 4 seeds (`raw/fedprobe/eat.txt`).
  Holding crystals is what keeps C alive.
- **The food stays where it is eaten.** At 5:00 the walkers within twice
  a crystal's radius hold 6-9 energy each, and the rest hold 0.3. At one
  walker a cell, the cells round a crystal are full, so that energy buys
  almost no new walkers. On screen the network looks the same whether it
  is fed or not.

## Three rules tried

**1. A fed walker lays more trail.** The walker lays (1 + k ×
min(energy / 4, 1)) times the trail. This was rejected
(`raw/fedprobe/fedtrail.txt`, from the probe's first version with `fed=k`):

- At k = 6, the trail round the crystals rose from 0.7 to 1.2-1.6 times
  the rest's. But the brighter tubes drew walkers onto the crystals, and
  the mould bred less: 25,545 walkers at 4:00 against 31,859, fewer on
  4 of 4 seeds.
- At k = 2, the trail ratio was 0.9-1.1 and the mould still had fewer
  walkers (28,960).

**2. A fed walker packs two to a cell** (`BigParams::fed_crowd` 1,
`fed_full` 2). This is built on both engines and kept off:

- On the CPU it gives 45,621 walkers at 4:00 against 31,859, more on 4 of
  4 seeds (`raw/fedprobe/crowd1-full2.txt`; with `fed_full` 4 the mean is
  45,320, `crowd1-full4.txt`).
- In the window (Fork, seed 1, farm hand, software Vulkan) it has 21,053
  walkers at 5:00 against 13,956, but mould covers only 7.6% of the
  screen against 7.0%.
- The look draws a cell's thickness as its trail × (1 - exp(-walkers
  near / n0)). That is already full at about one walker a cell, so packed
  walkers add nothing you can see. Re-test this when the look shows
  walkers a cell above one.

**3. The body shares its food** (`energy_flow` 0.05, the one-body
thread's rule, already on both engines). **This rule reads on screen and
is now shape C's default:**

- Energy evens out between neighbouring cells. Walkers along the network
  get fed, and they breed where there is room. The body swells round the
  crystals it holds and keeps growing while it holds them.
- In the window it has 45,237 walkers at 5:00 against 13,956, and mould
  covers 17.1% of the screen against 7.0%.
- It is ahead from the first die-off on: at 1:45, 8.9% against 4.6%.
- With the pack added on top (`flowcrowd.log`), it reaches the
  50,000-walker cap by 4:00, but covers no more of the screen (16.7%).

| Window, Fork seed 1, farm hand | 1:00 | 2:00 | 3:00 | 4:00 | 5:00 |
|---|---|---|---|---|---|
| C as it was: walkers | 27,601 | 10,406 | 12,573 | 13,379 | 13,956 |
| C as it was: mould on screen | 17.0% | 5.5% | 6.6% | 6.7% | 7.0% |
| C + pack two a cell: walkers | 30,701 | 15,707 | 18,968 | 20,294 | 21,053 |
| C + pack two a cell: on screen | 17.2% | 5.6% | 6.7% | 7.4% | 7.6% |
| **C + shared food: walkers** | 27,687 | 22,737 | 31,282 | 39,164 | 45,237 |
| **C + shared food: on screen** | 16.8% | 10.1% | 12.3% | 15.1% | 17.1% |
| C + both: walkers | 29,318 | 28,184 | 41,038 | 49,972 | 49,998 |
| C + both: on screen | 16.8% | 9.9% | 12.9% | 16.4% | 16.7% |

How the numbers were made:

- "Mould on screen" is the share of the map's pixels (the frame cropped to
  the map) whose red exceeds their blue by more than 60, with red above
  120.
- The counts are the window's own `t` lines (`raw/films/*.log`).
- The farm hand's squad is gone by 3:00 with C as it was, and by 3:45
  with shared food.
- The films were made before #181 (the shot sink) reached main.

**The key:** Film 1 is C with shared food (the new default). Film 2 is C
as it was. To play C as it was: `larder-world -- energy_flow=0`.

## What sharing costs and risks

- **Frame cost.** No new pass is added. One body's two passes run every
  tick: on the GTX 1070 the GPU thread measured `esum` 0.016 ms and
  `egain` 0.054 ms a tick (with hunger, main 71ed0f9; one-body report).
  A 1070 timing of shape C with sharing is to be booked through the PC
  runner.
- **Starving all at once.** At a matched size, the one-body thread found
  that lossless sharing lets random's attacks starve the whole mould at
  once (`Reports/dead-ends.md`, *Holding the one-body mould ...*). Here
  the mould is not held to the old size, so that cliff is not what this
  default rests on. The CPU smoke below is the check.
- **Cutting it off its crystals now pays more visibly.** The parked
  cut-the-network rule (the owner, 22:22Z) can be judged on this mould.
  The GPU has no `flow_loss`, so a body cut off from its crystals starves
  only through its own upkeep.

## Smoke (CPU race world, C's keys with shared food, seeds 1-4, cap 5:00)

Command: `larderworld census walkers=25000 upkeep=0.0002
keys=capacity:50000+energy_flow:0.05 seeds=1-4 hands=idle,farm,strike
cap=300`. It ran on the merged tree (with #181) and took 263 s on 4
threads. Raw output: `raw/smoke/shared-food-cap300.txt`.

- **No early end.** Every game ran to the 5:00 cap, so none ended in
  under a minute.
- **Idle doesn't win.** Idle's mean score is -0.333. Farm (+0.120) and
  strike (+0.116) each beat it on 4 of 4 seeds.
- **Nothing gets stuck.** No game was stuck for a minute or more.
- **The mould fills its slots.** It reaches the 50,000-walker cap by 5:00
  in every game, so the cap now binds.
- **This is a smoke check, not a balance pass.** A census of the window's
  default against the hands is the balance pass's job.

`examples/larderworld.rs` reads C's slots through `keys=capacity:N`.
`tiers::set_key` gained `capacity` and `fed_crowd` for this.

## Tests

- **GPU twin.** `the_world_matches_the_cpu_for_one_tick` gains the case
  "fed crowd: a fed walker packs two to a cell". Every other walker is
  fed to 3 before the tick. Result: 0 of 15,000 walkers off, energy L1 0,
  289 cells holding two walkers on both engines, and 1,513 born.
- **Clearing the move's claims.** The twin found that the GPU must clear
  the move's claims before births when a cell's winner leaves room for a
  fed parent's child. Without the `unclaim` pass after the walk, 75 of
  15,000 walkers were off.
