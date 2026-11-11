# Expander review (independent agent, 2026-11-11)

Prompt: [common.txt](common.txt) + [expander.txt](expander.txt). Report as returned:

# Expander review: Larder World roadmap (2026-11-11)

## The three most important points

**1. Make walls the first tool. They are the only tool the data has already shown to work.** In the field skirmish, one salt wall across the leader's road turned 6 of 12 lost games, and a wall every 15 s turned 9 of 12. Both beat a random wall 16/16, and the result held at 12 and 13 of 15 on a symmetric Wild One. The lamp and the cut never turned a game (`Reports/README.md:106-109`). The roadmap lists walls as one item among the player tools (`larder-world-roadmap:45`). Instead they should be W1's first field write: a player-painted `agar` mask, the same channel the rim uses. They are also the best answer to the race opening's snowball problem. Rounds 7 and 7b showed that late food goes to whoever leads at 1:00 (`dead-ends.md:31`), so a comeback has to come from denying food, not from adding it. **Test:** race opening, 16 paired seeds. Compare a trailing player AI that walls the leader's road against one that walls at random and against idle. The measure is how often the 1:00 leader wins (now 80-85%). **Cost:** one mask upload per stroke; zero cost per step.

**2. The world meets the re-test condition for both of M3's rejected mould answers.** Spores failed because "no new food comes with a spore". Their re-test condition is "a spore brings or makes food" (`dead-ends.md:34`). In a world of many crystals, a spore that lands on a far crystal does make food. The crust's re-test condition is that the attack can reach it (`dead-ends.md:35`). In the world it can: a crust left sitting on a drained crystal is reachable and harmless. These give two tiers nearly for free:
- **Sporer:** expands by jumping between crystals instead of by fronts.
- **Sleeper:** a crust that keeps the crystal's leftover charge, so clearing it pays out.

Together they also fix Larder's main flaw, that "the mould does not adapt to the squad" (`wiki/rts/shooter.md:25-26`). **Test:** W1 world, 16 seeds. The measure is how many of the plans in the M3 census (`larder-m3:24-30`) are best on 13 or more seeds, with and without each tier. **Cost:** CPU-side only, on the order of the 0.030 ms the game layer costs now (`larder-m3:42`).

**3. Give the player their own mould species. That is the campaign's spine.** `species_convert` and `species_cycle` (`big.rs:180-220`) already let one species take another's walkers. Make the player's team species 0. You feed it from extractors, and it pushes borders with its own trail. Your soldiers then guard a creature instead of a pile. In the campaign, the long-term goal is to grow that species from world to world. What carries over is its parameters, picked as upgrades (sensor reach, deposit, a converter trait). This turns "upgrades" into changes to the simulation, which the owner can see. It also gives the two soldier teams something to fight over. **Caution:** a 1.5× louder trail wins early but loses walkers by 16 s (`dead-ends.md:32`), and per-mould counter-patterns formed a straight ladder with no cycle (`dead-ends.md:17`). So each upgrade must cost something, not be a plain stat bonus. **Test:** a fed ally against no ally. The measure is the crystal charge the player banks, with paired seeds, and idle and random hands as controls.

## Player tools as field writes, ranked

| Rank | Tool | Field it writes | Tier it answers | How to tell it worked |
|---|---|---|---|---|
| 1 | Wall | `agar` mask | Sprawler (2): cuts thin fronts off from crystals; Rival pair (5): routes borders | Point 1 |
| 2 | Extractor | Drains the crystal's charge into the bank, so its scent fades as it empties | All tiers. It is the tithe rule (`dead-ends.md:21`): money is food the mould doesn't get | Charge the mould eats per crystal, extractor against none, 16 seeds; must beat a randomly placed extractor |
| 3 | False border | A player-written channel in `species_trail`, which other species read through `species_avoid` | Rival pair (5): "lure one into the other" becomes literal, because you push B toward A by painting B's foe smell behind it | Border shift in cells, 16 seeds, compared with a stroke of the same size placed at random |
| 4 | Scent damper | Multiplies the scent read by 0 inside a radius | Hunter (4): hides soldiers and turrets; the shooter report found smell is the deciding factor (`shooter.md:59`) | Soldier deaths per minute against Hunter, damper on and off |
| 5 | Plague shell | `infect` at a point (`big.rs:607`) | Trunk (3). Contagion alone left no lasting mark (`dead-ends.md:28`). The new idea, to re-test only after W0, is that sick walkers pay double upkeep, so the mark lasts as starvation | Trunk walker count 30 s after the shell, against a shell in the wrong place |
| 6 | Scorched ground | Sets `grass` to 0 in a radius | Drifter (1): denies grazing during a retreat | Drifter's charge eaten |

Lures that add scent come last. The `fed_scent` dial pulled the mould off its roads (`dead-ends.md:16`), and placed food lost on both maps tested (`README.md:105`). Bait works only as a decoy, not as steering.

## More mould tiers worth testing

- **Scout swarm:** short-lived explorers (`explore_steps`, `big.rs:228`) that find a base before the main body arrives. The answer is the scent damper. Measure: time to the first bite of the base.
- **Nomad:** grazing herds that leave as a crystal dims. Grass with `graze_regrow` 0 already behaves like this. Note that grass starts at 1 on every cell (`big.rs:194`), so crystals need grass only at their sites.
- **Cyclic triad** (`species_cycle`): three species in rock-paper-scissors. The player tips the cycle by walling off one of them. Check first that the cycle does not converge to one species, using 16 seeds and idle hands.
- **Sleeper and Sporer:** point 2.

## Persistent world and campaign

- **What persists within a world:** walls, extractors, which crystals are drained, and where each species' territory lies. The world grows poorer as it is played, and that forces the player to expand.
- **What changes between worlds:** the mix of mould tiers, the crystal layout, and the player's species parameters (point 3).
- **Fixed-budget rule:** each world adds one new tier, and the player's species carries one trait forward.
- **How a world ends:** the roadmap recommends "when its crystals are gone". Better: when your species holds a target share of the charge that remains. That leaves room for a timing choice, either leave early or hold.

## Openings and the two teams

- **Landing** is a defence puzzle: build walls and extractors before the mould finds you.
- **Race** is the snowball case, and walls are the comeback tool.
- **Rival teams:** add a hand-off so one team's false border can push mould into the other team's base. Larder's reader-against-farmer results suggest the plans could then counter each other.
- **Allied teams:** split the roles, one team builds walls and the other shoots. Test it with two player-AI command sources.
- **Measure for both:** paired seeds, sign test with Holm correction, and a pair that flips winner between fields.

## Costs

- Each per-cell field a sensor reads costs about what grazing costs. The fields that are switched off cost 0.01 ms on the 1070 (`README.md:194`).
- Quote every new read per sensor at 100k walkers, as W0's gate asks.

