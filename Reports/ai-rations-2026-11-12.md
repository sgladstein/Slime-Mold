# The player AI under rations (2026-11-12)

The fight-feel thread's tuning note after #170 (rations, 1 food a second a soldier): the AI's strike bought 30 soldiers whatever the bank could feed, and the army starved on 3 of 16 headless race games (5-20). This is a smoke check of the fix, not a census (the owner's testing rule, 17:26Z). Runner time: about 25 min on one machine.

**The change** (`src/mould/larder_world_ai.rs`, `Brain::think`):

- The brain keeps its income, food banked from harvest a second averaged over 60 s (`View::harvested`).
- It buys a soldier only while the army's rations fit inside 80% of that income, plus the spare bank spread over 120 s (the opening buys). They must also fit inside the spare bank plus half the food on crystals the mould is not near, spread over the next 5 minutes (or to the cap, if sooner). Counted to the cap, it stopped the claim at 8 soldiers with 4,000 banked in the campaign's Landfall, which asks for 12 (CI's `campaign::tests`). The income check also waits for the first minute, until there is an income to go by.
- **Lean:** when the rations outrun the income and the bank holds less than 60 s of them, the strike and the claim wait and every soldier farms.

**Smoke run**: race seeds 5-12, main a1f9865 against the change. Raw output in [ai-rations-2026-11-12/](ai-rations-2026-11-12/). `income.txt` is the first version, without the food-left check. The final version re-ran only strike and claim.

| Hand | Lost games, main | Lost games, change | Score higher-lower (change vs main) |
| --- | --- | --- | --- |
| strike | 2 (overrun on 7, squad wiped on 8) | 1 (squad wiped on 8) | 5-3 |
| claim | 1 (squad wiped on 8) | 0 | 3-5 |
| boom (first version) | 0 | 0 | 3-5, all within 0.08 |

- The strike also wiped the mould once (seed 6, +2.01, against +0.28 on main).
- Armies still go hungry late, when the mould takes the crystals. The strike ended seed 7 with 9 soldiers and seed 11 with 16, and claim seed 8 with 9. The rule only stops buying; nothing disbands a soldier.
- Eight seeds is a smoke check: none of these counts is significant. The claim's 3-5 is a small loss in score, mostly under 0.06 a seed.
- The cost is a sum over the crystals once a think, well inside the think's 180-220 µs.

## Re-check on main ab7a819 (after #174's drain), final version

The final version (food counted 5 minutes ahead, the income check from 1:00) against main ab7a819, race 5-12, strike and claim: `main-ab7a819.txt`, `final-ab7a819.txt`.

| Hand | Lost games, main | Lost games, change | Score higher-lower |
| --- | --- | --- | --- |
| strike | 2 (squad wiped on 7, 8) | 2 (the same) | 2-3, 3 identical |
| claim | 2 (squad wiped on 7, 8) | 1 (8) | 2-2, 4 identical |

On today's main the rule rarely binds. Half the games are identical, and the squads it doesn't save are lost to the fight (#174's drain), not to the rations. It is kept because it is what a player does and costs nothing, not because it measurably helps.
