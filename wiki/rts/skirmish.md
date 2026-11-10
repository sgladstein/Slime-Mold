---
title: The field skirmish (playable demo)
kind: prototype
status: open
updated: 2026-11-10
sources: [Reports/rts-skirmish-2026-11-10.md, Reports/rts-design-2026-11-10/field/README.md]
code: [skirmish::FIELD, skirmish::Habit, skirmish::Plan, skirmish::Rules, skirmish::Skirmish, skirmish::Verb, skirmish::TITHES, skirmish::late_food, Dish::tithe, Dish::beacons, Dish::plant]
---

# The field skirmish (playable demo)

## Now

**Version 2, the budget skirmish, is built and played only by scripts; it
has no page of its own yet.** You and an equal rival mould share the
field's 23 mirrored piles, with more food landing every 15 s from 0:20;
whoever has eaten more at 2:30 wins. Each side sets a tithe (0, 10, 25 or
40% of every bite goes to its bank instead of its mould) and spends it on
beacons (300: a pile its mould alone craves for 20 s), spores (1,000: 150
walkers planted on a pile) and habits (200). The owner called it "an
interesting direction" (21:17Z).

**As built, one strategy beats all the others.** In a round robin of nine
scripted plans, soft (a 25% tithe; beacons, then spores, on the rival's pile
with the fewest of its walkers) won 127 of 128 games; the rest stand on a
ladder with no cycle, doing nothing comes third, and saving for a late
attack loses to everything. The model explains it: each mould holds its
half by 0:10, a pile's food is split by head count, fed walkers win fights
four to one, so a spore on the thinnest pile takes it and starves what lies
behind, and the lead snowballs (300 of 576 games ended in a wipe-out). A
pre-emptive guard, a spore reach of 25 cells and spores that land asleep
for 5 s did not stop soft. **Finding counterplay is the open problem.**

**Version 1** (habits on keys 1-4, two salt walls, the lamp, a rival that
answers your habit) is playable at
https://claude.ai/artifact/NDJJZJuabHpjMjLNBr45Lw (source `web/page/`). The
owner played it and set it aside (21:14Z). `web/src/lib.rs` now runs the new
rules under that page's interface until version 2 has a page.

## Numbers

| Measure | Result | Seeds | Paired | Source |
|---|---|---|---|---|
| Habits on this layout (scatter), 32 games a pairing | hunt > maze 32/32, maze > cord 30/32, cord > hunt 25/32; each > spread 28-31/32 | 16 | Holm over 10 | [field retest, *The pattern grid*](../../Reports/rts-design-2026-11-10/field/README.md) |
| The same cycle on 9 other layouts | holds on none; cord beats maze on 7 of 10 layouts | 16 a layout | Holm over 3 a layout | [field retest, *Layout sweep*](../../Reports/rts-design-2026-11-10/field/README.md) |
| Side behind at 60 s, salt wall across the leader's road (scatter) | turned 6/12 lost games once, 9/12 every 15 s; beat a random wall 16/16 | 16 | clones | [field retest, *Comeback test*](../../Reports/rts-design-2026-11-10/field/README.md) |
| Version 1's rival (8 s delay) against scripted hands | idle won 0/16 (share 0.347-0.460); random 3/16; counter 14/16 (0.427-0.559); counter beat random 14/16, p 0.004 | 16 | by seed | [report, *Version 1*](../../Reports/rts-skirmish-2026-11-10.md) |
| Version 2, free bank (a tenth of food eaten, not taken from the mould), 8 plans | soft won all 7 pairings, 27 of 28 games; random beat idle 4/4 | 2 (4 games a pairing) | none survive Holm | [report, *Round 1*](../../Reports/rts-skirmish-2026-11-10.md) |
| Version 2, tithe and late food, 9 plans | soft beat all 8 others, 127 of 128 games (mean share 0.737); raid second (0.615); idle third (0.548); hoard lost all 8; no cycle among 36 pairings | 8 (16 games a pairing) | Holm over 36: 20 pairings survive, all 8 of soft's | [report, *Round 2*](../../Reports/rts-skirmish-2026-11-10.md) |
| Version 2, the game itself | 300 of 576 games ended before the bell with one mould gone; the side ahead at 1:00 won 416 of 473 | 8 | | [report, *The strategy map*](../../Reports/rts-skirmish-2026-11-10.md) |
| Answers to soft: guard; beacons only; spore reach 25; spores asleep 5 s | none beat soft (4/4 against each defender); a beacons-only plan won 1 of 4 against idle; asleep spores brought idle level with soft, 2/4 | 2 (4 games a pairing) | | [report, *Looking for an answer to soft*](../../Reports/rts-skirmish-2026-11-10.md) |
| Browser frame cost (version 1), 30 s in, loaded 4-core box | draw 9.3 ms, step and draw 14.3 ms a 60th of a second | 1 | | [report, *How it is built*](../../Reports/rts-skirmish-2026-11-10.md) |

## Owner's word

- 2026-11-10 20:33Z: "You can run these tests, especially if they are cheap and/or can be paralellized. But then lets focus on actually building playable demos"
- 2026-11-10 20:42Z, card: "Both, in two threads" (this demo here; Two Fevers in its own thread). See [decisions.md](../decisions.md).
- 2026-11-10 21:14Z, after playing version 1: "This is still using the old tools, but not sure what the strategy is with these tools. The lamp moves the slime mold, and it used to work well for pushing the slime mold to food it wasn't finding on its own. At 22.5 the mold moves and explores well on its own, so I don't know how to use the tool to cause any affect. The salt should block a path but it doesn't have as strong of a reaction as I was expecting. Overall, user input seems limited and doesn't lead to any complex stratgy"
- 2026-11-10 21:17Z, on the budget proposal: "This is an interesting direction. Brainstorm/explore how these tools/game plays out. What different strategies would different players use to win. How complex or interesting are the strategies? Does one strategy beat all the others?" See [decisions.md](../decisions.md).

## Tried and rejected

- Version 1's verbs as an RTS (habits, salt, the lamp): the owner found no strategy in them. See [`dead-ends.md`](../../Reports/dead-ends.md).
- A bank paid from food eaten without taking it from the mould: every purchase is pure gain. Replaced by the tithe.
- Beacons as an attack on their own: a beacons-only plan tied doing nothing.
- Taut's pull as a verb (always best at full on the field), oats and the knife: left out of both versions. See [directions](directions.md).

## Open

- Counterplay: a cheap quick defence against slow, telegraphed spores; a spore price that rises; a plan that answers an attack by hitting the attacker's thinnest pile; or a person finding what the scripts did not (the report's *What would make it deeper*).
- Version 2's own page, so the owner can play it against any of the nine plans.
- A second map needs its own counter table; the habit cycle is real only on this layout.
- Not in the native window (`cargo run --bin mould`), whose HUD calls the rival "wild".

## Sources

- [rts-skirmish-2026-11-10.md](../../Reports/rts-skirmish-2026-11-10.md): both versions, the rival's census, the round robins and the probes. Picture page: *Budget Skirmish Strategies* in [`pages.md`](../../Reports/pages.md).
- [field/README.md](../../Reports/rts-design-2026-11-10/field/README.md): the measurements version 1 rests on.
