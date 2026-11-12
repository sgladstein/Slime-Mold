# Food joined by tubes: why the food-reach droplets come out as waves (2026-11-11)

A measured test, no game change. The owner, 21:36Z, on the graphics
thread's video of the GPU world started with the "food reach" settings:
"The food reach settings look very different than the original slime
mold. Its walks shoot out in waves that reach the crystals but never (at
least within the timeframe of the video) connect a network to them." The
ask, relayed by the coordinator: find out why, and make settings that
explore and then join the food with tubes, the way Physarum does. Page:
https://claude.ai/artifact/Tcbk7VSkJ5STJn6SUuGHvV (source in [`food-tubes-2026-11-11/`](food-tubes-2026-11-11/)).

**In short.** The "food reach" settings were the wrong mould for this: a
droplet creature that has no way to make tubes. The settings that explore
and then join food with tubes already exist. They are the Larder game's
own race start: today's network rules with W0 growth, from a small blob on
the home crystal. On the race world it joined more crystals to its main
body at 60 s than the droplets on 16 of 16 seeds (sign test p 0.00003).

## Why the waves

The "food reach" settings are the grazing droplets of the
[first reach](first-reach-2026-11-11.md) test, which gave them the name. They
are a different creature from the network, built to crawl as bodies:

- **Round bodies, not tubes.** One walker a cell (crowd 1), blocked
  walkers that still lay trail where they stand (`blocked_deposit` 1) and
  a wide sensor fan (sensor angle 90) gather the walkers into round
  droplets ([outside rules](outside-rules-2026-11-11.md), *Packed
  walkers*). A tube needs walkers that run along a line; these turn into
  the thickest trail around them and close up into a ball.
- **Grazing pushes them on.** With `graze_weight` 10 a droplet eats the
  regrowing grass under it and steers away from bare ground, so every body
  crawls off the ground it has eaten. Started in one disc, they all crawl
  outward at once: the waves in the video.
- **Nothing holds a body on food.** No growth and a fixed walker count. A
  crystal's scent draws a body in but does not feed it or keep it, and the
  body eats the grass there and crawls on.
- **The test never asked for tubes.** First reach scored a pile as
  reached when any trail touched it once. A droplet passing over a crystal
  counts, so the droplets did well on a measure that cannot see whether the
  food stays joined.

## What joins food with tubes

Looks on the big dish and the race world (one seed each unless stated,
before the census):

- **The network** (today's rules: sensor angle 45, turn 45, deposit 5,
  keep 0.9, crowd 1) makes tubes to the food it reaches, and the mesh
  coarsens onto the piles.
- **Explorers** (`explore_steps`, `explore_spread`) send a fixed network
  out fast: they reached every pile within 5-15 s on the 1024 x 640 dish,
  the pace the owner called "crazy fast" (14:59Z).
- **The race start** (`larder_world::race_params`: the network rules,
  W0 growth at upkeep 0.001, 5,000 walkers packed on the home crystal,
  room for 50,000, 23 crystals of charge 512 that run dry) explores and
  then joins. A faint haze runs out along a few paths; the paths that
  meet crystals thicken into tubes, and the crystals become the mesh's
  nodes one after another. The mould grows on the crystals it eats and
  starves away from them (a median 2,332 walkers alive at 5 s from the
  5,000, then 12,957 at 90 s).
- **Growth or the small start?** Without growth (upkeep 0, the same 5,000
  walkers) the race start still sets out and makes tubes: 12, 15 and 14
  crystals joined at 90 s on seeds 1-3, against 21, 16 and 11 with growth.
  Three seeds can't tell the two apart. What growth adds that can be seen
  is a mould that gets bigger as it eats, and a finer mesh by 90 s where
  the fixed body keeps a few big loops (`look6.sh`). A start as dense as
  the graphics thread's GPU blob (25,000 walkers, about 15% of the world)
  joined 11-13 crystals by 5 s with or without growth: it covers too much
  at once to be a test of spreading.
- `energy_flow` (CPU only) gives blobs on the food joined by tubes; short
  explorer stages for the newborn give a faster, finer mesh. Neither was
  censused.
- Growth from the droplets' wide disc joined 19-22 of 23 crystals by 5 s
  (3 seeds): it covers the map before anything has to spread.

## The test

- **World.** The race world on `bigdish`: `BigParams::today()` with scale
  2 (512 x 320 fine cells, 256 x 160 dish cells: the GPU world's size in
  dish cells), 2 substeps, 23 crystals in a rect, charge 512, eaten at
  0.005, rock and wet ground 0.2, food smelled 36 fine cells out.
