# Step 0: the Grudge's threat memory on the GPU (2026-11-17)

The simulation-use plan's step 0 ([plan.md](plan.md), draft 3): port the
Grudge's scar to the GPU engine, so later steps (E2's setting slime) can read
where the slime was hurt, and put the Grudge back in the run's ladder.

## What the scar is

A field a fine cell (`BigDish::scar`, CPU since 2026-11-12,
[report](../mould-tiers-campaign-2026-11-12.md), *Habituation*). Each fine
step, after the alarm spreads, every agar cell's scar becomes
scar x `scar_keep` + `scar_gain` x alarm; walkers' sensors read
`scar_weight` x scar beside the trail and scent. The Grudge's keys: keep
0.9999 (about 40 game seconds to halve at two fine steps a tick), gain 0.01,
weight +0.5, so it comes back to where it was shot after the alarm (a second
or so) has faded.

## What was built

- `Settings::scar_keep`, `scar_gain`, `scar_weight` and their `gpu::set` keys,
  mapped both ways with `BigParams` (`Tier::settings` no longer refuses the
  Grudge).
- A Params row at `SCAR_AT` (byte 1168; the uniform grows from 1168 to 1184
  bytes): keep (0 unless the alarm is on too, as the CPU makes the field only
  then), gain, weight.
- The field is the damage memory's second block (binding 47, reserved for it
  since 11-12, so no new binding). The trail pass updates it right after
  the alarm (`scar_step`, the post-spread alarm, agar cells only; a gone
  cell's alarm is 0, so its scar only fades), and `sense_at` adds
  `scar_w x scar` beside the alarm term in the CPU's order.
- `Engine::inject` writes the CPU's scar, and `Engine::scar_of` reads it back.
- `run::LADDER` is 7 long: the Grudge is planet 7, and planet 8 picks from
  the Sporer, Sleeper and Grudge. Its `does_to_you` row: "It remembers
  where you hurt it, and comes back there", `scar_weight` 1 (twice the
  campaign tier's 0.5; a first number, see the film below).

## Twin (llvmpipe, 2026-11-17)

`the_scar_matches_the_cpu` (new, `src/bin/mouldgpu.rs`): the Grudge warmed 40
ticks on the CPU with three strikes on the body, then four ticks on both from
the same state.

| Check | Result |
|---|---|
| cells scarred over 1e-3 | 9,714 |
| scar L1, alarm L1 | 0, 0 |
| walkers off after 4 ticks | 0 of 25,000 |
| planted: GPU gain doubled | scar L1 0.18 (caught) |
| planted: GPU sensors blind to the scar, weight 20 on the CPU | 378 walkers off (caught); weight 20 on both: 0 off |

At the tier's own weight (0.5) a four-tick-old scar turns only 6 of 25,000
walkers when the GPU ignores it, under the twin's 1-in-1000 bar, hence the
plant at weight 20. The life-cycle twin gained a dish, "the Grudge, growth on
crystals" (a strike, a wall beside it, four ticks): 0 walkers off, every
field 0 or under 2e-9 L1. `mind_statuses_match_the_cpu` and
`one_gpu_tick_matches_one_cpu_tick` still pass (the uniform moved).

## Frame cost

With the scar off (every tier but the Grudge) the shader takes one more
uniform read a sensor (`scars()`), and no buffer work. With it on: one
atomic load and store a cell a fine step in the trail pass, and one more load
a sensor read. Timing on the GTX 1070 is booked through the PC runner; the
result goes here.

## Rejected or left

- A new binding for the scar: binding 47's second block was reserved for it.
- The planted "sensors blind" check at the tier's weight: too weak to catch
  (6 walkers in 25,000).
