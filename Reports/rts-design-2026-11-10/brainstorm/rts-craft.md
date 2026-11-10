# RTS craft for Bloomfront: what makes strategy fun, and what it asks of our mould

**Status: research and brainstorm only. Nothing built, no repo file edited.**
Written 2026-11-10 against `Reports/rts-design-2026-11-10.md` (Bloomfront),
`Reports/mould-review-2-2026-11-10.md` and `Reports/dead-ends.md`.

How to read the sources. Every claim from outside the repo has its URL beside
it, and each page was fetched in this session. Quotes are as the fetch tool
returned them. Where I lean on memory, the line says **from memory,
unverified**. Repo facts cite the repo file. The session's shared fetch budget
ran out partway. The pages I could not fetch are listed under *Not fetched*
at the end, and nothing from them is used.

---

## Part 1. The principles, each with its source

### 1.1 Interesting decisions

| # | Principle | Source |
|---|---|---|
| D1 | "Games are a series of interesting decisions." Interesting ones involve a **trade-off**, are **situational**, are **persistent**, let you **express a play style**, and come with enough **information** ("err on the side of providing the player with too much"). | Sid Meier, GDC 2012, written up by Leigh Alexander: https://gamedeveloper.com/design/gdc-2012-sid-meier-on-how-to-see-games-as-sets-of-interesting-decisions |
| D2 | A good decision is "should I", not "can I". Decisions come from "one big, tight gameplay system where everything is pointing to a central core mechanism." | Keith Burgun in "Three perspectives on strategy game design": https://www.gamedeveloper.com/design/three-perspectives-on-strategy-game-design |
| D3 | Strategy games must be transparent: players should "understand the rules and are able to think them through clearly." | Soren Johnson, same article: https://www.gamedeveloper.com/design/three-perspectives-on-strategy-game-design |
| D4 | "Given the opportunity, players will optimize the fun out of a game." "A single, dominant strategy actually takes away choice from a game because all other options are provably sub-optimal." | Soren Johnson, "Water Finds a Crack": https://www.designer-notes.com/game-developer-column-17-water-finds-a-crack/ |

**For Bloomfront:** our salt-wall opening is D4's crack. Once found, it is
the only opening worth making, so it removes choice rather than adding it.

### 1.2 Attention is the real resource: macro, micro and APM

| # | Principle | Source |
|---|---|---|
| A1 | "So attention is a resource." In Tooth and Tail the commander is also the camera and the scout, so you cannot build or fight while you look. That makes bluffing easier. | Andy Schatz, interviewed by Alex Wawro: https://gamedeveloper.com/design/crafting-i-tooth-tail-i-a-gamepad-rts-that-aims-to-i-hearthstone-starcraft-i- |
| A2 | Judge play at four time scales: one second, ten seconds, one minute, ten minutes. "In StarCraft, the one-second interaction is not particularly fun." | Andy Schatz, interviewed by Kris Graft: https://gamedeveloper.com/design/can-the-developer-of-i-monaco-i-solve-the-gamepad-rts-riddle- |
| A3 | In 3,360 StarCraft II players, action latency (the look-decide-act cycle) was the top predictor of league in the middle leagues. APM outranked it only in the Diamond-against-professional comparison. Worker production mattered in low leagues and faded higher up, read as automatization. The study is observational. | Thompson, Blair, Chen and Henrey, PLOS ONE 2013: https://pmc.ncbi.nlm.nih.gov/articles/PMC3776738 |
| A4 | Unit selection is "prone to error, difficult to manage, and mentally taxing." "Unit selection, however, should die," while RTS games "should live." Automating shipping gave players time for higher-level decisions and to watch rivals. | Soren Johnson, OTC Designer Notes #1: https://www.designer-notes.com/otc-designer-notes-1-unit-selection/ |
| A5 | Easing the interface "is *not* about removing micromanagement." "Zero-K made surface-level micro easier and found more interesting micro underneath." Each click should mean more, not that there are fewer. | Zero-K wiki, "Cold Takes/3": https://zero-k.info/mediawiki/Cold_Takes/3_-_Fight_your_opponent,_not_the_UI |
| A6 | Lower the skill floor, keep the ceiling: "It's okay to be a high-skill game." Stormgate's optional Buddy Bot can take over worker and army production, in parts the player chooses. | Tim Morten and Tim Campbell (Frost Giant), reported by Lincoln Carpenter: https://pcgamer.com/games/rts/stormgate-developers-say-they-want-to-lower-the-rts-skill-floor-not-the-skill-ceiling-its-okay-to-be-a-high-skill-game |
| A7 | StarCraft II kept micro, macro, flanking and terrain as the skills that make it a sport; Browder wanted it "easy to learn, impossible to master." | Dustin Browder, via Michael McWhertor: https://kotaku.com/the-sacrifices-of-starcraft-ii-made-in-the-name-of-spor-5777029 and Kris Graft: https://gamedeveloper.com/design/interview-making-the-fun-meant-taking-out-the-fun-in-i-starcraft-ii-i- |

**For Bloomfront:** our mould is already "the computer doing the micro". The
craft lesson is to spend the freed attention on *where* and *when*, never on
aim. Today the warden holds the lamp 102-150 s of a 150 s game and needs aim
within about 2 cells (`Reports/mould-review-2-2026-11-10.md`). That is A2's
dull one-second loop, and A3's precision skill rather than decision skill.

### 1.3 Economy: expansion, spending and investment

