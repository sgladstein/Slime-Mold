# Explorer E: games built on the plant growth engine

*2026-11-07. Five concepts, a ranking, prior art. Read for this: `wiki/plants.md` (core sections), the two short Noita notes, and a few greps of the source to confirm claims (cited inline).*

## Two facts every concept below has to live with

1. **Growth is slow because of one number.** A plant's cells are checked every 45 frames (`ORGANISM_TICK_INTERVAL`, `src/sim/plant.rs:5122`), so a tip adds roughly one cell a second. Every concept here speeds its own species up 5 to 10 times. That cost grows with the number of live tips, so each prototype has to report its worst frame from `ascii`.
2. **What the engine already does well is pay and fall.** A plant is one economy from root to leaf. Wood costs upkeep. Cut the foot and everything above lets go, coming down as logs, broken wood and litter. A heavy crown on a thin trunk breaks. A stripped plant regrows from what it has stored. A starving plant goes pale and then grey, and rots slowly. Those rules are where the surprises come from. Growth itself is the slow part; the paying and the falling happen fast.

---

## 1. Kudzu: a vine is eating the village, and every cut drops what grew past it

**Pitch.** Overnight a vine pours over a hill village. You have shears, a torch and the village's water, and one well-placed cut peels a house-sized curtain of green off the bell tower.

**30 seconds.** Green creeps up a wall about a cell every few frames. You hover the shears over a stem, and everything that would fall if you cut there lights up pale. You snip. A second later a sheet of vine the size of a house peels off the tower and lands in the street as clumps of litter. The stump you left starts budding again. You follow a root down to the well's overflow and close the gutter that waters it. Over the next half-minute the far side of the vine goes pale.

**Simulation.**
- *Exists:* tips that grow toward light; one shared economy; "anything no longer connected to the ground through its own tissue comes away and falls" (wiki, *What trouble looks like*); regrowth from stored reserves after stripping; only roots drink; a hungry plant goes pale in three steps; litter rots; fire; rigid stone buildings.
- *New:* tips that cling to and climb stone walls (the creeper wanders today, but whether it holds onto a wall is unconfirmed); a faster tick for this one species; a hover preview of what a cut would drop (it is the same "is this still connected to ground" walk the engine already does after a cut); tool charges.

**Verbs (tools, not cards; the order of plays is not the fun here).**
- **Snip**: 3 charges, one back every 8 s.
- **Dig out a crown**: hold it for 3 s. It takes away the stored reserves the vine regrows from.
- **Gutter lever**: send the spring's water to one side of the village or the other.
- **Salt**: a disc of soil with no food left in it.
- **Torch**: sets a fire, which also catches thatch roofs.

**The combo that surprises: torch the gutter, not the vine.**
1. Your earlier snips dropped litter into the gutter under the east wall.
2. You light it. Litter burns fast, and the vine leaves climbing that wall catch.
3. The east arm loses its leaves, so it stops earning. Its wood still costs upkeep.
4. The vine is **one organism with one budget**. The arm on the west side, which you never touched, now pays for the dead wood in the east.
5. The west arm stops growing and goes pale. If the budget stays negative long enough, it starts shedding from its tips inward.

You hurt the half of the vine you could not reach by burning the half you could. Nobody wrote that pairing; it falls out of one plant, one economy.

**Challenge.** Keep the bell tower, the well and five house fronts clear by dawn (5 minutes).
- *Doing nothing loses:* the vine covers about 90% of the village by dawn, and the tower's bell stops ringing when it is wrapped.
- *Random play loses to a plan:* snipping leaves is wasted, because the vine regrows them from its store. Burning near thatch destroys a house. The plan is to find the three crowns (one hidden in a cellar), cut off their water, dig out the worst one, and then snip at the collars.

**Graded outcome.** Stars at 50 / 75 / 95% of the house fronts clear at dawn, with a bonus star if no roof burned. On screen beside the score: cuts made, green dropped (cells), regrowths, and roofs lost.

