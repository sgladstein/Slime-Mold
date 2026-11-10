---
title: The RTS directions (design only)
kind: design
status: open
updated: 2026-11-10
sources: [Reports/two-fevers-2026-11-10/README.md, Reports/rts-design-2026-11-10.md, Reports/rts-design-2026-11-10/brainstorm/README.md, Reports/rts-design-2026-11-10/red-team/README.md, Reports/rts-design-2026-11-10/field/README.md]
code: [Dish::rival, Dish::store, Dish::food_share, Pile::bite, Params::fed_scent, Params::flow, Params::foe_repel, Params::pieces, Params::wound, Params::table]
---

# The RTS directions (design only)

## Now

**Nothing is built.** The owner asked on 2026-11-10 how the mould could
become a real-time strategy game. The first draft, Bloomfront, built a keeper
game on The Wild One's two moulds and the demo's five tools. Six brainstorm
agents then widened it. The two that set the old tools aside proposed a
contagion war over one neutral mould (Tincture, Two Fevers), timed food waves
(Squeeze), tension on the food sources (Taut), and pattern counters (Grain,
Grafts), with a card layer (the Rack) that could sit on any of them.

A red team then probed four of them on The Wild One. Its pull for Taut
collapsed the mould onto its piles, Squeeze from home moved food share by
nothing, and the pattern counters formed a straight ladder. **Two Fevers is
the strongest left**, because its verb rides the model and it scores host
share, but it is wholly unbuilt.

The owner then noted that 7 piles limit the options, so the findings were
rerun on **a field of 23 smaller mirrored piles**, where the moulds reach
about 6,600 walkers, near Jones's density
([field retest](../../Reports/rts-design-2026-11-10/field/README.md)). Two
verdicts turn over. **The counters form a cycle** (hunter > maze > cord >
hunter, every link surviving Holm) on one layout, though not on a second.
**Taut's pull wins share** on every seed, so it is a lever there, but as one
dial for the whole mould it is always best at full, a stat rather than a
choice. With no hands, the side ahead at 60 s keeps the lead in about 4
games of 5. The map matters as much as any verb: the same verb can do
nothing on The Wild One and win on the field.

Two facts constrain every direction. Food share mostly counts heads on piles:
each pile pays at most `Params::table` 40 walkers' worth, split by head count
(`Pile::bite`). And the food store (`Dish::store`) sits at a median of 0,
because with `Params::pieces` on each piece turns any surplus into walkers at
once, so no spending rule works until there is a currency. In mirror games on
The Wild One the likelier failure was a dead draw; on the field every mirror
fights, and the early leader usually holds on.

**The verb census has run** on both dishes (clone at 60 s, one verb in one
copy, 30 s, 16 seeds; the bar is 0.02 of food share the intended way on 13 of
16, seen by 5 s on all 16, and beating a random use). On the field a salt
line across the rival's road passes on share (14 of 16) and against random
(14 of 16), and misses the 5 s check by one seed. On The Wild One only the
lamp moves share (15 of 16 up, 11 by 0.02). Placed food loses on both. **Two Fevers is now a playable demo**
([its page](two-fevers.md)): owned ground with a hold, planner over random
16 of 16 but by only 0.537 of the summed score. Still to test: whether any
verb can overturn a 60 s lead. Bloomfront's Stage 0 is not the first step, because its
gates would pass without testing the opening.

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

## Owner's word

- 2026-11-10 16:44Z: "Start another thread to review how this simulation can be turned into a RTS game." (this subject's thread)
- 2026-11-10 17:52Z: "readability can be solved, I think we are still in the early planning stages, so we don't need that yet." See [decisions.md](../decisions.md).
- 2026-11-10 18:15Z: brainstorms must not anchor on the demo's tools: "I want to make sure we are truly brainstorms how this should best work and not doing limited exploration on what exists in our current demo." See [decisions.md](../decisions.md).
- 2026-11-10 18:25Z: on a card rack with no shuffle, "not sure why we don't want luck. Isn't that part of the core fun or card games. Not a decision that needs to be made at this stage anyways though". Left open.
- 2026-11-10 19:24Z: "Your test environment seems small with only 7 food sources. That probably limits the potential options." This led to the field retest.

## Tried and rejected

- Taut's pull through a pile's smell (`Params::fed_scent` as a dial), on The Wild One's 7 piles. See the `fed_scent` entry in [`dead-ends.md`](../../Reports/dead-ends.md), which repeats the older `deposit_food` entry; on the field the same dial wins share.
- Sensor settings as counter-patterns on The Wild One. See the `sensor_angle` entry in the same file; on the field a cycle forms on one layout of two.

## Open

- The owner's questions are in [HANDOFF.md](../../HANDOFF.md), decision 7: which direction to probe first; whether the RTS waits for queued work; fog of war drawn from a held quantity; two keepers or three to four.
- Missing from every direction: player count, match length and a food supply that lasts it (The Wild One runs dry at about 2:10-2:20), onboarding, why play twice, and whether the rival is an AI or a second human. No human has played The Wild One yet.
- Whether the field cycle survives more layouts, and what about a layout makes the maze–cord link hold.
- Whether any verb can overturn a 60 s lead, since with no hands the leader keeps it in about 4 games of 5.
- Taut set per pile, or with a cost, so that it becomes a choice rather than a stat.
- A long match with regrowing food and human play: the rest of the 2026-11-10 19:31Z list. The contagion strain is built ([Two Fevers](two-fevers.md)).

## Sources

- [rts-design-2026-11-10.md](../../Reports/rts-design-2026-11-10.md): the first draft, Bloomfront. Claude Doc: https://claude.ai/artifact/EDygsdTpEg8vK7HZzTXhdp (listed in [`pages.md`](../../Reports/pages.md)).
- [brainstorm/README.md](../../Reports/rts-design-2026-11-10/brainstorm/README.md): the synthesis, which links the six agents' full notes.
- [red-team/README.md](../../Reports/rts-design-2026-11-10/red-team/README.md): the review, with its [harness](../../Reports/rts-design-2026-11-10/red-team/harness/src/main.rs), [raw output](../../Reports/rts-design-2026-11-10/red-team/raw/) and [stills](../../Reports/rts-design-2026-11-10/red-team/stills/).
- [field/README.md](../../Reports/rts-design-2026-11-10/field/README.md): the field retest and the verb census, with [raw output](../../Reports/rts-design-2026-11-10/field/raw/) and [stills](../../Reports/rts-design-2026-11-10/field/stills/).