| # | Principle | Source |
|---|---|---|
| E1 | Expansion risk can be scored from time to pay back, relative cost and time to destroy. Higher risk makes expansion sites rigid and maps samey. Early expansions are the riskiest. | Philippe Cwik: https://gamedeveloper.com/design/the-impact-of-the-economy-in-a-real-time-strategy-game-on-its-multiplayer-level-design |
| E2 | "Sunk costs kill players." Units and buildings that cannot be recovered make snowballing worse. Supreme Commander's salvageable wrecks soften it. | Brandon Casteel: https://www.gamedeveloper.com/design/the-balance-of-power-progression-and-equilibrium-in-real-time-strategy-games |
| E3 | Company of Heroes territory only pays when it is connected to the HQ, so cutting a link severs the flow. The system is "rules-light and possibilities-heavy" and pushes players out of their base. | Brendon Chung: https://www.blendogames.com/news/post/2017-05-25-on-company-of-heroes/ |
| E4 | Relic's earlier spreading, influence-based territory "was a little bit unclear, and it didn't look nice" (Quinn Duffy). Fixed regions helped players read the map (Ian Thomson). | Tom Senior, PC Gamer: https://pcgamer.com/the-making-of-company-of-heroes-design-prototypes-and-the-donkeyschreck |
| E5 | Rush, economy and defence are best read as *timing*: "Rushdown is going for an early-game win," economy a mid-game win, defence a late-game win. Finite resources and a hard length limit make everyone cash in. Avoid hard counters, because "reacting isn't making decisions." | Keith Burgun: https://keithburgun.net/designing-strategy-rushdown-economy-and-defense/ |
| E6 | In Majesty you pay gold for a goal and the heroes decide whether to take it. Cancelling a reward does not refund it. | Damiano Gerli: https://genesistemple.com/majesty-kings-are-the-slaves-of-real-time-strategy |
| E7 | In Offworld Trading Company the map's shape is terrain plus "the vectors connecting buildings to their HQs"; travel costs discourage building far away. | Soren Johnson: https://www.designer-notes.com/otc-designer-notes-1-unit-selection/ |

**For Bloomfront:** E3 is our tube rule, already built. E4 is the warning:
emergent, spreading territory is hard to read. E5 says the bell is a design
tool, not just a timer.

### 1.4 Information: scouting, fog and reading the opponent

| # | Principle | Source |
|---|---|---|
| I1 | Hidden information makes "executing actions based on partial knowledge" a skill. "Knowledge then, becomes a resource that can be denied to their opponent or wrested from them." Most RTS games copy one fog model without rethinking it. | Brandon Casteel: https://waywardstrategy.com/2015/01/30/battlefield-uncertainty-and-fog-of-war/ |
| I2 | Fog should control *when* information arrives. "Don't think about fog in terms of a physical, literal thing." Do not let players buy the fog away. Keep static things known. Show the aggregate ("how pushed the lane is"), not every detail. | Keith Burgun: https://keithburgun.net/fog-of-war-in-push-the-lane-and-strategy-games-generally/ |
| I3 | Tooth and Tail gets its depth from hidden information and unit tiers: "So we give you enough information to make interesting decisions." | Andy Schatz, via Alex Wawro: https://gamedeveloper.com/design/crafting-i-tooth-tail-i-a-gamepad-rts-that-aims-to-i-hearthstone-starcraft-i- |
| I4 | Reading a moment needs familiar cues: "you are able to filter out everything except the one or two cues that you're waiting for." Yomi needs "some basis to predict" what the opponent will do. | David Sirlin, *Playing to Win*, ch. 9: https://www.sirlin.net/ptw-book/9-presence-of-mind |
| I5 | Mitigate early randomness, for example by revealing spawn points. Prefer randomness that gives options. | Frost Giant team: https://lr.in.psf.lt/r/FrostGiant/comments/ryl1hv/our_thoughts_on_competitive_map_design |

**For Bloomfront:** I2 argues against the design doc's sensor-reach fog, which
is literal line of sight. I4 points to a better source of reading: the
mould's own slowness. A move that takes seconds to bite is a cue the rival can
learn to see.

### 1.5 Map design: contested resources, chokepoints, the mid-game

| # | Principle | Source |
|---|---|---|
| M1 | A map pool should favour diversity, each map promoting "one of several distinct styles of play," over perfect balance per map. Strong faction asymmetry constrains chokepoint sizes and expansion sites. | Frost Giant team: https://lr.in.psf.lt/r/FrostGiant/comments/ryl1hv/our_thoughts_on_competitive_map_design |
| M2 | Map elements that react to play (wrecks, craters, broken bridges) make "both/all players ... react to the changing battlefield." Competitive players also value maps that do not "mess with a player's expectations." | Brandon Casteel: https://gamedeveloper.com/design/dynamic-map-elements-in-rts-games |
| M3 | Casteel suggests faster build-up and more points of interest, so a player who falls behind can still contest the map. Company of Heroes 2's partly independent objectives help. | Brandon Casteel: https://www.gamedeveloper.com/design/the-balance-of-power-progression-and-equilibrium-in-real-time-strategy-games |
| M4 | In Liquid War, open maps are easiest and ring maps hardest with more than two players. A strong tip: "cut your opponent off walls and surround him completely." | Liquid War manual (Christian Mauduit; tips by Jan Samohyl): https://manpages.ubuntu.com/manpages/trusty/man6/liquidwar.6.html |

**For Bloomfront:** the late seam piles failed because they landed
equidistant and unannounced (`Reports/dead-ends.md`). M2 and M3 suggest
contested points that *play creates*, not points the designer drops.

### 1.6 Asymmetry, balance and counters

