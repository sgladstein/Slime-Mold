# Clean slate: who is the player, and how do they reach the mould? (2026-11-10)

Brainstorm only. Nothing was built, run or measured. Nothing in the repo was edited.

**What was read.** `wiki/index.md`, the three `wiki/model/` pages, the Jenson report and its three contact sheets, and the model notes and `Params` in `src/mould/mod.rs`.
I grepped `Reports/dead-ends.md` for each proposed mechanism (method rule 5).
The RTS doc was read only after the concepts below were written.
The five old tools were set aside, then judged at the end on merit.

## What the model is good at, and what that asks of a player

These are the things the walkers do by themselves. A good control model hands them to the player instead of overriding them.

1. **It finds short paths, prunes dead ends and closes loops on its own.** A player who dictates shape throws this away.
2. **It carries things along tubes at walker speed.** Food already does this (sap, stream, pulse), and contagion would too. Travel time is distance along tube, so the network's shape is a timetable.
3. **Its texture is its rules.** Sensor angle and reach give whole pattern families: mesh, trunk, fibre, lattice. Jenson's ef_9 clip shows two of them side by side with a sharp border.
4. **It is restless at 22.5 degrees and settles at 45.** Restless mould shrugs off fine steering. Settled mould obeys, then freezes (wiki, sensors page).
5. **At GPU scale it holds hundreds of loops.** Jenson's network_2 goes from 3-5 loops to 180-340 in nine seconds. Steering one tube stops being readable at that density. Waves, regions and textures stay readable.

The lesson I drew: at scale, a player who draws shapes fights the model. A player who picks *when*, *what state* or *which rules* rides it.

## The control models (21)

Names are working names only.