- **Arms** (`run.sh`):
  - `droplets`: the food-reach settings as the graphics thread ran them
    (crowd 1, sensor angle 90, turn 45, sensor offset 9, deposit 5, keep
    0.93, `blocked_deposit` 1, graze weight 10), no growth, 7,500 walkers
    loose in a disc of 75 dish cells at the centre (about 10% full, the
    graphics thread's density).
  - `race`: the race start as the Larder game runs it (5,000 walkers,
    capacity 50,000, upkeep 0.001).
  - A third arm, `racecry` (race with `graze_eat` 0, meant as race without
    grass), was dropped after seed 1: it matched race to the end hash.
    With crystals on, growth makes no grass unless a `graze_weight` asks
    for it, so the two arms were one.
- **New measure, `joined`** (`examples/bigdish.rs joined=1`): the mould's
  main body is the largest 4-connected piece of ground with trail over the
  tube cut (1.0); a crystal is joined when any of its cells is on it. Pile
  0, the home, is not counted, so 22 can be joined. It runs only at a
  report, so it adds nothing to a tick.
- **Deciding measure, named before the run:** crystals joined at 60 s,
  race minus droplets, on paired seeds 1-16, sign test with ties dropped.
  It was named with racecry as a second comparison and Holm over the two;
  with racecry gone the p is also given doubled.

## Result

**The race start joined more crystals at 60 s than the droplets on 16 of
16 seeds, fewer on none (p 0.00003; 0.00006 doubled).** Over the 16 seeds,
crystals joined at 60 s ([`score.txt`](food-tubes-2026-11-11/score.txt)):

| | Joined at 15 s | 30 s | 60 s | 90 s | Reached by 60 s | Reached and joined at 60 s |
|---|---|---|---|---|---|---|
| Droplets | median 2 | 2 | 2 (1-4 a seed) | 2 | 352 of 352 | 40 of 352 |
| Race start | median 12 | 15 | 19 (10-22) | 21 | 351 of 352 | 278 of 351 |

- **Both reach everything; only the race start holds it.** The droplets'
  wide start touches every crystal within a median 2.4 s, and its trail
  stays in pieces: the main body holds 0.17-0.21 of its tube area
  (median), and its bodies drift. The race start's haze reaches every
  crystal by a median 8.3 s (2.5-36 s a seed), then joins them: its main
  body holds 0.83 of its tube at 15 s and 0.98 at 90 s
  ([`summary.txt`](food-tubes-2026-11-11/summary.txt)).
- **At 90 s**, the same: more on 16 of 16 (secondary).
- **The slow seeds.** Seed 12 joined 10 at 60 s and all 22 by 90 s. On
  seed 3 (11 at 90 s) the far half is still a faint loop past the
  crystals, under the tube cut (a still at 90 s, `looks/`).

## Pace, for the game

The `bigdish` clock runs 120 ticks a second; the Larder game runs the
same world at 15 ticks a game second (`larder_world::Rules::tps`), so a
`bigdish` second is 8 game seconds. In game time the race start's haze
reaches every crystal in about a minute, it joins half of them in about
two minutes (12 at 15 s), and it is still joining at 12 minutes (21 at
90 s). The videos on the page run at 3 times the `bigdish` clock: 24 game
seconds a second.

## Cost

No rule here runs each step; `joined` runs only at a report. On one core,
four runs at a time on four cores, the race world took 5.6, 6.3 and 7.0
ms a tick (median at 30, 60 and 90 s, as it grows) and the droplets 6.1-6.2.

## For the GPU world

The GPU engine already runs this mould: `gpu::larder_world::race_settings`
gives the engine the race world's params, and `GpuWorld` runs `scale`
engine ticks a game tick (a test keeps the two within 8% over 20 game
seconds, W2b). So `examples/worldspread.rs` can start from
`race_settings(seed, 2, 5000)` on a scale-2 engine instead of the
droplets. Not tested here: the race params at the graphics thread's scale
8, where trail sensors 9 fine cells out reach 1.1 dish cells instead of
4.5. The graphics thread's own start (`Settings::big_dish()`, 20:07Z) also
smelled food 36 fine cells out, 4.5 dish cells at scale 8 against the race
world's 18. Either may be why its packed 150,000-walker start stayed a
blob; neither is tested.

## Limits

- Two complete setups (rules, start and growth together), not one factor
  at a time; the looks above split the start from growth on 3 seeds only.
- `joined` counts a crystal on the main body. A crystal on a second,
  smaller piece of tube counts as not joined.
- The CPU world at scale 2, not the GPU world at scale 8.

## Raw

- [`food-tubes-2026-11-11/raw/`](food-tubes-2026-11-11/raw/): program
  output, one file per arm and seed. Racecry's seed 1 was rerun after the
  census and is kept in `looks/racecry-s1.txt` (end hash 180c6fe0cd15b653,
  as race).
- [`score.txt`](food-tubes-2026-11-11/score.txt): `score.py`;
  [`summary.txt`](food-tubes-2026-11-11/summary.txt): `summary.py`.
- [`looks/`](food-tubes-2026-11-11/looks/): the looks' scripts and output
  (`look6.sh` is growth against the small start).
- `pics/` and `video/`: the page's pictures (`strip.py`) and videos (the
  census's seeds 1 and 2 again, `frames=` every 30 ticks, hashes as the
  census).
