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
