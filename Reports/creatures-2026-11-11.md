# Creatures that move and fight on the big dish (2026-11-11)

Research with small tests, no game change. Follows
[`outside-rules-2026-11-11.md`](outside-rules-2026-11-11.md), which found
still droplets (`blocked_deposit`) and rival species but nothing that moved
as a body. The owner, 04:17Z: "Sound good. Keep exploring", and overnight:
"Keep building autonomously overnight". The two open questions were making
droplets crawl and making species fight over a border. Page: https://claude.ai/artifact/YYZJSPdm6VAb857m7WnJLR
(source and media in [`creatures-2026-11-11/`](creatures-2026-11-11/)).

## Summary

- **Grazing makes droplets crawl (`graze_weight`, `graze_eat`,
  `graze_regrow`).** A grass field starts at 1 on every cell; each step
  every walker eats 0.05 from its cell and grass regrows by 0.002 x (1 -
  grass); a sensor adds `graze_weight` x grass where it reads. A droplet eats
  the ground under it bare and leans toward fresh ground, so it keeps moving.
  The idea is Primordia's "Symbiosis" grazing
  ([card](../research/sources/primordia.md)), cut down to one field. On the
  droplet dish (30,000 walkers, one a cell, SA 90, RA 45, SO 9, deposit 5,
  keep 0.93, `blocked_deposit` 1), weight 10 against 0, 8 seeds, deciding
  measure named before the run, the median distance body centres moved over
  the last 2 s of 10 s: **6.5-7.6 cells against 0.67-0.77, more on 8 of 8,
  p 0.008**. Bodies stay bodies: 168-190 against 179-193 of them, about 200
  cells each. Cost: 12-15 ms a tick either way (one core). At weight 30 many
  walkers leave their bodies; at 100 the bodies dissolve into a gas (seed 1).
- **Two species of grazing droplets** (`species` 2, avoidance 0) make amber
  and teal cells that roam among each other and stay one colour each (80%+
  one species on 0.91 of tube cells at 20 s). This is the most
  creature-like dish so far. Seed 1.
- **Conversion (`species_convert`) does not make a fight yet.** A walker
  joins another species when that species' trail at its cell outweighs its
  own by the factor given (with `species_cycle`, only the species that chases
  it can take it). On two equal halves of the 1M network the trail shares
  stayed 0.49-0.51 over 12 s (avoidance 0, 0.5 or -0.5, factor 1.2-2): the
  bodies meet only along a border where both sides are equal. On grazing
  droplets, two species stayed at 0.50 each, and three cyclic species moved
  only 0.32-0.34 apart over 30 s while the cells blurred into a mixed soup.
  Seed 1 each. A fight needs an imbalance the player can make (more walkers,
  food, a push), which these symmetric dishes do not have.

## An edge in the fight (`species_boost`)

Species 0 lays `species_boost` times the deposit. On the two halves of the 1M
network with conversion 1.2 and no avoidance, boost 1.5 against 1.0 (even),
8 seeds, deciding measure named before the run: species 0's share of
walkers at 16 s. **0.448-0.518 (median 0.474) against 0.450-0.580 (0.500):
lower on 6 of 8, p 0.29.** A louder trail is not a lasting edge. It wins
early (share at 8 s higher on 6 of 8) and then loses walkers (falling from
8 to 16 s on 7 of 8): the boosted side pushes its tubes into the other
half, where they are outweighed and convert. Even games swing too (0.45 to
0.58 at 16 s), so conversion does move walkers; what it rewards is not yet
clear. On seed 1 over 30 s, the boosted side wrapped tubes round the rim,
lost them, and ended with 0.34 of the walkers, the two bodies drawn apart
with an empty band between them. Script:
[`creatures-2026-11-11/fight.sh`](creatures-2026-11-11/fight.sh).

## An edge in numbers (`species_share`), and hunting droplets

`species_share` gives species 0 that share of the walkers, interleaved down
the walker list; with `species_zones` the halves stay equal, so the bigger
side is denser, not wider. `species_convert_r` (new) makes conversion compare
each species' trail summed over the (2r+1) x (2r+1) square round the walker
(one summed-area table a species a step, about +5-10 ms a tick on the 30k
droplet dish with two species; the machine was loaded, so roughly), so near a
contact the side with more mould within r wins.