**Sample level.** 256x160.
- **Ground:** a hillside with five stone houses with thatch roofs and a bell tower in the middle. A spring at the top right feeds a gutter that runs to a well at the bottom left.
- **Crown A:** in the rich garden soil at the left, and already up two walls.
- **Crown B:** in a crack at the tower's foot.
- **Crown C:** in the cellar under house 4, with its roots in the well's seep.
- **Hand:** 3 snips, 1 dig, 1 salt, 2 torches, and the gutter lever.

**Risk.** A tangle of green you cannot read. If the player cannot tell which stem feeds which arm, every cut is a guess and it plays like random clicking. The hover preview is the whole bet. *Finding out:* show the owner a still frame, ask "where would you cut?", and then show the preview.

**Smallest prototype (3 days).** The shipped creeper with a 6x faster tick, in a box with two stone walls and one crown. The existing cut tool acts as the snip, plus the new hover preview and a "% of wall clear" counter. The question it answers: is a cut that drops a whole sheet satisfying, and is regrowth a threat or an annoyance?

---

## 2. Heartwood: your deck is your tree's genome

**Pitch.** Two trees, one valley, one sun. You write your tree's growth as cards that its growing tips read, and you win by shading the rival out.

**30 seconds.** Your leader is climbing. You tap it and play **Fork**, and the tip splits. The next card in your hand, **Lean to Sun**, goes to *both* new tips, and they swing toward the gap in the rival's crown. Your hand refills. A limb you over-built cracks under its own weight and falls as a log. Under your new crown, the rival's lower leaves turn yellow.

**Simulation.**
- *Exists:* a tip chooses its next cell by weighing four things: carrying on straight, light, wind, and up. Confirmed in `plant.rs`, where the weights are `[continuation, light, wind, upward]` around `:6663`. Also existing: shade kills leaves that cannot pay for themselves; wood upkeep grows faster than trunk size; a heavy top on a thin base breaks; deeper, wider roots drink more and anchor better; seeds.
- *New:* cards that override a tip's weights for its next N cells, a forced branch, a sugar purse the player spends from (the tree's own income), and a light-share count at the end of each season.

**Cards** (each costs sugar, which comes from what your leaves earn):
- **Grow Up**: the next 10 cells go straight up.
- **Lean to Sun**: the next 10 cells turn toward the brightest side.
- **Fork** *(modifier and multicast)*: this tip splits in two, and the next card goes to both halves.
- **Leaf Out**: a fan of leaves at this tip.
- **Thicken**: the trunk below this tip grows a cell wider. It costs upkeep for good, and it buys strength.
- **Deep Root**: a root tip goes after the wettest ground.
- **Bud** *(trigger)*: holds the next card and plays it if this branch is cut or breaks.
- **When Shaded** *(trigger)*: holds the next card and plays it when this tip's light falls below half.
- **Shed**: drops your lowest branch. Upkeep goes down, and the litter falls on the rival's seedlings.
- **Drop Seed**: a seedling of yours wherever it lands.

**The combo that surprises: Fork, Fork, Lean to Sun, Leaf Out, and a Bud under it.**
1. Two Forks turn one tip into four, and all four read Lean to Sun. That gives a broad fan reaching over the rival.
2. All four Leaf Out. The crown is now heavy, but it is sitting on a trunk you never thickened.
3. The engine's break rule is about proportion, not size: a heavy top on a thin base fails at one place. The limb snaps at its base and comes down onto the rival's sapling below as a log and broken wood.
4. The **Bud** you had placed on the stub fires because its branch broke. It plays the card it was holding, **Grow Up**.
5. A fresh leader rises from the stub into the light the fallen limb has just cleared.

What looked like a mistake was a planned demolition. The world supplied the break; your deck supplied what happened next.

**Challenge.** Three seasons. At the end of each, the sun counts the leaves of each tree that sit in light.
- *Doing nothing loses:* a tip with no card does not grow.
- *Random play loses:* Forks everywhere put leaves in your own shade, where they earn nothing. The upkeep sinks your purse. The crown breaks on the wrong side. A dry third season kills a tree that never played Deep Root.

**Graded outcome.** Light share at the end of season 3, with stars at 40 / 55 / 70%. Each season's share stays on screen, along with limbs lost, sugar spent and seedlings standing.

