# Explorer B: growth networks (2026-11-07)

Five puzzle/challenge games where the star is a **network that grows**: tips that extend, branch, fuse and fight for room. Flows and diffusing fields feed the tips. No plants, no creatures, no falling sand.

The rules every concept follows, taken from the satchel playtest:
- **The cards say what they do in world words.** The next play is always written on screen in a plain sentence ("Next: two bolts, bushy, from the cloud") with a faint preview of where it will go.
- **Every card leaves cells behind** (scorch, water, crystal, crack, vein) within a second of being played.
- **The threat grows on its own.** It is the thing you are fighting, so doing nothing loses.
- **The natural path is the bad path.** An unshaped, randomly aimed card follows the world's default route, and the level is built so that route is the disaster. That is why random play loses to a plan.

---

## 1. Arc: grow lightning, and spend the storm before it spends itself

**Pitch.** A storm cloud fills with charge. You cast it down as lightning that grows like a branching tree through the air, follows its own scorch marks, and lights, boils, melts or blows up whatever it reaches.

**Core loop (30 seconds).** The cloud's charge bar rises. You press cast. The wand deals its next card: a bolt crawls down from the cloud over about half a second, forking and feeling for the ground. It hits a puddle, which flashes to steam, and a branch touches a copper lamp, which lights. Violet ionised air hangs where the bolt went, drifting in the wind and fading. You have about two seconds to cast "Again" while the haze is still there. If you hold the charge too long, the cloud strikes by itself, on the tallest thing in the world.

**The simulation.**
- **Charge field (new).** A cheap potential field is worked out only at the moment of a cast, on a quarter-resolution grid: the cloud is high, the ground and water are low, metal carries whatever it touches. It costs nothing while nobody casts.
- **Bolt growth (new, borrowing plant tip logic).** The plant code's "weigh each neighbour, then pick one at random" step is reused. The weight is how steeply the field drops that way, raised to a "straightness" power. A low power gives a bushy, Lichtenberg-like tree and a high one gives a spear.
- **Ionised air (reuses the diffusing scalar fields, the pheromone machinery).** Every bolt cell deposits ions. Ions spread, fade over about 3 seconds and drift with the existing weather wind field, and later bolts strongly prefer ionised cells.
- **What a hit does (existing).** Water turns to steam, oil and anything dry ignite (fire), and ice cracks (fracture). A heavy bolt's end point explodes (explosions). Fire's hot gas counts as conductive, so a burning patch draws the next strike to it.
- The shipped weather already has a lightning flash, but it is only drawn on screen and touches no cells. Arc makes it real.

**Cards.**
1. **Bolt.** One bolt from the cloud. It costs charge, the way Noita spells cost mana, so spending charge is good.
2. **Bushy** (modifier). The next bolt forks a lot and reaches less far.
3. **Straight** (modifier). The next bolt hardly forks and goes twice as far.
4. **Heavy** (modifier). The next bolt carries double charge, and its end blows a small crater.
5. **Rain.** A curtain of water falls under the cursor. Puddles are conductive.
6. **Copper dust.** A spray of metal that settles as a thin conductive smear on whatever it lands on.
7. **Glass pane.** An insulating sheet. Bolts go round it.
8. **Two at once** (multicast). The next two cards fire together. Two bolts repel each other, so they splay apart.
9. **Rod** (trigger). Plants a copper spike that holds the next card. When any bolt touches the rod, the held card fires from the rod's tip.
10. **Again.** A bolt that follows the freshest ionised air, wherever that has drifted.

**The surprising combo: Bushy Bolt, then Again, on a windy level.**
1. The Bushy Bolt forks into eight thin branches. They die in open air and reach nothing. It looks like a wasted cast.
2. Every branch left ions. Because there were many branches, there is a wide sheet of violet haze, not a thin line.
3. The wind blows the haze sideways and it spreads. Some of it curls under a stone overhang where a lamp hides, out of reach of any straight line from the cloud.
4. Again prefers ions to the charge field, so it runs down the drifted haze and hooks *under* the overhang to the lamp.

The wasted bolt was the aim, and the wind did the bending. Nobody wrote "lightning goes round corners".

