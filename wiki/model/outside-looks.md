---
title: Outside looks and creature rules on the big dish
kind: mechanism
status: open
updated: 2026-11-12T01:11Z
sources: [Reports/mould-shape-2026-11-11.md, Reports/jenson-physarum-2026-11-10.md, Reports/jenson-compare-2026-11-10.md, Reports/jenson-looks-2026-11-11.md, Reports/outside-rules-2026-11-11.md, Reports/creatures-2026-11-11.md, Reports/spread-mix-2026-11-11.md, Reports/research-review-2026-11-11.md, Reports/first-reach-2026-11-11.md, Reports/species-sets-2026-11-11.md, Reports/food-tubes-2026-11-11.md, research/sources/jenson-physarum-post.md, research/sources/jones-2010.md, research/sources/mcpm-2022.md, research/sources/jenson-36-points.md, research/sources/bleuje-interactive-physarum.md, research/sources/lague-slime-simulation.md, research/sources/polyphy.md, research/sources/primordia.md, research/sources/reginato-2024.md]
code: [BigParams::respawn, BigParams::blob_r, BigParams::haze, BigParams::sense_scale, BigParams::deposit_cap, BigDish::infect, BigParams::chance_exp, BigParams::explore_spread, BigParams::graze_weight, BigParams::species_convert, BigParams::species_sets, SpeciesSet]
---

# Outside looks and creature rules on the big dish

## Now

Jenson's clips run the same step as ours; their look comes from how it is
run and drawn: no one-walker-a-cell limit (with our numbers that folds the
walkers into ropes), a soft brightness curve, and respawning walkers for
haze. Off-by-default rules on the big dish reproduce trunks (a dense start),
contagion that runs along tubes, and the outside creature rules: species
with bands, explorers, the chance-weighted turn (more loops), droplets that
crawl once they graze, and kinds mixed in one mould that sort apart
(`mix_share`; [spread-mix](../../Reports/spread-mix-2026-11-11.md)). Feathery fronts are still not reproduced, and every
look so far was judged with a fixed walker count. On food piles, grazing is
what makes droplets forage: grazers reached more far piles than still
droplets on 15 of 16 seeds and fewer on none, and scouts added to grazers
reached more on 10 of 16 and fewer on none ([first reach](../../Reports/first-reach-2026-11-11.md)).
Droplets touch food but don't join it: on the Larder race world they held a
median 2 of 22 crystals on their main body at 60 s, against 19 for the race
start's network with growth, 16 of 16 seeds; their bodies have no growth and
grazing pushes them on, so on a world they come out as waves
([food tubes](../../Reports/food-tubes-2026-11-11.md)). Use the network with
growth where food should end up joined by tubes.
Species can now have their own settings (`species_sets`): a droplet species
and a network species share one dish, each within half to twice its area
alone on 8 of 8 seeds, with no fight between them
([coexistence](../../Reports/species-sets-2026-11-11.md)). All are
controls in the [GPU sandbox](gpu-sandbox.md); tiers built on them are in
[the life cycle](life-cycle.md) and the [mould tiers](../world/mould-tiers.md).

## Why Jenson's clips look different

**Why Jenson's clips look different** (compared on the big dish,
2026-11-10, no food, spread start). Jenson runs the same step; the look
comes from how it is run and drawn. Dropping the one-walker-a-cell limit
with our numbers folds the walkers into a few ropes (0 loops at 10 s against
151.5, 8 of 8 seeds), so Jenson's rule needs other settings; sensor 45,
turn 22.5, sensors 18 out gave a polygon mesh. Half the look is the
brightness curve: the game's saturates at a trail of about 15, and the same
dish drawn with a soft curve or by walkers a cell shows veins as thin as
Jenson's. Respawning 1% of walkers a step (`BigParams::respawn`, off by
default; Jenson's later work respawns every 1,000 steps) held a fine mesh
and gave Jenson's haze. Four times the walkers changed little. In the GPU
sandbox's Jenson preset the count matters only with the limit
([mould shape](../../Reports/mould-shape-2026-11-11.md), 8 seeds, 30 s):
without it 32,000 to 2,000,000 walkers all fold into strands (0-1 loops);
with it 32,000, 100,000 and 200,000 give 43-54, 57-70 and 120-144 loops (the
GTX 1070 agrees: 46-53, 59-69, 122-144), a
net thickening into a sheet with holes (the sandbox forces the limit off
above half the agar). Every GPU
port read (Bleuje, Lague, PolyPhy, gpu-io, Barradeau) drops the limit.
A follow-up ([jenson looks](../../Reports/jenson-looks-2026-11-11.md))
added off-by-default rules for the three looks still missing. **Trunks**
come from the start: a dense centred disc (`BigParams::blob_r`, `haze`)
keeps its edge as a thick ring inside a mesh at 20 s, while respawn stays
slow (0.001). **Contagion** is built on the big dish (`sick_steps` and the
other `sick_*` fields, `BigDish::infect`, the same rule as Two Fevers): it
runs along the tubes in 2-4 s, coarsens the mesh while it passes when sick
walkers steer differently, leaves no mark by 20 s. Whether a knife cut stops it is untested: the
cut was made before the front arrived and healed first. **Feathery fronts** are still not reproduced: with Jenson's rule any
haze organises on its own. 36 Points' trail-dependent settings
(`sense_scale`) gave long curves, a ring that expands across the dish, or a
lasting round trunk; Bleuje's square-root deposit (`deposit_cap`) gave even
polygons at once. Every outside source has a card in
[`research/sources/`](../../research/sources/README.md).