| # | Principle | Source |
|---|---|---|
| B1 | Keep rosters small: "Too many units causes confusion." Counters that flip with upgrades keep a matchup uncertain. Units should be "as big as we dare" so the screen stays readable. | Dustin Browder, via Michael McWhertor: https://kotaku.com/the-sacrifices-of-starcraft-ii-made-in-the-name-of-spor-5777029 |
| B2 | Before changing a unit, say what it should excel at, struggle with, beat and lose to. No two units should fully overlap. "Buff strengths, nerf weaknesses," or every unit drifts to the average. Masses of a bad unit can beat a few good ones (Lanchester's square law). | Zero-K, Balancing Guidelines: https://zero-k.info/mediawiki/Balancing_Guidelines |
| B3 | Judge balance by play, not stats: "If a unit cannot be used effectively in game, then it isn't OP (yet)." Test on several maps and skill levels. | Same page: https://zero-k.info/mediawiki/Balancing_Guidelines |
| B4 | Browder kept pitching fun ideas that did not fit a sport, and they were cut: "all of those would get shot down for a sport." | Dustin Browder, interviewed by Kris Graft: https://gamedeveloper.com/design/interview-making-the-fun-meant-taking-out-the-fun-in-i-starcraft-ii-i- |

### 1.7 Pacing: openings, snowballs and comebacks

| # | Principle | Source |
|---|---|---|
| P1 | Slippery slope (falling behind makes you fall further) is "usually a bad property": "the real victor of the game is decided early on and the rest of the game is futile to play out." StarCraft is named. | David Sirlin: https://sirlin.net/articles/slippery-slope-and-perpetual-comeback |
| P2 | The fix is *limited* slippery slope: capped and temporary. In Street Fighter "you cannot get doubly knocked down." Perpetual comeback is useful but "can easily go wrong if tuned improperly." | Same page: https://sirlin.net/articles/slippery-slope-and-perpetual-comeback |
| P3 | "Homeostatic design" measures "how difficult it is to achieve an advantage that an opponent cannot overcome." "The early game is one of the most frustrating phases"; "Fast, or shallow, build up phases" help. | Brandon Casteel: https://www.gamedeveloper.com/design/the-balance-of-power-progression-and-equilibrium-in-real-time-strategy-games |
| P4 | Company of Heroes' retreat sends a squad home by the fastest route, out of your control, and squads are "overwhelmingly successful in making it home." Kept squads "build an advantage over time" (Quinn Duffy). | Brendon Chung: https://www.blendogames.com/news/post/2017-05-25-on-company-of-heroes/ ; Tom Senior: https://pcgamer.com/the-making-of-company-of-heroes-design-prototypes-and-the-donkeyschreck |
| P5 | Creeper World uses "safe areas" so players can regroup; its maps "can be forgiving when you make mistakes." | Knuckle Cracker, interviewed by Josh Bycer: https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/ |
| P6 | Browder defended the early rush: "By delaying the rush, you're just delaying when the fun begins." | Dustin Browder, via McWhertor: https://kotaku.com/the-sacrifices-of-starcraft-ii-made-in-the-name-of-spor-5777029 |

**For Bloomfront:** review 2's finding is P1 in miniature. Letting go at 0:30
keeps the first star on 8 of 8 seeds, and the late game adds little.

### 1.8 Accessibility against depth

| # | Principle | Source |
|---|---|---|
| X1 | Remove chores, keep decisions: "We wanted to take anything that felt like a chore in your classic RTS and dump it." The aim was "king of the dorm room, rather than king of South Korea." | Andy Schatz, via Alex Wawro: https://gamedeveloper.com/design/crafting-i-tooth-tail-i-a-gamepad-rts-that-aims-to-i-hearthstone-starcraft-i- |
| X2 | Rise of Nations' usability labs made "hundreds of changes to the game, making it more streamlined and easy to jump into." | Brian Reynolds, postmortem: https://gamedeveloper.com/disciplines/postmortem-big-huge-games-i-rise-of-nations-i- |
| X3 | Bullfrog's games were criticised because "what you saw in the first hour ... was what you would continue to see for the next hundred hours." | Jimmy Maher, *The Digital Antiquarian*: https://www.filfre.net/?p=4362 |
| X4 | Tooth and Tail's single cursor means "dividing your attention simply isn't allowed," and the game "does away with most of macro play too." Critic Bruno Dias calls it "secretly a game about micro." | Bruno Dias, Vice: https://vice.com/en/article/zm3vze/tooth-and-tail-is-a-cute-strategy-game-at-war-with-its-own-genre |
| X5 | A1-A6 above are the main newer thinking: delete selection (Johnson), make clicks mean more (Zero-K), offer optional automation (Frost Giant). | See 1.2 |

### 1.9 Indirect control and emergent units

| # | What worked or failed | Source |
|---|---|---|
| C1 | Indirect control hurts "Predictable Consequences" when the influence depends on randomness or hidden state. It also makes losing a unit less of a penalty, and turns a "go there" goal into a herding goal. | Game design pattern "Indirect Control", Eliens' course pages: https://math.vu.nl/~eliens/media/pattern-indirectcontrol.html (the page does not name its book; it reads like Björk and Holopainen's *Patterns in Game Design*, **from memory, unverified**; still unverified after verification pass: neither the page nor its index, https://www.few.vu.nl/~eliens/media/pattern.html, names a book; both credit only "(C) Æliens 04/09/2009". The quoted claim itself is confirmed on the page: it "affects Predictable Consequences of actions negatively when the influence depends upon Randomness or parts of the game state unknown to players".) |
| C2 | **Majesty, failure mode.** Bounties were "an expensive and unreliable process"; a large reward would "only inspire one or two parties to wander over"; "it seems absurd that your heroes take so long to respond." Verdict: "Just isn't very fun." | Tom Francis, Majesty 2 review: https://www.gamesradar.com/majesty-2-the-fantasy-kingdom-sim |
| C3 | Majesty's first game: "there's no real guarantee the hero will do what the player asks." Designer Jim DuBois: "the player makes up models in their own mind of what is happening behind the scenes." | Gerli: https://genesistemple.com/majesty-kings-are-the-slaves-of-real-time-strategy ; DuBois interview: https://genesistemple.com/interview-with-jim-dubois-on-pc-game-majesty |
| C4 | **From Dust.** "The player just controls the obstacles on the terrain." Eric Chahi: "It would be very frustrating if the player can't anticipate what the plan is." Builds where you could not redirect a villager were "a really difficult issue." | Eric Chahi, interviewed by Brandon Sheffield: https://gamedeveloper.com/design/the-core-of-i-from-dust-i- |
| C5 | From Dust's tribe has "trouble changing paths if you make an easier path," and unforeseeable disasters push levels toward trial and error and puzzles. | Josh Bycer: https://gamedeveloper.com/design/dusty-an-analysis-of-from-dust |
| C6 | **Liquid War.** The army flows by the shortest path to your cursor. Particles never die; they change sides. Tips: surround, cut foes off walls, and "always move your cursor through the enemy, not the other way around." Parking the cursor in your own army "is the best way to loose" [sic]. | Liquid War manual: https://manpages.ubuntu.com/manpages/trusty/man6/liquidwar.6.html ; Wikipedia (Thomas Colcombet's algorithm, Christian Mauduit's game): https://en.wikipedia.org/wiki/Liquid_War |
| C7 | **Populous and the god game.** The label came from "direct control over an environment but only indirect control over the people therein." Its two-player link-up went in before anyone considered a computer opponent. | Jimmy Maher: https://www.filfre.net/?p=4362 |
| C8 | **Black & White.** The team avoided random elements in the creature's AI to keep it predictable. "The Creature mirrors you and your actions." | Peter Molyneux, postmortem: https://gamedeveloper.com/design/postmortem-lionhead-studios-i-black-white-i- |
| C9 | **Empires of the Undergrowth.** Ants are steered by pheromone markers that end foraging trails, plus behaviour toggles, "eschewing the traditional drag-box-and-right-click." | Mike Connor (Slug Disco), postmortem: https://gamedeveloper.com/design/postmortem-how-empires-of-the-undergrowth-came-together-in-over-7-years-of-early-access |
| C10 | **The simulation must reach the player's head.** "Anything in the Game Model that doesn't copy into the Player Model is worthless." Aim for "story-richness," and "choose the minimum representation that supports the kinds of stories you want." | Tynan Sylvester, "The Simulation Dream": https://gamedeveloper.com/design/the-simulation-dream |
| C11 | **Company of Heroes' retreat** is a loved indirect order because it is reliable (see P4). | Chung, as P4 |
| C12 | **Mushroom 11**: the "random movements the mushroom tends to make really detract from the fun." | Cubed3, quoted in the repo's `Reports/mould-game-2026-11-08/brainstorm/research.md` (not re-fetched this session) |
| C13 | **Creeper World** set out to "simplify strategy gaming" with an enemy "based on a fluid simulation." | Knuckle Cracker, via Bycer: https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/ |

**The indirect-control rules these add up to:**

1. **Fast, reliable in aggregate.** Majesty failed on slow, unreliable
   response (C2). The CoH retreat works because it nearly always lands (C11).
2. **Anticipate, then redirect.** The player must foresee the plan and be able
   to change it mid-way (C4, C5).
3. **Shape the world, not the creature.** Populous, From Dust and Liquid War
   give direct control of the ground or a point (C4, C6, C7). The crowd is
   steered only through it. That is our salt, lamp and water.
4. **Randomness and hidden state eat agency** (C1, C8, C12).
5. **What the player cannot see did not happen** (C10, C3).
6. **Paying is fine if paying visibly works** (C2 against E6).

---

## Part 2. Bloomfront as written, against these principles

### 2.1 Where it is strong

| Strength | Principle it meets | Why |
|---|---|---|
| One food store pays for growth, upkeep and every tool | D1 trade-off, E6 | Every click becomes "should I" (D2). This is the design's best idea. |
| Tubes as supply lines; a cut starves what is beyond | E3 | Company of Heroes proves supply cut-off carries a whole RTS. |
| Fed tube halves fight losses; unfed doubles them | P3, P5 | A defender near home is stronger. That is built-in homeostasis. |
| Verbs act on the ground, not on units | Rule 3 (C4, C6, C7) | We sit in the god-game lineage, where indirect control has worked. |
| Feast scores a running share | P2 | A crash in the last ten seconds cannot swing it. |
| Blooms announced with a countdown | I5, C4, dead-ends re-test condition | Input randomness the player can prepare for. |
| AI via `Player`; `HJIT`/`HLAG` as difficulty | A6 | A difficulty slider is free, and AI-versus-AI is the balance test (B3). |
| Stage gates: "letting go loses to playing on" | P1, P3 | It names the mid-game as the thing to prove. |
| Mould answers over seconds, so lockstep delay is hidden | — | A real fit between engine and genre. |

### 2.2 Where it is thin

| Gap | Principle | What the doc says | What is missing |
|---|---|---|---|
| Decisions are listed as verbs, not as choices | D1, D2, E5 | Seven verbs with costs | What does a keeper choose *between* at 2:00? No rush/economy/defence timings. |
| No attention model | A1-A5 | "Every verb is a click, a hold or a drag" | The lamp is a *hold* verb. It eats all attention (held 102-150 s of 150) and rewards aim within about 2 cells. What does an unattended front do? |
| Snowballing not designed against | P1-P3, E2 | Risk 1 names "decided in the opening" | No limited-slippery-slope rule. More mould eats more; pile tables split by head count reward the bigger crowd; the running share banks an early lead. |
| Fog framed as line of sight | I1, I2 | Sensor-reach fog as an owner question | Burgun warns against literal fog. No other source of reading the rival is proposed. |
| Map has wells and blooms, not shape | M1-M4 | Home, contested wells, blooms | No chokepoints, lanes, damp or dry ground, or second front. The dish is 256 x 160, so rush distance is tiny. |
| Territory readability | E4, C10 | Colour per mould | Relic dropped spreading territory because players could not read it. Our borders are emergent trail. |
| Species before one species is deep | B1, B2, X3 | Four species as presets | No "beats / loses to" sheet per species, and one species has not yet shown a mid-game. |
| Temper may be a dominant switch | D4, B2 | Forage, Hold, Fight with a cooldown | No statement of what each temper loses to. |
| Indirect-control health is not measured | Rules 1-2 | Risk 3 notes 45 vs 22.5 | No gauge of response time or reliability per verb. |

### 2.3 What it is missing

1. **A timing triangle.** Rush, economy and defence must each be able to win
   at a different time (E5), with no opening dominant (D4).
2. **A way to fall back.** CoH's retreat (P4) and Creeper World's safe areas
   (P5) both cap a loss. Our sleep and crust could be that verb.
3. **Salvage.** Fights destroy food, so a lost front is pure sunk cost (E2).
4. **Contested points that play creates** (M2, M3).
5. **Telegraphed commitment.** The mould's lag is a cue the rival can learn to
   read (I4). The doc does not use it.
6. **Investment against army.** Spores are the only payback-later spend. The
   tube-memory rule (`Params::wall_grow`) is an untried road network.
7. **A precision budget.** Effects should scale with intent, not with cells of
   aim (A2, A3).

---

## Part 3. Proposals on the model's real levers

Repo facts behind these. The Wild One runs at 22.5 degrees, 150 s, seven
mirrored piles, tools 4 oats, 1 salt, 3 snips, no water
(`src/mould/level.rs`). Defaults: `feed_reach` 70, `flow` off, `pieces` on,
`wound` 4 s, `fight` 0.02, `wet` off (`src/mould/mod.rs`). Salt is a binary
`Ground::Salt` that kills walkers standing on it. `fight_losses` does not
treat sleepers differently.

**Common method for every measurement below.** Scripted hands on paired
seeds (16 unless said), mirrored dish, deciding measure named first.
Controls: idle and random always run. Quote paired counts with a two-sided
sign test. Correct across the comparisons in that census (Holm), and say
which survive. Quote mean and worst frame cost for anything that runs every
step. Look at the dish first (`mouldprobe look=DIR`).

### 3.1 Summary, ranked

| Rank | Proposal | Levers | Fixes | Owner ruling it touches | Cost |
|---|---|---|---|---|---|
| 1 | Response census for every verb | all tools | Indirect control unmeasured | Every tool visible; outcome a spread | Small |
| 2 | Lanterns: lamps you set down and pay for per second | lamp, store | Attention monopoly, aim test | Picture is the data; every tool visible | Small-medium |
| 3 | Salt that weathers | salt, store | Opening decides the game | Picture is the data | Small-medium |
| 4 | Long tubes cost more upkeep | tubes, feed reach, store | Snowball; no expansion risk | Picture is the data | Small |
| 5 | A cut that holds until regrown | knife, tubes | Raids move nothing | Every tool visible | Small |
| 6 | The fallen feed the ground (salvage) | fights, piles | Sunk costs; no mid-game hot spots | Picture is the data | Small-medium |
| 7 | Crust as a fall-back | sleep, lamp, water, fights | No capped loss | Every tool visible | Small |
| 8 | Opening triangle census | hands only, then levers 3-6 | Dominant opening | Outcome a spread | Small |
| 9 | Half-minute rounds inside the Feast | score | Banked early lead | Outcome a spread | Small |
| 10 | Map shape from fields | wet, salt, piles | No chokes, lanes or second front | Picture is the data | Small-medium |
| 11 | Telegraphed commitment | spores, oats, blooms | Nothing to read in the rival | Picture is the data | Medium |
| 12 | Temper triangle | sensor angle, `settle_seconds` | Temper as dominant switch | Fun over fidelity | Small |
| 13 | Trunks as investment | trail, `wall_grow`, store | No tech-versus-army choice | Picture is the data | Medium-large |
| 14 | Fog only as "what your mould felt", and late | trail, `walkers_seen` | Scouting | Picture is the data | Medium |

### 3.2 The proposals

#### 1. Response census for every verb (small)

- **Change.** Add a probe mode that applies each tool once, at fixed places,
  on 16 seeds. It logs the time to a visible effect. Examples: an oat's first
  walker and first join; a lamp's first trail moved; a cut's first grey cell
  beyond it; water's first woken walker.
- **Fixes.** Majesty's failure was slow, unreliable response (C2). From Dust's
  was poor anticipation (C4). We have no gauge of either.
- **Ruling.** "Every tool must deliver something you can see", and "an
  outcome is a spread". This makes both measurable per tool.
- **Measure.** Per tool: median and 10-90% range of seconds to effect, and
  the share of seeds with any effect. Bar to discuss with the owner: an
  effect on 16 of 16 seeds within about 3-10 s, with the 90th percentile
  under twice the median. Today's snip should fail it; that confirms the
  gauge.

#### 2. Lanterns: set the lamp down, pay per second (small-medium)

- **Change.** A click sets a lantern; a second click lifts it. Each burning
  lantern draws food per second from the store. Allow two or three. Light
  from all lanterns sums into the one `light` field, so `push_light` is
  unchanged. Two keepers' lanterns share the field (Stage 0 needs that
  anyway).
