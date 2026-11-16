# Review r1: combos, luck and compounding

2026-11-16. Written from the brief alone. Lens: how cards compound, how luck
picks the builds a run reaches, and how categories stay distinct.

## Lead: what "cards in a gun" is most likely missing

1. **Aiming.** In Noita the wand's program is half the build and the hand
   is the other half. Here soldiers shoot "slime in range", so every gun hits
   the nearest front, and the verbs that matter against a network (cut it,
   starve it, poison its source) never happen by auto-fire. The missing card
   kind is a **targeting doctrine**: what a soldier chooses to shoot. That is
   the RTS player's aim, and it should be a build choice.
2. **The world as the combo partner.** Most of Noita's chaos is materials
   reacting, not the wand. This sim already has reactions: the body stops
   fire, flung walkers live where they land, cut-offs panic and charge, sick
   walkers infect neighbours. Cards that set up and spring those rules
   compound for free; cards that only clear a disc don't.
3. **The slime as the fuse.** Killing is the weakest verb: the slime
   regrows. The strongest compounding is what the network carries itself:
   contagion, marked walkers flowing to their crystal, a charge running
   along veins. These scale with the enemy, which is what lets a lucky run
   go absurd: the bigger the slime, the bigger the fire.
4. **Cross-squad grammar.** With several squads, a combo can span guns: one
   squad paints or dries the slime, another reads the mark. Formation
   becomes combo order, which is the RTS version of a wand's slot order.
5. **Cost discipline as a design rule.** Chaos should live in fields and
   walker state bits, which cost the same every frame, not in bullet counts.
   That one rule lets a lucky run go wild without crashing a GTX 1070.
6. **Other directions** (section 3): a player-bred mould strain, emitter
   buildings, guns that grow, an adaptive slime.

## 1. Generate (33 effects, unjudged)

1. **Scalpel line**: kills a 1-cell-wide segment across the vein and leaves
   no-trail ground there for 3 s. A clean gap; the far side pales.
2. **Eraser**: zeroes the trail in a disc, kills nothing. Walkers scatter
   into loose dots, then reknit.
3. **Bait**: writes food scent at the impact point, a fake crystal. You see
   tendrils bending toward an empty spot.
4. **Repellent**: writes negative food scent. The slime parts around it like
   water around a stone.
5. **Alarm flare**: writes alarm. Flee-species recoil; swarm-species flood
   into a kill box.
6. **Sleep puff**: releases a small sleep-gas cloud. A dimmed patch drifts
   with the gas.
7. **Plague dart**: infects walkers in a disc. A sick tint creeps outward
   along the veins.
8. **Desiccant**: marks walkers dry, so the body no longer stops fire. Pale,
   cracked slime.
9. **Incendiary**: ignites at impact and spreads through dry plants and dry
   slime.
10. **Soak**: writes the moisture field. It puts fire out and leaves a damp
    firebreak (a darker ground tint).
11. **Crater**: leaves no-trail ground in a disc for N seconds, a temporary
    wall that the network flows around.
12. **Fling-out**: throws walkers away from the impact. They land and live,
    seeding growth. Risky.
13. **Fling-to**: throws walkers toward a chosen field instead: into water,
    fire, gas, or the friendly kill box.
14. **Siphon**: drains stored energy, kills nothing. The area pales, then
    withers seconds later.
15. **Brand**: tags walkers, who burst when they die. They flow with the body
    toward food, so the bomb travels.
16. **Paint**: writes a player-only "target" field that other squads'
    doctrines read. A coloured stain.
17. **Invert**: walkers turn away from trail for N seconds. The vein
    unravels like a frayed rope.
18. **Highway**: lays trail along the flight path, steering the slime down a
    lane you chose.
19. **Path-scorch**: writes no-trail along the whole flight path. One shot
    draws a wall.
20. **Fragment**: at impact, splits into N child bullets fired outward.
21. **Linger**: the payload's field write persists and drifts like gas
    instead of being a one-frame stamp.
22. **Stasis**: walkers freeze and stop laying trail: a living wall others
    route around. A crusted patch.
23. **Condense**: forces a dense lump for a heavy slug to delete. Left alive,
    it may break away.
24. **Density mine**: sticks to the ground and bursts when the trail under it
    passes a threshold: it waits for traffic.
25. **Vein-runner**: follows the trail ridge toward the thickest flow, racing
    along a vein like a bead on a wire.
26. **Scent hound**: climbs the food-scent gradient and strikes the feeding
    point on a crystal.
