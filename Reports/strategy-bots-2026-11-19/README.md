# Strategy bots and the planet 3 league (2026-11-19)

The owner asked, on 2026-11-18 at 23:58Z, for bots that explore strategies he has not thought of: "It should review how I play but it shouldn't match that exactly". At 23:59Z he added: "a variety of bots that play with different strategies or a bot that can switch/test different strategies". At 14:09Z on 11-19 he narrowed it: "Can we just focus on beating 3". He has given a standing OK for the search to run on his PC "all day and night" (14:08Z).

The code and the results to 05:44Z on 2026-11-20 merged as PR #529. The PC worker keeps pushing results to `claude/project-thread-8grg6d`; they reach main in a later docs-only pull request. A branch whose tip is in main is deleted by `branches.yml`, so the branch always carries at least one commit main lacks while the worker runs.

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
  - It pushes only `pc/`, at most once an hour or when a job file finishes.
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
- **Mop-up** (`mop=300` and `mop=1000`) changed 2 of 36 games. On the owner's planet, econ,cards=0 cleared all but 2 walkers by 11:00 with or without it. The `rest=` field (job 08-rest) showed why: the last walkers were set slime that shots passed through. Main's 5be6b0b (shots stop at set slime) fixed that.
- **Map difficulty.** Without the deck, map 6 was won by 32 of 40 plans and maps 1, 5 and the owner's by none.

## After the two fixes of 2026-11-20

Two changes on main changed planet 3, so results are kept by the game they were played on (the `code` stamp; `page.py` names the three games):

