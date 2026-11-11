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

## What this means for play

A grazing droplet is a small creature with a body, a wake of bare ground
behind it and a reason to move, all from the simulation. Several hundred of
them roam a dish; two species roam among each other without merging. That
gives the RTS direction countable units made of mould. Grass is also a map
resource that the creatures visibly use up, and its regrowth rate sets how
far they roam. The next question is what makes one body beat another: an
uneven start, food that grows a body, or a rule where the larger body takes
the smaller on contact.

## Commands

From the repo root after `cargo build --release --example bigdish`:

```
bash Reports/creatures-2026-11-11/graze.sh /tmp/graze      # the paired test; drift.py beside it
# two species of grazing droplets
./target/release/examples/bigdish preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 \
  rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=120 blocked_deposit=1 \
  graze_weight=10 species=2 species_avoid=0 seconds=20 style=species frames=out/ every=12
```

Raw output: [`creatures-2026-11-11/raw/`](creatures-2026-11-11/raw/).