**Sample level.** 160x120 valley, and the sun crosses from left to right each day.
- **The rival:** an oak on the right, two seasons old, grown from the shipped tree species with no cards.
- **Your side:** your seed on deep soil at the left. A rock shelf in the middle of the valley gives thin soil there.
- **Season 3** is dry.
- **The deck:** 14 cards (3 Fork, 3 Grow Up, 2 Lean, 2 Leaf Out, Thicken, Deep Root, Bud, When Shaded). You hold 5 and draw one every 4 s.

**Risk.** It turns out to be a slower way of drawing a tree, or which tip read the card is unclear. *Finding out:* in playtest, can the owner say, before playing a card, what the tree will look like afterwards?

**Smallest prototype (4 days).** The shipped tree species at 6x speed. Five cards (Grow Up, Lean, Fork, Leaf Out, Bud) applied to the tip you tap, as weight overrides. A rival tree with no cards, and the light-share number on screen.

---

## 3. Reclaim: the jungle takes the temple, one root at a time

**Pitch.** You are the forest. You topple a dead empire's temple by growing roots into its cracks before its masons can patch them.

**30 seconds.** You flick a fig seed onto a dusty ledge on the third step. It sprouts, and its root follows the damp down a mortar joint. You tip a rain jar so water runs along the ledge into that crack. The root thickens there. The block above shifts a cell, grinds, and then lets go. A corner of the roof drops as three blocks, a spill of cobbles and a lot of grit. Two masons on scaffolds hurry to the next crack.

**Simulation.**
- *Exists:* roots lean toward wetter ground, and they sense damp they cannot drink yet. Roots enter soil but not stone. Rain and liquid flow. Structural collapse with graded breakage into blocks, cobbles and grit. Trees on a narrow root plate build roots to catch up, and they break when the crown outgrows the base.
- *New:* mortar, a material that roots can enter; **root wedging**, where a root thickening in a joint pushes on the rigid body beside it (a force fed into the load model); masons, which are scripted crack-patchers rather than creatures; and faster growth.

**Verbs (tools):**
- **Fig seed**: a strangler with many roots.
- **Moss**: spreads over stone and holds rain on a ledge.
- **Pour**: tip a rain jar.
- **Dust**: a pinch of windblown soil into a crack, which gives a seed somewhere to sit.
- **Cut**: remove one of your own plants.

**The combo that surprises: the tree you grew to wedge the wall becomes the battering ram.**
1. You pour into the roof crack twice, so the fig there grows big. Its roots are confined to one joint, so its root plate is narrow.
2. Its crown outgrows that plate. The engine's rule is that a heavy top on a narrow base fails.
3. The fig's main limb snaps and falls as a solid log, not as dust. That is shipped behaviour.
4. It lands on the cornice of the tier below. Your second fig has been loosening that cornice for a minute.
5. The cornice fails under the blow. The structural model takes the falling stone the rest of the way down the stairs.

You planned a wedge and got a hammer.

**Challenge.** Bring down the idol tower before the restorers' season ends (6 minutes).
- *Doing nothing loses:* masons patch a cracked block every 20 s, working up from the bottom, and a seed on bare stone has no soil and dies.
- *Random play loses:* seeds scattered on stone die. Water poured where no root is waiting simply runs off. The plan is to put soil and water on one path toward a keystone.

**Graded outcome.** Stars at 30 / 60 / 90% of the temple's mass on the ground, with the idol as a separate prize. The counts beside it: blocks dislodged, cracks patched and plants alive.

**Sample level.** A stepped pyramid 200 cells wide, in four tiers of rigid blocks with mortar joints, and the idol on top.
- **Surroundings:** a spring on the hillside at the left, and wind that drifts dust from the left.
- **Weak points:** three visible cracks, one of them under the second tier's keystone.
- **Opposition:** two masons.
- **Hand:** 6 fig seeds, 4 moss, 3 pours, 5 dust.

**Risk.** Wedging may be either invisible (a block moves after a minute of nothing) or binary (the whole temple goes at once). *Finding out:* a filmstrip of blocks dislodged per minute, and the size spread of the pieces.

**Smallest prototype (4 days).** One wall of rigid blocks, one mortar joint with soil, one sped-up fig, and the wedge force. The question it answers: is the moment a block lets go satisfying, and can a player cause it on purpose?

