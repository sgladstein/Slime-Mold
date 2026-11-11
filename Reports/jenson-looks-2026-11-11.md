# Jenson's three missing looks on the big dish (2026-11-11)

Research with small tests, no game change. The owner, 00:06Z: "any thoughts
on the three things still not reproduced? Feel free to start sub-agents or
additional threads to explore this comparison further". Follows
[`jenson-compare-2026-11-10.md`](jenson-compare-2026-11-10.md), which
reproduced Jenson's thin veins, haze and polygons but not (1) feathery
fronts that branch into a haze and close into loops, (2) a thick first ring
that lasts as a trunk inside a fine mesh, or (3) contagion. Page:
https://claude.ai/artifact/DqJxgjQGVgnwQNwbbzoGrg (source and pictures in
[`jenson-looks-2026-11-11/`](jenson-looks-2026-11-11/)).

## Summary

- **Trunks: reproduced in part, by the start alone.** Starting 70% of 1,000,000 walkers in a dense disc
  (radius 45 dish cells, `BigParams::blob_r`) and 30% spread over the dish
  (`haze`) leaves the disc's edge as a thick bright ring inside a mesh. At
  20 s, the brightest 1% of trail lies in the ring's band (35-55 dish cells
  from the centre) for 0.70 of it (median, 0.20-0.86) against 0.15
  (0.09-0.31) from an even start: more on 8 of 8 seeds, p 0.008. By eye it
  is still a clear ring on 6 of 8. Slow respawn (0.001 a step) keeps the
  outer mesh getting finer while the ring lasts; 0.003 dissolves the ring
  into the mesh by 20 s and 0.01 by 5 s, so **respawn sets how long a trunk
  remembers**. Unlike Jenson's, our mesh inside the ring stays coarse.
- **Contagion: built, and it travels along the tubes.** Sick walkers lay a
  germ field; a healthy walker standing in enough of it falls sick, then is
  immune (`sick_steps`, `immune_steps`, `sick_catch`, and the rest; the same
  rule as Two Fevers, `src/mould/fever.rs`, now on the big dish with no
  crowd limit). Seeded in the middle it runs out along the network as a red
  ring, about 25 dish cells a second (catch 5) or 50 (catch 0.5), and covers
  the dish in 3-4 s. If sick walkers sense half as far and turn twice as
  hard (`sick_reach` 0.5, `sick_turn` 2, our old numbers, which make ropes
  without the limit), **the network coarsens behind the wave** into fewer,
  thicker loops, the nearest we got to the explosion clip's bright ropes:
  at 9 s 41-51 loops against 48-58 with no sickness, fewer on 8 of 8, p
  0.008. **It does not last:** by 20 s the healthy dish has coarsened as
  far on its own (fewer on 5, more on 2, one tie, p 0.45), repeating Two
  Fevers' finding that a wave passes and leaves the host as it was.
- **A knife cut does not stop the wave.** Clearing trail and germ in a band
  2 dish cells wide across the dish, 32 cells from the seed, as the wave
  starts: the far side was as sick 2 s later with the cut as without (more
  on 4 of 8 seeds, fewer on 4, p 1). Walkers carry the sickness across a
  cut, since a cut removes trail, not walkers.
- **With short immunity it never ends.** Sick 0.5 s and immune 1 s: after
  the first wave the dish cycles, 30-36% sick at any time, every walker
  catching it again the moment its immunity ends. No travelling waves or
  spirals appeared.
- **Feathery fronts: not reproduced.** Seven mechanisms, none of which made
  a front branch into a haze (below). The guess in the last report, a haze
  too thin to organise, fails first: **with Jenson's rule any haze
  organises**, down to 2% of the walkers (0.04 a cell), because the turn
  only compares sensors. A sensing floor (`sense_floor`) does keep a haze,
  but then the network swallows the haze near it rather than growing into
  it.
- **The 36 Points settings give four distinct looks** (`sense_scale` and
  `*_mod`, each setting = base + amp x s^exp, s the trail where the walker
  stands): narrower sensors in strong trail make long sweeping curves;
  turning less in strong trail makes the first ring **expand and sweep across
  the whole dish** as a travelling front; moving faster in strong trail keeps
  the ring round with finer mesh inside. Bleuje's capped square-root deposit
  (`deposit_cap`) **erases the dense start at once** into an even mesh of
  straight-sided polygons with a haze at the rim: the start no longer
  matters, so no trunk.

