# Whole-game review, 2026-11-15: game director

Reviewer role: game-director (the game's design as a whole). Main at
17f2e5b8. Software Vulkan (llvmpipe), `larder-world look=DIR`, 13 captures and
about 40 stills, scratch in `/tmp/wgr/game-director/`. Six key stills are in
`Reports/whole-game-review-2026-11-15/game-director/`. The scripted hands are
controls, not players. Wherever this page says what players do, it is
quoting the owner's 11-15 08:30Z notes.

## The game as it plays today

**What the run is meant to be:** land, claim crystals with pumps, turn food
into soldiers and gun cards, cut the slime to 2% of its peak, shop, and land
on a bigger and harder planet. **What it is in play:** an economy race with
no opponent, followed by one fight against a target that stays put and
regrows. On every run planet I filmed (1, 2, 3, 4, 5, and 8 from the kit),
the slime drops in its first 30 s to whatever its home crystals can feed:
1.4k walkers on planet 1, 2.3k on planet 2, 3.3k on planet 4, 5k on planet 5
and 9k on the 6x core world. It then sits in its nest bowl. It never touches
a pump. Meanwhile the squad banks 5,800 food by 2:30 on planet 4 with zero
shots fired. When you do attack, the nest refills what you kill: 4 soldiers
killed 12,262 walkers in 150 s on planet 4 while the slime's count stayed
near 3,000. So a planet comes down to one question: how much damage per
second beats the nest's regrowth? The answer is the 40-soldier cap with the
best gun, which is exactly how the owner says he wins. Turrets, walls,
lures, sirens, outposts and most tools only matter against a slime that
comes to you, so none of them has a job.

**The approach is right.** A simulated enemy whose body is its hit points
and whose crystals are its economy is distinctive. It is Creeper World with
a squad. What is missing is the property that makes Creeper World and They
Are Billions work: **the enemy keeps coming, and it wants what you hold.**
The run's levels got a great deal of map work (biomes, doors, back roads,
moats). The slime's economy, which decides whether any of that map matters,
was handed off as "the slime's economy, not the run's"
([run.md](../../wiki/world/run.md), *Open*), and no thread owns it. The
enemy's progression is flat while the player's compounds through the shop.
The core loop should be **claim, hold, strike**: pumps make the slime come
for them (hold), the slime grows on a clock (pressure), and the gun is how
you answer it (the draw). Every system should feed one of those three
verbs, and anything that doesn't should be parked.

## Findings, most important first

### 1. The run's slime never leaves home, so nothing needs defending
- **Kind:** issue
- **Evidence:**
  - Planet 1 (Drifter), `run=1 mode=squads hand=pumps`: the slime went 1,397 -> 1,462 walkers on its home crystal from 0:30 to 1:37. Both pumps stayed at hp 200/200 (`game-director/01-run-p1-lost-to-stuck-097s.jpg`).
  - Planet 2 (Sprawler), `hand=squadwork`: 2,900 -> 2,319 walkers by 3:20, thin dots and one nest, bank 4,926.
  - Planet 4 (Swarmer): 3,253 -> 3,390 walkers from 1:00 to 2:30, **0 shots**, bank 5,827, +34 food/s (`03-run-p4-static-nests-150s.jpg`).
  - Planet 5 (Sporer): 4,435 -> 5,233 walkers, 24 shots in 100 s.
  - Core world (kit `run-d8.jpg`): 9,011 walkers on a 6x map.
  - A building loses hp only when walkers touch it (`src/mould/larder_world/buildings.rs:624-627`), and nothing draws walkers to a pump. Squad mode has no larder (11-14 04:08Z), so in the run nothing at all draws the slime out.
