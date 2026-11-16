# How other games handle elites, mini-bosses and bosses

Outside research, 2026-11-16. Every claim taken from a source carries its URL;
lines marked "(reading)" are my inference. Gaps: no source loaded for
Mindustry's guardians; Sanctum, Katamari, Shadow of the Colossus and No Man's
Sky not covered; Risk of Rain 2's per-elite effects were not in the fetched
wiki text.

## 1. Spawn rules

**Left 4 Dead.** The AI Director estimates each survivor's "emotional
intensity" (raised by damage, incapacitation, nearby infected dying) and
cycles Build Up, Sustain Peak (3-5 s), Peak Fade and Relax (30-45 s of minimal
population): "Algorithm adjusts pacing, not difficulty." Specials spawn on
randomised timers, out of sight, in areas suited to each class (Boomers ahead,
Smokers above), never during Relax. Bosses follow a different rule: at map
start, events are placed every N units along the route with a random offset,
dealt from a shuffled set of Tank, Witch and Nothing, never the same twice in
a row, and exempt from adaptive pacing because missing them would break the
overall pacing.
(https://cdn.fastly.steamstatic.com/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf)

**Risk of Rain 2.** A difficulty coefficient rises with time and multiplies by
1.15 per stage. Directors earn credits in proportion, pick a random enemy and
buy a group of up to four; Blazing, Overloading and Glacial elites cost 6x the
base card, Malachite and Celestine 36x. Groups too cheap for the wallet are
skipped, so weak enemies fade out over a run
(https://riskofrain2.wiki.gg/wiki/Difficulty). The boss is a player-triggered
event: activating the teleporter spawns one that must die before the charge
completes, and after about 15 minutes the boss director can afford several or
elite bosses (https://rogueranker.com/risk-of-rain-2-bosses/).

**Space Marine 2.** Mini-bosses are 0-2 events per level, allowed only after a
long cooldown, at low team stress, while the team moves forward, in an area
that suits that mini-boss
(https://spacemarine2-modding.prismray.io/guides/modder_reference/enemy_spawn_director/mini_bosses/).

**Deep Rock Galactic.** Elite variants spawn only on missions with the "Elite
Threat" warning; they are larger, with a red aura of vertical lines and an
"Elite" title. Glyphids roam in small groups and periodically mass into swarms
that Mission Control announces (https://deeprockgalactic.wiki.gg/wiki/Enemies).
The golden Crassus Detonator is rolled at map generation, 3% per map, one per
mission (https://deeprockgalactic.wiki.gg/wiki/Crassus). Mission warnings
reshape the whole mix for a hazard bonus, e.g. Exploder Infestation +30%,
Regenerative Bugs +15%
(https://www.supercheats.com/deep-rock-galactic/walkthrough/mutators-warnings-guide).

**Hades.** Elites carry a skull, armour (a yellow bar blocking stun and
knockback until broken) and extra abilities; some appear only in miniboss
chambers (https://hades.fandom.com/wiki/Unblockable). Heat scales them:
Benefits Package gives armoured foes up to two perks, Middle Management adds
elites to miniboss fights, Extreme Measures gives bosses new techniques
(https://rpgsite.net/feature/10287-hades-pact-of-punishment-heat-modifiers-and-how-to-maximize-your-rewards).

**Diablo II.** A special pack is 20% champion pack (2-4 monsters), 80% unique
pack (a leader with 3-6 minions). Unique bonuses are drawn evenly from a
permitted list with exclusions, such as no third immunity
(https://classic.battle.net/diablo2exp/monsters/bonus.shtml).

**Path of Exile.** Magic monsters carry one affix in their name; rares carry
three and a random name; whole packs can roll a "bloodline" mod
(https://pathofexile.fandom.com/wiki/Monster_modifiers).

**Binding of Isaac.** Champions are about 15% larger and tinted, and each
colour is one fixed behaviour: red has more HP, green leaves creep, pulsing
green splits in two on death, dark red collapses into a flesh pile and
regenerates unless finished. Some colours unlock only on later floors
(https://bindingofisaacrebirth.wiki.gg/wiki/Champions).

**Brotato.** On Danger 4-5, challenging waves fall at 11/12, 14/15 and 17/18;
the first two are 60% elite, 40% horde, the third always elite; the shop shows
which is coming; no elite repeats in a run
(https://brotato.wiki.spellsandguns.com/Elite_Wave). Wave 20 (90 s) is the boss
(https://brotato.wiki.spellsandguns.com/Waves).

**Vampire Survivors.** Most evolution chests come from bosses at or after 10
minutes (https://vampire-survivors.fandom.com/wiki/Evolution); at 30 minutes
the Reaper appears and the waves vanish
(https://holdtoreset.com/vampire-survivors-boss-list).

**Dead Cells.** Elites are placed: a dormant "journeyman" on the path, or an
off-route chamber started at an altar; a level version may have none. Below
50% HP the elite teleports onto the player, summoning aides each time
(https://www.gamepressure.com/dead-cells/elites/z7a1b3).

(reading) Three families: budget directors (RoR2, L4D specials, Space Marine 2)
scale smoothly; dealt decks and schedules (L4D bosses, Brotato, Vampire
Survivors) guarantee pacing and variety; rolls at generation (DRG, Diablo,
Dead Cells) make maps differ. L4D runs a budget and a deck side by side.

## 2. Telegraphing and arrival

- **Left 4 Dead.** Each special has its own music cue; the Tank comes with
  grunts, roars and music that "suddenly starts building up"
  (https://spuf.org/?p=21937). The Witch is heard before seen (crying, a sting
  that escalates in stages), is passive unless approached, lit or startled,
  and once attacked the whole group, which wiped parties too often and was
  changed (https://en.wikipedia.org/wiki/Witch_(Left_4_Dead)).
- **Deep Rock Galactic.** Dreadnought cocoons sit on walls and ceilings,
  ringed by blue veins, with a heartbeat as you approach; any damage breaks
  the membrane and starts the fight
  (https://www.gamepressure.com/deep-rock-galactic/elimination-main-mission-objective/z0d4bf).
  The Haunted ghost is invulnerable; its aura and sounds grow as it nears
  (https://deeprockgalactic.wiki.gg/wiki/Haunted).
- **StarCraft.** A Mutalisk morphing to Guardian or Devourer is a static Cocoon
  (200 HP, no armour) for 40 s, and opponents usually attack it before it
  hatches (https://liquipedia.net/starcraft/Cocoon).
- **Helldivers 2.** A Terminid that spots a player takes a stance, calls, and
  releases visible orange mist to summon a breach; a cooldown, shorter at high
  difficulty, limits breaches (https://helldivers.wiki.gg/wiki/Bug_Breach).
- **Infested Planet.** Tower spawn, Bulwark and clone attacks give about 10 s
  of warning (https://steamah.com/infested-planet-all-mutations-guide/).
- **They Are Billions.** Scripted swarms are announced with time and
  direction, a music change and a minimap skull
  (https://www.gamepressure.com/they-are-billions/zombies/zea7d2); each Giant
  has a minimap icon from the start
  (https://www.moddb.com/news/they-are-billions-update-something-giant-is-coming).
- **Dwarf Fortress** pauses and prints a description when a forgotten beast
  first appears (https://dwarffortresswiki.org/index.php/Forgotten_beast).
- **Terraria.** In a Slime Rain, King Slime comes after 150 slime kills, and
  the Guide warns "you might invoke the wrath of their king"
  (https://terraria.wiki.gg/wiki/Slime_Rain).
- **Dragon Quest.** About eight slimes pile up and fuse into a King Slime; in
  DQ IX only if none of the eight is killed first
  (https://dragon-quest.org/wiki/KingSlime).
- **Slay the Spire** shows the act boss on the map from the start
  (https://slay-the-spire.fandom.com/wiki/Bosses).
- **What goes wrong.** A Hades II critique lists telegraphs hidden by dust,
  dim warning colours, shapes that don't match the hit, and inconsistent rules
  (https://nex-3.com/blog/hades-ii-has-a-legibility-problem/).

(reading) Where the enemy visibly forms (StarCraft, DRG, Dragon Quest, Isaac's
regenerating pile, Infested Planet), the forming window is the counterplay:
hit it early or break up the gathering.

## 3. Modifier and affix systems

- **Diablo III** affixes are drawn in the world: Molten leaves a fire trail,
  Desecrator a pool that grows while you stand in it, Plagued a poison pool
  spreading in an X after a delay, Waller raises walls, Mortar marks the
  ground, Fire Chains link pack members. Count rises with level from 1 to 4;
  composition is capped at 2-3 offensive, at most one defensive and one crowd
  control (https://maxroll.gg/d3/resources/elite-affixes).
- **Path of Exile's correction.** GGG said Archnemesis mods bundled several
  effects under thematic names, which "made too many encounters too complex".
  The fix: "Mods do one specific thing", names that say what they do, complex
  mods diluted by simple ones, rewards moved to hidden mods likelier on rares
  with more mods (https://www.pathofexile.com/forum/view-thread/3322245).
- **Riftbreaker** Omegas draw up to four of 11 abilities (auras, spawners,
  waller, shockwave, meteors) by difficulty, no duplicates. Healing plus
  resistance aura proved nearly unkillable, and waller with drones too
  punishing, so combinations are excluded
  (https://www.exorstudios.com/blog/omega-ability-gallery).
- **Hades Heat** turns difficulty into named dials the player picks, each with
  ranks and a Heat price
  (https://rpgsite.net/feature/10287-hades-pact-of-punishment-heat-modifiers-and-how-to-maximize-your-rewards).
- **Slay the Spire** splits roles: elites are optional map nodes paying a
  relic, gold and a card; one "burning" elite per act gets a buff and drops
  the Emerald Key (https://slaythespire.wiki.gg/wiki/Elites). Bosses are
  fixed, visible in advance, and pay boss relics, rare cards and a heal
  (https://slay-the-spire.fandom.com/wiki/Bosses).
- **Monster Train** bosses' powers depend on the floor they hold; optional
  Trials strengthen enemies for an extra reward
  (https://www.tvtropes.org/pmwiki/pmwiki.php/VideoGame/MonsterTrain).

(reading) What keeps combinations readable: one modifier, one effect; the
effect drawn in the world; caps per category; a short banned-pair list; count
tied to difficulty; a fixed look-to-behaviour mapping (Isaac's colours).

## 4. Reward and risk

- **Slay the Spire** elites pay a relic, 25-35 gold and a card; Ascension 1
  makes them about 60% more common (https://slaythespire.wiki.gg/wiki/Elites).
- **Risk of Rain 2.** A Blazing elite has a 1-in-4000 chance to drop Ifrit's
  Distinction, which makes the holder a Blazing elite
  (https://riskofrain2.wiki.gg/wiki/Ifrit%27s_Distinction). Shrine of the
  Mountain is an opt-in dial: harder or extra bosses for an extra copy of each
  drop (https://riskofrain2.wiki.gg/wiki/Shrine_of_the_Mountain).
- **Brotato** elites drop a Legendary crate and heal 100 HP
  (https://brotato.wiki.spellsandguns.com/Elite_Wave); **Isaac** champions drop
  a pickup matching their colour
  (https://bindingofisaacrebirth.wiki.gg/wiki/Champions); **Vampire
  Survivors** routes evolutions through boss chests
  (https://vampire-survivors.fandom.com/wiki/Evolution).
- **DRG's Crassus** explodes on death and coats a 13 m crater in gold; yield
  depends on rock at the blast's edge, so a kill in a narrow tunnel pays more,
  a frozen kill nothing (https://deeprockgalactic.wiki.gg/wiki/Crassus).

Costs of ignoring one: They Are Billions Giants join the final swarm if not
killed first (https://www.moddb.com/news/they-are-billions-update-something-giant-is-coming);
Dead Cells journeyman elites often block the way
(https://www.gamepressure.com/dead-cells/elites/z7a1b3); avoiding the Witch
costs attention, so players miss incoming hordes
(https://en.wikipedia.org/wiki/Witch_(Left_4_Dead)).

## 5. Masses, fluids and swarms

- **Creeper World.** The Creeper acts like a liquid, fills low ground first,
  and in the first game cannot be destroyed. CW2 adds Phantoms, spores that
  fly to a random building and become Creeper where they land, and makes
  destroying generators the goal; CW3 keeps emitters as targets
  (https://en.wikipedia.org/wiki/Creeper_World). In CW3 runner nests release
  runners along Digitalis that stun buildings, dead runners become
  anti-creeper, and spore towers fire salvos
  (https://lparchive.org/Creeper-World-Series/Update%20171). (reading) The
  strong threats are emitting structures, not big units.
- **They Are Billions.** Specials do one thing each: Venom spits area acid,
  Harpies jump walls, Tanks soak fire for others
  (https://www.gamepressure.com/they-are-billions/zombies/zea7d2). The Giant is
  deaf and slow to notice, then fast, ignores walls and sweeps a 120-degree arc
  that destroys anything, other infected included
  (https://www.moddb.com/news/they-are-billions-update-something-giant-is-coming).
- **Factorio.** Evolution rises with time, pollution produced and spawners
  killed, slowing near the top; mediums appear around 20%, bigs 50%,
  behemoths 90% (https://wiki.factorio.com/Evolution). The player drives the
  escalation.
- **StarCraft Zerg** make strong units by morphing existing ones through a
  vulnerable cocoon (https://liquipedia.net/starcraft/Cocoon); the Brood Lord
  spits Broodlings that expire after 5.71 s
  (https://liquipedia.net/starcraft2/Brood_lord).
- **Grey Goo.** Mother Goos roam absorbing resources and spitting out units;
  a mother can split into smaller mothers or grow into a tentacled super
  weapon (https://pcgamer.com/grey-goo-preview-2/).
- **Infested Planet.** Hives spew constantly, and late on the aliens answer
  each lost hive with drones plus bigger elites
  (https://www.gamewatcher.com/infested-planet/review). Mutations include a
  shell forming at about 25% hive health, a counterattack after each capture,
  and an egg left where a marine dies
  (https://steamah.com/infested-planet-all-mutations-guide/).
- **Plague Inc.** The mass mutates at random: each disease has a mutation
  chance (Virus 2, most others 0.5) granting a free random symptom
  (https://plagueinc.wiki.gg/wiki/Mutation).

(reading) A strong unit stands out from a mass by size; a reserved colour or
aura (DRG red lines, Hades yellow outline, Isaac tints); a different movement
rule (the deaf Giant, the seated Witch); being an emitter rather than a unit;
a minimap icon; its own music.

## 6. Many-unit, merging, splitting and generated bosses

- **Terraria.** King Slime summons slimes as it is hurt, shrinks and speeds up
  as HP falls (https://terraria.wiki.gg/wiki/King_Slime_strategies). The Eater
  of Worlds is a chain of segments with separate health; killing a middle
  segment splits it into two worms that each grow a head and tail
  (https://terraria.wiki.gg/wiki/EoW).
- **Dragon Quest.** In DQ VIII the crown holds the fusion; remove it and the
  King disperses into slimes (https://dragon-quest.org/wiki/KingSlime).
- **Splitters.** The "Asteroids Monster" splits into smaller ones when killed
  (https://tvtropes.org/pmwiki/pmwiki.php/PlayingWith/AsteroidsMonster).
- **Parts.** The Crassus has three breakable bulbs and unbreakable armour
  (https://deeprockgalactic.wiki.gg/wiki/Crassus); Noita's Limatoukka is immune
  to projectiles, so needs another damage type
  (https://noita.wiki.gg/wiki/Limatoukka).
- **Generated.** Dwarf Fortress forgotten beasts roll body plan, material
  (flesh to steel) and attack (venom, webs, fire, dust), from fragile to
  near-indestructible, and are drawn by wealth and noise
  (https://dwarffortresswiki.org/index.php/Forgotten_beast).
- **Nemesis.** An orc that kills the player is promoted; survivors keep their
  scars; orcs fight for rank and vacancies refill; each has learnable
  strengths and weaknesses
  (https://www.gamesradar.com/shadow-mordor-nemesis-system-amazing-how-works).
  Warner Bros. filed a patent on it
  (https://www.pcgamer.com/uk/warner-bros-filed-a-patent-for-the-mordor-games-nemesis-system).

## 7. Full bosses: help and harm

- For: "a change of pace after a long stretch of similar gameplay", and a good
  boss tests the player in a new way rather than being "a bigger harder
  enemy". Against: bosses often abandon the play style the player learned,
  and many exist only because they are expected
  (https://pcgamer.com/are-boss-fights-antiquated).
- In roguelikes, a one-enemy arena strips out the group and terrain
  interactions of normal play; the boss becomes a big body to wear down while
  it spawns minions and teleports; wins feel dull and losses unfair; reused
  bosses make it worse. Suggested instead: a finale that is an enhanced
  version of normal play
  (https://www.gridbugs.org/boss-fights-in-traditional-roguelikes/).
- What helps a run: knowing the boss in advance lets players build toward it
  (https://slay-the-spire.fandom.com/wiki/Bosses); keeping boss beats out of
  adaptive pacing keeps the arc
  (https://cdn.fastly.steamstatic.com/apps/valve/2009/ai_systems_of_l4d_mike_booth.pdf);
  a final swarm from all sides turns the ending into the biggest version of
  the normal game
  (https://www.gamepressure.com/they-are-billions/infected-swarm-15/z3c745).

## Collective organisms

- **Dictyostelium.** Starving amoebae release cAMP; up to about 100,000 cells
  aggregate, in some species move as a slug toward light, then build a
  fruiting body in which about 20% die as stalk
  (https://en.wikipedia.org/wiki/Dictyostelium;
  https://cosmosmagazine.com/biology/social-amoebae-reach-for-the-sky).
  Visible: inward streams, a travelling slug, a tower that kills a fifth of
  itself.
- **Locusts.** Touching of the hind legs raises serotonin; several contacts a
  minute over about four hours flips the phase; nymphs darken with
  yellow-and-black and march in bands of thousands that merge
  (https://en.wikipedia.org/wiki/Locust). Visible: colour and behaviour set by
  local density.
- **Army ants.** Ants that lose the trail circle each other, sometimes to
  exhaustion; Beebe saw a mill 370 m round
  (https://en.wikipedia.org/wiki/Ant_mill). Nightly, 150,000-700,000 workers
  link into a living nest about a metre across that breaks at dawn into
  fan-shaped raiding columns (https://en.wikipedia.org/wiki/Bivouac_(ants)).
  Visible: a vortex; a knot that unfolds into a raid front.
- **Myxococcus xanthus.** On contact with prey the swarm forms ripples, dense
  and sparse bands advancing through the prey, shorter when prey is plentiful
  (https://www.infectioncontroltoday.com/view/predatory-bacterial-swarm-uses-rippling-motion-reach-prey).
  Starving, about 100,000 cells build a fruiting body after a quorum signal;
  under 1% become spores, the rest are eaten
  (https://kaiserlab.stanford.edu/fruitingKL.html). Visible: ripples at the
  feeding front; mounds that consume themselves.
- **Physarum.** Cytoplasm shuttles back and forth roughly every 100 s (flagged
  as unverified in the article) with a peristaltic wave across the network;
  starved in the dark it becomes a dormant sclerotium, in light stalked
  sporangia (https://en.wikipedia.org/wiki/Physarum_polycephalum). Visible:
  pulsing tubes; a hard lump; many-headed stalks.
- **Siphonophores.** The man o' war is a colony of identical zooids working
  as organs: float, feeders, stinging tentacles, reproducers
  (https://en.wikipedia.org/wiki/Portuguese_man_o%27_war). Visible: one body
  of differently specialised parts.

## Structures worth borrowing

1. **Pacing cycle with a relax phase** (Left 4 Dead): elites form only in
   build-up, never in relax, so arrivals follow squad stress.
2. **Dealt deck of boss events** (Left 4 Dead): deal elite types plus
   "nothing" across a planet or the run, no repeats.
3. **Credit budget with cost multipliers** (Risk of Rain 2): the slime's
   stored food is the budget; an elite costs a fixed mass of agents.
4. **Spawn gates** (Space Marine 2): cooldown, low squad stress, forward
   progress, terrain that suits the type.
5. **Visible cocoon window** (StarCraft, Deep Rock Galactic): a dense,
   shootable knot of agents for 10-40 s before the elite hatches.
6. **Density-triggered fusion** (Dragon Quest, Terraria, locusts): agents fuse
   past a local density or kill threshold; breaking the gathering stops it.
7. **A caller that summons** (Helldivers 2): a scout lays a visible plume that
   draws an elite; kill the caller to cancel.
8. **One modifier, one visible field effect** (Diablo III, Path of Exile 3.20):
   each trait is a trail, pool, wall or pulse the simulation draws.
9. **Category caps and banned pairs** (Diablo III, Riftbreaker): at most one
   defensive trait; no healing with resistance.
10. **One colour, one behaviour** (Isaac, DRG): a fixed hue per trait, coming
    from the agents' own state.
11. **Pack-wide versus leader traits** (Path of Exile, Diablo): a trait covers
    a whole region of slime, or one leader and followers.
12. **Optional elite for a relic** (Slay the Spire, Monster Train): fight it
    for something the shop cannot sell.
13. **Ignored elites join the finale** (They Are Billions): survivors merge
    into the end-of-planet surge.
14. **Reward shaped by where it dies** (DRG Crassus): the kill floods its
    surroundings with a resource.
15. **Segmented split** (Terraria Eater of Worlds): cutting a tube elite
    makes two, each growing a head.
16. **Escalation from the player's actions** (Factorio, Plague Inc): the
    elite tier climbs with kills or pumping, not only the clock.
17. **An enemy that remembers** (Shadow of Mordor): an elite that wipes a
    squad returns on a later planet, scarred (mind the patent).
18. **Generated from parts** (Dwarf Fortress): body by material by attack,
    announced with a one-line description.
19. **Boss shown before the planet** (Slay the Spire): show it in the shop so
    the player buys against it.
20. **Finale as normal play at full size** (gridbugs, They Are Billions): the
    boss is the slime at peak mass plus surviving elites, not an arena.