**The challenge.** Light every lamp before the storm passes, which is five cloud-fills. A bolt left to itself, or the cloud's own overflow strike, goes to the tallest conductor. On every level that is something you must protect, such as the powder magazine on the cliff. So a random bolt *is* the disaster. A plan uses rain, copper and rods to build a cheaper path somewhere else.

**Graded outcome.**
- One star for each lamp lit.
- A bonus star for no strike landing on the magazine.
- The score is charge spent usefully against charge wasted.
- The magazine has a health bar (three hits), not one-hit death, so a sloppy run limps home on one star.

**Sample level ("The Overhang").** World 512x320.
- **Top 40 rows:** the cloud, filling every 18 seconds.
- **Left:** a stone cliff to row 90 with a black-powder magazine on top. It is the tallest point.
- **Middle:** a shallow basin, with a pond (rows 250-280) under a thin film of oil.
- **Right:** a granite overhang from x=380 to 470 at row 150. Two copper lamps sit under it at rows 190 and 230, and a third lamp stands in the open at x=300.
- **Wind:** a steady breeze blowing right.
- **Wand:** 4 slots, one card per cast, with the deck Bolt, Straight, Bushy, Bolt, Rain, Again, Rod, Glass.

The open lamp teaches Straight, the oily pond teaches that fire draws the next strike, and the hidden lamps need the wind trick or a Rod.

**Risk.** Bolts read as random, so the player feels they are rolling dice. To find out, add the faint "likeliest path" preview and measure whether a new player can hit a marked target 3 times in 5 with Straight. If not, raise straightness until they can. Fun beats fidelity.

**Smallest prototype (3-4 days).** A lab-style test scene with:
- the charge field solved on cast only;
- tip growth animated over 30 frames;
- an ion field on the existing scalar-field code;
- hits that reuse ignite, steam and explosion;
- four cards (Bolt, Bushy, Straight, Again) with no deck, picked from a strip;
- one lamp behind an overhang, and wind on.

Publish a GIF page with counts (lamps lit, cells ignited, ion-following bolts) and the `ascii` worst frame during a cast.

---

## 2. Veins: out-grow a rival network to the dry wells

**Pitch.** Your spring pushes glowing veins through rock. Veins that carry flow grow thick and bright, and veins that don't go pale, crack and crumble. You lure, cut and seal to beat a rival network, which follows the same rules, to the wells.

**Core loop.** Twenty tips feel their way through sandstone toward the dry wells. You drop a Lure. A drop of scent-like vapour soaks outward, and over five seconds nearby tips bend toward it. You cut the rival's trunk: the severed end spurts liquid, and its whole downstream half fades from bright red to grey over ten seconds.

**The simulation.**
- **Pressure along the veins (new, built on the diffusing-field code but confined to vein cells).** Flow through a vein is the pressure difference along it.
- **Thicken or starve (new).** Above a threshold flow, a vein gains a cell of width. Below it, the vein loses width, then dries to a brittle crust and crumbles. The death is graded, never an instant vanish.
- **Tips (borrowing plant tip and root logic).** Roots already "lean toward water they can smell". Here the tips climb a gradient of attractant gas that dry wells breathe out. It diffuses through porous rock (fast in sandstone, slow in shale, not at all through granite) and decays. A tip that meets its own network fuses into a loop, which makes the network sturdier.
- **Fronts.** Where two rival tips meet, the higher-pressure side pushes the front one cell at a time.
- **Spills and heat (existing).** A cut vein spills real liquid. Heat from fire dries veins faster.

**Cards.**
1. **Lure.** A puff of attractant at the cursor.
2. **Bitter.** A puff of repellent. Tips turn away.
3. **Cut.** Severs a line two cells wide.
4. **Pump.** Doubles your spring's push for ten seconds. The veins visibly swell.
5. **Seal.** Glazes a stretch of vein so it can't be cut or pushed.
6. **Flood.** Pours water that washes the gas out of a region.
7. **Burn.** Fire dries any vein in the patch, yours included.
8. **Every tip** (multicast). The next card happens at every one of your tips at once.
9. **Wide** (modifier). The next card's area is doubled.
10. **On touch** (trigger). Holds the next card and fires it at the first well or rival vein your network touches.

