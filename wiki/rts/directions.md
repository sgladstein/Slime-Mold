---
title: The RTS directions (design only)
kind: design
status: open
updated: 2026-11-10
sources: [Reports/rts-design-2026-11-10.md, Reports/rts-design-2026-11-10/brainstorm/README.md, Reports/rts-design-2026-11-10/red-team/README.md]
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

A red team then probed four of them on today's dish. **Taut fails:** its pull
collapses the mould onto its piles in 2-5 s, and turning it down starves the
mould. **Squeeze from home moves food share by nothing**, and from the seam
by a hair. **The pattern counters form a straight ladder with no cycle** at
today's density, about 5% of cells filled against Jones's 15%, so the result
says nothing about GPU scale. **Two Fevers is the strongest left**, because
its verb rides the model and it scores host share, but it is wholly unbuilt.

Two facts constrain every direction. Food share mostly counts heads on piles:
each pile pays at most `Params::table` 40 walkers' worth, split by head count
(`Pile::bite`). And the food store (`Dish::store`) sits at a median of 0,
because with `Params::pieces` on each piece turns any surplus into walkers at
once, so no spending rule works until there is a currency. In mirror games the
likelier failure is a dead draw rather than a snowball.

**The recommended next step** is a paired-clone verb census, which changes no
game rule for three of the five directions. Clone The Wild One at 60 s, apply
one verb in one copy only, and run both for 30 s on 16 seeds. To pass, food
share has to move 0.02 the intended way on 13 of 16 seeds, with a visible
change by 5 s on all 16. After that comes one contagion strain on the CPU.
Bloomfront's Stage 0 is no longer the recommended first step, because its
gates would pass without testing the opening.

| Direction | Core verb | Red team |
|---|---|---|
| Bloomfront | Lure, push, wall, cut, water, each paid from food | Weak as a direction; its two-keeper plumbing is a useful harness |
| Squeeze | Tap a held pile to send a wave of sap; hold it to charge | Home squeeze 0.000; flow costs 2.6x the step time; needs a fight scaled by load |
| Two Fevers | Seed a strain in a neutral mould; immunity feeds the trailer | Strongest; untested; risk of a rubber band that washes out skill |
| Tincture | The same, but converts by majority | Feeds the leader; a different game from Two Fevers, so pick one |
| Taut | Set each source slack, even or taut | Fails; a recorded dead end |
| Grain, Grafts | Regions or captured colonies with their own sensing rules | No cycle at today's density; untested at scale |

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Food store through The Wild One, sampled every 10 s | median 0, never above 108, against 1,500 at the start | 2 (idle, warden) | | [brainstorm, *Corrections*](../../Reports/rts-design-2026-11-10/brainstorm/README.md) |
| Taut: `fed_scent` 0.15 to 1.0 at 60 s, network length at +5 s | 142-225 to 60-80 cells; walkers within 4 cells of a pile 0.71-0.75 to 0.92-0.98 | 4, 22.5 degrees | clones, 4 of 4 | [red team, §4](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Taut: `fed_scent` 0 ("slack") | the mould leaves its food and is dead (0-63 walkers) by +30 s | 4 | clones, 4 of 4 | [red team, §4](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Squeeze: 10 s of a pile's output as sap, food share at +12 s | home: no change, while 134-181 extra walkers were born; seam pile: +0.001 to +0.004 | 3 | clones, 3 of 3 | [red team, §2](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| `flow` on both moulds, The Wild One, idle | step time 2.6x; frame mean 7.42 to 10.24 ms, worst 24.90 | 2 | | [red team, §2](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Pattern grid, 7 settings, symmetric dish, sides swapped | hunter > mesh > runner > cord > maze > bead; no cycle in 164 games | 4 a pairing (8 games) | no 8-0 survives Holm | [red team, §5](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Mirror games, self side's share | cord and bead 0.500 every seed (never touch); mesh 0.48-0.52; maze 0.34-0.66; runner 0.41-0.69 | 4 each | | [red team, §5](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Walker displacement at 60 s, 22.5 degrees | median 2-3 cells after 1 s, 9-11 after 10 s; top tenth 11-20 after 1 s | 2 | | [red team, §3](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Mid-game switch from mesh to cord, share since the switch at +90 s | ahead by +0.003 to +0.037; nothing visible at 5 s | 4 | 4 of 4, p 0.125 | [red team, §5](../../Reports/rts-design-2026-11-10/red-team/README.md) |
| Two moulds stepping, The Wild One | about 1.1 ms a step | 4 | | [first draft, *Technical feasibility*](../../Reports/rts-design-2026-11-10.md) |

## Owner's word

- 2026-11-10 16:44Z: "Start another thread to review how this simulation can be turned into a RTS game." (this subject's thread)
- 2026-11-10 17:52Z: "readability can be solved, I think we are still in the early planning stages, so we don't need that yet." See [decisions.md](../decisions.md).
- 2026-11-10 18:15Z: brainstorms must not anchor on the demo's tools: "I want to make sure we are truly brainstorms how this should best work and not doing limited exploration on what exists in our current demo." See [decisions.md](../decisions.md).
- 2026-11-10 18:25Z: on a card rack with no shuffle, "not sure why we don't want luck. Isn't that part of the core fun or card games. Not a decision that needs to be made at this stage anyways though". Left open.

## Tried and rejected

- Taut's pull through a pile's smell (`Params::fed_scent` as a dial). See the `fed_scent` entry in [`dead-ends.md`](../../Reports/dead-ends.md), which repeats the older `deposit_food` entry.
- Sensor settings as counter-patterns at today's density. See the `sensor_angle` entry in the same file.

## Open

- The owner's questions are in [HANDOFF.md](../../HANDOFF.md), decision 7: which direction to probe first; whether the RTS waits for queued work; fog of war drawn from a held quantity; two keepers or three to four.
- Missing from every direction: player count, match length and a food supply that lasts it (The Wild One runs dry at about 2:10-2:20), onboarding, why play twice, and whether the rival is an AI or a second human. No human has played The Wild One yet.
- The pattern grid at Jones's density, the only way the counters can still live.

## Sources

- [rts-design-2026-11-10.md](../../Reports/rts-design-2026-11-10.md): the first draft, Bloomfront. Claude Doc: https://claude.ai/artifact/EDygsdTpEg8vK7HZzTXhdp (listed in [`pages.md`](../../Reports/pages.md)).
- [brainstorm/README.md](../../Reports/rts-design-2026-11-10/brainstorm/README.md): the synthesis, which links the six agents' full notes.
- [red-team/README.md](../../Reports/rts-design-2026-11-10/red-team/README.md): the review, with its [harness](../../Reports/rts-design-2026-11-10/red-team/harness/src/main.rs), [raw output](../../Reports/rts-design-2026-11-10/red-team/raw/) and [stills](../../Reports/rts-design-2026-11-10/red-team/stills/).
