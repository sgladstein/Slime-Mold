# Gun builds (2026-11-19)

Which card decks remove the most slime, and whether the best follow the owner's Noita rule (no single card wins alone; strong builds come from combinations, often with rarer cards). Research only: no tuning changed.

Page: [Gun Build Findings](https://claude.ai/artifact/5DfhD7vyJPpb1m2LQ1DF6A) (source in `page/`).

## Findings

**Since changed (2026-11-20):** the owner picked "One burst per bullet" (PR #532): a Needle bullet carrying a payload bursts at its first slime hit and flies on plain. These games ran before it, when a needle's payload burst at all three of its pierced hits, which is the likely cause of Needle Blast's lead. Re-run the ranking on the new rule before citing it.

- **Needle Blast (common, cost 4, planet 1) decides the ranking.** Alone it removes +11k / +16k / +13k slime on planets 1 / 3 / 4 against the empty deck (arena, 9 soldiers, 90 s, seeds 1-6, 6/6 seeds each).
- **Top 3 and 4 card guns** (seeds 7-12, `triples-table.md`): seek+wide+blast.needle +13.1k / +21.8k / +18.6k; seek+wide+rate+blast.needle +8.8k / +19.5k / +23.8k; wide+rate+blast.needle (all common) +15.9k / +14.6k / +15.8k. None needs a rare card; every one is built on Needle Blast.
- **Defend** (`defend-table.md`, `arena=9:guard`, the squad holds the slimiest crystal with Guard and Defend): Needle Blast alone is within 2% of the best combo on planet 4 (+24.4k vs +24.9k), all 9 alive against 0 with no cards. Germ Needle (rare) leads on planets 1 and 3. Still, Smother, Blind, Sour, Mist alone do nothing measurable.
- **Full games** (strategy-bot thread, planet 3, 9 maps): no cards 5/9 won, Needle Blast 7/9, seek+wide+blast.needle 9/9 (better on every map, p = 0.004).
- **Not measured by this method:** lures and movers (Bait, Provoke, Pull, Heave), timing and chains (Delay, Trigger, Expire, Tinder+Torch, Shatter, Detonate), turret builds.

## Files

[method](method.md) (method and review fixes), tables: [pairs](pairs-table.md), [3 and 4 cards](triples-table.md), [defend](defend-table.md), `make_jobs.py` (job lists), `run_jobs.ps1` (PC runner), `jobs/`, `raw/` (one `gunlab` line a game), `table.py`, `pairs_table.py`, `defend_table.py` and their tables. Harness: `gunlab=DEPTH:SEED[:SECS] deck=... gun=KIND [arena=N[:focus|:guard]] [tail=T]` in `src/bin/larder-world/runbot.rs`.
