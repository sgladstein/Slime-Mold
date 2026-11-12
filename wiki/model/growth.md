---
title: Growth and starving on the big dish
kind: mechanism
status: open
updated: 2026-11-12T00:40Z
sources: [Reports/growth-2026-11-11.md, Reports/mould-shape-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md, Reports/food-tubes-2026-11-11.md]
code: [BigParams::upkeep, BigParams::energy_flow, BigParams::start_energy, BigParams::birth_energy, BigParams::birth_cost, BigParams::starve, BigParams::capacity, BigDish::alive, BigDish::energy, DRAW_STARVE, DRAW_BIRTH, CLAIM_BIRTH]
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
From a small blob on the home crystal (the Larder race start) the network
with growth explores, then joins the crystals with tubes one after another:
a median 19 of 22 on its main body at 60 s, against 2 for the food-reach
droplets, 16 of 16 seeds. Without growth the small start still makes tubes
(12-15 joined at 90 s against 11-21, 3 seeds, too few to separate); growth
adds a mould that gets bigger and a finer mesh
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
- `BigParams::energy_flow` (off, CPU only): keep it for the cut-a-tube
  lever? A cut has not been tested.
- With [the life cycle](life-cycle.md) on, only ordinary walkers pay
  upkeep, starve, divide and share energy; a spore or a dormant walker
  keeps its energy until it lands or wakes, and a struck walker frees its
  slot for a birth. Should sleeping cost upkeep? Untested in play.

## Sources

- [growth](../../Reports/growth-2026-11-11.md), all sections.
- [mould shape](../../Reports/mould-shape-2026-11-11.md), grass and crystals, size on crystals.
- [Larder World roadmap](../../Reports/larder-world-roadmap-2026-11-11.md), W0.
- [food tubes](../../Reports/food-tubes-2026-11-11.md): the race start joins crystals with tubes.