- **Why it matters for play:** this is the owner's "no rush ... no need to defend". It is the reason walls and turrets never come up, and the reason no strategy needs more than a deathball.
- **Smallest next step:** direction A (finding 7).
- **Impact:** 5. **Effort:** M. **Owner:** unowned (the run's design; the slime-behaviour review owns its parameters).

### 2. The run's slime is a fixed-size sponge that regrows, so each planet is a yes/no damage threshold
- **Kind:** issue
- **Evidence:**
  - `run=1:4 hand=rush`: 4 soldiers had killed 4,627 walkers by 1:00 and 12,262 by 2:30. Over the same span the slime went 2,725 -> 3,031, and its crystals fell only from 0.990 to 0.976 of their charge (`04-run-p4-four-at-the-nest-090s.jpg`).
  - The squad stood at the nest's edge on 55% health with 0 walkers touching it. Nothing pushed back.
  - The win line is 2% of the peak (`run::WIN_SHARE`).
- **Why it matters for play:** below the regrowth rate you can stand there forever. Above it the planet falls quickly. That is a switch, not a spread, and the only lever on it is more guns. This explains the 40-soldier plan better than any balance number.
- **Smallest next step:** a one-seed probe on each planet (not a census): the squad damage per second at which the slime starts to shrink. Put it next to what the shop sells by that depth. That pair of curves is the run's real difficulty curve. Nobody has drawn it.
- **Impact:** 5. **Effort:** S. **Owner:** unowned.

### 3. Enemy progression is flat while the player's compounds
- **Kind:** issue / owner decision
- **Evidence:**
  - `LADDER` changes the tier's shape, not its strength (`src/mould/run.rs:75`).
  - Walkers go up a fifth a planet times the planet's area (`run.rs:509`), but the slime sheds them within 30 s. The core world falls from 65,654 to 7,335, about 4.4x planet 1's slime on 10x its area ([run report, *Run pacing*](../roguelike-run-2026-11-14/README.md)).
  - The shop sells Armour, Heavier shots, Faster trigger, Longer barrels, Bigger landing, Supply drop, and guns with bigger budgets every planet (kit `run-shopmid.jpg`).
  - Owner: "more interesting progression in the enemy ... especially when your guns get stronger".
- **Why it matters for play:** if the planets get easier relative to the player, the run has no arc and the shop has no stakes.
- **Smallest next step:** have the owner pick the progression axes. My proposal is three axes he can see on screen: **reach** (how far it ranges from home), **appetite** (how fast it takes crystals and comes for pumps), and **answer** (one trait that counters the player's last build, direction C). Each axis goes up per planet, with numbers tuned later.
- **Impact:** 5. **Effort:** M. **Owner:** owner, then unowned.

### 4. The reports measured a different slime from the one the owner plays
- **Kind:** issue
- **Evidence:**
  - Free play on the run's own planet-1 map (`map=planet:1:1 hand=idle`) grows a sheet: 26,541 walkers at 1:00 and 35,497 at 6:00, covering half the map (`02-freeplay-p1-sheet-360s.jpg`). The run's slime on that planet is 1,445.
  - The census and the player AI run on the Fork with shape C at census size (cloud plan D1).
  - The AI's best plan, `take_claim` with 16 turrets, and the tier counters ([mould-tiers](../../wiki/world/mould-tiers.md)) were measured in the race world at around 25k walkers.
- **Why it matters for play:** findings about turret value, claim plans and tier counters do not carry over to the run. A confident report can be right about the Fork and wrong about the game.
- **Smallest next step:** label every strategy report with the slime it measured. Make run planets 1-4 the reference for any future strategy question.
- **Impact:** 4. **Effort:** S. **Owner:** census / player-AI thread.

### 5. The "stuck" rule ends a run while the squad is healthy and can still shoot
- **Kind:** issue / low-hanging
- **Evidence:**
  - `run=1 mode=squads hand=pumps`: both pumps ran their small crystals dry ("Crystal 6 has run dry"), the bank sat at 287 against a soldier's 400, and nothing came into range for 30 s. The verdict was "Mission lost: No food to buy or fire with since 1:07 ... The run ends on planet 1" at 1:37, with 4 soldiers at 100 health (`01-run-p1-lost-to-stuck-097s.jpg`).
  - `count_stuck` treats walking as stuck (`src/mould/larder_world/turn.rs:397-406`, `campaign.rs:612`, `STUCK_END` 30 s).
  - Walks between crystals run from 14 s to 60 s on planets 4-8.
- **Why it matters for play:** a pump economy that dries crystals in about 150 s, then a walk to the next crystal, can lose the run with no enemy involved. Firing costs nothing in squad mode, so the reason the game gives is false. Scripted-hand evidence. Whether a human hits this is a guess, but a likely one after pumps dry.
- **Smallest next step:** in the run, count a moving squad, or any standing pump, as not stuck, and warn instead of ending. Or end only when no soldier is alive.
- **Impact:** 4. **Effort:** S. **Owner:** run thread.

### 6. Plain shots are close to invisible; the spectacle is all in the payloads
- **Kind:** issue
- **Evidence:**
  - Zoom 20 on four rifles killing about 80 walkers a second (`05-run-p4-plain-rifle-zoom20-060s.jpg`, 1:00): across three frames 50 ms apart I saw no bullet, no muzzle flash and no hit. The only marks are the dotted aim lines and a soft dent in the body.
  - The shock payload on the Fork (`06-fork-shock-payload-8s.jpg`: 64 casts, 1,840 killed by 8 s) gives a ring crater, a scorched rim and thrown colour. The satisfying gun already exists, but only as a card.
  - Shot lights were cut on 11-14 ("too much") to 2.5 cells, and light now follows damage (decisions-game 20:03-20:22Z).
  - The run starts with the plain rifle.
- **Why it matters for play:** the owner says guns "should really be one of the draws of the game". The first ten minutes of every run are the least satisfying gun in the game.
- **Smallest next step:** see direction E (finding 11).
- **Impact:** 5. **Effort:** M. **Owner:** graphics / guns thread.

### 7. Direction A (first choice): the slime contests the crystals you pump
- **Kind:** direction
- **Evidence:**
  - A lure placed next to a planet-1 pump (`lure=300,62`) drew nothing in 70 s: the slime was about 250 units away and stayed at 1,430. The lure's reach is far too short for a run map.
  - Pumps empty a crystal in about 150 s (1/150 of its food a second, [pump report](../pump-economy-2026-11-15/README.md)), so claims are short-lived and the economy already forces the player to keep expanding.
  - Drawn from Creeper World (the creeper floods toward your network) and They Are Billions (noise draws the horde).
- **Why it matters for play:** this one rule gives turrets, walls and splitting the squad a job. The owner said it himself: "they don't attack my pumping crystals so no need to defend".
- **First experiment (one thread, one day):** a working pump lays the lure's food scent into the slime's own field. Its reach is about a third of the map and grows with the pump's level, and the field is drawn from its own values (the picture is the data). On run planets 1-4, film 3 minutes per planet. Counts: time to the first walker on a pump, pump hp lost, food lost. Smoke checks:
  - an undefended pair of pumps falls within 3 minutes;
  - a turret beside a pump keeps it standing;
  - idle still does not win.
- **Impact:** 5. **Effort:** S. **Owner:** unowned.

### 8. Direction B: the slime grows on a clock and sends fronts you can see coming
- **Kind:** direction
- **Evidence:**
  - Slime counts are flat over minutes on every planet (finding 1). The clock runs 15:00 to 55:30 with no pressure inside it.
  - Drawn from Risk of Rain (a difficulty clock), They Are Billions (waves), XCOM (the Avatar clock) and FTL (the pursuing fleet).
- **Why it matters for play:** this creates minute-to-minute tension and a reason to hurry ("there is no rush").
- **First experiment:** the nest's birth cap rises a set share each minute on the planet. Every 2-3 minutes the nest swells, so the simulation itself telegraphs it, and releases a hungry front toward the nearest pump or squad. On planet 2, film one front. Smoke checks:
  - idle loses by 10 minutes;
  - one wall plus turret line holds one front;
  - the 40-soldier rush still works but is no longer free.
- **Impact:** 5. **Effort:** M. **Owner:** unowned (the slime-behaviour review owns the numbers).

### 9. Direction C: the enemy answers the player's last build
- **Kind:** direction
- **Evidence:**
  - The Grudge already remembers where it was shot (`BigParams::scar_keep`, [mould-tiers](../../wiki/world/mould-tiers.md)), and the Swarmer reads alarm.
  - The run picks tiers by depth alone (`run.rs:486-489`).
  - Drawn from Left 4 Dead's director and the per-act elites of Slay the Spire.
- **Why it matters for play:** this is the direct answer to "challenge should rise as your guns get stronger". The enemy shifts against the dominant plan instead of just inflating.
- **First experiment:** after each planet, read three numbers from the run's own log: the largest squad, shots per spot, and buildings raised. Pick the next tier from them: a deathball meets a tier that floods clumps (Swarmer or Sporer bursts), turret creep meets the Grudge. Put one sentence on the shop's planet card saying why the tier was chosen. This is one function in `run.rs`.
- **Impact:** 4. **Effort:** S. **Owner:** run thread.

### 10. Direction D: break the deathball with linked nests and split objectives
- **Kind:** direction
- **Evidence:**
  - The owner already splits squads to flank a nest. Shape C already shares food (`energy_flow` 0.05, 11-13).
  - Planets have 2-3 nests from planet 4 on ([planets](../../wiki/world/planets.md)).
  - The squad cap is 40 (`rules.rs:506`).
- **Why it matters for play:** if nests feed each other along a visible trunk, one ball at one nest regrows forever, while cutting the trunk or hitting both nests at once pays. Splitting becomes a strategy that counters turtling or massing, not just a style.
- **First experiment:** on planet 4, link both nests' food through the network so they share. Film one ball on one nest against two squads, or a cut first. Count the time to 2%.
- **Impact:** 4. **Effort:** M. **Owner:** unowned.

### 11. Direction E: a gun-feel pass in which the slime shows every hit
- **Kind:** direction
- **Evidence:** finding 6. Drawn from Noita (whose spells are simulation), Nuclear Throne and Enter the Gungeon (shot feel).
- **Why it matters for play:** the owner names guns as the draw. Under the house rule, juice must come from the simulation, not from paint.
- **First experiment:**
  - Every bullet hit throws a few walkers outward (the bomb's throw at about a twentieth) and opens a dent for 0.3 s.
  - The bullet itself is a light-carrying particle in the light field, so it shows in flight.
  - Payload cards are louder versions of the same thing.
  - Start the run with one payload card in the gun.
  - Film a blind A/B at zoom 12 (today vs the pass) for the owner.
- **Impact:** 5. **Effort:** M. **Owner:** graphics / guns thread.

### 12. Map work is ahead of enemy work
- **Kind:** roadmap
- **Evidence:**
  - Planets got biomes, doors and back roads (111 of 112 nests), moats and rims ([planets](../../wiki/world/planets.md)).
  - The moat is "built but off: the slime covers most of it by 30 s".
  - Otherwise the slime never leaves its bowl (finding 1). A nest's front door matters only to the attacker.
- **Why it matters for play:** map polish cannot show up in play until the slime roams.
- **Smallest next step:** park new map features until direction A or B lands. Point the levelgen thread at routes the slime takes toward pumps.
- **Impact:** 4. **Effort:** S (a planning call). **Owner:** coordinator / levelgen thread.

### 13. The tiers barely show at the run's walker counts
- **Kind:** issue
- **Evidence:**
  - Planet 3's Trunk is one blob in its corner (`/tmp/wgr/game-director/p3/planet-3-1-squads-045s-all.png`, 1,665 walkers), with no cords.
  - Planet 1's Drifter is one droplet that does not drift (still 01).
  - Planet 4's Swarmer is two teardrops (still 03).
  - Only the Sporer on planet 5 looks like itself (stars with tendrils, `/tmp/wgr/game-director/p5/planet-5-1-squadwork-100s-all.png`).
  - The tiers' identities were built at about 25k walkers.
- **Why it matters for play:** "slime behaviour changes clearly from world to world" is the owner's word, but on the planets I filmed it reads as size and shape, not as behaviour you play against.
- **Smallest next step:** one still per `LADDER` tier at the run's counts at 1:00. Any tier that reads as a blob gets the walkers it needs, or is swapped.
- **Impact:** 3. **Effort:** S. **Owner:** slime-behaviour review / tiers thread.

### 14. The shop's planet card tells the player something untrue
- **Kind:** low-hanging
- **Evidence:** "the slime is The Swarmer, 2.2 times the first planet's walkers" (kit `run-shopmid.jpg`), but that size is gone within 30 s of landing (finding 3).
- **Why it matters for play:** it promises a harder planet that does not arrive.
- **Smallest next step:** say what the tier does in one sentence instead. Wording only; layout is the other review's.
- **Impact:** 2. **Effort:** S. **Owner:** run thread.

### 15. Two modes, two enemies: the campaign should become the run's opening or be frozen
- **Kind:** owner decision
- **Evidence:**
  - The eight missions teach one verb each on designed maps against the strong free-play slime (kit `mission.jpg`: 3,104 walkers spreading over many crystals at 0:20).
  - The run teaches nothing and its slime is weak (finding 1).
  - Owner: the run "might become the main game" (11-14 15:21Z), and now "the real game".
- **Why it matters for play:** effort is split between two games, and the one being played gets less enemy design.
- **Smallest next step:** offer the owner a card with two options: missions 1-3 become the goals of run planets 1-3 (one verb each), or the campaign is frozen as it stands.
- **Impact:** 3. **Effort:** M. **Owner:** owner.

### 16. The RTS round, the larder and the outpost do nothing in the run's squad mode
- **Kind:** owner decision / cut
- **Evidence:**
  - Squad mode has no larder (11-14 04:08Z), hand draining pays a fifth (`Rules::hand` 0.2), and pumps are the income (11-15 03:07Z).
  - The run still sells "Outposts: a second drop-off" (`run.rs:241`).
- **Why it matters for play:** dead purchases in the shop, plus a second control scheme that every feature must be built twice for.
- **Smallest next step:**
  - Drop the outpost from the run shop.
  - Ask whether the RTS round becomes a legacy sandbox that no longer gets new features.
- **Impact:** 3. **Effort:** S. **Owner:** owner, then run thread.

### 17. The tools and the payload bullets are the same effects bought twice
- **Kind:** owner decision / merge
- **Evidence:** payloads (blast, bomb, gas, germ) are bullets since 11-14 18:05Z. The tools folder (E) and the Tools shop folder still sell blast, mine, det cord, germ and gas ([squad toolkit](../../wiki/world/squad-toolkit.md)).
- **Why it matters for play:** two paths and two UIs for one verb. The gun gets less of the spotlight that the owner wants it to have.
- **Smallest next step:** fold the overlapping tools into gun cards. Keep only the non-gun verbs (mine, det cord) as one "kit" slot.
- **Impact:** 3. **Effort:** M. **Owner:** guns / toolkit threads.

### 18. Lures and sirens need a slime that moves; hold them until it does
- **Kind:** issue
- **Evidence:** the lure test in finding 7. Both are sold in the run (`run.rs:237`, `run.rs:242`).
- **Why it matters for play:** tools that deliver nothing visible break the owner's rule that every tool must deliver something you can see.
- **Smallest next step:** keep them out of the run shop until direction A or B. Afterwards, a lure is the counter to a pump-raiding slime: it pulls the slime off the pumps.
- **Impact:** 2. **Effort:** S. **Owner:** run thread.

### 19. The player's own mould is a strong idea with no place in the run yet
- **Kind:** owner decision / roadmap
- **Evidence:** it exists only as the garden race (`own=1`, kit `own.jpg`), and the run sets `own: false` (`run.rs:503`).
- **Why it matters for play:** later it could be the player's Creeper World network, a living wall that holds pumps. Today it competes with the run for thread time.
- **Smallest next step:** park it. Return to it as a late-run card ("spore pod: seeds your mould at a pump") once pumps are contested.
- **Impact:** 2. **Effort:** L. **Owner:** own-mould thread.

### 20. The owner's replays are the best evidence the project has, and nobody reads them
- **Kind:** low-hanging
- **Evidence:** the window saves `larder-world-replay-MAP-SEED.txt` on Esc or at the end (`src/bin/larder-world/main.rs:32-33`).
- **Why it matters for play:** this review leans on the owner's one paragraph. Replays would show when he reached 40 soldiers, how often he split, and what he bought.
- **Smallest next step:**
  - Ask the owner for two run replays.
  - Write a small reader that prints a play profile: time to N soldiers, splits, pumps, shots per spot, and buildings.
  - Use it as the reference hand instead of the scripted controls.
- **Impact:** 4. **Effort:** S. **Owner:** player-AI thread.

### 21. Is camera shake or hit-stop allowed under "everything on screen is the simulation"?
- **Kind:** owner decision
- **Evidence:** the 11-09 and 11-10 rulings forbid painted effects. Shake and hit-stop paint nothing; they move the camera and the clock. Guess: they are allowed, but no ruling says so.
- **Why it matters for play:** they are the cheapest gun feel there is (Vlambeer's "art of screenshake").
- **Smallest next step:** a card question to the owner, with a 10 s film of each.
- **Impact:** 3. **Effort:** S. **Owner:** owner.

### 22. A planet has no elite or boss moment, and the core world is a bigger planet 7
- **Kind:** direction
- **Evidence:**
  - The `LADDER` fills planets 7-8 by randomly choosing one of the last three tiers (`run.rs:488`).
  - The core world is 9k walkers in four stars with tendrils (kit `run-d8.jpg`).
  - Drawn from Slay the Spire's elites and act bosses, and FTL's flagship.
- **Why it matters for play:** the run's story ("towards the core slime world") has no climax.
- **Smallest next step:** design one "heart" per planet, a nest that does one special thing (spores, a moving nest, a nest that feeds the others). On the core world, all of them.
- **Impact:** 3. **Effort:** M. **Owner:** unowned.

### 23. Tension today is something the player does to himself
- **Kind:** issue (design note)
- **Evidence:** the only danger I saw came from walking into thick slime (deadly by the 11-12 ruling). Standing at a nest's edge was safe (finding 2).
- **Why it matters for play:** a game whose only risk is the player's own choice feels like a chore once the safe distance is known.
- **Smallest next step:** judge directions A and B by "seconds the enemy spends pressing on something of the player's" per minute. That is the measure to name before their films.
- **Impact:** 3. **Effort:** S. **Owner:** unowned.

## Systems: keep, merge, park

| System | Feeds the loop? | Call |
|---|---|---|
| Squad mode, gun, cards, rolled bullets | yes (strike) | keep; gun-feel pass (11) |
| Pumps | yes (claim) | keep; make them draw the slime (7) |
| Turret, wall | only once the slime comes (hold) | keep; they wake up with A or B |
| Lure, siren | not until the slime moves | hold out of the run shop (18) |
| Outpost, larder, hand carrying | no (squad mode) | drop from the run (16) |
| Tools overlapping payloads | duplicate | merge into cards (17) |
| Plants and fire | terrain and a hazard | keep; fire as a slime-burning tool fits strike |
| Campaign | a second game | make it the run's opening or freeze it (15) |
| Own mould | not in the run | park (19) |
| Map polish | outruns the enemy | pause until A or B (12) |

## Directions, ranked, each with a one-day first experiment

1. **A: the slime contests what you pump** (finding 7): pump scent into the slime's field; film planets 1-4.
2. **E: the slime shows every hit** (finding 11): a bullet throws a few walkers; blind A/B at zoom 12.
3. **B: a growth clock and telegraphed fronts** (finding 8): rising birth cap plus a front every 2-3 min; planet 2.
4. **C: the enemy answers your last build** (finding 9): pick the next tier from the last planet's log.
5. **D: linked nests** (finding 10): nests share food over a visible trunk; planet 4.

Games drawn on: Creeper World (spreading enemy that floods your network),
They Are Billions (noise draws the horde; waves against walls), Risk of Rain
(difficulty clock), XCOM and FTL (doom clock, pursuit), Left 4 Dead (the
director that answers the player), Slay the Spire (elites and act bosses),
Noita, Nuclear Throne and Enter the Gungeon (gun feel and simulated spells).