27. **Sterilant**: walkers can't divide for N seconds; the growth tips go
    still.
28. **Mitosis bomb**: forces immediate division at half energy each. The blob
    doubles, then mass-starves. A gamble.
29. **False cut**: walkers believe they are cut off and charge the nearest
    soldier, so a bait squad picks the fight.
30. **Vortex**: writes a pull vector field, so walkers drift into a knot for
    a follow-up burst.
31. **Conductor**: a "charge" field that diffuses only where trail is high,
    carrying the payload along veins. A real front moves through the network.
32. **Seed shot**: plants dry tinder for a later fire, or a wet plant as a
    living firebreak.
33. **Blind**: zeroes sensor input in a cloud; walkers random-walk and the
    network dissolves into fog until it clears.

## 2. Structure: the combination grammar

### Card kinds (what reads what)

| Kind | Reads | Writes | Per gun |
|---|---|---|---|
| **Doctrine** | world fields: trail width, scent, paint, state bits, alarm | the target choice | exactly 1, slot 0 |
| **Carrier** | fields in flight: trail ridge, scent gradient, terrain | the bullet's path (and optionally a path write) | 0-1 per payload group |
| **Modifier** | the next payload or carrier | its numbers or its shape | any |
| **Payload** | nothing | one walker rule or one field channel | the verb |
| **Trigger** | events: impact, timer, kills ≥ N, field over threshold, death of a branded walker | releases its child list | wraps the next K cards |
| **Multicast** | nothing | fires the next N groups as one volley | rare |

Read order for one cast: Doctrine (target cell), Carrier (path), Modifiers,
Payload, then Trigger → child list; then the drum advances. Cards cycle
forever, so a 5-card gun fires a visible **rhythm** (cut, cut, brand, cut,
burn). Cast delay per card is the only spend.

Doctrine options: nearest (default, free); **thinnest vein in range**
(a cut point, approximated by local trail width); **feeding point** (slime on
a crystal); **frontier tip**; **densest lump**; **painted**; **in field X**
(only shoot gassed, dry or sick slime); **healthiest** (aim away from the
plague so it can spread). Doctrine is where an auto-firing squad gets a brain.

### Five verb families, each owning a channel and a look

Every payload belongs to one family. Families differ in *what part of the
simulation they touch*, so numbers can't compare them:

- **Sever** (kills, no-trail ground, erase): shapes geometry. Seen as black gaps.
- **Steer** (scent, trail writes, repel, alarm, vortex): moves the body. Seen as
  tendrils bending.
- **Afflict** (state bits: sick, asleep, dry, inverted, branded, sterile):
  changes behaviour over time. Seen as a walker tint that spreads.
- **Hurl** (fling, condense, stasis): moves mass. Seen as arcs and lumps.
- **Element** (fire, water, gas, plants): changes the ground. Seen as
  environment fronts.

### What stops "more damage"

- **Same family adds, with diminishing returns; different families
  multiply.** Two Sever cards give about 1.4× the area; Sever plus Afflict
  gives a different outcome, not a bigger one. Stacking one thing caps early.
- **Damage is not a card.** It lives on the rolled bullet. Modifiers scale
  area, duration, drift, children and conduction, never kill count.
- **The best reactions fall out of rules the sim already has**, with no
  authored combo table. Dry plus fire burns through the body. Sick plus
  fling catapults the plague into healthy tissue (landed walkers live on,
  still sick). Sever plus bait strands a lured arm. False cut plus a guard
  squad is a kill box. Brand plus the slime's own flow delivers the bombs to
  its crystal. Players discover these on screen.
- **Rare relics add world rules** for the run, Isaac-style: "fire spreads
  through sleep gas", "sick walkers lay double trail", "branded deaths
  release sleep gas". A relic turns a pair into a named build: the main luck
  lever on build identity.

### Where luck enters

1. **Gun rolls**: slots, budget, rate, spread. Slot count is the
   run-making roll. A +2-slot tail gun lets you nest a trigger.
2. **Card rolls**, the key one: **design in-sim thresholds a tail roll can
   cross.** Sleep duration past gas decay time makes a self-sustaining
   cloud; crater duration past reknit time makes a permanent wall; scalpel
   width past vein width makes a sure cut; a fragment count of 2.6 rounds
   to 3. The normal distribution then yields rare *qualitative* jumps with
   no "lucky" label, as the owner's rule asks.
3. **Shop shelf**: which pieces show up (see below).
4. **Planet**: species and map, scouted before the shop, so luck sets the
   question and the player answers it.

### Caps that keep it on a GTX 1070

