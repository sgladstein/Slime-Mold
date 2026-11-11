# Outside rules on the big dish, judged as creatures (2026-11-11)

Research with small tests, no game change. The owner, 03:00Z, asked to test
Lague's area sensors, multiple species and MCPM's chance-weighted turn, add
them to the sandbox, and search wider for open-source code ("yes"). At 04:11Z
the owner narrowed the aim: "we don't necessarily need to ReDiscover
everything ourselves ... it generates some sort of creature for gameplay and
not just random organic patterns". So the rules below are judged first on
whether they make a creature (a body with its own identity that grows, holds
ground or moves), and only the creature rules got further work. Follows
[`jenson-looks-2026-11-11.md`](jenson-looks-2026-11-11.md). Page: https://claude.ai/artifact/VLnRN8wTX9RxniD4c8drBw
(source, stills and videos in
[`outside-rules-2026-11-11/`](outside-rules-2026-11-11/)).

The wider search read 14 more sources; its survey, with licences and the
rules ranked, is
[`outside-rules-2026-11-11/code-survey.md`](outside-rules-2026-11-11/code-survey.md).
Source cards: [`primordia`](../research/sources/primordia.md) and
[`reginato-2024`](../research/sources/reginato-2024.md);
[`jones-adamatzky-2012`](../research/sources/jones-adamatzky-2012.md) and
[`lague-slime-simulation`](../research/sources/lague-slime-simulation.md)
already existed.

## Summary

Every rule is a `BigParams` field, off by default (the default dish's hash
is unchanged), with a doc comment exact enough to port to WGSL. Base for the
paired tests: 1,000,000 walkers, no crowd limit, no food, Jenson's angles
(45/22.5), sensor 18 cells, deposit 0.5, 8 seeds, sign tests, Bonferroni
over the 5 tests named in [`run.sh`](outside-rules-2026-11-11/run.sh).

**Creature rules**

- **Rival species (`species`, `species_avoid`, `species_zones`).** Each
  species lays and smells its own trail field; it reads own x (1 + a) minus
  everything x a. Started in their own halves, two species hold two
  territories with a clean border. The named measure failed: species 0's
  share of trail in its own half at 20 s was 0.89-1.00 with avoidance and
  0.88-1.00 without (more on 5 of 8, p 0.73), because following your own
  trail already keeps you home. Avoidance makes the border *cleaner*: tube
  cells held 80%+ by one species 0.994-1.000 against 0.966-0.983 (higher on
  8 of 8, p 0.008), and the share of tube cells changing hands in 2 s
  0.000-0.005 against 0.007-0.028 (lower 8 of 8). Started mixed, two
  species weave an interleaved labyrinth; three or four braid into worms.
  Both arms carry two species fields, so the cost of species over one
  field was not timed (an extra spread pass a species each step).
- **Cyclic species (`species_cycle`, Primordia's preset: own +1, next
  +0.6, previous -1).** Three species braid into ribbons that keep flowing
  round the dish. The named measure went the wrong way: the share of tube
  cells whose leading species changed in 2 s was 0.65-0.70 against
  0.75-0.85 for plain three-way avoidance (lower on 8 of 8, p 0.008), since
  plain avoidance makes thin single-species tubes whose lead flickers at
  every crossing. What cycling does is mix: tubes are shared (one species
  80%+ on 0.46-0.52 of tube cells against 0.83-0.89) and three times as
  looped (97-125 loops against 22-63). Cost about +14%.
- **Explorers (`explore_steps`, `explore_deposit`, new `explore_spread`;
  Reginato 2024).** A walker younger than `explore_steps` steers to the
  *lowest* reading and lays 5% of the deposit, then becomes an ordinary
  walker. All switching at once (spread 0) gives an expanding disc with a
  fan-textured front that turns into a mesh everywhere at once. With
  `explore_spread` 1 (each walker explores for 0-2x the steps), walkers
  turn one by one: **the disc grows radial branching veins with a feathery
  front**, the first time any rule here has drawn something like Jenson's
  fronts, then closes into a mesh by 4 s. Seed 1 only; not paired.
