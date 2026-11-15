# Breakaway in the run from planet 2 (D2), 2026-11-15

The whole-game review's **W4, "Breakaway (condense + roam) is built and
off, and it gives the sitting slime reach"**, and its decision **D2**
(`Reports/whole-game-review-2026-11-15/README.md`). Recommended: "RP's
option, on from planet 2 in the next playtest pack with a clip." The owner,
2026-11-15 21:41Z: "5-7) agreed" (D2 among them; `wiki/decisions-game.md`).

Page for the owner (planet 2 with and without, time-lapse):
https://claude.ai/artifact/VTXFHoSuoBfRK9XZHBF3uc (source `page/`; clips by
`raw/make_clips.py` from the films of `raw/film.sh`).

## What changed

`run::enemy_keys` (the planet's own engine keys, over shape C and under the
tier's and the menu's) now also sets, from planet 2 to the core world:

- `condense=300`: a crowded or long-fed walker condenses after 300 steps
  (10 game seconds), absorbing a neighbour (`BigParams::condense`,
  `Reports/condense-2026-11-14/README.md`);
- `roam=300`: every 300 steps a condensed or thick walker may break away
  for the nearest fresh crystal at least 30 cells off (`BigParams::roam`).

Planet 1 keeps the plain body, so the first planet teaches the slime
before it spreads. The tiers set neither key, and the start menu passes
only the options the player moved, so the menu's Condense and Breakaway
still override a run planet. Test: `run::tests::the_slime_gets_fiercer_deeper`
(none on planet 1, both on 2 to 8).

## Why both keys, not breakaway alone

Since #338, thick walkers (shape C's `thicken`) can break away without
condensing. On planet 2 that did almost nothing. Run seeds 1 and 2,
planet 2 (the Sprawler), the squad hand, to 3:00, `mode=squads`, on
llvmpipe at main 6d072828 (`raw/`), with the keys on the command line:

| Seed | Arm | Walkers 1:00 / 2:00 / 3:00 | Soldiers 0:30 / 1:00 / 1:30 / 2:00 / 3:00 | Killed | Mass at 3:00 |
|---|---|---|---|---|---|
| 1 | off (`roam=0`) | 18,738 / 19,103 / 25,480 | 8 / 7 / 3 / 0 / 0 | 6,166 | 28,502 |
| 1 | `roam=300` alone | 18,738 / 19,093 / 25,576 | 8 / 7 / 3 / 0 / 0 | 6,166 | 28,566 |
| 1 | `condense=300` alone | 13,499 / 17,026 / 23,917 | 8 / 2 / 0 / 0 / 0 | 5,290 | 39,322 |
| 1 | both (now the run) | 14,384 / 18,379 / 24,928 | 8 / 1 / 0 / 0 / 0 | 4,929 | 40,003 |
| 2 | off | 17,185 / 14,358 / 16,245 | 8 / 8 / 8 / 8 / 1 | 8,922 | 17,624 |
| 2 | `roam=300` alone | 17,185 / 14,211 / 16,498 | 8 / 8 / 8 / 8 / 1 | 9,078 | 17,803 |
| 2 | `condense=300` alone | 11,820 / 10,925 / 12,293 | 8 / 8 / 8 / 6 / 0 | 7,344 | 19,960 |
| 2 | both | 12,502 / 12,055 / 13,863 | 8 / 7 / 0 / 0 / 0 | 6,359 | 21,646 |

Mass counts a condensed or thick walker twice (`World::mass`, the HUD's
goal). Breakaway alone matches off to the walker until 1:30 and stays
within 0.5% after. Most likely a thick walker sits inside the body with
every cell round it full, so it cannot leave; condensing absorbs half of
the ripe and leaves room. With both on, the planet-2 slime is a fifth to
two fifths heavier at 3:00 and the squad is gone by 1:30 on both seeds
(off: 2:00 on seed 1, one soldier left at 3:00 on seed 2). Breakaway's own
share on top of condensing is small on the Sprawler, whose mesh already
reaches most crystals, but on seed 2 it took the squad at 1:30 instead of
3:00.

`cmp-planet2-seed2.jpg`: rows off, condense alone, both; columns 1:00,
2:00, 3:00. With both on the far crystals hold raised bronze knots by
2:00, where off they hold a thin net.

## Planets 4 to 6 (smoke)

Same hand and build, `condense=300 roam=300` against `roam=0`, run seed 1
to 3:00, and planet 4 also on seeds 2 and 3
to 1:30. Soldiers at each 30 s check; mass at the last:

