# All explorer results, compiled (2026-11-07)

Four explorers worked independently from brief.md (in this folder): A flows and diffusion, B growth networks (plus section 6, Mould, added at the owner's request for a slime mould or mycelium game), C open (falling sand allowed), D card-first (whole-run structure). 21 concepts in all. Each explorer's file follows unchanged.


<!-- ===== explorer-a-flows.md ===== -->

# Explorer A: games built on flows and spreading fields

*7 Nov 2026. Five concepts where something moves or spreads (gas, air, water, charge, dye, damp air), and the cards push it around. No falling sand, no plants, no creatures.*

## The card frame all five share

Read this once; each concept only lists its own cards.

- **The row.** You hold an ordered row of up to six cards. Each click plays the next card where you clicked. When the row runs out it refills from the start after a short pause (a 3-second bar). You arrange it before the level and at one pause per level.
- **Modifiers** (Wide, Strong, Long) change the next card and are free: the row moves on to it in the same click.
- **Multicast** (Pair, Trio) plays the next two or three cards at once, on the same spot.
- **Triggers** (Fuse) drop a visible marker carrying the next card. It goes off when a plain-words condition happens at the marker ("when the gas here can burn", "when lightning hits this").
- **One cap.** A trigger set off by another trigger counts one deeper, and at depth three it fizzles with a grey puff. That is the only rule that stops loops.
- **The screen always says the next click in words**: "Next click: Wide Puff, then Spark", with a ghost outline of its size. Every card ends as something you can see.

---

## 1. Firedamp

**Pitch:** a mine is filling with gas. Push it, seal it, and burn it off in small controlled pops before it reaches the miners' lamps.

**Core loop (30 seconds).** Gas seeps from cracks as a slow yellow-green haze. It is heavier than air, so it pools in the low galleries like slow water and creeps along the floor. You click a Puff at a gallery mouth and the haze bends and streams away. You drop a Spark into a pale fringe, and a blue flame front runs along the haze edge, leaving smoke and warm air that rises up the shaft. A thick pocket goes off with a thump that cracks a thin wall, and now gas pours through a new hole.

**The simulation.**
- *Exists:* the coarse air grid with pressure and wind, which already carries smoke and heat sideways; fire; explosions with fracture; structural collapse; smoke that thins out.
- *New:* a gas concentration field on the fine grid. It would reuse the machinery of the existing spreading-and-fading fields, plus two additions: wind carries it, and a slow downhill drift makes it pool.
- *New:* a **burn band**. Gas that is too thin will not burn, gas that is too thick will not burn either, and only the middle burns. The flame front eats concentration as it goes. A thick enough burnable pocket hands off to the existing explosion code, so the size of the bang follows how much gas was in it.
- *Cost:* gas keeps awake only the chunks it is in; gas leaving the shaft is deleted.
- **The haze is drawn in three fixed colours**: pale for too thin, yellow for burnable, deep green for too thick.

**Cards (8):**
- **Puff:** a burst of air outward from the click.
- **Draw:** pulls air toward the click.
- **Spark:** a pinpoint flame.
- **Shutter:** a stone door, 8 cells wide.
- **Mist:** a water spray. Flame cannot cross wet cells, so it makes a firebreak.
- **Warm:** heats a patch of rock so air rises from it, a little chimney.
- **Chill:** cold air that makes gas sink and pool faster.
- **Modifiers:** Wide (twice the size), Long (lasts three times as long).
- **Multicast:** Pair.
- **Trigger:** Fuse ("goes off when the gas here can burn").

**The combo: Fuse + Spark, then Puff.** You set a Fuse carrying a Spark deep inside a dark green pocket, then Puff at the pocket's mouth.
1. The pocket is too thick to burn, so the fuse just sits there. A random player who sparks it directly sees nothing happen.
2. The Puff stirs air into the pocket's outer edge, and at the fuse the gas thins into the yellow band.
3. The fuse fires. Flame runs around the stirred edge only, because the core is still too thick.
4. Burning warms the air, which rises up the shaft and draws fresh air in along the floor (to check: whether heat already moves the air grid this way). That thins the next layer of the core into the band, and it burns too.
5. A fire "breathes" its way, pulse by pulse, into a pocket you could not light, and the last pulse sets off the next gallery's yellow fringe with a bang you never aimed, sometimes next to a lamp.

**The challenge.** The seams never stop leaking. Each work face has a lamp, and ore carts roll out of a face while its lamp is lit.
- If burnable gas reaches a lamp, that face blows. You see a real explosion, the face collapses and it is lost.
- **Doing nothing** loses inside about 40 seconds, because the gas pools down to the lamps on its own.
- **Random play loses too**: sparks in thick gas do nothing, sparks near lamps light their fringe, random Puffs push haze onto faces. A plan reads the colours, vents, shutters, and burns the fringe far from any lamp.

**Graded outcome.** The score is ore carts hauled in 3 minutes. A face with gas near it goes dark for a while (the miners down tools), and a face that blew is gone. One star for surviving with one face working, two for all faces, three for the ore target too. A small bang that only singes a face costs it ten seconds, not the level.

**Sample level, "The Dip"** (256x160).
- **Layout:** a fresh-air shaft at the top left, open to the sky, drops into a U-shaped gallery. The bottom of the U is a sump 30 cells below the entrance, and a crack in the sump floor leaks gas steadily. On the right the gallery climbs to two work faces, one high and one low, each with a lamp. A thin rock wall, 3 cells thick, separates the sump from a sealed empty void below and to the left.
- **Start:** no gas yet. The first haze reaches the low face's lamp at about 35 seconds if you leave it.
- **Hand:** Puff, Puff, Spark, Fuse, Wide, Warm, Shutter.
- **Goal:** 30 carts in 3 minutes.
- **Intended solves:** Warm at the shaft foot makes a chimney that drags the haze up and out. A Shutter across the climb protects the low face. A Wide Spark at the thin wall when the sump is yellow blows the wall and dumps the sump's gas into the void.

**Risk.** The burn band is invisible logic: players don't know why one Spark fizzled and the next boomed. The fixed three-colour haze is the fix. Find out by asking a fresh player, before each Spark, "what will happen?" and counting how often they are right. Below about half right after five minutes, the band is not readable.

**Smallest prototype (5 days).**
- Days 1–2: the gas channel (spread, wind carries it, downhill drift) with the three-colour tint, plus one leaking crack.
- Day 3: Spark and the burn band, with the flame front and the hand-off to the existing explosion.
- Day 4: Puff and Draw as pushes on the air grid.
- Day 5: the row and "The Dip".
- The one question it tests: does "too thick to burn, so thin it first" produce an *aha* in the hand?

---

## 2. Arc

**Pitch:** a storm cloud is charging over a valley. Shape the ground and air so the lightning strikes your beacons and not the village.

**Core loop.** A charge bar on the cloud fills. You soak a strip of ground, which darkens, or blow a puff of steam that drifts and glows faintly blue where the charge is strong. When the bar fills, a bolt grows down from the cloud in a split second, forking and feeling its way along whatever conducts best. Where it lands, oil catches, water boils to steam and stone glazes. Its branches stay burned on the screen for a few seconds as a picture of the path the field chose.

**The simulation.**
- *New:* a charge field solved by relaxation on the coarse grid: every cell drifts toward the average of its neighbours, so it is a diffusion field. Conductivity comes from materials: water, wet ground, metal and steam conduct; dry rock, ice and oil don't.
- *Exists:* the moisture field, which spreads and dries and can stand in for "wet ground conducts".
- *New:* the bolt is a **growing tip**. Each step it extends toward the neighbour with the steepest charge drop, plus some randomness, and it branches when two neighbours are close. That is the plant code's tip-and-branch idea used abstractly.
- *Exists:* fire, steam (which rises and, under open sky, joins the weather), and explosions, for a struck store of oil.
- *Cost:* the field only needs solving in the second before a strike, so it costs nothing while the bar fills.

**Cards (8):**
- **Soak:** rain on a patch of ground.
- **Steam:** a rising plume that conducts.
- **Rod:** an iron spike.
- **Dry:** heats a patch until it stops conducting.
- **Ice:** an insulating sheet.
- **Brine:** salty water that conducts three times better.
- **Discharge:** strike *now*, whatever the bar says.
- **Modifiers:** Wide, Tall (a Rod or Steam twice as high).
- **Multicast:** Pair.
- **Trigger:** Fuse ("when lightning hits this").

**The combo: Pair (Steam, Steam), then Discharge, with the bolt landing near the lake.** You cast two Steam plumes in the gap of a chasm, between the lake cliff and a beacon on the far rim, then Discharge.
1. The steam is the best conductor in the sky, so the bolt runs sideways through the plume instead of straight down, and lights the far beacon.
2. The bolt's tail grounds at the lake edge, and its heat boils a patch of water.
3. The new steam rises *beside* the old plume. The cloud still has charge left, because a Pair-built path drains it only partly.
4. A second bolt you did not cast follows the new plume down into the lake. The lake is next to the village's boathouse.

**The challenge.** Left alone, the bolt strikes the tallest conductive thing, which is the village's copper bell tower, and the tower burns. The beacons only light on a hit. Random soaking makes the whole valley conduct, so bolts spray everywhere. A plan builds one clean path per strike.

**Graded outcome.** Beacons lit out of five, minus village cells burnt. Three stars for all five beacons and nothing burnt. A branching near-miss often lights a beacon *and* singes a roof, so there is a middle.

**Sample level, "Bell and Beacons"** (320x200).
- **Layout:** the cloud is at the top. The village is at the bottom left, with a bell tower 40 cells tall. A lake is in the middle. A 30-cell chasm sits right of the lake, with beacon 1 on its far rim. Beacon 2 is in a dry rock bowl at the bottom right.
- **Hand:** Soak, Steam, Steam, Rod, Dry, Pair, Discharge.

**Risk.** The bolt is too random to plan, and players feel like spectators. Test it by running one setup over 10 seeds and counting how often the bolt lands on the intended target. Under about 70%, turn down the randomness. Over about 95%, it is boring.

**Smallest prototype (4 days).**
Charge field over a hand-drawn conductivity map (day 1), the growing bolt (day 2), Soak, Steam, Rod and the burn mark (day 3), the level (day 4).

---

## 3. Floodgate

**Pitch:** a mountain lake is filling with rain and will overtop onto the town below. You cannot destroy water, only send it somewhere.

**Core loop.** Rain fills the lake and its level line creeps toward the lip. You pour Lava into the outflow throat: it hisses, steam billows, and a crust of new stone builds and sinks to the floor. You Freeze the narrow neck and a floating crust forms. You Blast a side wall and a white ribbon of water pours into a dry valley, turning pale and foamy as it runs. Each card visibly bends where the water goes.

**The simulation.** Almost all of it exists:
- liquids that flow and level, and moving water that runs white;
- lava and water meeting to make stone that sinks, and steam;
- steam that, under open sky, joins the weather and comes back as rain;
- ice that floats and bears weight;
- explosions with fracture, and structural collapse.

New: a rain source over the lake, and a town of house cells that count as flooded when water stands in them.
Cost: a big moving body of water keeps its chunks awake, so the levels must be sized against the `ascii` worst-frame number.
Rubble from blasts should be kept to blocks and cobbles, not sand.

**Cards (8):**
- **Lava:** a short pour.
- **Freeze:** a cold patch that crusts the water over.
- **Boil:** heats a spot until the water steams off.
- **Blast:** a small charge.
- **Oil:** a floating slick.
- **Spark.**
- **Wall:** a stone block.
- **Modifiers:** Wide, Long.
- **Multicast:** Pair.
- **Trigger:** Fuse ("when water reaches this").

**The combo: Boil the lake, and it rains on the town.** You Wide-Boil the lake to lower it fast. It works: the level visibly drops. The steam rises into open sky, joins the weather and, a minute later, falls as rain over the town, downwind. The engine keeps the water you boiled, so it lands somewhere. One play teaches the game's one law. The fix is a second step: Boil under the rock overhang on the left, where steam condenses and drips back onto the uphill side, into the dry valley.

**The challenge.** Rain never stops, so doing nothing floods the town. Random Blasts open the wrong wall and Lava in deep water builds a sill that *raises* the level. A plan routes water to the dry fields and the mill wheel.

**Graded outcome.** The score is houses dry at the end, plus water delivered to the fields (each field counts as watered once it is soaked). Stars at 60%, 85% and 100% dry.

**Sample level, "The Lip"** (320x200).
- **Layout:** the lake fills the top middle with its lip on the right, and the town is down and to the right. A dry valley with two fields lies left, behind a 4-cell rock wall. An overhang sits above the lake's left shore.
- **Start:** rain overtops the lip in about 50 seconds.
- **Hand:** Blast, Lava, Freeze, Boil, Wide, Fuse, Wall.

**Risk.** It feels like an existing mobile game with cards stuck on, and the steam detour is too slow to connect to its card. Draw a thin line from the boiled spot to the cloud; in the prototype, ask players *why* the town got wet.

**Smallest prototype (3 days),** the cheapest of the five: the level, a rain source, a flooded-house counter, and cards wired to existing brushes.

---

## 4. Tincture

**Pitch:** mix the colour each bottle orders, in a river that never stops flowing.

**Core loop.** A slow river runs left to right past bottle mouths on the bank, and each bottle shows its order as a colour swatch. You drop Red upstream and a ribbon of red unrolls and smears downstream. A Stir makes a visible whirl that folds red into the blue next to it, and the bottle mouth turns purple. The bottle fills, and a meter shows how close the colour is.

**The simulation.**
- *New:* three dye concentration fields (red, yellow, blue) on the fine grid. They reuse the spreading-and-fading machinery, and the water's current carries them.
- *Exists:* the coarse air grid's flow solver, which could be run over water cells to give a current. That is an assumption to check.
- *New:* density. Salty dye sinks and lies along the bed. Heat warms the water, and warm water rises.
- 
**Cards (9):**
- **Red, Yellow, Blue:** a drop of dye.
- **Stir:** a whirlpool for 3 seconds.
- **Jet:** a push of current.
- **Salt:** the next dye sinks.
- **Warm:** a heated patch of bed that lifts water.
- **Bleach:** clears a patch.
- **Weir:** a low stone sill.
- **Multicast:** Pair.
- **Trigger:** Float ("drifts with the current; opens when it touches another colour").

**The combo: Salt Blue upstream, Warm downstream.** The blue vanishes. It is sinking to the riverbed and creeping along under the current, unseen beneath clear water. At the warm patch 60 cells downstream the water rises and lifts the hidden blue to the surface, right where your red was floating. The bottle there turns purple, out of an apparently empty river. 

**The challenge.** The current washes everything to a grey drain at the right edge. Doing nothing fills the bottles with clear water. Random drops make brown. A plan keeps colours apart until the mouth they are for.

**Graded outcome.** Each bottle scores 0–100% by colour match. The level score is the average, with stars at 50, 75 and 90.

**Sample level, "Three Mouths"** (320x120).
- **Layout:** a river 40 cells deep runs left to right, with bottle mouths at x=100 (orange), x=180 (purple) and x=260 (green).
- **Hand:** Red, Yellow, Blue, Stir, Salt, Warm, Pair.

**Risk.** It is calm rather than thrilling, and mixes go brown so fast that no plan survives. Test whether players can hit 75% on purpose in three tries.

**Smallest prototype (4 days).** A fixed, hand-drawn current field (no solver yet), three dye channels, four cards, one level.

---

## 5. Saltworks

**Pitch:** dry a tidal pool into salt, and grow the crystals into a causeway before the tide comes back.

**Core loop.** Sun-heated pools steam faintly. As the water level drops, the brine turns cloudy, and then white crystals spike out from the edges and grow toward each other like frost. You Wind the damp air out of a cave, and that pool starts drying fast. Every 60 seconds the tide surges in and dissolves thin crystals.

**The simulation.**
- *Exists:* evaporation and drying of standing water; the moisture field, which spreads and is carried by wind; weather and heat.
- *New:* a salt amount carried in each water cell, which rises as the water leaves. Air that is already damp slows drying, which may need tying to the existing moisture field.
- *New:* crystal growth. Tips grow into water that holds too much salt and branch at random (growth as the supporting idea), and the crystals are solid cells the existing load rules can bear weight on.
- *New:* a tide boundary.

**Cards (8):**
- **Sun:** heats a pool.
- **Wind:** blows damp air away.
- **Shade.**
- **Seed:** a grain that starts a crystal here.
- **Brine:** pours very salty water.
- **Fresh:** pours fresh water, which dissolves crystals.
- **Dam.**
- **Cold:** crystals drop out faster.
- **Modifiers:** Wide.
- **Multicast:** Pair.
- **Trigger:** Fuse ("when the tide reaches this").

**The combo: Pair (Wind, Sun) on a pool under a cave roof.** The cave's air is damp, so the pool barely dries. Wind sweeps the damp air out, and the water nearest the cave mouth meets the driest air first. Crystals start on the windward rim and grow *into* the wind as a ridge. The ridge becomes a dam that splits the pool in two, and the back half, now sheltered, stops drying. The wind, not your click, set the shape.

**The challenge.** The tide returns every 60 seconds and dissolves thin work, so doing nothing builds nothing. A cart must cross the causeway to score.

**Graded outcome.** Carts crossed before the third tide, plus how much of the causeway is still standing at the end.

**Sample level, "Low Water":** two rock shelves 80 cells apart, with five shallow pools between them, one of them under an overhang. Hand: Sun, Sun, Wind, Seed, Brine, Pair.

**Risk.** Too slow: drying takes minutes even sped up, and cause far from effect is unreadable (the satchel review's warning). Test whether each card shows within 3 seconds.

**Smallest prototype (5 days),** the most new work of the five.

---

## Ranking

1. **Firedamp.**
2. **Arc.**
3. **Floodgate.**
4. **Tincture.**
5. **Saltworks.**

**Why Firedamp will be the most fun to play.**
- **Dread, then release.** You *watch* the yellow haze creep toward a lamp, which is dread you can see, and you answer it with a click that makes the haze visibly swerve. When you finally light it, the payoff is a flame racing through gas you herded there, sometimes ending in a bang that cracks the rock open.
- **The rule is learned in one play.** The burn band gives one rule a beginner learns from one fizzle: too thick won't burn, so thin it first. Every clever play after that is built on it, and that is exactly the "random loses to a plan" property the satchel lacked.
- **The threat never stops.** It comes from the world rather than a timer, the seams keep leaking, so there is always a next thing to do.
- **The world multiplies the cards.** Fire, wind, heat and fracture are the engine's own levers, and the breathing fire and the bang next door are outcomes nobody wrote.
- **The result has a middle.** Lost carts, a dark face or one blown face, rather than win or lose.

Arc is second: the most spectacular moment of the five, resolving in a second as Noita's spells do, but it depends on a forking bolt being aimable. Floodgate is the cheapest to build and the least surprising.

---

## Prior art

- **Creeper World** (Knuckle Cracker): the enemy is a fluid spreading over terrain, so high ground and choke points come from the sim, not from scripted waves. https://knucklecracker.com/creeperworld4/cw4.php
- **PixelJunk Shooter** (Q-Games, 2009): water, lava, magnetic liquid and gas that react with each other. Reviewers loved the fluids and found the shooting secondary, so the reactions *are* the game. https://www.giantbomb.com/games/3030-26101/
- **Oxygen Not Included** (Klei): gases spread, displace each other by pressure and pool by weight, and managing invisible air is tense when it is drawn as an overlay. https://oxygennotincluded.wiki.gg/wiki/Atmospheric_Pressure
- **The Powder Toy:** a real air-pressure and wind grid with fans. It is the model this engine's air grid was built from, and shows how much play a pushable air field gives. https://powdertoy.co.uk/Wiki/W/Walls.html
- **Where's My Water?** (Creature Feep / Disney, 2011): routing a fluid, with three ducks per level as the graded outcome, and steam that rises instead of falling. It shows a fluid puzzle needs collectibles along the path, not just a goal at the end. https://en.wikipedia.org/wiki/Where%27s_My_Water%3F
- **SpaceChem** (Zachtronics, 2011): an ordered program of instructions runs and you watch it play out. It is the "plan, then watch the world run your plan" feeling. https://en.wikipedia.org/wiki/SpaceChem
- **Magicka** (Arrowhead, 2011): elements combine into new elements (water and fire make steam), and opposites cancel. Combination rules make a handful of cards into dozens of effects. https://en.wikipedia.org/wiki/Magicka
- **Balatro** (LocalThunk, 2024): cards resolve left to right, so order changes the result, and multipliers make combos feel explosive. https://thinkygames.com/games/balatro/
- **Slay the Spire** (Mega Crit, 2019): the deck-builder template of a small hand, cards added between fights, and a threat that escalates. https://store.steampowered.com/app/646570/Slay_the_Spire/


<!-- ===== explorer-b-growth.md ===== -->

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


<!-- ===== explorer-c-open.md ===== -->

# Explorer C (open brief): five card-and-sand puzzle games

*2026-11-07. Falling sand allowed for this explorer. Three of the five (High Tide, Landslide, Fulgurite) lean on piles, pours and slumps, which the other two explorers cannot use. All five keep the brief's other rules: no plants or creatures, a wand of cards, a puzzle or challenge.*

## Rules all five share

These fix what killed the satchel, so they are stated once.

- **The wand is a strip of 4 to 6 slots on screen.** One press plays the outlined card. Some cards pull more cards in with them. There are three kinds, each named in plain words:
  - *"changes the next card"*, a modifier (Wide, Twin, Heavy);
  - *"plays two at once"*, a multicast;
  - *"carries the next card until…"*, a trigger (on landing, when the water touches, after 3 seconds).
- **A line above the strip says, in world words, what the next press will do**: "Next: pour a wide heap of wet sand." A ghost outline on the map shows where.
- **Every card ends as cells**: sand, water, fire, glass, stone, a hole. Nothing ever just changes a hidden number.
- **One visible cap.** A chain of carried cards stops at depth 3 with a puff of grey dust, as Noita's recursion limit does.
- **A threat on a clock that the world does not beat by itself.** Tide, flood, lava, gas or storm. Doing nothing always loses.
- **Each run shows counters beside the picture**: cells moved, the depth a chain reached, and the score.

---

## 1. High Tide

**Pitch:** build a sandcastle with a wand, then hold it against a tide that comes back higher each time.

**30 seconds:** the tide is out. You stamp two wet towers, dig a moat in front of them and bake the seaward wall into glass. A wave rolls in. Dry sand washes off in pale streaks, and wet sand holds a little longer. A tower whose foot is cut slumps sideways, and its flag drops. The water draws back and leaves a darker, flatter beach. A bar fills toward the next tide. You rebuild.

**Simulation**
- *Exists:* sand piling at its own angle; water flowing and levelling; splashes when sand or a boulder hits water; fire; structural collapse for anything solid left hanging.
- *New:*
  1. **Wet sand stands steep.** Its slope angle rises with how wet it is, then falls back as it dries over about 20 seconds, so towers slowly sag. The friction angle is already a per-material number, so this is one wetness value per sand cell read by the slope rule.
  2. **Moving water carries loose sand.** Moving water already has its own flag (it draws white). Give it a chance to swap with a dry sand cell beside it, and a smaller chance for a wet one.
  3. **The tide.** The water level at the right edge rises and falls on a timer.
  4. **Fire turns sand to glass.** Glass is a new solid modelled on `crystal`, and it breaks into `shard`.

**Cards**
- Pour Sand
- Bucket Tower (a column of wet sand)
- Wet
- Dig
- Bake (sand to glass)
- Wide (changes the next card)
- Twin (changes the next card: it also lands at the mirror point)
- Two at Once
- When the Tide Touches (carries the next card until seawater reaches that spot)
- Wait 3 s

**The combo: Bucket Tower, then *on landing* Bake, beside a second tower.** Bake fires where the tower lands, but only its top 4 rows have time to glaze before the fire dies. That leaves a glass cap on a wet sand stalk. The tide cuts the stalk, and the sand under the cap washes out. The cap is now a solid with nothing beneath it, so the collapse rules decide its fate:
- **It touches the next tower along a wide enough edge:** it hangs there as a glass arch, the sea runs under it, and the flag stays up.
- **It doesn't:** it drops as a slab and throws the boulder-style splash up both sides. That water lands on the dry keep behind, and wet sand stands steeper. **Losing the outer tower saves the inner one.** Nobody wrote that rule. It comes from splash plus wetness.

**Challenge**
- *Doing nothing:* the first tide flattens the beach. Zero flags.
- *Random play:* wet sand cast too early has dried by the time the tide arrives. Bake cast before shaping glazes the wrong place. A moat dug on the landward side fills from behind.
- *A plan:* order and timing matter. Wet goes last, just before the tide. Bake comes after shaping. Triggers do the repairs mid-wave.

**Graded:** one point per flag still standing after each tide.
- 1 star: survive tide 1.
- 2 stars: survive tide 3.
- 3 stars: still hold the keep at tide 5.
- Endless mode scores the highest tide you outlast.

**Sample level:** a 320x160 beach sloping down to the right. Sea at x > 240. The tide reaches x = 200 on tide 1 and 20 cells further each time after. Three flag posts at x = 110, 140 and 170: a flag counts if 15 cells of sand stand under it. Each tide gives 40 seconds to build. The bag holds 600 sand cells, and Dig refunds what it removes.
- Wand: Bucket Tower, Wet, Two at Once, Dig, Bake.
- Spare: Wide, Twin, When the Tide Touches.

**Risk:** the tide reads as a smear and slumping reads as noise, not as a tower failing. Erosion is all moving water, so a big tide keeps chunks awake, and the frame cost has to be quoted. A prototype finds out with a GIF of one tower drying and one tide, plus `ascii` worst-frame numbers taken during a wave.

**Smallest prototype (3 days):** wetness-dependent slope angle, a tide at the right edge, a hard-coded 4-slot strip of Bucket, Wet, Dig and Bake, and a flag counter. No glass physics beyond "solid, falls when unsupported".

---

## 2. Quench

**Pitch:** a lava fall pours into a gorge. Turn it into the stone dam you need before the meltwater flood arrives.

**30 seconds:** lava crawls down from a crack. You throw a ball of water at the front of the flow:
1. It hisses.
2. A crust forms cell by cell, crumbles to rubble and sinks.
3. Steam rises. Under the rock roof it beads and drips straight back down, quenching more lava.

The dotted outline of the dam fills slowly from the bottom. Halfway through the round the flood is announced. When it comes, it tests what you built.

**Simulation**
- *Exists:* almost all of it. Lava and water make stone and steam where they touch. The crust crumbles to rubble and goes to the bottom. Steam under a roof drips back where it condensed. Rubble piles at gravel's angle. Sand, and water levelling.
- *New:* only the flood source (a timed spring at the top-left) and the outline counter.

**Cards**
- Splash (a ball of water)
- Drizzle (a wide, thin shower)
- Lid (a stone slab placed as a roof)
- Pour Sand
- Dig
- Wide
- Heavy (the next pour sinks faster)
- Two at Once
- When It Steams (carries the next card until steam reaches the spot)
- Wait 3 s

**The combo: Lid over the lava pool, then one Splash under it.**
1. The Splash quenches the surface. Steam rises, hits the lid and condenses.
2. Indoors, condensed steam drips back exactly where it formed: onto the lava under the lid. More quench means more steam, and more steam means more drips.
3. One card of water becomes a quench engine that keeps running for many seconds.
4. The rubble it makes sinks straight down, so the dam's footing lands exactly under the lid.

**You aim the dam by aiming the roof, not the water.** It is true to the engine as shipped.

**Challenge**
- *Doing nothing:* the lava runs over the spillway into the village row, and the flood then has no dam to stop it.
- *Random play:* Splash cast in the open sends its steam off as weather, so the water is gone for good. Sand dropped on lava is swallowed, because lava sinks through sand. Lids placed over nothing waste the round.
- *A plan:* lids first, water second, and When It Steams placed where the drip will land so that a second Splash chains on.

**Graded:** the share of the outline filled, then the share of village cells kept dry in the flood test. Three stars for 90% filled and the village dry. The middle results come naturally: a short dam that the flood overtops, or a leaky dam of loose rubble.

**Sample level:** a 256x200 gorge. A vent at the top-left pours lava at 3 cells per tick for 90 seconds. A rock overhang spans x = 60–120. The dam outline sits at x = 140–170, from the floor up to y = 60. The village row sits beyond x = 200. The flood comes at 100 seconds.
- Wand: Lid, Splash, When It Steams, Splash.
- Spare: Drizzle, Wide.
- Mana is a 400-cell water tank. It refills a little from steam that condenses into a cistern.

**Risk:** the quench engine is too slow, or too fast, and reads as nothing happening under a rock. Before anything else, measure how long one Lid plus Splash keeps going, and how many rubble cells it makes.

**Smallest prototype (2 days):** the scene above, three cards, the outline counter and the flood. Nearly no new simulation, which is why it is the cheapest test of whether combos plus chemistry are fun.

---

## 3. Fulgurite

**Pitch:** steer lightning through a sand dune to fuse glass pipes inside it. Then pull the plug, let the loose sand pour out, and see whether water runs through what you made.

**30 seconds:**
1. You drag a wet streak down through the dune, and a dark line soaks in.
2. You cast Strike. A bolt grows down from the cloud, branching, and follows your damp line. A glowing glass tube is left where it passed.
3. A storm strike you did not call lands somewhere else and fuses a junk branch.
4. You cast Pull the Plug. The floor opens, and the whole dune drains out like an hourglass in a roaring sheet. Glass stands where the bolts went, and thin branches snap off and fall as shards.
5. You pour water in at the top.

**Simulation**
- *Growth (new, reusing the plant code's tip-and-branch mechanics in the abstract):* a bolt is a growing tip that steps toward the ground. It prefers wet cells (moisture is an existing per-cell field) and forks at random. It ends on reaching a rod or when its energy runs out.
- *Glass (new):* sand the bolt passes through becomes a 3-wide glass wall with a 1-cell hollow core.
- *Exists:* sand draining through an opening; the collapse rules, which decide which glass branches hold once the sand around them has gone; water flowing through the channel.

**Cards**
- Wet Streak (drag)
- Salt (one spot the bolt strongly seeks)
- Rod (the bolt ends here)
- Strike
- Fork (changes the next card: more branches)
- Straight (changes the next card: fewer branches)
- Two at Once
- When the Bolt Lands (carries the next card to the bolt's tip)
- Pour Sand (refill)
- Pull the Plug

**The combo: Strike, then When the Bolt Lands: Fork + Strike.**
1. The first bolt follows your wet streak to the salt spot.
2. There, deep inside the dune, the payload fires a second bolt that branches more.
3. There is no wet path below it, so it fans into a delta of glass in dry sand.
4. Pull the Plug. The delta hangs from the trunk only, so the collapse rules snap the thinnest branches into falling shards. A few blocks and a lot of grit: a distribution, not a binary.
5. Pour water. It runs down the trunk and splits at the delta, and each broken branch-end becomes a separate spout. The level's three basins fill from **one press** you aimed at none of them.

**Challenge**
- *Doing nothing:* the storm strikes on its own every 15 seconds and fills the dune with junk glass that blocks every route.
- *Random play:* bolts wander without a wet path. Pull the Plug cast early drains the dune before the glass exists.
- *A plan:* paths are laid first, then the strikes, then the drain. Rods and Straight are spent where precision matters.

**Graded:** the share of poured water that reaches each basin. Stars at 30/60/90%. A bonus for using less glass.

**Sample level:** a 200x240 sand-filled box with a plug in the floor at x = 90–110. The cloud spans the top. Three basins sit below the box at x = 30, 100 and 170. Storm strikes come every 15 seconds and are shown 3 seconds ahead as a flicker.
- Wand: Wet Streak, Salt, Strike, When the Bolt Lands, Fork, Strike.
- Spare: Rod, Pull the Plug.

**Risk:** the work is hidden inside the sand until the reveal, so the player builds blind. Draw the glass glowing through the sand while the bolt crackles. A prototype finds out whether the reveal is a thrill or a let-down.

**Smallest prototype (4 days):** bolt growth biased by a wetness field, sand to glass, a floor plug, and a water test. No storm at first.

---

## 4. Firedamp

**Pitch:** a mine slowly fills with gas seeping from the coal. Dig out the ore, and burn the gas off in small controlled pops before it builds into a cloud that the lantern will set off.

**30 seconds:**
1. A faint violet haze creeps along the ceiling from the coal seam toward the lit office at the shaft head.
2. You dig a side tunnel to the ore. Coal dust puffs up.
3. You set a tripwire spark at the haze's edge. It fires, and a blue flame ribbon runs back along the ceiling.
4. Where the gas was thick, it bangs. The roof cracks, and gravel and sand pour down.

**Simulation**
- *Exists:* per-cell fields that spread and fade (the field code that already carries light and moisture); fire; explosions with fracture and crater smoke; structural collapse; powders pouring into tunnels.
- *New:*
  1. A gas field that rises: diffusion plus an upward drift.
  2. A burn rule. Above a low concentration a flame front travels. Above a high one, a blast with a radius set by the local concentration.
  3. Coal dust, a light flammable powder.

**Cards**
- Pick (dig a short tunnel)
- Vent (dig straight up to the sky, so gas pours out)
- Spark
- Sandbag (pour sand that plugs a tunnel)
- Spray (wets dust so it will not burn)
- Wide
- Wait 3 s
- Two at Once
- When Gas Reaches Here (carries the next card until the gas at that spot passes the mark)

**The combo: Pick into a dead-end, then When Gas Reaches Here: Spark at its mouth.**
1. The Pick leaves the tunnel hazed with coal dust.
2. Gas creeps in along the roof. The tripwire fires, and the flame ribbon runs inward along the thin gas.
3. At the end it meets the dust cloud, which goes up in a puff just strong enough to crack the tunnel roof.
4. The roof's gravel pours down and **seals the tunnel behind the flame**, cutting it off from the main gas body.

You used fire to close a tunnel safely, and the seal is a real pile you can dig through later.

**Challenge**
- *Doing nothing:* the gas reaches the lantern at full strength, and the blast collapses the office and the shaft.
- *Random play:* sparks cast into thick pockets blow out your own galleries, and vents dug in the wrong place draw gas across the ore.
- *A plan:* burn gas while it is thin, vent at high points, and wet the dust before you light anything.

**Graded:** ore cells raised, minus damage to the office. Stars for ore with zero big blasts. The middle is common: you got the ore but lost the east gallery.

**Sample level:** a 300x200 cross-section. The surface is at y = 20. A coal seam diagonal runs across the lower left and seeps gas. Three ore pockets lie inside it. The office and its lantern are at the shaft foot, x = 260, y = 120. Gas reaches the office in about 120 seconds if left alone.
- Wand: Pick, When Gas Reaches Here, Spark, Vent.
- Spare: Sandbag, Spray.

**Risk:** gas is invisible, or reads as a fog with no shape. The haze has to show its concentration as a fixed ramp, never as a blend. Render the field before building any cards.

**Smallest prototype (4 days):** a rising gas field, the two-level burn rule, Pick, Spark, Vent and one tripwire.

---

## 5. Landslide

**Pitch:** a flash flood is coming down the valley. Plant charges in the cliffs so the mountain falls where it dams the water or turns it away from the town.

**30 seconds:**
1. Rain darkens the top of the valley, and a counter shows the flood's arrival.
2. You plant a line of three charges along a seam in the overhang, and pour gravel on top to load it.
3. You set them off. The slab breaks off in a few big blocks, more cobbles and a lot of grit.
4. It lands across the river bed and throws a crown of water up both banks.
5. The flood arrives, backs up behind your rubble into a lake, and finds the low point.

**Simulation**
- *Exists:* explosions with fracture; structural collapse with weight from piles; rubble at gravel's angle; boulder splash; water levelling; rain.
- *New:* a timed flood source upstream. Optional: overtopping water carrying rubble away, so that a bad dam fails later and worse.

**Cards**
- Charge
- Big Charge
- Crack (weakens without blasting)
- Gravel (a pile of load)
- Pour Sand
- Line of Three (changes the next card: three in a row along your drag)
- Two at Once
- Wait 3 s
- When Water Touches (carries the next card until water reaches it)

**The combo: When Water Touches: Charge, planted at the foot of a cracked pillar below a dam you have already built.**
1. Your first dam holds, and a lake forms.
2. As the lake rises it reaches the pillar, and the charge fires.
3. The pillar falls into the lake, and the splash sends a wave over the dam's low corner. That reroutes the flood down the side channel you dug, into the dry ravine, instead of toward town.

You built a dam that opens itself into a spillway at exactly the right height.

**Challenge**
- *Doing nothing:* the flood reaches the town.
- *Random play:* a charge without load tips the slab the wrong way. Rubble dropped in the wrong place makes a lake that bursts toward the houses.
- *A plan:* order matters: load, then crack, then blast, then the trigger.

**Graded:** houses kept dry, out of 8. Stars at 4, 6 and 8.

**Sample level:** a 400x240 valley. The flood source is at the top-left. A river runs left to right, and the town lies at x > 340. Two overhangs at x = 120 and 220, and a dry ravine branches off at x = 260. The flood comes at 60 seconds.
- Wand: Gravel, Crack, Line of Three, Charge.
- Spare: When Water Touches, Big Charge.

**Risk:** the collapse is one-shot and chaotic, so the same plan gives wildly different results. Run one plan over 10 seeds and see whether the spread of results is exciting or just unfair. Structural collapse has a long history of surprising this project, so measure it first.

**Smallest prototype (3 days):** an existing collapse scene, a flood source, a house counter, and Charge plus Gravel on a two-slot strip.

---

## Ranking

1. **High Tide**
2. **Quench**
3. **Fulgurite**
4. **Firedamp**
5. **Landslide**

**Why High Tide is the most fun to play.** Everyone already knows the fantasy, so nothing needs explaining: build a sandcastle, and the sea wants it. That answers the satchel's "I have no idea what's going on" before the first card. The tide comes back higher every time, so there is always a next thing to do and never a phase of just watching. Failure is the best part to watch: towers sag as they dry, slump when undercut, and glass caps hang or drop with a splash. A loss comes in degrees, one flag at a time, which is the first law. The cards are timing puzzles in plain words. Wet too early and it dries, so a random hand visibly loses to a planned one. The order of a combo matters, and the sim pays it back in ways nobody wrote: a fallen tower splashing water onto the keep behind it. It is not the most elegant to build, since erosion and wet sand are new and the tide keeps chunks awake. **Quench** is the cheapest honest test of the whole idea, cards plus engine chemistry, because the engine already does the lid-and-drip trick. Build Quench first as the two-day probe, and High Tide as the game.

## Prior art

- **Noita.** Ordered spell cards in a falling-sand world. The world multiplies the combos, and your own magic is the main hazard. https://noitagame.com/
- **From Dust** (Ubisoft, 2011). Carry sand, water and lava to reshape terrain against floods and eruptions on a timer. It shows that a threat on a clock makes material-moving tense. https://en.wikipedia.org/wiki/From_Dust
- **Where's My Water?** (Disney, 2011). Dig dirt to route a flow, with a three-duck graded score and hazards that react (ooze eats dirt, mud hardens). Proof that flow-routing reads instantly on a phone. https://en.wikipedia.org/wiki/Where%27s_My_Water%3F
- **Sandtrix** (2023). Blocks turn to sand on landing, and the collapse is the satisfying part. It shows that a pour can be the reward, not just noise. https://msky-dev.itch.io/sandtrix
- **The Powder Toy.** A sandbox of reacting materials. It shows how much a few rules give, and how a sandbox with no goal stays a toy. https://powdertoy.co.uk/
- **Sandustry.** A falling-sand factory and digging game. The digging is praised and the resource busywork is criticised, a warning against bookkeeping layers. https://store.steampowered.com/app/2764460
- **Slay the Spire.** The enemy's intent is always shown in plain icons. This is the model for the "Next:" line and the tide and flood timers. https://store.steampowered.com/app/646570/Slay_the_Spire/
- **Balatro.** Combo scoring the player can read, with the arithmetic on screen as it happens. It shows that a multiplier feels good only when the player can see it build. https://www.playbalatro.com/
- **Peggle.** One shot, then you watch the cascade, with a graded score and a fanfare. This is the right rhythm for "press, then the sim answers". https://en.wikipedia.org/wiki/Peggle


<!-- ===== explorer-d-cards.md ===== -->

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
