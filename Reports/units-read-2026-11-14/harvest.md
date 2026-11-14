# Harvesting that looks like work (soldier side), 2026-11-14

The owner's playtest (04:53Z, via the art director): "harvesting crystals
could look way better". The crystal's own reaction is the crystals thread's
(branch `claude/project-thread-e0mk6m`, rides with PR #228); this is the
soldier and larder side.

## What changed

- **One shared input.** `Game::last_takes` lists this step's takes (`Take`:
  who, which crystal, where the taker stands, how much charge), cleared every
  step, filled in the harvest loop of `turn.rs`. Agreed with the crystals
  thread; they read the per-cell charge buffer for their glow and can use
  this list to line up with the soldiers. Test
  `takes_are_listed_each_step_for_the_harvest_look`: the listed takes equal
  what the soldier carried in, every step.
- **Working pose and pulse.** A soldier that took this step turns to face the
  crystal (unless it is firing) and a pulse of charge light shows at its
  hands for 0.3 s (`TAKE`), so a working squad flickers as it drains.
- **The load on the sprite.** The pack fills with cyan charge from its foot
  as the soldier carries (`s.carry / Rules::carry`), hotter when full, with a
  tight glow round it. A carrier walking home is the cyan one you can follow.
  The tiny carry bar is gone.
- **The larder banks with a flash.** When food is banked
  (`Game::harvested` rises) the larder's cells flare and the whole store
  washes cyan for 0.6 s (`BANKED`).

Packed in the mark's `extra` word (`Marks::soldier_working`: carry bits
9-16, take 17-24; `Marks::building_pulse`: pulse 17-24).

## What was tried and changed

- The first glow (`exp(-d²/(0.06 + 0.12 carry))`) reached past the body.
  Each screen pixel shows one mark, the highest that claims it, so a
  carrier's faint glow claimed its neighbour's pixels and cut a dark bite
  out of it in the zoom 12 film. Tightened to `0.02 + 0.04 carry`; no bites.
- The larder flare as lit cells alone was a few pixels at zoom 12 and lost
  under the carriers walking in. Added a wash over the store's body (no new
  alpha, so it claims no neighbour's pixels).

## Measured

Fork map, seed 1, farm hand, 0:12 to 0:17: 6 to 9 soldiers, harvested 22 to
1222. Paired llvmpipe bench (`bench=30 hand=farm`, main then branch, twice):
GPU tail p50 48.2/46.3 and 45.4/45.1 ms, frame p50 68.5/65.4 and 63.6/63.9
ms. No measurable cost (a few ALU ops a soldier pixel). 1070 timing waits for
the PC.

Page: section "Harvesting" of
[Squad You Can Count](page/index.html), films in `page/harvest/`.