- **Fixes.** The held lamp is a one-second loop (A2) that eats attention and
  rewards aim (A3). A lantern turns it into a ten-second choice: where, and
  for how long can I afford it. A lantern left burning is Meier's persistent
  decision (D1). Several lanterns create fronts (A1).
- **Ruling.** The light is the simulation's own field and is already drawn.
  Defaults on. Dead-ends: this keeps the push lamp, not the rejected sensor
  lamp (`light_weight`).
- **Frame cost.** One light box per lantern; quote mean and worst frame at 1,
  2 and 3 lanterns.
- **Measure.** Deciding measure: the aim gap, share(exact) minus
  share(`HJIT=3`), paired on 16 seeds, held lamp against lanterns. Pass:
  the gap shrinks while the clumsy lantern hand still beats random and idle
  on at least 14 of 16. Second: `STOPAT=30` against playing on, with
  lanterns. Playing on must beat today's margin: clumsy play to the
  bell led letting go 7-0 with a tie (8 seeds, review 2).

#### 3. Salt that weathers (small-medium)

- **Change.** Salt costs food per cell and fades over about 20-40 s back to
  agar, the way a `Ground::Wound` heals. Keep a per-cell time left and draw
  salt by it, so a fading wall looks thinner because it is. Optional later:
  mould pressing on salt wears it faster.