- **Chaos lives in fields and bits.** A status is one bit on an existing
  walker; a field is a fixed-cost grid pass. A whole slime sick, dry,
  branded and burning costs about an ordinary frame.
- **Stamp buffer.** All bursts and field writes of a frame go into one
  buffer and one dispatch, capped near 256 stamps and 64k cells. Stamps in
  the same 8×8 tile merge (max radius, OR the writes). Overflow defers a
  frame: a slight delay, never a crash.
- **Bullets**: at most 48 live per squad and 400 globally. At the cap,
  children are dropped before primaries.
- **Children**: at most 8 per bullet. Bullets may spawn bullets **one level
  deep**. Field-only payloads may nest to depth 3.
- **Proc coefficient.** A kill-count trigger converts kills into credit at
  0.5 per generation, from a per-squad pool that refills at a fixed rate.
  Risk of Rain's fix: a proc can't feed itself without decay.
- **Conduction is a field, not a flood fill.** A "charge" field diffuses,
  masked by trail. That costs a fixed amount, runs the same on CPU and GPU,
  and is visible as a real front (not a painted pulse).

### The shop between planets: making luck matter

- **Six-item shelf**: two guns, three cards, one relic, upgrade or building.
  One free **lock** carries an item to the next shop; rerolls cost more each
  time.
- **Synergy bias**: about 30% of card offers come from families you own, so
  a lucky run finds the second and third piece. At least one offer comes
  from a family you lack, so an average run still sees new verbs.
- **Scouting report** before buying: the next planet's species, plant cover,
  moisture and crystal layout.
- **Polish**: re-roll one stat and keep the better (best of two): money
  into luck, controlled.
- **Fuse**: merge two copies of a card (mean roll plus a small bump), a slow
  road to the threshold-crossing tails.
- **Odd-shaped guns**: offer extremes (9 slots and slow, 2 slots and a
  firehose); gun shape variance drives build variance.
- **Field salvage**: a breakaway lump carries a card. Chasing it is a
  risk-reward choice: luck you earn on the map.

### An average run and a lucky run

*Average*: three squads, 3-4 slots each. One cuts (thinnest vein), one
brands and baits, one burns. Every common card is a verb, so the choices
are which pair each squad runs, which doctrine, and where to stand.
Pictures differ by planet because species and map pick the pair.

*Lucky*: a 7-slot gun rolls a tail sleep duration that outlasts the gas
(self-sustaining clouds), and the shop shows the relic "fire spreads
through sleep gas". Now a nested trigger (desiccant → sleep → spark) turns
every cloud into a slow firestorm that the slime's own flow feeds walkers
into. It looks absurd, but it is still a few hundred stamps and four state
bits a frame.

## 3. Big picture

### Three best builds on planet 6

1. **Surgeon.** Doctrine: thinnest vein. Scalpel line, path-scorch, linger,
   plus a guard squad for the panic charges. It cuts narrow points and the
   cut-offs starve. Strong on open maps with far-apart crystals (long
   veins). Weak against spores (re-seed past cuts) and dense species (no
   thin veins).
2. **Plague engine.** Doctrine: healthiest. Plague dart, brand, conductor, a
   scent-hound carrier; a "kills ≥ N" trigger releases fling-to toward the
   thickest flow. Scales with the slime. Weak against condensing species
   (less contact) and anything that sheds sick walkers.
3. **Firestorm.** Doctrine: slime in plants. Desiccant, then incendiary on
   an impact trigger; a second squad seeds dry tinder, a third soaks the
   ground round the pumps. Strong on dry plant maps, useless on rivers,
   dangerous to your own base.

(A fourth, **Herder**: bait, alarm, false cut and a vortex feeding a
condense-and-slug squad. Strong against the swarm-to-alarm species.)

**Do they differ?** Only if the world makes them. Without counters, the
plague engine wins: it is the only one that scales with the enemy. Divergence
comes from:

- **Species read fields differently, so each family has a natural enemy.**
  Alarm-flooders punish (or reward) alarm writers. Sever wakes the sleeper.
  Spores beat Sever; dense species beat Afflict; a memory species avoids
  where you hurt it, which beats static mines.
- **The map sets the elements**: plant cover and moisture for fire; rivers
  for fling-to-water; rock corridors that make choke points for the Surgeon;
  crystal spread for the Herder.
- **Scouting before the shop** turns these into purchase decisions.

### What is missing from the framing

- **Targeting doctrine** (covered above): the biggest gap.
- **Squad stances**: hold fire, fire at will, fire only on painted.
  Auto-fire with no ammo removes timing from the build, so give it back to
  the player as RTS orders.
