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

**Page:** https://claude.ai/artifact/W3ZnFFGTNoD8XYWjGzrSRZ. Version 1
(source `cut-page/v1/`) was a blind A/B; version 2, with withering, is
below.

**The key to version 1:** Film 1 is today's C (shared food). Film 2 is C
with `flow_loss` 0.05. To play film 2: `larder-world -- flow_loss=0.05`.
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

## A cut-off part withers: food's news (11-13)

The game designer's review of the cut test asked for a cut piece that
falls steadily, with "roughly half gone by mid-window and no cliff", and
an uncut C the same size as today ("retune the numbers instead of
shrinking C"). Both rules on the page's first version fail that: the piece is whole and
then gone at once. Three rules were tried on the CPU. The deciding
measures, named before the third run: the piece about half gone by 20-30 s
with no 5 s drop much over 0.2 of it, and uncut C at 3:00 within about 5%
of today's, paired by seed.

**The rule kept (`BigParams::wither`, off by default; both engines).**
News of food travels through the body. A walker that takes crystal hears
it now. Each step, each cell holding walkers starts from the latest news
its walkers carry and takes the latest of the eight cells round it, twice
over (`wither_reach` 2), so news crosses two cells a step through cells
holding walkers and stops at an empty one. Each walker then carries its
cell's news on. Every 15 steps (`wither_every`, twice a game second), a
walker whose news is more than 150 steps old (`wither_lag`, 5 game
seconds) dies with chance `wither`, whatever energy it holds. A child
carries its parent's news.

**Held ring cuts as before** (CPU race world, C's keys, seeds 1-4, 3 cuts
each at 3:00, `raw/cut/news-*.txt`, `news-*-series.csv`, summary in
`news-summary.txt` from `raw/cut/summ.py`). The share of the piece left:

| | 10 s | 15 s | 20 s | 30 s | 45 s | 60 s | 90 s | largest 5 s drop | C at 3:00 |
|---|---|---|---|---|---|---|---|---|---|
| shared food (today) | 1.00 | 1.00 | 1.00 | 1.00 | 0.93 | 0.84 | 0.83 | 0.57 | 49,949-50,000 |
| `wither` 0.01 | 0.90 | 0.82 | 0.74 | 0.61 | 0.41 | 0.28 | 0.15 | 0.34 | the same |
| `wither` 0.02 | 0.81 | 0.66 | 0.54 | 0.36 | 0.18 | 0.09 | 0.03 | 0.21 | the same |
| `wither` 0.03 | 0.73 | 0.54 | 0.40 | 0.22 | 0.08 | 0.03 | 0.00 | 0.29 | the same |
| `flow_loss` 0.05 (rerun) | 0.97 | 0.92 | 0.86 | 0.65 | 0.28 | 0.21 | 0.00 | 0.77 | 37,001-39,549 |

- The piece thins steadily and stays one lump: its largest part holds
  all its walkers at 20 s (`lcs20` 1.00) and it stays round
  (`compact20` 1.03-1.17) and in place (moves 0.4-4.6 dish cells).
- Its life (to under 10% of the walkers at the cut) is shorter than
  today's on 12 of 12 cuts at 0.02 and at 0.03 (sign test p 0.0005):
  40-65 s at 0.02, 39-45 s at 0.03. At 0.01 the last tenth lingers
  (42-120 s; shorter on 7 of 12, none longer).
- **Uncut C is unchanged to the walker** on 4 of 4 seeds (49,995,
  50,000, 49,949, 49,998) and eats from the same crystals (15, 13, 13,
  15 of 23). A body joined to its crystals always hears of food within
  the lag, so nothing in it withers.
- 0.02 meets both measures (half gone at 20 s, largest drop 0.21) and is
  the lead. 0.03 falls faster than asked. 0.01 is half gone at about
  40 s, nearer "half by mid-window", but slow enough that a piece can
  also run out of its pooled energy first: one cut lost 34% in 5 s.
- `flow_loss` 0.05, run again with the probe's fix below, reads as
  before: 86% left at 20 s, then a cliff (mean largest 5 s drop
  0.62), and C 21-26% smaller at 3:00 (4 of 4).

**Tried first and rejected** (both in `Reports/dead-ends.md`):
- *Withering when a walker's food in fails.* A walker whose food in over a
  second (crystal plus net sharing) did not cover half its upkeep withered
  at 0.02 a check. A cut piece thinned (53% at 20 s), but the fed body
  lives on its store between meals as much as a cut piece does: C fell to
  15-29k walkers at 3:00 (4 of 4) and reached 5-13 crystals of 23
  (`raw/cut/wither.txt`).
- *News kept only in cells.* The same news, but no walker carried it, so a
  lone explorer whose cell held no one else heard nothing and withered.
  C at 3:00 was 2-16% smaller (4 of 4) and ate from 10-13 crystals
  (`raw/cut/heard.txt`).

**A fix to the cut probe** (`examples/cutprobe.rs`). It tags the piece's
walkers and gives a newborn its neighbour's tag. A slot that died and was
born again within one tick kept the old tag, so once walkers died in the
piece, main-body births in their slots counted as the piece (under the
cell-news rule the "piece" reached crystals and moved 60-80 cells). A slot
that jumps more than 4 fine cells in a tick is now tagged as a birth. The
runs above use the fix. Today's C reads the same with it (it loses no
walker until its cliff), and the sizes of C at 3:00 never used the tags.
`flow_loss` 0.05 run again with the fix reads the same to two places.
The piece curves of the two rejected rules, where walkers died all
through the piece, read somewhat high; their rejections rest on C's size
and stand.