- **Shots stop at set slime** (5be6b0b). The planet 3 search on that game (`pc/p3/gen3-5`, 992 games on maps 1-16 and the owner's) found plans without a Needle deck winning up to 13 of 17 maps. `econ,go=4,aim=6,mop=300` won the owner's planet in 8:03, the first bot win there without a deck. Plans with a Needle deck won 16 or 17 of 17.
- **Needle Blast bursts once per bullet** (#532, the owner's card 11-20 01:23Z). Job 09-needle-once played the deck test again on maps 1-8 and the owner's:

  | econ,go=4,aim=6,cards=0 with | Wins of 9 (19 Nov game) | Wins of 9 now | Better/worse than no cards | Sign p | Owner's planet |
  |---|---|---|---|---|---|
  | no cards | 5 | 5 | | | not cleared |
  | blast.needle | 7 | 8 | 8/1 | 0.039 | 8:52 |
  | seek/wide/blast.needle | 9 | 7 | 8/1 | 0.039 | wiped at 6:03 |
  | wide/rate/blast.needle | 8 | 8 | 9/0 | 0.004 | 6:10 |

  Only wide/rate/blast.needle survives a correction for the six comparisons (p < 0.0083). The Seek deck's wins take 6:26 on average (4:50 before). On the game with set-slime shots alone it won 8 of these 9 maps, so #532 cost it the owner's planet and about two minutes a win. With `lure,go=4,aim=6,cards=0` (4 of 9 without cards) no deck helps now: 4, 4 and 3 of 9.
- **Split groups, Heave and slug bullets on the new game** (job 10-split-now, 171 games; slug bullets, `want=plain.slug`, added on Direction's ask, 11-20 06:25Z). Split groups alone changed nothing (econ 3/3 and 2/4 better/worse, rush 5/4, lure 2/5). Heave took econ,go=4,aim=6 from 5 to 8 wins of 9 (6/3, p 0.51). Slugs helped rush: rush,aim=6 went from 4 to 8 wins (6/3), and with split=2 to 8 wins, better on 8 of 9 (p 0.039), winning the owner's planet in 8:22. Slugs cost econ and lure wins (econ 5 to 3, lure 5 to 4 and worse on 7 of 8, p 0.07). None of the 17 comparisons survives a correction (it needs p < 0.003; nine maps give at best 0.004).
- **Planet 2 on today's game** (Direction's job 11-planet2, 11-20 06:24Z, before any change to the Sprawler's size). 13 plans on maps 1-8 and the owner's three planet 2 runs (1795038793, 1795106014, 1795119328), 143 games: 57 won, 77 with the squad wiped (half by 3:04), 9 out of time. Plans that go straight at the slime (rush, rush,aim=6, bot, taker) won 33 of 44; plans that build at home first (econ, turtle, scott, econ,go=4,aim=6, scott,go=4,aim=6 and the wide/rate/blast.needle deck plan) won 9 of 66, so the best planet 3 plans are the worst here. rush won 10 of 11 (mean 5:38). Map 3 fell to no plan; maps 2 and 5 to 10 of 13. On the owner's runs he won in about 7:31, 9:05 and 13:37 (his logs, the 11-18/19 game); the bots' fastest were 5:53, 3:47 and 3:47, and 4, 8 and 2 of 13 plans won them.
- **Switching in the game** (11-20, after the owner's "Neither of those done threads sound done"; strat.rs `grow`, `regroup`, `spare`). Job 12-switch-p2, planet 2 on the game's own 15-minute clock, maps 1-8 and the owner's three, 275 games. All three switches with four soldiers bought first (`hold=8,grow=1.3,regroup=3,spare=1`) took econ,go=4,aim=6 from 1 to 8 wins of 11 (better on 9, worse on 1, p 0.021), scott,go=4,aim=6 from 1 to 7 (11/0, p 0.001) and lure,go=4,aim=6 from 5 to 9 (8/3). Wiped squads fell from 10 to 3, 10 to 2 and 5 to 1. Each switch alone did little; `hold=8` alone did most (econ 1 to 5). rush,aim=6 (8 of 11) never switches, and turtle got worse (1 to 0). Only scott's survives a correction for the 20 comparisons (p < 0.0025). Job 13-switch-p3 played the same switches on planet 3, maps 1-8 and the owner's, 108 games: they cost nothing there. econ went from 6 to 7 wins of 9, lure from 5 to 8 (mean win 10:06 against 7:24), and rush did not change. No comparison was significant.
- **Map 3 of planet 2** fell to no plan in jobs 11 and 12. The slime starts at 15.3k round a big crystal and holds about 13k while the squad kills 50 walkers a second, then doubles to 30k once the squad thins. Two shop buys that every plan made do nothing there. Tinder lays brush that only a fire burns, and the bots carried no Torch. Heavier shots (`strike`) do not reach the steered squad's shots, whose radius 2.5 x scale 2 x the lead's 1.4 is already squad::R_MAX 7 (squads.rs `strike_of`). In the cloud, rush,aim=6 with Tinder and strike and the same plan with an empty shop played the same game to the walker (wiped at 227.4 s, 14,292 kills). With Faster trigger in their place (cards=0) it grew to 40 soldiers and timed out with 14.7k slime left. Its bank sat at 169 from 5:30: it drained nothing more. Jobs 14-fire-p2 and 15-fire-p3 test Tinder and the Torch on every map; Direction made Heavier shots fire faster past the cap for the steered squad (#557), so 14-nostrike was dropped unplayed, and 14-map3-starve tests `starve` (clear and drain the crystal with the most slime on it). In the cloud `rush,aim=6,cards=0,starve=30` won map 3 in 8:20 with 41 soldiers left, the first bot win there (one game, lavapipe).
- **Whole runs** (job 14-runs, the game before main's #554 and #557, 66 runs: each plan from planet 1 with the real shop between planets until it loses one, on the owner's three runs and runs 1-8). The owner reached planets 4, 5 and 4 on his runs. econ,go=4,aim=6 with the switches reached 5 on 1795038793, past him, and lure with the switches 5 on 1795106014, level with him. The best bot on 1795119328 (rush,aim=6) reached 3. No plan was good everywhere: econ with the switches lost planet 1 on 5 of 11 runs, and rush,aim=6 lost planet 3 or earlier on 7. The Wide + Rate + Needle Blast deck bought only when offered (`shelf=0`) was never offered, so that plan played econ's games exactly. Job 15-runs-staged plays plans by planet (`rush,aim=6|rush,aim=6|econ,...`) on main's game.
- Main's pump-lamp fix (eb959ba) changed the build but not the play: 16 games replayed on both builds came out identical.
- The search restarted as `pc/p3n/` on the current game (16 plans in `p3n-plans.txt` plus three kept plans, 17 maps).
- The worker now pushes hourly. Each push to a branch with an open pull request re-runs the whole CI; pushing every chunk cancelled 24 of 26 runs on 11-20.

## Files

- `strat.rs`, `utility.rs` and `league.rs` in `src/bin/larder-world/`.
- `league.py`, `fetch.sh` (reads LEAGUEROW lines from GitHub job logs), and `page.py` with `page-template.html`.
- `scott-planet3.csv`: the owner's planet 3 every 5 s, from his play log on the `play-logs` branch.
- `queue/pc/NN-*.txt` hold the tests, and `pc/NN-*.jsonl` their results.
- `pc/p3/genN-*`: the planet 3 search.
- `gen0/`: generation 0 on GitHub.
- `pc/gen1-3*`: the mixed-planet night.
- `pc/worker-SCOTT-DESKTOP.log`: the worker's log.
