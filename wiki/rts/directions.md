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

**In the build phase, and the main line is now Larder World.** The owner
asked on 2026-11-10 how the mould could become a real-time strategy game,
and at 20:33Z moved the work to playable demos: **the field skirmish**
([skirmish.md](skirmish.md)) and **Two Fevers** ([two-fevers.md](two-fevers.md)),
a contagion war over one neutral mould, now at version 3 (three plans form a
ladder). A third, **Larder** ([shooter.md](shooter.md)), made the mould the
enemy of a squad of soldiers, and on 2026-11-11 14:06Z the owner moved it
into a persistent world on the GPU big dish with energy crystals and several
mould species, where all Larder work now goes.

The design work behind them: a first draft (Bloomfront), six brainstorm
agents, a red team on The Wild One, and a retest on **a field of 23 small
mirrored piles**, where the moulds reach about 6,600 walkers
([field retest](../../Reports/rts-design-2026-11-10/field/README.md)). On the
field the habits form a cycle (hunt > maze > cord > hunt) on one layout, but
a sweep of 8 more layouts found it on none of them: usually cord simply
wins. With no hands the side ahead at 60 s keeps the lead in about 4 games of
5, and a salt wall across the leader's road is the one old tool that turns
lost games (6 of 12 once, 9 of 12 repeated); the lamp and the cut never do.

The skirmish's first version, built on those numbers (habits, salt, the
lamp), was played by the owner and set aside: "user input seems limited and
doesn't lead to any complex stratgy" (21:14Z). Its second, **the budget
skirmish**, gives the player costed decisions instead: a tithe of its food
into a bank, spent on beacons, spores and habits. Nine scripted strategies
played 576 games, and **one beat all the others**: beacons then spores on the
rival's thinnest pile won 127 of 128 games, and the rest stand on a ladder
with no cycle ([skirmish report](../../Reports/rts-skirmish-2026-11-10.md)).
Neither new prices, nor changes to the model, nor food after 1:00 stopped it;
a strike counted down in view with a Fortify that kills attackers did, but
the plan that strikes and fortifies (brace) then won every pairing. One
strategy still beats the others in every demo so far, and the side ahead
early keeps the lead about 4 games in 5: finding counterplay is the open
problem ([skirmish.md](skirmish.md)).

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

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Food store through The Wild One, sampled every 10 s | median 0, never above 108, against 1,500 at the start | 2 (idle, warden) | | [brainstorm, *Corrections*](../../Reports/rts-design-2026-11-10/brainstorm/README.md) |
| Taut: `fed_scent` 0.15 to 1.0 at 60 s, network length at +5 s | 142-225 to 60-80 cells; walkers within 4 cells of a pile 0.71-0.75 to 0.92-0.98 | 4, 22.5 degrees | clones, 4 of 4 | [red team, §4](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Taut: `fed_scent` 0 ("slack") | the mould leaves its food and is dead (0-63 walkers) by +30 s | 4 | clones, 4 of 4 | [red team, §4](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Squeeze: 10 s of a pile's output as sap, food share at +12 s | home: no change, while 134-181 extra walkers were born; seam pile: +0.001 to +0.004 | 3 | clones, 3 of 3 | [red team, §2](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| `flow` on both moulds, The Wild One, idle | step time 2.6x; frame mean 7.42 to 10.24 ms, worst 24.90 | 2 | | [red team, §2](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Pattern grid, 7 settings, symmetric Wild One, sides swapped | hunter > mesh > runner > cord > maze > bead; no cycle in 164 games | 4 a pairing (8 games) | no 8-0 survives Holm | [red team, §5](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Mirror games, self side's share | cord and bead 0.500 every seed (never touch); mesh 0.48-0.52; maze 0.34-0.66; runner 0.41-0.69 | 4 each | | [red team, §5](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Walker displacement at 60 s, 22.5 degrees | median 2-3 cells after 1 s, 9-11 after 10 s; top tenth 11-20 after 1 s | 2 | | [red team, §3](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Mid-game switch from mesh to cord, share since the switch at +90 s | ahead by +0.003 to +0.037; nothing visible at 5 s | 4 | 4 of 4, p 0.125 | [red team, §5](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Field, 23 piles: walkers at 60 s, two mesh moulds | 6,553-6,661 (The Wild One about 2,000) | 16 | | [field retest](../../Reports/rts-design-2026-11-10/field/README.md) |
| Field pattern grid, 5 settings, sides swapped | scatter: hunter > maze 32/32, maze > cord 30/32, cord > hunter 25/32, a cycle surviving Holm; scatter2: cord v maze 21/32, p 0.11, no cycle; runner loses all 64 games against cord and maze on each layout | 16 a pairing (32 games) | Holm over 10 | [field retest](../../Reports/rts-design-2026-11-10/field/README.md) |
| Field, no hands: side ahead at 60 s still ahead at 150 s | 238/299 (scatter), 322/399 (scatter2); mirrors 48/60, 66/80 | 12 and 16 | | [field retest](../../Reports/rts-design-2026-11-10/field/README.md) |
| Field Taut: one mould's `fed_scent` 0.15 to 1.0 at 60 s, share since at +30 s | 0.653-0.708 against 0.505-0.571 (scatter), 0.683-0.710 against 0.461-0.533 (scatter2); census +0.089 to +0.194 | 4 a layout; 16 | clones, 8 of 8; 16 of 16 | [field retest](../../Reports/rts-design-2026-11-10/field/README.md) |
| Verb census, share since the action at +30 s, plan minus control | field: salt +0.052 median, 15/16 up, 14 by 0.02, beats random 14/16; taut +0.139, 16/16; lamp +0.022, 12/16; snip +0.007; oats -0.004. Wild One: lamp +0.029, 15/16 up, 11 by 0.02, beats random 14/16; salt, snip, oats, molt about 0; taut -0.024 | 16 a dish | clones; Holm over 12: field taut, field salt, Wild One lamp, field molt survive | [field retest](../../Reports/rts-design-2026-11-10/field/README.md) |
| Two moulds stepping, The Wild One | about 1.1 ms a step | 4 | | [first draft, *Technical feasibility*](../../Reports/rts-design-2026-11-10.md) |
| Field layout sweep: the scatter cycle on 9 more layouts | holds on none; hunt > maze on 9 of 10 layouts, cord > hunt on 8, cord > maze on 7; 20 of 24 links survive Holm | 16 a layout (32 games a pairing) | Holm over 3 a layout | [field retest, *Layout sweep*](../../Reports/rts-design-2026-11-10/field/README.md) |
| Comeback: the side behind at 60 s gets one verb | scatter: salt once turns 6/12, every 15 s 9/12, both beat a random wall 16/16; taut 12/12; hunt 10/12 (beats random 9/16); lamp 2-3/12; snip 1/12. scatter2: salt 2/14. Symmetric Wild One: salt once 12/15, every 15 s 13/15 | 16 a dish | clones | [field retest, *Comeback test*](../../Reports/rts-design-2026-11-10/field/README.md) |
| Budget skirmish round robin, 9 plans | soft beat all 8 others, 127 of 128 games; no cycle among 36 pairings; 300 of 576 games ended in a wipe-out | 8 (16 games a pairing) | Holm over 36 | [skirmish report, *Round 2*](../../Reports/rts-skirmish-2026-11-10.md) |

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