**In the window** (Fork, seed 1, farm hand, the same held ring at 2:30,
`cut=150,386,239,22`; all three rules filmed again on one build, since
the earlier films came from an older one; `raw/cut/film-news02.log`,
`film-today.log`, `film-loss2.log`). Walkers inside the ring:

| after the cut | 0 s | 10 s | 20 s | 25 s | 30 s | 40 s | 50 s | 60 s | 70 s | 80 s | 90 s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| shared food (today) | 272 | 289 | 289 | 289 | 289 | 289 | 289 | 289 | 289 | 18 | 1 |
| `flow_loss` 0.05 | 146 | 158 | 158 | 158 | 90 | 2 | 0 | 0 | 0 | 0 | 0 |
| `wither` 0.02 | 244 | 234 | 148 | 120 | 90 | 67 | 50 | 39 | 21 | 14 | 2 |

- With withering the piece fades over about a minute, half gone at
  +25 s; the other two are whole and then gone within about 10 s.
- The whole mould at 4:00: 28,254 walkers and 12.9% of the screen today,
  28,372 and 13.5% with withering, 18,108 and 9.5% with fading food.
  The rules act before the cut too (withering on bits the squad's shots
  cut off), so the rounds differ a little.

**Page (version 2):** https://claude.ai/artifact/W3ZnFFGTNoD8XYWjGzrSRZ
(source `cut-page/`; version 1's two films in `cut-page/v1/`). A blind
A/B/C. **The key:** Film A is fading food (`flow_loss` 0.05), Film B is
withering (`wither` 0.02), Film C is today's C. To play B:
`larder-world -- wither=0.02`; A: `larder-world -- flow_loss=0.05`.

**Built for it:**
- CPU: `BigParams::wither`, `wither_every`, `wither_lag`, `wither_reach`;
  `BigDish::news` (a step a walker) and `heard` (a step a cell, nonzero
  only where walkers stand). The pass touches only cells holding walkers.
- GPU: binding 48 `news` (a word a slot, then three blocks of cells:
  gathered, and the spread's two turns), passes `ngather`, `nspread`,
  `nab`, `nba` and `nhear` after eating, the withering in `life`, and the
  child's news in `born`. `Params` stays 896 bytes: `wither` and
  `s_wither` take the growth row's spare words (360, 364) and the clock
  takes the life-cycle row's (732, `wclock`: lag, check interval, and
  which block the spread ends in). Claimed in `ROADMAP.md`.
- Keys: `wither`, `wither_every`, `wither_lag`, `wither_reach` for the
  window, the census (`tiers::set_key`) and the cut probe.
- With the life cycle on, a spore that lands keeps the news it had when
  it fruited, so one that lands far from food withers after the lag.
  No tier uses withering yet; a tier that does should decide that.

**Cost.** GPU on software Vulkan (`mouldgpu look=DIR`, 50,000 walkers at
scale 2 with crystals and shared food, median of 11 ticks): the four news
passes take 2.14 ms a tick (`ngather` 0.71, `nspread` 0.44, `nab` 0.44,
`nhear` 0.55) against 10.8 ms for the rest of the tick; one body's `esum`
and `egain`, the same shape of work, take 1.3 ms. The GTX 1070's figure
waits for the owner's go-ahead on timing runs. On the CPU world (cut
probe, seed 1, 50,000 walkers, each run alone) a tick takes 22.8 ms with
withering against 18.4 ms without: the news pass walks every walker three
times and every cell holding one twice a step. It only runs with
`wither` on.

