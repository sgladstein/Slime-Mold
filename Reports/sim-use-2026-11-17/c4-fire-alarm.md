# C4 Fire scares it, a little (2026-11-17)

The simulation-use plan's C4 ([plan.md](plan.md), draft 3, Part C: "Fire
lays alarm near its front ... measure fire kills and the body's centre at
+30 s"). Page: https://claude.ai/artifact/Nq2iGDUWQN4SuF5VruRQF6 (source `page-c4/`).

**Status: built on both engines as `BigParams::fire_alarm`, off.** At the
small value the plan meant ("a little", 5) it changes nothing; at 50 and
more the slime draws back from the flames and the fire, no longer doused
by its trail, eats holes in the body ringed by slime. That is the same
result the gun thread found for heat in the sensors (`fire_weight`, built
11-17 as a flag, off, [element](../gun-element-2026-11-17/README.md)), and
the game designer's call on that flag (10:14Z: keep it off until a
crossing test, food beyond a lit line) covers this one too. So both stay
off together, and the designer and gun thread are told this is a second
route to the same behaviour.

## What was built

- `BigParams::fire_alarm` (key `fire_alarm=`, 0 off). Each fire step every
  burning agar cell adds `fire_alarm` x its fire to the alarm field, where
  the slime has one (shape C's flinch: keep 0.9, weight -1). The alarm
  spreads by the field's 3x3 mean, about three fine cells, so the flinch
  reads it as ground to back off. Through the alarm it also wakes sleepers
  and, on a scarring tier, scars; `fire_weight` reads the fire itself and
  does neither.
- CPU: in `fire_tick`, after the fire's swap. GPU: in the fire pass, after
  the new fire is written, into the alarm `germ_out` holds (the plant
  passes run on the tick's side). The value rides in the plants' header,
  **word 44**: the first try used word 49, which `Engine::ignite`
  overwrites with its count of lights, so the GPU laid nothing while the
  CPU did (the twin caught it; a constant in the shader showed the write
  itself was right).

## Tests

| Test | Result |
|---|---|
| `fire_lays_alarm_on_the_gpu_as_on_the_cpu` (llvmpipe; the C1 twin's flats map with plants, flinch 0.9 on both, lit at 12 s, compared each second for 30 frames) | largest alarm L1 0; alarm summed 1 s after the light 2,702.8 against 0 without; planted, the GPU laying none: L1 3.67e3 (fails) |

## Measure

`examples/fearprobe.rs`, raw lines in `c4-raw/`. Planet depth 2 under full
meadow, the default slime (shape C, flinch 0.9, `fire_dry` on) on the CPU
race world (scale 2), grown 60 s with the surges held; a disc of 10 dish
cells on the body, 25 clear of crystals (as `dryprobe`'s), lit whole and
watched 30 s, once per `fire_alarm`. **Deciding measure, named before the
run:** fire kills per walker in the disc at the light, 5 against 0, paired
by seed.

| Seed | Disc | Killed at 0 | at 5 | at 50 | at 200 | Walkers within 20 at the end (0 / 50 / 200) |
|---|---|---|---|---|---|---|
| 1 | 1,212 | 920 | 915 | 1,146 | 1,324 | 818 / 574 / 452 |
| 2 | 991 | 305 | 305 | 308 | 308 | 1,151 / 1,074 / 990 |
| 3 | 1,170 | 340 | 340 | 339 | 339 | 1,291 / 1,297 / 1,266 |
| 4 | 1,208 | 889 | 900 | 1,044 | 1,176 | 543 / 357 / 250 |
| 5 | 1,180 | 380 | 377 | 377 | 522 | 354 / 354 / 298 |
| 6 | 1,165 | 454 | 451 | 590 | 558 | 1,286 / 1,203 / 1,097 |
| 7 | 11 | 0 | 0 | 0 | 0 | 34 / 44 / 37 |
| 8 | 530 | 301 | 287 | 262 | 300 | 64 / 5 / 12 |
| sum | | 3,589 | 3,575 | 4,066 | 4,527 | |

(Seed 7's disc found almost no slime, so it is a tie throughout.)

- **At 5** (deciding): no change. More kills on 1 seed, fewer on 4, ties on
  3 (sign p 0.375). The alarm a fire lays at 5 is a tenth of one rifle
  hit's 50, and spreads only three cells.
- **At 50**: more kills on 4, fewer on 3 (not significant), 13% more in
  all; **at 200**: more on 5, fewer on 2, 26% more. On the seeds where the
  fire takes hold (1, 4, 6) the slime near it thins (818 to 574 to 452
  walkers within 20 cells on seed 1).
- **The body's centre**: the mean of the walkers within 20 cells moved
  from the disc by about the same at every value (3.9 to 12.6 cells on
  seed 1 at 0, to 9.8 at 50); the lit disc dies at the light either way,
  so this measure mostly reads the hole, not a retreat.
- **What it looks like** (the page, seed 1): at 0 the fire dies in a few
  seconds in the damp body; at 50 the slime steps back from each flame,
  the ground it leaves dries, and the fire burns three holes ringed by
  slime; at 200 the holes join into one ring of fire 20 cells across.

Nothing here keeps the slime away from fire or herds it; it opens the
body to fire. Whether that is wanted is the same question the designer
left open on `fire_weight`, so it is off until that test.

## Frame cost

Off: none (`fire_alarm` 0 skips the loop on the CPU; the shader tests
`pf(44u) > 0` after its own write). On: one add per burning cell a fire
step. Not timed on the 1070.

## The crossing test (gun thread, 11-17 12:05Z)

The gun thread filmed the designer's test with `fire_alarm` as a third arm
([Tinder](../gun-tinder-2026-11-17/README.md), page
https://claude.ai/artifact/DMHMAsve7qyjLV8JXsK54D): planet 1:3, one seed,
a lit Tinder line across a stream of walkers going to food, crossings from
16 to 45 s. No line: 214 crossed. Weight 0: 10 crossed, 119 burned.
`fire_weight` -5: 57 crossed (4 while it burned), 44 burned. `fire_alarm`
50: 31 crossed (0 while it burned), 28 burned. Neither is a fence: both
hold the stream at the edge until the fire dies, then it crosses the char.
`fire_alarm` held harder and reuses the field the flinch already reads, so
the two threads agreed it is the one kept (off) and `fire_weight` goes, one
switch for one mechanism (a reviewer's ask). The technical lead (12:05Z)
pairs the two in ROADMAP's switches-off table for the owner's blind A/B
before the delete; if they look alike, the alarm route wins.

## Left

- **A fire-shy tier** (the game designer, 12:10Z): with `fire_alarm` the
  slime waits out a burning line without dying, so fire becomes a fence
  rather than a weapon. Keep it off globally and give it to one tier where
  Tinder lines are the counter (the Sprawler, whose counter is already
  "walls; chasing loses"), as a per-tier key, when the run's tier pass next
  touches the Sprawler; record it in the card table's Tinder row then.

- 1070 timing with the rest of the plan's passes.
