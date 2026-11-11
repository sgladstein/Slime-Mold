---
title: How RTS games are designed (outside research)
kind: design
status: current
updated: 2026-11-11
sources: [Reports/rts-research-2026-11-10.md, research/sources/sirlin-balancing-2014.md, research/sources/johnson-water-finds-a-crack-2011.md, research/sources/browder-sc2-esport-2011.md, research/sources/shelley-design-by-playing-2007.md, research/sources/meier-interesting-decisions-2012.md, research/sources/leblanc-feedback-2009.md, research/sources/czarnecki-spinning-tops-2020.md, research/sources/alphastar-2019.md, research/sources/chung-on-coh-2017.md, research/sources/sahui-endgames-2010.md, research/sources/tooth-and-tail.md, research/sources/creeper-world-2025.md]
---

# How RTS games are designed (outside research)

## Now

A reading of 22 outside sources (designer talks, postmortems, developer
essays, balance research), with lessons for the mould RTS and feedback on
both demos: [`rts-research-2026-11-10.md`](../../Reports/rts-research-2026-11-10.md).
No runs; nothing here is a measurement of our game.

What the best RTS games share: a few decisions that are trade-offs, depend
on the opponent and are made with partial information (Meier; Sirlin's yomi
layers); no dominant strategy, because every strong move has a counter and
strong moves are telegraphed; few pieces with several uses each (Browder);
feedback loops managed on purpose, since an economy snowballs (LeBlanc's
handout; Company of Heroes' retreat and victory points); and a design found
by daily play of an early multiplayer build, with much cut (Shelley,
Wyatt, Meier). Balance work uses tier lists and experts (Sirlin), and
research adds restricted play, exploiters and counting cycles (Jaffe;
AlphaStar; the spinning top).

For us, the report reads soft and Trunk as dominant strategies and the
1:00 leader's wins as unmanaged positive feedback, and says to fix both in
the structure: a cost that grows with size (Civilization IV's fix for its
dominant strategy, in the mould's terms tube length or piles fed), one
pull-back rule, a points ending instead of a wipe-out, attacks that travel
and can be seen coming, then an exploiter search and a person's hands.
These are proposals to test, not results. On 2026-11-11 they became one
design for the skirmish (supply lines, points from held ground, Call,
Fortify and a counted-down Strike), with tests:
[`skirmish-economy-sketch.md`](../../Reports/rts-research-2026-11-10/skirmish-economy-sketch.md).


## Owner's word

- 2026-11-10 23:12Z: "Start a new thread that first just real-time strategy games in general. How they function? The general design philosophy behind them, especially the best ones. How people design and build them."
- 2026-11-11 00:09Z: "What do you think about our resources/economy and options for the player to control/interact with the game"; 00:16Z, on the answer: "These all sound good".
- 2026-11-10 23:12Z: "Then after it does that it reviews our demos and give [feedback] on them".

## Open

- Which of the structural fixes to try first in each demo (the report's Part 5); owner's call or the demo threads'.
- Two cards rest on partial reading: the Jaffe paper (listing only) and the spinning-top paper (abstract only).

## Sources

- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md): Parts 1-3 general, Part 4 lessons, Part 5 demo feedback.
- Cards: [`research/sources/README.md`](../../research/sources/README.md), the RTS rows.
