# Simulation strategy games: lessons for Bloomfront

**Status:** brainstorm and survey only, 2026-11-10. No repo file was edited.
Read against `Reports/rts-design-2026-11-10.md`, `CLAUDE.md`,
`src/mould/mod.rs` and `Reports/dead-ends.md`.

**Scope.** Strategy and strategy-adjacent games whose core is a running
simulation. Creeper World, Particle Fleet and general RTS theory are left to
the other agents.

**How facts are labelled.** Every fact carries the URL it came from. The
session's shared web-fetch budget (100 an hour, all agents) ran out partway
through. Pages I could not fetch are listed apart at the end of the sources.
Anything I state without a fetched page is marked *(from memory,
unverified)*. Treat those as leads to check, not as findings. Still
unverified at the end: Oxygen Not Included entirely, Plague Inc.'s verbs,
Factorio's checksums, and a few single claims marked in place. A
verification pass on 2026-11-10 fetched sources for these; each outcome is
marked in place and tabled in the last section.

## The games at a glance

| Game | Year | What is simulated | How the player acts | Lesson for Bloomfront |
| --- | --- | --- | --- | --- |
| Liquid War | 1995 (net play from 5.4.0, 2001; https://en.wikipedia.org/wiki/Liquid_War) | An army of pixels flowing down a distance field to a cursor; touching foes drain and convert | Moves one cursor; never a unit | A conserved, converting army makes fronts swing; the steering point must move slowly |
| SimAnt | 1991 | Two ant colonies, pheromone trails, castes, hazards | Plays one ant; lays trails others follow | Trail-as-order works; weather that erases trails is an event both sides must plan for |
| Osmos | 2009 | Motes under momentum; bigger absorbs smaller | Ejects own mass to move | Every move costs body; render state continuously, never as a flip |
| Eufloria | 2009 | Trees shed seedlings; ten in orbit make a tree; seedlings drain asteroid cores | Sends seedlings between asteroids | Stats drawn on the body; calm pace scored mixed |
| Galcon | 2006 (iPhone 2008) | Planets that make ships, faster when larger | Sends a chosen share of a planet's ships; redirects in flight | One verb with one dial can carry online play; the node's number is the state |
| Tentacle Wars | iOS HD 2012 | Fixed cells making units; tentacles paid in units | Draws tentacles; swipes to cut | A cut can be a strike: the units in the link land on the target at once |
| They Are Billions | 2019 | Undead swarms; infection spreads once a wall breaks | Builds walls and a colony, places units | "One mistake and it's game over": a cascade needs a visible fuse |
| Mushroom 11 | 2015 | A cellular organism that regrows as you delete it | Deletes cells; splits and shapes the mass | Cutting as a way to steer, not only to harm |
| Jones's guided blob | c. 2015 (research) | A Jones-model Physarum blob | Moves attractant points or a light mask | Repellent light steers faster and truer than attractants; attractants overshoot and stick |
| From Dust | 2011 | Lava, soil, water and plants; erosion | Moves matter with a cursor; tribe walks on its own | Warned disasters are fair; stuck agents are blamed on the game |
| Plague Inc. | 2012 | A disease spreading through world population (verbs confirmed, https://plagueinc.wiki.gg/wiki/DNA_Point) | Buys traits; changes rules, not units | Acting on rules can be strategy if the spread is readable |
| Factorio | 2016 (1.0 2020) | Pollution spreading and absorbed per chunk; spawners spend it on attacks | Builds factory; enemies react to the cloud | The enemy's budget is a field you emit and can see |
| Oxygen Not Included | 2019 | Gas, liquid and heat on a grid | Digs, builds, sets priorities | Overlays that draw the real fields make a chaotic sim readable |
| Noita | 2019-2020 | Every pixel follows simple material rules | Plays one wizard in the world | 64x64 chunks, dirty rects and checker-pattern threads scale a grid sim; unnoticed systems earn nothing |
| PHYSARUM: Slime Mold Simulator | 2021 | Millions of Physarum agents | Edits species and the environment | A slime sandbox with no opponent stays a toy |
| Slimepolis | prototype | A Jones/Jenson slime on a Voronoi map | Builds a city that uses and fights the slime | The mould can be resource and enemy at once |
| FUNG | 2023 | A slime mould as a cellular automaton | Nurtures it in levels | CA rules give puzzles, not contests |
| Spore Valley | 2024 | A slime's metabolite budget on hexes | Turn-based growth choices | Starvation and toxicity as two-sided limits |
| The Largest Organism | 2026 (due) | An agent fungal network | Buys upgrades with trade-offs | Emergent veins sell an idle game |
| Sprawl Grid | in development | Slime-like territory on a 6x6 board | Places stones, rolls mutation dice | Slime plus Go: territory is the natural win |

## Game notes

Each note answers three things. **Sim:** what runs and how the player acts.
**Play:** what reception says is fun or not. **Tech:** any lesson on scale,
determinism, multiplayer or AI.

### Armies that are a fluid or a swarm

#### Liquid War (Thomas Colcombet's algorithm, Christian Mauduit's game, 1995)

- **Origin.** Colcombet invented the shortest-path algorithm "before the game
  itself in Spring 1995". His friend Christian Mauduit "enhanced the algorithm
  and coded the game". Liquid War 3.0 came out on 1 July 1995; network support
  came in 5.4.0, on 7 July 2001 (https://en.wikipedia.org/wiki/Liquid_War).
  *(corrected in verification pass: the heading said Mauduit wrote only later
  versions; he coded the game from the start.)*

- **Sim.** You move a cursor and your army follows. "There are no weapons,
  the only thing you have to do is to move a cursor in a 2-D battlefield"
  (manpage, https://manpages.ubuntu.com/manpages/noble/man6/liquidwar.6.html).
- Fighters take the shortest path to the cursor. The game computes a
  distance field ("gradient") each round, to a 10% error tolerance
  (https://ufoot.org/liquidwar/v5/techinfo/algorithm/en/).
- Fighters first dodge. If blocked, they attack. An attacked fighter "gets
  darker". At zero health it joins the attacker's team, so the total count
  never changes (manpage).
- Stuck teammates heal each other, so big groups regenerate (algorithm page).
- Win: most fighters at the end, or wipe every other team (manpage).
- **Play.** The author's tip: surrounding opponents usually works. A
  contributor: "always move your cursor through the enemy, not the other way
  around" (manpage).
- The computer players are weak. Their named mistake: they never keep the
  cursor still in the middle of their own fighters (manpage).
- **Tech.** The field is computed per zone, not per pixel, with up to 12
  links per zone. It settles over passes, which works because "cursors move
  really slowly" (algorithm page).
- It is poor with many separate targets (algorithm page).
- It looks random but is not: the game "behave[s] the same when given the
  same keyboard input" (algorithm page).
- Net play arrived in 5.4.0. The server relays key presses and stores no
  game state, which limits cheating. Internet play may lag (manpage).
- **For us.** This is the closest ancestor of Bloomfront's control. It shows
  a conserved, converting army can be read and can swing. It also shows
  inputs alone can carry a networked match.

#### SimAnt (Maxis, Will Wright, 1991)

- **Sim.** A black colony against a red one in a back yard, then the house
  (https://en.wikipedia.org/wiki/SimAnt).
- You control one yellow ant and can switch ants. You lay pheromone trails,
  recruit nearby ants, dig, carry and fight (Wikipedia).
- Footsteps, bug spray, lawnmowers and rain are hazards. Rain washes away
  trails (Wikipedia).
- An Experimental mode gives tools to place trails, walls, food and
  pesticide (Wikipedia).
- **Play.** PC Gamer: your "stinky pheromone trail" decides how fellow ants
  spend their time. "You can lay trails across the map that act as alarms,
  give orders..." (https://www.pcgamer.com/saturday-crapshoot-simant).
- The early game is a race for food before the rival nest grows too strong
  (PC Gamer).
- It won Best Simulation at the 1992 Codies. Wright later called it his
  biggest disappointment: he failed to convey "how cool ants are". He found
  it "too simple" (Wikipedia).
- **For us.** Trail-as-command is proven and old. The failure is a warning:
  the wonder of the simulation did not reach players through play.

#### They Are Billions (Numantian Games; console review 2019)

- **Sim.** Swarms of undead against a walled colony. Undead "wandering in"
  can infect survivors. Once a wall is breached, "the infection is spreading
  like wildfire" (Gamereactor, Roy Woodhouse, 2019-07-03,
  https://www.gamereactor.eu/they-are-billions-review/).
- **Play.** Early on, troops handle stragglers, "before long you'll be
  overrun by swarms". Growing one side leaves another exposed (Gamereactor).
- Difficulty is brutal even on easy: "One mistake and it's game over". Yet
  the reviewer kept "playing, losing, and then coming back for more"
  (Gamereactor). Score 6/10.
- Noise draws the undead. Early on "you want to make as little noise as
  possible"; snipers "generate much more noise, and will eventually attract
  lots of undead if they're firing often". A breach can end a long run: the
  guide's author lost a "three-hour-old" colony. The guide's answer is to
  "pause the game when there's a breach, and delay the horde with layers after
  layers of walls" (PC Gamer guide, Vivien Le Coarer, 2018-01-18,
  https://www.pcgamer.com/uk/they-are-billions-guide-5-essential-tips).
- **For us.** Contagion makes one gap a collapse. That is thrilling, and also
  the main complaint. A cascade needs a fuse the defender can see and answer.

### Flows between nodes

#### Galcon (Phil Hassey; desktop 2006, iPhone 2008)

- **Sim.** Owned planets produce ships, larger planets faster. A number on
  each planet shows its ships, or the ships needed to take it
  (https://en.wikipedia.org/wiki/Galcon).
- **Verbs.** You choose what percentage of a planet's ships to send, and can
  redirect them in flight. Win by taking every enemy planet (Wikipedia).
- **Play.** Classic got good reviews and awards; the iPhone versions
  "excellent reviews" (Wikipedia).
- The official guide confirms "The size of the planet indicates its
  production" and the mouse wheel sets "% Ships"
  (http://www.galcon.com/classic/guide.html).
- *(from memory, unverified; still unverified after verification pass)*
  Matches last minutes, and early leads compound. The guide says neither.
- **Tech.** Full multiplayer; iGalcon rooms hold up to four; Galcon 2 plays
  across phones, tablets and PCs (Wikipedia).
- **For us.** One verb with one dial (what share to send) carried a long-lived
  online series. The number on each node is the state itself, drawn plainly.

#### Tentacle Wars (FDG Entertainment; iOS HD reviewed 2012)

- **Sim.** Fixed cells on each stage make units at a fixed rate. There is no
  base, unit or resource building (Gamezebo, Mike Bellmore, 2012-07-26,
  https://www.gamezebo.com/reviews/tentacle-wars-review/).
- **Verbs.** Each cell sends out a few tentacles. Extending one spends the
  cell's units on its length (Gamezebo).
- If the target counterattacks, the two tentacles lock in a tug of war and
  drain each other (Gamezebo).
- If the target cannot block, you swipe to cut your tentacle and send its
  units into the enemy cell "all at once" (Gamezebo).
- Where you cut matters: "Depending on where players 'cut' the tentacles,
  the power is distributed accordingly" (148apps, Kevin Stout, 2012-04-27,
  3.5/5, https://www.148apps.com/reviews/tentacle-wars-hd-review).
  *(from memory, unverified; still unverified after verification pass)* The
  exact split, that a cut near the source pulls the units back home.
- **Play.** It is "all about outmaneuvering and flanking enemy cells": tie one
  cell up while another tentacle hits it. 80/100 (Gamezebo).
- Weak points: multiplayer was "either broken or yet to be implemented";
  some stages demand one exact approach (Gamezebo).
- **For us.** The link is paid for in units, so a long reach is costly. A cut
  is a strike, not only a loss. Locked tentacles that drain each other look
  like our seam fights. Bloomfront's knife needs the same: a cut must do
  something to both sides of it.

#### Eufloria (Rudolf Kremers and Alex May, 2009)

- **Sim.** Dyson trees on asteroids shed seedlings. Ten seedlings in orbit
  make a new tree. Branches grow by a fractal rule, so no two trees match
  (https://www.wikiwand.com/en/articles/Eufloria).
- Seedlings attack by burrowing to an asteroid's core and draining its
  energy. Their Speed, Strength and Energy stats also change how they look
  (Wikiwand).
- Defense trees fire explosive fruit and grow stronger over time (Wikiwand).
- You move forces by "clicking and dragging from asteroid to asteroid"; you
  "can't set rally points for your troops or even select more than one
  asteroid at a time" (Destructoid, Anthony Burch, 2009-10-24, 4/10,
  https://destructoid.com/?p=58482). So you send groups from one asteroid at
  a time, and the reviewer found it "far more tedious than I would have
  expected".
- **Reception.** Mixed: Resolution 6/10, Destructoid 4/10, Gamezebo 3/5. It is
  described as "simple but charming", with calming music and play (Wikiwand).
- **Play.** Kremers called it "an ambient game of conquest and intergalactic
  gardening". It began as an asteroid-mining idea flipped into growing things
  on asteroids (https://alternativemagazineonline.co.uk/2010/09/11/interview-in-conversation-with-rudolf-kremers-co-creator-of-eufloria/).
- Destructoid's complaint was waiting, not slow battles. Campaign missions
  are "frustratingly slow", the enemy AI is "so docile for about 80% of the
  campaign mode", and you "sit around and wait, in fifteen-minute
  increments". Real strategy is needed only in the final levels (Destructoid,
  above). *(corrected in verification pass: the earlier note said battles
  were slow and the middle game long; the review blames idle waiting and a
  docile AI.)*
- **For us.** A stat that shows on the body (seedling looks follow stats)
  obeys our picture rule. Ambient pace drew mixed scores; calm can tip into
  watching.

### Mass as the currency

#### Osmos (Hemisphere Games, 2009)

- **Sim.** Motes drift under momentum. "Absorb or be absorbed." Propulsion
  is "tied to size": you move by ejecting mass, so every move costs life
  (Growing Osmos, IGS 2010, https://www.hemispheregames.com/misc/GrowingOsmos_IGS2010_final_print.pdf).
- **Play.** The ambient levels had "minimal time pressure", which rewards
  patience and planning (slides).
- Time warp and zoom were player powers. Solar-system levels needed physics
  tweaks to fit play time (slides).
- An early render was binary: a mote was in a state or not, then
  "ASPLOSION!". It felt abrupt. A continuous render of the mote's state, with
  warnings, "avoids frustration" (slides).
- **Tech.** Difficulty was a set of functions of the level number. The team
  built challenging AI opponents early (slides).
- **For us.** Osmos is the purest case of "the store that grows you pays for
  every act". It also backs the owner's rule against binary pictures.

### Growing organisms you steer

#### Mushroom 11 (Untame, Itay Keren, 2015)

- **Sim and verb.** You control a green mushroom by "deleting parts of cells
  while it simultaneously regrows". It "can be split and shaped to solve
  puzzles" (https://en.wikipedia.org/wiki/Mushroom_11).
- Every erased cell regrows: the mushroom can "regrow a new cell for every
  destroyed cell". To move left you erase on its right. But cells "regenerate
  along any open cell surface, [so] some will sprout upward rather than
  opposite the cursor", and pushing hard splits the mass, so you sweep the
  cursor to herd the growth (Destructoid hands-on, Darren Nakamura,
  2014-04-14, https://www.destructoid.com/unlike-with-most-fungi-i-enjoyed-mushroom-11-growing-on-me/).
  *(corrected in verification pass: the earlier note said regrowth appears on
  the far side; it appears on any open surface.)*
- It began at the 2012 Global Game Jam (Wikipedia).
- **Play.** Critics praised its "unique mechanic and theme". IGN 9/10; GameSpot
  6/10; Metacritic 78 (PC) and 80 (iOS). IGF 2014 Excellence in Design
  finalist (Wikipedia). The page gives no reason for the low scores.
- **For us.** Destruction is the steering verb, and the mass is conserved
  cell for cell (now confirmed, Destructoid above). Steering is coarse, because
  regrowth goes to any open edge. In Bloomfront the knife could steer your own
  mould, not only raid theirs.

#### Jones's guided Physarum blob (research, c. 2015)

- **Sim.** Jeff Jones (UWE Bristol) guided a Jones-model blob along a course
  (https://arxiv.org/pdf/1511.07654).
- **Two ways to steer.** One is a point attractant at the next node. The
  other is a light mask that repels the blob from all but its target (paper).
- **Result.** "Guidance by repellent stimuli (a light illumination mask)
  provides faster and more accurate guidance" (paper).
- Attractant guidance overshot corners. Under strong momentum "the blob
  'crashes' into the circular obstacle". It got entwined on an obstacle and
  cycled there; noise did not free it (paper).
- Light without oscillation was far too slow: over 80,000 steps against
  1,900-8,000 with oscillation (paper).
- A persistence parameter "may therefore be used to control the momentum of
  the collective" (paper).
- **For us.** Our lamp is a repellent and our oats an attractant. Jones's
  result predicts the lamp is the precise tool and oats the coarse one. It
  also suggests momentum is a lever worth exposing.
- **Caution from our dead ends.** Our own sensor-repellent lamp was rejected:
  it cut the tube it meant to steer (`Params::light_weight` entry). Today's
  lamp pushes trail and walkers instead. Jones's result must be re-checked
  on the push lamp, not assumed.

### Gods over a running world

#### From Dust (Ubisoft Montpellier, Éric Chahi, 2011)

- **Sim.** Lava, soil, water and vegetation. Lava cools into rock, plants
  spread through soil, and moving water erodes terrain
  (https://en.wikipedia.org/wiki/From_Dust).
- "Physical changes to the world occur extremely rapidly, allowing players
  to restructure islands within minutes" (Wikipedia).
- **Verbs.** "With a spherical cursor, the user controls certain types of
  matter in real time." The tribe follows the Breath to totems and builds
  villages beside them (Wikipedia).
- Shamans warn of tsunamis and eruptions ahead. You answer by jellifying a
  tsunami, putting out fires or diverting lava (Wikipedia).
- **Play.** Metacritic 80 (Xbox 360), 81 (PS3), 76 (PC). Critics praised the
  physics and simulation (Wikipedia).
- The main gameplay complaint: tribespeople "have trouble pathfinding and are
  often stuck in places" (Wikipedia).
- GameSpot agrees and adds a limit to the warnings. "There is usually a
  warning when a tsunami will hit", but an unseen spring can still drown a
  village. The tribe is "sometimes stymied by a puddle of water or a small
  hitch in the terrain", and the pacing is "uneven" (GameSpot review, no
  byline in the fetched text, https://gamespot.com/reviews/from-dust-review/1900-6325105/;
  verification pass). Ranked item 2 stands, with one sharpening: every event
  must be announced, because the unwarned ones were the complaint.
- **For us.** Two lessons. Warned disasters give a god game its rhythm, and
  the warning is what makes them fair. And when the agents you cannot
  command get stuck, players blame the game. Bloomfront's mould must show
  why it goes where it goes.

#### Plague Inc. (Ndemic Creations, James Vaughan, 2012)

- **Origin.** "A key inspiration ... came from a 2008 Flash game called
  Pandemic 2." Vaughan added more strategic choices, graphs and data lists,
  and more realism (PocketGamer.biz, Simon Parkin, 2013-03-12,
  https://www.pocketgamer.biz/feature/49231/plague-vs-pandemic-how-james-vaughans-ios-hit-plague-inc-took-the-market-from-its-inspiration/).
- Vaughan: "There is no honour in mindlessly copying someone else's game."
  He built the algorithms in Excel before a team built the engine
  (PocketGamer.biz).
- **Sim and verbs.** You spend DNA points "to evolve different traits such
  as abilities, symptoms or transmissions", and on active abilities. Each
  trait evolved raises the price of the next by 1. You never buy a unit
  (Plague Inc. wiki, https://plagueinc.wiki.gg/wiki/DNA_Point).
- A cure races you: "there is a time pressure to complete the game before
  humans (the opponent) develop a cure". The spread runs on "an epidemic
  model with a complex and realistic set of variables"
  (https://en.wikipedia.org/wiki/Plague_Inc.).
- The disease crosses between countries by air, sea and land. Island
  countries "can usually only be infected via their seaports", most have no
  airport, and land borders carry it between neighbours
  (https://plagueinc.wiki.gg/wiki/Island_Countries).
- **For us.** Rule-level verbs (our Temper switch) can be strategy if the
  change shows on the map soon after. Tuning the model in a spreadsheet first
  is the same move as our `mouldprobe` census.

### Fields and fluids

#### Factorio (Wube Software, 2016; 1.0 in 2020)

- **Sim.** Pollution is a cloud "updated per chunk every 64 ticks". Machines
  emit it at fixed rates: a boiler 30 a minute, a stone furnace 2
  (https://wiki.factorio.com/Pollution).
- Above 15 per chunk it spreads "in all four cardinal directions" at 2% per
  64 ticks (wiki).
- Ground and trees absorb it. Water takes about 1.536 per chunk per minute,
  grass 1.106, paths none. Trees over 60 pollution lose leaves or turn grey
  (wiki).
- **The enemy is a reaction.** Over 20 pollution, a spawner absorbs
  20 + 1% of the chunk's pollution every 64 ticks. Absorbed pollution buys
  attackers: a small biter costs 4, a behemoth 400 (wiki).
- The cloud is drawn as a "blocky red cloud" overlay. Trees greying is the
  field showing on the world itself (wiki).
- One drawn-only effect: water near a source turns green, "visual only"
  (wiki). Our owner's ruling would forbid that one.
- **Play.** The enemy's anger is your own footprint: "biters and spitters
  will only aggressively engage the player's factory if the pollution cloud
  of the factory reaches a nest", and "pollution is consumed to send biters
  or spitters to join the next attack". Evolution rises slightly with time,
  with "global pollution production", and "significantly" when you destroy
  nests (https://wiki.factorio.com/enemies). *(from memory, unverified;
  still unverified after verification pass)* Players plan expansion against
  the cloud.
- **Tech.** Multiplayer is lockstep, peer to peer. "Each of the networked
  peers is running the simulation of everything that happens in the world"
  (Friday Facts 76, 2015-03-06, https://factorio.com/blog/post/fff-76).
- Only actions travel: "only the action is somehow transferred to all other
  players". Local actions wait out a set latency buffer (FFF 76).
- Small errors accumulate until both players "see a completely different
  game": a desync. The game runs at the slowest peer's pace (FFF 76).
- Desyncs are caught by checksum. A fast CRC, "incomplete but very fast", is
  taken every tick, and "the CRC checks are compared for all the players in
  the game for every tick". A full game-state CRC every tick is the heavier
  check used to debug (Friday Facts 55, 2014-10-10,
  https://factorio.com/blog/post/fff-55).
- FFF 47, "CRC fun" (2014-08-15, https://direct.factorio.com/blog/post/fff-47),
  is a test mode, not live play. It takes "a CRC from the whole map every
  tick" and saves every tick, then replays the save. The first tick whose CRC
  differs narrows the bug to "typically just a value of one variable". At the
  time: "We haven't yet quite figured out how we will handle multiplayer
  desync problems reported by users." *(corrected in verification pass: the
  earlier note read FFF 47 as the live multiplayer check; FFF 55 is that.)*
- **For us.** Attacks are paid from a field the player emits and can see.
  The enemy's budget is the player's own pollution. That is readable cause
  and effect with no scripted waves.

#### Oxygen Not Included (Klei Entertainment, 30 July 2019)

- **Sim.** Air moves through the base in real time, and temperature is "a
  constant concern in space; too cold and you'll freeze, too hot and you'll
  fry" (Steam, https://store.steampowered.com/app/457140/Oxygen_Not_Included/).
  The world is cells: "only a single solid cell, tile, liquid or gas can
  occupy a cell at any given time", and liquids and gases swap places by set
  rules (https://oxygennotincluded.wiki.gg/wiki/One_element_per_cell_rule).
  *(from memory, unverified; still unverified after verification pass)* Heat
  also moves cell by cell.
- Colonists' work has priorities: the game has a Priority overlay
  (https://oxygennotincluded.wiki.gg/wiki/Overlay). How a colonist picks its
  next task was not checked.
- **Play.** The game has 16 overlays. Oxygen "Displays ambient oxygen
  density"; Temperature "Displays ambient temperature"; others show power,
  germs, decor, plumbing and more (wiki). The store page sells the overlay
  itself: "Enter the Oxygen Overlay and watch air moving through your base in
  real time" (Steam). *(from memory, unverified; still unverified after
  verification pass)* Players learn the sim by reading them.
- **For us.** An overlay that draws a real field obeys "the picture is the
  data". A sensor-field view of the trail is allowed under that rule.

#### Noita (Nolla Games; early access 24 September 2019, 1.0 15 October 2020, https://noitagame.com/press/)

- **Sim.** Every pixel follows "rather simple rules", and combined they
  surprise (80.lv interview, Daria Loginova, 2019-04-05,
  https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/).
- **Play.** The wand system "creates a lot of interesting situations and
  combinations" (80.lv).
- Ambient systems (grass, moss, freezing, decay) are real. But "when you're
  playing the game you rarely notice them", so the team spent little on them
  (80.lv).
- Herringbone Wang tiles let the team keep enough control of layout to pace
  the game (80.lv).
- **Tech.** The world is split into 64x64 chunks. "Each chunk keeps a dirty
  rect to indicate which pixels need to be updated" (80.lv).
- Threads update the world "in a checker pattern of sorts", in four passes.
  A pixel may move up to 32 pixels past its chunk's edge. So "every pixel
  inside that area is only updated by that thread", with no locks (80.lv).
- Updates run bottom-up so falling rows stay consistent (80.lv).
- Petri Purho's GDC 2019 talk, "Exploring the Tech and Design of 'Noita'",
  covers "scaling up the falling sand simulation to support large continuous
  worlds". The talk itself is members-only; only its description was read
  (https://gdcvault.com/play/1025695/Exploring-the-Tech-and-Design;
  verification pass).
- **For us.** This is the known recipe for Bloomfront's big-map stage. Our
  walkers move one cell a step and the trail spreads 3x3, so the margin
  needed is far smaller than Noita's 32. Also note the readability lesson: a
  real system the player does not notice earns nothing.

### Slime mould and fungal games

Six titles were checked. None is a competitive real-time strategy game.

- **PHYSARUM: Slime Mold Simulator** (Ka Hin Yuen, 2021). Millions of
  particles in a "multi-agent chemoattractant system". You tweak and mutate
  species in real time and build environments from images or a webcam. 90%
  positive from 21 reviews (https://store.steampowered.com/app/1667120).
- **Slimepolis** (acellular, prototype). A city builder on a slime adapted
  from Sage Jenson's version of Jones's model, modified to a Voronoi diagram.
  The slime is "the same evolving slime mold": your resource and your enemy.
  Half-hour sessions (https://acellular.itch.io/slimepolis).
- **FUNG** (Mellow Pty Ltd, 2023, early access). A slime mould as a cellular
  automaton, "every pixel simulated", over 100 totalistic rules. Tagged
  puzzle and tower defence. 5th at LÖVE Jam 2023. Last update over two years
  ago (https://store.steampowered.com/app/2467310).
- **Spore Valley** (Lystrosaurus Studio, 2024). Turn-based resource puzzle.
  Five metabolites; running out starves body parts and excess turns toxic.
  15 forms on a hex grid (https://store.steampowered.com/app/3240350).
- **The Largest Organism** (BINAN Games, due Q4 2026). An idle game on a
  live agent fungal network. Veins and highways emerge unscripted; the colony
  retreats when the air dries (https://store.steampowered.com/app/5143760).
- **Sprawl Grid: The Logic of Slime** (j3rzyc41n3, in development). A
  two-player print-and-play game "inspired by slime mold behavior" and Go,
  on a 6x6 board (https://j3rzyc41n3.itch.io/sprawl-grid-the-logic-of-slime).

**For us.** The slime genre so far is sandbox, idle, city builder, puzzle and
board game. The beauty sells. Nobody has made the mould fight a thinking
opponent in real time.

## Synthesis

### What works when the army is a simulation

1. **One stock is body, army and wallet.** Osmos spends mass to move. Liquid
   War conserves fighters by conversion. Galcon and Eufloria spend the units
   that make more units. Bloomfront's food store already is this.
2. **One steering point, not many orders.** Liquid War's cursor, SimAnt's
   trail and Jones's light mask all steer a mass with one input. Liquid
   War's field only settles because cursors move slowly. Jones's attractant
   overshot when it jumped from node to node.
3. **Fights that convert or drain read as a tide.** Liquid War fighters
   darken, then change sides. Fronts swing visibly instead of vanishing.
4. **State shown continuously.** Osmos found binary renders frustrate and
   continuous ones with warnings do not. This matches the owner's ruling.
5. **The rival reacts to fields you emit.** Factorio's spawners buy attackers
   with the pollution that reaches them. Cause and effect are on the map.
6. **Inputs are tiny, so lockstep works.** Liquid War relays key presses and
   replays the same from the same input. Factorio sends only actions while
   every peer runs the whole world. The cost: everyone waits on the slowest.
7. **The world, not the opponent, makes the mid-game.** SimAnt's rain washes
   trails away. From Dust's shamans warn of tsunamis and eruptions
   before they land. These events reset positions that the opening fixed. This
   is the answer to Bloomfront's top risk, a match decided in the opening.
8. **The link costs what it carries.** A Tentacle Wars tentacle is paid in
   units for its length. Reach is a spend, so over-extension is a choice.
   Our `feed_reach` and upkeep already make long tube costly.

### Common failure modes

1. **Watching, not playing.** Sandboxes with no opponent (PHYSARUM) and
   ambient games (Eufloria scored 4/10 to 6/10) drift here. Destructoid's
   4/10 named the cause: waiting "in fifteen-minute increments" against a
   docile AI (verification pass). SimAnt was
   "too simple".
2. **The agents look stupid, and the game takes the blame.** From Dust's
   tribespeople "have trouble pathfinding and are often stuck in places".
   Liquid War's fighters do not route round other fighters, so they get
   blocked (manpage). There it is a rule, but it costs readability.
3. **Snowball.** Bigger absorbs smaller in Osmos. In Galcon larger planets
   make ships faster. A sim with growth rewards whoever is ahead.
4. **Illegible or unnoticed systems.** Noita's team found players "rarely
   notice" its ambient systems and stopped investing in them. ONI leans on
   16 overlays, oxygen and temperature among them
   (https://oxygennotincluded.wiki.gg/wiki/Overlay). A system earns its cost only if it shows.
5. **Cascades with no fuse.** In They Are Billions, once a wall is breached
   infection spreads "like wildfire". "One mistake and it's game over."
6. **One point cannot be everywhere.** Liquid War's field is poor with many
   targets. Jones's attractant blob stuck on an obstacle and cycled there.

### Lessons for Bloomfront, mapped to our levers

Rulings: **F** fun over fidelity; **P** the picture is the data (no
drawn-only effects); **T** every tool shows something; **S** an outcome is a
spread, not a yes/no. Costs: small is a census or a `Params` change; medium
is new rules in `mod.rs`; large is a restructure.

| # | Lesson (from) | Our levers | Concrete idea | Ruling | Cost |
| --- | --- | --- | --- | --- | --- |
| 1 | Spend is body (Osmos) | food store, `pieces`, `Walker::food` | Pay each tool from the piece of mould nearest the click. That piece visibly thins and greys. A click far from your mould has no one to pay, so tools have a natural range. | P, T | Medium |
| 2 | Repel beats lure for precision (Jones 2015) | lamp `push`, oats | Price the lamp as the precise, dear tool and oats as the cheap, coarse one. First census: lamp-led against oat-led fronts round a corner, paired seeds. | F, S | Small |
| 3 | Momentum is a lever (Jones 2015) | `sensor_angle`, `settle_seconds`, a new random-turn chance | Make Temper set persistence as well as angle. Forage wanders; Fight keeps its heading. The 22.5 against 45 degree census is the baseline. | F | Small |
| 4 | Conversion swings fronts (Liquid War) | `fight_losses`, `remove_walker`, walker lists | Today a fight death drops the loser's carried food. A species whose kills join it, keeping that food, fills the design's fifth-species slot. Watch the snowball; Liquid War's brake is that groups heal. | P, S | Medium |
| 5 | A cut is a strike (Tentacle Wars) | knife, `wound`, `fight_losses` fed factor | A cut behind their front should flip it from fed (losses x0.5) to unfed (x2.0). That is a 4x swing, due at the next census (every 6 steps). Measure fight deaths at a front in the 4 s after a cut behind it. If the swing is real, the knife already bites and only the heal time needs tuning. | T, S | Small to measure; Medium to tune |
| 6 | Cutting steers (Mushroom 11) | knife on own tube, `pieces` share, `remove_walker` | Today a snip deletes walkers and the food they carry (`remove_walker` drops `Walker::food`). Pruning still steers if a piece's food is shared: cutting off a dead branch leaves more per walker at the front. Measure the front's divisions in the 10 s after a prune, paired against no prune. Verification pass: Mushroom 11 conserves cells one for one but regrows on any open edge, so its steering is coarse; the lesson stands. | F, T | Small to measure |
| 7 | A visible fuse (They Are Billions) | `starve`, `hungry`, grey creep | Tune starvation beyond a cut so the defender gets 5-10 s of visible grey to answer with oats or water. | P, S | Small |
| 8 | Brakes on the snowball (Osmos, Galcon) | `table`, `upkeep`, fed-tube fight bonus | Keep the table cap: a pile feeds 40 walkers whoever is bigger. Deciding measure: lead changes per match and how often the 2:00 leader wins. | S | Small |
| 9 | React to emitted fields (Factorio) | trail, scent, `foe_trail` | AI keepers sense only through the fields the player can see. Neutral wild colonies wake when your trail reaches them. | P | Small (AI) / Medium (neutrals) |
| 10 | Show why the agents move (From Dust) | trail plus scent plus foe trail in the sensors | A lens drawing what a walker senses. It is a real field, so the rule holds. | P | Small-Medium |
| 11 | Continuous state (Osmos) | fed multiplier 0.5 vs 2.0 in `fight_losses` | Draw front walkers by carried food, so a starving front looks weak before it breaks. | P | Small |
| 12 | Weather erases orders (SimAnt) | wet field, trail, Water | An announced shower over a band or disc that wipes trail and soaks the ground, waking sleepers. Announced, it meets the late-piles dead end's re-test condition (a preview). Not plate-wide: that would split every network, like the rejected three-cut storm (`STORM_CUTS`). | P, S | Medium |
| 13 | Same input, same game (Liquid War, Factorio) | seeded `Rng`, single-thread step | Print a per-second hash of walkers and stores in `mouldprobe`, so drift is caught years before netcode. | — | Small |
| 14 | Build AI early (Osmos; Liquid War's weak AI) | `Player`, warden, `HJIT`, `HLAG` | AI against AI on paired seeds from Stage 0. Teach the AI Liquid War's lesson: park the lamp or oats in your own mass to thicken a front. | S | Small |
| 15 | Step only what moves (Noita) | `spread`, `step_own` | Tile the plate; skip tiles with no walker and trail near zero. Trail at keep 0.90 falls from 5 to below 1e-6 in about 146 steps (1.2 s); scent at 0.97 takes about 4.6 s (my arithmetic). Piles keep scent alive, so their tiles never rest. Thread in a fixed checkerboard to keep the step order, and so determinism. | — | Large |

**Lesson 16, chaos against control (Liquid War).** Liquid War feels obedient
because every fighter reads one global field to the cursor. Our walkers feel
only 9 cells ahead, plus the scent field. So how far a lure is felt sets how
responsive the mould feels. The `sensor_offset` dead end agrees: shorter
reach sensed oats from less far, and every planned hand got worse. A census
of oat scent reach against time-to-obey, paired seeds, is small. Ruling F.

### The eight to try first, ranked

1. **Pay each tool from the mould nearest the click** (Osmos). Spend shows
   as thinning; tools gain a natural range. Medium. P, T.
2. **Announced, local world events make the mid-game** (From Dust, SimAnt).
   A forecast shower over a band, never plate-wide. Medium. P, S.
   Verification pass: GameSpot says From Dust warns of tsunamis "usually",
   and its unwarned threats drew the complaint, so announce every event.
   Rank unchanged.
3. **Measure whether a cut is already a strike** (Tentacle Wars). Fight odds
   behind a cut should swing 4x. Small to measure. T, S.
4. **The rival reacts only to fields you emit** (Factorio). AI keepers sense
   what the player sees; neutrals wake on your trail. Small to medium. P.
5. **Repel for precision, lure for reach** (Jones 2015, Liquid War). Census
   lamp-led against oat-led fronts, and oat reach against time-to-obey.
   Small. F, S.
6. **A converting species** (Liquid War). Kills join the killer with their
   food. Medium. P, S.
7. **A per-second state hash in `mouldprobe` now** (Liquid War, Factorio).
   Small. Verification pass: confirmed and sharpened. Factorio compares a
   fast CRC every tick across players (FFF 55), and its debug mode finds a
   bug from the first tick whose CRC differs (FFF 47). A per-step hash would
   pin drift to one step; the rank is unchanged.
8. **Dirty-tile, checker-pattern stepping for big maps** (Noita). Large.

### Gaps Bloomfront could own

1. **A competitive real-time game on a live Physarum model.** The six slime
   games found are sandbox, idle, city builder, puzzle and board game. Jones's
   blob is a lab result. None pits two moulds and two thinking players.
2. **The army is the road.** In node-flow games the link is a drawn line,
   apart from the units. In Bloomfront the tube is both the soldiers and the
   supply line. Cutting it is a kill and a raid in one act.
3. **Shared verbs that touch both sides.** Liquid War's cursor pulls only your
   own army. Bloomfront's lamp shoves any mould. Pushing their tube off a
   pile is a move no surveyed game offers.
4. **A battlefield that remembers.** With `wall` memory (flow level 3, off
   by default), busy routes thicken and fade over 30 s half-lives. Old routes become terrain. Foe trail repels,
   so a retreating mould leaves a short-lived border behind.
5. **An honest picture as the competitive skill.** Factorio draws its
   pollution as a toggled overlay and tints water green for show. Bloomfront
   draws nothing but the data, so reading the dish is the skill. Fog drawn
   from sensor reach fits.

## Sources

**Fetched and used**

- Liquid War core algorithm: https://ufoot.org/liquidwar/v5/techinfo/algorithm/en/
- Liquid War manpage (Ubuntu noble): https://manpages.ubuntu.com/manpages/noble/man6/liquidwar.6.html
- SimAnt, Wikipedia: https://en.wikipedia.org/wiki/SimAnt
- SimAnt, PC Gamer (Richard Cobbett): https://www.pcgamer.com/saturday-crapshoot-simant
- Growing Osmos, IGS 2010 slides: https://www.hemispheregames.com/misc/GrowingOsmos_IGS2010_final_print.pdf
- Eufloria, Kremers interview: https://alternativemagazineonline.co.uk/2010/09/11/interview-in-conversation-with-rudolf-kremers-co-creator-of-eufloria/
- Jones, guidance of a Physarum model: https://arxiv.org/pdf/1511.07654
- PHYSARUM: Slime Mold Simulator, Steam: https://store.steampowered.com/app/1667120
- FUNG, Steam: https://store.steampowered.com/app/2467310
- Spore Valley, Steam: https://store.steampowered.com/app/3240350
- The Largest Organism, Steam: https://store.steampowered.com/app/5143760
- Slimepolis, itch.io: https://acellular.itch.io/slimepolis
- Sprawl Grid, itch.io: https://j3rzyc41n3.itch.io/sprawl-grid-the-logic-of-slime
- Tentacle Wars review, Gamezebo (Mike Bellmore, 2012): https://www.gamezebo.com/reviews/tentacle-wars-review/
- From Dust, Wikipedia: https://en.wikipedia.org/wiki/From_Dust
- Factorio pollution, official wiki: https://wiki.factorio.com/Pollution
- Noita, 80.lv interview (Daria Loginova, 2019): https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/
- Mushroom 11, Wikipedia: https://en.wikipedia.org/wiki/Mushroom_11
- Galcon, Wikipedia: https://en.wikipedia.org/wiki/Galcon
- They Are Billions review, Gamereactor (Roy Woodhouse, 2019): https://www.gamereactor.eu/they-are-billions-review/
- Factorio Friday Facts 76, MP inside out (2015): https://factorio.com/blog/post/fff-76
- Eufloria, Wikiwand: https://www.wikiwand.com/en/articles/Eufloria
- Plague Inc., PocketGamer.biz (Simon Parkin, 2013): https://www.pocketgamer.biz/feature/49231/plague-vs-pandemic-how-james-vaughans-ios-hit-plague-inc-took-the-market-from-its-inspiration/

**Found by search, not fetched (budget ran out)**

- Oxygen Not Included: fetched in the verification pass (Steam page and the
  wiki's Overlay page, both listed in the verification table).

- Physarium, itch.io (fetch failed at the site): https://steffko.itch.io/physarium/purchase
- Osmos review, Pocket Gamer: https://www.pocketgamer.com/osmos/review-4123/
- Galcon guide: https://www.galcon.com/classic/guide.html (fetched in the verification pass)
- Factorio Friday Facts 286: https://www.factorio.com/blog/post/fff-286 ; FFF 47, CRC fun: https://direct.factorio.com/blog/post/fff-47 (FFF 47 fetched in the verification pass)
- Mushroom 11, Destructoid review: https://www.destructoid.com/reviews/review-mushroom-11/
- GameSpot review of From Dust: https://gamespot.com/reviews/from-dust-review/1900-6325105/ (fetched in the verification pass)
- Noita, GDC Vault talk: https://gdcvault.com/play/1025695/Exploring-the-Tech-and-Design (description fetched in the verification pass)

## Verification pass (2026-11-10)

A later pass fetched sources for every claim marked from memory, the "still
unverified" list at the top and the pages found but not fetched. The shared
fetch budget ran out several times during it, so a page that failed twice
stays unverified.

| Item | Outcome | Source |
| --- | --- | --- |
| Liquid War "mid-1990s" | Confirmed: 3.0 on 1 July 1995; net play 5.4.0 on 7 July 2001 | https://en.wikipedia.org/wiki/Liquid_War |
| Liquid War heading: Mauduit wrote only later versions | Corrected: Colcombet made the algorithm, Mauduit "coded the game" from the start | https://en.wikipedia.org/wiki/Liquid_War |
| They Are Billions: noise draws hordes; a late breach ends a long run | Confirmed (sniper noise "will eventually attract lots of undead"; a "three-hour-old" colony lost) | https://www.pcgamer.com/uk/they-are-billions-guide-5-essential-tips |
| Galcon: matches last minutes; early leads compound | Still unverified; the official guide says neither | http://www.galcon.com/classic/guide.html |
| Tentacle Wars: a cut near the source pulls units back | Confirmed in part (where you cut decides how units are shared); the exact rule is still unverified | https://www.148apps.com/reviews/tentacle-wars-hd-review |
| Eufloria: you send groups between asteroids | Confirmed (drag "from asteroid to asteroid", one asteroid at a time) | https://destructoid.com/?p=58482 |
| Eufloria: battles slow, middle game long | Corrected: the complaint is idle waiting and a docile AI until the last levels | https://destructoid.com/?p=58482 |
| Mushroom 11: regrowth appears on the far side | Corrected: one new cell per erased cell, on any open surface | https://www.destructoid.com/unlike-with-most-fungi-i-enjoyed-mushroom-11-growing-on-me/ |
| Plague Inc.: you spend points on traits, never a unit | Confirmed (DNA points buy abilities, symptoms, transmissions) | https://plagueinc.wiki.gg/wiki/DNA_Point |
| Plague Inc.: a cure races you | Confirmed | https://en.wikipedia.org/wiki/Plague_Inc. |
| Plague Inc.: spread between countries by air, sea and land | Confirmed (seaports, airports, land borders) | https://plagueinc.wiki.gg/wiki/Island_Countries |
| Factorio: the enemy's anger is your own footprint | Confirmed (attacks only once pollution reaches a nest) | https://wiki.factorio.com/enemies |
| Factorio: players plan expansion against the cloud | Still unverified | — |
| Factorio: desyncs caught by comparing checksums | Confirmed: a fast CRC compared across all players every tick | https://factorio.com/blog/post/fff-55 |
| FFF 47 "CRC fun" as the multiplayer check | Corrected: a debug mode that CRCs the whole map every tick on replay; ranked item 7 sharpened, not changed | https://direct.factorio.com/blog/post/fff-47 |
| ONI: Klei, 2019 | Confirmed: 30 July 2019 | https://store.steampowered.com/app/457140/Oxygen_Not_Included/ |
| ONI: overlays draw the real fields (oxygen, temperature) | Confirmed (16 overlays; "watch air moving through your base in real time") | https://oxygennotincluded.wiki.gg/wiki/Overlay |
| ONI: gases, liquids and heat move cell by cell | Confirmed in part: one element per cell; heat per cell still unverified | https://oxygennotincluded.wiki.gg/wiki/One_element_per_cell_rule |
| ONI: colonists act on priorities you set | Confirmed in part (a Priority overlay exists) | https://oxygennotincluded.wiki.gg/wiki/Overlay |
| ONI: players learn the sim by reading overlays | Still unverified (the store page sells the overlay, which is suggestive) | https://store.steampowered.com/app/457140/Oxygen_Not_Included/ |
| Noita dates | Confirmed: early access 24 September 2019, 1.0 15 October 2020 | https://noitagame.com/press/ |
| Galcon guide (found, not fetched) | Fetched; see Galcon rows | http://www.galcon.com/classic/guide.html |
| FFF 47 (found, not fetched) | Fetched; see FFF 47 row | https://direct.factorio.com/blog/post/fff-47 |
| From Dust, GameSpot review (found, not fetched) | Fetched; confirms warnings ("usually") and stuck tribespeople; sharpens ranked item 2, rank unchanged | https://gamespot.com/reviews/from-dust-review/1900-6325105/ |
| Noita, GDC 2019 talk by Petri Purho (found, not fetched) | Description fetched; the talk covers scaling the sand sim to "large continuous worlds"; the talk itself is members-only | https://gdcvault.com/play/1025695/Exploring-the-Tech-and-Design |
| Physarium, itch.io; Osmos review, Pocket Gamer; FFF 286; Mushroom 11 review, Destructoid (found, not fetched; nothing rests on them) | Still unverified: not fetched, budget kept for claims in use | https://steffko.itch.io/physarium/purchase ; https://www.pocketgamer.com/osmos/review-4123/ ; https://www.factorio.com/blog/post/fff-286 ; https://www.destructoid.com/reviews/review-mushroom-11/ |

**Corrections and ranked items.** No correction changes a rank. Three
sharpen a ranked item or a lesson:

- Ranked 7, state hash: Factorio compares a CRC every tick, and FFF 47's debug
  mode finds the first bad tick. A per-step hash would pin drift to one step.
- Ranked 2, announced events: From Dust warned "usually", and the unwarned
  threats were the complaint. Announce every event.
- Lesson 6, cutting steers: Mushroom 11 does conserve cells, but they regrow on
  any open edge, so its steering is coarse. The lesson stands.

The Eufloria correction, waiting and a docile AI rather than slow battles,
strengthens failure mode 1 ("watching, not playing").
