# C4b: the fire-shy Sprawler (2026-11-17)

C4's `fire_alarm` ([c4-fire-alarm.md](c4-fire-alarm.md)) as one tier's
trait instead of a switch for every slime. The game designer (11-17 12:10Z,
and 14:38Z on the playtest pack): set for every slime, every fire becomes a
fence (0 burned while lit against 119 without it), so it came out of the
new-slime preset; give it to one fire-shy tier, the Sprawler. The
integration thread (14:39Z) passed it here.

**Status: on for the Sprawler's planet in a run (planet 2), off for every
other slime.** The C4 page (https://claude.ai/artifact/Nq2iGDUWQN4SuF5VruRQF6, source
`page-c4/`) is the film of the trait
itself: at 50 the slime rings a fire and the fire eats holes in its edge.

## What was built

- `run::does_to_you(Tier::Sprawler)` gains the key `fire_alarm=50`
  (`SPRAWLER_KEYS`), set on the GPU engine where a run lands on the
  Sprawler's planet, as every tier's keys are. The planet card and landing
  alert now say "Its fast threads smell your soldiers from far off, but it
  shies from fire. Bring Tinder" (the designer, 15:26Z).
- `tiers::set_key` takes `fire_alarm`, so the CPU world and the probes can
  set it too.
- An off switch on the command line, as for every engine key:
  `fire_alarm=0` (engine keys apply after the tier's).

## Tests

| Test | Result |
|---|---|
| `every_ladder_tier_does_something_to_you`, extended | the Sprawler's keys set `fire_alarm` 50 on both the GPU settings and the CPU params; every other rung leaves it 0 |
| `cargo test --release --locked --lib`, clippy | 597 pass; clippy clean |

## A look on the Sprawler's planet (run seed 1)

Five plain fires (`light=40,X,Y,8` at (93, 87), (120, 40), (60, 150),
(250, 60), (200, 260); and, in runs of their own, one at 50 s under the
body at (150, 176) and one at (326, 190), each with and without the key) on run
seed 1's planet 2, idle hand: none caught. That planet's ground has nothing
to burn where it was lit, and the slime's own trail is wet. So on this seed
the trait shows only with fire that brings its own fuel (the gun thread's
Torch and the coming Tinder), not a plain light. Not checked on other run
seeds.

**The designer (15:26Z): keep it on the Sprawler.** A planet with nothing
to burn is the point: plain fire isn't the answer there, Tinder (which
brings its own fuel) is, so the choice is to buy Tinder before the
Sprawler's planet, the shop as strategy.

## Left

- The run shop should offer Tinder at least once before the Sprawler's
  planet (the designer, 15:26Z; the run or gun card thread's).
- A film on a Sprawler planet with fuel from the fire cards, once Tinder is
  in.
