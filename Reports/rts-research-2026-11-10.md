# How real-time strategy games work and are made, and what it means for the mould (2026-11-10)

The owner, 23:12Z: "Start a new thread that first just real-time strategy
games in general. How they function? The general design philosophy behind
them, especially the best ones. How people design and build them." Then,
23:12Z: "Then after it does that it reviews our demos and give [feedback] on
them".

This is a reading report: no runs, no new measurements. Parts 1-3 are about
RTS games in general and do not mention the mould. Part 4 draws lessons for
the mould RTS from the model, not from the demo tools. Part 5 is feedback on
the two playable demos, judged by Parts 1-4. Each claim from outside is
cited to a card in [`research/sources/`](../research/sources/README.md);
what is the author's own reading is marked as such. Where a source was read
only second-hand (talk notes, interviews), the card says so.

**Short version.** The best RTS games are built around a few choices that
are trade-offs, that depend on what the opponent is doing, and that the
opponent can see only partly. They keep no single plan dominant by giving
every strong move a counter, and they manage leads on purpose, because a
competitive economy is a positive feedback loop. They are found by play, not
specified: a playable multiplayer build early, daily play, and a lot cut.
Both of our demos have the same two faults the literature warns about most:
a dominant strategy (soft; Trunk) and a lead that snowballs, with nothing
pulling the other way. The research says to fix those in the structure of
the game (a cost that grows with size, a scoring rule that does not end in a
mop-up, attacks that are telegraphed and answerable) before adding verbs.

---

## Part 1. How RTS games function

### The loop

An RTS is a contest on a shared map in continuous time: gather resources,
spend them on production and technology, and use the army to take more of
the map's resources from the opponent. Every common element serves that
loop.

- **Economy.** Most games gather in batches (workers carry loads to a base).
  Total Annihilation streamed it instead: metal and energy "are accumulated
  at a constant rate rather than in small batches", resources never run
  out, and spending above income slows every build at once ("nanostalling")
  ([total-annihilation](../research/sources/total-annihilation.md)). Company
  of Heroes ties income to territory: each sector has a point, and it pays
  only while it is connected back to the headquarters, so cutting a supply
  line is an economic attack ([chung-on-coh-2017](../research/sources/chung-on-coh-2017.md)).
- **Production and technology.** Spending turns into units, upgrades and new
  options. Each purchase is a bet on what the opponent is doing.
- **Map and territory.** Expansions, control points and terrain (high
  ground, cover, choke points) make position worth something. In Company of
  Heroes directional cover makes flanking the centre of combat, and craters,
  rubble and broken bridges change the map as the match goes
  ([coh-making-of-2016](../research/sources/coh-making-of-2016.md),
  [chung-on-coh-2017](../research/sources/chung-on-coh-2017.md)).

### Macro and micro