| Planet (tier) | Seed | Soldiers off | Soldiers on | Mass off | Mass on |
|---|---|---|---|---|---|
| 4 (Swarmer) | 1 | 8 / 0 / 0 / 0 / 0 / 0 | 8 / 8 / 8 / 8 / 8 / 8 | 55,177 | 62,206 |
| 4 (Swarmer) | 2 | 8 / 8 / 7 | 8 / 7 / 6 | 62,834 | 75,655 |
| 4 (Swarmer) | 3 | 8 / 8 / 8 | 8 / 8 / 6 | 55,702 | 76,121 |
| 5 (Sporer) | 1 | 8 / 8 / 4 / 2 / 0 / 0 | 8 / 8 / 6 / 2 / 0 / 0 | 120,090 | 123,398 |
| 6 (Sleeper) | 1 | 7 / 7 / 7 / 7 / 6 / 6 | 7 / 6 / 6 / 6 / 6 / 6 | 121,416 | 123,029 |

- Nothing crashes or sticks, and the slime is as heavy or heavier with it
  on every planet and seed.
- **The Swarmer, seed 1, is the one surprise.** Off, its flood to the
  squad's wound killed all eight by 1:00; with condensing on, the body's
  walkers are fewer (36,053 against 48,697 at 1:00, though heavier), the
  flood never reached the squad and all eight lived to 3:00
  (`cmp-planet4-seed1.jpg`, rows off and on at 0:30, 1:00, 2:00). On
  seeds 2 and 3 the squad did a little worse with it on (6 against 7 and
  8 at 1:30). One seed in three, so a tuning note for the playtest, not a
  reason to hold planet 4 back.
- The Sporer gains 3% and the Sleeper 1%: their own rules hold the
  body's size (the Sleeper's shrinks either way).

## Checks

- `cargo test --release --locked`: passes (516 + 39 + 39).
- `cargo clippy --all-targets --release --locked -- -D warnings`: clean.
- **The build plays the same game.** This branch's `larder-world run=2:2`
  with no keys matches main's with `condense=300 roam=300` to the walker at
  0:30 (14,900 walkers, mass 22,598), so the films are what planet 2 now
  plays; `run=1:1` matches main to the walker (10,028).
- Smoke: no crash or stuck game on planets 2, 4, 5 and 6 with both on; the
  squad still has soldiers at 1:00 on every run (planet 2 seed 1 only one,
  none at 1:30); the slime grows on every run, so an idle hand cannot win.

## Frame cost

**None measurable on the GTX 1070.** Condensing's GPU passes (`ripen`,
`massclear`, `masscount`, `dclear`, `dcount`;
`Reports/condense-2026-11-14/README.md`, *On the GPU*) already ran in every
run, since shape C's thickening uses them; turning `condense` on makes
`ripen` do its full work. Breakaway adds no pass.

The PC runner timed `bench=45 run=1:R mode=squads hand=squads
screen=1920x1080 passes=60 viewpasses=60` at 90946e27, on (A, the build's
default) and with `condense=0 roam=0` (B), interleaved A B A B on planet 2
and then the core world. The GPU was idle before each run. Runs 3, 5, 7
and 8 started on a down-clocked card (746 to 835 MHz), and the card warmed
from 32 to 62 C over planet 8. Raw files: branch
`claude/project-thread-26drse` at 22b76272,
`runner-out/breakaway-pr370-90946e27/`. Times in ms:

| Run | Frame p50 / p95 | GPU step | of which `ripen` | View | Walkers at 0:45 |
|---|---|---|---|---|---|
| 1, planet 2, on | 7.18 / 7.86 | 0.977 | 0.055 | 6.461 | 14,632 |
| 2, planet 2, off | 7.13 / 7.82 | 0.949 | 0.010 | 6.431 | 19,581 |
| 3, planet 2, on | 7.24 / 7.93 | 0.985 | | 6.548 | 14,632 |
| 4, planet 2, off | 7.21 / 7.86 | 0.953 | | 6.471 | 19,581 |
| 5, core world, on | 11.14 / 16.70 | 6.179 | 0.405 | 7.791 | 94,367 |
| 6, core world, off | 11.23 / 16.75 | 6.280 | 0.090 | 7.941 | 130,683 |
| 7, core world, on | 11.21 / 16.78 | 6.189 | | 7.825 | 94,367 |
| 8, core world, off | 11.25 / 16.75 | 6.279 | | 7.941 | 130,683 |

- On planet 2 the step costs 0.03 ms more (`ripen` +0.045 ms) and the
  frame's p95 0.04 to 0.07 ms more, about the gap between two runs of
  the same arm (7.86 and 7.93).
- On the core world condensing is cheaper: `ripen` costs 0.32 ms more, but
  over a quarter fewer walkers to sense, breed and draw save more (step 6.18
  against 6.28, view 7.79 against 7.94). The frame does not move.
- Both arms' core-world frame max (544 to 579 ms) is the head start's 87 s
  of growth before the clock.
- At 0:45 on the core world the squad had killed 1,971 walkers with
  condensing on and 25 without (shots 92 against 189). That is one game,
  and why is not checked; the films do not cover the core world.

## Open

- The owner's playtest. The specks a piece leaves as (the review's art
  note, and STP's objection that breakaway steers straight at a chosen
  crystal and skips pumped ones) are unchanged.
- STP's alternative (pieces follow a far smell; drop `ROAM_FRESH`) stays a
  next step if the owner finds the straight runs read as scripted.
