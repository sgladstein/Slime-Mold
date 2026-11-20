# Strategy bots and the planet 3 league (2026-11-19)

The owner asked, on 2026-11-18 at 23:58Z, for bots that explore strategies he has not thought of: "It should review how I play but it shouldn't match that exactly". At 23:59Z he added: "a variety of bots that play with different strategies or a bot that can switch/test different strategies". At 14:09Z on 11-19 he narrowed it: "Can we just focus on beating 3". He has given a standing OK for the search to run on his PC "all day and night" (14:08Z).

Page: [Planet 3 Bot League](https://claude.ai/artifact/3pLVvJbt8JxP59oMvWE1Dk). Its source is `page.html`, built by `page.py` from the result files here (`page-template.html` holds the text).

## What was built

- **The strategy bot**, `hand=strat:PLAN` (`src/bin/larder-world/strat.rs`). A plan is one of eight families (bot, rush, econ, turtle, scott, lure, boom, taker) plus dials, for example `econ,go=4,aim=6`. There are 26 dials. Besides the build and shop dials, they include:
  - `aim`: re-aims a second
  - `kite`: steering away from close slime
  - `split` and `group`: the owner's crystal holding
  - `want`: gun cards the shop buys, including ammo as `shell.set`
  - `mop`: hunt the last walkers
- **The utility bot**, `hand=util:WEIGHTS` (`utility.rs`). It scores 41 actions by weights the search can tune.
- **The league**, `league=DEPTH:SEED[:SECS] plan=PLAN` (`league.rs`). It lands a plan on planet DEPTH of run SEED with the earlier planets counted as 2-star wins. The plan's own shop spends the points. Each game prints one `league` line of key=value pairs.
- **The driver**, `league.py`, with the commands run, jobs, table, rank, breed, evolve and worker. `league.py worker --push` runs on the owner's PC from its own worktree (`C:\Users\Scott\Code\slime-league`):
  - It plays the job files in `queue/pc/` first, then the search in `queue/pc-evolve.txt`.
  - It rebuilds when `src/` changes and restarts when `league.py` changes.
  - It waits while the owner's own game is open.
  - It pushes only `pc/`.
- GitHub batches (`.github/workflows/strategy-league.yml`, which runs 4 shards at a time) were used for generation 0 only. The account's 20 runners are shared with CI.

The same plan on the same map, with the same binary on the same machine, always plays the same game. So the maps are the noise, and plans are compared by paired maps with a sign test.

## Results (planet 3, maps 1-8 and the owner's own, run 1795038793)

The score of a game is 1 plus the share of the 12 minutes left for a win, and otherwise 0.9 times the share of the slime's peak that was cleared. 880 planet 3 games, 95 plans. The 28 comparisons on the page are corrected at p < 0.05/28.

- **Mixed-planet search (overnight, 974 games, `pc/gen1-3.jsonl`).** It learned to clear planets 1 and 2 faster with rush plans. Planet 3 wins fell from 6 of 128 to 0 of 128 to 0 of 64, which is what the owner predicted.
- **Faster aim** (`aim=6` against 1). econ was better on 6 maps and worse on 1 (p 0.125), with wins going from 3 to 4. rush was 5 better, 4 worse.
- **Leaving home at 4 soldiers** (`go=4`). lure,aim=6 was better on 8 of 9 (p 0.039), scott 7 better and 2 worse, econ,aim=6 3 better and 5 worse.
- **Kiting** hurt. rush,aim=6 wins fell from 3 to 1 at kite 0.3 and 0.5.
- **The utility bot** won 0 of 192 planet 3 games and was dropped from the search.
- **The owner's crystal holding** (`split=N`, groups of 4 drain and hold one crystal each; his 11-19 playtest) gave no gain for the bot. Split groups alone were mixed: econ,go=4,aim=6 with split=2 or 4 was 3 better and 4 worse. Heave alone was worse on 5 to 6 of 9. Split with Heave was worse on 7 of 9 for all three plans (p 0.07 each).
- **The gun lab's damage decks** (from the gun-builds thread, `want=`, cards=0). econ,go=4,aim=6,cards=0 won 5 of 9 maps on its own. The decks changed that as follows:

  | Deck | Wins | Paired result | Sign p |
  |---|---|---|---|
  | blast.needle | 7 of 9 | better on 7 | 0.18 |
  | seek/wide/blast.needle | 9 of 9 | better on all 9 | 0.004 |
  | wide/rate/blast.needle | 8 of 9 | better on all 9 | 0.004 |

  With seek+wide+blast.needle the bot won in a mean of about 4:50, and won the owner's planet in 4:05 against his 7:21. Its 4 soldiers killed 34,821 walkers in the first 2 minutes; he killed 4,547. lure,go=4,aim=6,cards=0 with the same deck went from 4 to 6 wins, better on 8 of 9 (p 0.039).

  **This deck may break the owner's rule** that no cheap combination should win everything (three cards, cost 8). It is flagged here, not retuned; tuning is Direction's. One caveat: the league puts wanted cards on the shelf, so the bot always gets them, while a real run has to offer them.
- **Mop-up** (`mop=300` and `mop=1000`) changed 2 of 36 games. On the owner's planet, econ,cards=0 cleared all but 2 walkers by 11:00 with or without it. The draining stops, but the last walkers are still not killed (open).
- **Map difficulty.** Without the deck, map 6 was won by 32 of 40 plans and maps 1, 5 and the owner's by none.

## Files

- `strat.rs`, `utility.rs` and `league.rs` in `src/bin/larder-world/`.
- `league.py`, `fetch.sh` (reads LEAGUEROW lines from GitHub job logs), and `page.py` with `page-template.html`.
- `scott-planet3.csv`: the owner's planet 3 every 5 s, from his play log on the `play-logs` branch.
- `queue/pc/NN-*.txt` hold the tests, and `pc/NN-*.jsonl` their results.
- `pc/p3/genN-*`: the planet 3 search.
- `gen0/`: generation 0 on GitHub.
- `pc/gen1-3*`: the mixed-planet night.
- `pc/worker-SCOTT-DESKTOP.log`: the worker's log.
