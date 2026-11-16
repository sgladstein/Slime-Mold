# Noita's spell and wand system: a design reference for verb-based ammo cards

Scope: Noita (Nolla Games). Early access 24 Sep 2019, 1.0 on 15 Oct 2020, "Epilogue" update 30 Mar 2021, "Epilogue 2" update 8 Apr 2024 ([Wikipedia](https://en.wikipedia.org/wiki/Noita_(video_game))). I found no later content update; Wikipedia lists none after April 2024. Numbers below are for the post-Epilogue-2 build unless marked.

**Main catalogue source.** A third-party CSV export of the game's spell table (`data/scripts/gun/gun_actions.lua`): 422 actions with type, mana, uses, spawn tiers, English description and a code summary of each action ([Acidmoon/mc-noita CSV](https://raw.githubusercontent.com/Acidmoon/mc-noita/main/Documents/NoitaMechanics/00_%E6%9D%A5%E6%BA%90%E4%B8%8E%E6%96%B9%E6%B3%95/%E6%B3%95%E6%9C%AF%E5%8A%A8%E4%BD%9C%E7%B4%A2%E5%BC%95_422%E9%A1%B9.csv), cited below as **[CSV]**).
- Cross-check: the official wiki's Cargo "Spells" table also has 422 rows ([wiki Cargo table](https://noita.wiki.gg/wiki/Special:CargoTables/Spells)).
- The CSV includes Epilogue 2 spells (Hookbolt, True Orbit, Giga White Hole, Anti Homing, Holy Lance, Null Shot, Inner Spell, the Sparkly, Lightning and Vacuum bounces), so it postdates April 2024 ([Epilogue 2 devlog](https://nollagames.itch.io/noita/devlog/711710/noita-epilogue-2-update)).
- Not verified: which exact patch it was exported from.

**Unit notes (inferred from the CSV code).**
- `fire_rate_wait` (cast delay) is in frames at 60 fps. Divide By 10's `+80` matches the wiki's "+80 (+1.33 s)" ([Divide By page](https://noita.wiki.gg/wiki/Divide_by_10)).
- Internal projectile damage is the displayed number divided by 25. Divide By 10's internal `-1.5` is the wiki's displayed "-37.5", so Damage Plus's internal `+0.4` displays as +10.

---

## 1. Taxonomy: spell types, counts, and the behaviour families inside them

### Takeaway
Noita has 422 spells in 8 types. About 77% of its 143 projectile modifiers are behaviour components bolted onto any projectile; only about 17% are pure number tweaks and about 6% are purely cosmetic. Its "stronger version of X" cards (Large/Giant/Giga tiers) make up only 14 of 122 projectiles. Distinctness comes from a shared projectile-plus-shot-state core that takes attachable verbs, not from stat ladders.

### Cited Findings
**Counts by type** (code type name in brackets; from [CSV]; 406 of the 422 have a nonzero natural spawn probability):

| Type | Count | Note |
|---|---|---|
| Projectile modifier (MODIFIER) | 143 | 1 never spawns (Rainbow trail) |
| Projectile (PROJECTILE) | 122 | |
| Static projectile (STATIC_PROJECTILE) | 45 | |
| Other (OTHER) | 42 | 13 are ocarina/kantele music notes and 1 is Summon Portal, none of which spawn naturally, leaving 28 "real" Other spells |
| Material (MATERIAL) | 26 | |
| Utility (UTILITY) | 25 | |
| Multicast (DRAW_MANY) | 14 | |
| Passive (PASSIVE) | 5 | |

**Architecture: spells never modify other spells.** Every spell modifies a "shot state" holding damage, spread, cast delay and the projectiles added. When the cast completes, the projectiles fire with every stored modification applied, which is why one modifier affects every projectile multicast with it ([Expert Guide: Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).

**In code, most behaviour modifiers attach an entity file to the projectile.** Examples:
- Homing appends `data/entities/misc/homing.xml` to `c.extra_entities`.
- Spells to Power appends `spells_to_power.xml`.
- Chaos Larpa appends `larpa_chaos.xml`.
- Damage Plus only adds `damage_projectile_add += 0.4` and `fire_rate_wait += 5`.

([CSV] action summaries.) A crude code heuristic (does the action attach an entity or add a projectile?) puts 122 of 143 modifiers in the "attach something" class ([CSV]). That heuristic overcounts, because Damage Plus attaches a cosmetic spark.

**Hand classification of all 143 modifiers** (my grouping of [CSV] descriptions; every modifier is assigned exactly once):

| Family | n | % | Examples (behaviour verb) |
|---|---|---|---|
| Stat/number | 24 | 16.8 | damage+, heavy/light shot, speed up, crit+, spread, lifetime ±, recharge −, knockback, recoil, remove/concentrate explosion, add mana, accelerate/decelerate, bloodlust, mana-to-damage |
| Emitter (spawns sub-projectiles in flight or on an event) | 21 | 14.7 | 8 "X on bounce" (explosion, bubbles, light, plasma beam, self-copy, sparks, lightning, earth-removal); 5 "throwers" that cast fireballs/lightning/tentacles/beams in random or perpendicular directions while flying; 4 orbits (four sawblades/fireballs/nukes/beams circling the shot); clusterbolt (bolts on wall hit), downward bundle (5 bolts when it starts falling), octagonal bundle (8 bolts when slow enough), chaos magic (random spell on hit), firecrackers on expiry |
| Trajectory | 14 | 9.8 | gravity, anti-gravity, slithering, chaotic, ping-pong, avoiding arc (shies from obstacles), floating arc (hovers above ground), fly up/down after a delay, linear arc (cardinal/diagonal only), orbiting arc (orbits its origin), spiral, phasing arc (slow plus short teleports), true orbit (orbits the caster) |
| Role-change | 13 | 9.1 | Nolla (lifetime set to 0), Null shot (huge lifetime, no damage), bounce/remove bounce, piercing (through enemies, hurts caster), drilling (through ground), horizontal path (forced horizontal, more damage), quantum split (3 entangled copies), projectile energy shield (deflects projectiles), damage field (3.5 dmg aura), light, invisible, fizzle |
| On-hit charge/curse | 12 | 8.4 | electric charge, freeze charge, venom curse, 4 weakening curses (+25% of one damage type), necromancy (kills rise as allies), "personal" throwers (the creature hit becomes a fireball thrower, lightning storm, tentacle grower or gravity well) |
| Seeking/steering | 10 | 7.0 | see Section 3 |
| World/material conversion | 10 | 7.0 | water→poison, blood→acid, lava→blood, liquids→gunpowder, toxic→acid, ground→sand, chaotic transmutation, matter eater, earthquake shot, vacuum bounce |
| Status-conditional on the target | 10 | 7.0 | always-crit on burning, wet, oiled or bloody targets; explode (or giant explode) on slimy or drunk targets; charm on toxic-sludge targets; petrify wounded |
| Cosmetic | 8 | 5.6 | 6 colour glimmers, rainbow glimmer, rainbow trail |
| Self-copy (the "larpa" family) | 7 | 4.9 | copies orbit it, a copy on expiry up to 5 times (chain spell), copies in random directions, downward copies, upward copies, a trail of copies, 8 copies on expiry or ground hit |
| Trail | 7 | 4.9 | acid, poison, oil, water, gunpowder, fire particles, burning tail |
| Arc between projectiles | 4 | 2.8 | electric/fire/gunpowder/poison arcs between two projectiles of the same cast |
| Power converters | 2 | 1.4 | Spells to Power (eats your nearby projectiles for damage), Essence to Power (damage per nearby creature) |
| Random | 1 | 0.7 | random modifier |

**Hand classification of all 122 projectiles** (my grouping of [CSV] descriptions):

| Family | n | Verb | Examples |
|---|---|---|---|
| Trigger/timer/expiry carriers | 21 | carries a payload | |
| Physical bomb/object | 15 | summoned physics objects that fall, roll, stick, or wait to be set off | bomb, dynamite, propane tank, bomb cart, boxes, rock, proximity crystal, explosion-triggered crystal, nukes |
| Plain bolts | 14 | straight-flying; differ by speed, damage, size, element | |
| Explosive missiles | 12 | | magic missile, fireball, meteor, death cross, glue ball, cursed sphere, deadly heal |
| Melee/short-range dig | 9 | | chainsaw, luminous drill, digging bolt/blast, glowing lance, air burst, flamethrower, freezing gaze, dark-flame path |
| Summons | 10 | | fish, deer decoy, ducks, worm, tentacle, missile, rock spirit, egg, portal, earthquake |
| Arcing/gravity/slow | 7 | | energy sphere, energy orb, dropper bolt, expanding sphere (damage grows over time), pollen (floats and homes), spiral shot, infestation |
| Bouncing | 6 | | |
| Beams/instant rays | 6 | | |
| Movement/teleport | 6 | | hook pulls caster; teleport bolt moves caster; return after a delay; swap places; pull target to you |
| Clouds/mists | 4 | | |
| Chain/electric | 3 | | chain bolt jumps enemy to enemy; ball lightning |
| Saw/disc | 3 | | |
| Holes | 2 | terrain eaters | black, white |
| Orbit guards | 2 | | |
| Random | 2 | | |

**Stat-tier duplicates exist but are few.** 14 projectiles are named size tiers (Large/Giant/Giga/Big/Intense) of another spell ([CSV]). The 20 named "with trigger/timer" variants are payload-carrying versions of base bolts: same projectile, new verb ([CSV]).

**Static projectiles (45)** ([CSV]):
- Fields: 9 "Circle of" fields (berserk, sheep polymorph, unstable metamorphosis, thunder, freezing stillness, regeneration, teleport, levitation, shielding).
- Projectile-affecting fields: projectiles caught inside become critters, become lightning, or are pulled to the field's centre.
- Vacuum fields that suck up powders, liquids, or creatures and projectiles.
- Weather clouds that rain water, oil, blood, acid or lightning.
- Barriers that harm anything passing, including you.
- 6 element explosions.
- Growing black/white holes.
- Swarm summons (flies, firebugs, wasps).
- Explosive Detonator (sets off all your nearby explosive spells).
- Delayed Spellcast (casts 3 more spells after a delay).
- Random barrages: meteors (Meteorisade) and worms (Matosade).

**Material (26)** ([CSV]):
- 0-mana sprays of water, oil, blood, acid and cement.
- Expanding circles of fire, acid, oil and water.
- "Touch of X": transmutes everything in a radius, "including walls, creatures… and you", into gold, water, oil, alcohol, urine, earth, blood or smoke.
- "Sea of X": a large pool below the caster.

**Utility (25)** ([CSV]):
- Cast-origin changers: cast from a distance; cast "from the closest enemy"; warp a long distance (stopped by walls); cast from where the caster stands.
- Seven "-plicate" formation copies that still cost mana per copy.
- Six "Spells to X" converters that transform every projectile currently in the air into nukes, saws, missiles, crosses, black holes or acid.
- Alternative-currency spells: Blood Magic (−100 mana, costs 4 HP), Gold to Power (5% of gold → damage), Blood to Power (20% HP → damage).
- Wand Refresh (reloads immediately), wall/platform summons, a wand-ghost summon, all-seeing eye.

**Passive (5)**, active while the wand is held: torch, electric torch, energy shield, shield sector, tiny ghost ([CSV]).

**Multicast (14)**: double/triple/quad/octuple; Myriad (draws the whole deck); scatter versions with low accuracy; formations (behind, bifurcated, above-and-below, trifurcated, pentagon, hexagon) ([CSV]).

### Inferences
- Noita's answer to "a stronger explosion that looks the same" is that the stat cards exist but are cheap glue. Damage Plus costs 5 mana and Speed Up 3 ([CSV]). Their value comes from structure that multiplies them (Section 2: multicast, Divide By, Mu), not from being the card you build around.
- Nearly every behaviour modifier is a component that works on any projectile: a homing component, an "emit X on bounce" component, a trail component. 122 projectiles × 143 modifiers gives the combinatorial space; the content count is additive while the play space is multiplicative. This is the structure to borrow.
- Behaviour families cluster around a few event hooks:
  - on fire (trajectory)
  - every frame (trails, throwers, orbits, fields)
  - on bounce
  - on hit or wall
  - on expire
  - on condition of the target (status crits)
  - on world material (conversions)

  An ammo-card system could expose the same hooks for a bullet hitting slime: on enter trail, on kill walker, on expire, while inside a field.
- Even the "tier" projectiles mostly come with a different cost profile (fewer uses, more mana, longer recharge; e.g. Giga Nuke 500 mana and 1 use vs Nuke 200 and 1 use) ([CSV]), so they act as "rare big card" rather than "+1 version".

### Gaps
- No official per-type count was found on the wiki (its pages truncated in fetch); counts come from the third-party CSV and match the wiki's 422-row Cargo table only in total.
- The family classification is mine, built from one-line descriptions, not from playing each spell.

---

## 2. Exact casting mechanics: draw, triggers, multicast, always-cast, Divide By, Greek letters, requirements, wrap, cast delay and recharge

### Takeaway
A wand is a deck. Each cast is a depth-first walk in which each card may draw more cards. Triggers open nested "payload" shot states, so modifiers stay inside their payload. Only the outermost state's cast delay counts, while recharge is global. Copy spells "call" rather than "draw", which bypasses mana and charges. Hard caps are a recursion limit of 2 and a Divide chain of at most 800 copies.

### Cited Findings
**Piles.** Spells live in Deck, Hand and Discard. A spell leaves the Hand only when the cast ends, which prevents double-casting ([Expert Guide: Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).

**Draw.** Attempts the first spell in the Deck. If mana and charges suffice, it moves to Hand and is cast. If not, it is discarded and a "Wand Draw" follows, which cannot wrap ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).

**Spells/Cast.** The wand draws that many spells at the start of each cast. Spells that draw extend the evaluation, so one cast can cover a large tree. Evaluation is depth-first ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).

**Shuffle.** Shuffle wands randomize on the first cast, on reload, on wrap, and on a Wand Refresh cast ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)). The wiki says "No is preferable in almost all circumstances" ([Wands](https://noita.wiki.gg/wiki/Wands)).

**Draw counts, from the code** ([CSV]):
- Every modifier draws 1.
- Double Spell draws 2; Myriad draws `#deck`, the whole remaining deck.
- Requirement spells draw 1 ([Requirement](https://noita.wiki.gg/wiki/Requirement)).

**Triggers, timers, expiration.**
- A trigger casts the next spell at the projectile's location when it hits an enemy or solid object.
- A timer casts the next spell after the projectile has existed for a set time.
- Both pull in the next spell like multicast or modifiers do ([Guide: Wand Mechanics](https://noita.wiki.gg/wiki/Guide:_Wand_Mechanics)).

**In code** ([CSV]):
- Spark Bolt With Trigger is `add_projectile_trigger_hit_world(light_bullet.xml, 1)`, where 1 is the payload draw count.
- "Double trigger" is the same call with 2.
- Spark Bolt With Timer is `add_projectile_trigger_timer(…, 10, 1)`.
- Add Timer is `(target, 20, 1)`. I infer the second argument is frames (10 ≈ 0.17 s), but this is unverified.
- Expiration (death) triggers use `add_projectile_trigger_death`: the payload fires when the projectile expires, as with Black Hole with Death Trigger and Add Expiration Trigger.
- Add Trigger, Add Timer and Add Expiration Trigger ("Other" type) convert any projectile into a carrier for 10, 20 and 20 mana respectively.

**Payload shot states.** Each trigger creates a separate shot state for its payload. Spells drawn inside the payload modify that state, and once its draws run out evaluation returns to the base state ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)). Separating modifications therefore requires separate casts or triggers ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).

**Cast delay vs recharge.**
- Cast delay applies after each cast, and only the base shot state's cast delay counts, so "spells inside triggers don't change the real delay".
- Recharge time can be changed by spells from anywhere.
- At reload, recharge and cast delay are applied together and the longer wins.
- Mana is capped to max at the start of each cast but can exceed the cap during it.
- Recoil resets each cast.

([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw).) Negative cast delay and negative recharge are allowed on wands and offset spell delays ([Wands](https://noita.wiki.gg/wiki/Wands)).

**Cast-delay values in code** ([CSV]):

| Spell | Code effect | Plain meaning |
|---|---|---|
| Spark Bolt | `+3` frames | |
| Bomb | `+100` | about 1.7 s |
| Nuke | sets cast delay to 20 and adds `current_reload_time += 600` | 10 s recharge |
| Chainsaw | `c.fire_rate_wait = 0` (an assignment) and cuts recharge | the community "cast-delay eraser" ([Greek letters guide](https://noita.wiki.gg/wiki/Advanced_Guide:_Introduction_to_Greek_Letters): "Adding a Chainsaw nullifies the cast delay"; [Wikipedia](https://en.wikipedia.org/wiki/Noita_(video_game))) |
| Luminous Drill | `−35` | |

**Wrap and reload.**
- A wrap happens when a spell tries to draw from an empty Deck, or when Wand Refresh is cast. It returns Discard to Deck in the original order (reshuffled on shuffle wands).
- Spells/Cast alone cannot cause a wrap; only a spell's draw can.
- After a wrap, the remaining draws in the cast use the new Deck.
- A reload triggers at the end of a cast if the Deck was empty at any point in it.
- Zero-charge or no-mana spells don't wrap.

([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw).) Spells already in hand are not re-drawn: "If every other spell is already being cast, it does not wrap" ([Guide: Wand Mechanics](https://noita.wiki.gg/wiki/Guide:_Wand_Mechanics)).

**Always Cast.**
- Picked when the wand is generated and cannot be changed. A faint purple glow marks such wands.
- Natural generation excludes Utility, Multicast, Material and Other spells.
- The Always Cast perk can add Utility spells, up to 4 Always Casts total ([Wands](https://noita.wiki.gg/wiki/Wands)).
- Always-cast spells cost no mana and have unlimited uses, which is why an explosive always-cast becomes a "chaingun" ([TV Tropes YMMV](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
- A wand with an Always Cast "casts that spell with every cast" ([wiki Other page](https://noita.wiki.gg/wiki/Other)).

**Draw vs call** ([Calling and Recursion](https://noita.wiki.gg/wiki/Expert_Guide:_Calling_and_Recursion)).
- Drawing moves a card and checks and pays mana and charges.
- Calling performs only the spell's action. Mana and uses are not part of the action; cast delay and recharge are.
- So Gamma can copy a 0-charge Black Hole for Gamma's own 40 mana, though Black Hole's cast delay still applies.

**Recursion limit.**
- A drawn spell is level 0. Calling a recursive spell adds 1, and the limit is 2 in vanilla.
- Recursive spells: all Greek letters, Spell Duplication, Wand Refresh, Random and Copy-Random spells, Giga Nuke, The End of Everything.
- Divide By is not recursive: it inherits its caller's level, but it checks and deducts its target's uses ([Calling and Recursion](https://noita.wiki.gg/wiki/Expert_Guide:_Calling_and_Recursion)).

**Greek letters.** All are unlocked by killing the High Alchemist (Ylialkemisti), and all ignore the copied spell's mana and charges ([Greek Letter Spells](https://noita.wiki.gg/wiki/Greek_Letter)). Mana and cast-delay values are from [CSV]:

| Letter | What it does | Mana | Cast delay |
|---|---|---|---|
| Alpha | copies the first spell in the wand | 40 | +15f |
| Gamma | copies the last spell: the last in the deck, or the last in hand if the deck is empty ([Calling](https://noita.wiki.gg/wiki/Expert_Guide:_Calling_and_Recursion)) | 40 | +15f |
| Tau | copies the next two spells | 90 | +35f |
| Omega | copies every spell: discard, then non-recursive hand spells, then deck, never Wand Refresh ([Calling](https://noita.wiki.gg/wiki/Expert_Guide:_Calling_and_Recursion)) | 320 | +50f |
| Mu | applies every modifier in the wand to a projectile, then draws 1 | 120 | +50f |
| Phi | copies every projectile spell in the wand; copied spells' cast delay and recharge are not applied ([Greek guide](https://noita.wiki.gg/wiki/Advanced_Guide:_Introduction_to_Greek_Letters)) | 120 | +50f |
| Sigma | copies every static projectile in the wand, then draws 1 | 120 | +30f |
| Zeta | copies a random spell from another wand you are holding, then draws 1 | 10 | |

The Gamma/Black Hole rule (copy keeps its cast delay) and the Phi rule (copied spells' cast delay and recharge not applied) come from different wiki guides and concern different letters; they don't conflict.

**Divide By** ([Divide By 10 page](https://noita.wiki.gg/wiki/Divide_by_10)). Each one casts the next spell N times, with a penalty to projectile damage and explosion radius only; other damage types, including heal, are unaffected.

| Spell | Mana | Cast delay | Damage | Explosion radius | Notes |
|---|---|---|---|---|---|
| Divide By 2 | 35 | +20f | −5 | −5 | |
| Divide By 3 | 50 | +35f | −10 | −10 | |
| Divide By 4 | 70 | +50f | −15 | −20 | |
| Divide By 10 | 200 | +80f | −37.5 | −40 | 5 uses, +20 recharge |

- All set a 5° pattern angle.
- Divides can chain: 10 × 10 × 4 × 2 = 800 Spark Bolts, the largest chain the page describes, with an iteration limit beyond that.
- Dividing a modifier multiplies it: Divide By 2 on Damage Plus yields 3 Damage Pluses.
- Unlocks: Divide By 2–4 from the Coral Chest; Divide By 10 by bringing the Curse of Greed to the Avarice Diamond.

**Requirement spells** ([Requirement](https://noita.wiki.gg/wiki/Requirement)). All cost 0 mana and draw 1. Their cost is the wand slot.

| Spell | Skips the next spell when |
|---|---|
| Enemies | fewer than 15 enemies are nearby |
| Projectile Spells | fewer than 20 projectiles are nearby |
| Low Health | HP is above 25% |
| Every Other | every other cast (global counter) |
| Otherwise | an earlier Requirement "succeeds" (else) |
| Endpoint | (block delimiter) |

Block semantics:
- An If scans forward to an Endpoint, another If, or the wand's end, remembering the last Else.
- When its condition calls for a skip, it discards from the next spell up to that Else or the Endpoint.
- When it doesn't skip, it discards from the Else onward.
- Requirements do not nest; an inner If ends the outer scan. The workaround hides the inner If behind Alpha or Gamma.
- An Else or Endpoint in the last slot can cause a wrap, so skipped spells may cast on the wrap.
- The page's own wording is internally inconsistent on Otherwise; treat that as unresolved.
- The wiki does not say when these spells were added.

**Wand Refresh.** Costs 20 mana and resets the wand. Spells to its right are never cast, so they act as storage, unless mana runs out before it ([Greek guide](https://noita.wiki.gg/wiki/Advanced_Guide:_Introduction_to_Greek_Letters)).

**Multicast basics.** Multicast "grabs and casts several following spells at once". Modifiers apply to every projectile in a multicast, but separate clicks and trigger payloads are modified separately ([Guide: Wand Mechanics](https://noita.wiki.gg/wiki/Guide:_Wand_Mechanics)).

**Wiki worked examples** ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)):
- [Double Spell, Damage Plus, Spark Bolt, Spark Bolt] fires two Spark Bolts, both with Damage Plus.
- Spark Bolt With Trigger followed by Damage Plus and Digging Bolt puts Damage Plus and Digging Bolt in the payload, while a later Speed Up modifies the base state.
- A Light spell whose draw finds an empty Deck wraps, draws from the refilled Deck, and the cast ends in a reload.

### Worked example A: nested trigger chain (my derivation from the rules above)
Non-shuffle wand, Spells/Cast 1, deck = [Double Spell, Spark Bolt w/ Trigger, Heavy Shot, Magic Arrow w/ Timer, Bomb, Speed Up, Spark Bolt].

1. The wand's innate draw takes **Double Spell** into base state S0 and opens 2 draws.
2. Draw 1 takes **Spark Bolt w/ Trigger**. It adds the bolt to S0 (+3f cast delay to S0) and opens payload state T1 with 1 draw.
3. T1's draw takes **Heavy Shot** (a modifier in T1). It draws 1 more into T1.
4. That draw takes **Magic Arrow w/ Timer**, added to T1 and modified by Heavy Shot. It opens payload state T2 with 1 draw.
5. T2's draw takes **Bomb**, added to T2 unmodified because Heavy Shot belongs to T1. Bomb's +100f cast delay lands on T2 and is ignored for the wand's real delay.
6. Back in S0, Double Spell's draw 2 takes **Speed Up** (S0), which draws **Spark Bolt** (S0).
7. The deck is now empty, so a reload follows this cast.

On release, two fast Spark Bolts fly (Speed Up applies to both S0 projectiles). The triggered one, on impact, releases a heavy, slow Magic Arrow. That arrow, after its timer, drops a plain Bomb. The real cast delay is only S0's (Spark Bolt ×2 + Speed Up, a few frames), even though a Bomb is in the tree. Mana was paid for all seven cards at draw time. A Nuke in T2 would still add its +600 recharge, because recharge is global ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw); [CSV]).

### Worked example B: wrap-around
Non-shuffle wand, Spells/Cast 1, [Bomb, Double Spell, Spark Bolt].

1. Cast 1: the innate draw takes Bomb. Bomb is fired and discarded.
2. Cast 2: Double Spell draws Spark Bolt, and its second draw finds the deck empty, so it wraps. Discard (Bomb) returns to the deck, and the draw takes Bomb. Spark Bolt and Bomb fire together.
3. The deck was empty during the cast, so the wand reloads afterwards.

This is the pattern of the wiki's Chainsaw example ("a Chainsaw, then a Double Spell with Spark Bolt, wraps back to the Chainsaw") ([Guide: Wand Mechanics](https://noita.wiki.gg/wiki/Guide:_Wand_Mechanics)). Players exploit wraps to get a first-slot cast-delay eraser (Chainsaw) or an Add Mana cast every cycle ([Requirement](https://noita.wiki.gg/wiki/Requirement) examples 3–4).

### Inferences
- **Three separate design levers live in the casting rules:**
  - Ordering: deck, wrap and shuffle.
  - Scoping: payload states contain modifiers.
  - Timing: cast delay is local to the root, recharge is global.

  The "only root cast delay counts" rule is the single biggest power multiplier. Expensive spells hidden in trigger payloads cost mana but no fire-rate. For an auto-firing squad, an equivalent rule ("payload cards don't slow the gun") would make carrier cards the build-around.
- Calling vs drawing creates two currencies of copying. Copies that bypass charges (Greek letters) are run-breaking; copies that respect charges (Divide By) are merely strong. The designers gated the former behind a boss unlock.
- The Requirement spells turn a wand into a conditional program (crowd size, projectile count, HP, alternation). For a game whose target is a population of tens of thousands, "if crowd > N" style cards map directly onto the simulation's state.

### Gaps
- Exact trigger timer durations per spell and the unit of `add_projectile_trigger_timer`'s argument are inferred from code, not confirmed by a wiki page.
- The Divide By iteration-limit formula wasn't retrieved (the page points to an "Expert Guide: Divide By Spells" I did not fetch).
- I found no official statement of whether Noita caps the number of live projectiles.

---

## 3. The homing family

### Takeaway
Noita ships about 10 steering modifiers plus homing projectiles, perks and cast-origin utilities. Each is a different control law, not a different strength:
- magnet force
- proximity snap
- turning without acceleration
- accelerating pursuit
- nearest-visible-target turn
- cursor-guided
- return-to-caster
- teleport-onto-target
- steer toward items
- steer away from foes
- terrain avoidance

### Cited Findings
Descriptions and mana are from [CSV]; the Homing notes are from the [Homing page](https://noita.wiki.gg/wiki/Homing).

| Variant | Description | Mana |
|---|---|---|
| Homing | "accelerate towards your foes". A constant force toward enemies within 150 px; best on slow or stationary projectiles; "rather heavy" cost | 70 |
| Short-range Homing | "flies towards targets when near them" | 40 |
| Rotate Towards Foes | "turn towards your foes" | 40 |
| Accelerative Homing | homes "at an increasing pace" | 60 |
| Auto-Aim | turns toward "the nearest visible enemy" | 25 |
| Aiming Arc | rotates toward where you are aiming (the cursor) | 30 |
| Boomerang | a path "that curves towards you" | 10 |
| Projectile Area Teleport | teleports "right on top of" a valid target in proximity | 60 |
| Wand Homing | accelerates toward wands (items), not enemies | 200 |
| Anti Homing (Epilogue 2) | accelerates away from foes | 1 |
| Avoiding Arc | "shy away from obstacles" (terrain, not enemies) | 0 |

**Homing outside the modifiers:**
- Pollen: a projectile that "homes towards nearby creatures".
- Chain Bolt: "jumps from enemy to enemy".
- Teleporting Cast (utility): "Casts a spell from the closest enemy".
- All from [CSV].

**Perks** ([Homing](https://noita.wiki.gg/wiki/Homing); [Perks](https://noita.wiki.gg/wiki/Perks); [Infinite Lifetime guide](https://noita.wiki.gg/wiki/Guide:_Infinite_Lifetime_Spells)):
- Homing Shots: a weaker version of the spell.
- Boomerang Spells and Bouncing Spells: they also add lifetime.

**Play notes:**
- Disc projectiles can fly through you after a kill, because they re-home on the next enemy.
- Plasma beams have similar problems.
- Homing Rock with Unlimited Spells is "very strong", and Telekinetic Kick can carry it around.
- Homing mists (slime/toxic/spirits) pair with Damage Plus or Heavy Shot.
- A polymorphed player becomes a homing target of their own shots.

([Homing](https://noita.wiki.gg/wiki/Homing).)

**Combo uses** ([TV Tropes YMMV](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)):
- Boomerang + Homing on harmless spells plus a damage aura makes spells orbit the player and strike anything close.
- A homing damage-cloud bug ticked every frame and hunted enemies, and was later nerfed.

**Self-targeting in general.** The community term "Noita'd" covers unfair or unpredictable deaths, citing spells like Giga Disc that turn back toward the caster ([Wikipedia](https://en.wikipedia.org/wiki/Noita_(video_game))).

### Inferences
- **How each might feel in play** (inferred from descriptions; not verified by play):
  - Homing (force) curves lazily and overshoots.
  - Rotate (turn) keeps speed and changes heading.
  - Accelerative starts dumb and ends deadly.
  - Short-range only bends at the last moment.
  - Auto-Aim is a line-of-sight aimbot.
  - Aiming Arc is player-guided.
  - Boomerang returns.
  - Area Teleport skips travel entirely.

  Each gives a visibly different path. That makes them strong "verbs you can see", matching the owner's rule that the picture is the data.
- Homing also creates the self-hazard (re-homing through the caster, homing on a polymorphed caster), so steering cards carry their own risk without extra rules.
- For a slime target made of many walkers, the analogues would be per-target laws: nearest walker, densest cluster, trail-strength gradient, scent-field gradient, away from fire. The "target" can be a field, not an entity. This is an inference, not a Noita mechanic.

### Gaps
- Turn rates, ranges and acceleration values exist only for Homing (150 px). The Homing page gives none for the others, and I did not read the projectile XMLs.
- I did not verify whether Auto-Aim re-targets continuously or only at cast.

---

## 4. Modifiers that change behaviour rather than stats

### Takeaway
Behaviour modifiers fall into a small set of reusable shapes:
- reshape the path (trajectory)
- add an event and what it emits (on bounce, on hit, on expire, while flying)
- change the projectile's role (null, pierce, drill, shield, lifetime 0)
- read the target's state (status crits)
- write to the world (trails, conversions)
- copy the projectile (larpa)

Their chaos comes from stacking: each event hook multiplies the next.

### Cited Findings
**Trajectory** ([CSV]):
- Orbiting arc orbits its point of origin; True Orbit circles the caster "like an orbiting planet".
- Spiral, ping-pong (back and forth), slithering, chaotic.
- Phasing arc: "much slower, but teleport short distances".
- Gravity and anti-gravity; floating arc (hovers above ground); fly up/down after a delay; linear arc (cardinal or diagonal only); horizontal path ("increases its damage").

**Lifetime side-channels on trajectory modifiers** ([Infinite Lifetime guide](https://noita.wiki.gg/wiki/Guide:_Infinite_Lifetime_Spells)). Several trajectory modifiers also add lifetime frames:

| Modifier | Lifetime change (frames) |
|---|---|
| Orbiting Arc | +25 |
| Ping-pong | +25 |
| Spiral | +50 |
| Increase Lifetime | +75 |
| Phasing Arc | +80 |
| True Orbit | +80 |
| Null Shot | +280 |
| Chain Spell | −30 |
| Reduce Lifetime | −42 |

**Event emitters** ([CSV]):
- On bounce: explosive, bubbly, concentrated light, plasma beam, larpa (a copy of itself), sparkly, lightning, vacuum (removes earth).
- On wall hit: clusterbolt.
- On starting to move downward: 5 bolts.
- On moving slowly: 8 bolts.
- On hit: chaos magic (random spell from a limited set).
- On expire: firecrackers; Larpa Explosion (8 copies on expiry or ground hit); Chain Spell (a copy on expiry, up to 5 times).
- While flying: throwers (fireballs, lightning, tentacles or beams in random directions; two-way perpendicular fireballs); orbits of 4 objects around the shot; copy trail.

**Role changes** ([CSV]):
- Nolla sets duration to zero, so a projectile expires at once and fires any expiry payload where it was cast (inference).
- Null Shot: huge lifetime, no damage. It is used as a carrier ([CSV]; [Epilogue 2 devlog](https://nollagames.itch.io/noita/devlog/711710/noita-epilogue-2-update) notes it affects modifiers cast after it).
- Piercing ("through enemies, but harmful to the caster"); Drilling (through ground).
- Projectile Energy Shield (deflects other projectiles).
- Quantum Split ("three projectiles whose existences are entangled").

**Target-state readers** ([CSV]):
- Always crit on burning, wet, oiled or bloody targets.
- Explosion (or giant explosion) on slimy or drunk targets.
- Charm on targets covered in toxic sludge.
- Petrify wounded enemies.
- Four weakening curses, each +25% taken from one damage type.

**Target transformers** ([CSV]). The creature hit becomes a living fireball thrower, a thunderstorm, a tentacle grower or a gravity well that draws projectiles. Necromancy raises kills as allies.

**World writers** ([CSV]):
- Trails of acid, poison, oil, water, gunpowder or fire.
- Arcs of electricity, fire, gunpowder or poison between two projectiles cast together.
- Conversions within range: water→poison, blood→acid, lava→blood, nonmagical liquids→gunpowder, toxic sludge→acid, hard solids→sand, chaotic transmutation.
- Matter eater ("eat the environment as it flies"); earthquake shot.

**Elemental charges** ([CSV]): electric charge and freeze charge release on impact. Frozen targets can't move, and melee hits remove 50% of current HP and kill at ≤25% max HP ([Status Effects](https://noita.wiki.gg/wiki/Status_Effects)). Tentacle (melee) + Freeze Charge is listed as a game breaker ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).

**Projectile-field statics** ([CSV]). Fields that turn projectiles inside them into critters, into lightning, or pull them toward the centre. Vacuum Field "sucks nearby projectiles and creatures into the middle".

**Global converters** ([CSV]). The "Spells to X" utilities transform every projectile currently in the air (into nukes, saws, missiles, death crosses, black holes or acid). "Spells to Power" converts your nearby projectiles into damage on one shot.

### Inferences
- **The status-crit and explode-on-X modifiers form a painter/reader pair.** One card paints a state onto targets or the world (water trail, oil cloud, blood mist), and a second card cashes it in (crit on wet/oiled/bloody, explode on drunk/slimy). Two cheap cards (10–20 mana each, [CSV]) become a multiplicative combo that the player can see, because the material is visible on screen. This is the most transferable structure for a game with scent, sleep gas, contagion and fire fields: "mark" cards that write a field and "detonate" cards that read it.
- Lifetime acts as a hidden shared currency between trajectory modifiers and expiry triggers. Path cards secretly change when expiry payloads fire, which produces combos the designer did not plan (the infinite-lifetime "wisps", Section 7).
- "Throwers", "personal throwers" and orbits turn a single bullet into a moving emitter. In a squad game, bullets that become temporary turrets, or that convert the creature hit into an emitter, would visibly differ from base stats.

### Gaps
- I did not confirm per-modifier numbers (turn speeds, emit rates). The CSV has them in code form for some, but I did not extract them all.

---

## 5. World interactions: spells in the falling-sand world

### Takeaway
Many famous Noita interactions are not numbers at all: oil burns, water stops fire and conducts electricity, fire turns water to steam, explosions crumble terrain into falling sand, transmutation spells rewrite the world (including the caster), and digging spells reshape it. Combos rely on the world as a shared medium that spells write to and read from.

### Cited Findings
**Developer descriptions of the simulation** ([80.lv interview, 5 Apr 2019](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/)):
- "Oil is flammable, but water is not so the fire will spread in the oil, but stop when it hits the water."
- "If the fire tries to ignite the water it will convert into steam instead."
- Burning wood is destroyed after burning long enough.
- After a large enough explosion, "good candidate" pixels become collapsing sand, and rigid bodies recalculate their shape when pixels are destroyed.

**Statuses from material contact** ([Status Effects](https://noita.wiki.gg/wiki/Status_Effects)):
- Wet, Bloody, Slimy and Urine-stained all prevent ignition.
- Electricity does more damage to Wet targets because Stun grants no invincibility frames.
- Oiled makes targets more flammable and keeps them on fire longer.
- On Fire costs 2% max HP/s and makes creatures move 15% faster.
- Slimy slows by 15–40%.
- Berserk (from the Circle of Fervour) doubles damage and blast radius.
- Polymorph turns the target into a sheep.
- Bloody increases crit chance; urine gives +20% crit chance against the stained target.

**Electricity** ([Electricity](https://noita.wiki.gg/wiki/Electricity)):
- The Electricity perk electrifies nearby conductive materials ("metal and liquids around you") every 0.5 s.
- Water Trail and Sea spells spread electricity through water to many enemies.
- Electricity can set off conductive explosives such as propane tanks, and it breaks zapped flasks.
- One example shows a player killed when a toxic sludge barrel exploded.

**Rewriting the world** ([CSV]):
- "Touch of X" transmutes everything in a radius, including walls, creatures and the caster, into gold, water, oil, alcohol, urine, earth, blood or smoke.
- "Sea of X" summons a large pool below the caster.
- Rain clouds drop water, oil, blood or acid; a thundercloud strikes.
- Liquid Detonation turns liquids into gunpowder.
- Ground to Sand makes hard solids fall.

**Digging** ([CSV]; [TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)):
- Digging Bolt and Digging Blast cost 0 mana; Black Hole "eats through all obstacles".
- Luminous Drill + Ping-pong digs nearly anything, including Extremely Dense Rock.

**Holy Mountain brickwork.** It resists digging, and damaging it angers the gods and spawns a hostile guardian ([Holy Mountain](https://noita.wiki.gg/wiki/Holy_Mountain)).

**Transmutation as a weapon** ([Steam discussion, 27 Jun 2021](https://steamcommunity.com/app/881100/discussions/0/3051733836216326106)):
- Players argue that "literally any trigger plus any touch of spell does effectively infinite damage".
- A trigger + Touch of Gold under Gamma (to bypass charges) is called a contender for strongest 3-spell wand.

**Developer anecdote** ([Road to the IGF, 13 Mar 2019](https://www.gamedeveloper.com/game-platforms/road-to-the-igf-nolla-games-i-noita-i-)). Purho recounts an enemy's falling body that "broke an oil lantern", setting him "ablaze". He describes setting an oil-drenched enemy on fire.

### Inferences
- The world doubles as a combo bus: spell A writes material, spell B (or the world itself) reacts. This is why many Noita combos feel discovered rather than designed.
- A slime game already has fields (scent, sleep gas, contagion, fire through plants). Cards that write to those fields, and cards whose effect depends on what is there, would reproduce this structure without new content.
- The same medium is the main self-hazard (fire spread, conductive explosives, self-transmutation). Noita uses the world as its limiter as much as its multiplier.

### Gaps
- No wiki page I read gives the full material-reaction table. I relied on developer quotes and status pages.
- I did not verify freezing of water by ice spells, or acid's interaction specifics.

---

## 6. Luck and chaos: wand generation, shops, perks, rare builds

### Takeaway
Noita controls luck through rolled wand stats (shuffle, capacity, spells per cast, delays) and rare always-cast wands. Holy Mountain shops between biomes (10 spells or 5 wands) and a 3-perk altar with a doubling-cost reroll add more control. Unlock gates keep the most broken spells (Greek letters, Divide By 10) out of early runs. Run-breaking comes from rare co-occurrences: the right wand stats with a copy spell, or a perk like Unlimited Spells.

### Cited Findings
**Wand stat ranges** ([Wands](https://noita.wiki.gg/wiki/Wands)):

| Stat | Range or rule |
|---|---|
| Spells/Cast | 1–26 in practice; guaranteed above 1 if recharge ≥ 1 s |
| Capacity | 1–66; soft-capped at 26, with a 1-in-10,000 chance of being uncapped |
| Speed multiplier | usually 0.8–1.2, rarely 0.5 or 2.0 |
| Cast delay, recharge | may be negative |
| Starter wand mana | 80–130 |

**Starter-wand spells** ([Wands](https://noita.wiki.gg/wiki/Wands)):
- Starter wand: Spark Bolt 50%, otherwise Bouncing Burst, Energy Sphere or Spitter Bolt.
- Starter bomb wand: Bomb 60%.

**Always Cast.** Rare, with a faint purple glow, fixed at generation; the Always Cast perk allows up to 4 ([Wands](https://noita.wiki.gg/wiki/Wands)).

**Shuffle.** Shuffle wands roll higher stats, so the No More Shuffle perk turns them strong ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).

**Holy Mountain** ([Holy Mountain](https://noita.wiki.gg/wiki/Holy_Mountain)):
- A Holy Mountain separates each of the eight main biomes.
- Each holds a shop selling either 10 spells or 5 wands, a 3-perk altar with a reroll machine, a full heal and a Spell Refresher.
- Wand editing is possible only until the ceiling collapses, which happens when you leave.

**Perk rerolls and perks** ([Perks](https://noita.wiki.gg/wiki/Perks)):
- The first reroll costs 200 gold and doubles per reroll in that mountain.
- Spell-altering perks include Always Cast, Unlimited Spells, Concentrated Spells, Bouncing Spells, Boomerang Spells, Homing Shots, Projectile Duplication, Projectile Repulsion Field, Faster Projectiles and Glass Cannon.
- Edit-anywhere perks: Tinker with Wands Everywhere, Wand Experimenter.
- Perk Lottery and Extra Perk.

**Spawn tiers** ([CSV]; tier 10 appears to be a special or endgame tier, my inference):
- Black Hole spawns from tier 0 at probability 0.8; Chainsaw from tier 0.
- Homing from tier 1 at 0.1, then 0.4.
- Nuke at tier 1 (0.3), tiers 5–6 (1.0).
- Greek letters at tiers 5–6 (0.1–0.2) and tier 10 (1.0).
- Divide By 10 and the Requirement spells only at tier 10.

**Unlock gates.**
- Greek letters require killing the High Alchemist ([Greek Letter Spells](https://noita.wiki.gg/wiki/Greek_Letter)).
- Divide By 10 requires the Curse of Greed delivered to the Avarice Diamond; Divide By 2–4 come from the Coral Chest ([Divide By 10](https://noita.wiki.gg/wiki/Divide_by_10)).

**Rare-luck breakers** ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)):
- Unlimited Spells makes limited spells (including Divide By 10) unlimited.
- Perk Lottery lets perks stockpile.
- Draught of Midas appears "roughly once every few thousand games" without external tools.
- Glass Cannon gives 5× damage and 5× explosions at 50 max HP.

**Streaming integration.** In a DiGRA 2023 ethnographic study, Twitch-voted random events let the streamer assemble an "overpowered character" that reached the final boss ([Kultima, Ojanen & Nylund, DiGRA 2023, p. 9](https://dl.digra.org/index.php/dl/article/download/1938/1937)).

**Players hunting "the strongest 2-spell wand"** ([Steam discussion, 2021](https://steamcommunity.com/app/881100/discussions/0/3051733836216326106)):
- Wasps + Homing ("it's so broken").
- Piercing + Thunder Charge.
- Trigger + Touch of Gold.
- Divide By 10 + Triplicate Bolt was debunked, because the −37.5 damage exceeds each pellet's ~4.6.

### Inferences
- Breaks come from co-occurrence luck rather than single lucky drops. A copy spell needs an expensive target, and a wand needs enough capacity and mana. Breaks therefore feel earned and rare, and stay discoverable.
- Unlock gating is a deliberate limiter: the infinite-copy tools appear only after a boss or secret, so a fresh player meets the deck mechanics before the recursion.
- For auto-firing squads, the analogue would be rolled gun frames (capacity, cards per volley, delay) plus a between-wave shop, with card pools keyed to depth.

### Gaps
- The wand-generation algorithm (from memory, a per-tier cost budget that "buys" stats; unverified) was not retrieved.
- Natural always-cast frequency is not given on the wiki.
- I found no data on how often players feel they "got lucky and broke the game"; only anecdotes.
- Holy Mountain prices and tier-by-depth rules were not on the page I read.

---

## 7. Famous absurd combos, their mechanisms, and Noita's limits

### Takeaway
Noita's broken combos come from rule-level multipliers:
- payloads escaping cast delay
- calls bypassing charges and mana
- multiplying divides
- lifetime arithmetic
- free always-casts
- world-scale effects (transmutation, electricity in water)

Base damage is not one of them. Noita limits these with mana, recharge, charges, recursion and iteration caps, unlock gates and pervasive self-damage. It does not cap projectile counts or lag, and some builds can crash the game.

### Cited Findings
**Combos and mechanisms:**

1. **Payload stacking.** Plasma Beam or Plasma Beam Cross delivered by a trigger, with homing, "kills nearly all bosses" ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)). Mechanism: payload cast delay doesn't count toward the wand's delay ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).
2. **Divide By on modifiers or chains.** Two Divide By 10s on Heavy Shot give 100 Heavy Shots ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)). A chain of 10 × 10 × 4 × 2 gives 800 projectiles ([Divide By 10](https://noita.wiki.gg/wiki/Divide_by_10)).
3. **Calls that ignore charges and mana:**
   - Gamma copies a 0-charge Black Hole for 40 mana ([Calling](https://noita.wiki.gg/wiki/Expert_Guide:_Calling_and_Recursion)).
   - Alpha with a 0-charge Giant Magic Missile in slot 1 casts it for 30 mana instead of 200. Gamma→Mu then applies Orbit Larpa, Chaos Larpa and Larpa Explosion; a Chainsaw zeroes cast delay; two Add Mana make the whole thing free ([Greek guide](https://noita.wiki.gg/wiki/Advanced_Guide:_Introduction_to_Greek_Letters)).
   - Greek letters plus Healing Bolt give unlimited healing ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
4. **Always-cast explosives.** They cost no mana and have unlimited uses, giving an explosive "chaingun" that needs protective perks ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
5. **Infinite-lifetime "wisps".**
   - A lifetime of exactly −1 is treated as infinite; below −1 lasts one frame.
   - Players tune lifetime with path modifiers to hit −1, making permanent platforms or immortal Healing Bolts (Piercing + Drilling + Boomerang + Chain Spell).
   - Risks: Piercing turns on friendly fire, payloads are lost on save/reload, and some spells destroy projectiles ([Infinite Lifetime guide](https://noita.wiki.gg/wiki/Guide:_Infinite_Lifetime_Spells)).
6. **Luminous Drill + Ping-pong.** A bouncing beam that kills most enemies and digs nearly anything, limited by mana drain ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
7. **Tentacle + Freeze Charge.** A two-hit kill on nearly anything, because frozen targets lose 50% of current HP per melee hit ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita); [Status Effects](https://noita.wiki.gg/wiki/Status_Effects)).
8. **Orbiting sentry.** Boomerang + Homing on harmless spells, plus a damage aura and field spells ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
9. **Trigger + Touch of X** ("effectively infinite damage"), and **Piercing + Thunder Charge** ([Steam discussion](https://steamcommunity.com/app/881100/discussions/0/3051733836216326106)).
10. **Conditional accumulators.** Requirement: Projectile Spells is used in a Spells to Power build that piles up projectiles and spends them once 20 are nearby ([Requirement](https://noita.wiki.gg/wiki/Requirement), example 5).
11. **Electricity perk in water.** Water-creating spells electrify large groups for "significant gold" with trick-kill perks ([Electricity](https://noita.wiki.gg/wiki/Electricity)).
12. **Spell Duplication.** It duplicates every earlier spell in the cast, including divided ones, and is "likely to crash the game" ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).

**Limits Noita uses:**
- **Mana.** Drawn spells must be affordable; otherwise they are discarded and the next is tried ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).
- **Alternative currencies.** Blood Magic (4 HP), Blood to Power (20% HP), Gold to Power (5% of gold) ([CSV]).
- **Charges.** Spells have limited uses, refilled by the Holy Mountain Spell Refresher ([Holy Mountain](https://noita.wiki.gg/wiki/Holy_Mountain)). Nuke has 1 use and +10 s recharge ([CSV]).
- **Recursion limit of 2** ([Calling](https://noita.wiki.gg/wiki/Expert_Guide:_Calling_and_Recursion)), and the Divide iteration limit of 800 ([Divide By 10](https://noita.wiki.gg/wiki/Divide_by_10)).
- **Self-damage.** Piercing and Bloodlust hurt the caster. Plasma Beam Cross, barriers and Touch of X hit or transmute the caster. Destruction costs HP ([CSV]).
- **Per-copy damage penalties** on Divide By ([Divide By 10](https://noita.wiki.gg/wiki/Divide_by_10)).
- **Fizzle,** a chance to short-circuit ([CSV]).
- **Editing only at Holy Mountains** ([Holy Mountain](https://noita.wiki.gg/wiki/Holy_Mountain)).
- **Unlock gates** (Section 6).

**Limits that fail, or that players exploit:**
- Payloads escape cast delay ([Draw](https://noita.wiki.gg/wiki/Expert_Guide:_Draw)).
- Calls ignore mana and charges ([Greek Letter Spells](https://noita.wiki.gg/wiki/Greek_Letter)).
- Always-casts are free ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
- Crashes from Spell Duplication ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
- Very fast projectiles can phase through terrain ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).
- The developers did nerf some breakers, for example the homing damage-cloud ([TV Tropes](https://www.tvtropes.org/pmwiki/pmwiki.php/YMMV/Noita)).

### Inferences
- Every famous combo pairs one multiplier rule (copy, payload, divide, lifetime, always-cast) with one world or target verb (transmute, freeze-then-melee, beam, electrify water). Stat cards appear only as the thing being multiplied.
- Self-damage is Noita's most important soft limit, and it is visible: the player sees the beam cross, the orbiting saw, the burning oil. It keeps extreme builds from being free without needing hidden caps.
- For a squad-vs-slime game, lag is the realistic failure mode at tens of thousands of agents. Noita's lack of a projectile cap is a warning, not a model. A hard per-volley or per-squad projectile budget is likely needed (inference; not a Noita mechanic).

### Gaps
- I found no designer statement on whether these breaks are intended.
- I found no player-count statistics on how often runs break.

---

## 8. Explosives: explosive spells vs thrown items

### Takeaway
Within spells, Noita splits explosives into distinct verbs:
- physical objects that fall, roll, wait or chain-react (15 projectiles)
- explosive missiles (12)
- instant static explosions with an element (6)
- modifiers that remove, concentrate or add terrain damage to any explosion
- a detonator that sets off all your explosives at once

I found no designer or community discussion of the split between explosive spells and thrown items.

### Cited Findings
**Physical explosive objects** ([CSV]):
- Bomb ("destroys ground very efficiently"), Dynamite, Glitter Bomb ("explodes into volatile fragments"), Holy Bomb and Giga Holy Bomb.
- Propane Tank ("Be careful what you wish for"), Bomb Cart ("self-propelled mine cart"), Explosive Boxes, Rock.
- Unstable Crystal ("explodes when someone comes nearby": a proximity mine).
- Dormant Crystal ("explodes when caught in an explosion": a chain-reaction node).
- Prickly Spore Pod (sticks to a surface, grows, then bursts into spikes).
- Nuke and Giga Nuke.

**Static explosions** ([CSV]): Explosion; Magical Explosion ("doesn't damage the ground"); brimstone (fire), poison, spirits (alcohol) and thunder (electric) explosions.

**Explosion-editing modifiers** ([CSV]):
- Remove Explosion; Concentrated Explosion ("limits the radius … heavily").
- Explosive Projectile ("more destructive to the environment").
- Explosive Bounce; Clusterbolt; Downwards Bolt Bundle.
- Explosive Detonator (static): "All nearby explosive spells cast by you instantly detonate".

**Cast-delay and uses** ([CSV]). Bomb has 3 uses and +100f cast delay; Nuke has 1 use, sets cast delay to 20 and adds +600 recharge. Divide By reduces explosion radius per copy ([Divide By 10](https://noita.wiki.gg/wiki/Divide_by_10)).

**Explosions reshape the world** ([80.lv](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/)). Large explosions turn "good candidate" pixels into collapsing sand.

**Conductive world explosives** ([Electricity](https://noita.wiki.gg/wiki/Electricity)). Electricity can set off conductive explosives such as propane tanks and break flasks. A toxic sludge barrel exploding killed the player in the wiki's example.

### Inferences
- Noita's explosive spells differ by delivery and trigger condition: thrown and rolling, proximity, chain-reaction, sticky-growing, instant at the cast point, remote-detonated. The blast numbers matter far less. That answers the owner's "a stronger explosion that looks the same" directly: make explosive cards differ in when and how they go off, and in what they leave behind (fire, poison, gas, terrain).
- Explosion-editing modifiers let one explosive card be retuned by another card (shrink, remove, make terrain-only), which keeps "bigger boom" out of the base-card list.

### Gaps
- I found no developer or prominent community commentary on spells vs thrown items (potions, flasks, kicked physics objects). Fetches of the Rock Paper Shotgun interview and the 2021 developer Q&A failed: the site was blocked and the fetch budget ran out.
- From memory (unverified): thrown flasks shatter on impact and spill their material, which makes them a "deliver a material" verb parallel to Material spells.

---

## 9. Designer commentary (GDC talk, interviews)

### Takeaway
Nolla describes the wand system as deck-building ("inspired by … Dominion"; the wand "draws up the cards"). They frame the simulation as an "emergence-generating machinery" while warning it "can, in fact, be in the way of more interesting gameplay". The 2019 GDC talk was mainly technical. A dedicated 2020 talk on spell design ("Spellcrafting in Noita", Arvi Teikari) exists, but I could not retrieve its content.

### Cited Findings
**80.lv interview (5 Apr 2019)** ([80.lv](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/)):
- "The wand system allows the players to modify and create their own spells."
- Wands are randomly generated and found around the world, and "in between each area, you get to modify those wands".
- "The system was inspired by the deck building games like Donald X. Vaccarino's Dominion."
- "When the player shoots their wand it draws up the 'cards' from the wand and applies the effects."
- "It sounds easy in theory, but in practice, it creates a lot of interesting situations and combinations."
- "Every pixel in the game follows rather simple rules, but when you combine them together you get surprising and unexpected results."
- On slow natural effects: "when you're playing the game you rarely notice them."

**Road to the IGF (Petri Purho, 13 Mar 2019)** ([Game Developer](https://www.gamedeveloper.com/game-platforms/road-to-the-igf-nolla-games-i-noita-i-)):
- The simulation is "emergence-generating machinery that keeps on generating these interesting situations of play".
- "The simulation can, in fact, be in the way of more interesting gameplay."
- "Technical questions are much easier to answer" than design ones.
- The team worked by "implementing things and then testing them to see what worked and what didn't".
- A simulated-building idea failed: "you very often ended up with just a bunch of rubble at the bottom of the screen".
- Permadeath with procedural generation makes players learn systems "not just through rote memorization".
- The 2012 prototype had a wizard who "would cast spells to manipulate the world"; "The gameplay never quite worked, but it was really fun to mess around in it."

**GDC 2019 "Exploring the Tech and Design of Noita"** (Petri Purho) ([GDC Vault](https://gdcvault.com/play/1025695/Exploring-the-Tech-and-Design); [Game Developer listing](https://gamedeveloper.com/design/video-understanding-the-remarkable-tech-and-design-of-i-noita-i-); [Tildes thread](https://tildes.net/~games/hsh/exploring_the_tech_and_design_of_noita)). The listing covers:
- scaling a falling-sand simulation to "support large continuous worlds"
- integrating destructible rigid-body physics
- tuning physics-based emergent gameplay

The write-ups I found don't summarise any spell-design content.

**Other primary sources that exist but weren't read** ([Official Interviews and Videos](https://noita.wiki.gg/wiki/Official_Interviews_and_Videos)):
- "GN! Recap: Arvi Teikari – Spellcrafting in Noita" (2020, Games Now! lecture series).
- The r/Games AMA (21 Sep 2019).
- Rock Paper Shotgun, "The Noita devs on how to make a fun game when everything is falling" (23 May 2019).
- "Further Questions for Creators with Arvi" (Noita Summit, 30 Mar 2021).

**Other sources:**
- Purho's one-line description of the engine: "based on a falling sand style simulation. Essentially, it's complex cellular automata" ([Wikipedia](https://en.wikipedia.org/wiki/Noita_(video_game))).
- Academic view ([DiGRA 2023](https://dl.digra.org/index.php/dl/article/download/1938/1937)):
  - The authors (not Nolla) write that "Noita is designed to be emergent".
  - "Many of the streamers highlighted the Noita wandcrafting as their favorite feature."
  - "Noita does not explain almost anything to the player."
  - "The main creators do not necessarily even understand what kind of game they have created."
  - Engine work began in 2011 and game design work in January 2013.

### Inferences
- The designers' own frame (deck-building + simple local rules → surprising results) supports the owner's direction. Make cards that are rules acting on a shared state (shot state, world fields), not stat bundles.
- Purho's warning that simulation can get in the way matches the owner's "fun over fidelity" ruling. Noita still cut or tuned simulation-driven ideas that produced rubble rather than play.

### Gaps
- The GDC talk's design section, the 2020 Spellcrafting talk, the 2019 AMA, the RPS interview and the 2021 Q&A doc were not read. The RPS site was blocked, and the session's web-fetch budget ran out before the Q&A doc and AMA could be fetched.
- From memory (unverified): the GDC talk's design part discusses emergent deaths players don't understand and tuning the simulation for play. Treat this as unconfirmed until the video or its transcript is checked.