- **Fixes.** The opening salt wall takes a star for the whole game (D4, P1).
  A weathering wall buys time, not territory. Re-laying it is a recurring
  spend that competes with growth.
- **Ruling.** The picture is the data: the drawn strength is the held time
  left, through one ramp.
- **Baseline.** On food share, letting go at 30 s scores 0.567-0.577 and the
  clumsy warden 0.575-0.584 (8 seeds, comment in `src/mould/level.rs`).
- **Measure.** Deciding measure: `STOPAT=30` against the warden to the bell,
  share of food eaten, 16 paired seeds. Pass: playing on wins at least 13 of
  16 (p under 0.05, Holm-corrected across this census). Also check that
  `nestsalt` and `jail` stop beating idle 8-0 but still beat it on most
  seeds: the opening should matter, not decide.

#### 4. Long tubes cost more upkeep (small)

- **Change.** Upkeep per walker grows with its tube distance from food.
  Read it from `Dish::fed`, the u16 tube distance the census already keeps.
  For example `upkeep * (1 + k * fed / feed_reach)`.
- **Fixes.** Snowballing (P1, P3) and expansion with no risk (E1). A mould
  that sprawls pays for its reach. A compact defender near home is cheaper.
  This is a limited slippery slope (P2): the leader's growth is taxed, not
  reversed.
- **Ruling.** The hunger blend already drawn shows the walker's food. The
  extra cost shows as faster greying at the tips. With `flow` on, carried
  food already thins toward tips; check whether flow gives this for free
  before adding a term.
