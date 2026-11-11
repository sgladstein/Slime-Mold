# Larder: game design review (independent agent, 2026-11-11 ~02:00Z)

Asked for by the owner ("What about a more game design and game mechanics
review", 01:55Z). The reviewer was a separate agent briefed as an RTS,
tower-defence and twin-stick designer. It read the report, the mould's module
notes, the knife page, the RTS research page and the skirmish and Two Fevers
pages. It was told about the capped theft measure and the wound bug, but not
given any conclusions. Its report, verbatim:

---

**1. Core loop.** As tested, there is nothing to do moment to moment. The mould reaches the larder in a median 4.2 s and covers the field (test results, "Look first"), so there is no front to see, aim at or steer. What is left is one verb: kill the walkers eating your pile, about every 5 seconds, for 8 minutes. That is mowing a lawn that grows back, not a shooter. Each minute you are meant to choose what to spend on, but the tests show most of the menu does nothing yet, so there is little to choose. A good 5-second beat needs a target that is moving and a cost for being in the wrong place, and fronts that travel would supply both. The report's own fix (a map the mould needs 30 s or more to cross) is the prerequisite for everything else, not a tuning detail.

**2. Dominant strategy.** Holding the ring is the dominant play. Firing through the larder cut theft tenfold (1,220 against 12,273, 16/16), and since the re-fire fix a ring held from the start keeps the mould off entirely on one seed. Check the four strategies against what the mould actually does:
- **Hunter is empty.** Cutting a supply line does not protect a pile, because mould on a pile is fed by that pile (Test 1). Cuts only starve mould with no food of its own, such as fronts and branches, and at a 4 s crossing those do not exist.
- **Baiter is empty for the same reason.** With no travelling front, there is nothing to steer.
- **Farmer loses its harvesters.** They stand on contested piles with no defence.
- **Fortress becomes "ring the larder, sit".** The counter the design relies on is that "loops route round each wall". It does not apply to a ring, which has no gap to route round.

So the design is a ladder topped by the ring, the same shape as Soft in the skirmish (127/128, wiki/rts/skirmish.md) and Trunk in Two Fevers (wiki/rts/two-fevers.md). The owner's ruling that "a ring of fire should stop all mold" is fine, but only if the ring is **expensive and temporary**. It should cost food per cell of perimeter per second, so holding one pile drains more food than that pile earns. The ring then becomes a decision about when to hold, not a resting state.

**3. The mould as an enemy.** It is not readable yet: it arrives everywhere at once. Ideas that stay within the simulation:
- **Soldiers are food.** An engulfed soldier becomes a food pile. Losses then feed the enemy, and you can see why a fight went wrong.
- **Starving walkers fight harder** (`fight_unfed`, already in skirmish.md). A mould cut off from food would lunge instead of fading, which gives cuts a risk.
- **Timed food drops** (the skirmish's drop every 15 s). The scent shows where the next front will go before it moves, which is the "see it coming" beat. It turns the mould's sense of smell into its warning.
- **Fat against lean as the threat level.** Bright, looped mould reads as dangerous and grey branches read as vulnerable, so the picture is the threat display (Test 3: 1 loop against 10, 16/16).

**4. Economy.** Using food for ammo, building and score at once is a good tension, because shipping food lowers your firepower. Three faults:
- **Smell does nothing at this size** (Test 2: first bite 4.6 s against 4.1 s, theft changed by under 1%). The anti-snowball cost does not exist yet.
- **It snowballs:** more food buys a ring, a ring keeps the pile, the pile pays for more ring.
- **It has a dead state:** with no food you cannot shoot, the mould sits on every pile, and the game is over with minutes left. The basic rifle must be free so the player is never locked out.

Bait that feeds the mould when it misses is a good, honest risk. Keep it.

**5. Squad and weapons.**
- **Rifle (clear only) is redundant.** It does nothing on a fed pile (Test 1). Make the burner the default weapon and keep the rifle as a long-range tool for branches.
- **Stake and turret overlap.** Merge them.
- **Missing: a repellent ("sour") field.** The old sensor lamp cut the food join on 5/5 seeds while lit (dead-ends.md). It is the only shaping tool with evidence behind it.
- **Missing: "spoil" (scorched earth).** Destroying a pile you cannot hold is a real denial decision.
- **Upgrades should be sidegrades:** longer wound against faster rate of fire; wide cone against long line; ring radius against cost per second; harvester armour against harvester count. None should be pure damage.

**6. Win/lose and pacing.** Mould that saturates the map in 4 s, with half the food gone by 1:00 (skirmish.md), makes 8 minutes far too flat. Give the round phases:
- 0:00 to 2:00, a land race for piles.
- 2:00 to 5:00, drops that pull fronts across the map.
- 5:00 to 8:00, a rich central drop that both sides must contest.

The score spread (food shipped against food the mould ate) is right. Add one hard loss, the whole squad engulfed, so that rings far from base carry risk.

**7. The three most important changes.**
1. **Make fronts travel:** a mould that needs 30 s or more to reach the larder, by slowing the walkers or enlarging the map. Without it, Hunter, Baiter, smell and telegraphed attacks are all empty.
2. **Price the ring by perimeter-seconds, and make the home pile run dry,** so income has to come from piles you cannot all ring. That turns "ring forever" into a question of where and when.
3. **Give the mould teeth that come from the simulation:** soldiers become food, starving mould fights harder, and timed drops can be smelled before they land. These make it threatening and give it attacks you can see coming.

**Cheapest prototype for the riskiest assumption.** The riskiest assumption is that a priced ring leaves room for several strategies. The harness already exists (`examples/larderprobe.rs`). Add ammo cost per ring cell per second, income from two away piles plus a larder that runs dry, and slowed walkers. Then run scripted hands on 16 paired seeds: Ring-forever, Ring-then-raid, Raid-only, Farmer, random and idle, measuring food shipped. The test passes if Ring-forever is not top on every layout and random loses to at least two plans. If Ring-forever still tops everything, fix the price before writing any game code. Only after that, build the browser v0 with one soldier, the burner, a free rifle and the ring, and ask the owner whether mowing fronts feels like play.