- **On the two halves of the 1M network** (conversion 1.2, no avoidance),
  0.6 against 0.5, 8 seeds, deciding measure named before the run: species
  0's gain in walker share from the start to 16 s. **-0.081 to +0.082 against
  -0.044 to +0.062: higher on 3 of 8, p 0.73.** A denser half is not an edge
  either: the border does not move for it. Script:
  [`creatures-2026-11-11/numbers.sh`](creatures-2026-11-11/numbers.sh).
- **Hunting droplets.** Grazing droplets whose species are drawn to each
  other (`species_avoid` -0.5: each reads own x 0.5 + total x 0.5), with
  conversion 1.2. Even, one side pulls ahead by chance (seed 1: 0.60 at 30 s
  with r 0, 0.45 with r 8). Started mixed in one blob, 0.55 against 0.5, 8
  seeds, same measure at 30 s: **gain +0.20 to +0.33 against -0.10 to +0.05
  with r 0, and +0.19 to +0.29 against -0.05 to +0.07 with r 8: higher on 8
  of 8 both, p 0.008** (survives Bonferroni over these three tests). The
  bigger side ends with 0.75-0.88 of the walkers. Nearly all of it happens
  in the first 10 s (0.77-0.85 by then), while the mixed blob sorts itself
  into droplets: conversion is a local majority vote, and a vote amplifies a
  55/45 mixture fast.
- **Hunting droplets started apart** (each species in its own half, r 8, the
  same 0.55 against 0.5): gain -0.009 to +0.017 against -0.027 to +0.029,
  higher on 5 of 7 with one tie, p 0.45. Once sorted into one-colour
  bodies, the sides hold; droplets meet and part without taking each other.
  Making the droplets blind to species (`species_avoid` -1, so bodies of
  both colours merge freely) did not help: gain -0.020 to +0.047 against
  -0.015 to +0.024, higher on 5 of 8, p 0.73
  (`raw/hunt-blind.txt`).

So numbers win where the species are mixed and nowhere else. Script:
[`creatures-2026-11-11/hunt.sh`](creatures-2026-11-11/hunt.sh) (`ZONES=1`
for the halves). Raw: `raw/numbers.txt`, `raw/hunt.txt`,
`raw/hunt-zones.txt`.

## What this means for play

A grazing droplet is a small creature with a body, a wake of bare ground
behind it and a reason to move, all from the simulation. Several hundred of
them roam a dish; two species roam among each other without merging. That
gives the RTS direction countable units made of mould. Grass is also a map
resource that the creatures visibly use up, and its regrowth rate sets how
far they roam. A stronger trail is not that edge: it overreaches. Numbers are an edge only
where the two species are mixed: there the majority takes almost all of it
within 10 s, and once bodies are sorted they hold. That suggests a verb
rather than a stat: a player who can *mix* (spill spores into an enemy body,
or break a body so the two colours mingle) starts a vote the bigger side
wins. Untested: a rule that keeps bodies mixing at contact (a push, or
walkers that cross into the other body), so the vote happens at every
meeting.

## Commands

From the repo root after `cargo build --release --example bigdish`:

```
bash Reports/creatures-2026-11-11/graze.sh /tmp/graze      # the paired test; drift.py beside it
# two species of grazing droplets
./target/release/examples/bigdish preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 \
  rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=120 blocked_deposit=1 \
  graze_weight=10 species=2 species_avoid=0 seconds=20 style=species frames=out/ every=12
# hunting droplets, 55% amber, mixed start
./target/release/examples/bigdish preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 \
  rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=120 blocked_deposit=1 \
  graze_weight=10 species=2 species_avoid=-0.5 species_convert=1.2 species_convert_r=8 species_share=0.55 \
  seconds=12 style=species frames=out/ every=12
bash Reports/creatures-2026-11-11/numbers.sh; bash Reports/creatures-2026-11-11/hunt.sh; ZONES=1 bash Reports/creatures-2026-11-11/hunt.sh
```

Raw output: [`creatures-2026-11-11/raw/`](creatures-2026-11-11/raw/).