**The surprising combo: Cut the rival's trunk above the lake well.**
1. Cut severs the rival's main vein. Its downstream half starves and crumbles over ten seconds.
2. A crumbling vein releases its liquid, so a red spill runs downhill through the open cave (the liquid engine).
3. The spill pours into the dry well the rival was racing for. A wet well stops breathing attractant, because dry wells are the ones that call.
4. With that source gone, the gas gradient flips. *All* tips, yours too, swing toward the next well across the map.

You meant to hurt the rival and turned your own network round as well.

**The challenge.** Hold the most wells when the timer ends. The rival starts with a stronger spring, so doing nothing loses on pressure. A random Lure drags your tips into long thin dead ends that cannot carry enough flow, and they starve. A plan makes short, fused, thick routes.

**Graded outcome.**
- Wells held, from 0 to 7.
- The amount of fluid delivered.
- A "tidy network" bonus for few starved veins.

**Sample level ("Seven Wells").**
- **Rock:** 512x320, sandstone with shale bands at rows 120 and 220 and a granite wedge in the middle.
- **Springs:** yours bottom-left with push 1.0, the rival's bottom-right with push 1.3.
- **Wells:** seven dry wells, one of them under a lake held back by a thin stone wall at the top middle.
- **Clock:** 3 minutes.
- **Wand:** 5 slots, 1 cast every 4 seconds, with the deck Lure, Lure, Bitter, Cut, Pump, Every tip, On touch, Flood.

**Risk.** It is slow, so it becomes watching. Cause and effect are seconds apart, which is the same complaint the ant lab got. To find out, time a tip from the Lure to visibly turning. If it takes over three seconds, speed up growth. Also test whether players can tell their veins from the rival's at phone size.

**Smallest prototype (4-5 days).**
- pressure on vein cells;
- thicken and starve;
- tips on the gas field;
- one rival with no cards;
- three cards (Lure, Cut, Pump).

Publish a 4x-speed GIF of one match with counts (wells held, veins starved).

---

## 3. Fault: crack the ice wall before the lake breaks it

**Pitch.** A meltwater lake swells behind an ice wall. You tap, drill, soak and freeze to run cracks through the ice. You want a controlled spillway before the lake bursts the whole wall onto the mill below.

**Core loop.** You tap the ice. A white crack runs, branches and stops at a drilled hole. You pour water and it seeps into every hairline. You play Frost and every wet crack jumps forward at once with a crackle. A slab breaks free, slides and drops into the valley. Water starts to pour through the gap you cut.

**The simulation.**
- **Tension field (new; a diffusing scalar).** Weight on the ice, the lake's push on the wall, hot spots (from the existing heat) and freezing water in cracks all feed it.
- **Crack tips (new; the plant tip logic again).** A tip steps to the neighbour with the most tension, forks when tension is high, and stops dead at a hole or the edge of a gap. That is the real crack-stopping trick.
- **Breaking loose (existing).** When cracks close a loop, the piece becomes a falling body through the existing fracture and structural-collapse code. Water flows through gaps (existing liquids), and cold nights freeze standing water (existing weather).

**Cards.**
1. **Tap.** Starts a crack at the cursor.
2. **Drill.** Bores a hole that stops any crack that enters it.
3. **Score.** A dotted line of weakness that cracks prefer.
4. **Warm.** Heat makes tension around it.
5. **Meltwater.** Pours water.
6. **Frost.** Freezes all water standing in cracks, and every wet crack grows.
7. **Weight.** Drops a boulder.
8. **Long** (modifier). The next crack runs twice as far.
9. **Fork** (modifier). The next crack splits.
10. **When it breaks** (trigger). Holds a card until the next slab breaks loose, then fires it where the slab was.

**The surprising combo: Meltwater, then Frost.** Water runs into *every* open crack, including hairlines left by earlier taps that you had forgotten. Frost then freezes all of them, so twelve cracks advance at once. Some meet and close loops, and three slabs come loose on one card. The water was the multicast, not the card.

