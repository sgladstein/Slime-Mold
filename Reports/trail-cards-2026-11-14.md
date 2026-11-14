# Trail cards: bullets that lay real fields (2026-11-14)

This is step 5 of the rounds' look plan ([research](noita-spell-look-2026-11-14.md)).
It follows Noita's *Burning Trail* and *Gunpowder Trail*, where a projectile
leaves real material along its path and the trail is the mechanic. There
are three new flight cards (`rounds::Card::Scorch`, `Alarm`, `Mist`). Like
pierce and split, they ride on every soldier bullet rather than on a cast.

## What they do

Each frame, every soldier bullet's leg lays toolkit bursts along it, up to
its hit (`rounds::trail_bursts`, `Rounds::lay_trails`). There is a disc every
`RoundRules::trail_r` (2 dish cells) with radius 2, multiplied by the square
root of the number of copies of the card:

- **Scorch** leaves crater for `trail_crater_secs` (4 s). The mould shuns
  crater, so the trail is a fire line it will not cross while it lasts.
- **Alarm** lays alarm at `trail_alarm` (0.5) times a blast's. The mould
  flees alarm, so the trail works as a fence.
- **Mist** lays `trail_cloud` (0.4) of the gas cloud from
  [the gas cloud report](gas-cloud-2026-11-14.md), without gassing the
  walkers under it now. It does this through a new `squad::Burst::cloud`
  field. The cloud puts to sleep whoever stands in it from the next step,
  and it shows as the violet haze.

None of them kills or costs food. Each weighs `card_weight` on the
recharge, like any flight card, and takes a slot. Every card works through
`World::burst`, so both engines play them. A burst that does not kill now
skips the GPU's kill-count read-back (`GpuWorld::burst`).

## Checks

- `trail_bursts_lay_the_same_fields_on_the_engine_as_on_the_cpu`
  (`mouldgpu`): a 16-disc leg of all three trails on the race world and the
  GPU world from the same dish. Crater, alarm and cloud each covered 486
  cells, and 0 cells were off on the GPU. Nothing was killed.
- `trail_cards_lay_discs_along_the_leg_and_cast_nothing` (spacing, radii,
  stacking) and `a_mist_round_lays_a_cloud_along_the_flight` (a real cloud,
  no casts, no food spent).

## Films and counts

[Rounds in Flight](rounds-look-2026-11-14/index.html), section "Trail cards".
The setup is Fork seed 1, `hand=hold@150,195 bank=20000`, 0:19 to 0:22, zoom
4.5. The counts at 0:22 are below. The plain row is the same run with no
round.

| round | mould | killed (shots) | shots | trail bursts |
|---|---|---|---|---|
| plain | 24,500 | 750 | 138 | 0 |
| scorch+scorch | 24,603 | 774 | 144 | 987 |
| alarm | 24,590 | 737 | 137 | (not counted in that run) |
| mist | 24,365 | 1,019 | 103 | 719 |

Mist puts the mould in front of the squad to sleep. Sleepers are still
killed by shots, so the squad kills more with fewer shots. This is one
seed, so treat it as a tuning note and not a result.

Frame cost on llvmpipe: a run to 0:22 took 8.6 s with no round, 10.0 s with
scorch (987 bursts) and 10.4 s with mist (719 bursts), so about 1.5 ms per
trail burst on the software GPU. Each burst is its own GPU submit. Batching
a frame's trail bursts into one pass is the obvious next step if the 1070
shows the same cost. That timing goes in the C6 batch.

## Not done

- Crater and alarm on bare ground are not drawn by the view. Those two
  trails show only through the mould's answer (it shuns or flees the line).
  A drawn crater field would sit close to the char look the owner rejected
  on 11-12 04:38Z, so that is for the owner or the graphics thread to call.
- A bait trail (food scent along the flight) needs a scent-laying burst,
  which `World::burst` does not have yet.
