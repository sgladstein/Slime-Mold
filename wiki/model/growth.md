---
title: Growth and starving on the big dish
kind: mechanism
status: open
updated: 2026-11-12
sources:
  - Reports/food-tubes-2026-11-11.md
  - Reports/growth-2026-11-11.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/mould-shape-2026-11-11.md
  - Reports/one-body-2026-11-12.md
  - Reports/slime-enemy-review-2026-11-12.md
code:
  - BigDish::alive
  - BigDish::energy
  - BigParams::birth_cost
  - BigParams::birth_energy
  - BigParams::capacity
  - BigParams::energy_flow
  - BigParams::flow_loss
  - BigParams::hunger_scent
  - BigParams::lone_step
  - BigParams::lone_upkeep
  - BigParams::start_energy
  - BigParams::starve
  - BigParams::tether
  - BigParams::upkeep
  - CLAIM_BIRTH
  - DRAW_BIRTH
  - DRAW_STARVE
---

# Growth and starving on the big dish

## Now

The big dish's walker count can follow its food (Larder World W0;
`BigParams::upkeep` above 0, default 0, off). Each walker holds energy,
eats its equal share of what its cell loses (grass, and on Larder's
crystals its take `took`), pays upkeep, starves at zero with chance
`starve` a step, and divides at `birth_energy` into a free agar cell
beside it. Walkers live in `capacity` fixed slots, a child taking its
parent's slot xor one keyed mask a step, so a GPU thread a slot can run
it. On grass a fed mould grows and a starved one dies (8 of 8 seeds), and
its size is nearly linear in the regrowth
([growth, *Fed against starved*](../../Reports/growth-2026-11-11.md)).
Growth off changes nothing. On the GPU it is three passes that match the
CPU in a one-tick twin test and add 0.16 ms to a 1.82 ms GTX 1070 frame at
100,000 walkers ([growth, *On the GPU*](../../Reports/growth-2026-11-11.md)).
A fix after the merge: the GPU cleared its grass eater counts only when
grazing steered, so with growth on and grazing's weight 0 it over-ate the
grass; the twin test now runs the GPU a few ticks first, which catches it
(grass L1 0.12 before, 0 after).
On crystals the size stops near 20,000 whatever the charge: the crystals'
area sets the intake and the charge only how long it lasts; grass under
the crystals swamps them
([mould shape](../../Reports/mould-shape-2026-11-11.md), *On crystals*).
From a small blob on the home crystal (the race start) the network with
growth explores, then joins crystals with tubes: a median 19 of 22 on its
main body at 60 s against 2 for the food-reach droplets, 16 of 16 seeds
([food tubes](../../Reports/food-tubes-2026-11-11.md)).

## Owner's word

- 2026-11-11 14:06Z: "Crystal should run out which forces you to expand."
  (the Larder World ruling that needs growth; [decisions](../decisions.md)).
- 2026-11-11 14:54Z (Larder thread), on crystals that run out: "If you do nothing, the
  slime should get much stronger than you and kill you."

## Tried and rejected

- Pairing slots by cycling single bits: a small mould bred about one step
  in five ([`dead-ends.md`](../../Reports/dead-ends.md)).

## Open

- Run-dry crystals: `crystal_flow` makes the whole disc reachable (one
  seed); for a mould left alone to keep growing, a crystal's yield has to
  rise with its charge (the Larder thread's rule).
- `BigParams::energy_flow` (off; on the GPU too since 11-12, `esum` and
  `egain`, twin-tested): with `BigParams::hunger_scent`
  8 it makes the race-world mould one body on more seeds (largest part
  higher 8/8) and 2.5x its size; a cut gap refills in 1-2 s, so a cut-off
  piece only starves while the gap is held
  ([one body](../../Reports/one-body-2026-11-12.md)). On by default awaits
  the owner's look. On the designed maps a cut-off hunk alone still finds
  a crystal in about 2 s but arrives as a stream; aligning headings did not
  hold it together (dead end). In the Larder census (race 16 seeds) H keeps the
  planned gate (farm, wall beat idle and random) but rush stops winning and
  every hand scores lower; kept off by default for the owner's call.
  The one-body page's 25,000-walker films ran with one walker a cell off
  (corrected 11-12 17:10Z); the 5,000-walker game and the CPU numbers here
  are unaffected ([slime enemy review](../../Reports/slime-enemy-review-2026-11-12.md)).
- Growth's numbers are per fine step, so the race start holds at the
  game's scale 2 but not on the 2048 x 1280 GPU world (scale 8): there it
  stays a knot at home, and with trail sensors 36 fine cells out it touches
  20 of 22 crystals in 12 game minutes but joins few with thick tubes (one
  seed; [food tubes](../../Reports/food-tubes-2026-11-11.md), *For the GPU
  world*). Without growth the small start still makes tubes (3 seeds).
- With [the life cycle](life-cycle.md) on, only ordinary walkers pay
  upkeep, starve, divide and share energy; a spore or a dormant walker
  keeps its energy until it lands or wakes, and a struck walker frees its
  slot for a birth. Should sleeping cost upkeep? Untested in play.

## Sources

- [growth](../../Reports/growth-2026-11-11.md), all sections.
- [mould shape](../../Reports/mould-shape-2026-11-11.md), grass and crystals, size on crystals.
- [Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), W0.
- [food tubes](../../Reports/food-tubes-2026-11-11.md): the race start joins crystals with tubes.
