# Explorer D: card-first (2026-11-07)

I started from what makes a card or spell-building *run* good, and only then looked for the flow, gas or growth rule each idea needs. Five concepts follow. Each one adds a section on **the run**: what happens between levels, and why level ten feels different from level one.

## What makes a whole run compelling: eight principles

1. **Power multiplies, and the target grows faster.** Balatro scores chips times mult, and the score needed climbs from 300 at the first ante to 20,000 by the sixth and into the millions after that. A run feels like a run because the late numbers get absurd. *Sim need:* an effect that feeds itself, such as a flame front that runs through all connected gas, or a network that draws more lightning the bigger it gets. A wider brush only adds, and adding makes a run feel flat. ([Giant Bomb, Balatro](https://www.giantbomb.com/balatro/3030-90552/); [PC Gamer interview](https://www.pcgamer.com/i-dont-play-poker-at-all-says-solo-developer-who-made-the-poker-roguelike-i-cant-stop-playing/))
2. **Synergy is about where, not only what.** In Luck be a Landlord, many symbols pay only when they land next to a partner. Monster Train's three floors make placement the decision. *Sim need:* a field with slopes, plus a flow that carries what one card makes to where another card landed. The map becomes the grid that decides which cards sit "next to" each other. ([Rogueliker review](https://rogueliker.com/luck-be-a-landlord-review/))
3. **Cutting a card is as strong as adding one.** Slay the Spire sells card removal in its shops, and Luck be a Landlord hands out removal tokens. A six-card deck that always deals its combo beats a twelve-card deck that sometimes does. *Sim need:* few wand slots, dealt in order, so one weak card in the loop visibly breaks the combo. ([Twinfinite, removal](https://twinfinite.net/guides/how-remove-symbols-luck-be-a-landlord/))
4. **You build from what the run hands you: pick one of three, or skip.** That is Slay the Spire's card reward; Noita's version is random wands. The designers tracked which cards were picked and which won ([GDC 2019 talk](https://gdconf.com/news/learn-slay-spires-successful-metrics-driven-approach-game-balancing-gdc-2019)). *Sim need:* generated levels, so a card that is great on one map is useless on the next. Without that, the best deck is solved once and stays solved.
5. **Upgrades bend a card, and they can cost you.** Inscryption's campfire makes a card stronger but may lose it, and its altar moves a card's ability onto another card. A broken build feels earned when you took a risk to get it. *Sim need:* card numbers that map onto a knob you can see in the world (spread rate, branching, how fast something fades), so an upgrade changes a shape on screen and not a tooltip. ([Game Developer, Inscryption](https://www.gamedeveloper.com/design/how-game-jam-sacrifices-became-inscryption))
6. **The threat is a clock you can read, and bosses change one rule.** Luck be a Landlord's rent keeps going up. Balatro's boss blinds each break one rule, and Slay the Spire's ascension levels stack. Doing nothing loses because the bill keeps rising. *Sim need:* an opposing flow or growth that the world itself produces, and that gets bigger every level. Creeper World's spreading fluid is the model. ([Indie Hell Zone, rent](https://indiehellzone.com/2021/03/01/luck-be-a-landlord))
7. **Random in, readable out.** In Dicey Dungeons the enemies do the same thing every time, and only the dice are random. Shiny Shoe kept movement effects off one Monster Train 2 boss because nobody could reason about the result. *Sim need:* the next cast shown on the map as a ghost with one plain sentence, and a simulation that is fixed for a given seed. ([Game Developer, Dicey Dungeons](https://www.gamedeveloper.com/design/witch-craft-how-i-dicey-dungeons-i-balances-chance-and-predictability); [PC Gamer, Monster Train 2](https://www.pcgamer.com/games/card-games/monster-train-2-echoes-from-the-void-interview/))
8. **A few words, many sentences.** Magicka has eight elements, queues of up to five, and water plus fire makes steam. In Noita, the world does half the combining. *Sim need:* reactions between materials and fields that nobody wrote as pairs. ([StrategyWiki, Magicka elements](https://strategywiki.org/wiki/Magicka/Elements))

**Why players start another run:** (1) they once saw it go absurd and want that again, (4) the offers will be different, and (6) there is a harder rung waiting.

## The wand all five concepts share

The wand starts with 4 slots and can grow to 8. Cards are dealt left to right, one cast per press, and the deck reloads when it runs out.

- **The next cast is always drawn on the map as a ghost**, with one sentence in world words, for example *"Next: a spark that waits until the gas is thick, then lights it."* This answers the satchel playtest, where the player could not tell what "NEXT WAKE+FIRE R4" meant.
- **Modifiers** change every effect in the shot they are dealt into, then draw another card.
- **Multicasts** ("Two at once") draw several cards that go off together.
- **Triggers** ("On strike: ...") carry the next card until a *world event* fires them.
- **Anything a trigger fires is one level deeper.** At depth 3 the chain stops with a visible grey puff.
- **You edit the wand only between levels.**

---

## 1. ARC: the lightning farm

**Pitch:** a storm is building over your town. Grow copper so the lightning goes where you want it, then spend what it gives you.

**30 seconds:** the cloud darkens as it charges, and a faint glow shows where the charge is thickest. You cast **Wire**: a copper tip grows from your rod, branching as it goes. A violet leader creeps down from the cloud toward the tallest conductor below it. It touches your copper, the bolt flashes, and current runs down the network. Lamps light up and score. Thin branches that carry too much melt, spitting sparks. If you miss, the bolt hits the town spire and a roof catches fire.

**Simulation:**
- **Charge in the air** is a scalar field that spreads and fades. This is the shipped field machinery behind pheromone. The storm feeds it.
- **The leader** is new: a growing tip that steps toward the neighbour cell where the charge drops fastest, and branches at random, weighted by that drop. This is how real lightning is modelled, and it can reuse the plant code's tip-growth and branching mechanics in the abstract. Copper is held at ground, so the field gets steep around it, and that is how the leader finds it.
- **Current** is new: a walk along the copper from the strike point to ground that splits at forks.
- **Shipped:** fire for overloads, explosions for the capacitor, storms in the weather. Today's lightning (`weather::strike`) is drawing only: it puts a bolt at an x and touches nothing in the world.

**Cards:** Wire (grow copper 20 cells toward the cursor), Rod (a tall spike that lightning prefers), Ground (soaks up a strike safely), Lamp (scores when current passes through it), Capacitor (stores a strike, then lets you fire it as an aimed bolt), Glass (neither lightning nor current can cross it), **Fork** (modifier: every wire in this shot branches twice), **Thick** (modifier: carries double before it melts), **Two at once** (multicast), **On strike** (trigger: fires the next card where the lightning lands).

**Combo: On strike, then Fork, then Wire.** (1) A bolt hits your rod. (2) The trigger fires a forked wire out of the strike point. (3) The new copper is grounded, so the charge field steepens around its tips. (4) The next leader goes for those tips, because they are now the tallest conductors. (5) It strikes, the trigger fires again, and more wire grows. The network climbs toward the cloud by itself. The surprise is where it goes: it can reach the cloud base and drain the storm in six quick bolts, or it can grow sideways over the houses and invite lightning onto your own roofs.

**The challenge:** get the town through each storm and light the lamps to a quota.
- **Doing nothing loses**, because every bolt hits the spire, the town burns and the lamps stay dark.
- **Random wire loses**, because copper pointed at random pulls bolts into the houses. A plan routes copper from rod to lamps to ground.

**Graded outcome:** one star at the light quota, two at double it, three with no house burnt. Burnt houses stay burnt for the rest of the run.

**Sample level:**
- **Map:** 320x200, with the ground at y=170. Six wooden houses stand in x 120–220, with a 40-cell spire at x=170.
- **The storm:** a cloud band at y 0–30 drifts left to right over 90 seconds while its charge rises from 0 to 1.
- **Lamps:** four, sunk at x=60, 90, 250 and 280. Your rod is at x=40.
- **Wand:** Rod, Wire, Fork, Wire, Ground.
- **Goal:** 200 light before the storm passes.

**The run:**
- **One town for the whole run.** Your copper stays between storms, and the map widens by a screen every three storms.
- **The Workshop between storms.** Pick one of three cards. Spend copper on removing a card, upgrading one (Wire+ grows 30 cells) or a wand with more slots. You salvage that copper from melted branches.
- **Storms escalate.** Storm 1 is one cloud and storm 5 is two clouds crossing. Bosses: a supercell whose leaders branch three ways; ball lightning, a charge pocket that drifts low; and hail, which breaks Glass.
- **Builds that emerge:**
  - *the farm* compounds with On strike and Wire;
  - *the bank* stores strikes in Capacitors and fires aimed bolts that blast open buried lamps;
  - *the cage* is a Glass-and-Ground dome: safe, but it scores little;
  - *the string* wires lamps one after another, and light is current times the lamps on the path. This is Balatro's mult.
- **Level 1 against level 10.** In the first storm you plant one rod and hope. By the tenth, the copper you laid over the run reaches the cloud, one press sets off a dozen strikes into a string of lamps, and the sky visibly drains.

**Risk:** the leader looks like noise and the player cannot tell why the bolt went where it did. **Test:** show the charge field on a key, then ask whether playtesters predict where the next bolt lands better than chance.

**Smallest prototype (3–4 days):** the charge field, the leader, Wire, Rod and Lamp, a 3-slot wand and one storm. Put strike counts (lamps hit, houses hit) beside a GIF, and record `ascii` worst-frame with the field running over the whole grid.

---

## 2. FIREDAMP: the breathing mine

**Pitch:** the mine leaks gas. Herd it with draughts and doors, then light it where it pays and nowhere near your lamp.

**30 seconds:** vents hiss pale gas, which spreads and pools under the ceilings. You cast **Draught**, and a push shoves the cloud down a tunnel. **Seal** shuts a door. **Spark** in a thick pocket sends a flame front through every cell above the burn line: the pocket booms, the rock cracks, and ore tumbles out. If gas reaches the lamp at your station, it goes off by itself.

**Simulation:**
- **Gas** is a concentration field that spreads and fades, using the shipped field machinery. It needs a new upward drift, because the pheromone field has none.
- **The flame front** is new and small: a cell burns if its gas is above the threshold and a neighbour is burning, and burning uses the gas up.
- **Draught** is new: it pushes the field along.
- **Shipped:** fire visuals and heat, and explosions with fracture for the boom and the ore.

**Cards:** Spark, Draught, Seal (a door), Vent (open a new gas vent: more fuel, more danger), Damp (a water mist that raises the burn line), Fan (a draught that lasts ten seconds), **Long fuse** (modifier: three seconds later), **Wide** (modifier), **Two at once** (multicast), **When thick** (trigger: fires the next card when the gas under it passes the burn line).

**Combo: When thick and Spark at the far end of an empty gallery, then Vent at the near end, then Draught toward the trap.** (1) The vent fills its end of the gallery. (2) The draught pushes the plume down to the trap. (3) The gas thickens at the trap, and the trigger lights the spark. (4) The flame runs *back* through the whole connected plume to the vent. (5) The vent keeps feeding gas, so it becomes a standing torch that cuts rock for as long as it hisses. The surprise is that a trap at one end turned the far end into a cutting tool. It may also light the next pocket you were saving.

**The challenge:** blast out the level's ore quota before the gas reaches your station.
- **Doing nothing loses**, because the gas rises until the station goes up.
- **Random play loses**, because a spark in thin gas fizzles and wastes a cast, and a spark in thick gas near the station blows up your own station. A plan herds the gas first and lights it second.

**Graded outcome:** stars for ore against the quota; station damage is kept between levels.

**Sample level:**
- **Map:** 256x160, a U of tunnels. The station and its lamp are at bottom left (x 20, y 140).
- **Gas:** three vents in ceiling pockets on the right. Their output rises over two minutes.
- **Ore:** seams in the roof of a side chamber at x 180–230, y 30–60.
- **Wand:** Draught, Seal, Spark, When thick, Wide.

**The run:**
- **You go deeper each level.** Deeper levels have more vents and hotter rock.
- **The Lift between levels.** Pick one of three cards or rules, remove a card for ore, or upgrade one.
- **The rules are Noita-style perks that flip how everything reads.** *Gas is heavier than air* makes it pool in floors instead of ceilings. *Burning leaves smoke that blocks draughts.* *Each blast refills the vents.*
- **Bosses:** a vent that keeps widening, a wet level where flowing water blocks the tunnels, and coal dust that makes the whole mine burnable.
- **Builds that emerge:** *the herder* (Draught, Fan and Seal, for one huge blast), *the trapper* (When thick everywhere, a chain you set up and then watch), and *the torch* (lit vents used as cutters).
- **Level 1 against level 10.** In the first level you light one pocket by hand. By the tenth you lay three traps and one draught, and the galleries go off in sequence like fireworks while the ore counter climbs.

**Risk:** gas is hard to see, and a trap build can become watching rather than playing. **Test:** tint by concentration, then check whether playtesters can say where the flame will stop before they light it.

**Smallest prototype (3 days):** the gas field, the flame front, Draught, Spark and When thick, on one hand-drawn level.

---

## 3. LODE: veins against the quota

**Pitch:** each round, plant a hand of crystal seeds in a slab of rock. They grow, branch and fuse, and the loops they close pay the quota.

**30 seconds:** the slab shows a blue haze of mineral, richest near the seams. You play up to four seeds from a hand of seven. Glowing tips grow toward the richest mineral and use it up as they go, so the tips compete with each other and drained ground dims. When two veins meet, they fuse into a loop, and a pulse runs round it. The round ends after 20 seconds. **Score = vein length times loops closed.**

**Simulation:**
- **Shipped:** the mineral field reuses the field machinery.
- **The tips** borrow the plant code's tip-growth and branching mechanics in the abstract.
- **New:** tips eating the field, and finding the loops (a union-find over vein cells).

**Cards:** Seed, Spike seed (one long straight vein), Fan seed (wide branching), Feeder (pours in mineral, which spreads out), Acid (softens rock so mineral flows faster), **Seek** (modifier: tips in this shot grow toward other veins rather than toward mineral), **Mirror** (multicast: copies the next seed across the slab), **Bloom** (trigger: when this vein closes a loop, plants the next card at the joint). **Lenses** sit outside the wand. They are permanent rules, Balatro's jokers: *"loops through a Feeder count double"*, *"veins that touch the edge score triple"*.

**Combo: Bloom carrying Seek and a Fan seed.** (1) Two veins meet and close a loop. (2) Bloom plants the fan at the joint. (3) Seek sends its tips toward the nearby veins rather than toward mineral. (4) It closes four small loops in a second, and the mult jumps from 1 to 5. The surprise is the cost: the fan drank the mineral the long spike needed, so the spike stalls. You traded length for loops, and you can watch it happen.

**The challenge:** the quota rises by about 1.6 times each round, with three rounds per stage and a boss at the end of each stage.
- **Doing nothing scores zero.**
- **Random seeds lose**, because they wander into barren rock and starve. A plan reads the haze.

**Graded outcome:** your score against the quota, and going over pays extra coin.

**Sample level:** a 200x120 slab with three diagonal seams and two barren patches. The quota is 400. The hand of seven is drawn from a deck of twelve.

**The run:**
- **A shop between rounds** sells lenses (five slots), cards, removal and seed upgrades (Fan+ adds a branch).
- **Length:** eight stages, about 30 minutes.
- **Bosses:** a dry slab with half the mineral, faults that cut any vein crossing them, and frost that slows tips.
- **Level 1 against level 10.** In the first round you close two loops. By the tenth, the slab lights into a lattice of forty loops in a few seconds, and the mult is in the hundreds.

**Risk:** it is the most Balatro-like of the five, and the simulation may end up as decoration with a predictable score. **Test:** after a round, ask whether a playtester can say why one hand beat another.

**Smallest prototype (3 days):** one slab, four seed types and a score counter.

---

## 4. QUENCH: hold the lava

**Pitch:** a volcano pushes lava down a valley toward a forge town. Turn water into walls, and turn steam into rain.

**30 seconds:** a tongue of lava advances. You cast **Rain** over it. Black stone crusts where the water hits, and steam rises, gathers under the cavern roof, condenses and drips onto the next flow downstream.

**Simulation:**
- **Mostly shipped:** liquids flow and level; lava and water make stone and steam; and indoors, steam condenses and drips back where it formed.
- **One change:** today the crust crumbles to rubble and sinks, which is falling sand. Here the crust must stay put as a wall, so it needs a material flag.
- **New:** a draught for steam.

**Cards:** Rain, Cold snap, Channel (dig a trench), Dam, Pump (lift water uphill), Open roof (let steam escape, so it comes back later as rain somewhere else), **Wide**, **Two at once**, **When lava arrives** (trigger).

**Combo: Channel the lava into a cavern that has a pool in it.** (1) The pool boils into steam. (2) The steam can't leave the closed cavern, so it condenses on the roof and drips back into the pool. (3) That keeps quenching the lava. One card's worth of water quenches about five times its volume and leaves the cavern sealed with a stone plug.

**The challenge:** the forge needs *some* lava to work, which is its score, and the houses need none.
- **Doing nothing loses**, because the houses are buried.
- **Blocking everything loses too**, because the forge goes cold. Random play does one or the other. A plan gives the forge a controlled trickle.

**Graded outcome:** forge output and houses saved.

**Sample level:**
- **Map:** 300x200. The vent is at top left, the forge in a hollow at bottom right, and four houses sit on the slope between them.
- **Water:** a cave pool at mid-left.
- **Wand:** Channel, Rain, Dam, When lava arrives.

**The run:** one mountain for the whole run. The stone you make stays and reshapes the valley, so your dams from wave 1 are cliffs by wave 8. Eruptions get hotter and longer. **Level 1 against level 10:** in the first wave you throw water at a tongue of lava. By the tenth you are a landscape engineer, steering lava down channels you carved earlier in the run.

**Risk:** it is slow, and close to Creeper World. **Test:** time from a cast to a visible result; it should be under 3 seconds.

**Smallest prototype (2 days):** shipped lava and water, a crust that stays put, and four cards.

---

## 5. ETCH: patterns that grow on the plate

**Pitch:** drip two reagents on a copper plate. They spread, react and grow into spots, stripes and coral, and you have to outgrow the rust.

**30 seconds:** orange rust worms creep in from a corner. You drop reagent, and a blob spreads, then splits into spots that divide and march outward. **Stir** smears them into stripes. Where your pattern meets the rust, the side with more feed wins the boundary.

**Simulation:**
- **New:** two scalar fields per cell that spread at different rates and react (Gray-Scott), plus a stir that pushes them along. It is cheap per cell, and the field machinery exists.
- **Why it fits:** diffusion alone produces branching, splitting growth, which is the owner's "growth" with no plants in it.

**Cards:** Drop, Feed (patterns here grow faster and fatter), Starve, Stir, Heat (spreads faster, so the patterns grow larger), Chill (freezes an area), **Mirror** (multicast), **On split** (trigger: when a spot divides, plays the next card there).

**Combo: Stir, then Drop in a narrow channel.** (1) The current stretches the drop into a line. (2) The line is unstable and breaks into ten spots. (3) Each spot grows until its middle runs out of feed, then splits in two. Within 15 seconds one drop has become a front across the plate.

**The challenge:** hold more of the plate than the rust.
- **Doing nothing loses**, because the rust covers the plate.
- **Random play loses**, because drops in starved ground die. A plan feeds corridors.

**Graded outcome:** the share of the plate you hold, in per cent.

**Sample level:** a 256x256 plate with rust in the top-left corner, three feed vents along the bottom and a dead strip down the middle.

**The run:** each plate is a commission. New pattern families arrive as card packs: coral, worms and holes. Bosses are rust strains that change the reaction's numbers. **Level 1 against level 10:** in the first plate you are learning that spots split. By the tenth you know which pattern each card makes, and you grow a fence of stripes on purpose.

**Risk:** these patterns are famously hard to predict, so play may feel like random seeding. **Test:** check whether playtesters learn to tell the patterns apart within ten minutes.

**Smallest prototype (2–3 days):** the two fields, four cards and one plate.

---

## Ranking

1. **ARC** 2. **FIREDAMP** 3. **LODE** 4. **QUENCH** 5. **ETCH**

ARC has the best run, and it is the most fun because **the threat is the fuel**. Every storm is more dangerous than the last, and every storm pays more to a player who has built for it, so the run escalates on both sides the way Balatro's quotas and jokers do. It is also the only concept where the run leaves a mark on the world: one town and one copper network grow across all the storms, so your tenth storm is fought with everything your first nine built. Every principle has somewhere to land:
- *On strike* makes compounding a visible thing;
- the network makes synergy positional;
- melted copper is the currency;
- bosses break one rule;
- the self-harm is real and graded, because a greedy network invites a bolt onto your own roofs.

Lightning is also a spectacle on every cast: a flash, a fork and a lamp lighting, well under a second after the strike. FIREDAMP is a close second, with the best boss rules ("gas is heavier than air" changes every level), but its threat is something you vent away rather than something you turn into power. LODE has the cleanest run on paper but the weakest reason to exist on this engine.

## Prior art

| Game | What it teaches | Link |
|---|---|---|
| Noita | Order is grammar, and the world multiplies the wand | project research: `game-design/noita-wands/noita-wands.html` |
| Slay the Spire | Pick one of three, removal and upgrades between fights; tune by data | [GDC 2019 talk](https://gdconf.com/news/learn-slay-spires-successful-metrics-driven-approach-game-balancing-gdc-2019) |
| Balatro | Multiplied scoring against targets that rise even faster; boss rules | [Giant Bomb](https://www.giantbomb.com/balatro/3030-90552/) |
| Luck be a Landlord | Rising rent as the clock; synergy between neighbours; removal tokens | [Rogueliker](https://rogueliker.com/luck-be-a-landlord-review/) |
| Monster Train | Placement as the decision; cut effects nobody can reason about | [PC Gamer](https://www.pcgamer.com/games/card-games/monster-train-2-echoes-from-the-void-interview/) |
| Inscryption | Upgrades that risk the card; abilities moved between cards | [Game Developer](https://www.gamedeveloper.com/design/how-game-jam-sacrifices-became-inscryption) |
| Dicey Dungeons | Random inputs into readable, fixed slots | [Game Developer](https://www.gamedeveloper.com/design/witch-craft-how-i-dicey-dungeons-i-balances-chance-and-predictability) |
| Magicka | A small set of elements that combine into named results | [StrategyWiki](https://strategywiki.org/wiki/Magicka/Elements) |
| Creeper World | A spreading fluid as the enemy; you don't have to wipe it out | [PC Gamer](https://www.pcgamer.com/fighting-massive-waves-of-goo-in-creeper-world-4-is-oddly-blissful) |
| Oxygen Not Included | Gas that spreads and pools as the puzzle; design grown from the simulation outward | [Game Developer](https://gamedeveloper.com/design/behind-the-design-of-hit-sim-game-i-oxygen-not-included-i-) |

*Sources were checked through search excerpts. Fetching pcgamer.com was blocked in this session, so its claims come from search summaries.*