## Open

- Jenson's feathery fronts: seven mechanisms failed ([jenson looks](../../Reports/jenson-looks-2026-11-11.md)); explorers that turn into networkers one by one (`explore_spread`) grow radial feathery veins from a disc, one seed so far ([outside rules](../../Reports/outside-rules-2026-11-11.md)).
- A creature that moves as a body: still droplets crawl once they graze a regrowing field ([creatures](../../Reports/creatures-2026-11-11.md)). What lets one body beat another is partly open: numbers win where two species are mixed (a 55% side ends with 75-88%), but sorted bodies hold their border against a denser side. With no avoidance two species don't sense each other, so they pass through each other and only conversion acts. Per-species settings now exist ([coexistence](../../Reports/species-sets-2026-11-11.md)); a fight between two species with different rules is untested.
- How the creatures behave at the game's operating point. The independent [review](../../Reports/research-review-2026-11-11.md) found every look so far was judged with no food, a fixed walker count and runs of 20-30 s. The dish wall is also a rail that walkers ride. Its proposed bar for a creature: identity over time, self-propulsion, behaviour away from walls and at 0.25-4x the walkers, a 2-5 min run, and a play probe.
- Tiers with a life cycle (spores, dormancy, an alarm the player's strike lays) and how each answers attack: [the life cycle](life-cycle.md).
- Whether contagion can leave a lasting mark for play (see the contagion entry in [`dead-ends.md`](../../Reports/dead-ends.md)). The outside code was read on 2026-11-10; only PolyPhy, gpu-io (MIT) and Barradeau (public domain) may be reused.

## Sources

- [mould-shape-2026-11-11.md](../../Reports/mould-shape-2026-11-11.md): walker count with and without the crowd limit.
- [spread-mix-2026-11-11.md](../../Reports/spread-mix-2026-11-11.md): spread from one spot, and mixed walkers in one mould.
- [research-review-2026-11-11.md](../../Reports/research-review-2026-11-11.md): the independent review of the creature work.
- [first-reach-2026-11-11.md](../../Reports/first-reach-2026-11-11.md): which droplet moulds and mixes reach food piles first.
- [species-sets-2026-11-11.md](../../Reports/species-sets-2026-11-11.md): per-species settings, and a droplet species beside a network species.
- [food-tubes-2026-11-11.md](../../Reports/food-tubes-2026-11-11.md): why the food-reach droplets come out as waves, and the network with growth that joins food.
- [jenson-physarum-2026-11-10.md](../../Reports/jenson-physarum-2026-11-10.md), with the owner's PDF and GIFs in [its folder](../../Reports/jenson-physarum-2026-11-10/).
- [jenson-compare-2026-11-10.md](../../Reports/jenson-compare-2026-11-10.md): the clips against the big dish, and how the outside versions build the step.
- [jenson-looks-2026-11-11.md](../../Reports/jenson-looks-2026-11-11.md): trunks, contagion and fronts on the big dish; [page](https://claude.ai/artifact/DqJxgjQGVgnwQNwbbzoGrg).
- [creatures-2026-11-11.md](../../Reports/creatures-2026-11-11.md): grazing droplets that crawl, two species of cells, conversion.
- [outside-rules-2026-11-11.md](../../Reports/outside-rules-2026-11-11.md): outside rules judged as creatures (species, explorers, droplets) and a survey of 14 more open-source simulations.
- Source cards: [index](../../research/sources/README.md); [Jenson's post](../../research/sources/jenson-physarum-post.md), [36 Points](../../research/sources/jenson-36-points.md), [Jones 2010](../../research/sources/jones-2010.md), [MCPM](../../research/sources/mcpm-2022.md), [Bleuje](../../research/sources/bleuje-interactive-physarum.md), [Lague](../../research/sources/lague-slime-simulation.md), [PolyPhy](../../research/sources/polyphy.md), [Primordia](../../research/sources/primordia.md), [Reginato 2024](../../research/sources/reginato-2024.md).
