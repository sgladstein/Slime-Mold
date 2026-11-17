# E1b and C2: the slime reads its old ground, and old ground burns (2026-11-17)

The simulation-use plan's E1b and C2 ([plan.md](plan.md), draft 3): the crust
(E1a, [report](e1a-crust.md)) gets its effects. Page:
https://claude.ai/artifact/6Qpdh7rTKKpGEgMhD9btAR (source `page-e1b/`).

## What was built

Four `BigParams` (and `Settings`, `gpu::set` keys), each off at 0, in both
engines:

| Key | What it does |
|---|---|
| `crust_weight` | sensors read weight x crust beside the trail, after the scar; over 0 the slime keeps to ground it held, under 0 it moves on |
| `crust_fast` | a walker's step on crust is x (1 + fast x crust), never past 1.5 cells by the crust (`big::CRUST_STEP_CAP`: `walk` tests only the landing cell, so a longer step could hop a two-cell wall) |
| `crust_fuel` | crust burns as that many plants a unit (`plants::fire_step` takes the crust; fire eats the plants first, then the crust, and both leave char); lights (`ignite`) count it too |
| `char_barren` | char of this much or more is barren ground while it lasts: no child born there, `barren_upkeep` on it |

The living slime's trail is still wet to fire (`fire_damp`), so the fed body
is its own firebreak (owner 01:13Z: fed slime fireproof, starving slime
burns). Blasts don't wipe the crust (the plan said they would): the slime
comes back to ground it was blasted off, which is E1b's point.

On the GPU: a Params row `CRUST_FX_AT` (byte 1200; the uniform 1200 -> 1216
bytes) with the four numbers (the first three 0 on a dish without the
crust, `char_barren` 0 without plants); `sense_at`, the walk's step, the
`fire` and `ignite` passes, upkeep and the birth spot read them
(`crust_of`, `charred`).

`crust=1` now also turns on `crust_fuel` 3 and `char_barren` 0.3 (the fire
films below). The pull and fast ground stay 0 until the owner picks a sign.
The crust stays off by default until the blind "name the ground" quiz.

## Twins (llvmpipe)

| Test | Result |
|---|---|
| `the_crust_pull_and_fast_ground_match_the_cpu` (weight 2, fast 1; warmed 40 ticks, a strike, four ticks on both) | crust L1 0, 0 of 25,000 walkers off |
| planted: GPU sensors blind to the crust | 1,715 walkers off (caught) |
| planted: GPU step blind to the crust | 23,217 walkers off (caught) |
| `crust_burns_on_the_gpu_as_on_the_cpu` (Flats, plants 0.04 a cell under the fire's fuel 0.05, crust worth 1, char over 0.3 barren, pull and fast ground on; a burn at 12 s; 24 s) | plants, fire, char and crust all max diff 0 at every second; walkers equal (1,810 at 24 s); 377 cells burning and 263 barren at 24 s, so the crust carried the fire |
| `plants_and_fire_run_on_the_gpu_as_on_the_cpu`, `the_crust_matches_the_cpu`, the scar, life-cycle and one-tick twins | still pass |

CPU tests: `fire_runs_along_crust_on_bare_ground` (a fire runs along a crust
strip on bare ground, leaves char, stops at its end; worth 0 it doesn't
burn) and `soil_kinds_act_on_their_terms` (a dish all char over
`char_barren` plays exactly as the all-barren dish, trail and living; at 0
as the plain one).

## Films (planet 3, run seed 1, idle hand, llvmpipe)

One seed each, a look, not a census. Raw lines in `e1b-raw/`.

**The pull** (`crust=1 crust_weight=W`, `crust_fast=1` with the +-8 runs):

| Weight | 3:00 mould | 5:00 mould | Ground ever held at 5:00 (crust over 0.05) | Over 0.3 | Walkers' cells on crust over 0.3 |
|---|---|---|---|---|---|
| -8 (fast 1) | 35,674 | 50,869 | 98,284 | 75,823 | 44,978 |
| -2 | 30,355 | 43,920 | 80,749 | 56,017 | 37,045 |
| 0 | 30,166 | 39,999 | 72,507 | 53,555 | 37,798 |
| +2 | 31,807 | 45,595 | 71,720 | 53,954 | 39,047 |
| +8 (fast 1) | 33,820 | 43,366 | 73,244 | 52,964 | 39,243 |

Under 0 the slime moves on: at -8 it has covered 36% more ground by 5:00 and
fills the middle of the map in a diffuse sheet; at -2, 11% more. Over 0 the
idle film shows little: the range is the same as at 0 and the slime keeps
its roads. The plan's measure for +w (blast half the home away at 2:00,
then count the old ground re-held 60 s later) needs a push this idle hand
doesn't give. Six `rounds=blast` booms on the home at 2:00 killed nothing
in run mode (0 killed, 0 flung each; the watched disc kept its 171
walkers), so that film is still to make, with a hand or tool that does
push.
Both signs grew the mould at 5:00 over 0 on this seed (one seed: not a
finding).

**The fire** (`crust=1 crust_fuel=F char_barren=0.3`, a plain fire lit over 4
cells at 2:30 on the clearing the slime left after 1:00, `boom=150,120,50,4`):

| Fuel | 2:32 | 2:40 | 2:50 | 3:00 | 3:30 | 4:00 | Mould at 4:00 |
|---|---|---|---|---|---|---|---|
| 0 | 32 / 0 | 12 / 1 | 2 / 0 | 0 / 0 | 0 / 0 | 0 / 0 | 35,004 |
| 1 | 51 / 0 | 41 / 3 | 30 / 0 | 4 / 0 | 0 / 0 | 0 / 0 | 35,004 |
| 3 | 84 / 0 | 180 / 8 | 331 / 36 | 889 / 168 | 4,382 / 3,530 | 2,743 / 6,336 | 34,785 |
| 6 | 139 / 0 | 371 / 14 | 1,111 / 105 | 2,853 / 1,138 | 6,682 / 6,771 | 4,740 / 8,140 | 34,396 |

(cells burning / fine cells of char over 0.3). At 0 and 1 the light dies in
20-30 s on the clearing's plants; at 3 a front crosses the whole clearing
the slime left and stops at the living body's edge, leaving barren char
behind; at 6 it does the same faster. The mould is barely touched (35,004 ->
34,785): the fire takes the ground, not the body. 3 is in `crust=1`.

## Frame cost

Off (the default): one more uniform read a sensor and a step, one in the
fire pass, upkeep and births. On: one load from `news` a sensor read and a
step, one load and maybe a store a cell in the fire pass, one load of char
a walker in upkeep and a birth. 1070 timing to book with E2's (the PC
runner holds the scar's).

## Rejected or left

- Blasts wiping the crust (plan E1a): left out, so the slime returns to
  blasted ground.
- Fuel 1 (the first number): a light on old ground went out as on bare
  ground (20-30 s against 30 s).
- The +w film with a push: still to make.
- `crust_wade`: dropped in draft 3.