**The challenge.** The lake rises on a clock. If it reaches the top of the wall, the wall fails all at once: the worst outcome. Random tapping cracks the wall top to bottom and causes exactly that. A plan drills stoppers first, then scores a notch.

**Graded outcome.** How much the flood is tamed:
- the spillway flow kept below a mark (stars);
- the mill hit by no slabs;
- the lake drained by more than 50% before the timer.

**Sample level ("The Mill").**
- **Ice wall:** x=200-300, from row 60 to row 300.
- **Lake:** behind it on the left, starting at row 140 and rising 1 row every 3 seconds.
- **Mill:** a wooden structure at x=360-420 on the valley floor.
- **Wand:** Tap, Drill, Drill, Score, Meltwater, Frost, Long, When it breaks.

**Risk.** The tension field is invisible, so cracks look arbitrary. To find out, test the prototype with and without a faint tension shimmer and see whether players predict where a crack will run.

**Prototype (4 days):**
- the tension field;
- crack tips;
- loop-close handing over to the existing rigid-body code;
- three cards (Tap, Drill, Frost).

---

## 4. Brine: grow salt crystals that fight each other for the salt

**Pitch.** You pour brine into a flooding cave and seed crystals. The needles race each other for the dissolved salt, and you steer them with heat and cold to seal the vents before the cave fills.

**Core loop.** You seed a crystal. White needles spike out where the brine is richest, and the brine around them pales as they drink it. You chill one side and the needles there shoot ahead. A second crystal that you placed badly steals the salt, and the first stalls.

**The simulation.**
- **Dissolved salt (new; a diffusing scalar carried by the existing liquid).**
- **Crystal tips (new).** Tips grow where salt is richest. Because the outermost tips reach fresh brine first, needles beat lumps and neighbours starve each other. This is real diffusion-limited growth, and it makes feathered dendrites for free.
- **Temperature (existing heat).** It sets how much salt the water holds: cold makes needles grow fast, and warm dissolves them.
- **Other existing pieces.** Crystals are solids that carry weight (structural). Surface brine dries to a crust (evaporation).

**Cards.**
1. **Seed.**
2. **Chill.**
3. **Warm.**
4. **Brine.** Pours salty water.
5. **Fresh water.** Pours plain water.
6. **Stir.** A current that pushes salt along.
7. **Needle** (modifier). The next crystal grows one long spike.
8. **Bloom** (modifier). The next crystal grows bushy.
9. **Two at once.**
10. **Sinker** (trigger). A seed that sinks and sprouts where it lands.

**The surprising combo: Fresh water poured gently onto brine.** Fresh water floats on brine. This needs a new rule: the brine is heavier the saltier it is. The salt can only meet the cold fresh layer along the flat boundary, so Chill plus Two at once grows a flat crystal shelf right along that line. A card that is "just water" built a floor across the cave.

**The challenge.** Hot springs (the vents) refill the cave. Each vent you seal stops its share. Doing nothing floods the exit. Random seeds compete for the salt, and none of them reach the vents.

**Graded outcome.** The number of vents sealed, the water level at the end, and a bonus for using less brine.

**Sample level ("Three Vents").** A 512x320 cave:
- vents at the floor at x=100, 260 and 420;
- the water rising from row 300;
- an exit at the top right, row 40;
- a cold wall on the left.

Wand: Seed, Seed, Chill, Brine, Fresh water, Needle, Sinker, Two at once.

**Risk.** Crystal growth is slow and samey, and it all looks like white fuzz. To find out, run the prototype at 4x and check whether needles from different seeds stay visibly separate (give each seed its own tint).

**Prototype (4 days):** the salt field in water, crystal tips, cold/warm from the existing heat, and three cards.

---

## 5. Delta: let rivers carve themselves, then steal one with another

**Pitch.** A spring's water carves its own branching channels through soft clay. You notch, plug, bake and freeze to steer the rivers into every dry basin before the spring runs dry.