- **Measure.** Deciding measure: lead changes. Run warden against warden with
  one side handicapped (for example a 30% smaller starting blob). Count games
  where the handicapped side's share at the bell is within 0.05 of 0.5, with
  and without the term, 16 paired seeds. Pass: more near-even finishes, while
  warden still beats the clumsy warden on at least 13 of 16 (skill still
  wins).

#### 5. A cut that holds until regrown (small)

- **Change.** Already handoff step 3 in the design doc. Make the wound last
  until tube regrows across it, or a longer `Params::wound`, so the far piece
  (`pieces` on) runs out of food and greys.
- **Fixes.** Supply raids are the CoH core (E3). Today the snip moves
  nothing: three cuts through the nest left 0 stars on 8 of 8 (review 2).
- **Ruling.** Every tool must deliver something visible. The grey is the
  piece's real food running out.
- **Dead-ends.** `STORM_CUTS` found three cuts split even a looped network.
  That entry is about the storm's toughness star; a player's raid *should*
  split. Its re-test condition (healing across a scar) is the same change.
- **Measure.** Deciding measure: grey cells beyond the cut at +5 s and +10 s,
  and food share change, `nestsnip` against idle, 16 paired seeds. Pass:
  grey on at least 14 of 16, and a repair hand (oats across the cut) recovers
  more share than a non-repair hand.

#### 6. The fallen feed the ground (small-medium)

- **Change.** A walker killed in a fight leaves a little food on its cell,
  a fraction of what it cost. Either mould can eat it. The amount is a free
  parameter; zero is today.
- **Fixes.** Sunk costs (E2). It also makes the seam where fights happen into
  a resource, so play creates mid-game points of interest (M2, M3).
- **Risk.** It may feed whoever holds the seam. The fed-tube fight bonus
  favours the defender, so the defender may reap it. The census decides.
- **Ruling.** The food is real food on the plate, drawn as food.
- **Frame cost.** Deaths are already listed by `fight_losses`; deposit cost
  is per death. Quote it.
- **Measure.** Deciding measure: the rate of lead reversal. Count games
  where the sign of (share − 0.5) at 0:30 differs from the bell. Run warden
  against clumsy warden and warden against warden, 16 paired seeds each. Pass:
  reversals rise in the mirror, while warden still beats clumsy on at least
  13 of 16. Idle against idle must stay within 0.45-0.55.

#### 7. Crust as a fall-back (small)

- **Change.** In Bloomfront set `wet` on. Lamp-dried mould sleeps
  (exists). Give sleepers their own fight factor: they lose less to contact
  and kill nothing. Water wakes them (exists). Drying your own losing front
  becomes a retreat that holds ground cheaply but does not eat.
- **Fixes.** No capped loss (P2, P4, P5). It also adds a defensive verb
  other than salt, which helps break the salt opening.
- **Ruling.** Every tool visible: crust is a real state of real walkers.
  Fun over fidelity: crust armour is not biology, and does not need to be.
- **Measure.** Deciding measure: share over the last 60 s. Compare an
  outnumbered hand that crusts its front at contact with one that does not,
  16 paired seeds. Pass: crusting wins at least 13 of 16. Guard: an all-crust hand must
  lose to a greedy hand, so crust is not dominant (D4).

#### 8. Opening triangle census (small, then tunes 3-7)

- **Change.** Write three scripted openings, then let each play the warden's
  mid-game. **Greed:** oats to the far wells first. **Raid:** lanterns and cuts
  on the rival's supply in the first 30 s. **Wall:** salt and Hold. Run a 3 x 3
  round robin in Stage 0's mirror.
- **Fixes.** Tests D4 and E5 directly. A dominant opening shows up as a row
  that beats both others.
- **Ruling.** An outcome is a spread: report each pairing's share spread,
  not only wins.
- **Measure.** Pass: no opening beats both others at p under 0.05 after Holm
  across three pairings, and each wins or ties at least one. Expected today:
  Wall dominates. Proposals 3-7 are the knobs to fix it.

#### 9. Half-minute rounds inside the Feast (small)

- **Change.** Score each 30 s window by the share of food eaten in it. Win a
  window, take a point. Ties go to the running Feast share, as now.
- **Fixes.** A running share banks an early lead. Rounds cap what one window
  can carry, so you "cannot get doubly knocked down" (P2). It also gives a
  ten-second-scale goal (A2).
- **Risk.** If a leader's piles run dry late, rounds swing to the trailer by
  luck. Review 2 saw a crash in the last ten seconds. Watch for perpetual
  comeback (P2).
- **Ruling.** The score is drawn from what was eaten; nothing on the plate
  changes.
- **Measure.** Deciding measure: share of games won by the side behind at
  0:30, warden against warden, 16 paired seeds. Pass: it rises against
  the running share. Warden against clumsy still goes to warden at least 13
  of 16, and idle against idle stays near even.

#### 10. Map shape from fields (small-medium)

- **Change.** Build a few mirrored layouts in `level.rs` from fields that
  exist. Damp channels (`wet` high: mould stays awake, spores can land) and
  dry flats (mould sleeps). Pre-laid salt ridges with gaps as chokepoints. A
  small safe home well, rich far wells, and one contested well per flank so
  there are two fronts.
- **Fixes.** No lanes, chokes or second front (M1, M4). Two fronts make
  attention a resource (A1).
- **Ruling.** Wet and salt are simulation fields, drawn as they are. Any
  dampness drawing must be the field through one ramp.
- **Measure.** Deciding measure: the value of attention, share against
  `HLAG` = 0.5, 1, 2, 4 s, 16 paired seeds. Pass: share falls steadily with
  lag (each step loses on most seeds) rather than a cliff or a flat line.
  Also: control of a contested well changes hands after 1:00 in at least half
  of warden-against-warden games, and the mirror splits 45-55.

#### 11. Telegraphed commitment (medium)

- **Change.** Make big moves ripen in plain sight. A fruiting body grows over
  about 5-8 s before it launches spores (R6 in the depth report). A large oat
  gives off rising scent over seconds. Blooms count down, as the doc says.
- **Fixes.** Nothing to read in the rival (I1, I4) without fog. The rival
  sees a commitment while it can still answer it: light the fruiting body,
  salt the landing ground.
