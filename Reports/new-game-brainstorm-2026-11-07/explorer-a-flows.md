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