**The owner's pick, and the default (11-13 15:51-15:57Z).** Answering the
A/B/C: "B is the best but still too fast." Then: "Lets just double it and
call it good. I will have to playtest." So the window's shape C now
carries `wither` 0.01 (`shape_keys` in `src/bin/larder-world.rs`; the
engine default stays 0, so no other dish, tier or census changes);
`larder-world -- wither=0` plays C without it. On the CPU cuts above, 0.01
leaves 90% of a cut piece at 10 s, 74% at 20 s, 41% at 45 s and 15% at
90 s; one cut of 12 lost 34% in 5 s near the end, as its pooled energy ran
out first. Asked in the same minutes, as an inspiration ("if possible"):
"It shouldn't have a stronger attack but it should act more like a scared
animal. Either run away back to the main body or go right for the
soliders. It should be more reckless." That is the next piece of work.

**Smoke at 0.01** (CPU race world, C's keys with `wither` 0.01, seeds
1-4, cap 5:00, idle, farm and strike; `raw/smoke/wither01-cap300.txt`,
against `shared-food-cap300.txt` without it). Idle does not win (-0.333 on
4 of 4, as before); farm +0.119 (+0.120 before), strike +0.114 (+0.116).
All 12 games run to the cap; none ends under a minute and none is stuck.
The mould at 5:00 is 49,935-50,000 walkers, as before. A CPU tick takes
24-26 ms against 18-20 ms without (the news pass; the window runs it on
the GPU, 2.1 ms a tick on software Vulkan).

## A cut-off part panics (11-13)

