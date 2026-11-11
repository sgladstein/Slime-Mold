# Who reaches food first: droplet moulds and their mixes (2026-11-11)

A measured test, no game change. It replaces the first crystal-mix test,
which the [independent review](research-review-2026-11-11.md) (section 5,
C5) showed was mostly measuring where the start disc landed. The Larder
thread asked for mixes with a clear strength and a clear weakness for its
ladder of mould species. Page: https://claude.ai/artifact/AYs2JNZBfvKZ4NTUTV691D
(source in [`first-reach-2026-11-11/`](first-reach-2026-11-11/), shared with
the [coexistence test](species-sets-2026-11-11.md)).

## Setup

- **Dish.** The 1024 x 640 big dish (`preset=dense`), 12 plain food piles,
  60 s. 30,000 droplet walkers (crowd 1, sensor angle 90, turn 45, sensor
  offset 9, deposit 5, keep 0.93, `blocked_deposit` 1) start in a disc of
  50 dish cells at the dish centre (`blob=1 blob_r=50`).
- **Arms** (`run.sh`):
  - `droplets`: the above.
  - `grazers`: plus `graze_weight` 10 (they eat a regrowing field and steer
    away from bare ground).
  - `grazescouts`: grazers, plus 20% scouts (`mix_reach` 3, `mix_deposit`
    0.3).
  - `dropscouts`: droplets plus the same scouts.
  - `dropfast`: droplets plus 20% that step twice as far and lay 0.3x.
  - `scoutsonly`: every walker a scout (`mix_share` 1). This is the
    review's control for "the kinds sort themselves".
- **Scored piles.** A pile counts if its index is 1 or more and its centre
  is more than 66 dish cells from the start: the disc, a pile's radius and
  a 10-cell margin. That leaves 77 piles over 16 seeds (2-7 a seed); the
  other 99 touch the start and are reached at once. Pile 0 is never scored.
- **Reached** means trail over the tube cut (1.0) on a pile cell, recorded
  by the new `firsts` line in `examples/bigdish.rs`.
- **Deciding measure, named before the run** (in `run.sh`): scored piles
  reached by 30 s. Each arm is compared with its base on paired seeds 1-16,
  by a sign test with ties dropped, with Holm over 5 comparisons.
- **Disclosure.** The measure was chosen after seeing the flawed run's
  "piles reached" column on seeds 1-8. There, grazing outriders reached
  10-12 piles against 5-9 for plain droplets, with the start covering 3-6
  piles. So the direction for grazers, grazing outriders and fast droplets
  was known. Seeds 9-16 had not been run before, and they are shown
  separately below.

## Result

Scored piles reached by 30 s, summed over 16 seeds (`score.txt`):

| Arm | Reached by 30 s | By 60 s | Against its base, 30 s | Holm p |
|---|---|---|---|---|
| Droplets | 9 of 77 | 10 | (base) | |
| Grazers | 49 of 77 | 56 | more on 15 seeds, fewer on 0, +40 | 0.0003 |
| Grazing outriders | 62 of 77 | 72 | over grazers: more on 10, fewer on 0, +13 | 0.0078 |
| Fast droplets | 19 of 77 | 23 | more on 7, fewer on 0, +10 | 0.047 |
| Scouts only | 3 of 77 | 4 | more on 1, fewer on 6, -6 | 0.25 |
| Bridged droplets | 5 of 77 | 6 | more on 0, fewer on 3, -4 | 0.25 |

- **Grazing is what makes a droplet mould forage.** Plain droplets stay as
  a still field of bodies; the few piles they "reach" sit just past the
  start. Grazers send small droplets off toward the piles they smell. They
  reached piles up to 113 dish cells out (the wall is at about 128 along
  the long axis), with a median first reach of 7.7 s.
- **Scouts help grazers and nothing else.** Grazing outriders beat plain
  grazers on 10 seeds and lost on none. They reached 94% of the scored piles
  by 60 s. Scouts on their own reached fewer than droplets (not
  significant). Scouts added to still droplets pull them into chains and
  reach a little less. So the gain needs both: bodies that crawl, and
  scouts that haze out ahead of them.
- **Fast droplets** survive Holm at 0.05, barely, and only on 7 of 16
  seeds (9 ties).
- **Fresh seeds 9-16 alone** go the same way. Grazers beat droplets on 8 of
  8 (Holm 0.039 over 5). Grazing outriders over grazers were 5 more, 0
  fewer and 3 ties, and fast droplets also 5, 0 and 3 (neither significant
  on 8 seeds).
- **Near the wall.** Of scored piles at under 0.8 of the dish radius,
  grazers reached 22 of 28 by 30 s against 2 of 28 for droplets; at 0.8 and
  over, 27 of 49 against 7 of 49 (`summary.txt`). The gain is not a wall
  ride.

## Speed, for the game

The owner, 14:59Z, on explorers filling the dish in 10 s: "this is crazy
fast from the game perspective". These moulds are slow. The median 90th
percentile walker distance, from a start disc whose own r90 is 190 fine
cells, was:

| Arm | r90 at 30 s (fine cells) | At 60 s |
|---|---|---|
| Droplets | 192 | 197 |
| Grazers | 215 | 228 |
| Grazing outriders | 253 | 270 |

So the edge of a grazer mould's body moves about 10 dish cells in a
minute, on a dish 256 dish cells wide, and its middle hardly moves (r50
142 to 147). Only its small droplets and its scouts go far.

## Cost

No new rule runs here. Every arm costs 12.6-14.2 ms a tick on one core at
30,000 walkers (median at 30 s and 60 s, `summary.txt`), with no arm
clearly dearer than another.

## Limits

- Plain piles, not crystals. Nothing here depletes, and the walker count
  is fixed (no W0 growth).
- `mix_reach` scales the trail and wall sensors only. Scouts smell food at
  the same `scent_offset` as everyone else (the review's point).
- "Reached" is trail over 1.0. A scout lays 1.5 a step, so one scout on a
  pile can count. Scouts alone still reached the fewest piles.
- The 1M-walker network arms were left out: about 25 min a run on one core.
- The binary was built from this thread's branch at 16:18Z, before main's
  Larder W1a was merged in. Rerun at main (3dabcb7) on one seed of each
  test, grazing outriders seed 3 and the Drifter alone seed 1, the output
  and end hash are the same, apart from the newer `crystals` column and
  the timing ([`main-check/`](first-reach-2026-11-11/main-check/)).

## Raw

- [`first-reach-2026-11-11/raw/`](first-reach-2026-11-11/raw/): program
  output, one file per arm and seed.
- [`score.txt`](first-reach-2026-11-11/score.txt): `score.py` at 30 s and
  60 s, and on seeds 9-16 alone.
- [`summary.txt`](first-reach-2026-11-11/summary.txt): `summary.py`.
- The flawed crystal-mix runs are not kept as a result. They are described
  in the review's response.
