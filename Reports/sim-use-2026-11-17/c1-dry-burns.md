# C1: starving slime burns, fed slime doesn't (2026-11-17)

The simulation-use plan's C1 ([plan.md](plan.md), draft 3, "C1. Dry slime
burns"). The owner ruled at 01:13Z: "fed slime is fireproof; starving slime
burns". Page: https://claude.ai/artifact/6CrHne8QszUbfgVzpcG825 (source `page-c1/`).

## What was built

- **The rule** (`BigParams::fire_dry`, key `fire_dry=1`; **on by default**,
  the owner's ruling). The trail wets a cell against fire (`fire_damp`)
  only where the slime there is fed. A cell's 3 x 3 decides it. Walkers
  whose food news is older than the wither lag, with none fresh among them,
  leave it dry; so does a set walker (E2a), fresh news or not. Fire still
  needs fuel under it (plants or crust). A fresh walker beside a stale one
  keeps the cell from drying, so the fed side of a junction never burns.
- **No new state.** It reads the food news the wither already spreads (the
  CPU's `heard`; on the GPU the spread's last block in `news`, which `nhear`
  reads) and the set count of E2b (`set_cells`). Both are read as fire steps,
  after the tick's last step, on both engines (`BigDish::slime_fire`, the
  shader's `slime_fire`). It needs the wither or panic on, which the default
  slime (shape C) has. With them off nothing changes.
- **The stronger form** (`BigParams::fire_fed`, key `fire_fed=1`, off): a
  cell by fresh news is wet whatever its trail. Built after the first look.
  With the trail alone, fire on full meadow burns the thin trail between a
  fed body's tubes, taking 65% of a fed disc in 20 s (below). Whether fed
  slime should be wholly fireproof is the owner's call (card in the thread).
- `plants::fire_step` takes the slime's marks (`SLIME_DRY`, `SLIME_WET`). The
  GPU reads both flags from the scar row's spare word (`fire_dry` bit 0,
  `fire_fed` bit 1).

## Tests

| Test | Result |
|---|---|
| `stale_and_set_slime_is_dry_and_fed_slime_wet` (CPU, the rule cell by cell) | stale news dries its 3 x 3; fresh beside stale doesn't dry; a set walker dries it beside fresh; thin trail unmarked; with `fire_fed`, fed thin trail wet and stale or set still dry; news exactly the lag old is fresh |
| `dry_slime_burns_on_the_gpu_as_on_the_cpu` (llvmpipe, flats map in plants, wither lag 2, every plant within 20 dish cells of home lit at 12 s) | plants, fire, char exact at every second, both rules; planted, a GPU blind to `fire_dry`: fire off by 0.59; blind to `fire_fed`: 0.73. Fire killed 164 with `fire_dry` against 157 without, 0 with `fire_fed` too |

With `fire_dry` on by default, the whole suite passes unchanged
(`cargo test --release`, llvmpipe for the GPU tests): no existing test
lights fire by slime that is out of the news.

## Measure: a cut part on grass

`examples/dryprobe.rs`, raw output in `c1-raw/`. Planet 2 under full meadow,
shape C at its 25,000 walkers on the CPU race world (scale 2), grown 60 s.
A disc of 10 dish cells on the body with no crystal within 35. Cut: an
8-cell ring struck round it and the rest of the slime removed, so it can't
rejoin. Joined: the same disc untouched. Lit (every plant in the disc) once
half the cut part's walkers have stale news, or at 15 s if they never do,
at the same tick in every arm; fire kills counted over the next 20 s, per
walker in the disc at the light. **Deciding measure, named before the run:
kills per walker, cut against joined, paired by seed.** The guard: joined
with the rule must burn no more than without.

