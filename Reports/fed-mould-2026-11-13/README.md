# A fed mould you can see (page, 2026-11-13)

The owner, 2026-11-12 22:22Z, after playing shape C: "honestly what is
going on with the slime is unclear to the player so that makes this type
of strategy [cutting the mould off its crystals] not attractive at the
moment. I cannot tell if it is getting stronger by holding the crystals.
Right now it is mostly moving around and I kill it."

Page: https://claude.ai/artifact/HwcxYxZWdzPdttXgLXn2q1 (source
`page/index.html`; films and posters in `page/`). It is a blind A/B, and
the key is below, not on the page.

**The owner's answer (2026-11-13 02:39Z): "Definitely film 1."** Film 1
is C sharing its food, so that is where the owner saw the mould grow
stronger from its crystals, and it stays the window's default. The
cut-the-network rule, parked at 22:22Z because feeding did not read, can
now be judged on this mould.

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

## One walker a cell at the peak (tech lead on #183)

The tech lead's note on #183: `gpu::crowd_holds` checks only the start
walkers, and C now grows to its slots. Near one walker per open cell, the
one-a-cell rule would quietly start to matter.

**Across the map, C stays well under the limit.**
- C's 50,000 slots fill at most 34.8% of the open cells at scale 2. That
  is the noise world's 143,604 cells.
- The designed maps fill 32.5-34.4%.
- The slots stop its growth well before the cells run out.
- `shape_c_at_its_peak_fills_at_most_half_of_any_map` (in
  `src/bin/larder-world.rs`) now holds this to half, the same bound
  `crowd_holds` sets on the start.

**Inside the body, the rule binds, and it already did.**
`examples/fedprobe.rs` now measures, at the end of a run, the share of
living walkers boxed in: all eight of their neighbouring cells are full
or not agar, so the rule lets them neither step nor breed. On the CPU
race world, idle squad, at 5:00, seeds 1-4 (raw: `raw/peak/`):

| | walkers / open cells | boxed in |
|---|---|---|
| C as it was | 16.3-28.6% | 58.8-65.9% |
| C with shared food | 34.6-35.7% | 66.4-83.3% |

With shared food, more walkers are boxed in on 4 of 4 seeds (sign test
p 0.125, the floor for 4 seeds). Most of C has always been a packed
sheet, and that is the look the owner picked. Sharing the food makes the
sheet fuller, and the slots cap it. A look that drew walkers a cell
above one would make the packing visible (`fed_crowd` is waiting on
that).

## Cutting it off from its crystals (11-13)

The owner parked a cut-the-network rule at 22:22Z ("I cannot tell if it
is getting stronger by holding the crystals"). With sharing picked, the
question is whether a part cut off from every crystal now withers where
the player can see it.

**Today's C does not wither when cut off.** It stays whole for a minute
or more, then dies all at once.
- `examples/cutprobe.rs` cut a held ring (`wound=120`, a gap 4 dish cells
  wide) round thick parts of the body with no crystal within 4 dish cells
  (`ring=8 unfed=1`). This was on the CPU race world at 3:00, seeds 1-4,
  3 cuts each, with C's keys (`raw/cut/ring.txt`, `ring-series.csv`).
- **C as it was** thinned out steadily: 59% of the piece was left at
  30 s, 26% at 60 s and 8% at 90 s.
- **C with shared food** kept every walker for the full 2 minutes in 10
  of 12 cuts. The shared energy evens out, so the whole piece runs down
  together, from 0.8-1.3 a walker at the cut. The 2 cuts that started at
  0.2-0.3 went from whole at 30 s to 5-15% at 60 s.
- Cutting across a tip instead (`raw/cut/held.txt`) cuts nothing off.
  C's tips sit on crystals and the body flows round the wound (every cut
  `rejoin+fed`).

**Food that fades as it travels** (`BigParams::flow_loss`, the one-body
thread's lossy sharing): what a cell takes from a richer neighbour arrives
x (1 - `flow_loss`). Parts far from a crystal hold little in reserve.
- On the CPU, ring cuts as above (`raw/cut/loss.txt`, `loss-series.csv`).
  The share of the piece left at 10, 30, 45, 60 and 90 s:

| | 10 s | 30 s | 45 s | 60 s | 90 s | C at 3:00 (walkers, seeds 1-4) |
|---|---|---|---|---|---|---|
| shared food (today) | 1.00 | 1.00 | 0.93 | 0.85 | 0.84 | 49,949-50,000 |
| + `flow_loss` 0.05 | 0.97 | 0.65 | 0.28 | 0.21 | 0.01 | 37,001-39,549 |
| + `flow_loss` 0.2 | 0.69 | 0.42 | 0.27 | 0.22 | 0.08 | 8,594-13,888 |

- Against today's, the piece's life (to under 10%) is shorter on 11 of 12
  cuts at both strengths (sign test p 0.006). At 0.2 the mould is a
  fifth of its size, so it is out.
- **In the window** (Fork, seed 1, farm hand, a held ring cut at 2:30
  with `cut=150,386,239,22` round a stretch of tube between two
  crystals; `raw/cut/film-*.log`):
  - Today's C: 242 walkers inside at the cut, whole at +75 s, 58 at
    +80 s, gone by +90 s.
  - With `flow_loss` 0.05: 146 inside, whole at +35 s, 46 at +40 s, gone
    by +50 s.
  - The mould at 4:00: 27,326 walkers and 12.7% of the screen today,
    against 18,007 and 9.3% with the loss. C as it was had 6.7%.
- **In both, the piece dies all at once.** Once its pooled energy runs
  out, it is gone within about 10 s. It does not thin out gradually.

**Page:** https://claude.ai/artifact/W3ZnFFGTNoD8XYWjGzrSRZ (source
`cut-page/`). It is a blind A/B, and the key is below.

**The key:** Film 1 is today's C (shared food). Film 2 is C with
`flow_loss` 0.05. To play film 2: `larder-world -- flow_loss=0.05`.
Nothing changed in the default; the owner's answer on the decision card
decides it.

**Smoke** (CPU race world, C's keys plus `flow_loss` 0.05, seeds 1-4,
idle/farm/strike, cap 5:00; `raw/smoke/fading-food-cap300.txt`; 341 s on 3 threads):
- Every game ran to the 5:00 cap, so none ended in under a minute.
- Idle doesn't win: its mean score is -0.329, against farm +0.144 and
  strike +0.140, each higher on 4 of 4 seeds.
- No game was stuck for a minute or more.
- The idle hand's mould reaches 43,532-49,977 walkers by 5:00, and the
  hands' 37,856-44,923. So the slots still bind for an unchecked mould.

**Built for it:**
- The GPU's `egain` takes `eloss` (the soil row's spare word, so Params
  keeps its 896 bytes).
- `Settings::flow_loss` and the `flow_loss` key.
- A twin case "lossy flow" at 0.2, with energy L1 1.1e-6. With the GPU
  ignoring the loss it fails at 4.3e-3.
- `cut=` in the window's no-window mode.
- `flow_loss` in `tiers::set_key` and `capacity` in the cut probe.

## Tests

- **GPU twin.** `the_world_matches_the_cpu_for_one_tick` gains the case
  "fed crowd: a fed walker packs two to a cell". Every other walker is
  fed to 3 before the tick. Result: 0 of 15,000 walkers off, energy L1 0,
  289 cells holding two walkers on both engines, and 1,513 born.
- **Clearing the move's claims.** The twin found that the GPU must clear
  the move's claims before births when a cell's winner leaves room for a
  fed parent's child. Without the `unclaim` pass after the walk, 75 of
  15,000 walkers were off.