- **Backfire as part of the spread.** Flung walkers seed growth, cut-offs
  charge your soldiers, fire reaches your pumps. A combo that can go wrong
  is more exciting, and fits "a spread, not a yes/no".
- **The economy link.** Crystals are both your income and the slime's food.
  Doctrines and payloads that act on feeding points (scent hound, siphon,
  sterilant at crystals) tie the gun build to the pump map.

### Other directions worth a look

- **A tamed strain.** A second walker species the player breeds in the
  shop. Its genes (sensor angle, trail affinity, appetite, aggression) are
  the build, and it fights the wild slime for trail and food: the core model,
  used more deeply than any gun.
- **Emitters.** Buildings that write a field all the time (bait towers,
  repellent fences, sleep vents), with guns kept for Sever and Afflict.
  This is the tower-defence half of the same grammar.
- **Guns that grow.** A card's rolls drift with the walkers it kills, and at
  a threshold it rolls one new trait. The build becomes the run's history.
- **An adaptive slime.** Later planets' strains partly resist your most-used
  family (shown in the scouting report). This forces mid-run pivots and
  keeps builds from converging, but keep it partial so commitment still pays.

## 4. The owner's questions

**4a. Chains: yes. Bullets one level deep, fields up to three.** A trigger
wraps the next K cards, and its child list may hold another trigger. A child
bullet cannot spawn bullets; a field-only child may nest to depth 3, because
fields cost a fixed amount and bullets don't. Each generation carries a 0.5
proc coefficient and draws on a per-squad credit pool. Only a tail roll on
slots fills a deep nest: rare, and that is the absurd run.

**4b. Seekers: only seekers that read the slime's own fields.** "Seek the
nearest slime" duplicates the soldier's aim and adds nothing. What is worth
it are carriers that take the payload somewhere the soldier cannot aim: the
vein-runner rides the trail ridge to the thick flow, the scent hound climbs
food scent to the feeding crystal behind a wall, and others chase alarm or
the healthy tissue. They move like walkers, by the same fields, so the
player reads them at a glance.

**4c. Thrown devices: join the card language, keep their own cadence.**

- *Keep them separate*: throwing is the game's only hand-aimed, player-timed
  verb, a different rhythm from auto-fire. Big payloads (sleep canister,
  demolition charge) would break if auto-fired. Two systems are easier to
  balance and explain.
- *Join them*: one grammar means one set of combos to learn. A canister that
  carries cards lets builds reach the toolbelt, and the shop gets simpler. A
  mine is already "sticky carrier + density trigger".
- *Pick*: each squad gets one **kit slot**: a device with its own 2-3 card
  slots in the same card language, thrown on the player's command with a
  cooldown (the only cooldown in the system). The language is shared, the
  timing stays the player's, and the big payloads stay off auto-fire.

## 5. Top 5, each with a first cheap experiment

1. **Targeting doctrine** (thinnest vein, feeding point, painted).
   *Experiment*: one gun, nearest vs thinnest-vein, 10 paired seeds, 90 s.
   Film it. Count cut-off components over 200 walkers, starvation deaths in
   them, and crystals the slime loses. Deciding measure: starvation deaths,
   sign test across pairs.
2. **State bits plus sim-native reactions** (desiccant × fire, plague ×
   fling). *Experiment*: incendiary alone vs desiccant-then-incendiary on a
   plant map. Film the fire front entering the body. Count how far the burn
   penetrates into the slime (cells past the edge) and walkers burned in 30 s.
3. **Conductor field / brand-with-flow** (the slime as the fuse).
   *Experiment*: brand 50 frontier walkers and film where they burst. Count
   bursts within 5 cells of a crystal vs at impact; for the charge field,
   count cells reached in 2 s.
4. **Field-reading carriers** (vein-runner, scent hound). *Experiment*:
   from a fixed squad position, straight bullet vs scent hound, with a
   feeding point beyond range behind rock. Count the fraction of shots that
   strike within 3 cells of a feeding point. Film the paths.
5. **Threshold-crossing rolls plus caps.** *Experiment*: draw 1,000 rolls
   each of three cards (sleep vs gas decay, crater vs reknit, scalpel vs
   vein width) and count threshold crossings, aiming for 3-8%. Then run the
   worst-case build (multicast 4, fragment 8, depth 2, linger) on 100k
   walkers on the 1070: quote ms a frame, stamps merged and deferred, and
   film it to check the screen still reads.