- **Ruling.** Fruiting bodies are real cells (`Ground::Stalk` in R6), so the
  telegraph is the simulation.
- **Measure.** Deciding measure: share for a "reader" hand that answers
  visible fruiting, against a hand that ignores it. Both face a sporing
  hand, 16 paired seeds. Pass: the reader wins at least 13 of 16, and the
  sporing hand gains less against the reader than against idle.

#### 12. Temper triangle (small)

- **Change.** Forage, Hold and Fight as presets of sensor angle, divide and
  the fed-tube fight factor. Tune them so Fight beats Forage at the seam,
  Forage out-eats Hold, and Hold beats Fight on its own tube.
- **Fixes.** A global mode with no counter becomes a dominant switch (D4,
  B2). A triangle makes it a low-attention macro choice (A4, A6).
- **Ruling.** The temper is visible because the tube shape changes: 22.5
  branches, 45 pulls tight (`Reports/mould-review-2026-11-10.md`).
- **Measure.** 3 x 3 tempers on the mirror with idle hands, 16 paired seeds
  per pairing. Pass: each temper beats one and loses to one, Holm across
  three. Then a switching hand must beat every fixed temper.

#### 13. Trunks as investment (medium-large)

- **Change.** Turn on tube memory (`wall_grow`, which needs `sap`). Let the
  keeper pay store to feed a chosen route, which fattens into a trunk that
  carries food faster and regrows after a cut.
- **Fixes.** Investment against army (E1, E5). Food spent on a trunk is
  walkers not grown now, for food that arrives later.
- **Ruling.** The wider trunk is the wall field, already drawn wider.
- **Frame cost.** Flow level 3 costs step time; quote it before anything.
- **Measure.** Timing test from E5. Play an investing hand against an army
  hand and a raiding hand, with bells at 2:30 and 5:00, 16 paired seeds. Pass:
  investing loses to raiding at 2:30 and beats the army hand at 5:00.

#### 14. Fog only as "what your mould felt", and late (medium)

- **Change.** Do not add line-of-sight fog in Stages 0-2 (I2). If the owner
  wants fog later, show the rival's trail only where your walkers' sensors
  sampled it recently, faded through one ramp. The `walkers_seen` field is a
  model of how a faded record is kept and drawn.
- **Fixes.** Scouting (I1) without a painted overlay, if it is wanted at all.
- **Ruling.** The fog is a held quantity, so it obeys "the picture is the
  data". Owner's call, as the doc says.
- **Measure.** Pass only if equal hands still split 45-55 with fog on. A
  scouting hand that spends a tendril must beat a blind hand on 13 of 16.

### 3.3 Ideas noted, not ranked

- **The keeper's lantern as presence.** Tools place only near your own
  lantern, which moves at a finite speed. That makes attention spatial, as
  Tooth and Tail's commander does (A1). Try only after proposal 2.
- **Surround bonus already exists.** Fight odds scale with foe neighbours out
  of 8, so pincers pay, as in Liquid War (C6). A lesson card could teach it.
- **A front's strength is width times supply.** One walker per cell means
  depth adds nothing except feeding. Worth saying in the lesson, and checking
  in a census.

---

## Part 4. Sources

Fetched in this session:

- Leigh Alexander on Sid Meier, GDC 2012: https://gamedeveloper.com/design/gdc-2012-sid-meier-on-how-to-see-games-as-sets-of-interesting-decisions
- "Three perspectives on strategy game design" (Soren Johnson, Keith Burgun, Brad Muir): https://www.gamedeveloper.com/design/three-perspectives-on-strategy-game-design
- Soren Johnson, "Water Finds a Crack": https://www.designer-notes.com/game-developer-column-17-water-finds-a-crack/
- Soren Johnson, OTC Designer Notes #1: Unit Selection: https://www.designer-notes.com/otc-designer-notes-1-unit-selection/
- Alex Wawro on Tooth and Tail (Andy Schatz): https://gamedeveloper.com/design/crafting-i-tooth-tail-i-a-gamepad-rts-that-aims-to-i-hearthstone-starcraft-i-
- Kris Graft on LEADtoFIRE (Andy Schatz): https://gamedeveloper.com/design/can-the-developer-of-i-monaco-i-solve-the-gamepad-rts-riddle-
- Bruno Dias, Vice, on Tooth and Tail: https://vice.com/en/article/zm3vze/tooth-and-tail-is-a-cute-strategy-game-at-war-with-its-own-genre
- Thompson, Blair, Chen, Henrey 2013, PLOS ONE: https://pmc.ncbi.nlm.nih.gov/articles/PMC3776738
- Zero-K, Cold Takes/3: https://zero-k.info/mediawiki/Cold_Takes/3_-_Fight_your_opponent,_not_the_UI
- Zero-K, Balancing Guidelines: https://zero-k.info/mediawiki/Balancing_Guidelines
- Lincoln Carpenter on Frost Giant: https://pcgamer.com/games/rts/stormgate-developers-say-they-want-to-lower-the-rts-skill-floor-not-the-skill-ceiling-its-okay-to-be-a-high-skill-game
- Frost Giant, competitive map design: https://lr.in.psf.lt/r/FrostGiant/comments/ryl1hv/our_thoughts_on_competitive_map_design
- Michael McWhertor on Dustin Browder, GDC 2011: https://kotaku.com/the-sacrifices-of-starcraft-ii-made-in-the-name-of-spor-5777029
- Ben Reeves on Dustin Browder, GDC 2011: https://gameinformer.com/b/news/archive/2011/03/04/creating-starcraft-ii-was-like-creating-a-new-sport.aspx
- Kris Graft interviews Dustin Browder: https://gamedeveloper.com/design/interview-making-the-fun-meant-taking-out-the-fun-in-i-starcraft-ii-i-
- Philippe Cwik on economy and map design: https://gamedeveloper.com/design/the-impact-of-the-economy-in-a-real-time-strategy-game-on-its-multiplayer-level-design
- Brandon Casteel, progression and equilibrium: https://www.gamedeveloper.com/design/the-balance-of-power-progression-and-equilibrium-in-real-time-strategy-games
- Brandon Casteel, dynamic map elements: https://gamedeveloper.com/design/dynamic-map-elements-in-rts-games
- Brandon Casteel, battlefield uncertainty and fog: https://waywardstrategy.com/2015/01/30/battlefield-uncertainty-and-fog-of-war/
- Keith Burgun, fog of war: https://keithburgun.net/fog-of-war-in-push-the-lane-and-strategy-games-generally/
- Keith Burgun, rushdown, economy and defence: https://keithburgun.net/designing-strategy-rushdown-economy-and-defense/
- David Sirlin, slippery slope and perpetual comeback: https://sirlin.net/articles/slippery-slope-and-perpetual-comeback
- David Sirlin, *Playing to Win* ch. 9: https://www.sirlin.net/ptw-book/9-presence-of-mind
- Brendon Chung on Company of Heroes: https://www.blendogames.com/news/post/2017-05-25-on-company-of-heroes/
- Tom Senior on the making of Company of Heroes: https://pcgamer.com/the-making-of-company-of-heroes-design-prototypes-and-the-donkeyschreck
- Brian Reynolds, Rise of Nations postmortem: https://gamedeveloper.com/disciplines/postmortem-big-huge-games-i-rise-of-nations-i-
- Damiano Gerli on Majesty: https://genesistemple.com/majesty-kings-are-the-slaves-of-real-time-strategy
- Interview with Jim DuBois: https://genesistemple.com/interview-with-jim-dubois-on-pc-game-majesty
- Tom Francis, Majesty 2 review: https://www.gamesradar.com/majesty-2-the-fantasy-kingdom-sim
- Brandon Sheffield on Eric Chahi and From Dust: https://gamedeveloper.com/design/the-core-of-i-from-dust-i-
- Josh Bycer on From Dust: https://gamedeveloper.com/design/dusty-an-analysis-of-from-dust
- Josh Bycer interviews Knuckle Cracker: https://www.superjumpmagazine.com/crafting-creeper-world-an-interview-with-knuckle-cracker/
- Liquid War manual page: https://manpages.ubuntu.com/manpages/trusty/man6/liquidwar.6.html
- Liquid War, Wikipedia: https://en.wikipedia.org/wiki/Liquid_War
- Jimmy Maher, "Bullfrog after Populous": https://www.filfre.net/?p=4362
- Peter Molyneux, Black & White postmortem: https://gamedeveloper.com/design/postmortem-lionhead-studios-i-black-white-i-
- Mike Connor, Empires of the Undergrowth postmortem: https://gamedeveloper.com/design/postmortem-how-empires-of-the-undergrowth-came-together-in-over-7-years-of-early-access
- Tynan Sylvester, "The Simulation Dream": https://gamedeveloper.com/design/the-simulation-dream
- Indirect Control pattern: https://math.vu.nl/~eliens/media/pattern-indirectcontrol.html

