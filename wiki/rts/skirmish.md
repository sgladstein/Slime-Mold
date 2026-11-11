---
title: The field skirmish (playable demo)
kind: prototype
status: open
updated: 2026-11-11
sources: [Reports/rts-research-2026-11-10.md, Reports/rts-skirmish-2026-11-10.md, Reports/rts-design-2026-11-10/field/README.md]
code: [skirmish::FIELD, skirmish::Habit, skirmish::Plan, skirmish::Rules, skirmish::Skirmish, skirmish::Verb, skirmish::TITHES, skirmish::late_food, Dish::tithe, Dish::beacons, Dish::plant, Skirmish::set, Params::fight_fed, Params::upkeep_size, Params::hold, Params::supply]
---

# The field skirmish (playable demo)

## Now

**Version 2, the budget skirmish, is playable at
https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS** (source `web/page/`),
against any of the scripted plans, under any of five rule sets; it opens on
**desperate fights** (below). You and an equal rival mould share the
field's 23 mirrored piles, with more food landing every 15 s from 0:20;
whoever has eaten more at 2:30 wins. Each side sets a tithe (0, 10, 25 or
40% of every bite goes to its bank instead of its mould) and spends it on
beacons (300: a pile its mould alone craves for 20 s), spores (1,000: 150
walkers planted on a pile), rallies (150) and habits (200). The owner
called it "an interesting direction" (21:17Z).