The owner, 15:53-15:57Z, as an inspiration ("if possible"): "it would be
interesting if it got more aggressive when cut off. It needed to get back
to the core", then "It shouldn't have a stronger attack but it should act
more like a scared animal. Either run away back to the main body or go
right for the soliders. It should be more reckless", and "I understand
that is a complicated behavior that might not be possible just letting you
know my inspriation". The game designer's review (16:18Z) asked for a tell
from the simulation 1-2 s before a response ("A sudden charge with no
warning will feel like the game cheating") and a counter for each.

**The rule (`BigParams::panic`, `panic_r`, `panic_lean`; both engines; off
in `BigParams`, on at 2 in the window's shape C).** It reads withering's
news: a walker whose news is older than `wither_lag` (5 s) is cut off.
- **Target.** The nearest squad unit (soldier, larder or building, from
  `BigDish::threats`) within `panic_r` (20 dish cells; 40 fine cells at
  scale 2); else the nearest crystal pile with more than `PANIC_LEFT`
  (2%) of its charge left, measured to the pile's centre cell
  (`Pile.centre`). Distances are whole fine cells, ties to the first, so
  both engines pick the same target.
- **The tell, then the run.** For `panic_lean` steps (45, 1.5 s) the
  walker steers straight at its target at its own pace; after that at
  `panic` times it (2). Its sensors read only the direction to the target
  (a wall reads as far worse), with no wander, no random flip and no lone
  step. Its bite is unchanged.
- **Calming.** A walker joined to the fed body hears news and never
  panics; a scared one that eats at a crystal or touches the fed body
  hears it again and goes back to normal.
- **Rejected first:** the lean starting 1.5 s *before* the lag (so the
  run began at 5 s). It changed uncut C on the CPU race world (cut probe,
  seeds 1-4, 3:00): seed 3 46,941 walkers against 49,949 and 11 crystals
  eaten from against 13 (seeds 1, 2 and 4 within 21 walkers), since far
  explorers go stale for a moment and leaned. Starting the lean at the lag
  mostly fixed it: uncut C at 3:00 (`raw/panic/uncut-seed1.txt` to
  `-seed4.txt`) has 49,992, 50,000, 48,492 and 49,922 walkers against
  49,995, 50,000, 49,949 and 49,998 (seed 3 2.9% smaller, inside the 5%
  bar withering used), but it has eaten from one crystal fewer on 3 of 4
  seeds (14, 13, 12, 14 of 23 against 15, 13, 13, 15): a far explorer
  that goes stale turns back for a crystal instead of exploring on. All
  12 cuts of that probe (thin cuts across a
  tip, 4 dish cells wide) rejoined at 12 s in both arms
  (`raw/panic/wound10-lean-before-lag.txt`), so the films below use held
  rings.

**Films (the window, Fork, seed 1, farm hand, software Vulkan; page
https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP, source `panic-page/`).**
Labelled, not blind: it shows a behaviour the owner asked for. Filmed
after merging #197 (wading counts every walker), with `panic=0` for today
and the new default for scared; logs in [`raw/panic/window/`](raw/panic/window/README.md) (`c7-*`,
`b7-*`, `p4-*`; the same scenes before #197 are `c6-*`, `b6-*`, `p3-*`,
with the same story).
- **Charge.** `cut=150,365,240,24 cutsquad=365,256,150,4`: a held ring
  round a stretch of tube with no crystal, and four soldiers set down
  inside it, 34 cells from the tube. Walkers inside the ring at the cut,
  +5, +6, +7, +10, +20, +30 s: today 123, 127, 126, 123, 113, 90, 53 (the
  four soldiers live); scared 110, 76, 41, 11, 8, 7, 5 (soldiers 4, 4, 4,
  3, 3, 3, 1). The scared piece gathers and turns at +5 s and reaches the
  soldiers at +6 s; their 6 shots kill 94 walkers; the last ~10 sit on
  the soldiers, no more shots are fired, and three soldiers are dead by
  +25 s. At 34 cells the visible tell is about a second: a walker's own
  pace covers most of the gap during the lean.
- **Bolt.** `cut=150,386,239,22,8,16`, no soldiers: a ring held 8 s round
  a piece of tube near a crystal. The scared piece bunches on the ring's
  crystal side and, when it opens, pours into the crystal's lobe, leaving
  its ground empty; today's joins back up where it stands. Walkers inside
  at 0, +5, +8 (opens), +9, +12, +20, +30 s: today 252, 271, 268, 289,
  265, 220, 189; scared 199, 214, 211, 186, 160, 104, 50.
- The rule acts before the cut, so each pair's rounds differ by 2:30
  (18,512 walkers today, 22,825 scared; the charge scene's scared squad
  had one soldier left and was made up to four).

**Counters (the designer's second point).** A charge: shoot it before it
closes (four soldiers' shots stopped 94 of about 105; more guns or more
range wins). A run for a crystal: a wall in its way stops it (a wall reads
as far worse to its sensors, and it bites it as any walker does); a squad
on its route makes it charge instead; if it reaches the crystal it eats,
calms and breeds there.

**What it changes before any cut.** The squad's own shots knock small
bits off, and those charge it now. Few walkers are scared at once: a CPU
game on Fork (farm hand, seed 1, `larderworld look=`, every 10 s to 2:30)
has 0-16 scared of 13,000-27,000, yet its squad is 4 at 2:30 against 8
without (`raw/panic/window/cpu-fork-farm-*.log`). In the window (Fork,
seed 1, farm, after #197; `p4-*.log`) soldiers today at 1:00, 1:20, 1:40,
1:50, 2:00, 2:30: 19, 13, 12, 12, 10, 14; scared 17, 10, 6, 2, 1, 1. The
scared round's farm hand stops harvesting at 1:50 (6,262 food against
11,430 by 2:30) with about 1,000 in the bank, under its 1,200 buying
line, so it never rebuilds; a player could still buy two soldiers. One
round, so a tuning note, not a measure; but this is the biggest change
the rule makes, and it makes standing and shooting costlier, which the
owner's rulings treat as the safe play. Knobs if play finds it too much:
a shorter `panic_r` (20 dish cells reaches past a gun's 15), or only
pieces above a size panicking (not built). Before #197 the same round
had 3 soldiers against 0 at 2:00 (`p3-*.log`).

**Smoke** (CPU race world, C's keys with
`panic` 2, seeds 1-4, cap 5:00; `raw/smoke/panic2-cap300.txt` against
`wither01-cap300.txt`): idle -0.333 on 4 of 4 (as before), farm +0.119
(+0.119), strike +0.116 (+0.114); paired by seed, farm 1 higher, 1 lower,
2 tied, strike 2 higher, 2 tied. All 12 games run to the cap, none stuck;
the mould at 5:00 49,906-50,000; idle's squad keeps 3-4 soldiers and
farm's 7-8, as before.

**Cost.** A calm walker reads its news word and compares it (withering
already keeps it); only a scared one searches up to 96 units and 64 piles.
GPU on software Vulkan (`mouldgpu look=DIR`, 50,000 walkers at scale 2,
crystals, shared food, withering 0.01; two runs each, uncut so few
walkers are scared): `sense` 2.27 and 2.11 ms without, 1.86 and 2.15 ms
with; whole ticks 25.0 and 25.2 ms against 25.4 and 25.7 ms; the dish at
40 s the same to the walker. No measurable cost; the GTX 1070's figure
waits for the owner's go-ahead on timing. CPU (the smoke): 23.3-25.6 ms a
tick against 23.7-26.2 ms.

**Built for it:**
- CPU: `BigParams::panic`, `panic_r`, `panic_lean`; `BigDish::threats`
  (fine cells, set by `squad::frame` from the frame's units), `panics`,
  `panic_haste`, `pile_centre`; the news pass runs when either `wither` or
  `panic` is on.
- GPU: `Params` stays 896 bytes: `panic_r2` (460, the squared reach in
  fine cells) and `panic` (484) take the species row's spare words, the
  lean takes `wclock` bits 25-31, and `Pile`'s spare word is `centre`.
  Claimed in `ROADMAP.md`. `Engine::unblock` puts cells back on the agar.
- Keys `panic`, `panic_r`, `panic_lean` for the window, the census and
  the cut probe. `larder-world look=DIR cut=T,X,Y,R[,HOLD[,WIDTH]]
  cutsquad=X,Y[,T[,N]]` stages the scenes (a ring that opens after HOLD;
  N soldiers set down and kept there, made up from the larder if short).
  `larderworld look=DIR` prints `scared` beside each picture.

**Next, if play asks for it:** a slower lean (the tell is short when
soldiers are close), a shorter reach or a size floor (above). Seen in the
charge film: no shot is fired at the ~10 walkers sitting on the soldiers,
though the owner ruled (11-13 16:15Z, #197) that a soldier rushed while
standing still must be able to shoot what reaches it; passed to the
thread that owns soldiers' fire.

## Tests

- **Panic, GPU twins.** `the_world_matches_the_cpu_for_one_tick` gains
  "panic: cut-off walkers run for a crystal" (4-step lag, a 2-step lean;
  every other walker's news stale, every fourth inside the lean): 4,518
  walkers scared in the tick, some leaning and some running, the farthest
  moved 2.00 fine cells (twice the step), 0 of 15,000 walkers off.
  `the_squad_matches_the_cpu_for_one_frame` gains "plain, a cut-off part
  charges the squad" (`panic_r` 10, stale walkers round the squad): 0
  slots and 0 walkers off, walkers charging. Both run on the final rule.
- **Panic, CPU.** `a_cut_off_walker_runs_for_a_crystal_or_a_unit`: panic
  changes nothing while news is fresh (same hash); with stale news the
  walkers' mean distance to a crystal falls from 87.5 to 77.7 cells in 30
  ticks (87.3 without panic); with a unit in reach they close on it
  (164.8 to 157.5, against 164.8), and with `panic_r` 0.5 they run for a
  crystal instead.

- **Withering, GPU twin.** `the_world_matches_the_cpu_for_one_tick` gains
  two cases, news spread twice and three times a step (so every news
  pass runs and the spread ends in each block). The warm-up makes the
  news with a lag no walker reaches; every other walker's news is set
  stale before the tick, which has a 4-step lag, checks every step and
  even odds. Result: 2,005 and 1,967 walkers die in the tick, 0 slots
  live or die apart, 0 of the living walkers' news differs, 645 and 667
  stale walkers heard again, and the gathered block is clear after. With
  the spread's last block read wrong (planted), 10 slots and 137 news
  differ and the test fails.
- **Withering, CPU.** `food_news_travels_through_the_body_and_not_across_a_gap`
  (a row of walkers: news crosses two cells a step, is carried on, and
  stops at an empty cell) and `a_walker_without_news_withers` (0 changes
  nothing, news that never goes stale kills no one, stale news thins the
  mould).

- **GPU twin.** `the_world_matches_the_cpu_for_one_tick` gains the case
  "fed crowd: a fed walker packs two to a cell". Every other walker is
  fed to 3 before the tick. Result: 0 of 15,000 walkers off, energy L1 0,
  289 cells holding two walkers on both engines, and 1,513 born.
- **Clearing the move's claims.** The twin found that the GPU must clear
  the move's claims before births when a cell's winner leaves room for a
  fed parent's child. Without the `unclaim` pass after the walk, 75 of
  15,000 walkers were off.
