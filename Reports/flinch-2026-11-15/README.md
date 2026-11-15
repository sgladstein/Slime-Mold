# The flinch: the slime backs off fire for a moment (W19), 2026-11-15

The whole-game review (`Reports/whole-game-review-2026-11-15/README.md`)
item **W19, "A hit should make the body flinch, and the model already
can"**: "alarm keep about 0.9, so it is a flinch and not a retreat, on
every tier. Draw the alarm at a tenth of its strength (W46). Show it as an
A/B." The haze strength (W46) went in with #354 at a quarter; this is the
flinch.

Page for the owner (blind A/B of four strengths):
https://claude.ai/artifact/NiqQovqWtGqbzBX9YsM1bS (source `page/`). The
key is `raw/ab-key.txt`, not on the page.

## What it is

Every shot already lays an alarm in the engine's alarm field
(`squad::SHOT_ALARM` 50 within twice the strike's radius). On the plain
mould the field is off and nothing reads it. The flinch turns it on and
has the walkers read it at -1, so ground a shot just hit is ground they
turn away from. At keep 0.9 a fine step, one shot's alarm is gone in a few
fine steps, so the body pulls back only while the fire keeps coming and
closes in again when it stops. The old flee (keep 0.98, 2026-11-12) was the
same rule with a slow fade, a retreat rather than a flinch.

- `BigParams::flinch` and `gpu::Settings::flinch` (key `flinch=K`, both
  engines; 0 is off). It is folded into `alarm_keep` and `alarm_weight` when
  the world is made (`big::flinch_alarm`, `BigParams::flinched`,
  `Settings::flinched`, called by `BigDish::build` and the `GpuWorld`
  constructors), so the keys may come in any order.
- It acts only on a mould with **no alarm of its own** (keep and weight
  both 0): the field at keep `flinch`, read at `big::FLINCH_WEIGHT` (-1).
  The Swarmer, Sporer, Sleeper and Grudge keep theirs, because their alarm
  is what makes them what they are (the Swarmer floods to a wound). So in a
  run the flinch is on planets 1-3 (Drifter, Sprawler, Trunk) and off on
  4-8. The review asked for every tier; a Swarmer that both floods to and
  backs off a wound reads one field one way, so that is not possible
  without a second field.
- **On in shape C at 0.9** (`larder_world::shape_keys`), so the sandbox and
  every run planet whose tier has no alarm flinch by default. The start
  menu's Options page has **Flinch** (0, 0.8, 0.9, 0.95).
- `flinch=0.9` plays exactly as the raw keys `alarm_keep=0.9
  alarm_weight=-1` on planet 1 (seed 1, to the walker at 30 s), so the
  seed table below, run with the raw keys before the key existed, is the
  flinch.

## Does it change who wins?

Planet 1 of runs 1-5 (`run=SEED:1`, the Drifter), the squad hand
(`hand=squads`), to 2:00, three arms paired by seed: off, 0.9 and 0.95
(build 26262751, `raw/seeds/run1.sh`; raw `raw/seeds/`). Soldiers left at
60 / 90 / 120 s, walkers killed by 120 s, and the slime's walkers at 120 s:

| Seed | Off: soldiers | Off: killed | Off: slime | 0.9: soldiers | 0.9: killed | 0.9: slime | 0.95: soldiers | 0.95: killed | 0.95: slime |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 3 / 0 / 0 | 6,221 | 13,566 | 8 / 5 / 1 | 13,395 | 11,318 | 4 / 3 / 0 | 5,517 | 14,001 |
| 2 | 8 / 8 / 8 | 3,965 | 12,604 | 7 / 5 / 5 | 6,801 | 10,442 | 7 / 5 / 5 | 10,902 | 9,237 |
| 3 | 6 / 3 / 3 | 12,967 | 10,639 | 5 / 0 / 0 | 7,814 | 12,899 | 6 / 1 / 0 | 8,831 | 12,420 |
| 4 | 4 / 3 / 1 | 14,829 | 11,416 | 5 / 2 / 0 | 14,722 | 10,869 | 2 / 0 / 0 | 10,114 | 13,653 |
| 5 | 8 / 7 / 7 | 3,579 | 21,448 | 7 / 7 / 7 | 4,904 | 20,435 | 8 / 8 / 8 | 4,950 | 20,424 |

The deciding measure, named before the runs, was soldiers left at 120 s,
paired by seed:

- **0.9 against off:** more on 1 seed, fewer on 3, 1 tie (sign test p 0.63).
- **0.95 against off:** more on 1, fewer on 3, 1 tie (p 0.63).
- Walkers killed: 0.9 more on 3 of 5 (p 1.0), 0.95 more on 2 of 5 (p 1.0).
  The slime at 120 s: 0.9 smaller on 4 of 5 (p 0.38).

None of these is significant, before or after a correction for the three
measures. On five seeds the flinch does not steadily make planet 1 easier
or harder for this hand; seed 1, the first one filmed, made it look
easier (8 soldiers at 60 s against 3), and seeds 2-4 went the other way.
What it changes every time is the picture: a clear ring opens round a
squad that keeps firing (see the page).

## A unit test

`big::tests::a_flinching_mould_backs_off_fire_and_comes_back` (CPU, the
256 x 160 test dish, one pile): struck at its home pile every other tick
for 200 ticks (a squad's steady fire), 27 walkers stay within 8 cells of
the spot against 383 without the flinch; 200 ticks after the fire stops,
210 are back (418 without). Struck only every 10th tick (with a radius-2
kill) it did not flinch (400 against 403): at keep 0.9 one shot's alarm is gone before the next,
so a lone rifle barely moves it and a squad's volley does. The same test
checks the fold: the Swarmer's (0.98, +3) and the Sporer's (0.98, 0)
alarms are left alone, and folding twice changes nothing. The engines'
twin for a mould reading the alarm at -1 is
`mouldgpu::a_squad_firing_on_the_fleeing_mould_plays_on_the_engine_as_on_the_cpu`
(keep 0.98; the flinch differs only in the keep).

## Films

The page's four clips: planet 1 of run 1, the squad hand, 11 to 19 s at
the game's own pace, the camera on the squad; flinch 0, 0.8, 0.9 and 0.95
under blind letters (`raw/make_page.py`; filmed at 640 x 400, the same
game to the walker as at 320 x 200). Soldiers left at 30 / 60 / 90 / 120 s,
walkers killed by 2:00 and the slime's walkers at 2:00:

| Flinch | Soldiers | Killed | Slime |
|---|---|---|---|
| 0 (off) | 8 / 4 / 0 / 0 | 6,369 | 13,437 |
| 0.8 | 8 / 8 / 6 / 1 | 12,222 | 11,983 |
| 0.9 (the default) | 8 / 8 / 5 / 1 | 13,126 | 11,442 |
| 0.95 | 8 / 4 / 3 / 0 | 5,422 | 14,058 |

This is seed 1 on the flinch build, whose main differs from the seed
table's build (26262751) by #354's merge with main, so its numbers are
close to the table's seed 1 but not the same. With any flinch a clear
ring has opened round the squad by 12.5 s, about a second into its fire,
and from 14 s the slime's front stands off at the edge of the fire in a
band (0.95's ring a little wider); with none the slime closes on the
squad in the gap between the rocks and bites it by 14 s. The pale glow round the squad is the alarm field
itself, drawn at #354's quarter strength; with the flinch off the field is
off, so that clip has none.

## Smoke checks

On the flinch build (`flinch=0` against the default 0.9; 320 x 200,
headless):

- **The Swarmer is untouched.** Planet 4 of run 1 (`run=1:4`, squad hand)
  plays the same game to the walker at 30 s with and without the flinch
  (49,860 walkers, 284 shots, 4,049 killed): its own alarm (keep 0.98,
  +3) wins the fold.
- **Idle doesn't win, the game doesn't end early.** Planet 1 of run 1
  with an idle squad: 4 soldiers left at 2:00 with and without, and the
  slime grows (16,370 walkers against 16,278); nothing stalls.
- **A siren in the slime.** On the Fork with an idle hand, a siren at
  (256, 170), inside the body: 75 of 150 hp at 10 s with the flinch and 74
  without, worn down by 20 s in both. Its call (alarm 150 over 15 cells
  every 4 s) is read at -1 by a flinching slime, but it fades at the
  flinch's keep and does not hold the slime off.

## Frame cost

The flinch adds no pass of its own: it turns on the alarm field, which the
Swarmer's, Sporer's and Sleeper's planets already run (one 3x3 mean over
the fine grid a fine step, the same pass on both engines), on planets 1-3
and the sandbox. The view already sampled the field everywhere. The 1070
timing of planet 1 with and without it is booked through the PC runner.

This change also makes the view's alarm sample a branch, not a `select`:
WGSL's `select` runs both arms, so with the haze off (`alarm_look=0`) the
1070's `base` pass still paid 0.11 ms of #354's 0.16
(`Reports/alarm-look-2026-11-15/README.md`, *On the GTX 1070*).

## Before trying again

`Reports/dead-ends.md` lines on the flee (2026-11-12): the flee (keep
0.98) failed the census gate on the race world because random held the
core once the mould stopped overrunning soldiers there, and Wary herded
the mould off its crystals. Since then the game has changed under it: the
mould is headless (no core to hold), shape C shares its food, thickens and
surges at units near its body (#343, #350), so it defends what it holds
whatever it backs off. The re-test condition ("the mould defends its core
and crystals whatever it avoids") is what the surges were built for; this
report is that re-test for the squad hand on planet 1, and the census gate
itself is no longer a gate (build first, balance later, 2026-11-12).

## Not done

- No census of the race world or the missions with the flinch on; the
  player AI's censuses are paused until the owner's playtest.
- A siren's call on a flinching mould now pushes it off for a moment (see
  *Smoke checks*); it was ignored before. The siren's own keep (0.95) is
  used only where the slime has no field, so on planets 1-3 a call fades
  at the flinch's 0.9.

## Files

- `src/mould/big.rs`: `BigParams::flinch`, `FLINCH_WEIGHT`,
  `flinch_alarm`, `BigParams::flinched`, the fold in `BigDish::build`, the
  test.
- `src/gpu/mod.rs`: `Settings::flinch`, `Settings::flinched`, the key and
  both conversions; `src/gpu/larder_world.rs`: the fold in `GpuWorld::new`,
  `with_map`, `garden` and `garden_on_map`.
- `src/mould/tiers.rs`: the CPU key.
- `src/mould/larder_world/shapes.rs`: shape C's `flinch` 0.9.
- `src/bin/larder-world/menu.rs`: the Flinch option.
- `src/world/view.wgsl`: the haze's alarm sample in a branch.
- `raw/`: `seeds/` (the paired runs and `run1.sh`), `make_page.py`,
  `ab-key.txt`; `page/`: the A/B page and its clips.