Kills per walker in the disc at the light, 20 s after it (`c1-raw/`; seeds
1-4 from `dryprobe-seeds1-8.txt`, 5-8 from `dryprobe-seeds5-8.txt`, the
first file's run stopped after seed 5 to run 5-8 beside it):

| Seed | Lit at | Cut part (stale) | Joined disc | Cut, rule | Joined, rule | Cut, old | Joined, old | Cut, fed-proof | Joined, fed-proof |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 4.1 s | 937 (937) | 1,203 | **1.05** | 0.65 | 0.06 | 0.65 | 1.05 | **0** |
| 2 | 15 s | 619 (0) | 949 | 0.46 | 0.24 | 0.46 | 0.24 | 0 | 0 |
| 3 | 4.3 s | 936 (936) | 1,162 | **1.14** | 0.37 | 0.04 | 0.37 | 1.04 | **0** |
| 4 | 3.9 s | 986 (986) | 1,192 | **1.00** | 0.57 | 0 | 0.57 | 1.00 | **0** |
| 5 | 2.7 s | 1,035 (1,035) | 1,177 | **1.01** | 0.33 | 0.003 | 0.33 | 1.01 | **0** |
| 6 | 4.3 s | 887 (887) | 1,166 | **1.09** | 0.34 | 0.02 | 0.34 | 1.06 | **0** |
| 7 | 15 s | 0 | 14 | 0 | 0 | 0 | 0 | 0 | 0 |
| 8 | 4.2 s | 334 (334) | 472 | **1.13** | 0.58 | 0.04 | 0.58 | 1.02 | **0** |
| mean | | | | 0.86 | 0.38 | 0.08 | 0.38 | 0.77 | 0 |

(Over 1 a walker: the part breeds while it burns, and those die too.)

- **Cut against joined, with the rule (deciding):** the cut part burnt more
  on 7 of 7 untied seeds (sign p 0.016). Where it went stale (6 seeds) the
  fire took all of it in 20 s, against 0-6% of it with the old rule (6 of
  6, p 0.031).
- **The guard holds:** the joined disc burnt exactly as much with the rule
  as without on all 8 seeds (no fed cell changes by construction).
- **But the fed body was never fireproof:** with the trail alone the joined
  disc lost 24-65% (mean 38%) in 20 s on full meadow. The thin trail between
  its tubes is under `fire_damp` 3. The cut part, pulled into thick tubes,
  lost almost none with the old rule.
- **Fed fireproof too (`fire_fed`):** the joined disc lost nothing on 7 of
  7 untied seeds (p 0.016), and the stale cut parts still burnt whole (6 of
  6 against joined, p 0.031). On seed 2 the cut part found food before its
  news went stale; there it burnt 46% with the trail alone and 0 fed-proof.
- Seed 7: the picked disc emptied before the light (14 walkers joined, none
  cut); no data.
- None of the five comparisons survives a Bonferroni correction at 8 seeds
  (0.05 / 5 = 0.01; the best is 0.016), but every untied pair goes the same
  way.

## How the measure got here

Each of these first tries left nothing stale to burn (seed 1):

- A 3-cell ring struck once, lit at 10 s: the body closed the gap within
  seconds, 0 of 1,116 stale.
- The ring held open every tick: the part panicked out through it and died
  there, none left in the disc at 10 s.
- An 8-cell ring struck once: the oldest news anywhere 15 s on was 83 steps,
  under the lag of 150.
- The part alone but a crystal 14 dish cells away: it found it, 0 of 1,395
  stale at 14 s.
- On the planet's own plants (patches): two small patches in the disc burnt
  out within 2 s whatever the rule.

So in the game as it is, a part cut by a gap rejoins or finds food before
its news goes stale. The rule bites where a part stays cut: behind walls or
water, specks and breakaways far from food, a part whose crystal ran out,
and set slime (which E2a makes under fire).

## Frame cost

Off: nothing (one test a cell in the fire pass). On, in the fire pass,
which runs every plant step (every game tick in the run): a 3 x 3 of news
loads a cell where the trail would wet it (and with `fire_fed`, every cell),
plus a 3 x 3 of set counts with setting on. On the CPU the same as one pass.
Not timed on the 1070.

## Left

- The owner's card: `fire_fed` on or off.
- 1070 timing with the rest of the plan's passes.
