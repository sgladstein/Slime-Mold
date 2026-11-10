# Creeper World and Particle Fleet: Emergence as inspiration for Bloomfront

Brainstorm only, 2026-11-10. Nothing in the repository was changed. Read
against `Reports/rts-design-2026-11-10.md`, `CLAUDE.md`, `Reports/dead-ends.md`
and `src/mould/mod.rs` / `level.rs` / `look.rs` as they stand.

**How to read the sources.** Every fact carries its link. Much of what the
developer said lives in the comment threads of the Knuckle Cracker blog, so
"developer reply" means their answer under a post, not the post body. Vaporlens
pages are automated summaries of Steam reviews; their mention counts are
theirs, not mine. Anything not fetched is labelled "from memory, unverified".

## 1. The games in one table

| Game | Released | The enemy | Player builds | Win condition |
| --- | --- | --- | --- | --- |
| Creeper World | 27 Jul 2009 ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Creeper, a fluid that floods low ground first ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Collectors and relays tied to a home city; blasters and mortars ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Connect to totems, open a gate ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) |
| Creeper World 2 | 16 May 2011 ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Side-view creeper with "invisible steps" for gravity ([KC blog p=108](https://knucklecracker.com/blog/?p=108)) | Reactors, flying packets, beacons; anti-creeper from ore ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Destroy generators with nullifiers ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) |
| Creeper World 3 | 4 Oct 2013 direct ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)); Steam 27 Mar 2014 ([Steam](https://store.steampowered.com/app/280220)) | Creeper on "terraformable terrain" ([Steam](https://store.steampowered.com/app/280220)) | Command node, collectors, relays, reactors, cannons, mortars, sprayers ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)) | Nullify emitters, collect a key ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)) |
| Particle Fleet: Emergence | 29 Sep 2016 ([Steam](https://store.steampowered.com/app/422900?l=english)) | "Particulate", a physics particle swarm ([Steam](https://store.steampowered.com/app/422900?l=english)) | A small fleet of modular, self-repairing ships ([Steam](https://store.steampowered.com/app/422900?l=english)) | Destroy the particle sources ([Awkward Mixture](https://awkwardmixture.blogspot.com/2020/06/particle-fleet-diminishment.html)) |
| Creeper World 4 | 3 Dec 2020 ([Steam](https://store.steampowered.com/app/848480/Creeper_World_4)) | 3D creeper "with wave mechanics" ([Steam](https://store.steampowered.com/app/848480/Creeper_World_4)) | Land, air and orbital weapons ([KC CW4 page](https://knucklecracker.com/creeperworld4/cw4.php)) | Campaign, community maps, generator ([Steam](https://store.steampowered.com/app/848480/Creeper_World_4)) |
| Creeper World IXE | 12 Dec 2024 ([Steam](https://store.steampowered.com/app/2257010?l=english)) | Creeper on falling-sand terrain ([Steam](https://store.steampowered.com/app/2257010?l=english)) | A squad of multi-weapon ships ([Steam](https://store.steampowered.com/app/2257010?l=english)) | Destroy emitters and cores ([Steam](https://store.steampowered.com/app/2257010?l=english)) |

Steam ratings: CW3 96% of 3,269 ([Steam](https://store.steampowered.com/app/280220)); PF:E 92% of
1,203 ([Steam](https://store.steampowered.com/app/422900?l=english)); CW4 94% of 3,865
([Steam](https://store.steampowered.com/app/848480/Creeper_World_4)); IXE 69% of 863, "Mixed"
([Steam](https://store.steampowered.com/app/2257010?l=english)). PF:E has no critic Metascore; users
give 7.9 from 13 ratings ([Metacritic](https://www.metacritic.com/game/particle-fleet-emergence/)).

The developer is Virgil Wall, founder of Knuckle Cracker
([SGJ podcast notes](https://www.spacegamejunkie.com/podcasts/sgj-podcast-180-particle-fleet-emergence/);
[KC blog p=1044](https://knucklecracker.com/blog/?p=1044)). Their stated aim: "to simplify strategy
gaming and also explore a new type of RTS enemy… based on a fluid simulation"
([SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/)).

## 2. How the simulations work (question 1)

### Creeper as a height field

- Creeper in games 1 and 2 "is modeled using cellular automata"
  ([KC blog p=108](https://knucklecracker.com/blog/?p=108)).
- An underlying structure sets "how thick the Creeper needs to be before it can flow into a cell".
  In game 1 that is terrain levels ([KC blog p=108](https://knucklecracker.com/blog/?p=108)).
- It is neither liquid nor gas, "something sort of in between". It has viscosity, hangs over
  ledges and "snaps back" when its supply is cut ([KC blog p=108](https://knucklecracker.com/blog/?p=108)).
- Community reverse-engineering, not the developer: high values flow "towards lower values by a
  fraction of their absolute difference" ([KC forum](https://knucklecracker.com/forums/index.php?msg=211782)).
- Same thread: terrain height is added to depth for flow, and creeper below a threshold is deleted
  ([KC forum](https://knucklecracker.com/forums/index.php?msg=211782)).
- Emitters "'set' the creeper height to their strength, rather than adding it on top"
  ([KC forum](https://knucklecracker.com/forums/index.php?msg=211782)).
- Anti-creeper "is just negative creeper. Add them up and it vanishes"
  ([KC forum](https://knucklecracker.com/forums/index.php?msg=211782)).
- The code is closed. A moderator: "Creeper World is not open source"
  ([KC forum](https://knucklecracker.com/forums/index.php?msg=169525)).
- CW4 adds velocity and waves: commenters on the dev blog note "it has velocity, waves, etc."
  ([KC blog p=1326](https://knucklecracker.com/blog/?p=1326)).
- CW2 had "friendly creeper": your units pass through it, and flooding your own chambers shields
  structures for a while ([KC blog p=108](https://knucklecracker.com/blog/?p=108)).

### Player structures against the creeper (CW3)

- The command node makes a little energy and sends packets to build units
  ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)).
- Collectors draw energy from the ground; one cell feeds only one collector
  ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)).
- In game 1 an isolated collector goes inactive ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)).
- Weapons need packets of ammunition and cannot fire while moving
  ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)).
- A nullifier must be built near an emitter to kill it
  ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)).
- A TERP raises and lowers terrain ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)).
- Runners stun buildings, and killed runners turn into anti-creeper
  ([LP archive 171](https://lparchive.org/Creeper-World-Series/Update%20171)).
- Spore salvos hit hard ([LP archive 171](https://lparchive.org/Creeper-World-Series/Update%20171)).
- Digitalis is "ground/void spreaders (accelerators) for creeper", with lines showing where it
  will grow ([KC forum](https://knucklecracker.com/forums/index.php?msg=171303)).
- Pause is allowed; orders run on unpause
  ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)).

### Particles in Particle Fleet: Emergence

- Each particle feels "the field value of the space cell it occupies". Particles have mass, max
  speed and drag ([KC blog p=1044](https://knucklecracker.com/blog/?p=1044)).
- Particles are linked by damped springs under Verlet integration, one constraint pass "because
  performance matters a lot" ([KC blog p=1044](https://knucklecracker.com/blog/?p=1044)).
- Shape is health: circle 1, triangle 2, square 3. A hit drops one step
  ([KC blog p=1115](https://knucklecracker.com/blog/?p=1115)).
- Red particles bounce off the map edge; blue die there and have a lifetime
  ([KC blog p=1115](https://knucklecracker.com/blog/?p=1115)).
- Particles hitting land "mire" it. Mire of the other colour destroys land units
  ([KC blog, Emergence](https://knucklecracker.com/blog/index.php/2015/11/emergence/)).
- Fully mired land lets particles pass; metallic land never mires
  ([KC blog, Emergence](https://knucklecracker.com/blog/index.php/2015/11/emergence/)).
- Mired land spawns "Emergent" at a random rate, "like radioactive decay": more mire, more spawns
  ([KC blog, Emergence](https://knucklecracker.com/blog/index.php/2015/11/emergence/)).
- Plasma slows the other side's units; particle beams clear enemy plasma
  ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558);
  [LP archive 202](https://lparchive.org/Creeper-World-Series/Update%20202)).
- Emitters seem capped in count and rate: "you won't get completely overrun"
  ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558)).
  That guide page now says it was removed from the Steam Community.

### The player's verbs

| Verb | Creeper World 3 | Particle Fleet: Emergence |
| --- | --- | --- |
| Expand economy | Place collectors, relays, reactors ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)) | Claim energy sources and mines; mire land ([Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/)) |
| Fight | Place and move turrets ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)) | Move ships; weapons fire on their own ([Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/)) |
| Shape the ground | TERP terrain ([Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Mire land, clear plasma ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558)) |
| Kill the source | Nullifier beside an emitter ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)) | Lathe the emitter ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558)) |
| Design | Maps and scripts ([Steam](https://store.steampowered.com/app/280220)) | Ships, maps, scripts ([KC PF page](https://www.knucklecracker.com/particlefleet/pf.php)) |
| Control time | Pause ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal)) | Pause, slow, fast ([Save or Quit](https://saveorquit.com/2016/11/04/review-particle-fleet-emergence/)) |

## 3. What players say (question 2)

### What makes them fun

- **Watching it spread is soothing, not stressful.** CW4 fighting is "oddly blissful" and
  "unexpectedly soothing", like tidying a room ([PC Gamer, CW4](https://www.pcgamer.com/fighting-massive-waves-of-goo-in-creeper-world-4-is-oddly-blissful)).
- **Predictable but inevitable.** "Oceans aren't very smart, but they are determined." There are no
  "unpleasant surprises" ([Quarter to Three](https://www.quartertothree.com/fp/?p=51574)).
- "The predictability makes it oddly satisfying for a game about warfare"
  ([The Stack, CW3](https://www.wurb.com/stack/?p=5626)).
- **The enemy is the star.** "Creeper World 3 has the best monster of any game" (Sin Vega, RPS,
  quoted on [Wikipedia](https://en.wikipedia.org/wiki/Creeper_World); the RPS page itself was not fetched).
  *(Second-hand, unverified; still unverified after verification pass: searches found no RPS
  page, and the Wikipedia page could not be re-fetched when the shared budget ran out.)*
- "What makes CW III so incredibly fun and challenging is the enemy"
  ([Sound & Vision, CW3](https://soundandvision.com/node/116073)).
- **The network is a machine to watch.** "Packets zipping along their lines, guppies sailing back
  and forth" ([The Stack, CW3](https://www.wurb.com/stack/?p=5626)).
- **Reclaimed ground becomes yours.** Cleared patches become buildable "bulwarks against the tide"
  ([PC Gamer, CW4](https://www.pcgamer.com/fighting-massive-waves-of-goo-in-creeper-world-4-is-oddly-blissful)).
- **Puzzle feel.** "Creeper Worlds are basically puzzle games"
  ([Quarter to Three](https://www.quartertothree.com/fp/?p=51574)). IXE's terrain gives "a more
  puzzle-like feel" ([PC Gamer, IXE](https://www.pcgamer.com/games/rts/creeper-world-ixe-breaks-the-rts-mold-and-adds-a-dash-of-noitas-alchemy-to-the-gooey-war/)).
- **Material interactions.** IXE's fire, water, oil and acid mixing is its top pro
  ([3rd Strike](https://3rd-strike.com/creeper-world-ixe-review)).
- **Readable at a glance.** Big Download liked the "ridiculously simple" iconic graphics
  (quoted on [Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)).

### What falls flat

| Complaint | Where | Source |
| --- | --- | --- |
| Won in the opening, then cleanup | PF:E: "once the player stabilizes in the first minutes of each map, it's nearly impossible to lose" | [Awkward Mixture](https://awkwardmixture.blogspot.com/2020/06/particle-fleet-diminishment.html) |
| Same, from players | PF:E late mission "is mostly just clean up, really" | [Arcen forum](https://forums.arcengames.com/off-topic/particle-fleet-emergence/30) |
| Trivial middle and end | CW4: "a steep start followed by a trivial middle and end" (41 mentions) | [Vaporlens CW4](https://vaporlens.app/app/848480/creeper_world_4) |
| Late grind | CW4: late creep-clearing is "a long, tedious grind" | [Vaporlens CW4](https://vaporlens.app/app/848480/creeper_world_4) |
| Diffuse pressure | PF:E applies "a diffuse pressure on each part of the player's territory", no focused assault | [Awkward Mixture](https://awkwardmixture.blogspot.com/2020/06/particle-fleet-diminishment.html) |
| The swarm reads as dumb | PF:E particulate seems "magnetically attracted to ships"; promised emergent behaviour never showed | [Steam reviews](https://steamcommunity.com/app/422900/reviews/?browsefilter=toprated) |
| Less dread than creeper | "The intensity the Creeper causes just by being there… is missing in PF:E" | [Sound & Vision, PF:E](https://www.soundandvision.com/content/particle-fleet-emergence-review?qt-related_posts=1) |
| Hidden outcomes | Emitter gem drops: "no way to tell when they do or don't ahead of time" | [LP archive 224](https://lparchive.org/Creeper-World-Series/Update%20224) |
| Unseen randomness | A giant cannon's RNG with "lack of visual indication where it's gonna shoot" | [Arcen forum](https://forums.arcengames.com/off-topic/particle-fleet-emergence/30) |
| Sameness | CW3 bonus maps: "a certain sameness to the maps after a while" | [The Stack, CW3](https://www.wurb.com/stack/?p=5626) |
| Too fast a campaign | IXE ideas "don't really feel like they have room to breathe" | [PC Gamer, IXE](https://www.pcgamer.com/games/rts/creeper-world-ixe-breaks-the-rts-mold-and-adds-a-dash-of-noitas-alchemy-to-the-gooey-war/) |
| Readability | CW4's 3D is "less readable" (41); IXE's "color ramps are often lacking in contrast" | [Vaporlens CW4](https://vaporlens.app/app/848480/creeper_world_4); [3rd Strike](https://3rd-strike.com/creeper-world-ixe-review) |
| Interface | No hotkeys for unit groups in IXE; CW4 UI "clunky" (54) | [PC Gamer, IXE](https://www.pcgamer.com/games/rts/creeper-world-ixe-breaks-the-rts-mold-and-adds-a-dash-of-noitas-alchemy-to-the-gooey-war/); [Vaporlens CW4](https://vaporlens.app/app/848480/creeper_world_4) |
| Sim cost | IXE: CPU use and lag from liquids at high speed | [Vaporlens IXE](https://vaporlens.app/app/2257010/creeper_world_ixe) |

### Pacing, comebacks and mission design

- CW2 missions were planned at 5 to 60 minutes, 20 to 30 for experienced players
  ([KC blog p=108](https://knucklecracker.com/blog/?p=108)). PF:E maps run about 30 minutes
  ([Sound & Vision, PF:E](https://www.soundandvision.com/content/particle-fleet-emergence-review?qt-related_posts=1)).
- The loop: "Rush to grab as much land as you can defend. Defend it."
  ([The Stack, CW3](https://www.wurb.com/stack/?p=5626)).
- The one level that "makes you hurry", Farbor, is the one the reviewer remembers. They quit it
  "when all seemed lost", then came back months later and won ([The Stack, CW3](https://www.wurb.com/stack/?p=5626)).
- The developer uses forgiving maps, not a difficulty slider, with safe areas to "regroup and try
  again rather than just fail" ([SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/)).
- They like "missions that have more than one possible path to victory"
  ([SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/)).
- They "err on the side of easier" and "leave the really hard stuff to the community"
  ([SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/)).
- On difficulty, story missions get "Too easy" and "I rage quit" complaints. "Adding a gross control
  over enemy 'output' might be one way to scale mission difficulty" (developer reply,
  [KC blog p=1134](https://knucklecracker.com/blog/?p=1134)).
- PF:E's soft failure: losing the HQ "causes it to warp out for 20 seconds, deactivating all Energy
  Mines" ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558)).

### Editors and custom maps (the toy side)

- PF:E's most-praised feature is its ship editor (82 mentions); community maps follow (47)
  ([Vaporlens PF:E](https://vaporlens.app/app/422900/particle_fleet_emergence)).
- The random map generator is QT3's "favorite way to play" CW4
  ([Quarter to Three](https://www.quartertothree.com/fp/?p=51574)).
- PF:E ships a generator (Simulacrum), map editor, ship editor, scripting and online sharing
  ([KC PF page](https://www.knucklecracker.com/particlefleet/pf.php)).
- A "Mission Heap" lets players browse and rate maps (developer post,
  [KC blog p=1155](https://knucklecracker.com/blog/?p=1155)).
- CW3's story missions were built with the same editor and script players get
  ([Steam](https://store.steampowered.com/app/280220)).
- CW3's tester-made Alpha Sector: "Some levels are jokes. All of them feel personal"
  ([The Stack, CW3](https://www.wurb.com/stack/?p=5626)).
- The developer expects "thousands of user-generated missions" for IXE
  ([SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/)).

## 4. Particle Fleet: Emergence in detail (question 3)

### Ship building and modules

- Every ship has a cockpit (3x3). It builds the hull and modules and stores energy. "If the cockpit
  is destroyed, the entire ship blows up" ([PF wiki](https://particlefleet.fandom.com/wiki/Modules)).
- Hull and weapons build outward from the command module. Large ships can deploy half-built
  ([KC blog p=1044](https://knucklecracker.com/blog/?p=1044)).
- Mass comes from hull and modules; thrust from the drive plus engines; speed from both
  ([KC blog p=1044](https://knucklecracker.com/blog/?p=1044)).
- Rotation was kept so it would "matter in tactical situations"
  ([KC blog p=1044](https://knucklecracker.com/blog/?p=1044)).
- Ships are "partially destructible and self repairing" ([Steam](https://store.steampowered.com/app/422900?l=english)).
  Damage visibly reshapes them: "That's character development"
  ([The Stack, PF:E](https://www.wurb.com/stack/archives/6520)).
- "Any map can be played with any ships" (developer reply, [KC blog p=1199](https://knucklecracker.com/blog/?p=1199)).
- But custom ships do not count toward story progress, which players call punishing (51 mentions)
  ([Vaporlens PF:E](https://vaporlens.app/app/422900/particle_fleet_emergence);
  [Steam reviews](https://steamcommunity.com/app/422900/reviews/?browsefilter=toprated)).
- "Later ships make earlier ones obsolete" (62 mentions, ships "limited and underwhelming")
  ([Vaporlens PF:E](https://vaporlens.app/app/422900/particle_fleet_emergence)).
- The fleet is small, about a dozen ships, capped in the fiction by licences
  ([The Stack, PF:E](https://www.wurb.com/stack/archives/6520)).

### The energy economy

- Energy comes from "onboard tanks, supply drones, other supply ships, energy mines, and onboard
  reactors" (developer reply, [KC blog p=1199](https://knucklecracker.com/blog/?p=1199)).
- A big, energy-heavy ship on thin supply "can definitely 'starve'"
  ([KC blog p=1199](https://knucklecracker.com/blog/?p=1199)).
- Energy sources are fixed on the map. They heal and resupply units in range, which steers how a
  player advances ([The Stack, PF:E](https://www.wurb.com/stack/archives/6520)).
- Energy mines yield more the longer you hold them. The bonus resets if destroyed and caps at 3x
  or 5x (developer replies, [KC blog p=1101](https://knucklecracker.com/blog/?p=1101)).
- Port modules share energy between ships "until their tanks equalize"
  ([KC blog p=1101](https://knucklecracker.com/blog/?p=1101)).
- Tankers carry shuttles that fetch 50 energy from a field and bring it back
  ([Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/)).
- Players priced weapons per particle killed: cannon about 0.5, laser 0.53, missile 1, omni 0.35
  ([Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/)).
- Advice: "Build only as many ships as you can support with your energy output"
  ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558)).
- A pre-release commenter found early play energy-starved and late play energy-irrelevant, "too
  linear" ([KC blog p=1143](https://knucklecracker.com/blog/?p=1143)).

### How particle fights read on screen

- Health is shape; side is colour (red bad, blue yours) ([KC blog p=1115](https://knucklecracker.com/blog/?p=1115);
  [LP archive 202](https://lparchive.org/Creeper-World-Series/Update%20202)).
- In pre-alpha, particle colour showed speed ([KC blog p=1044](https://knucklecracker.com/blog/?p=1044)).
- The developer prefers packing "more 'information' on the screen" over box-art spectacle
  ([KC blog p=1155](https://knucklecracker.com/blog/?p=1155)).
- Reviewers call it "red balls of space dandruff" that "flow and bounce"
  ([Sound & Vision, PF:E](https://www.soundandvision.com/content/particle-fleet-emergence-review?qt-related_posts=1)).
- Players read the battle as counts: "Mire is 56-41 against us, particulate 1200+ to less than 300"
  ([LP archive 208](https://lparchive.org/Creeper-World-Series/Update%20208)).
- Damage shows as sparks and missing hull ([LP archive 202](https://lparchive.org/Creeper-World-Series/Update%20202)).
- Individual particles are harmless; "it takes a significant amount of them"
  ([LP archive 202](https://lparchive.org/Creeper-World-Series/Update%20202)).

### Many particles against player agency

| Lever | What PF:E does | Source |
| --- | --- | --- |
| Few hands, many bodies | About a dozen ships against hundreds to thousands of particles | [The Stack](https://www.wurb.com/stack/archives/6520); [LP 208](https://lparchive.org/Creeper-World-Series/Update%20208) |
| Simple per-particle rules | Field value of the cell, mass, drag, one spring pass | [KC p=1044](https://knucklecracker.com/blog/?p=1044) |
| Caps | Spawn rate and count capped, so no total overrun | [Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558) |
| Price per effect | Every shot costs energy | [Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/) |
| Area verbs | Fields push particles; a module that is "incredibly awesome at kiting giant blobs" | [KC p=1115](https://knucklecracker.com/blog/?p=1115); [Arcen forum](https://forums.arcengames.com/off-topic/particle-fleet-emergence/30) |
| Difficulty knob | "Gross control over enemy 'output'" | [KC p=1134](https://knucklecracker.com/blog/?p=1134) |

**What went wrong with the balance.** The swarm has no goal of its own beyond drifting toward your
ships, "weakly attracted to your own forces" ([The Stack, PF:E](https://www.wurb.com/stack/archives/6520)).
So pressure is diffuse, and a stable perimeter wins
([Awkward Mixture](https://awkwardmixture.blogspot.com/2020/06/particle-fleet-diminishment.html)).
Our rival mould avoids this: it wants the same food the player needs.

### The developer's stated design aims

- "I like making a game with a strategic element that hasn't been explored (very much) in the
  strategy genre" ([KC blog p=1115](https://knucklecracker.com/blog/?p=1115)).
- The official tagline: "A fluid particle simulation that outsmarts you"
  ([KC PF page](https://www.knucklecracker.com/particlefleet/pf.php)).
- Mire exists for "offense and defense, not about energy"
  ([KC blog, Emergence](https://knucklecracker.com/blog/index.php/2015/11/emergence/)).
- Multiplayer was ruled out for PF:E, though both sides were kept balanced
  ([KC blog p=1101](https://knucklecracker.com/blog/?p=1101)).
- Custom scripting was chosen over Lua or Python because it ties into the game loop and
  save state ([KC blog p=1155](https://knucklecracker.com/blog/?p=1155)).

## 5. What transfers to Bloomfront (question 4)

Bloomfront's premise already differs from both series in one key way. Knuckle Cracker gave the
player a fluid only as a sideline (anti-creeper, friendly creeper). Bloomfront makes the player's
own living fluid the whole army, against a rival with hands. Keep that the centre.

**Already confirmed by these games.** Spend-from-store is PF:E's energy rule without ships.
Announced blooms match CW's "predictable but inevitable" enemy. Indirect control matches the
developer's aim to "simplify strategy gaming". Pause and speed already exist (`P` in
`src/bin/mould.rs`).

### Ranked ideas

| # | Idea | From | How it works in our sim | Owner ruling it touches | Cost and code |
| --- | --- | --- | --- | --- | --- |
| 1 | **Wells that ripen while held** | PF:E energy mines: more output the longer held, reset on loss, capped ([p=1101](https://knucklecracker.com/blog/?p=1101)) | A well's `r` grows while your eaters outnumber the foe's, up to about 1.5x. `table` scales with its area. It shrinks back when the foe out-crowds you | Picture is the data: the pile disc is already drawn from `p.r` and its glow from `food/full` (`look.rs` ~1245). A spread: share swings by degrees | **Small.** `Pile` gains a held timer; `census`, `Pile::bite`, `level.rs`. `pile_key` uses x,y only, so growth is safe. Per-pile cost, negligible |
| 2 | **End the mop-up: a mercy bell** | CW4 "trivial middle and end"; PF:E "nearly impossible to lose" after the opening ([Vaporlens CW4](https://vaporlens.app/app/848480/creeper_world_4); [Awkward Mixture](https://awkwardmixture.blogspot.com/2020/06/particle-fleet-diminishment.html)) | The match ends when the Feast lead exceeds all food still on the plate plus well refill to the bell. Nobody plays out a decided game | A spread: the final share is still the score. HUD only, nothing on the plate | **Small.** The win check in `level.rs` beside `food_share`; reads `Dish::ate`, `wild_ate`, pile food. No per-step cost |
| 3 | **Blooms announce themselves by scent** | CW: "no unpleasant surprises"; emitters "set" a level ([QT3](https://www.quartertothree.com/fp/?p=51574); [KC forum](https://knucklecracker.com/forums/index.php?msg=211782)) | A bloom gives off scent for N seconds before its food lands. Walkers smell it too, so the preview is real. This meets the late-piles dead end's re-test condition: a preview to prepare for | Everything on screen is the sim: scent is a real field, but `look.rs` does not draw it yet. Draw it through one fixed ramp, or keep the countdown in the HUD | **Small** for the sim (`deposit_food`; a pile flag so `census` and `bite` skip it until it lands). **Small-medium** to draw scent in `look.rs` |
| 4 | **A strain designer with a trait budget** | PF:E's ship editor is its most-praised feature ([Vaporlens PF:E](https://vaporlens.app/app/422900/particle_fleet_emergence)) | Players build a species from `Params` (sensor angle, divide, crowd, fight bonus, crust life) under a point budget. Every gain costs elsewhere, so no strain is an upgrade | Fun over fidelity. Every tool visible: each trait must change the look (branchy versus thick). Avoid PF:E's mistake: custom strains count everywhere | **Medium.** The T panel (`tune.rs`) already edits `Params` live; add a budget and a preset file. Balance needs AI-against-AI census per strain pair (stage 4 work) |
| 5 | **Supply you can see travel** | CW packets "zipping along their lines"; PF:E ports equalise tanks ([The Stack CW3](https://www.wurb.com/stack/?p=5626); [p=1101](https://knucklecracker.com/blog/?p=1101)) | Turn on `Params::flow` with `sap`: food moves in the tube and the drawn colour is carried food. A cut branch dims within about a second (`CARRIED_KEEP`). This helps risk 4, "cuts do not bite" | Picture is the data: carried food is drawn, no painted packets. Food flow is the owner's open decision 4 in `HANDOFF.md`; ask first | **Medium.** Flow exists but is off. The rival must run it too; The Wild One needs retuning. Sap passes run every step: frame cost unmeasured, measure before adopting |
| 6 | **Lose tempo, not the game: crust holds** | PF:E HQ warps out for 20 s instead of dying; CW maps with safe areas to regroup ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558); [SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/)) | A sleeping walker takes fight losses at the fed rate (0.5) and does not attack. A keeper can bake a losing front to sleep with the lamp, survive the push, and water it awake later | Every tool visible: sleep is already drawn (`crusted`, `look.rs` ~2412). Gives lamp and water a combat job | **Small.** One branch in `fight_losses`; water must be on in Bloomfront (it is 0 on The Wild One). Negligible cost |
| 7 | **Wet maps as terrain** | CW: creeper fills low ground first; high ground and choke points; TERP ([Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal); [Wikipedia](https://en.wikipedia.org/wiki/Creeper_World)) | Each plate gets a per-cell resting wetness: damp channels, dry plateaus where mould sleeps. Lamp dries and water wets, so they become terraforming. Many paths to each well | Picture is the data: wet ground is already drawn (`wet_ground`). Draw absolute wet through the same ramp, never stepped bands | **Small-medium.** `wet_base` becomes a field; `wet_tick`, `wet_field` in `look.rs`, and a wet layout in `Level`. No extra pass per step |
| 8 | **Fair-plate check and plate codes** | CW/PF:E editors, generators, rated map heaps ([KC PF page](https://www.knucklecracker.com/particlefleet/pf.php); [p=1155](https://knucklecracker.com/blog/?p=1155)) | A plate is a short text: wells, salt, wet. Before sharing, `mouldprobe` runs warden against warden on paired seeds and prints the share spread. Only plates near 50/50 publish | A spread, not a yes/no: the check reports a spread. Our census is an edge no Knuckle Cracker tool has | **Small** for a seeded daily plate. **Medium** for a text format (`Level` is static today) and an editor in `src/bin/mould.rs`. Needs stage 0 (a hand on the rival) |

### Further ideas, ranked lower

| Idea | From | How it works here | Why lower | Cost |
| --- | --- | --- | --- | --- |
| Fallen walkers feed the winner | CW3 runners turn into anti-creeper when killed ([LP 171](https://lparchive.org/Creeper-World-Series/Update%20171)) | A walker killed in a fight leaves its load in the tube cell (`sap`), which either side can eat | Rewards winning fights, but may snowball. Needs flow on | Small-medium; `remove_walker(Kill::Fight)` |
| Held ground remembers; lamp clears it | PF:E plasma slows foes; beams clear it ([Steam guide](https://steamcommunity.com/sharedfiles/filedetails/?id=790297558)) | Your `wall` field (flow level 3) also repels foe walkers; lamp light fades the foe's wall | Territory inertia is exactly what made PF:E "won in the opening". Test only with the mercy bell | Medium; `wall`, `foe_repel`, `push_light` |
| Seam annihilation | Anti-creeper is negative creeper ([KC forum](https://knucklecracker.com/forums/index.php?msg=211782)) | Where both trails overlap, each loses the smaller; contested ground visibly empties | Overlaps `foe_repel` and fights; unclear gain | Small; one extra W x H pass a step, unmeasured |
| Pressure scales with territory | PF:E mire spawns Emergent in proportion to area ([KC blog](https://knucklecracker.com/blog/index.php/2015/11/emergence/)) | Spores fruit at random from held tube, more tube, more spores | Already Bloomfront's Fruit verb in another form | Medium; the R6 spore design |

### Design lessons that are not mechanics

- **Give the swarm a want.** PF:E's swarm drifts toward ships and reads as dumb. Our rival seeks
  food the player needs. Keep every rival rule tied to food.
- **Teach one interaction per dish.** IXE was faulted for introducing ideas too fast. Our lesson
  already does one tool at a time; keep that for each Bloomfront rule.
- **Readable ramps beat pretty ones.** IXE's low-contrast ramps and CW4's 3D both hurt reading.
  Keep one fixed, high-contrast ramp per held quantity.
- **Counts beside the picture.** PF:E players read battles by tallies. Show territory and share
  counts in the HUD, off the plate.
- **Watch the frame.** IXE's liquids lag at high speed. Every new per-step field needs its cost
  quoted against our 16.7 ms budget.

## 6. What not to take (question 5)

### Signature elements to stay away from

| Kind | Avoid | Source |
| --- | --- | --- |
| Terms | Creeper, anti-creeper (AC), emitter, nullifier, command node, Odin City, totems, collector/relay/reactor, packets, TERP, digitalis, runners, inhibitor, shield key | [Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal); [Wikipedia](https://en.wikipedia.org/wiki/Creeper_World); [LP 171](https://lparchive.org/Creeper-World-Series/Update%20171); [KC forum](https://knucklecracker.com/forums/index.php?msg=171303) |
| PF:E terms | Particulate, Emergent, Doppel, mire, plasma, lathe, omni, energy mine, HQ warp, Simulacrum, Mission Heap, Redacted Space, Ticon, PRPL/CRPL/RPL | [Steam](https://store.steampowered.com/app/422900?l=english); [p=1115](https://knucklecracker.com/blog/?p=1115); [Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/) |
| Roster | Turret tiers (short, long, lobbing), a fluid sprayer, a source-killer placed beside the source, a terraformer unit, flying supply ferries, modular warships | [Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal); [Steam thread](https://steamcommunity.com/app/422900/discussions/0/348293292498151557/) |
| Look | Blue goo on fractal relief terrain; red versus blue particles in space; pixel sci-fi | [SUPERJUMP](https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/); [LP 202](https://lparchive.org/Creeper-World-Series/Update%20202) |
| Frame | Galactic war, a corporation's CEO, "the eternal harvester of galactic empires" | [Steam CW4](https://store.steampowered.com/app/848480/Creeper_World_4); [Steam PF:E](https://store.steampowered.com/app/422900?l=english) |
| Missions | Kill every source to win; collect artifacts or schematics to unlock the next star system | [Gigazine](https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal); [LP 202](https://lparchive.org/Creeper-World-Series/Update%20202) |
| Core loop | Tower defence: build a network outward, place guns, push a mindless flood back | [Vaporlens CW4](https://vaporlens.app/app/848480/creeper_world_4) |

**Careful with "spores".** CW3 has enemy spore salvos that strike buildings
([LP 171](https://lparchive.org/Creeper-World-Series/Update%20171)). Bloomfront's Fruit verb is real
slime-mould biology and fine. Do not make spores homing shots at the rival.

Bloomfront's own words (keeper, plate, well, bloom, lure, push, wall, sever, wake, fruit, temper)
and its agar-plate look in gold and violet are already clear of these.

### Their ideas that clash with our owner's rulings

| Their idea | Ruling it breaks | Use instead |
| --- | --- | --- |
| Packets as sprites moving along drawn link lines | No drawn-only effects; no painted pulse | Carried food in the tube (`flow` + `sap`), drawn from the data |
| Range rings, health bars, placement ghosts on the map | Everything on screen is the simulation | HUD only, off the plate |
| Terrain or depth shown as stepped bands (CW1 used discrete terrain levels, [p=108](https://knucklecracker.com/blog/?p=108); the third game's design post: "There are currently 10 terrain levels (0-9)", [p=449](https://knucklecracker.com/blog/?p=449), verification pass) | No thresholded edges | One continuous ramp per held quantity |
| Hidden drops and unseen RNG (gem drops, unmarked cannon fire) | Every tool must deliver something you can see | Every effect shows on the plate as it happens |
| Pass or fail missions ("destroy all emitters") | An outcome is a spread | Share of food eaten, time held |
| Fog of war (CW2, [p=108](https://knucklecracker.com/blog/?p=108)) | Open owner question | Only as a held quantity, such as sensor reach |
| Selecting and moving units, formations (PF:E) | Not a ruling, but breaks Bloomfront's indirect-control premise | Lure, push, wall, sever, wake |
| Pause with queued orders | Fine against the AI; impossible in lockstep online play | Keep pause for single player only |

## 7. Pages that could not be fetched or held nothing

- The session's shared web-fetch budget ran out once; two pages failed then:
  `https://lparchive.org/Creeper-World-Series/Update 135` and `https://knucklecracker.com/blog/?p=449`.
  Nothing from them is used. Verification pass: p=449 was fetched. It is a design preview of the
  third game: command nodes dropped from orbit, path-line networks back from CW1 with planned line
  capacity limits, storage pods, Terrap terraforming over "10 terrain levels (0-9)", and AC from
  well heads. It changes nothing here; it backs the stepped-terrain row in the clash table. Update
  135 hit the budget limit twice and is still unfetched.
- `https://store.steampowered.com/app/559970/...` was a wrong app id from memory; it is another game.
  Verification pass: Creeper World: Anniversary Edition is app 422910 (found by search, not
  fetched: https://store.steampowered.com/app/422910/Creeper_World_Anniversary_Edition/). Nothing
  in this note rests on either id.
- The RPS "best monster" article and Kieron Gillen's 2009 piece were not found; their quotes come
  from Wikipedia only. Verification pass: still not found by search, and the Wikipedia page could not
  be re-fetched (budget), so the RPS quote stays second-hand and unverified.
- `https://www.moddb.com/news/using-a-cellular-automaton-in-video-games` is about Nanotale; it only
  names Creeper World in passing.
- `https://knucklecracker.com/blog/index.php/2016/12/monster/` holds only reader comments.
- The Steam CW3 page and the original CW page describe no mechanics.

## Sources

Developer and official
- Knuckle Cracker, Particle Fleet page: https://www.knucklecracker.com/particlefleet/pf.php
- Knuckle Cracker, Creeper World page: https://knucklecracker.com/creeperworld/cw.php
- Knuckle Cracker, Creeper World 4 page: https://knucklecracker.com/creeperworld4/cw4.php
- KC blog, CW2 walkthrough and Q&A: https://knucklecracker.com/blog/?p=108
- KC blog, PF Q&A, late Nov 2015: https://knucklecracker.com/blog/?p=1115
- KC blog, Emergence, Nov 2015: https://knucklecracker.com/blog/index.php/2015/11/emergence/
- KC blog, PF video thread, 2015: https://knucklecracker.com/blog/?p=1101
- KC blog, Game 4 pre-alpha video 2: https://knucklecracker.com/blog/?p=1044
- KC blog, PF video thread, early 2016: https://knucklecracker.com/blog/?p=1134
- KC blog, PF video thread, Mar-Apr 2016: https://knucklecracker.com/blog/?p=1143
- KC blog, PF status update: https://knucklecracker.com/blog/?p=1155
- KC blog, PF Steam page announcement: https://knucklecracker.com/blog/?p=1199
- KC blog, CW4 thread, 2017: https://knucklecracker.com/blog/?p=1326
- KC blog, rocks and packets screenshot: https://knucklecracker.com/blog/?p=1357
- KC blog, "monster" comments, Dec 2016: https://knucklecracker.com/blog/index.php/2016/12/monster/
- KC forum, creeper spread formulae, Nov 2023: https://knucklecracker.com/forums/index.php?msg=211782
- KC forum, code for creeper flow, Jan 2018: https://knucklecracker.com/forums/index.php?msg=169525
- KC forum, digitalis in CW4, Apr 2018: https://knucklecracker.com/forums/index.php?msg=171303

Stores and aggregates
- Steam, Particle Fleet: Emergence: https://store.steampowered.com/app/422900?l=english
- Steam, Creeper World 3: https://store.steampowered.com/app/280220
- Steam, Creeper World 4: https://store.steampowered.com/app/848480/Creeper_World_4
- Steam, Creeper World IXE: https://store.steampowered.com/app/2257010?l=english
- Steam top-rated PF:E reviews: https://steamcommunity.com/app/422900/reviews/?browsefilter=toprated
- Steam PF:E unit roles thread: https://steamcommunity.com/app/422900/discussions/0/348293292498151557/
- Steam PF:E campaign guide (Kuryaka, 30 Oct 2016): https://steamcommunity.com/sharedfiles/filedetails/?id=790297558
- Steam PF:E preliminary review (75Karsten, 23 Sep 2016): https://steamcommunity.com/app/422900/discussions/0/350542683194729164/
- Vaporlens, PF:E: https://vaporlens.app/app/422900/particle_fleet_emergence
- Vaporlens, CW4: https://vaporlens.app/app/848480/creeper_world_4
- Vaporlens, IXE: https://vaporlens.app/app/2257010/creeper_world_ixe
- Metacritic, PF:E: https://www.metacritic.com/game/particle-fleet-emergence/
- Wikipedia, Creeper World: https://en.wikipedia.org/wiki/Creeper_World
- Knuckle Cracker fandom wiki, structures: https://knucklecracker.fandom.com/wiki/Structures
- Particle Fleet fandom wiki, modules: https://particlefleet.fandom.com/wiki/Modules

Reviews, interviews and play records
- SUPERJUMP interview (Josh Bycer, 16 Apr 2025): https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/
- Game-Wisdom, same interview: https://game-wisdom.com/?p=31329
- Space Game Junkie podcast 180 notes (17 Nov 2016): https://www.spacegamejunkie.com/podcasts/sgj-podcast-180-particle-fleet-emergence/
- PC Gamer, CW4 (Fraser Brown, 8 Jan 2021): https://www.pcgamer.com/fighting-massive-waves-of-goo-in-creeper-world-4-is-oddly-blissful
- PC Gamer, IXE (Dominic Tarason, 18 Dec 2024): https://www.pcgamer.com/games/rts/creeper-world-ixe-breaks-the-rts-mold-and-adds-a-dash-of-noitas-alchemy-to-the-gooey-war/
- Quarter to Three, CW4 (Tom Chick, 22 Dec 2020): https://www.quartertothree.com/fp/?p=51574
- The Stack, CW3 (16 Dec 2018): https://www.wurb.com/stack/?p=5626
- The Stack, PF:E (6 Jan 2021): https://www.wurb.com/stack/archives/6520
- The Stack, IXE (15 Jan 2025): https://www.wurb.com/stack/?p=7750
- 3rd Strike, IXE (8 Feb 2025): https://3rd-strike.com/creeper-world-ixe-review
- Sound & Vision, PF:E: https://www.soundandvision.com/content/particle-fleet-emergence-review?qt-related_posts=1
- Sound & Vision, CW3: https://soundandvision.com/node/116073
- Awkward Mixture, PF:E (5 Jun 2020): https://awkwardmixture.blogspot.com/2020/06/particle-fleet-diminishment.html
- GamingOnLinux, PF:E (Liam Squires-Hand, 30 Sep 2016): https://www.gamingonlinux.com/2016/09/i-think-particle-fleet-emergence-just-became-my-new-linux-gaming-addiction/page=1/
- Save or Quit, PF:E (JimDeadlock, 4 Nov 2016, 83/100): https://saveorquit.com/2016/11/04/review-particle-fleet-emergence/
- Arcen Games forum, PF:E thread (Oct 2016): https://forums.arcengames.com/off-topic/particle-fleet-emergence/30
- Gigazine, CW3 (19 Jul 2015): https://gigazine.net/gsc_news/en/20150719-creeper-world-3-arc-eternal
- Let's Play Archive index: https://lparchive.org/Creeper-World-Series/
- Let's Play Archive 171: https://lparchive.org/Creeper-World-Series/Update%20171
- Let's Play Archive 202: https://lparchive.org/Creeper-World-Series/Update%20202
- Let's Play Archive 208: https://lparchive.org/Creeper-World-Series/Update%20208
- Let's Play Archive 224: https://lparchive.org/Creeper-World-Series/Update%20224

## Verification pass (2026-11-10)

A later pass fetched what this note could not. The shared fetch budget ran
out several times during it, so a page that failed twice stays unverified.

| Item | Outcome | Source |
| --- | --- | --- |
| RPS (Sin Vega): "Creeper World 3 has the best monster of any game", taken from Wikipedia | Still unverified | No RPS page found by search; https://en.wikipedia.org/wiki/Creeper_World failed twice (budget) |
| Kieron Gillen's 2009 piece (no quote of it is used) | Still unverified | Not found by search |
| `lparchive.org/Creeper-World-Series/Update 135` (nothing used) | Still unverified | Failed twice (budget) |
| `knucklecracker.com/blog/?p=449` (nothing used) | Confirmed, as a third-game design preview; adds "10 terrain levels (0-9)" to the stepped-terrain clash row | https://knucklecracker.com/blog/?p=449 |
| Steam app 559970 was the wrong id (nothing used) | Corrected: Creeper World: Anniversary Edition is app 422910 (found by search, not fetched) | https://store.steampowered.com/app/422910/Creeper_World_Anniversary_Edition/ |

No ranked idea changes. The RPS quote supports only the "enemy is the star"
bullet in section 3, which the Sound & Vision quote beside it also carries.
