# E2a: the slime sets where it is hurt (2026-11-17)

The simulation-use plan's E2a ([plan.md](plan.md), draft 3, "E2. Setting"):
a walker on its old ground (the crust, E1a) where the scar (step 0) says it
was hurt stops, stops breeding and takes blows to kill. The owner chose
"toughen while under fire" (02:30Z card), so it sets while the fire lasts.
Page: https://claude.ai/artifact/Vx8Qz8v3VAqvZaUL95FMK2 (source `page-e2a/`).

## What was built

Four `BigParams` (and `Settings`, `gpu::set` keys), in both engines, off with
`set_scar` 0 (the default):

| Key | What it does |
|---|---|
| `set_scar` | a walker on crust of `set_crust` or more sets once its cell's scar is this much (0: nothing sets) |
| `set_wake` | a set walker wakes when its cell's scar falls under this (the gap stops flicker) |
| `set_crust` | the crust a walker must stand on to set |
| `set_armour` | the blow that kills a set walker (1-255): at 3 it shrugs off two and dies to the third |

A set walker doesn't move or breed (its sensors and step are skipped; on the
GPU the sense pass sends it nowhere and the walk pass only tallies it). It
still deposits, eats and pays its upkeep, so a set rim cut off from food
starves. Its armour is the lump's (`armoured`): each blow that would kill it
spends one; a set walker in a lump uses
the lump's. A birth clears the child's word. Needs the scar and the crust: a
dish without either never sets.

`setting=1` turns it on at once: the crust (as `crust=1`) and the scar as a
plain memory (keep 0.999, gain 0.01, no pull) if they are off, then
`set_scar` 0.05, `set_wake` 0.01, `set_crust` 0.2, `set_armour` 3. It is off
by default: the crust is (until the owner's blind ground quiz), and set
slime has no look of its own yet (below).

On the GPU: a Params row `SET_AT` (byte 1216; the uniform 1216 -> 1232
bytes); one word a walker after the crust block in `news` (`set_word`,
`Engine::set_of`); `set_step` in the sense pass, a skip in the walk pass,
the set check in `armoured` and `breed`, a clear in `born`.

## Tests

| Test | Result |
|---|---|
| `setting_matches_the_cpu` (llvmpipe; the Grudge's settings with the crust, warmed 40 ticks under strikes, `set_scar` at the 60th percentile of the scar under walkers on crust, wake at half; then a radius-8 strike on both) | set words, walkers and living equal |
| planted: GPU `set_scar` 0 | caught |
| planted: GPU armour 1 | caught |
| `walkers_on_crust_set_under_fire_and_wake_after` (CPU, a small dish struck every 10 ticks at its home pile for 200 ticks) | set off: 574 killed, 0 set. Set 0.1: 61 killed, 537 set at most, none moved while set, 8 still set 1,500 ticks after the last blow |
| the crust, scar, fire, life-cycle and one-tick twins | still pass |

## Films (planet 3, llvmpipe, `gun=rifle crust=1 crustlook=1`)

A look, not a census: the plan's deciding measure named 8 paired seeds;
this is the 2 x 2 on one seed and the standing squad on four.

**The 2 x 2** (seed 1, `hand=hold@344,169`: eight rifles stand at the
first front; `hold@344,169,45,360,120` walks them 50 cells north to a fresh
front at 45 s). Kills in each window:

| | 0-45 s | 45-60 s | 60-75 s | 45-75 s | Soldiers at 90 s | Walkers set at 60 / 75 s |
|---|---|---|---|---|---|---|
| Stands, setting off | 442 | 125 | 401 | 526 | 5 | - |
| Stands, setting on | 422 | 117 | 204 | **321 (-39%)** | 7 | 157 / 164 |
| Moves, setting off | 442 | 291 | 801 | 1,092 | 4 | - |
| Moves, setting on | 422 | 317 | 399 | **716 (-34%)** | 7 | 912 / 1,316 |

Setting cut the standing squad's kills by 39%, and moving recovered them
(716 against 321 standing). But the move helped as much with setting off
(1,092 against 526), and the new front set within 15 s (145 set at 45 s,
912 at 60 s), so the moved squad also killed a third less than without
setting. The plan's shape (kills fall for the standing squad, recover when
it moves) holds; setting does not yet punish standing more than moving.
The scar at the new front builds as fast as at the old: at keep 0.999 a
rifle line's scar passes 0.05 in seconds. The dials for that are
`set_scar` (higher sets only where fire lasts) and the scar's gain.

**The standing squad on four seeds** (`hand=stand`, four rifles attack-move
to the core and stop at the first slime; killed by 75 s, before the 80-90 s
surge that ends most of these films):

| Seed | Off | On | Walkers set at 60 s |
|---|---|---|---|
| 1 | 733 | 414 | 217 |
| 2 | 474 | 454 | 0 |
| 3 | 504 | 484 | 15 |
| 4 | 1,253 | 1,079 | 1,098 |

Fewer killed on 4 of 4 (sign test p = 0.06 one-sided; one comparison, so no
correction). How much depends on where the squad stops: on seed 2 it stops
off the old ground and nothing sets (20 fewer from the scar's small
differences); on seed 4 the slime holds a set rim of a thousand walkers.
Soldiers left at 90 s: 7 with setting, 6 without, over the four.

What you see: with setting on the body stays further from a standing squad
(seed 1 and 4 at 75 s) and the squad shoots at a rim that doesn't come on;
without it the slime presses into the line. The mould fell from about
43,500 to 29,000 between 75 and 90 s in all the seed-1 films, on and off;
the squad's kills are only about 1,000 of that (the run's surge).

## Frame cost

Off (the default): one uniform read a walker in the sense and walk passes
and in `armoured`. On: one load from `news` a walker in the sense pass (the
set word), one scar and one crust load for a walker on crust, one load in
the walk pass, `breed` and `armoured`; a store when a walker sets or wakes.
1070 timing to book with the scar's and crust's (the PC runner holds them).

## Left for E2b

- **The look.** The plan draws set walkers by their own state, a material
  settled by a blind A/B (translucent amber against raised and glossy; not
  dark ridges, not bronze). The view reads fields, not walkers, so it needs
  a count of set walkers a cell: the same count E2b's ground bit needs.
  Until then set slime looks like the body; it shows as a rim that holds.
- **Blocking soldiers** (E2b): the ground bit at `set_block` set walkers a
  cell, `steer.rs` stopping a soldier at it, unset walkers through it at a
  slow step, and the nest-door check on planets 1-2.
- **The 8-seed census**: for the balance pass, with a moving hand that
  works on every map (the `hold@` posts are seed 1's).

## Rejected

- `set_scar` 1 (the first number): 3-6 walkers set. The scar runs to the
  hundreds only on the cells shot most, which the flinch moves walkers off;
  under the walkers it is small (p99 under 0.01 while the line holds).
- `set_scar` 0.2: 150-167 set and 289 killed by 60 s against 265 at 0.05
  (one seed); 0.05 sets more of the rim.