**Core loop.** Water spreads across the clay. Where it runs fast it digs, the channel deepens, the deeper channel takes more water, and it digs faster. A river forms on its own. You notch a bank and a new branch breaks off. You freeze a channel shut and the river backs up and cuts round the ice.

**The simulation.**
- **Water (existing).** The engine already knows which water is moving: it draws moving water white. Clay touching moving water wears away.
- **Silt (new).** Worn clay becomes suspended silt, a diffusing amount carried by the water, which settles as fresh clay where the flow slows. That gives deltas and blocked channels with no falling sand.
- **Ice and heat (existing).** Ice comes from weather. Heat bakes clay hard.

**Cards.**
1. **Notch.**
2. **Clay plug.**
3. **Stone bank.**
4. **Bake.** Clay becomes brick.
5. **Ice plug.** Melts after 20 seconds, so it is a built-in timer trigger.
6. **Surge** (modifier). The next pour is fast and carves hard.
7. **Seep** (modifier). The next pour is slow and drops silt.
8. **Split** (multicast).
9. **Dig ahead.** A tip of erosion that runs uphill from a river's head, which is how one river steals another.

**The surprising combo: Ice plug across the main channel.** The water backs up and carves a second channel round the plug. Twenty seconds later the ice melts and both channels run. The old one silted up partly while it was blocked, so the river splits for good. A braided river from one card.

**The challenge.** Fill four basins from a finite spring. Doing nothing lets the river find the natural fall into the sinkhole, and everything drains away. Random notches spread the water thin, and it soaks away.

**Graded outcome.** The percentage each basin is filled, with stars at 50, 80 and 100%.

**Sample level.** A clay slope from the top left to the bottom right, a sinkhole at the bottom right, and four basins at different heights. Wand: Notch, Notch, Plug, Bake, Ice plug, Surge, Split, Dig ahead.

**Risk.**
- Erosion is slow and looks muddy.
- Running water keeps chunks awake, so it is the costliest concept to run.

To find out, measure the worst frame with a whole slope of moving water, before any card work.

**Prototype (4 days):** clay wear on moving water, silt settling, and three cards.

---

## Ranking

1. **Arc.** It is the only one where a card resolves in under a second, the same speed as a Noita spell.
2. **Veins.** The most "network as the star" of the five, and the best match for the owner's growth brief, but it risks being watched rather than played.
3. **Fault.** It reuses the most shipped engine (fracture, rigid bodies, freezing), and slabs falling are satisfying.
4. **Brine.** The prettiest fields, and the slowest.
5. **Delta.** The costliest frames, and close to an existing genre.

**Why Arc will be the most fun to play.** It has the satchel's missing ingredient: an instant, loud, branching answer every time you press the button. The bolt grows in front of you, it lands somewhere and something happens: steam, fire, a lit lamp, a crater. It also fixes both of the satchel's deadly complaints at the root:
- Doing nothing loses, because the storm's charge is the mana and it overflows onto your magazine.
- Random play loses, because an unshaped bolt goes exactly where the overflow would.

Every card pulls a lever the bolt really reads (water conducts, glass blocks, fire draws strikes, wind carries the trail), so combinations come from the world. The network part is real but short-lived. The ionised haze is a diffusing field the next bolt grows along, and scorched copper paths stay, so over a level you build a wiring tree out of your own past strikes. It is also the cheapest to run: the field is only worked out on a cast.

---

## Prior art