| # | Who the player is | Core verb | What you see happen | Competitive structure | Why it could be fun | Obvious failure |
|---|---|---|---|---|---|---|
| 1 | **Ink.** The mould's will | Paint a private signal that only your strain senses | A faint stroke on the agar; your walkers swing onto it within a second; tube follows | Two strains on one dish; ink regrows | Direct and expressive, with instant feedback | The mould becomes a cursor. The best painter wins, not the best reader. At 45 degrees it obeys, then freezes |
| 2 | **Temper.** Editor of your mould's rules, by region | Brush a dial (sensor angle, reach, deposit, fade) onto part of your body | That patch changes texture: mesh to trunk, fibre to lattice | Two moulds, territory | Jenson's regimes become choices, and the picture shows the rule | Dials feel like a lab. Effects are hard to predict, and then the best setting gets solved |
| 3 | **Tip.** One heavy walker at the front | Steer the tip; its trail is so strong the rest follows | Your path thickens into trunk; side branches sprout off it | Two tips race to food and cross each other's routes | Action-game speed | The mould becomes your tail. It is close to snake games and the old ant game's trail-laying hero |
| 4 | **Heart.** A slow, moving food heart | Drag the heart; the network must stay joined to it | The whole body leans after you; tubes snap if you rush | Two hearts forage and try to cut each other's supply | Weight and momentum; your body is huge | Standing still is best. It reads as a big snake again |
| 5 | **Ground.** Shaper of the world | Raise ridges, dig channels, wet or dry ground | The mould reroutes round the new ground | Shared dish; each strain likes different ground | Architecture and long plans | Too slow for real time. Ground is scenery until the mould meets it |
| 6 | **Hubs.** Commander of organs | Order a held pile to send mass to another | Mass runs from node to node | Capture nodes | Clear goals | It collapses into a node-capture game, which exists in several forms. Walkers become decoration on a graph |
| 7 | **Keeper.** Tender of several species | Tune births and deaths of mould, grazers and predators | Populations rise and crash | Each player owns one species in a shared tank | Long chains of cause and effect | Too many fields to read. Your action shows minutes later |
| 8 | **Weather.** Asymmetric: one player is mould, one is the world | The world drops drought, glare and famine; the mould steers away | Hazards land; mould flees or dies | Swap sides each round | Hunter and hunted | The world side is the old toolbox as a griefer. Balance is hard and the mould side lacks agency |
| 9 | **Pair.** Co-op: two hands on one mould | One player runs the front, the other the supply | Front and supply react to each other | Co-op against a wild mould | Talk and trust | One player quarterbacks. It needs a strong AI opponent first |
| 10 | **Squeeze.** You choose *when*, not where | Squeeze a held pile to send a wave of sap down your tubes | A bright wave runs along the tubes; where it lands the front swells or the seam moves | Two moulds meet at seams; time your waves against the enemy's ebb | Rhythm and reading; the enemy's charging is visible | If timing does not matter, it is a cooldown button |
| 11 | **Laws.** Bidder for rules | Bid biomass on a rule offered every 20 s | After the gavel the winner's texture changes | Auction rounds during play | Bluff and value judgement | Downtime while reading cards. The rich win auctions and get richer |
| 12 | **Two Fevers.** A contagion inside a shared, neutral host mould | Seed your strain on host tube; choose what carriers do | Your colour runs along tubes at walker speed, stops at gaps and arrives twice through loops | Two strains race through one host; immunity is per strain | Waves collide, and the leader's old ground becomes the trailer's chance | Once seeded, you watch. Colour soup. A competitive plague mode exists, on a country map |
| 13 | **Grafts.** Curator of inherited rules | Capture rule sets from wild colonies; graft them into your body | Patches of your mould change texture; the graft spreads as walkers divide | Two lineages, seams and counters | The tech tree is living colonies on the map; you read the enemy's rules off its texture | No counter cycle exists, and one regime wins everything |
| 14 | **Prune.** You only remove | Cut mould away; its mass regrows elsewhere | The cut part greys; the far front surges | Race to food by pruning | Calm and clever | Close to a known erase-to-move fungus game. The knife was already the weakest tool |
| 15 | **Custody.** Two wills, one shared body | Lay a call the mould senses (or, in a variant, shove it off your pantry) | The mould weighs both calls and commits, visibly | Tug-of-war over one neutral creature | You persuade a creature that decides | The mould joins both calls and sits still. It turns into "who paints more" |
| 16 | **Clock.** Bender of time | Speed up or slow down the steps in a patch | A patch races or crawls | Speed your front, slow theirs | Strange and new | Faster is simply better. The choice has no texture |
| 17 | **Spores.** Colonist | Spend biomass to start a new colony elsewhere | A new network germinates and grows | Plant to claim ground | Expansion feels big | Unit placement in disguise; spam wins. The spores' flight must be simulated too |
| 18 | **Valves.** Plumber | Pinch or open tubes | Food banks up behind a pinch; the far side dims | Feed your fights and starve your idle ends | A logistics puzzle at speed | The network rewires under your valves. Micromanagement |
| 19 | **Taste.** Setter of your strain's likes | Toggle what walkers like: own trail, enemy trail, scent, damp | The whole mould changes stance: it hunts, flees or gathers | Stance against stance | One choice makes a big visible swing | Coarse. The best stance for each phase gets solved quickly |
| 20 | **Memory.** Teacher of habits | Bless or forget a route (tube walls) | Blessed tubes fatten into trunks over seconds | Build trunks that carry your fights | Long-term shaping | Walls are slow and hard to see. Little happens minute to minute |
| 21 | **Reflexes.** Programmer of local rules | Write and edit small if-then rules that walkers run | Behaviour changes wherever the condition holds | Program against program, edited live | Deep for tinkerers | It becomes programming. Newcomers and watchers are lost |

## How they were judged

Each model got 1 to 3 on six measures (3 is best).

- **Seen:** does the action show within about a second, as simulation?
- **Read:** is there skill in reading the opponent off the dish?
- **Rides:** does it use what the model does by itself?
- **Comeback:** does the structure resist being decided in the opening?
- **Cheap:** build cost (3 is small).
- **New:** distance from existing games.