## The rules added (all `BigParams` fields, off by default)

With every new field at its default the dish hashes as before (checked on
`preset=dense`, 2 s, seed 1: `869141a7bf398adf` before and after). Each
field's doc comment is written to port to WGSL as it stands.

| Field | What it does | Default |
| --- | --- | --- |
| `blob_r` | Starting blob radius in dish cells, centred on the dish, walkers drawn evenly over it | 0 (today's packed inoculum on home) |
| `haze` | Share of walkers that start spread over the whole dish instead of in the blob | 0 |
| `sense_scale`, `offset_mod`, `angle_mod`, `turn_mod`, `step_mod` | 36 Points: s = min(trail here x `sense_scale`, 1); each setting = base + amp x s^exp | 0 (off); mods `[0, 1]` |
| `deposit_cap` | Each cell with n walkers gets `deposit` x sqrt(min(n, cap)), shared among them | 0 (one `deposit` a moving walker) |
| `sense_floor` | If all three sensors read under it, turn `rotate_angle` left or right at random | 0 |
| `wander` | Chance a look of taking a random heading | 0 |
| `sick_steps`, `immune_steps`, `sick_catch`, `sick_deposit`, `sick_keep`, `sick_speed`, `sick_trail`, `sick_reach`, `sick_turn`, `sick_seed_r` | Contagion (above); `BigDish::infect(x, y, r)` seeds it | `sick_steps` 0 (off) |

`examples/bigdish.rs` takes every new field, `style=soft` (the comparison's
soft curve) and `style=sick` (germ in red, full at four times the catch
level), and the events `infect=T` and `cut=T` (`cut_x`, `cut_w`); with
contagion on it prints the share sick, the share ever sick, the front (90th
percentile distance of sick walkers from the centre) and the share beyond
the cut ever sick.

## The tests

Base for every run (the comparison's Jenson settings): `preset=dense piles=1
food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5
sensor_offset=18`, one thread a run, four at once
([`j.sh`](jenson-looks-2026-11-11/j.sh)). Stills drawn from raw trail dumps
on a log curve over the 99.9th percentile ([`tone.py`](jenson-looks-2026-11-11/tone.py)).
Raw output of every run: [`raw/`](jenson-looks-2026-11-11/raw/). Deciding
measures were named before each paired run; Bonferroni over the three
paired tests (p < 0.0167).

| Test | Arms | Seeds | Deciding measure | Result |
| --- | --- | --- | --- | --- |
| Trunk ([`trunk.sh`](jenson-looks-2026-11-11/trunk.sh) seeds 1-3, [`trunk2.sh`](jenson-looks-2026-11-11/trunk2.sh) the rest, [`trunk.py`](jenson-looks-2026-11-11/trunk.py)) | disc (`blob_r` 45, `haze` 0.3) vs even start; both `respawn` 0.001, `sense_scale` 0.02, `step_mod` 1:1 | 8 | share of the brightest 1% of trail within 35-55 dish cells of the centre, 20 s | 0.70 (0.20-0.86) vs 0.15 (0.09-0.31); more on 8 of 8, p 0.008, survives |
| same | same | 8 | loops at 20 s (secondary) | 49.5 (40-52) vs 52.5 (48-58); fewer on 7 of 8, p 0.07 |
| Cut ([`cut.sh`](jenson-looks-2026-11-11/cut.sh)) | knife at 4 s vs none; sickness seeded at 4 s, r 10 | 8 | share of walkers beyond the cut ever sick, 6 s | 0.155-0.254 vs 0.157-0.260; more on 4, fewer on 4, p 1 |
| Rewiring ([`rewire.sh`](jenson-looks-2026-11-11/rewire.sh)) | sick 2 s (`sick_reach` 0.5, `sick_turn` 2), seeded at 4 s, vs none | 8 | loops at 20 s | 15-25 vs 17-25; fewer on 5, more on 2, tie 1, p 0.45 |
| same, while the wave passes | same | 8 | loops at 9 s (named with the deciding one) | 41-51 vs 48-58; fewer on 8 of 8, p 0.008, survives |

Single-seed looks (seed 1, pictures on the page):

| Look | Settings over the base | What happens |
| --- | --- | --- |
| Dense disc + haze | `blob=1 blob_r=45 haze` 0, 0.1, 0.3, 0.6 | Thick ring with a few chords inside; the haze organises into its own mesh at once, a dark moat between |
| + respawn | 0, 0.001, 0.003, 0.01 to 30 s | Ring lasts past 30 s at 0 and 0.001 (outer mesh: 14 loops at 0, 43 at 0.001); merges by 20 s at 0.003; gone by 5 s at 0.01 (210 loops at 30 s) |
| Thin haze | `haze` 0.02, 0.05 | Organises into a mesh within 2 s even at 2% |
| 36 Points | `sense_scale` 0.02 with `angle_mod -30:1`, `turn_mod -20:1`, `step_mod 1:1`; `deposit_cap` 100 | Long curves; expanding ring that sweeps the dish; round lasting ring with finer mesh; instant even polygons (71 loops at 20 s) |
| Contagion, catch 0.5 / 5 | `sick_steps` 120 or 60, `immune_steps` 480 or 120 | Wave crosses in about 2 s / 3.5 s; with immune 1 s, endemic cycling at 30-36% sick |
| Contagion + ropes | `sick_steps` 1200 `sick_reach` 0.5 `sick_turn` 2 `sick_trail` 3 | Behind the front the mesh thickens into fewer loops |

**Fronts, what was tried** (all with the dense disc, seed 1, 8-10 s):

1. Thin haze, 2-60% of walkers: organises on its own at every density.
2. `sense_floor` 2-20 with 30% haze: at 2 the haze organises, at 3 worms
   nucleate everywhere at once, from 3.5 up the haze stays a haze and the
   ring swallows the haze near it, leaving a dark moat.
3. Floor 3.5-4 plus sticky capture (`step_mod -0.9:2`, walkers slow in
   strong trail) and short sight in thin trail (`sensor_offset` 4,
   `offset_mod` 14:1): the disc reorganises inside; nothing grows out.
4. Wider trail spread (`trail_passes` 4, `keep` 0.95) with floors 6-8: the
   disc becomes fat worms; at floor 6, worms nucleate from the rim and grow
   inward, the only front seen, but smooth, not feathery.
5. Short reach (6) and wide spread: worms everywhere or a shrinking blob.
6. Leaking walkers (`wander` 0.01-0.03) with no haze: the leakers are pulled
   back; no glow builds.
7. Freezing in strong trail (`step_mod -1:1`) with random-walking haze, as
   diffusion-limited aggregation: a frozen disc with a rough edge; growth
   too slow to see in 8 s.

## Cost

One thread, 1,000,000 walkers, 1024 x 640, 2 s from an even start, ms a
tick: base 132.7; `sense_scale` with `step_mod` 176.5 (+33%, a `powf` a
walker a mod); `deposit_cap` 150.9 (+14%); `sense_floor` 139.6 (+5%);
`wander` 142.5 (+7%); contagion with everyone sick 142.9 (+8%, a second
field spread every step). Seed 1, one run each, idle machine.

## What it means

- **For play, contagion is the strongest of the three.** It is visible, it
  follows the network, and it can change how the mould moves. As built it
  is a ring that passes; to matter it needs a lasting effect, the same
  condition as Two Fevers' dead end (`Reports/dead-ends.md`). Two leads:
  the sick-ropes rewiring lasts while the wave is there, so a longer
  sickness, or one that changes the trail (`sick_trail`), may hold; and
  since a knife cannot stop it, a wall or a gap in the walkers would be the
  counter-tool.
- **Trunks come from where the mould starts**, kept by slow respawn. A
  player spawning a dense patch would get one; the respawn rate is a dial
  for how long a structure remembers.
- **The expanding ring** (`turn_mod` negative) is a travelling front of its
  own: a visible wave that rewrites the mesh as it passes. Untested beyond
  one seed.
- **Feathery fronts remain open.** The next guesses: Jenson's network_2 may
  start from a few dense spots rather than a disc, or use a rule we have
  not read (a sensor of the walker count, or a deposit that depends on
  speed). The 2020 paper's "critical density" is not a density at which
  the haze stops organising in this rule.
