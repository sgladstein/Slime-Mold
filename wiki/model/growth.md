---
title: Growth and starving on the big dish
kind: mechanism
status: open
updated: 2026-11-11T19:30Z
sources: [Reports/growth-2026-11-11.md, Reports/mould-shape-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md]
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
On crystals the size stops near 20,000 whatever the charge: the crystals'
area sets the intake and the charge only how long it lasts; grass under
the crystals swamps them
([mould shape](../../Reports/mould-shape-2026-11-11.md), *On crystals*).

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
