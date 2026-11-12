---
title: The field skirmish (playable demo)
kind: prototype
status: open
updated: 2026-11-11
sources:
  - Reports/rts-design-2026-11-10/field/README.md
  - Reports/rts-research-2026-11-10.md
  - Reports/rts-skirmish-2026-11-10.md
code:
  - Dish::beacons
  - Dish::fortify
  - Dish::plant
  - Dish::tithe
  - Params::fight_fed
  - Params::hold
  - Params::supply
  - Params::upkeep_size
  - Skirmish::points
  - Skirmish::set
  - Skirmish::winner
  - skirmish::FIELD
  - skirmish::Habit
  - skirmish::Plan
  - skirmish::Rules
  - skirmish::Skirmish
  - skirmish::Strike
  - skirmish::StrikeLog
  - skirmish::TITHES
  - skirmish::Verb
  - skirmish::late_count
  - skirmish::late_food
---

# The field skirmish (playable demo)

## Now

**Version 2, the budget skirmish, is playable at
https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS** (source `web/page/`):
you and an equal rival mould share 23 mirrored piles; each side tithes its
bites into a bank and spends it on beacons, spores, rallies, Fortify and
habits; whoever has eaten more at 2:30 wins.

**One strategy still beats the others.** Soft (beacons then spores on the
rival's thinnest pile) won 127 of 128 games, and neither prices (round 3),
model changes (rounds 4-6) nor food after 1:00 (round 7b: the 1:00 leader
still wins 80-85%; desperate fights, `fight_fed` 2 and `fight_unfed` 0.5,
eased it most and are the default) stopped it, because a pile's food is split by head count
and fed walkers win fights, so the lead snowballs. A strike counted down in
view with a Fortify that kills attackers dethrones soft, but brace (strike
and fortify) then wins every pairing; the page opens on those rules. Next:
Fortify's price and the exploiter search against brace
([report](../../Reports/rts-skirmish-2026-11-10.md), rounds 3-7b).

**Version 1** (habits, salt, the lamp) is playable at
https://claude.ai/artifact/NDJJZJuabHpjMjLNBr45Lw; the owner set it aside
(21:14Z).

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
- Food after 1:00 as the snowball's fix (round 7b): a 4:00 match alone adds wipe-outs; regrowth cuts them but the side ahead at 1:00 still wins 80-85%; a held-ground score changes nothing. Regrowth stays in the Rules menu. See [`dead-ends.md`](../../Reports/dead-ends.md).
- Fortify as a lower death rate for defenders alone: held fewer struck piles than no answer. Now it also kills attackers in its ring. See [`dead-ends.md`](../../Reports/dead-ends.md).
- Taut's pull as a verb (always best at full on the field), oats and the knife: left out of both versions. See [directions](directions.md).

## Open

- Brace wins every pairing under Strike and Fortify. Next: Fortify's price (150 against a strike's 1,000+; try 300-600, a smaller ring or `fortify_harm` 2) and the economy sketch's exploiter search (test 4's second half) against brace. Greed has no payoff against a defender who fortifies only when struck; income a strike forfeits might give it one.
- The snowball outlives food after 1:00. Untried: food landing near the trailing mould (a rubber band, which the sketch avoided), size upkeep below 0.2.
- The whole economy sketch at once (Strike and Fortify, 4:00, regrowth, held score) is on the page but was not measured together.
- A person on the page may find answers the scripts did not.
- A second map needs its own counter table; the habit cycle is real only on this layout.
- Not in the native window (`cargo run --bin mould`), whose HUD calls the rival "wild".

## Sources

- [rts-skirmish-2026-11-10.md](../../Reports/rts-skirmish-2026-11-10.md): both versions, the rival's census, the round robins and the probes. Picture page: *Budget Skirmish Strategies* in [`pages.md`](../../Reports/pages.md).
- [field/README.md](../../Reports/rts-design-2026-11-10/field/README.md): the measurements version 1 rests on.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md): outside RTS design research and feedback on this demo (Part 5); page [rts-craft.md](rts-craft.md).