| # | Model | Seen | Read | Rides | Comeback | Cheap | New | Total |
|---|---|---|---|---|---|---|---|---|
| 10 | Squeeze | 3 | 3 | 3 | 3 | 2 | 3 | **17** |
| 12 | Two Fevers | 3 | 3 | 3 | 3 | 1 | 3 | **16** |
| 13 | Grafts | 3 | 3 | 3 | 2 | 1 | 3 | **15** |
| 15 | Custody | 2 | 2 | 3 | 2 | 3 | 3 | 15 |
| 19 | Taste | 3 | 2 | 3 | 2 | 3 | 2 | 15 |
| 2 | Temper | 3 | 2 | 3 | 2 | 2 | 2 | 14 |
| 18 | Valves | 3 | 2 | 2 | 2 | 2 | 2 | 13 |
| 4 | Heart | 2 | 2 | 2 | 2 | 2 | 2 | 12 |
| 8 | Weather | 3 | 2 | 2 | 1 | 3 | 1 | 12 |
| 1, 3, 11, 14, 16 | Ink, Tip, Laws, Prune, Clock | | | | | | | 11 each |
| 5, 7, 17, 20, 21 | Ground, Keeper, Spores, Memory, Reflexes | | | | | | | 10 each |
| 6, 9 | Hubs, Pair | | | | | | | 9 each |

**Breaking the tie at 15.**

- Taste and Temper fold into Grafts. Their stances and dials become rule sets you must capture first. That supply limit answers their "solved setting" failure.
- Custody is the most original structure on the list, and I rejected it. The mould's best trick is joining every food source. So it joins both players' calls and sits there. The only fixes I found were a lure (oats again) or a shove (the lamp again).

## Concept 1: Squeeze (the player chooses when)

Every pile you hold is a heart. You choose when each heart squeezes. Waves of sap race down your tubes. Where they land, your mould grows or wins its seam.

**Minute-to-minute loop.**

- Two moulds grow from opposite sides of a dish with 10-14 finite piles. They meet in seams.
- Each held pile pumps a little sap all the time and stores the rest in its heart.
- Tap a heart to squeeze it. Its store leaves as one bright wave along every tube it feeds.
- The wave splits at junctions by tube thickness. A long route delivers it late and thin. A loop delivers it twice.
- Where the wave lands, loaded walkers divide and the front bulges. At a seam, loaded walkers hold and eat into the hungry side.
- **Hold** a heart instead, and it stops pumping to charge a bigger squeeze. The tubes it feeds dim while it charges.
- **Pinch** a tube for three seconds. Sap banks up behind the pinch, and the wave goes the other way.
- Read the enemy the same way. Dim tubes mean they are charging. A bright wave coming means brace: squeeze the heart nearest that seam.
- A decision comes every 3-8 seconds.

**What is on screen.** The wave is the sap field itself, which is already simulated. Charging shows as tubes dimming. A pinch shows as sap banking. Nothing is painted.

**Spend and earn.**

- Earn: sap from the piles you hold. More piles give more hearts.
- Spend: squeezes. Holding spends your network's health now for a bigger wave later.
- Piles run out. A heart whose pile is empty goes quiet.

**How a match is won.** The bell rings at about five minutes. The score is each mould's share of all food eaten, so the outcome is a spread (say 0.58 to 0.42). Losing every heart ends the match early.

**Why the opening does not decide it.**

- Waves thin with distance. A seam near your hearts is strong, and a far one is weak. A leader stretches its supply, so its fronts weaken.
- Piles deplete. The leader eats faster and runs dry sooner.
- Charging is visible. A greedy early charge is punished by a wave timed into your dim tubes.
- It does **not** rely on late piles landing on the seam. The Wild One tried that, and the race went to whoever was nearer (`dead-ends.md`). Any new pile comes with a long preview.

**Rules it bends.**

- `pulse_seconds` and `pulse_strength`: the sine beat becomes a beat each heart fires on command.
- `stream` and `stream_bias`: the out-stroke's strength becomes the squeeze's size.
- `fight`: today it is halved when fed and doubled when unfed. Here it scales with the load carried, so a wave tips a seam by degrees.
- `wall_grow`: routes that carry waves fatten into trunks, so repeated attacks build their own highway.
- `flow` with `sap` runs on both moulds. The rival is a whole second `Dish` with its own `Params`, so each can carry its own sap.

**Build cost: medium.** Flow, sap, stream, pulse, walls, the rival and the fight all exist. What is new: a store per heart, the squeeze, hold and pinch inputs, a load-scaled fight, and flow on both moulds at once.
Frame cost: two moulds measured about 2.2 ms of simulation a frame (RTS doc). A second sap field adds its spreading passes. My guess is a few tenths of a millisecond, not measured.

**At GPU scale,** waves running through hundreds of loops become the explosion clip's sweep, started by a player.