---

## 4. Tinder: tend a forest for one summer, then the fire comes

**Pitch.** The wildfire arrives on a day you know. Until then you thin, rake and burn small, or the forest burns all at once.

**30 seconds.** Under a crowded stand, shaded saplings are dying and grey deadwood is piling up. You fell one. It comes down as two logs, a heap of broken wood and a shower of litter. You drag a log across the creek. On a damp morning you light a line of raked litter. It creeps, blackens a strip and stops at the creek. The fire-day clock reads 90 s.

**Simulation.**
- *Exists:* a crowded stand sheds shaded leaves as litter; trees die and stand grey before rotting; fire and heat; wind; rain; felling yields logs, broken wood and litter; ash becomes soil, and a burnt patch reseeds.
- *New:* fire that spreads more slowly through damp fuel (*unconfirmed* whether fire reads moisture today), a scripted wildfire front, and the forest at about 20x speed.

**Verbs (tools):**
- **Fell**: the axe.
- **Rake**: sweep litter into a line or a pile.
- **Drip-torch**: light a line.
- **Bucket**: carry water.
- **Plant**: a seedling.

**The combo that surprises: your fix makes a worse fire path.**
1. You fell the big tree in the middle to open a gap, because less canopy means less fuel.
2. Light now reaches the floor. Seed waiting in the soil, which the wiki says is about nine seeds for every standing plant, sprouts all at once.
3. Within a minute the gap is a thicket of saplings, waist-high, packed and leafy.
4. On fire day, that thicket is a ladder. Flame climbs it into the crowns of the old trees on either side.

The player learns to burn the gap again once it fills, which makes the cleared ground a chore that comes back every summer. The ecology did that, not a script.

**Challenge.** On fire day a front enters from the left on the wind.
- *Doing nothing loses:* the deadwood and litter of a crowded summer carry the front through every crown.
- *Random play loses:* you burn on a dry afternoon and lose your own old trees, or you clear gaps and grow ladders.

**Graded outcome.** Old trees standing out of 12, with stars at 4 / 8 / 11. The counts beside it: litter burnt on purpose, wildfire cells, and logs saved.

**Sample level.** A valley 256x128.
- **Trees:** 12 old trees, with crowded saplings on the left slope.
- **Ground:** a creek down the middle and a dry ridge on the right.
- **Timing:** morning dew lasts 20 s a day, and fire day comes at 4:00.
- **Tools:** unlimited, with a stamina bar.

**Risk.** It plays as maintenance rather than a puzzle, and the fire is either out or total. *Finding out:* run the wildfire 10 times on the untouched forest and 10 times on a thinned one, and check that the number of survivors spreads out.

**Smallest prototype (3 days).** A lab-style bed fast-forwarded to a crowded summer, the existing fire, a scripted front, and a survivor counter.

---

## 5. Seedfall: get a family of plants across a dead land, one lifetime at a time

**Pitch.** Lemmings over generations. Each plant flowers, seeds and dies, and you decide where its children land.

**30 seconds.** Your herb flowers and drops seed. You flick two seeds toward a ledge of the broken aqueduct. One lands on bare stone and dries out. The other lands in a pocket of ash you made by burning last generation's dead stem, and it sprouts. You break a soil lip so a puddle drains into that pocket. Behind you, the mother plant browns and starts to rot.

**Simulation.**
- *Exists:* seeds fall as powder; they need light, solid ground and damp to germinate; a seed on dry ground waits for rain; a herb dies after it sets seed; seed is set per mature cell, so a bigger plant gives more seed; crowded roots draw the soil down; damp ash turns to soil.
- *New:* **Flick**, because today wind moves air and smoke but not seed (wiki, *Species*); a herb lifetime of about 40 s; a generation counter.

**Verbs (tools):**
- **Flick**: throw seed from the seed heads that exist now.
- **Burn**: turn dead stems into ash.
- **Breach**: break a soil lip so water flows.
- **Wait**: let a plant grow bigger before it seeds.