- **Creeper World 4** (Knuckle Cracker). The enemy is a fluid that floods the map, so doing nothing always loses, and a front you can see reads at a glance. https://knucklecracker.com/creeperworld4/cw4.php
- **Pipe Mania / Pipe Dream.** The flow is the clock, and you build just ahead of it. A threat that advances along *your* network makes every placement tense. https://store.steampowered.com/app/1385320
- **Mini Metro.** A network grown under rising pressure ends with a score, not a win or loss, and hub versus grid becomes strategy with no tutorial. https://humantransit.org/2014/03/the-lessons-of-mini-metro.html
- **Mushroom 11.** You shape a growing mass only by cutting it away. Cutting a growth to steer it is a satisfying verb, which supports Cut in Veins and Drill in Fault. https://store.steampowered.com/app/243160
- **The Powder Toy.** Electricity as cell rules (spark, conductor, insulator, one-way metals) multiplies with every material. Arc's rules can be few if the materials do the work. https://powdertoy.co.uk/Wiki/W/Element:SPRK.html
- **Balatro.** Cards score left to right, adds before multipliers, so order is visible grammar. A big readable number at the end makes a combo feel earned. https://www.pcgamer.com/balatro-tips/
- **Magicka.** Elements combine by plain rules (water and fire make steam) that players learn from the names alone. That is the jargon-free card language the satchel lacked. https://en.wikipedia.org/wiki/Magicka
- **Oxygen Not Included.** Gas diffusion is gameplay only when the player can see it. Overlays are not optional for an invisible field. https://oxygennotincluded.wiki.gg/wiki/Atmospheric_Pressure
- **Terra Nil.** A chain of world transformations (burn, then rain, then grow) is scored on a clean finish, and an ordered chain where each step enables the next feels like a puzzle. https://mechanicsofmagic.com/?p=55125

---

## 6. Mould: garden a slime mould into the best possible network

*Added at the owner's request (slime mould or mushroom mycelium). The brief's no-living-things rule is waived here, because the organism is the network.*

**Pitch.** A yellow slime mould spreads across a dish, pulsing, hunting for food. You steer it with food, light, salt and scissors until it grows a network that is short, fast and survives being cut. It is the Tokyo rail experiment as a puzzle.

**Who the player is: the gardener.**
- As the organism, you would aim every tip yourself, and the surprise would be gone.
- As its rival, you would only destroy.

As the gardener, your cards change the dish and the mould decides what to do about it. That is the Noita shape: you cast into a world that answers. Mushroom 11 shows that steering a growth you do not control is satisfying in itself.

**Core loop (30 seconds).**
1. The mould throbs: a bright pulse sloshes along every tube about every 2 seconds, and you cast on the beat.
2. You drop Oats. Within two pulses the nearest fan of front bends toward it, finds it, and a tube to the food thickens and brightens.
3. The side branches that carry nothing fade, go pale and are drawn back in.
4. You switch on a Lamp. The mould pulls its veil out of the light, and the tube under the lamp thins.

**The simulation.**
- **Existing pieces:**
  - the light field (the lab already has a Lamp tool);
  - ground moisture (the mould spreads fast on damp and stalls on dry);
  - the diffusing scalar field (food breathes a scent the front climbs);
  - weather wind;
  - decay (uneaten oats go off).
- **The tube network (new).** Each tube's thickness grows with the flow through it and shrinks without it, which is the published Physarum model. Unused tubes prune and loops that earn their keep stay, and that is what draws the shortest-path networks. The flow is solved on the graph of junctions every few frames, not per cell, to keep frame cost down.
- **The pulse (new, cheap).** The sloshing is mostly drawn on screen: a travelling brightness wave on the tubes. It also carries anything dropped into a tube, and it is the game's beat.
- **Slime trail (new scalar, slow to fade).** Ground the mould has crossed and left keeps a pale crust, and the front avoids it, which is how it remembers where it has been. Water washes the crust away.
- **Fusion.** Two parts of the mould that meet fuse into one, and their food is shared.
- **Fruiting.** A region whose food runs out gathers into stalks and releases spores, which the wind carries. Death is graded: it turns into the next generation, and it never just vanishes.
- **Fairy rings for free.** A blob in an even field eats out its centre and spreads outward as a ring.

**Cards.**
1. **Oats.** Food at the cursor. It attracts the mould and pays it reserve when reached.
2. **Lamp.** Bright light. The mould flees it, and tubes under it thin.
3. **Salt.** A line the front will not cross. Tubes touching it shrivel slowly.
4. **Damp.** Wets the ground so growth there runs fast.
5. **Wash.** Water that rinses away slime crust and food scent, so the mould will explore there again.
6. **Snip.** Cuts a tube.
7. **Wide** (modifier). The next card covers twice the area.
8. **Along the tubes** (modifier). The next card goes *into* the network at the cursor and rides the pulse to wherever the flow is heading.
9. **Both at once** (multicast). The next two cards land on the same beat.
10. **Buried** (trigger). Hides the next card in the dish. It fires when the mould's front reaches it.

