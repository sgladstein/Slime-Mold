---
title: The simulation look (how the dish is drawn)
kind: look
status: current
updated: 2026-11-10
sources: [Reports/mould-probe-2026-11-07.md, Reports/mould-looks-2026-11-10.md, Reports/mould-review-2026-11-10.md, Reports/mould-review-thread-handoff-2026-11-10.md]
code: [field_mould, FIELD, Dish::looks_fed, Dish::wet, Walker::asleep]
---

# The simulation look (how the dish is drawn)

## Now

The default look draws the mould **straight from the simulation**
(`look::field_mould`, on via `look::FIELD`; F7 flips back to the older tube
look). Brightness is `1 - exp(-0.3 * trail)` per cell, blended between cell
centres and screened over the dish, so there is no tube edge. Gold to grey
is hunger (seconds since fed) spread along a strand, not a fed/unfed line.
Every walker is one speck of light, sleepers dark amber; sleeping mould
takes a matte crust ramp by the share asleep; wet ground darkens to blue and
dry pales; the wild mould is its own trail on a violet ramp. Piles, the lamp
(the light profile the simulation shines), water and salt are drawn over it.
Hard edges only where the simulation is hard: salt, the rim, a cut.

**The rule for anything new** (owner, 2026-11-09 and 11-10): colour from a
held quantity through one fixed ramp, a dot per walker, no fills, outlines
or binary colour switches on graded quantities. A new mechanic is drawn in
`field_mould`, not as a new tube-style layer.


## Owner's word

- 2026-11-09: "Everything we're seeing should be based on real pixel
  simulations" ([looks report](../../Reports/mould-looks-2026-11-10.md)).
- 2026-11-10 13:51-13:54Z: "I don't like the sharp edge between the orange
  and black background. That also doesn't look real simulation"; "I want all
  the visuals to look like they're coming from a real simulation and not
  just graphics animations." Picked "Build the from-the-simulation look" on
  a card at 13:58Z ([review hand-off](../../Reports/mould-review-thread-handoff-2026-11-10.md)).
- 2026-11-10 14:49Z: "The new simulation based look is much better."
  ([looks, *Where this stands*](../../Reports/mould-looks-2026-11-10.md)).

## Tried and rejected

- The drawn pulse (a band riding along fed tube from the feeding search's
  distances): drawing only, disliked by the owner, off ([probe, *Food that
  flows*](../../Reports/mould-probe-2026-11-07.md)).
- The thresholded tube fill and the yes/no fed colour: the two sharp edges
  the owner named; kept only behind F7.
- Stain blended per pixel cost 2 ms a frame at 22.5; softened and drawn a
  cell at a time. A stain full the moment tube touched a cell changed 29-31%
  of the agar; it now builds over 10 s of dish time.
- Thinner tubes by shorter sensor reach: changes play, not just looks
  ([sensors](../model/sensors.md)).

## Open

- **Grains or glints** for walker specks, and **retiring the tube look
  (F7)**: both open in [HANDOFF.md](../../HANDOFF.md); ask before deleting,
  the owner chose to keep a key back.
- Not checked at the owner's real window size, or for shimmer from 1,800
  moving specks on a large screen.

## Sources

- [mould-looks-2026-11-10.md](../../Reports/mould-looks-2026-11-10.md): all sections, *Where this stands* for the open steps.
- [mould-review-2026-11-10.md](../../Reports/mould-review-2026-11-10.md), *Looks, ranked*: draw the walkers, finer mould, one ramp.
- [mould-review-thread-handoff-2026-11-10.md](../../Reports/mould-review-thread-handoff-2026-11-10.md): the owner's words with times.