**The combo that surprises: fire as a seedbed.**
1. Burn the dead mother where she stands on bare rock.
2. Wait for rain. Damp ash turns to soil.
3. Your next seed now has ground on stone where nothing could have grown.

A second, slower trick is **Wait**. A plant left to grow big sets far more seed, so one patient generation can fund three flicks across a gap.

**Challenge.** Reach the spring at the far right within 12 generations.
- *Doing nothing loses:* seed falls at the parent's feet, the crowded roots exhaust the pot, and the dry spell finishes the line.
- *Random play loses:* flicked seed lands on stone and dies.

**Graded outcome.** Stars at half the distance, three quarters of it, and the spring, plus a bonus for each plant alive at the end. The generation count and seeds lost are on screen.

**Sample level.** A strip 320x100.
- **Start:** a pot with one herb at the left.
- **Middle:** a broken aqueduct with three stone ledges, a dry riverbed, and a ridge of dead brush you can burn.
- **Goal:** the spring at the right.
- **Weather:** 10 s of rain every 60 s.

**Risk.** Each generation is too slow to feel like a turn, and flicking turns it into an aiming game rather than a growth game. *Finding out:* time one generation and ask whether the owner cares about the line surviving.

**Smallest prototype (3 days).** The herb with a shortened life at 8x speed, a flick tool, the existing fire, and a distance counter.

---

## Ranking

1. **Kudzu.** There is a verb every few seconds, each cut drops a visible sheet, and the vine's single economy produces surprises nobody scripted.
2. **Heartwood.** The most Noita-like (Fork as multicast, Bud as trigger). Its risk is that it plays as slow drawing.
3. **Reclaim.** The biggest payoff, growth that ends in falling stone, but root wedging is new and may read as binary.
4. **Tinder.** It is fire-heavy, built on existing machinery, and its backfire is good. It may play as chores.
5. **Seedfall.** A charming premise, but each turn is a lifetime, which is the slowest loop here.

**Why Kudzu will be the most fun to play.** It reverses the problem the lab had. In the lab, growth was something you watched. In Kudzu, growth is the enemy, and it keeps coming, so there is always a next thing to do: the vine regrows from its store whether you are ready or not. The verb is the one the owner already ruled on for this line: "the verb works, and what it produces is pieces." Every snip drops a house-sized sheet of green with a sound and a heap. Doing nothing visibly loses, because the village goes green. Random snipping visibly loses, because leaves grow back. The good moves are readable deductions: follow the stem to its crown, starve it of water, and burn one side to weaken the other. A town under a vine also reads on a phone at a glance. Green on grey is the whole score.

## Prior art

- **Mushroom 11** (Untame, 2015). You move a fungus by erasing it, and it regrows on the far side. It shows that cutting a growth can be steering, which is Kudzu's verb. https://store.steampowered.com/app/243160/
- **Photosynthesis** (Blue Orange, 2017). Trees compete for light from a sun that moves, which makes shade a legible board-game rule. It is the model for Heartwood's season count. https://blueorangegames.com/games/photosynthesis
- **Terra Nil** (Free Lives, 2023). A reverse city-builder in which ecology spreads as a puzzle with a fixed toolset. It shows that growth works as a puzzle when the tools are few and the spread is legible. https://www.pcgamer.com/restore-a-desolate-ecosystem-in-reverse-city-builder-terra-nil/
- **Grow Home** (Ubisoft Reflections, 2015). You steer a beanstalk to islands of nutrients. Steered growth works when it is fast and you ride it. https://gamingbolt.com/grow-home-ubisofts-new-downloadable-game-available-on-steam
- **Reus** (Abbey Games, 2013). You shape nature with giants and their abilities combine. Combos in a nature setting can carry a game. https://store.steampowered.com/app/222730/Reus/
- **Equilinox** (ThinMatrix, 2018). A gentle nature sandbox whose reviewers found its evolution cosmetic. It is the warning for Seedfall: generations have to *change* something you can see. https://www.gamingonlinux.com/articles/12902/
- **The Algorithmic Beauty of Plants** (Prusinkiewicz and Lindenmayer, 1990). L-systems are grammars that growing tips read. Heartwood's "deck as genome" is a playable L-system. https://algorithmicbotany.org/papers/abop/abop.pdf
