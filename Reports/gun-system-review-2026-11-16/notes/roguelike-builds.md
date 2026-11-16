# Gun cards and build variety: Flashes of Chaos and other roguelikes

Research notes, 2026-11-16. Web research only, no play. Sources were opened on
2026-11-16 unless marked otherwise. Several wikis (poewiki.net, poe2wiki.net)
blocked the fetch with a bot wall, and the Steam web API was blocked by this
session's proxy, so some PoE and Flashes of Chaos details are thinner than
planned. Quotes that came through a page summariser are given as the
summariser returned them; treat exact wording as close but not guaranteed.
Game and item names below are there only to identify the structure; the reader
will not copy them.

## 1. Flashes of Chaos: what is documented as of 16 November 2026

### Takeaway
There is still no wiki, Steam guide, patch note or developer write-up of the
card rules ten days after launch. The public record is the same as on
2026-11-12: three guns, cards in slots, cards move between guns, modifiers
stack, one copy of a modifier per shot, and a sticker system that edits a
card's stats. Firing order, slot counts, how a trigger picks its payload, the
card list, the perks and the damage types are still not public.

### Cited Findings
**What the developer and store say**
- Store page: "A fast-paced roguelike top-down shooter about building brutal guns"; "Collect ammunition cards, add modifiers, and load them into your weapon of choice"; three weapons a run (Primary, Secondary, Melee); each weapon has "ammo card slots that define its behavior"; "All ammo cards are interchangeable across weapons"; modifiers "stack and synergize". No slot count is given. — [Steam store](https://store.steampowered.com/app/3399100/Flashes_of_Chaos/)
- Store page on 2026-11-16: 94% positive from 19 user reviews, US$13.99, released 6 Nov 2026, tags include Deckbuilding and Card Game, single-player, Windows only. — [Steam store](https://store.steampowered.com/app/3399100/Flashes_of_Chaos/)
- Press kit: a "card-based weapon system offering nearly unlimited combinations"; factsheet "Released (November 6 2026)". It names no cards, categories, slot counts, triggers or damage types. — [Brainburn press kit](https://brainburnstudio.com/flashes-of-chaos.html)
- Launch news (6 Nov 2026) repeats the store text and quotes the pitch "Mix. Match. Break the game." — [Press Continue](https://www.press-continue.com/news/flashes-of-chaos-launch/)
- The Steam news hub for the game lists no news posts or patch notes. — [Steam news hub](https://store.steampowered.com/news/app/3399100)
- The demo (app 3806250) was released 8 Aug 2025 and its SteamDB record was last updated 15 Nov 2026; SteamDB shows no patch notes on the app page. — [SteamDB demo](https://steamdb.info/app/3806250/)
- Developer, in the demo's Steam forum (found on 2026-11-12, not reopened today): "Not allowing double modifiers in a shot is a design choice", because doubles "would allow builds that crash the game". — [Repo report 2026-11-12](../../flashes-of-chaos-guns-2026-11-12.md), citing [Steam discussions](https://steamcommunity.com/app/3399100/discussions/)

**No wiki or guides yet**
- A search for a Flashes of Chaos wiki returns only the store, demo, SteamDB, press release and press kit pages. — [search results: Steam store](https://store.steampowered.com/app/3399100/Flashes_of_Chaos/), [Games Press](https://www.gamespress.com/en-US/Flashes-of-Chaos-Launches-Today-on-PC)
- The game's Steam forum has 16 active threads; none is about stickers, perks, damage types, a wiki, patch notes or balance by title. The only build thread is the demo-era "not much synergy?" (9 Aug 2025). The most recent are "Feedback" (8 Nov) and "way to disable tutorial?" (9 Nov). — [Steam discussions](https://steamcommunity.com/app/3399100/discussions/)
- Search found a YouTube upload titled "Flashes of Chaos" and the official release-date trailer; the video page could not be fetched, so no description was read. — [YouTube](https://www.youtube.com/watch?v=FqJVye3w8Ws), [trailer](https://www.youtube.com/watch?v=260yx55B0KI)
- Do not confuse it with "My Card is Better Than Your Card", a different card roguelike whose YouTube coverage is about "crafting chaos with stickers". — [YouTube](https://www.youtube.com/watch?v=v4yzJzrGb30)

**What players say (top-rated Steam reviews, read 2026-11-16)**
- Davicro (8.8 h, recommended): praises the "FlexMod/Sticker system"; "Being able to modify the card's stats directly is quite refreshing and opens up a lot of potential"; mentions "machine gun bullets that trigger a rotating laser beam"; "I never really felt like the game was 'solved' with a single build". — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)
- MarioPlaysCoD (1.4 h, not recommended): guns "take modifiers and be modified", but "guns are semi varied but the way I played they all ended up being LMG, sniper, shotguns"; modifier interactions are "annoying when attempting to experiment"; wasted money on "non-interacting tool cards"; boss modifiers "are interesting". — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)
- Thunor Thunorson (2.8 h): "I thought I'd made an OP build but then realised on the next level that the game is balanced around your build being broken." — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)
- RhinOctoClops (0.8 h): an "electrical shock explosion attached to your pistol that has a .2 m/s cool down", hinting that at least some attached effects carry a cooldown. — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)
- Lekon (4.2 h): "a modifier card that gives 'Slow Teleport'" put on a melee weapon gave unexpected sawblade effects (on 11-12 the same review said he "instagibbed myself"). — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)
- Spiral: "noita wand/gun building, complete with sawblades and electricity"; Kinix: "if you like making busted builds, this game is pretty good actually"; VALIS: the gun-building is "pretty straight forward". — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)

**Pre-release article (low reliability)**
- Final Boss (23 Aug 2025, updated 3 Jan 2026) names cards Piercer, Splitter, Ricochet, Cluster, Incendiary and Homing, combos such as "Ricochet + Cluster: bullets bounce between targets and explode on the third hit", modifiers that adjust spread, rate of fire and damage, and aggression that "reportedly" unlocks loot, temporary perks and card draws. The article is hedged throughout, says the developer is not officially named, and still describes a June 2025 beta as upcoming, so it is internally inconsistent. — [Final Boss](https://finalboss.io/flashes-of-chaos-demo-drops-aug-8-heres)

### Inferences
- The one negative review describes the failure the reader's owner is complaining about: when cards mostly change a bullet's stats, every build slides toward one of three stat shapes (fast and many, slow and long, wide and close). The two features players praise most are the ones that are not stat changes: a trigger that fires a different attack ("bullets that trigger a rotating laser beam") and a modifier that changes the attack's verb (teleport on a melee weapon).
- "Non-interacting tool cards" suggests some cards in FoC fail to touch the shared hooks the others use. A card that can't be read by a trigger or modifier is dead weight in a combo game.
- The sticker system (editing one card's numbers) is the FoC feature closest to the owner's "luck is the normal distribution" rule: it lets a found card get a bit better. It is praised, but no reviewer says it is what makes builds absurd; the triggers are.
- The "balanced around your build being broken" remark points at a known roguelike stance: if players don't compete, the enemy curve can assume a broken build (compare Slay the Spire's "decks don't need to be evenly matched" in section 9).

### Gaps
- Firing order, whether a gun cycles its cards or reloads, slot counts and budget, how a trigger picks its payload (next card, all following cards, a fixed slot), whether a trigger has a cooldown, the full card list and categories, the perks, the damage types, and what stickers can change. None is in any source found. Answering it needs the free demo, a long-play video, or the game's Discord (its store link was reported broken on 8 Nov: [Steam discussions](https://steamcommunity.com/app/3399100/discussions/)).
- The "Feedback" thread (8 Nov 2026) and the demo "Neat Ideas" thread (9 replies) were not opened; they may hold developer replies.
- No press reviews were found. No Reddit threads turned up in search.

## 2. How other build-crafting roguelikes structure their systems

### Takeaway
The games known for absurd builds share one structure: a small set of
**carriers** (what the attack is) plus many **orthogonal modifiers** (verbs that
apply to whatever carrier you hold), plus **triggers** that turn one event into
another attack, with a small number of hand-written special cases on top.
Where items mostly add numbers, builds converge.

### Cited Findings
**The Binding of Isaac: carriers plus tear modifiers**
- McMillen (2012 postmortem): "70 percent of the items in Isaac stack, and all the item abilities will affect Isaac in some way"; "Everything about the game was based on complex variables that multiply with each level you pass." Testing could not keep up: "there were bugs that actually took 100,000+ people four weeks to find." — [Game Developer postmortem, 28 Nov 2012](https://www.gamedeveloper.com/business/postmortem-mcmillen-and-himsl-s-i-the-binding-of-isaac-i-)
- Tear effects combine on one tear: piercing passes enemies, spectral passes obstacles, both together pass both; one item gives piercing, then homing and double damage after the tear pierces one enemy; a trinket gives a 10% chance per shot of a piercing, spectral tear. — [Isaac wiki: Tear Effects](https://bindingofisaacrebirth.wiki.gg/wiki/Tear_Effects)
- Some items replace the tear as the attack type (a piercing laser, a charged blood beam, light blasts that can hit the same enemy repeatedly, scythes, homing fetuses); the tear modifiers then apply to the new carrier. — [Isaac wiki: Tear Effects](https://bindingofisaacrebirth.wiki.gg/wiki/Tear_Effects)
- With the beam carrier: any homing item "will cause the beam to make sharp turns and even loops to hit enemies"; a bounce item makes beams "bounce off of walls once, creating a second beam"; a control item turns the beam into "a large ring that is controllable and does not need to be charged"; a familiar makes it shoot "ten beams in all directions"; a bomb modifier makes explosions "release ten beams in all directions". — [Isaac wiki: Brimstone](https://bindingofisaacrebirth.wiki.gg/wiki/Brimstone)
- Carriers have explicit priority when two collide: "Mom's Knife has priority", "The Ludovico Technique has priority", "Tech X has priority"; one beam item's 1.5x bonus is "overridden" by the beam's powered-up version. — [Isaac wiki: Brimstone](https://bindingofisaacrebirth.wiki.gg/wiki/Brimstone)

**Enter the Gungeon: distinct guns plus authored synergies**
- Synergies are specific named combinations that "will produce unique effects when acquired together". Activation is signalled by "a special light blue arrow above the player's head" and the synergy name in the item pop-up; in the menus, selecting an item outlines its partners in light blue. — [Gungeon wiki: Synergies](https://enterthegungeon.wiki.gg/wiki/Synergies)
- Synergy effects are new behaviour, not numbers: a rocket launcher that spawns seven homing flak bullets on impact; a gun that "shoots out rows of fire on the ground"; black holes that fire three tentacles while active; a pistol that fires bouncing shots from its partner gun. — [Gungeon wiki: Synergies](https://enterthegungeon.wiki.gg/wiki/Synergies)
- Beyond authored synergies, the wiki keeps an "Interactions" page for unscripted combination effects, which can be "positive, negative, neutral, or odd". — [Gungeon wiki: Interactions](https://enterthegungeon.wiki.gg/wiki/Interactions)
- A Synergy Chest turns blue (drops an item that completes a synergy) or red (random drop); it has "roughly 5% chance" to appear in reward rooms past the first floor. — [Gungeon wiki: Synergy Chest](https://enterthegungeon.wiki.gg/wiki/Synergy_Chest)

**Risk of Rain 2: on-hit procs, proc coefficient, stacking curves**
- Proc coefficient is "the factor by which an item's effect, trigger chance, duration, or any combination of the three is scaled". A 10% bleed item drops to 6% on a 0.6 nailgun; bleed duration is 3 s × coefficient, 1.8 s on the nailgun and 9 s on a 3.0 railgun. — [RoR2 wiki: Proc Coefficient](https://riskofrain2.wiki.gg/wiki/Proc_Coefficient)
- Fast or multi-hit attacks get low coefficients (0.45 × 32+ hits, 0.5 × 19, 0.2 × 16); slow heavy shots get high ones (railgun 3.0, which "may have a higher proc coefficient to make up for the fact that it doesn't hit as often"); many skills sit at 1.0. Some items and skills have 0.0 and cannot set off effects that "Must be >0 to activate". — [RoR2 wiki: Proc Coefficient](https://riskofrain2.wiki.gg/wiki/Proc_Coefficient)
- A chain-lightning item: 25% on hit ("multiplied by the hit's proc coefficient"), 80% damage to 3 targets (+2 per stack) within 20 m (+2 m per stack); its own lightning has coefficient 0.2. — [RoR2 wiki: Ukulele](https://riskofrain2.wiki.gg/wiki/Ukulele)
- Items can set off other items, so chains form; with low coefficients "The chance gets lower and lower, but it could last forever!" A sticky-bomb item has coefficient 0 so it cannot activate other items. — [Boss Level Gamer](https://bosslevelgamer.com/risk-of-rain-2-proc-coefficient-what-does-proc-mean/)
- Stacking curves differ by item: linear (10% crit per stack, useless past 100%), hyperbolic (15% per stack but 47% at 6 stacks, never 100%), and doubling (one item doubles base damage per stack). Player thread, Jul–Aug 2025. — [Steam discussion](https://steamcommunity.com/app/632360/discussions/0/599658601187019562/)
- A popular balance mod "Limits proc chains based on damage sourcing", capping chains from skills at 3 (later 2) and chains started by items at 1; it refers to the vanilla "proc chain mask". — [Thunderstore: ProcSolver](https://thunderstore.io/c/riskofrain2/p/RiskOfBrainrot/ProcSolver/)
- Hopoo (2019): "All of the items that we end up adding to the game start with a specific purpose in mind"; a flat heal-on-kill "cuts out a lot of the design and player choice", so it became orbs the player decides when to collect; "Giving players a choice is always something we want to weave into our item designs"; balance "comes more from how the items make players feel". — [Game Developer, 14 Aug 2019](https://www.gamedeveloper.com/disciplines/how-moving-from-2d-to-3d-shaped-the-design-of-i-risk-of-rain-2-i-)

**Path of Exile and Path of Exile 2: supports and triggers**
- PoE 1 Cast On Critical Strike: "Must support both an attack skill and a spell skill to work"; "The attack skill will trigger a spell when it critically strikes an enemy"; cooldown 0.15 s; 120% mana multiplier; "Cannot support totems, traps, or mines"; supported skills deal 10–19% less spell damage. Patch 3.5.0 made triggered spells "scale from the triggering attack's attack time" and removed an earlier 80% damage penalty on all triggered skills. — [PoEDB](https://poedb.tw/us/Cast_On_Critical_Strike_Support)
- PoE 2 puts sockets on the skill gem, not the item; each skill takes up to 5 supports; "You can only use 1 copy of each Support gem on your build". — [Maxroll, updated 6 Jan 2025](https://maxroll.gg/poe2/resources/skills-in-path-of-exile-2)
- PoE 2 meta gems trigger socketed skills when an energy meter fills; Cast on Shock "builds energy whenever you shock a foe"; "The energy requirement varies based on the base cast time of the triggered Skill"; meta gems reserve Spirit. — [Maxroll](https://maxroll.gg/poe2/resources/skills-in-path-of-exile-2)
- PoE 2 Cast on Critical: "Gains 1 energy per power of enemies you Critically Hit"; "Triggers all socketed Spells and loses all energy on reaching maximum energy." — [Game8, updated 31 May 2026](https://game8.co/games/Path-of-Exile-2/archives/488224)
- Players have objected to the one-copy rule (forum thread title: "cannot use the same support gem twice in different spells"). — [PoE forum](https://www.pathofexile.com/forum/view-thread/3595302/page/3)

**Vampire Survivors: evolutions, banish**
- An evolution needs a max-level base weapon, its paired passive, and a treasure chest, usually from bosses "at or after the 10-minute mark"; generally "only one weapon can be evolved per chest". A union removes both base weapons, "freeing a weapon slot"; a gift adds a weapon without replacing the base. — [VS wiki: Evolution](https://vampire-survivors.fandom.com/wiki/Evolution)
- Banish removes one offered option "from the pool entirely for the remainder of the run", at the cost of that level's pick. It has finite uses (ten at max rank against 100+ level-up choices), and it unlocks only after the pool has grown: "The player's ability to make use of Banish only scales with the potential need to use it." — [Game Math Done Quick, 5 Dec 2022](https://gmdq.substack.com/p/vampire-survivors-banish-mechanic)

**Hades and Hades II: duo boons**
- Duo boons combine two gods' effects and appear only once you hold specific prerequisite boons from both; 28 exist in Hades; a god's keepsake raises the chance of that god's duos, and a meta-progression talent raises the chance a boon is Legendary or Duo. Example: a collectible left on the floor "strikes nearby foes every 1.5 seconds for 100 Lightning Damage". — [Prima Games](https://primagames.com/tips/hades-what-are-duo-boons-how-get-duo-boons)
- Hades II (Kasavin, Dec 2025): popular boon ideas that were too strong or misplaced were moved rather than cut: "Sometimes ideas for Boons and abilities find new homes in this way!"; "we think it's important not to be too precious about these things when they exist in service of a cohesive whole." — [PC Gamer, 15 Dec 2025](https://www.pcgamer.com/games/roguelike/hades-2-lead-says-they-didnt-nerf-the-games-most-popular-boons-but-rather-found-new-homes-for-them-during-early-access-development)

**Balatro: additive then multiplicative, order matters**
- Score = Chips × Mult. Scoring resolves played cards left to right, then held cards, then jokers left to right. With base 4 mult, a +100 joker then a ×10 joker gives 1040 mult; the other order gives 140. Players' rule: "put xmult jokers to the far right". — [gameplay.tips scoring guide](https://gameplay.tips/guides/balatro-score-calculation-explanatory-guide.html)
- "Balanced totals like 100 chips and 100 mult are much easier to build than lopsided ones like 1000 chips and 10 mult", so two separate axes both need investing in. — [gameplay.tips scoring guide](https://gameplay.tips/guides/balatro-score-calculation-explanatory-guide.html)
- Joker rarity per generated joker: 70% common, 25% uncommon, 5% rare; pools of 61, 64, 20 and 5 (legendary); shop item-type weights Jokers 20, Tarots 4, Planets 4; editions 0.3% Negative, 0.3% Polychrome, 1.4% Holographic, 2% Foil; jokers with unmet preconditions are resampled out. — [gameplay.tips RNG guide](https://gameplay.tips/guides/balatro-rng-mechanics-guide.html)
- LocalThunk (Mar 2024): Jokers began as "a kind of 'passive' item"; "Sometimes I release a card and observe that it's just too good and cannibalises all the adjacent strategies around it"; "almost all balance changes can be made by changing a number (cost, rarity, chip bonus, mult bonus, etc)"; the metastrategy is "mitigating risk and having a build that can't be easily countered while still being powerful enough to win"; "There is a lot of randomness, possibly too much." — [Rogueliker, 7 Mar 2024](https://rogueliker.com/balatro-interview/)

**Ball x Pit: fusion frees a slot**
- "Only max level balls can combine into new balls"; evolution is offered on level-up when the right pair is held, "effectively saving space for another passive or ball". — [PCGamesN](https://www.pcgamesn.com/ball-x-pit/evolutions-tier-list)
- Balls must reach level 3; roughly 90 base pairings across 17 base ball types, plus evolutions of evolutions (for example a fire and iron pair makes a bomb, and bomb plus poison makes a bigger bomb). — [Stevivor](https://stevivor.com/guides/ball-x-pit-ball-evolutions-guide-all-ball-combinations/)
- One early evolution is valued because its bonus balls carry whatever is strong on a single ball: "On-hit abilities are particularly potent." — [PCGamesN](https://www.pcgamesn.com/ball-x-pit/evolutions-tier-list)

**Backpack Battles: space, marked tiles, timers**
- Items act only on grid tiles they mark (star, diamond); the backpack is 7 × 9 plus bags; large weapons cost space that could hold synergy pieces. Items repeat "every N seconds" on cooldowns that Heat speeds and Cold slows. — [dood.gg synergy guide](https://dood.gg/en/backpack-battles/guides/synergy-guide)
- Behaviour-changing synergies: when a marked weapon crits, "its next attack strikes twice"; one item pays out every time marked items trigger five times; a battery charge moves one wired tile at a time. Diminishing stacking is built in: if another cube already speeds an item, a second one's reduction "is cut sharply". — [dood.gg synergy guide](https://dood.gg/en/backpack-battles/guides/synergy-guide)

**Brotato: shop as the build engine**
- Shop of up to 4 items, each slot 65% item / 35% weapon; free unlimited locking that carries to the next wave at the locked price; rerolls rise by floor(0.40 × wave) each. Tier odds rise with wave (tier 4 from wave 8, max 8%) and Luck multiplies them. Item slots have a 5% chance to come from the character's tagged pool; weapon slots are 20% same weapon, 15% same class (higher in shops 1–5). Two identical weapons combine into one a tier higher, up to tier 4. — [Brotato wiki: Shop](https://brotato.wiki.spellsandguns.com/Shop)

**Slay the Spire: archetypes and pity**
- Each reward offers 3 cards with a skip; normal fights roll 60/37/3% common/uncommon/rare, elites 50/40/10, bosses 100% rare; a rarity offset starts at −5%, rises 1% each time a common is rolled (cap +40%) and resets on a rare. — [StS wiki: Card Rewards](https://slay-the-spire.fandom.com/wiki/Card_Rewards)

**Magicraft, Nova Drift (thin)**
- Magicraft wands can have secondary slots that run on "energy" from movement, shots fired or time instead of mana; one relic can convert a whole wand to secondary slots, which then requires firing constantly. Players call the in-game wording confusing and the translation "kind of sketchy". — [Steam discussion, Jan 2025](https://steamcommunity.com/app/2103140/discussions/0/595137806003696674)
- Nova Drift's "Wild Metamorphosis" update added "24 new mods that enhance the game's rogue-like and deck-building aspects"; a later patch changed two mods "as some players are wanting this combo for specific builds". — [itch devlog](https://pixeljam.itch.io/nova-drift/devlog/102320/wild-metamorphosis-is-live-soundtrack-also-released), [itch devlog 0.23.11](https://pixeljam.itch.io/nova-drift/devlog/103398/slight-update-to-wild-mods-version-02311)

**2025–2026 games known for wild builds**
- Megabonk (PC Gamer, 14 Jan 2026, 88): "Truly blessed runs, where you're a near-invincible tornado of death, are just rare enough to make them always feel like an accomplishment"; its "lo-fi look means it can have hundreds of enemies on screen at once". — [PC Gamer](https://pcgamer.com/games/roguelike/megabonk-review)
- Mewgenics (PC Gamer, 6 Feb 2026, 92): a class "adds new randomised abilities"; up to four mana-limited spells and two passives per cat; mutations and disorders are inherited. Emergent combos: an incontinence disorder plus plastic bottles became "an endless source of free healing water"; a cleric's grooming compulsion spread a disease to enemies; a blood-frenzy cat reached "20 turns a round and endless healing". The cost: "it can often feel unfair", and "Fair and precise is not the goal." — [PC Gamer](https://www.pcgamer.com/games/roguelike/mewgenics-review)

### Inferences
- Isaac's structure maps cleanly onto the reader's gun: a gun's first card could be its **carrier** (what a shot physically is), and later cards **verbs** that apply to any carrier. Isaac's "priority" table is the honest cost of that structure: when two carriers meet, one rule must win, and players learn it.
- Gungeon shows the other route: distinct guns, plus a hand-written list of pair effects with a loud on-screen signal. It is content-hungry but guarantees that some pairs are spectacular.
- RoR2's proc coefficient is the most transferable cap for an auto-firing gun with no recharge: a card's chance or strength scales with how heavy the shot is, so a fast gun and a slow gun both get roughly the same value per second from an on-hit card. PoE 3.5 did the same for triggered spells by tying them to the triggering attack's time.
- PoE 2's one-copy-per-build rule and FoC's one-copy-per-shot rule are the same fix: a ban on duplicates forces breadth. PoE 2 shows players resent it when it blocks a build they can picture.
- Balatro's left-to-right additive-then-multiplicative order is the cleanest example of "order matters" producing absurd numbers on purpose. For the reader, order already matters (cards fire in order); the open question is whether any card multiplies the *effect* of the cards before it.
- Ball x Pit and Vampire Survivors make the combined item cheaper than its parts (frees a slot). That is how a lucky find "compounds": the combo pays you room for another piece.

### Gaps
- PoE 1 support-gem rules (tag matching, more vs increased, six-link limit) could not be read: poewiki.net and poe2wiki.net returned bot walls. From memory and unverified: supports only apply to skills that share their tags; most damage supports grant "more" multipliers that multiply each other; items allow at most six linked sockets.
- RoR2 proc chain mask: from memory and unverified, an item already in a chain cannot proc again in that chain, which stops loops. The ProcSolver page confirms a "proc chain mask" exists but not its rule.
- Gungeon's loot weighting toward synergy partners ("Synergy Factor") is listed on the wiki's contents but its text was not retrieved; from memory and unverified, items that complete a synergy are more likely to drop.
- Hades II's duo and "infusion" systems were not documented in any source opened; from memory and unverified, Hades II has boons that grow with how many boons of one element you hold.
- Magicraft's wand rules (slot order, mana, cast loop) and Nova Drift's mod system were not documented by any source found.
- Isaac: from memory and unverified, chance-based tear effects scale with the Luck stat.

## 3. What makes items feel distinct rather than bigger

### Takeaway
Items feel distinct when each one adds a **verb** (pierce, bounce, home, split,
leave fire, trigger another attack, change the carrier) that applies to
everything else the player holds, and when items read and write **shared
hooks** (on hit, on crit, on kill, timers, tags). Stat items converge because
they all push one number.

### Cited Findings
- Isaac's tear effects are verbs with clear jobs (piercing passes enemies, spectral passes obstacles, bouncing returns off walls and goes random off enemies, homing diverts to enemies) and they apply to replacement carriers too: a homing beam loops, a bouncing beam makes a second beam. — [Isaac wiki: Tear Effects](https://bindingofisaacrebirth.wiki.gg/wiki/Tear_Effects), [Isaac wiki: Brimstone](https://bindingofisaacrebirth.wiki.gg/wiki/Brimstone)
- Gungeon's synergy payoffs are almost all new behaviour (impact spawns homing shrapnel, ground fire rows, tentacles from black holes). — [Gungeon wiki: Synergies](https://enterthegungeon.wiki.gg/wiki/Synergies)
- RoR2 on-hit items all read one hook (a hit) and one scale (proc coefficient), so any new attack works with every on-hit item. — [RoR2 wiki: Proc Coefficient](https://riskofrain2.wiki.gg/wiki/Proc_Coefficient)
- PoE's Cast On Critical Strike turns one event (a crit) into another attack; PoE 2 meta gems turn an event count (shocks, crits) into an energy meter that triggers socketed spells. — [PoEDB](https://poedb.tw/us/Cast_On_Critical_Strike_Support), [Maxroll](https://maxroll.gg/poe2/resources/skills-in-path-of-exile-2)
- Backpack Battles items refer to shared categories (weapons, pets, food, potions, spells, fire items), and some items count as several fire items, so tags are the hook. — [dood.gg synergy guide](https://dood.gg/en/backpack-battles/guides/synergy-guide)
- Hopoo rejected a flat stat item because it "cuts out a lot of the design and player choice" and replaced it with something the player acts on. — [Game Developer, 2019](https://www.gamedeveloper.com/disciplines/how-moving-from-2d-to-3d-shaped-the-design-of-i-risk-of-rain-2-i-)
- Patrick Casey (2013) on orthogonality: "a small pool of items have multiple interactions with the beings in the game", interactions should be "logical progressions from one another", "all 'good' items should have some potential disadvantages", and monsters should be able to use items too. — [Game Developer, 11 Sep 2013](https://gamedeveloper.com/design/roguelike-item-orthogonality)
- FoC's critic complained of "non-interacting tool cards" and builds that "all ended up being LMG, sniper, shotguns". — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)

### Inferences
- For the reader's game, the slime's fields are ready-made hooks that no other game on this list has: a card can trigger "on hitting thick slime", "on a walker dying", "when trail drops below X", "when sleep gas covers the bullet". Cards that read those hooks are distinct from each other even with identical numbers.
- Casey's "good items should have downsides" matches the reader's simulation: a card that spikes alarm, flings walkers that seed new growth, or feeds scent is good against one species and harmful against another. That downside is what makes the card a choice rather than an upgrade.
- A useful test for each card: can it be described without a number? If its whole description is "+X% something", it belongs in the gun's rolled stats, not in a card slot.

### Gaps
- No source measured how players perceive distinctness (for example a study of which items players call "build-defining"). This rests on designers' statements and wiki structure.

## 4. Designed combos versus discovered combos

### Takeaway
The successful games do both: a broad emergent layer (every verb applies to
every carrier, every on-hit item reads every hit) produces combos nobody wrote,
and a thin authored layer (named synergies, duos, evolutions, fusions) puts a
guaranteed spectacular payoff on the path and tells the player where to look.

### Cited Findings
- Emergent layer: Isaac stacks 70% of its items and its designer describes "complex variables that multiply". — [Game Developer postmortem](https://www.gamedeveloper.com/business/postmortem-mcmillen-and-himsl-s-i-the-binding-of-isaac-i-)
- Authored layer on top of the emergent one: the Isaac wiki lists per-item "Synergies" and "Interactions", including written special cases with priorities. — [Isaac wiki: Brimstone](https://bindingofisaacrebirth.wiki.gg/wiki/Brimstone)
- Gungeon keeps both explicitly: named "Synergies" and a separate catalogue of unscripted "Interactions" that can be "positive, negative, neutral, or odd". — [Gungeon wiki: Interactions](https://enterthegungeon.wiki.gg/wiki/Interactions)
- Authored pairs that need set-up: Hades duos need prerequisite boons from two gods ([Prima](https://primagames.com/tips/hades-what-are-duo-boons-how-get-duo-boons)); Vampire Survivors evolutions need a max weapon, its passive and a late chest ([VS wiki](https://vampire-survivors.fandom.com/wiki/Evolution)); Ball x Pit needs two max-level balls ([PCGamesN](https://www.pcgamesn.com/ball-x-pit/evolutions-tier-list)).
- Discovered combos are what reviewers of 2026 games quote: Mewgenics' disorder-plus-item healing loop and a cat reaching "20 turns a round". — [PC Gamer](https://www.pcgamer.com/games/roguelike/mewgenics-review)
- LocalThunk: "people are able to come up with strategies and scores I never would be able to do myself"; the game has "synergies that steer you in interesting directions". — [Rogueliker](https://rogueliker.com/balatro-interview/)
- Slay the Spire's team built archetypes on purpose: they added cards "in batches to establish an archetype, then added individual cards to refine each playstyle". — [Game Developer, 27 Feb 2018](https://gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder)

### Inferences
- In the reader's game the simulation is the emergent layer for free: two cards that each write a field (scent, sleep, contagion, fire, no-trail ground) meet in the world, not in code, so combos arise that no card text describes. The authored layer could be a handful of named pairs that the shop or the gun screen flags with a visible signal, Gungeon-style.
- A named pair is also a teaching tool: it shows a new player that cards talk to each other, which is the thing the owner says is missing.

### Gaps
- No source gives the share of combos that designers wrote versus found (for example how many Isaac synergies were hand-coded). That number would help size an authored layer.

## 5. How luck is shaped so absurd builds are rare but reachable

### Takeaway
Games keep the jackpot rare with rarity tiers and late gates, and make it
reachable with player levers: several offers to pick from, skips, rerolls with
rising cost, free locks, limited banishes, pity counters, and partner-weighted
offers. The lever matters more than the odds.

### Cited Findings
- Rarity tiers: Balatro jokers 70/25/5% with a separate legendary pool ([gameplay.tips RNG guide](https://gameplay.tips/guides/balatro-rng-mechanics-guide.html)); Slay the Spire 60/37/3% on normal fights, 10% rare on elites, 100% rare on bosses ([StS wiki](https://slay-the-spire.fandom.com/wiki/Card_Rewards)); Brotato tier odds that open by wave (tier 4 from wave 8, max 8%) and scale with Luck ([Brotato wiki](https://brotato.wiki.spellsandguns.com/Shop)).
- Pity: Slay the Spire's offset starts at −5%, rises 1% per common rolled to +40%, and resets on a rare. — [StS wiki](https://slay-the-spire.fandom.com/wiki/Card_Rewards)
- Choice and skip: StS offers 3 cards with a skip ([StS wiki](https://slay-the-spire.fandom.com/wiki/Card_Rewards)); Brotato offers 4 shop slots ([Brotato wiki](https://brotato.wiki.spellsandguns.com/Shop)).
- Rerolls and locks: Brotato rerolls cost more each time within a shop and reset next shop; locking is free, unlimited and carries over at the locked price. — [Brotato wiki](https://brotato.wiki.spellsandguns.com/Shop)
- Banish: limited uses (ten at max) against 100+ picks; "What makes Banish work is that you can only use it a finite number of times." — [Game Math Done Quick](https://gmdq.substack.com/p/vampire-survivors-banish-mechanic)
- Partner weighting: Brotato weapon slots are 20% "same weapon" and 15% "same class" ([Brotato wiki](https://brotato.wiki.spellsandguns.com/Shop)); Gungeon's Synergy Chest can drop an item that completes a synergy ([Gungeon wiki](https://enterthegungeon.wiki.gg/wiki/Synergy_Chest)); a Hades keepsake raises one god's duo chance ([Prima](https://primagames.com/tips/hades-what-are-duo-boons-how-get-duo-boons)).
- Late gates: most Vampire Survivors evolutions need a chest from a boss at or after 10 minutes. — [VS wiki](https://vampire-survivors.fandom.com/wiki/Evolution)
- Merging duplicates: Brotato combines two identical weapons into one a tier higher. — [Brotato wiki](https://brotato.wiki.spellsandguns.com/Shop)
- Reviewers' framing of the goal: Megabonk's blessed runs "are just rare enough to make them always feel like an accomplishment". — [PC Gamer](https://pcgamer.com/games/roguelike/megabonk-review)
- Hopoo's counterweight: "skill takes a front seat in being successful in Risk of Rain 2"; players can "extend their run by playing smart". — [Game Developer, 2019](https://www.gamedeveloper.com/disciplines/how-moving-from-2d-to-3d-shaped-the-design-of-i-risk-of-rain-2-i-)
- Too much luck has a cost: LocalThunk, "There is a lot of randomness, possibly too much"; Mewgenics "can often feel unfair". — [Rogueliker](https://rogueliker.com/balatro-interview/), [PC Gamer](https://www.pcgamer.com/games/roguelike/mewgenics-review)

### Inferences
- The owner's rule ("luck is the normal distribution itself", nothing labelled lucky) fits the rolled stats on guns and bullets. On its own it cannot produce absurd builds: however rare, a high roll is a bigger number, not a new behaviour. The absurd end has to come from *which cards meet* (combinatorics), and the shop levers above are how a player steers toward a meeting.
- Between eight planets the reader's shop is the only draft point, so the levers that fit are: several cards on offer, a reroll that costs more each time, free locks that carry to the next shop, and a small weight toward cards that interact with what the squad already holds (shown by the same signal that marks a named pair).
- A pity counter on the rarest card class (rises each shop without one, resets on one) keeps "rare but reachable" without labelling anything lucky.

### Gaps
- No source found gives offer counts or reroll curves tuned for an RTS with several squads, where one shop serves many guns.

## 6. How runs avoid converging on one build

### Takeaway
Designers measure convergence (pick rate, presence in winning decks), redesign
the card that "cannibalises adjacent strategies" rather than banning it, and
let the opposition change what wins (boss modifiers, enemy rules). Duplicate
bans and slot limits force breadth.

### Cited Findings
- Slay the Spire's two key metrics: pick rate (too low and the card is "basically not a card in our game at that point") and presence in winning decks (too high signals overpowered). Their metrics grew from "three graphs" to "at least 90". — [Game Developer, 27 Feb 2018](https://gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder)
- StS used an enemy rule to push against one archetype: a boss that gains Strength when the player plays Powers; they tuned it after players with many Powers struggled. — [Game Developer, 2018](https://gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder)
- Balatro: a card can be "just too good and cannibalises all the adjacent strategies around it"; the fix is usually a number (cost, rarity, bonus). — [Rogueliker](https://rogueliker.com/balatro-interview/)
- Hades II moved over-strong boon ideas to other weapons or boons instead of deleting them. — [PC Gamer, Dec 2025](https://www.pcgamer.com/games/roguelike/hades-2-lead-says-they-didnt-nerf-the-games-most-popular-boons-but-rather-found-new-homes-for-them-during-early-access-development)
- One copy only: FoC per shot ([repo report 2026-11-12](../../flashes-of-chaos-guns-2026-11-12.md)); PoE 2 per build ([Maxroll](https://maxroll.gg/poe2/resources/skills-in-path-of-exile-2)).
- Space costs: Backpack Battles' large weapons take room that could hold synergy pieces ([dood.gg](https://dood.gg/en/backpack-battles/guides/synergy-guide)); Ball x Pit and Vampire Survivors unions free slots ([PCGamesN](https://www.pcgamesn.com/ball-x-pit/evolutions-tier-list), [VS wiki](https://vampire-survivors.fandom.com/wiki/Evolution)).
- FoC's critic found convergence on three gun shapes but praised "boss modifiers" as interesting. — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated)
- Balatro's two-axis score (chips and mult) rewards balance: 100 × 100 is easier to build than 1000 × 10. — [gameplay.tips scoring guide](https://gameplay.tips/guides/balatro-score-calculation-explanatory-guide.html)

### Inferences
- The reader has a stronger anti-convergence lever than any game here: the slime species read the fields differently (one floods to alarm, one ignores it, one sleeps until disturbed, one spreads by spores). If the best card set changes with the species of a planet, a run of eight planets cannot settle on one build. This is the "deciding measure" proposed in the 11-12 report (best loadout differs on at least three tiers).
- StS's two metrics are cheap to log from the reader's census and player AI: how often a card is bought when offered, and how often it is in the winning squad's gun. They answer "which card cannibalises its neighbours" without a full balance pass.

### Gaps
- No source quantified how much build diversity players actually see (for example the share of runs won with each archetype) for any game here.

## 7. How designers cap runaway combos without killing the fun

### Takeaway
The caps that keep the fun scale the combo's cost with its power instead of
forbidding it: proc coefficients, cooldowns on triggers, energy meters sized by
the triggered skill, triggered damage tied to the trigger's own speed,
diminishing stacks, chain-length masks, and one-copy rules. Hard bans are
reserved for loops that would end the game or crash it.

### Cited Findings
- Proc coefficient scales an on-hit item's chance, effect or duration by how heavy the hit is; zero-coefficient sources cannot set off effects that need >0. — [RoR2 wiki](https://riskofrain2.wiki.gg/wiki/Proc_Coefficient)
- Chains fade instead of stopping: chain lightning's own coefficient is 0.2 ([RoR2 wiki: Ukulele](https://riskofrain2.wiki.gg/wiki/Ukulele)); "The chance gets lower and lower, but it could last forever!" ([Boss Level Gamer](https://bosslevelgamer.com/risk-of-rain-2-proc-coefficient-what-does-proc-mean/)). A balance mod caps chains at 3 (later 2) links from skills ([ProcSolver](https://thunderstore.io/c/riskofrain2/p/RiskOfBrainrot/ProcSolver/)).
- Trigger cooldown and cost: PoE Cast On Critical Strike has a 0.15 s cooldown, a 120% mana multiplier and 10–19% less spell damage; since 3.5.0 triggered spells "scale from the triggering attack's attack time". — [PoEDB](https://poedb.tw/us/Cast_On_Critical_Strike_Support)
- Energy sized by payload: PoE 2 meta gems need more energy for triggered skills with longer base cast times. — [Maxroll](https://maxroll.gg/poe2/resources/skills-in-path-of-exile-2)
- Diminishing and capped stacks: hyperbolic stacking never reaches 100%; linear crit stops at 100% ([Steam discussion](https://steamcommunity.com/app/632360/discussions/0/599658601187019562/)); in Backpack Battles a second speed source on the same item "is cut sharply" ([dood.gg](https://dood.gg/en/backpack-battles/guides/synergy-guide)).
- One-copy rules: FoC bans doubles in one shot because doubles "would allow builds that crash the game" ([repo report 2026-11-12](../../flashes-of-chaos-guns-2026-11-12.md)); PoE 2 allows one copy of each support per build ([Maxroll](https://maxroll.gg/poe2/resources/skills-in-path-of-exile-2)).
- Targeted restriction rather than removal: StS's Dual Wield enabled "infinite" combos, so it was restricted to one card type; "Going infinite is the number one thing we try to make really rare." — [Game Developer, 2018](https://gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder)
- Priority instead of stacking: Isaac resolves clashing carriers with a written priority (one carrier wins). — [Isaac wiki: Brimstone](https://bindingofisaacrebirth.wiki.gg/wiki/Brimstone)
- Raising the bar instead of capping: one FoC reviewer says the game "is balanced around your build being broken"; StS notes PvE decks "don't need to be evenly matched". — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated), [Game Developer, 2018](https://gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder)

### Inferences
- The reader's guns fire forever with no recharge, so the natural budget for a trigger is fire rate: a card that releases a second effect could add to the gun's time between shots (PoE's "scale from the triggering attack's time" in reverse), or the released effect could inherit a coefficient below 1 so each link is weaker (RoR2).
- A chain-depth cap of 2–3 links (the 11-12 report's proposal, and ProcSolver's number) plus a per-card one-copy-per-shot rule (FoC) bounds the per-frame cost the GTX 1070 has to pay, while leaving room for every card to appear once in a chain.
- Because the slime pushes back (it outgrows bullets, panics, condenses), "the next planet is balanced around your build being broken" may be the cheapest cap of all: raise the slime's growth, not the cards' limits.

### Gaps
- No source gives frame-cost numbers for live-projectile caps in these games. From memory and unverified: Noita builds that multiply projectiles can lag or crash the game, which matches FoC's stated reason for its doubles ban.

## 8. How chaos stays readable

### Takeaway
Readable chaos comes from a loud, consistent signal when a combo is live, from
the build being visible on the character or weapon, from art simple enough that
hundreds of effects don't blur, and from fixed resolution rules the player can
learn (order, priority).

### Cited Findings
- Gungeon signals a live synergy with "a special light blue arrow above the player's head" and the synergy name, and outlines partner items in light blue in the menus. — [Gungeon wiki: Synergies](https://enterthegungeon.wiki.gg/wiki/Synergies)
- RoR2 shows items on the character model: "It gives you a true sense of progression to see your character standing at the Obelisk decked out in random items." — [Game Developer, 2019](https://www.gamedeveloper.com/disciplines/how-moving-from-2d-to-3d-shaped-the-design-of-i-risk-of-rain-2-i-)
- Isaac's items were "random, but still designed" so the character visibly grows. — [Game Developer postmortem](https://www.gamedeveloper.com/business/postmortem-mcmillen-and-himsl-s-i-the-binding-of-isaac-i-)
- Megabonk's "lo-fi look means it can have hundreds of enemies on screen at once". — [PC Gamer](https://pcgamer.com/games/roguelike/megabonk-review)
- Backpack Battles draws each item's reach as symbols on the grid before you place it. — [dood.gg](https://dood.gg/en/backpack-battles/guides/synergy-guide)
- Balatro resolves strictly left to right, so players can reason about order ("put xmult jokers to the far right"). — [gameplay.tips](https://gameplay.tips/guides/balatro-score-calculation-explanatory-guide.html)
- Failure case: FoC players find modifier interactions "annoying when attempting to experiment", and on 11-12 players said walls are hard to tell from the floor. — [Steam reviews](https://steamcommunity.com/app/3399100/reviews/?browsefilter=toprated), [repo report 2026-11-12](../../flashes-of-chaos-guns-2026-11-12.md)

### Inferences
- The owner's rule that the picture is the data is a readability asset here: if each card writes a different field (sleep, contagion, fire, no-trail ground, scent), the mark it leaves in the slime is its signature, and a combo reads as two signatures meeting. The Gungeon-style signal can be a property of the world (for example a combo's field visibly reacting), not a painted icon.
- At RTS zoom with several auto-firing squads, the player reads the outcome, not each shot. Isaac's and RoR2's "build visible on the body" suggests showing a squad's gun card set on the squad itself or its selection panel.

### Gaps
- No designer source on readability at RTS scale (many units firing at once) was found; every example is a single-hero game.

## 9. Designer talks and writing

### Takeaway
The written record is thinner than the talks' reputations. Usable primary
sources: McMillen's 2012 postmortem, Hopoo's 2019 interview, Mega Crit's GDC
material (2018–2019), LocalThunk's 2024 interviews, Kasavin's 2025 remarks and
Casey's 2013 orthogonality essay. GMTK's synergy video and Grinding Gear's
support-gem writing could not be read.

### Cited Findings
- McMillen: "70 percent of the items in Isaac stack"; variables "multiply with each level you pass" ([Game Developer, Nov 2012](https://www.gamedeveloper.com/business/postmortem-mcmillen-and-himsl-s-i-the-binding-of-isaac-i-)); "I want there to be a reward in the game for everything you do" ([GamesRadar, 24 Jul 2015](https://gamesradar.com/i-did-make-it-so-it-goes-forever-edmund-mcmillen-binding-of-isaac)).
- Hopoo: every item starts "with a specific purpose in mind"; choice is woven into item designs; balance from "how the items make players feel"; items displayed on the body. — [Game Developer, 14 Aug 2019](https://www.gamedeveloper.com/disciplines/how-moving-from-2d-to-3d-shaped-the-design-of-i-risk-of-rain-2-i-)
- Mega Crit (Giovannetti and Yano): a metrics server tracking every decision from early prototypes; pick rate and win-deck presence; archetypes added in batches; "Going infinite is the number one thing we try to make really rare"; "a single well-reasoned post is a lot more useful than lots of players saying 'nerf this'". — [Game Developer, 27 Feb 2018](https://gamedeveloper.com/design/how-i-slay-the-spire-i-s-devs-use-data-to-balance-their-roguelike-deck-builder). The GDC 2019 talk "Slay the Spire: Metrics Driven Design and Balance" is announced at [GDC news](https://gdconf.com/news/learn-slay-spires-successful-metrics-driven-approach-game-balancing-gdc-2019); its text was not read.
- LocalThunk: quotes in sections 2 and 6. — [Rogueliker, 7 Mar 2024](https://rogueliker.com/balatro-interview/). He made 150 jokers instead of 120 rather than correct a publisher's number (headline only). — [PC Gamer](https://pcgamer.com/games/card-games/balatro-was-only-supposed-to-have-120-jokers-but-instead-of-correcting-a-publisher-mistake-localthunk-just-made-30-more-of-them)
- Kasavin: boon ideas "find new homes"; don't be "too precious". — [PC Gamer, 15 Dec 2025](https://www.pcgamer.com/games/roguelike/hades-2-lead-says-they-didnt-nerf-the-games-most-popular-boons-but-rather-found-new-homes-for-them-during-early-access-development)
- Casey on orthogonality: section 3. — [Game Developer, 11 Sep 2013](https://gamedeveloper.com/design/roguelike-item-orthogonality)
- A GMTK video titled "Why Synergies are the Secret to Slay the Spire's Fun" exists (subtitle page only). — [Amara](https://amara.org/v/C3BET)

### Inferences
- Across these sources the recurring design claims are: give every item a job and a choice (Hopoo, Casey), let numbers multiply but watch for loops (McMillen, Mega Crit), fix a dominant card by changing a number or moving its idea rather than deleting it (LocalThunk, Kasavin), and measure what players pick and what wins (Mega Crit).

### Gaps
- GMTK's synergy video content could not be read (subtitle page needs a login; YouTube fetch failed). From memory and unverified, it argues that synergy (cards that are weak alone and strong together) is the core pleasure of deck-builders.
- Grinding Gear Games' own writing on support gems and the PoE 2 one-copy rule was not found; the PoE wikis were behind a bot wall.
- No Hopoo talk on RoR2 proc design or McMillen talk specifically on synergies was found in text.