Fetched but not used (did not hold what was searched for): Mike Rose on
Eufloria (no design detail), Wikipedia on Galcon (mechanics only), Joey
Gibbs on indirect controls (about open-world level design, not RTS).

Not fetched (the shared fetch budget ran out); nothing from them is used:
Sirlin's "Balancing Multiplayer Games" series, Knuckle Cracker's blog on
Creeper World's network, the Dungeon Keeper interview with Peter Molyneux,
Ian Schreiber's feedback-loop lesson (the page needs JavaScript).

Verification pass (2026-11-10) on that list; nothing above changes:
- Sirlin, part 1 (December 2001), fetched:
  https://www.sirlin.net/articles/balancing-multiplayer-games-part-1-definitions .
  "A multiplayer game is balanced if a reasonably large number of options
  available to the player are viable"; if "an expert player can consistently
  beat other experts by just doing one move or one tactic", it is not. This
  agrees with D4 and the salt-wall verdict.
- Dungeon Keeper: Rick Lane, "Hexes, drugs, rock and trolls: The rise and
  fall of Dungeon Keeper", 2024-12-30, fetched:
  https://store.epicgames.com/news/dungeon-keeper-rise-and-fall-interview-peter-molyneux .
  Little on indirect control. Molyneux calls possession "more of a gimmick";
  the slap is credited to lead artist Mark Healey.
- Knuckle Cracker on the network: the page meant was not named. The nearest
  found, https://knucklecracker.com/blog/?p=449 , is a design preview of the
  third game: path-line networks return from CW1, with planned line capacity
  limits and storage pods.
- Schreiber's feedback-loop lesson: still unfetched (budget).

Repo sources: `Reports/rts-design-2026-11-10.md`,
`Reports/mould-review-2-2026-11-10.md`, `Reports/mould-review-2026-11-10.md`,
`Reports/mould-wild-one-2026-11-10.md`, `Reports/dead-ends.md`,
`Reports/mould-game-2026-11-08/brainstorm/research.md`,
`Reports/mould-game-2026-11-08/depth-2026-11-09/complexity.md`,
`src/mould/mod.rs`, `src/mould/level.rs`.

## Verification pass (2026-11-10)

A later pass fetched what this note could not. The shared fetch budget ran
out several times during it, so a page that failed twice stays unverified.

| Item | Outcome | Source |
| --- | --- | --- |
| C1: Indirect Control hurts Predictable Consequences when influence depends on randomness or hidden state | Confirmed (text on the page) | https://math.vu.nl/~eliens/media/pattern-indirectcontrol.html |
| C1: the page comes from Björk and Holopainen's *Patterns in Game Design* | Still unverified; the page and index credit only "(C) Æliens 04/09/2009", and a pattern-card PDF found by search credits no one | https://www.few.vu.nl/~eliens/media/pattern.html |
| Sirlin, "Balancing Multiplayer Games" (nothing used) | Fetched; agrees with D4 | https://www.sirlin.net/articles/balancing-multiplayer-games-part-1-definitions |
| Knuckle Cracker on Creeper World's network (nothing used) | Page not identified; nearest found is a third-game design preview | https://knucklecracker.com/blog/?p=449 |
| Dungeon Keeper interview with Peter Molyneux (nothing used) | Fetched; little on indirect control | https://store.epicgames.com/news/dungeon-keeper-rise-and-fall-interview-peter-molyneux |
| Ian Schreiber's feedback-loop lesson (nothing used) | Still unverified; failed twice (budget) | https://learn.canvas.net/courses/3/modules/items/44541 |

No principle, lesson or ranked item in this note changes.