**The surprising combo: Snip your own trunk beside a salt wall.** Two oat piles sit behind a salt wall the mould cannot cross.
1. Snip cuts the main tube to the branch along the wall. That branch now has no supply, so its reserve drains.
2. At zero it does not vanish: it fruits, and a row of stalks rises.
3. The stalks release spores, which the wind carries over the salt.
4. The spores land on damp ground on the far side, sprout, and a new front grows and finds the oats.
5. If any gap joins the two halves (a Wash through the salt, say), they fuse and share food across it.

Cutting off part of your own mould is how it got over a wall nothing could cross. The cut, the life cycle and the wind combined, and no card says "jump".

**The challenge.** The mould starts with a fixed food reserve, and every cell of veil and tube costs upkeep.
- **Doing nothing loses.** Left alone it spreads a wide, even sheet that burns the reserve before it finds the far food. The oats go off while it wanders, and the level hides food where blind spreading reaches it last.
- **Random play loses.** Oats in the wrong place grow tubes to nowhere, a stray Lamp cuts the route you need, and a stray Snip starves a branch.
- **The finish is a test.** At the end comes "the storm": three random tubes are cut, shown with red marks. A tree-shaped network loses whole regions. A planned one with a few loops keeps everything fed.

**Graded outcome.** Three stars, earned separately, using the plain-word versions of the three numbers the Tokyo experiment measured:
- **Reach:** food piles joined.
- **Thrift:** total tube length, against the shortest possible.
- **Toughness:** food still joined after the storm.

The mould then fruits, and the number of stalks (how well fed it was) carries into the next dish as a bigger starting reserve. So there is always a next thing to do.

**Sample level ("The Bay").**
- **Dish:** 512x320 of damp agar.
- **Bare dry band:** x=0-512, rows 0-20, plus a dry "bay" (a U-shaped region from x=200 to 330 open at the top, down to row 200) the mould grows across only very slowly.
- **Mould:** starts as a blob at x=120, y=160, with reserve 100.
- **Food:** nine oat piles around the bay.
- **Salt wall:** at x=430 from row 40 to row 300, with the two far-right piles behind it and a breeze blowing to the right.
- **Timing:** the oats go off over 4 minutes, and the storm comes at 4:00.
- **Wand:** 5 slots, one card per pulse, with the deck Oats, Oats, Lamp, Salt, Damp, Snip, Along the tubes, Both at once, Buried.

**Risk.** The mould is slow and has its own mind, so the game turns into watching, which is the satchel's last complaint.

How a prototype finds out:
- Time from Oats to a visible bend. It must be at most two pulses (about 4 seconds).
- Draw tube thickness with strong brightness so pruning reads at phone size.
- In five paired runs per seed, the planned network must beat blind growth by at least one star.

**Smallest prototype (4-5 days):**
- tube cells with thicken and prune on a junction graph;
- the pulse;
- the front climbing the food-scent field;
- crust avoidance;
- four cards (Oats, Lamp, Snip, Along the tubes);
- a fixed three-cut storm and the three-star score.

Publish a page with a paired GIF, blind against planned, with the counts beside each: piles joined, tube length, piles fed after the storm, and the `ascii` worst frame.

**How it differs from Veins, and which should absorb which.**
- **What Veins is:** two abstract networks fighting over territory. It is a strategy race, and the tips go where the lures send them.
- **What Mould adds:**
  - one organism with its own mind: memory (the crust), a rhythm (the pulse) and a life cycle (starve, fruit, spores);
  - a puzzle about the *quality* of a network rather than about territory.

**Mould should absorb Veins.** Veins' rules (thicken with flow, prune, climb a gas gradient, fuse) are a subset of Mould's. A rival can come back as a later dish, with a mushroom mycelium fairy ring spreading from the centre and eating the oats first.

In my ranking, Mould replaces Veins at number 2. Arc stays first because its verb resolves in under a second. Mould's charm is greater, but so is its risk of being watched rather than played.
