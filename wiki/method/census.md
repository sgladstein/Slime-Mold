---
title: The census (how a change is judged)
kind: method
status: current
updated: 2026-11-10
sources: [Reports/gpu-plan-2026-11-10.md, Reports/mould-probe-2026-11-07.md, Reports/mould-review-2-2026-11-10.md, Reports/mould-wild-one-2026-11-10.md, Reports/mould-keep-alive-2026-11-10.md, Reports/mould-scratch-2026-11-10/README.md, Reports/pixel-physics-review-2026-11-10.md]
code: [mouldprobe, state_hash, every_dish_replays_bit_for_bit, Dish::step_cost, HJIT, HLAG, STOPAT, NOWATER, INVAR, WARDEN]
---

# The census (how a change is judged)

## Now

Every dish and every change is judged by **scripted hands over paired
seeds**, with `examples/mouldprobe.rs` (`cargo build --release --example
mouldprobe`, then run it with `level=N players=... seeds=...`). The same seed
is played by each hand, so differences are counted per seed, and quoted as
paired counts with a two-sided sign test. **Doing nothing and a random hand
are the controls**: a good dish is one a plan wins and random loses. Say how
many comparisons were made and which results survive a Bonferroni correction
(p < 0.05 / k). The owner's play overrules a table.

Everything the probe draws goes through the window's own frame function, so
its GIFs and stills are what a player sees. Pictures go on an Artifact page
with the counts beside each, and the page's source is saved under
`Reports/` ([pages.md](../../Reports/pages.md)).

## The hands

| Hand | What it does | Where |
|---|---|---|
| `idle` | does nothing | `mould::play` |
| `random` | random tools at random places | `mould::play` |
| `plan`, `fresh`, `intended`, `route` | scripted plans per dish | `mould::play` |
| `sleeper` | Keep It Alive: bakes mould asleep when food runs out, waters it near the next pile | `mouldprobe` |
| `warden` | The Wild One: lamp, salt, snip, oats; `WARDEN=lamp` etc. for one tool | `mouldprobe` |
| `camp`, `nestsnip`, `nestsalt`, `jail`, `strike` | one-move openings on the wild nest | `mouldprobe` |

Review 2's knobs make any hand clumsier on purpose: `HJIT=n` (aim error, a
disc of n cells), `HLAG=s` (looks only every s seconds), `STOPAT=s` (lets go),
`NOWATER=1`, and `INVAR=1` (checks the two-mould step's invariants every 15
steps). A hand that only wins at exact aim is not yet a game a person plays.

## How many seeds

The smallest two-sided sign-test p a clean sweep can reach is 2 / 2^n
(arithmetic, not a measurement):

| Seeds | Best p (n-0) | Survives 5 comparisons? |
|---|---|---|
| 5 | 0.0625 | no |
| 8 | 0.0078 | no (bar 0.01) |
| 16 | 0.00003 | yes |

So 5 seeds give a direction only (the first probe said so), 8 seeds give a
direction that a correction usually does not pass (review 2's strict bar was
0.004 for about 14 comparisons), and star bars want 16 or more.

## Frame cost

Quote the frame cost of anything that runs every step: mean and worst frame
at scale 4, before and after, same session. The plan hand's run prints it, and since
2026-11-10 it also prints the simulation's time pass by pass
(`Dish::step_cost`): diffusion 62%, moving the walkers 24%, the census 4.5%
([gpu plan, *Phase 1 progress*](../../Reports/gpu-plan-2026-11-10.md)). The Wild One's second dish costs
about +1.4 ms a frame; the first probe ran at a 2.3-2.8 ms mean.

## Same game, bit for bit

A change meant to be faster, not different, must leave every game as it
was. `Dish::state_hash` fingerprints the whole dish (fields, walkers, piles,
store, random stream, tallies, the wild mould), and `mouldprobe` prints
`hash=` per run: diff two builds' hash lines, and a timing beside identical
hashes is a real speed-up. The test `every_dish_replays_bit_for_bit` replays
all seven dishes under plan and random and goes red on a clock-seeded draw
([pixel-physics-review](../../Reports/pixel-physics-review-2026-11-10.md),
*Step 1*). The frame splits sim 1.10 + draw 3.83 ms on the free dish, plan
hand, scale 4 (same report, *The frame, split*).

## Owner's word

- 2026-10-27 02:51Z (Pixel Physics): "Do you take into account normal
  variability in these comparisons? Is 111 significantly different than 180
  in a statistical sense?" Source of the paired-counts rule
  ([project-memory note](../../handoff/project-memory/report-significance.md)).
- 2026-10-25 06:03Z: "I cannot see the images that you post". Source of the
  pictures-on-a-page rule ([note](../../handoff/project-memory/images-via-artifact.md)).

## Runners

- [`Reports/mould-scratch-2026-11-10/tune/`](../../Reports/mould-scratch-2026-11-10/tune/run.sh):
  `run.sh` runs a hand x seed grid four wide; `w5.py` prints shares and every
  pairwise sign test. Their paths still point at the old Pixel_Physics
  scratch folder; edit `S=` and the probe path before use.
- Review 2's runs, commands and probe patch:
  [`Reports/mould-review-2-2026-11-10/`](../../Reports/mould-review-2-2026-11-10/runs.txt).

## Open

- Nothing about the method itself. The repo has no saved census baseline
  that CI checks; every report re-runs its own.

## Sources

- [mould-probe-2026-11-07.md](../../Reports/mould-probe-2026-11-07.md), *Measurements*.
- [mould-review-2-2026-11-10.md](../../Reports/mould-review-2-2026-11-10.md), *Method*.
- [mould-wild-one-2026-11-10.md](../../Reports/mould-wild-one-2026-11-10.md) and [mould-keep-alive-2026-11-10.md](../../Reports/mould-keep-alive-2026-11-10.md): the hands' definitions.
- [mould-scratch-2026-11-10/README.md](../../Reports/mould-scratch-2026-11-10/README.md): the runners.
- [pixel-physics-review-2026-11-10.md](../../Reports/pixel-physics-review-2026-11-10.md), *Step 1*: the hash and the replay test.