**The single measurement.**

- Hands: **Reader** squeezes so the wave lands on a seam when the enemy's load there is lowest. **Random-timing** makes the same number of squeezes from the same hearts at random moments. **Metronome** squeezes whenever a heart is full. **Idle** never squeezes.
- Symmetric dish, sides swapped, 16 paired seeds.
- **Deciding measure:** food share at the bell, Reader against Random-timing. Pass: Reader wins at least 13 of 16 (sign test p = 0.02, two-sided).
- This isolates the premise: same budget, same hearts, only the timing differs.
- Guard: Random-timing must not beat Idle by a wide margin, or squeezing is free power, not a choice. Holm-correct across the three pairings.

**The obvious risk.** If a wave crosses a seam's reach in under a second, nobody can react. If it takes over ten seconds, nothing happens. Tune travel time first, by looking.

## Concept 2: Two Fevers (the player is a contagion inside a shared host)

A big neutral mould fills the dish and never stops rewiring. Each player is a fever spreading through its walkers. Your fever changes how carriers behave, so you reshape the map as you spread.

**Minute-to-minute loop.**

- The host runs restless at 22.5 degrees, fed by piles that regrow. Its tubes shift all match, so chokepoints open and close.
- Tap host tube to seed. A small disc of walkers turns your colour.
- Carriers lay a trail of their own strain. Clear walkers that walk into enough of it catch it. So your colour runs along tubes at walker speed. It stops at a gap, and through a loop it arrives from both sides.
- Carriers recover after about ten seconds. They are then immune to your strain for a while, but not to the enemy's. So a wave passes like a ring, and the ground behind it is open to the enemy.
- You run two of six traits at a time and swap on a cooldown. Traits change what carriers do:
  - **Swarm:** carriers like their own strain trail. They bundle into thick tubes and spread slower.
  - **Roam:** carriers sense wider. They sprout across gaps and jump between host pieces.
  - **Linger:** carriers recover slower. You hold ground longer but spread less.
  - **Hush:** carriers lay little ordinary trail. Host tubes under them thin and break, making a firebreak.
  - **Beckon:** carriers give off food scent. The host grows toward them and its mass shifts your way.
  - **Bloom:** fed carriers divide faster, so more host walkers are born carrying your strain.
- You watch the enemy's waves and choose. Seed ahead of them, seed on their immune wake, or cut them off with Hush.

**What is on screen.** A carrier's colour is its state, and the strain trails are real fields. Immunity is a pale tint, which is also state. Every colour is data.

**Spend and earn.**

- Spores pay for seeds. One regrows every six seconds.
- Each host pile your carriers sit on adds one spore per tick. You hold at most three.
- Traits are free but slow to swap.

**How a match is won.** The score is your carriers' share of the host, summed over a five-minute match. It is a spread, not a knockout. The host never dies, so the picture stays full.

**Why the opening does not decide it.**

- Immunity is per strain. The more host you have held, the more of it is immune to you and open to your rival. Leading feeds the trailer.
- The host rewires, so a chokepoint you owned at 1:00 may be gone by 2:00.
- Spores are mostly a flat income, so a lead buys little extra seeding.

**Rules it bends.**

- **Contagion,** Jenson's rule: one state per walker and two new trail channels. It is planned for GPU phase 1 and not built.
- The strain trails are their own fields, not tube and never counted as tube. Putting scent into the trail field was a dead end for exactly that reason.
- Sensing changes per walker by trait. This is the 36 Points idea, with parameters set by state.
- `sensor_angle` 22.5 for the host. The restlessness that drowns fine steering becomes the moving map.
- Piles regrow so the host never starves. The `fight` rule is not used: nothing is killed, only converted.

**Build cost: medium to large.** Two diffusing fields, each about one more trail blur pass per step. Per walker: one byte of state and two timers. Six trait overrides.
Frame cost: a guess of 0.1-0.3 ms per frame at 256 x 160, not measured. It plays as a prototype at today's 2,000 walkers. It shines at GPU scale, where the host has hundreds of loops and waves have room.

**The single measurement.**