Players command at more than one scale. Bycer names three layers: micro
(orders to one unit or a small group), wide (one screen's battle) and macro
(the whole match: economy, production, research). "The challenge when it
comes to RTS games is balancing control between the micro and macro layers"
([bycer-three-layers-2012](../research/sources/bycer-three-layers-2012.md)).
Browder, on StarCraft II: "Micro-management is important: that's the R of
RTS" ([browder-sc2-esport-2011](../research/sources/browder-sc2-esport-2011.md)).
Andy Schatz took the other road in Tooth and Tail: the camera is tied to one
commander, so "attention is a resource", and "anything that felt like a
chore in your classic RTS" was cut ([tooth-and-tail](../research/sources/tooth-and-tail.md)).

### Information

Fog of war hides what the opponent is doing; Warcraft borrowed it from the
older strategy game Empire ([wyatt-warcraft-multiplayer-2012](../research/sources/wyatt-warcraft-multiplayer-2012.md)).
Scouting is paid for, and so the core choices become what Sirlin calls
double-blind: each side commits without fully knowing the other's choice
([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)).
Tooth and Tail makes every production building of a tier look the same:
"we give you enough information to make interesting decisions, but not
enough information to totally give away what the opponent is doing"
([tooth-and-tail](../research/sources/tooth-and-tail.md)).

### Tempo, timing and counters

Because production takes time, a choice is a commitment: an army takes a
minute to build and is wrong if the opponent built its counter. Sirlin's
model of this is the **yomi layer**: layer 0 is doing your move, layer 1
countering the expected move, layer 2 countering the counter, layer 3
countering that; past layer 3 the first move comes back, so the designer
need not build further. "The Yomi Layer concept is a reminder that moves
need to have counters", and RTS games "have the same property, but on a
much slower time-scale" ([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)).

Counters live at two levels. **Units** may be locally unbalanced (Sirlin:
two Zealots beat or lose to eight Zerglings, and that is fine); **starting
options** (races, factions) must be fair, each with "a reasonable chance of
winning a tournament in the hands of the right player"
([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)). A
counter can also be timed: in StarCraft II Banelings beat Marines, Stim
lets Marines beat Banelings, and a Baneling upgrade turns it back, a cycle
that moves as the match goes on ([browder-sc2-esport-2011](../research/sources/browder-sc2-esport-2011.md)).
How strong a counter is is itself a dial: Age of Empires' team talked of
making scissors five times stronger than paper for hardcore players and only
twice for casual ones ([aoe-retrospective-2007](../research/sources/aoe-retrospective-2007.md)).

### Ending a match

Elimination games end in a slow mop-up after the result is clear. Sahui
reviews victory conditions built against that: Company of Heroes' victory
points (an odd number of points; whoever's ticket count reaches zero first
loses, so stalling costs), and Rise of Nations' wonder countdown, which can
be stopped by building or razing wonders. They "automatically [rule] out the
possibility of a grind, a slog, or a stalemate" ([sahui-endgames-2010](../research/sources/sahui-endgames-2010.md)).
Sirlin: a checkmate "can be ok", a lame-duck ending (the game is lost but
goes on) should be avoided ([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)).

### How the engine works

Most RTS games run **deterministic lockstep**: every machine runs the same
simulation and only player commands are sent. Age of Empires did this
because sending unit state capped the game at about 250 moving units; its
team checksummed the world, pathfinding and targeting to catch the
out-of-sync bugs, which they called the hardest part ([bettner-terrano-1500-archers-2001](../research/sources/bettner-terrano-1500-archers-2001.md)).
At Gas Powered Games the simulation ran at a fixed 10 ticks a second, the
renderer up to 60, and replays and rejoins re-run the command stream: "The
input data is tiny" ([smith-sync-harder-2011](../research/sources/smith-sync-harder-2011.md)).
Physics can be part of the game: Total Annihilation's projectiles,
explosions and wreckage are simulated, and hills block artillery
([total-annihilation](../research/sources/total-annihilation.md)). Warcraft's
first networked game hit a desync on day one (from the article's summary,
not checked verbatim; [wyatt-warcraft-multiplayer-2012](../research/sources/wyatt-warcraft-multiplayer-2012.md)).

---

## Part 2. The design philosophy behind the best ones

1. **A game is a series of interesting decisions.** Sid Meier (GDC 2012): an
   interesting decision is a trade-off, it is situational ("Good decisions
   are situational"), it persists (early choices shape later play), and the
   player has enough information to make it. "It's easier to look at it as
   what is not an interesting decision"
   ([meier-interesting-decisions-2012](../research/sources/meier-interesting-decisions-2012.md)).
   Bruce Shelley's "inverted pyramid": start with one unit and one decision,
   and let each decision open more ([shelley-inverted-pyramid-2020](../research/sources/shelley-inverted-pyramid-2020.md)).

2. **No dominant strategy.** "The worst thing you can have in a competitive
   multiplayer game is a dominant move", and "If an expert player can
   consistently beat other experts by just doing one move or one tactic, we
   have to call that game imbalanced" ([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)).
   Soren Johnson: "a single, dominant strategy actually takes away choice
   from a game because all other options are provably sub-optimal", and
   players will find it: "water finds a crack". His example is the fix, not
   just the fault: Civilization's "infinite city sleaze" (build as many small
   cities as possible) was ended in Civilization IV by a per-city upkeep that
   grows with the number of cities, a change to the structure, not a price
   tweak ([johnson-water-finds-a-crack-2011](../research/sources/johnson-water-finds-a-crack-2011.md)).

3. **Few pieces, each with several uses.** Browder: StarCraft II was held to
   45 units and 3 sides when Dawn of War had over 60 units and Supreme
   Commander over 100, so each unit could do more: the Colossus walks up
   cliffs to reach a base from behind; Force Field protects, splits an army,
   or funnels the enemy ([browder-sc2-esport-2011](../research/sources/browder-sc2-esport-2011.md)).
   Roles should overlap as little as possible: "if you introduce a unit and
   another unit is no longer used by players then your new unit overlapped
   the old unit too closely" ([doll-unit-roles-2018](../research/sources/doll-unit-roles-2018.md)).

4. **Feedback loops are designed, not left to chance.** A system "that gives
   the winning player an advantage will drive the players further apart"; one
   that helps the trailing player drives them closer. A resource that runs
   out "can create the sense of forward progress", and its rate predicts the
   length of the game (GDC 2009 game design workshop handout, the source of
   the positive / negative feedback vocabulary; [leblanc-feedback-2009](../research/sources/leblanc-feedback-2009.md)).
   An RTS economy is positive feedback by nature (more territory, more
   income, more army, more territory), so the best games add something that
   pulls back: Civilization's upkeep (above), Company of Heroes' retreat,
   which sends a beaten squad home alive so a lost fight is not a lost army
   ([chung-on-coh-2017](../research/sources/chung-on-coh-2017.md)), and
   victory points that tick so the leader must keep holding
   ([sahui-endgames-2010](../research/sources/sahui-endgames-2010.md)).

5. **Strong moves are telegraphed.** Sirlin cites Rob Pardo that RTS super
   weapons are generally a bad idea, and notes that StarCraft's nuke has
   built-in weaknesses: a Ghost must stand near, a red dot shows the target,
   and there is a countdown ([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)).
   The counter is built into the move.

6. **Readable at a glance.** Browder kept large units smaller than
   buildings so they could not hide a rush
   ([browder-sc2-esport-2011](../research/sources/browder-sc2-esport-2011.md)).
   Tooth and Tail chose animals so that "the player generally already
   understands the function of a unit simply by knowing the species"
   ([tooth-and-tail](../research/sources/tooth-and-tail.md)); Doll: visual
   clarity matters for experts' micro and for beginners learning roles
   ([doll-unit-roles-2018](../research/sources/doll-unit-roles-2018.md)).

7. **The world is a player.** Company of Heroes is "rules-light and
   possibilities-heavy": cover, craters, broken walls and supply lines make
   the map change with every fight ([chung-on-coh-2017](../research/sources/chung-on-coh-2017.md)).
   Creeper World made the enemy a fluid: its creator "wanted to simplify
   strategy gaming and also explore a new type of RTS enemy... based on a
   fluid simulation", later running it over terrain "controlled by a falling
   sand simulator", with maps that have "more than one possible path to
   victory" and safe areas to regroup ([creeper-world-2025](../research/sources/creeper-world-2025.md)).
   Creeper World is the closest published game to ours: a continuous
   simulated substance, not units, is the thing the player fights over.

8. **Cut what is a chore, and cut a lot.** Meier: "probably a third of the
   things that we try... end up getting taken out of the game"
   ([meier-interesting-decisions-2012](../research/sources/meier-interesting-decisions-2012.md)).
   Age of Empires had "a five-page list of things we wanted in Age 1, and we
   realized only half a page" ([aoe-retrospective-2007](../research/sources/aoe-retrospective-2007.md)).
   Schatz on fixed build orders: "Build orders get memorized and the game
   boils down to the player who can execute more efficiently", which is why
   Tooth and Tail's maps are procedural ([tooth-and-tail](../research/sources/tooth-and-tail.md)).

9. **A strategy space is a spinning top.** Czarnecki and others (DeepMind,
   2020) find that real games, StarCraft II among them, share one shape: an
   axis of plain strength, and at each strength some number of cycles
   (rock-paper-scissors loops). The cycles are widest at middling strength
   and narrow towards the top. The paper's point for training is that it
   "clarifies why populations of strategies are necessary"
   ([czarnecki-spinning-tops-2020](../research/sources/czarnecki-spinning-tops-2020.md)).
   AlphaStar's League kept exploiter agents whose only job was to find the
   main agents' weaknesses, because "playing to win is insufficient" and
   plain self-play can "forget" how to beat old strategies and chase its
   tail round a cycle ([alphastar-2019](../research/sources/alphastar-2019.md)).
   A design reading (mine): weak strategies sit on a ladder, cycles appear
   among competent ones, and a game whose top is one point has a dominant
   strategy.

---

## Part 3. How people design and build them

**Prototype early, and play it.** Shelley's talk "Designing by Playing":
plan, prototype quickly, playtest (a year or more), polish, publish.
"Designers are only guessing until they play", "You must play to judge a
game", and "A multiplayer prototype existing at the six month mark is the
goal". Ensemble ran regular, semi-compulsory multiplayer sessions ("Angry
Tuesdays") and asked testers to "play to test, not to win"
([shelley-design-by-playing-2007](../research/sources/shelley-design-by-playing-2007.md)).
Warcraft had no spec: "we instead experimented and culled the things that
didn't work"; its first multiplayer game ran about ten months in
([wyatt-warcraft-multiplayer-2012](../research/sources/wyatt-warcraft-multiplayer-2012.md)).
Relic prototyped Company of Heroes for nine to ten months inside an older
engine (Impossible Creatures); almost none of that tuning carried to the
final engine, and the fixed territory sectors that define the game arrived
"like nine months from the end" ([coh-making-of-2016](../research/sources/coh-making-of-2016.md)).
The lesson: the prototype finds the game; its numbers do not survive.

**Balance by tiers, with experts, over time.** Sirlin's process: playtest
into a tier list, remove the "god tier" first, then lift the garbage tier,
then compress the tiers; make a new move too strong at first so testers use
it (Pardo's advice); avoid special-case fixes for one matchup; "Ignore their
first reactions to nerfs, let them play it more"; and "all the theory in the
world does not substitute for playtesting"
([sirlin-balancing-2014](../research/sources/sirlin-balancing-2014.md)).
Johnson: "Often, designers don't even understand their own games until they
finally see them in the wild" ([johnson-water-finds-a-crack-2011](../research/sources/johnson-water-finds-a-crack-2011.md)).

**One build for two audiences.** "Every one of our design decisions had to
work for both casual and hardcore audiences" (Age of Empires; the
counter-strength dial in Part 1 is how) ([aoe-retrospective-2007](../research/sources/aoe-retrospective-2007.md)).

**Balance tools from research.** Three ideas fit a scripted census:

- **Restricted play** (Jaffe and others, AIIDE 2012): measure balance by how
  well an agent limited in some way (only one unit type, never attacking,
  and so on) does against an unlimited one. A big win for a restricted agent
  marks a degenerate option. (Card: listing only; the paper's site refused
  automated reading, so this summary is from the title and memory, not
  checked; [jaffe-restricted-play-2012](../research/sources/jaffe-restricted-play-2012.md).)
- **Exploiters** (AlphaStar): keep a population, and train some members only
  to beat the current best ([alphastar-2019](../research/sources/alphastar-2019.md)).
- **Measure the cycles, not only the winner** (the spinning top): Nash
  clustering sorts strategies into strength layers and counts the cycles in
  each ([czarnecki-spinning-tops-2020](../research/sources/czarnecki-spinning-tops-2020.md)).

**Build the engine for determinism.** Lockstep, a fixed simulation tick
apart from the frame rate, seeded random numbers, world checksums, and
replays as re-run inputs (Part 1). Smith: "you could just fix the desyncs"
rather than build recovery ([smith-sync-harder-2011](../research/sources/smith-sync-harder-2011.md)).
Test on the target network early ([bettner-terrano-1500-archers-2001](../research/sources/bettner-terrano-1500-archers-2001.md)).

---

## Part 4. Lessons for the mould RTS

From here on it is my reading, applying Parts 1-3 to the model. The model,
neutrally: thousands of walkers follow and lay trail, so the body builds
tubes between food; traffic thickens a tube and neglect drops it; the body
re-solves when food changes; a piece cut from food starves; two bodies meet
and fight, and the fed side wins (`Params::fight`); a pile's food is split
by head count (`Pile::bite`). Facts from the reports are linked.

1. **Our results are the textbook failure, and the textbook says where to
   fix it.** Soft winning 127 of 128 ([skirmish report, *Round 2*](rts-skirmish-2026-11-10.md))
   and Trunk topping a ladder ([Two Fevers, *Version 3*](two-fevers-2026-11-10/README.md))
   are dominant strategies; water found the crack (the head-count split and
   the fed fight bonus). Round 3 tried local fixes (a rally, a rising spore
   price) and they did not move soft. Johnson's lesson is that the fix that
   works changes the structure: a cost that grows with size. For the mould,
   size has a natural cost in the model: tube length. A body spread over
   many piles has long thin supply tubes; making upkeep scale with the
   length or the number of piles fed (or making fed status decay along a
   tube's length) is Civilization IV's fix in the mould's own terms. This is
   the skirmish report's item 2, *Make size cost something*, and the
   research says try it before more verbs.

2. **Add one negative feedback loop on purpose, and make it visible.** Both
   demos stack positive loops (more piles, more fed walkers, more fights won,
   more piles) with nothing pulling back; the leader at 1:00 wins 416 of
   473 skirmish games and keeps the lead in 26-32 of 32 Two Fevers games.
   Candidates from the model: supply-line cost (above); starving walkers
   that fight harder (desperation, the skirmish report's item 1); a retreat
   in the Company of Heroes sense, where a mould pushed off a pile withdraws
   along its tube alive instead of starving there; food that regrows faster
   where nobody has eaten lately, so the trailer has somewhere to go. Each
   is a rule to sweep, judged by the leader-keeps-lead count against
   today's.

3. **End on points, not on a wipe-out.** 300 of 576 skirmish games ended
   with one mould gone, a lame-duck ending by Sirlin's measure. Victory
   points that tick while held (Company of Heroes) make the leader keep
   defending and let the trailer win by holding, not by killing. Two Fevers
   averages ownership over 3:00, which also rewards the early lead; ticking
   points from contested ground only would weight the middle of the match.

4. **The mould's slowness is the yomi engine.** A network takes ten seconds
   or more to grow and minutes to re-solve, so a choice of shape is a
   commitment, which is what yomi layers need. What the demos lack is the
   other half: partial information and an answer. Both show everything at
   once, and the strong attack (a spore, a seed) lands instantly. The model
   can supply both: fog from your own walkers' reach (the open HANDOFF
   question of fog drawn from a held quantity), and attacks that travel
   through the body and can be seen coming, like StarCraft's nuke. A spore
   carried along your own tube to the front is visible, slow and cuttable;
   one dropped anywhere is not. (The skirmish's asleep spores were tried and
   weakened soft without making it answerable: the answer has to be a
   different act, not a wait.)

5. **Fewer verbs, each with several meanings.** Browder's Force Field
   protects, splits or funnels depending on where it lands. The mould's verbs
   should do the same through the model: one act that is income at home, a
   bridge at the seam and a trap in the rival's ground. Two Fevers v3's call
   is the right kind (it sets income and the network's shape and its
   exposure at once); the skirmish's beacon is a weaker spore, and habits
   barely move share, which is Doll's overlap test failing.

6. **The world is the mould, so the map matters.** Company of Heroes pays
   territory only when it is connected; the mould already starves a piece
   cut from food (`Params::pieces`), so supply lines are native to the model
   and could be the economy, not a side effect. Creeper World shows a game
   can be built on a continuous substance, and that terrain that also moves
   (its falling sand) makes it a strategy game rather than a race. Our habit
   cycle held on 1 layout of 10 ([field retest, *Layout sweep*](rts-design-2026-11-10/field/README.md)),
   so counters here depend on the map: either design maps for the cycle or
   make the map a choice (Tooth and Tail's procedural maps break fixed
   build orders).

7. **Expect a ladder from scripts, and look for cycles among good plans.**
   Our one-line plans are low on the spinning top, where a ladder is
   normal; a cycle among them would be the surprise. Two method changes
   follow. For each rule set, search for a counter to the current best (an
   exploiter: sweep a plan's parameters to maximise its score against soft
   or Trunk) rather than only playing a fixed roster. And quote restricted
   play: how much does "spores only" or "never spend" win? Our idle and
   random controls are already restricted agents.

8. **Fix fairness first.** The left mould wins 14 of 16 mirror games
   ([skirmish report, *Which side you play matters*](rts-skirmish-2026-11-10.md)).
   Sirlin's rule is that starting options must be fair; until that is
   fixed, a person's results on the page mix skill with side.

9. **Play every day, and cut.** Shelley wants a multiplayer prototype at six
   months and a year of play; we have a playable demo each evening, which is
   the right speed. The owner's two playtests (21:14Z, 21:52Z) did more to
   steer the design than any census. A two-human version (hot seat or two
   browsers) is the cheapest way to find yomi, since scripts do not read.

10. **Let the mould do the micro.** The owner's finding that the mould
    "explores well on its own" is the Tooth and Tail choice made by the
    model: the player is the commander, the walkers are the army. That frees
    the design to put all its depth in macro (where, when, what shape), and
    it means a verb that only does what the mould does anyway is a chore to
    cut.

---

## Part 5. Feedback on the two demos

Judged by Parts 1-4. I read each demo's page text and report, not played.

### The Budget Skirmish (https://claude.ai/artifact/936ACBn8mJUkvFpb4iSnUS)

**What works.** The tithe is a real Meier decision: money now against body
now, priced by where you stand (cheap where you are alone, dear at the seam).
The census with idle and random controls is restricted play already. The
page shows its numbers and its losing cases honestly.

**What the research says is wrong, most important first.**

1. **A dominant strategy (soft), held up by structure, not price.** Round 3's
   rally and rising price are the local fixes Johnson warns about. Try a
   structural cost: upkeep that grows with the number of piles fed or the
   length of tube that feeds them (Part 4, lesson 1).
2. **Three positive loops and no negative one.** Head-count share, the fed
   fight bonus and starvation all reward the leader; 416 of 473 games go to
   the side ahead at 1:00. Add one pull-back rule and measure it by that count
   (lesson 2).
3. **A lame-duck ending.** Half the games end in a wipe-out, and the food
   runs out at about 1:50 of a 2:30 match, so the late game cannot turn.
   Score by held points ticking over a longer match with regrowing food
   (lesson 3), so a midgame exists for counters to live in.
4. **No information game.** Both moulds, every pile and every beacon are
   in plain view, and a spore lands at once anywhere, so there is nothing to scout and nothing to
   read. Telegraph the strong move and hide some of the rest (lesson 4).
5. **Verbs that overlap.** The beacon is a weaker spore and habits barely
   matter. Fewer, multi-use verbs (lesson 5).
6. **The side edge** (left wins 14 of 16 mirrors) must go before anyone's
   play on the page is read as a strategy result (lesson 8).

### Two Fevers v3 (https://claude.ai/artifact/UMzPJmevoYJYmx8KRqG37a)

**What works.** This is the closer of the two to what the research
describes. It started from a trade-off in the model (a trunk feeds you best
and carries the enemy fastest), its call has several meanings at once
(income, network shape, exposure), its pinch is a sacrifice with a visible
cost, and its rules are on the screen. It answers the owner's 21:52Z
complaint that the mould was background.

**What the research says is wrong.**

1. **The designed counter does not bite.** The cycle needed Raider to beat
   Trunk, but the fever is not faster on thick tube (5.3 against 5.1
   cells/s), and resistance (added to stop the first seed taking all) means
   a seed into the trunk lands on the best-fed walkers. So the one
   weakness meant to balance Trunk is missing. Make the trunk's exposure
   real in the model: for example, sick walkers lay no trail, so a fever
   riding a trunk thins it and starves what lies behind, and the raid's
   reward is the starving part the report says no script yet targets.
2. **Income is the game, and income snowballs.** Ground pays spores, so the
   1:30 leader keeps the lead in 26-32 of 32 games in every matchup.
   Income that falls as a body grows, or a cap on spores from ground (the
   report's own levers), is the negative loop (lesson 2).
3. **The score averages the whole match,** which counts the early lead
   most. Ticking points from contested ground, or weighting the last minute,
   would let a comeback count (lesson 3).
4. **Everything is visible.** Good for learning; for yomi, consider showing
   the rival's calls only where your walkers can sense them. The tube a call
   grows is a natural, slow telegraph and should stay visible.
5. **No one has played v3.** The ladder (Trunk over Web over Raider) is a
   ladder among scripts; Shelley's rule says the owner's hands are the test
   that counts.

### Both demos

The two threads found the same thing from opposite sides: income decides,
and the lead snowballs. The research's strongest advice is the same for
both: fix the feedback structure in the model (one size cost, one pull-back
rule, a points ending) before adding verbs, test each rule against the
leader-keeps-lead count and an exploiter searched against the current best,
and then put it in a person's hands. If one demo shows a cycle among good
plans after that, the other can borrow its economy, as the v3 design
already suggests.

## 2026-11-11: the economy and the player's actions, as a design

The owner, 00:09Z: "What do you think about our resources/economy and
options for the player to control/interact with the game". The answer gave
each part of the mould its own job (food as income, walkers as army and
workers, tube as infrastructure whose length costs, score from held ground)
and proposed three actions (Call, Fortify, a telegraphed Strike). The owner,
00:16Z: "These all sound good". The design and its tests, for the skirmish
first: [`skirmish-economy-sketch.md`](rts-research-2026-11-10/skirmish-economy-sketch.md).
Nothing is built or run yet.

## Sources

The cards, one per source, are listed in
[`research/sources/README.md`](../research/sources/README.md). Every source
was read on 2026-11-10 by a fetch of the page; the cards say how fully. Not
reached: the AIIDE page for Jaffe and others (blocked automated reading);
the GDC Vault videos themselves (the talks are cited through their press
and blog write-ups).
