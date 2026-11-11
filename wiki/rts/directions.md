---
title: The RTS directions
kind: design
status: open
updated: 2026-11-11T15:40Z
sources: [Reports/rts-research-2026-11-10.md, Reports/two-fevers-2026-11-10/README.md, Reports/rts-design-2026-11-10.md, Reports/rts-design-2026-11-10/brainstorm/README.md, Reports/rts-design-2026-11-10/red-team/README.md, Reports/rts-design-2026-11-10/field/README.md, Reports/rts-skirmish-2026-11-10.md]
code: [Dish::rival, Dish::store, Dish::tithe, Dish::food_share, Pile::bite, Params::fed_scent, Params::flow, Params::foe_repel, Params::pieces, Params::wound, Params::table]
---

# The RTS directions

## Now

**In the build phase; the main line is now Larder World.** Three RTS demos
were built: the field skirmish ([skirmish.md](skirmish.md)), Two Fevers
([two-fevers.md](two-fevers.md)) and Larder, the mould as a squad's enemy
([shooter.md](shooter.md)), which the owner moved into a persistent GPU
world on 2026-11-11 14:06Z. In every demo so far one strategy beats the
others and the side ahead early keeps its lead about 4 games in 5; finding
counterplay is the open problem.

The design work behind them (Bloomfront, six brainstorm agents, a red team,
a retest on a field of 23 small piles): on the field the habits cycle on 1
layout of 10, and a salt wall is the one old tool that turns a lost game
([field retest](../../Reports/rts-design-2026-11-10/field/README.md)).
The directions considered are below.

## Directions considered

Two facts constrain every direction. Food share mostly counts heads on piles:
each pile pays at most `Params::table` 40 walkers' worth, split by head count
(`Pile::bite`). And the food store (`Dish::store`) sits at a median of 0,
because with `Params::pieces` on each piece turns any surplus into walkers at
once; the skirmish's tithe (`Dish::tithe`) is the currency that this needed.

| Direction | Core verb | Red team |
|---|---|---|
| Bloomfront | Lure, push, wall, cut, water, each paid from food | Weak as a direction; its two-keeper plumbing is a useful harness |
| Squeeze | Tap a held pile to send a wave of sap; hold it to charge | Home squeeze 0.000; flow costs 2.6x the step time; needs a fight scaled by load (not rerun on the field) |
| Two Fevers | Seed a strain in a neutral mould; immunity feeds the trailer | Built as a demo ([page](two-fevers.md)); v1 carriers-only had no strategy; v2 owned ground was a thin tug of war; v3 (calls, fever by tube, foraging, pinch) gives three plans that form a ladder, and the leader snowballs |
| Tincture | The same, but converts by majority | Feeds the leader; a different game from Two Fevers, so pick one |
| Taut | Set each source slack, even or taut | Fails on The Wild One; wins share on the field, but untested per pile and always best at full |
| Grain, Grafts | Regions or captured colonies with their own sensing rules | No cycle on The Wild One; a cycle on one field layout of two |

## Owner's word

- 2026-11-10 16:44Z: "Start another thread to review how this simulation can be turned into a RTS game." (this subject's thread)
- 2026-11-10 17:52Z: "readability can be solved, I think we are still in the early planning stages, so we don't need that yet." See [decisions.md](../decisions.md).
- 2026-11-10 18:15Z: brainstorms must not anchor on the demo's tools: "I want to make sure we are truly brainstorms how this should best work and not doing limited exploration on what exists in our current demo." See [decisions.md](../decisions.md).
- 2026-11-10 18:25Z: on a card rack with no shuffle, "not sure why we don't want luck. Isn't that part of the core fun or card games. Not a decision that needs to be made at this stage anyways though". Left open.
- 2026-11-10 19:24Z: "Your test environment seems small with only 7 food sources. That probably limits the potential options." This led to the field retest.
- 2026-11-10 20:33Z: "You can run these tests, especially if they are cheap and/or can be paralellized. But then lets focus on actually building playable demos". See [decisions.md](../decisions.md).
- 2026-11-10 20:42Z, card: "Both, in two threads" (the field skirmish here, Two Fevers in its own thread).
- 2026-11-10 21:14Z, on the skirmish's first version: "Overall, user input seems limited and doesn't lead to any complex stratgy". 21:17Z, on the budget skirmish: "This is an interesting direction." See [skirmish.md](skirmish.md).

## Tried and rejected

- Taut's pull through a pile's smell (`Params::fed_scent` as a dial), on The Wild One's 7 piles. See the `fed_scent` entry in [`dead-ends.md`](../../Reports/dead-ends.md), which repeats the older `deposit_food` entry; on the field the same dial wins share.
- Sensor settings as counter-patterns on The Wild One. See the `sensor_angle` entry in the same file; on the field a cycle forms on one layout of two.

## Open

- The owner's questions are in [HANDOFF.md](../../HANDOFF.md), decision 7: which direction to probe first; whether the RTS waits for queued work; fog of war drawn from a held quantity; two keepers or three to four.
- Missing from every direction: player count, match length and a food supply that lasts it (The Wild One runs dry at about 2:10-2:20), onboarding, why play twice, and whether the rival is an AI or a second human. No human has played The Wild One yet.
- What about the scatter layout makes the maze–cord link hold, since 9 other layouts lack it.
- Counterplay in the budget skirmish: see [skirmish.md](skirmish.md).
- Taut set per pile, or with a cost, so that it becomes a choice rather than a stat.
- A long match with regrowing food and human play: the rest of the 2026-11-10 19:31Z list. The contagion strain is built ([Two Fevers](two-fevers.md)).

## Sources

- [rts-design-2026-11-10.md](../../Reports/rts-design-2026-11-10.md): the first draft, Bloomfront. Claude Doc: https://claude.ai/artifact/EDygsdTpEg8vK7HZzTXhdp (listed in [`pages.md`](../../Reports/pages.md)).
- [brainstorm/README.md](../../Reports/rts-design-2026-11-10/brainstorm/README.md): the synthesis, which links the six agents' full notes.
- [red-team/README.md](../../Reports/rts-design-2026-11-10/red-team/README.md): the review, with its [harness](../../Reports/rts-design-2026-11-10/red-team/harness/src/main.rs), [raw output](../../Reports/rts-design-2026-11-10/red-team/raw/) and [stills](../../Reports/rts-design-2026-11-10/red-team/stills/).
- [field/README.md](../../Reports/rts-design-2026-11-10/field/README.md): the field retest, the verb census, the layout sweep and the comeback test, with [raw output](../../Reports/rts-design-2026-11-10/field/raw/) and [stills](../../Reports/rts-design-2026-11-10/field/stills/).
- [rts-skirmish-2026-11-10.md](../../Reports/rts-skirmish-2026-11-10.md): the field skirmish, both versions, and the budget skirmish's round robin.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md): outside RTS design research and feedback on this demo (Part 5); page [rts-craft.md](rts-craft.md).