- Hands: **Choke** seeds on the narrowest host tube between clear regions, and on the enemy's immune wake right after it passes. **Random** spends the same spores on random tube cells at random times. **Idle** gets one free seed at the start.
- 16 paired seeds, each hand playing both colours.
- **Deciding measure:** summed share, Choke against Random. Pass: Choke wins at least 13 of 16.
- Guard for the comeback claim: in Choke-against-Choke games, the leader at 1:30 should lose at the bell on at least a quarter of seeds. That rate should be clearly higher than with per-strain immunity off, on the same seeds.

**The obvious risk** is watching. If one seed does most of the work, the player waits. The spore clock and the immune wake are meant to give a decision every few seconds. The census must show that Choke beats Random through timing, not just through aim.

**Originality.** Plague Inc: Evolved has a two-player VS mode in which two plagues compete for one world map ([wiki](https://plagueinc.wiki.gg/wiki/Multiplayer/VS_Mode)). That mode works on countries. It does not mention immunity between the two plagues. Two Fevers differs in three ways. The map is a living network that the fevers rewire. Spread is walker to walker in real time. Immunity is per strain. Names, look and trait words must stay clear of it.

## Concept 3: Grafts (the player curates the mould's rules)

Every walker carries its rules and passes them to its children. Wild colonies on the dish live by other rules. Capture them, then graft their rules into your body where you need them.

**Minute-to-minute loop.**

- Two founder moulds start on opposite sides. Between them sit four to six small wild colonies on food. Each has a different rule set, so each looks different:
  - **Lace:** wide angle, short reach. A fine mesh that keeps its loops after damage and finds food fast.
  - **Cord:** 45 degrees, heavy deposit. A few thick tubes that hold seams.
  - **Feeler:** narrow angle, long reach. Long thin runners that smell far food.
  - **Hunter:** drawn toward enemy trail instead of pushed away.
  - **Grid:** straight lines locked to the grid, as in Jenson's header still.
- These must be tuned so that each is best somewhere.
- When your mould touches a wild colony, they fuse. Jones 2015 has kin fusing and strangers keeping apart.
- Fusion spreads through the colony by contact. If both players touch it, it splits by who reaches which part first.
- A captured colony keeps its rules in place, and its rule set goes into your bank.
- **Graft:** pick a banked rule set and brush a disc of your own mould. Those walkers switch rules at once. Within a second or two the patch changes texture. As they divide, the new rules spread with the growth.
- Put Cord at a seam to hold it. Put Feeler at the front to find food. Fill the interior with Lace so cuts do not matter. Send Hunter into their Lace.
- You read the enemy's rules off its texture. That is the picture being the data.

**Spend and earn.**

- Earn: biomass from food, and rule sets from captured colonies. Overrunning part of the enemy can bank one of its rule sets.
- Spend: each walker you graft costs biomass. A big graft starves you for a moment.

**How a match is won.** By share of food eaten at the bell, which is a spread.

**Why the opening does not decide it.**

- Counters. If the regimes form a cycle, any lead built on one regime has an answer in some bank.
- Grafts act in seconds, so a change of plan is quick to make and quick to see.
- Each side has its own wild colony near home, and the contested ones sit in the middle.

**Rules it bends.**

- `sensor_angle`, `sensor_offset`, `deposit` and `foe_repel` move from per dish to per walker. They are inherited on division. 36 Points already sets parameters per particle.
- The rival rule extends to a third population, the wild one, with fusion between kin. The wild colonies share one population and differ by the rules their walkers carry.
- Optional: small mutation on division. Leave it off at first, since it adds noise before it adds play.

**Build cost: large.** Per-walker rules, a third population, fusion and a bank.
Frame cost: one rival costs about +1.4 ms a frame (RTS doc), so the wild population should cost about that again. A separate `Dish` per wild colony would cost far more, which is why they share one population.
A cheap first step is a sweep, not a build. The rival is its own `Dish` with its own `Params`, so two regimes can meet today.

**The single measurement,** taken before any game is built: the counter matrix.

- Each pair of regimes meets on a symmetric seam dish, over 16 paired seeds with sides swapped. Score: seam ground at the bell.
- Pass: the win graph has a cycle (A beats B, B beats C, C beats A). Each link must be a sign-test win that survives Holm correction.
- Fail: one regime beats every other. Then the game is "get that one", and Grafts is dead.
- Then the game check: a counter-grafting hand against random grafting and never grafting.

**The obvious risk.** The regimes may look different and play the same. The matrix catches that.

## The old five tools

**None of the five survived as a tool.** Two of their mechanisms survive inside other rules.

- **Lamp (push).** Its job is moving tube without cutting it. None of the three asks the player to shape tube. The one place it won on merit was Custody's hot-potato variant, where you shove a shared mould off your pantry. Custody did not make the three.
- **Oats (lure).** Dropping free food into a competitive dish prints money beside your own front. Its mechanism, scent that pulls growth, survives as Two Fevers' Beckon trait.
- **Salt (wall).** A good map feature, but not a verb. Squeeze's pinch blocks sap, not walkers, and lasts only seconds.
- **Knife (cut).** The wiki already records it as the weakest tool. Two Fevers' Hush does the job a cut wanted, separating parts, through the walkers' own deposit.
- **Water (wake).** Damp ground and sleep stay a good map rule. Squeeze's hold is its nearest cousin: let part of yourself go hungry now for a bigger move later.

## Against the existing RTS doc (`Reports/rts-design-2026-11-10.md`)

The RTS doc, Bloomfront, is an indirect-control RTS built on the five old tools. It adds paying for tools from the store, spores (Fruit) and a Forage/Hold/Fight switch (Temper). Its species are `Params` presets chosen before a match. It leaves an open slot for contagion as a species rule.

| My idea | In Bloomfront? | What is genuinely new here |
|---|---|---|
| Squeeze | No. Bloomfront never uses food flow, sap or the pulse | All of it: control by timing, waves as the weapon, visible charging as information, distance thinning waves as the catch-up rule. The shared idea is that spending is the decision |
| Two Fevers | Only as a slot: "contact can convert a foe's walker rather than kill it" | The structure: a neutral host as the map, players as contagions, per-strain immunity as the comeback rule, traits that rewire the host |
| Grafts | Partly. Species as `Params` presets, and Temper presets | Rules carried per walker and inherited. Rule sets captured from the map in play, not picked before. Grafting by region. A gate that asks for a counter cycle, where Bloomfront asks each pair to land at 40-60% |
| Spores (#17), Temper (#2), Taste (#19) | Yes: Fruit, Temper, species | Nothing new |
| Ink (#1), Valves (#18), Custody (#15), Clock (#16), Laws (#11), Weather (#8) | No | New, but not chosen |

**Where the two documents meet.** Bloomfront's top risk is a game decided in the opening. Its third risk is indirect control: at 45 degrees the mould obeys then freezes, and at 22.5 the tools drown in rewiring. My three answer the third risk by not steering shape at all.

- Squeeze's waves read the same at either angle.
- Two Fevers turns 22.5's restlessness into the moving map.
- Grafts makes the angle itself the player's choice, region by region.

Each answers the first risk with a structural catch-up rule: waves thin with distance, immunity is per strain, and regimes counter each other. Bloomfront relies on spending, wells and announced blooms instead.

## Recommendation

**Build Squeeze first, as a census probe.** It needs the least new code: food flow, sap, stream, pulse, walls and the rival all exist. One measurement settles the premise: does a hand that times its squeezes beat the same squeezes at random times?
If that fails, timing is not a game here, and the next probe is the Grafts counter matrix. The matrix may need only a sweep.
Two Fevers is the most exciting at GPU scale, but it waits on contagion from phase 1.

## Sources

- [Plague Inc wiki, VS Mode](https://plagueinc.wiki.gg/wiki/Multiplayer/VS_Mode), opened once to check that Two Fevers is not a copy.
- A web search for slime mould strategy games listed several small itch.io and Steam titles. Two examples are *Sprawl Grid: The Logic of Slime* (https://j3rzyc41n3.itch.io/sprawl-grid-the-logic-of-slime) and *Slimepolis* (https://acellular.itch.io/slimepolis). I could not open them: the session's shared fetch budget ran out after one page. Their mechanics are unchecked.
- Repo: `wiki/model/*.md`, `wiki/index.md`, `Reports/jenson-physarum-2026-11-10.md` and contact sheets, `src/mould/mod.rs` (`Params` and model notes), `Reports/dead-ends.md`, and `Reports/rts-design-2026-11-10.md`, read last.