- **Packed walkers that smell of themselves (`blocked_deposit`, new).**
  Jones and Adamatzky's amoeboid rules (one walker a cell, SA 90, RA 45,
  SO 9, blocked walkers keep their heading) did not make a crawling blob:
  30,000-100,000 walkers spread over the dish in 2 s and the trail broke
  into specks (walkers off tubes 0.92-0.94). Our walkers only lay trail
  when they move, so a jammed core smells of nothing and the moving edge
  draws the crowd out. Letting a blocked walker lay its deposit where it
  stands turns 30,000 walkers into **about 180 round droplets** of about
  200 cells each that slowly merge (185 at 7 s, 177 at 9 s) but do not
  move (centres shift a median 0.6-0.7 cells a second). With a longer
  reach (SO 27) the same walkers make **beads on strings**: a network
  with a blob at each junction that coarsens over time. With food piles the
  piles grow big blobs, but the droplets do not crawl to them. Making the
  droplets move is open. Seed 1 only.
- **Blocked walkers keep their heading (`blocked_keep`, `blocked_reset`)
  and retry rounds (`blocked_retry`, new).** Retrying a blocked step into
  cells emptied that step lets a packed crowd move together, as Jones's
  random sequential update does. Neither made a blob move on its own; kept
  as options.

**Texture knobs (sandbox only, no more testing)**

- **Chance-weighted turn (`chance_exp`, MCPM).** Turn toward a side with
  probability g^e / (f^e + g^e). At e 4, loops at 10 s 60-73 (median 66.5)
  against 38-52 (median 44): more on 8 of 8, p 0.008, survives Bonferroni.
  Finer, more regular cells; lower contrast (5.3-5.9 against 7.7-9.6).
  Cost about +15%.
- **Area sensors (`sensor_r`, Lague).** Each sensor sums a (2r+1)^2
  square. At r 1, loops at 10 s 32-46 (median 38) against 38-52 (median
  43.5): fewer on 8 of 8, p 0.008, survives Bonferroni. A coarser mesh for
  **+50% cost** (290 against 190 ms a tick); a wider `sensor_offset` is the
  cheaper way to coarsen.
- **Size spread (`size_spread`, Primordia).** Walker k's sensor reach and
  step are scaled by 2^(spread x u), u fixed per walker. Thick trunks with
  fine side capillaries on seed 1. The named measure failed: tube contrast at 10 s 107-151 (median 123.6) against 124-157 (134.5), lower on 6 of 8, p 0.29, so the trunks are not brighter against the rest on the measure; the look is one seed's picture. Cost about +5%.

## The tests

| Test | Deciding measure (named before) | With the rule | Without | Paired | p |
|---|---|---|---|---|---|
| chance turn, e 4 | loops at 10 s | 66.5 (60-73) | 44 (38-52) | more 8/8 | 0.008 |
| area sensors, r 1 | loops at 10 s | 38 (32-46) | 43.5 (38-52) | fewer 8/8 | 0.008 |
| territory, 2 species in halves, avoid 1 v 0 | species 0's trail in its half, 20 s | 0.99 (0.89-1.00) | 0.98 (0.88-1.00) | more 5/8 | 0.73 |
| cyclic v plain 3 species | tube cells changing lead in 2 s, 10 s | 0.68 (0.65-0.70) | 0.80 (0.75-0.85) | lower 8/8 (wrong way) | 0.008 |
| size spread 0.6 | contrast (p99 / median tube trail), 10 s | 123.6 (107-151) | 134.5 (124-157) | lower 6/8 | 0.29 |

Bonferroni over 5 tests needs p < 0.01, which 8 of 8 (p 0.0078) meets.
Costs are from runner machines running 4 jobs at once, so they are
relative only. Raw lines: `outside-rules-2026-11-11/raw/`.

## What this means for play

The species rules are the clearest creatures: each species is a body with
its own colour, its own ground and a border with the other, which is what
an RTS faction needs. Cyclic species give three bodies that never settle.
Explorer spread gives a body that visibly grows outward with a feathery
front before it settles, which is a creature *reaching*. Droplets and beads
give the first countable bodies (a few hundred small cells, or a network of
nodes) rather than one mesh. None of these yet moves across the dish as a
body; that, and species that fight over a border rather than just keeping
apart, are the next questions.

## Commands

From the repo root after `cargo build --release --example bigdish`:

```
bash Reports/outside-rules-2026-11-11/run.sh chance|area|territory|cycle|size [JOBS]
# droplets
./target/release/examples/bigdish preset=dense piles=1 food_deposit=0 crowd=1 walkers=30000 deposit=5 \
  rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=120 blocked_deposit=1 \
  seconds=10 style=count look=out/ at=10
# beads on strings: the same with sensor_offset=27
# explorer fronts
./target/release/examples/bigdish preset=dense piles=1 food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 \
  rotate_angle=22.5 sensor_offset=18 blob=1 blob_r=20 explore_steps=900 explore_spread=1 \
  seconds=6 style=raw look=out/ at=2,4,6
```
