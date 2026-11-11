# Two species with their own settings on one dish (2026-11-11)

A measured test of the per-species settings (`BigParams::species_sets`,
PR #61). The independent [review](research-review-2026-11-11.md) ranked
them as direction 2: the Larder ladder needs moulds with different rules
on one map, and until now every species shared one set of numbers. Page:
https://claude.ai/artifact/AYs2JNZBfvKZ4NTUTV691D (shared with [first reach](first-reach-2026-11-11.md); source in
[`first-reach-2026-11-11/`](first-reach-2026-11-11/)).

## What was built (PR #61)

- `SpeciesSet` holds one species' sensor angle and offset, turn, step,
  deposit, keep, crowd, `blocked_deposit`, `graze_weight` and
  `explore_steps`. `BigParams::species_sets` is empty by default, and every
  species then uses the global numbers, so the default dish is unchanged
  (hash `869141a7bf398adf`).
- Each walker moves, claims, lays and grazes with its own species' set,
  and each species' trail fades with its own keep.
- `examples/bigdish.rs` takes `sN_field=` keys (for example
  `s0_crowd=1`), and prints an `own` line: each species' walkers, its own
  area (dish cells over the tube cut scaled by its deposit) and its pieces.
- Tests: sets equal to the globals give the same hash; two different sets
  give different grazing and crowding; keep is per species.
- Cost: 105-108 ms a tick at 1M walkers and 12.6-13.2 ms at 30,000, with
  and without sets (within noise).

## Setup

- **Dish.** The 1024 x 640 big dish, no food, 60 s, two species in
  separate halves (`species_zones`), avoidance 1 (each species avoids the
  other's trail).
- **Species 0, the Drifter**: grazing droplets (crowd 1, sensor angle 90,
  turn 45, sensor offset 9, deposit 5, keep 0.93, `blocked_deposit` 1,
  graze 10), 29,870 walkers in the left half.
- **Species 1, the Sprawler**: the Jenson network (crowd 0, sensor angle
  45, turn 22.5, sensor offset 18, deposit 0.5), 1,000,130 walkers in the
  right half.
- **Arms** (`species-sets-2026-11-11/run.sh`): both together; the Drifter
  alone (same walkers, same half); the Sprawler alone.
- **Deciding measure, named before the run:** each species' own area at
  30 s and 60 s, together over alone. They coexist if the ratio is within
  0.5-2 for both species on 8 of 8 seeds. Secondary: the Drifter's pieces
  (bodies).
- Seeds pair the dish but not the walkers: a walker's species comes from
  its index, so the alone arms start their walkers at other spots.

## Result

**They coexist: both ratios are within 0.5-2 on 8 of 8 seeds, at 30 s and
at 60 s** (`ratios.txt`).

| | Drifter, together over alone | Sprawler, together over alone |
|---|---|---|
| 30 s | 0.99-1.02 | 0.68-1.89 |
| 60 s | 0.99-1.02 | 0.65-1.90 |

- **The Drifter does not notice the network.** Its own area is within 2%
  of its area alone on every seed, and 97-99% of its trail stays in its
  half.
- **The Sprawler varies.** On five seeds it ends at 0.65-1.06 of its area
  alone. On seeds 3, 6 and 7 it ends at 1.59-1.90, and its tubes run into
  the Drifter's half. There, the share of tube cells laid mostly by one
  species falls to 0.963-0.980, against 0.995-0.999 on the other seeds,
  and 5.5-8.6% of tube cells change their leading species in 10 s, against
  0.4-1.5%. The droplets don't react to the tubes.
- **Secondary: the Drifter forms fewer bodies beside the network.** At
  60 s it had 40-47 pieces together against 44-53 alone, fewer on 8 of 8
  seeds (p 0.0078, which survives a correction over the two times). At 30 s
  it was 4 fewer and 4 more.
- **Cost.** On seeds 1-4, run two at a time on 4 cores, the dish with both
  species took 214-222 ms a tick at 30 s, the Sprawler alone 194-206 and
  the Drifter alone 17.8-18.3. So the second species, with its own trail
  field, adds about 10%. Seeds 5-8 shared the cores with up to 5 other
  runs and are slower (`raw/`).

**For the ladder.** Two species with different rules can share one map,
and neither wipes the other out. With avoidance as the only rule between
them there is no fight: the network can push tubes across, and the
droplets carry on. A rival-pair tier needs a contact rule (conversion,
or the fusion and rejection fights the behaviours thread is testing).

**Limits.** No food, a fixed walker count (no W0 growth), the species
start sorted into halves, and no conversion. One pair of species was
tried.

## Raw

- [`species-sets-2026-11-11/raw/`](species-sets-2026-11-11/raw/): program
  output, one file per arm and seed. Seeds 5-8 were rerun after a machine
  restart killed them.
- [`ratios.txt`](species-sets-2026-11-11/ratios.txt): `ratios.py` on the
  raw folder.
- The pictures on the page are 3 seeds at 1, 5, 10 and 20 s, made before
  the census (`look=`, `style=species`).
