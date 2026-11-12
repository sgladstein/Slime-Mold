# The player's own mould: do body forms counter each other? (2026-11-12)

The owner approved a player-owned mould species on 2026-11-11 at 14:54Z,
and on 2026-11-12 at 04:45Z chose its shape on a card: **gardeners**, where
soldiers grow the player's mould (feed it, lay its roads, cut it) and it
fights the enemy mould by conversion. The proposal, written from the model
rather than from the old demo's tools, is the Claude Doc
https://claude.ai/artifact/LBDQgb8x9RoH6bE11G8H3m; its text is copied in
[`player-mould-2026-11-12/proposal.md`](player-mould-2026-11-12/proposal.md).

This report is the probe the proposal named first: whether the model itself
gives body forms that beat one form and lose to another, so that a
player's choice of form is a strategy and not a stat. No game rule and no
default changed.

```
cargo build --release --example ownmould
./target/release/examples/ownmould census seeds=1-16 forms=net,hunk,scout,heavy threads=4
./target/release/examples/ownmould look=DIR seeds=2 forms=net,hunk secs=180 every=90
```

## Setup

- **World**: per seed, the race world (`larder_world::race_params`, scale
  2, 5,000 walkers growing in 50,000 slots, crystals that run dry, rock and
  wet ground) taken as a map with its left half mirrored onto the right.
  Two species start on mirrored home crystals, 2,500 walkers each.
- **Fight**: conversion over a square (`species_convert` 1.2,
  `species_convert_r` 4), species blind to each other's trail
  (`species_avoid` 0).
- **Forms** (`SpeciesSet`s from the race mould): `net` (the race mould),
  `hunk` (the Drifter droplets: crowd 1, sensor 90, turn 45, offset 9,
  deposit 5, keep 0.93, not grazing), `scout` (explorers, 60 steps),
  `heavy` (three times the deposit, half the step).
- **A game** of A and B is played with A on each side and A's measure is
  the mean, so the per-seed side bias cancels (on seed 1 the right side
  out-ate the left 0.641 to 0.359 with the same form on both).
- **Deciding measure, named before the run**: A's share of the crystal
  charge both species ate over 3 game minutes (each walker's `took`, by
  its species). A beats B on 14 or more of 16 seeds (sign test p 0.004,
  inside Bonferroni over 6 pairings).

## Looking first: two fixes before the census

1. **Raw trail makes deposit a stat.** Conversion compared trail, so
   `heavy`, laying three times as much, converted the race network whole:
   on seed 1 the network fell from 2,500 to 50 walkers by 60 s and 0 by
   3:00. A switch now weighs each species' trail over its own deposit
   (`BigParams::species_convert_per`, off by default, CPU only), so
   conversion counts walkers' presence. The census runs with it on.
   Test: `conversion_per_deposit_counts_presence` (raw trail leaves under a
   tenth of the light species, per deposit over nine tenths).
2. **Grazing hunks fill the slot cap.** With grazing on and growth on, the
   hunks ate the grass and bred to 49,744 walkers by 3:00 on seed 1. The
   census's hunks don't graze.

## Who eats more (the deciding measure)

| A vs B | A ahead (of 16) | p | Median A share | Mean walkers at 3:00 A / B | Crystals held A / B |
|---|---|---|---|---|---|
| net vs hunk | **14** | **0.004** | 0.570 | 4,324 / 1,345 | 6.0 / 2.0 |
| net vs scout | 11 | 0.21 | 0.598 | 4,635 / 4,209 | 5.8 / 5.5 |
| net vs heavy | 7 | 0.80 | 0.485 | 3,101 / 3,661 | 4.0 / 5.8 |
| hunk vs scout | 5 | 0.21 | 0.457 | 1,400 / 3,914 | 2.0 / 5.3 |
| hunk vs heavy | 6 | 0.45 | 0.471 | 1,296 / 2,036 | 1.9 / 3.4 |
| scout vs heavy | 4 | 0.08 | 0.455 | 2,460 / 3,549 | 3.1 / 6.0 |

Only net over hunk survives. Read as a ranking it is a ladder, not a cycle:
net and heavy, then scouts, then hunks. The proposal's pass bar (no form
beats all three others) is met only because few pairings reach
significance. Raw: [`raw/census-per1.txt`](player-mould-2026-11-12/raw/census-per1.txt).

## Who wins where they touch

Counting walkers that change species (alive before and after a tick)
showed that the census above mostly measured an expansion race: on seed 2
no pairing turned more than 125 walkers in 3 minutes. But where the bodies
did meet, the fight went the other way from the eating:

| A vs B (8 seeds) | Seeds with contact | Contacts won by A / B | Turned to A / B, mean a game |
|---|---|---|---|
| net vs heavy | 7 | 0 / 7 (the 8th: 1 vs 0) | 673 / 2,048 |
| net vs hunk | 7 | 0 / 7 | 36 / 399 |
| hunk vs heavy | 6 | 5 / 1 | 110 / 6 |

A seed is won by the side more walkers turned to. 7 of 7 is p 0.016, just
inside Bonferroni over the three pairings. So the slow, dense forms win
fights and the fast network wins the economy: **the network out-eats the
hunks (14 of 16) and loses every contact to them (7 of 7).** That is two
axes, not one ladder, and it is the opening a gardener needs: whether to
spread your mould to eat or bunch it to fight is a choice with a cost.
Raw: [`raw/turned.txt`](player-mould-2026-11-12/raw/turned.txt).

## The fight's settings (4 seeds, net vs heavy)

| Settings | Turned to net / heavy, a game | Net's median share |
|---|---|---|
| convert 1.2, r 4, blind (census) | 673 / 2,048 (8 seeds) | 0.485 |
| convert 1.0, r 4, blind | 283 / 861 | 0.594 |
| convert 1.05, r 8, blind | 584 / 1,225 | 0.545 |
| convert 1.2, r 4, attracted (avoid -0.5) | 449 / 2,077 | 0.316 |
| convert 1.0, r 8, attracted (avoid -0.5) | 953 / 2,366 | 0.318 |
| convert 1.0, r 4, attracted (avoid -1) | 598 / 1,958 | 0.278 |

Attraction (a species drawn to the other's trail) brings the bodies into
contact and the dense form then takes the economy too (net's share falls
to about 0.3). Four seeds each: direction only. Raw:
[`raw/sweep.txt`](player-mould-2026-11-12/raw/sweep.txt).

## What this means for the gardeners build

- Form is not a counter cycle by itself; it is a trade between eating and
  fighting. The soldiers are what can make it a choice: Road spreads your
  mould to eat, Feed and Cut bunch it to fight.
- The fight needs contact. With the species blind to each other it is rare;
  the enemy tiers can be made to seek the player's mould (attraction) where
  a level wants a fight.
- Cost: `species_convert_per` adds one division a species a walker to
  conversion (not timed; conversion over a square of radius 8 costs 1.3 ms
  a frame on the 1070 at 100,000 walkers, [species on the
  GPU](species-gpu-2026-11-11.md)). It is CPU only; the GPU port is the
  GPU thread's when the game turns it on.

## Not done

- Crust and fliers: the life-cycle rates are global, not per species, so
  those forms need per-species life settings first.
- The one body thread's cohesion rules (energy flow, hunger scent, tether)
  change every form; the matrix should be rerun when they land.
- Pictures: [`player-mould-2026-11-12/pics/`](player-mould-2026-11-12/pics/),
  seed 2 at 0:01, 1:30 and 3:00 for each pairing, counts in `looks.txt`.