**One family of plans still beats all the others.** Soft (a 25% tithe;
beacons, then spores, on the rival's pile with the fewest of its walkers)
and keep (soft plus a rally where a rival spore lands) top every round
robin. The model explains it: each mould holds its half by 0:10, a pile's
food is split by head count, fed walkers win fights, so a strike on the
thinnest pile takes it and starves what lies behind, and the lead
snowballs. Neither prices (round 3: a cheap rally, a rising spore price,
spores asleep 5 s abroad) nor changes to the model (rounds 4-6, after the
owner's "then lets explore this", 23:29Z) stopped it. On 448 paired games a
rule set: a holder's bonus at piles made the snowball worse; supply lines
changed nothing; upkeep that grows with size eased it most but let doing
nothing tie the best plans; **desperate fights** (`fight_fed` 2,
`fight_unfed` 0.5: starving walkers fight harder) eased it and left the
tools mattering, so the page opens on them.

**Why no fight rule can fix it alone:** about half the food is eaten by
1:00 and the bell is at 2:30, so even with no snowball the side ahead at
1:00 would win about 70% of the games that reach the bell (it wins 77-89%).
A comeback needs food after 1:00. The next step is the research thread's
[economy sketch](../../Reports/rts-research-2026-11-10/skirmish-economy-sketch.md),
which the owner said "all sound good" (2026-11-11 00:16Z): a longer match
with regrowing piles, a score from held ground, and Fortify against a
counted-down Strike. Its supply-line test has already failed (above). The
sides are even since the piles moved half a cell in (left 14 of 32 idle v
idle, 6 of 16 soft v soft).

**Version 1** (habits on keys 1-4, two salt walls, the lamp, a rival that
answers your habit) is playable at
https://claude.ai/artifact/NDJJZJuabHpjMjLNBr45Lw (copy in
`Reports/rts-skirmish-2026-11-10/page-v1/`; source `web/` at 1b23018). The
owner played it and set it aside (21:14Z).

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
| Round 3, counterplay rules (rally, spore ramp 0.5, spores asleep 5 s abroad), 10 plans | soft lost no pairing, 124 of 144 games; keep 8/16 and trade 7/16 against it; ward (rally only) lost to soft 16/16; one cycle survives, fort > ward > trade > fort; 353 of 720 wipe-outs | 8 (16 games a pairing) | Holm over 45: soft's 6 of 8 | [report, *Round 3*](../../Reports/rts-skirmish-2026-11-10.md) |
| Same seeds and sides, round 2 rules v round 3 rules | soft v idle won 15 then 12 of 16, share lower in 8/16 (p 1); v raid, centre, fort 16 and 16 | 8 | paired by game | [report, *Round 3*](../../Reports/rts-skirmish-2026-11-10.md) |
| Side edge, mirror games (round 3 rules) | left won idle v idle 7/8 (0.524), soft v soft 7/8 (0.612); 14/16, p 0.004 | 8 | | [report, *Which side you play matters*](../../Reports/rts-skirmish-2026-11-10.md) |
| Side edge traced, idle v idle | as built left 24/32 (0.523); taking turns 24/32; your mould started right, left still 19/32; piles half a cell in 16/32 (0.505) | 32 | by seed | [report, *Which side you play matters*](../../Reports/rts-skirmish-2026-11-10.md) |
| Side edge fixed (piles half a cell in) | idle v idle left 14/32 (0.504); soft v soft left 6/16 (0.488) | 32 and 16 | | [report, *Which side you play matters*](../../Reports/rts-skirmish-2026-11-10.md) |
| Round 4 screen, model changes on round 3's rules, 7 plans (old layout) | soft lost a majority for the first time: holder's bonus trade 6/8, even fights keep 6/8, desperate fights trade 5/8, size upkeep keep 5/8; the control none | 4 (8 games a pairing) | none tested | [report, *Rounds 4 to 6*](../../Reports/rts-skirmish-2026-11-10.md) |
| Rounds 5-6, 8 plans, the control and six rule sets (fixed layout) | side ahead at 1:00 won: control 89%, holder's bonus 91%, size upkeep 82%, desperate fights 84%, supply 10/20/40 87-89%; wipe-outs of 448: 235, 271, 100, 198, 228-241; keep lost no pairing except under size upkeep (to idle 6/16) | 8 (16 games a pairing) | Holm over 28 a rule set | [report, *Rounds 4 to 6*](../../Reports/rts-skirmish-2026-11-10.md) |
| The same, game by game against the control | wipe-outs only under it / only under the control: hold 65/29 (p 0.0003), size 2/137 (p 3e-38), desperate 44/81 (p 0.001), supply 33-42/32-40 (p 0.48-0.72); size comebacks 67/36 (p 0.003); soft and keep v idle 15/16 under the control, 6/16 under size | 8 | 448 paired games each; Holm over 24 | [report, *Rounds 4 to 6*](../../Reports/rts-skirmish-2026-11-10.md) |
| Snowball against a null (games to the bell) | side ahead at 1:00 won 77% (size) to 89% (control); shuffling the rest of the game between games, 67-70%; 46-47% of the food eaten by 1:00 (3 traced games) | 8 | | [report, *Rounds 4 to 6*](../../Reports/rts-skirmish-2026-11-10.md) |
| Browser frame cost (version 1), 30 s in, loaded 4-core box | draw 9.3 ms, step and draw 14.3 ms a 60th of a second | 1 | | [report, *How it is built*](../../Reports/rts-skirmish-2026-11-10.md) |
| Browser frame cost (version 2's page), first 30 s, headless Chromium, 4-core box | 8.9-9.4 ms a frame on average, drawing every other frame | 1 | | [report, *Version 2's page*](../../Reports/rts-skirmish-2026-11-10.md) |

## Owner's word

- 2026-11-10 20:33Z: "You can run these tests, especially if they are cheap and/or can be paralellized. But then lets focus on actually building playable demos"
- 2026-11-10 20:42Z, card: "Both, in two threads" (this demo here; Two Fevers in its own thread). See [decisions.md](../decisions.md).
- 2026-11-10 21:14Z, after playing version 1: "This is still using the old tools, but not sure what the strategy is with these tools. The lamp moves the slime mold, and it used to work well for pushing the slime mold to food it wasn't finding on its own. At 22.5 the mold moves and explores well on its own, so I don't know how to use the tool to cause any affect. The salt should block a path but it doesn't have as strong of a reaction as I was expecting. Overall, user input seems limited and doesn't lead to any complex stratgy"
- 2026-11-10 23:29Z, on "The fix probably has to be in the model itself": "then lets explore this".
- 2026-11-10 21:17Z, on the budget proposal: "This is an interesting direction. Brainstorm/explore how these tools/game plays out. What different strategies would different players use to win. How complex or interesting are the strategies? Does one strategy beat all the others?" See [decisions.md](../decisions.md).

## Tried and rejected

- Version 1's verbs as an RTS (habits, salt, the lamp): the owner found no strategy in them. See [`dead-ends.md`](../../Reports/dead-ends.md).
- A bank paid from food eaten without taking it from the mould: every purchase is pure gain. Replaced by the tithe.
- Beacons as an attack on their own: a beacons-only plan tied doing nothing.
- A quick defensive rally, a rising spore price and a trade-blows plan as
  the answer to soft (round 3). See [`dead-ends.md`](../../Reports/dead-ends.md).
- Changes to the model as the answer to soft (rounds 4-6): a holder's bonus at piles (worse), supply lines (no effect), upkeep that grows with size (eases the snowball, but doing nothing ties the best plans). Desperate fights stay, as the page's default. See [`dead-ends.md`](../../Reports/dead-ends.md).
- Taut's pull as a verb (always best at full on the field), oats and the knife: left out of both versions. See [directions](directions.md).

## Open

- The research thread's [economy sketch](../../Reports/rts-research-2026-11-10/skirmish-economy-sketch.md) (the owner, 2026-11-11 00:16Z: "These all sound good"): food after 1:00 (a 4:00 match, regrowing piles) and a score from held ground (its test 3), then Fortify against a counted-down Strike (test 1). Its supply-line test failed in round 6. Fortify must be stronger than "counts as fed", since walkers at a pile already are.
- A person on the page may find answers the scripts did not.
- A second map needs its own counter table; the habit cycle is real only on this layout.
- Not in the native window (`cargo run --bin mould`), whose HUD calls the rival "wild".

## Sources

- [rts-skirmish-2026-11-10.md](../../Reports/rts-skirmish-2026-11-10.md): both versions, the rival's census, the round robins and the probes. Picture page: *Budget Skirmish Strategies* in [`pages.md`](../../Reports/pages.md).
- [field/README.md](../../Reports/rts-design-2026-11-10/field/README.md): the measurements version 1 rests on.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md): outside RTS design research and feedback on this demo (Part 5); page [rts-craft.md](rts-craft.md).
