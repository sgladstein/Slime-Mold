# Mould tiers against the player AI: the plan-by-tier matrix (2026-11-12)

**Pre-#113.** This matrix predates shot craters (#113) and core defence (#126), and both hit the strike directly. Don't balance off it. The re-run on main after #126 will use seed blocks 5-36 and 201-232, add the toolkit hands, apply Holm over the whole matrix, and call a cell "best" only when it beats the runner-up significantly.

The tier counter tests the creature-rules thread handed over after PR #85: each mould tier against the world player AI's styles ([larder-world-ai-2026-11-12.md](larder-world-ai-2026-11-12.md)), idle and random, on the CPU race world. It grew into the plan-by-tier matrix the design review asked for (07:31Z): which plan is best against which mould. Raw output and scripts are in [larder-world-tiers-2026-11-12/](larder-world-tiers-2026-11-12/). Pictures, with the counts beside them, are on the [Artifact page](https://claude.ai/artifact/T4S63gUxYWbJqJEQfviHDe). Its source is archived in `larder-world-tiers-2026-11-12/page.html` (see [pages.md](pages.md)).

The rules moved three times while this ran, so there are three runs. Only the last is today's game. It is still from before #113's shot craters.

| Run | Rules | Folder |
| --- | --- | --- |
| 1 | W2b's score (crystals held at the cap) | `score1/` |
| 2 | Score v2 (#91: charge taken, the mould's size), the dry-crystal rule | `score2/`, `confirm/` |
| 3, the matrix | Main at b115954: v2 plus #106's free shots and soldier scent, the buildings (#112), the campaign tiers (#115) | `main/` |

## How it was run

- **World.** `larderworld census` on the race world: scale 2, 5,000 walkers, cap 900 s, 23 crystals that run dry. `tier=NAME` applies `mould::tiers::Tier` to the race world's settings (CPU only).
- **Seeds 53-60** for every run, used for nothing before. Fresh seeds 61-76 checked run 2's main claim.
- **Plans.** The player AI's boom, claim, fort and strike (`src/mould/larder_world_ai.rs`, unchanged apart from the dry-crystal fix below), the scripted farm hand, idle and random.
- **Measures, named before run 1** (`score1/MEASURES.txt`). D1: each style beats idle and random 8/8 on each tier. D2: each plan on a tier, paired with the same plan on the plain mould. D3: strike minus boom on a tier, paired with the same difference on plain. Holm over all 57 tests. The note said it then: with 8 seeds this is a first look. 8/8 gives p 0.0078, which is about 0.45 after a correction over 57. No single cell survives Holm, so read the matrix as a spread.
- **Cost.** A think is about 180 µs once a game second (unchanged). The world step is 5-10 ms a tick on plain and 13-27 ms on the Sporer and Sleeper (their life cycles).

## The matrix (run 3, today's main before craters)

Median score of each plan on seeds 53-60. The ends are mould wiped / cap / squad wiped. The score is `Game::score` v2: a win is above 1, and a stalemate at the cap is about −0.4 to +0.5. Raw output: [main/](larder-world-tiers-2026-11-12/main/). The table comes from [matrix.txt](larder-world-tiers-2026-11-12/main/matrix.txt) (`python3 matrix.py main`).

| Tier | idle | random | farm | boom | claim | fort | strike | Best plan (seeds best on) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| plain | −0.37 0/8/0 | −1.40 0/2/6 | +0.19 0/8/0 | +0.22 1/7/0 | +0.34 3/5/0 | +0.22 1/7/0 | **+2.08 5/3/0** | strike (5) |
| Sporer | −0.41 0/8/0 | −1.70 1/2/5 | +0.00 0/8/0 | +0.10 1/7/0 | **+0.17 1/7/0** | +0.11 0/8/0 | +0.09 1/7/0 | **claim (5)**, strike 0 |
| Sleeper | −0.16 0/8/0 | −1.48 0/3/5 | +0.30 0/8/0 | +0.32 0/8/0 | +0.48 0/8/0 | +0.29 0/8/0 | **+0.49 0/8/0** | strike (8) |
| Swarmer | −0.38 0/6/2 | −1.64 0/2/6 | +0.15 0/8/0 | +0.18 1/7/0 | +0.25 1/7/0 | +0.19 1/7/0 | **+2.18 6/2/0** | strike (7) |
| Drifter | −0.21 0/8/0 | −0.73 0/4/4 | +0.36 0/8/0 | +0.43 0/8/0 | +0.45 0/8/0 | +0.42 0/8/0 | **+2.31 8/0/0** | strike (8) |
| Sprawler | −0.88 0/4/4 | −1.60 1/1/6 | +0.20 1/7/0 | +0.10 1/7/0 | +0.14 1/7/0 | +0.08 1/7/0 | **+0.42 3/5/0** | strike (5), farm (3) |
| Trunk | −0.26 0/8/0 | +0.02 2/3/3 | +0.32 0/8/0 | +0.29 0/8/0 | +0.36 2/6/0 | +0.30 0/8/0 | **+2.21 7/1/0** | strike (7) |
| Grudge | −0.37 0/8/0 | −1.70 0/2/6 | +0.14 0/8/0 | −0.01 0/8/0 | −0.02 0/5/3 | +0.00 0/8/0 | **+0.18 1/7/0** | strike (5), farm (2) |

- **The strike is the best plan on seven of eight tiers.** It wipes the plain mould on 5 of 8 seeds, the Swarmer on 6, the Trunk on 7 and the Drifter on 8. Since #106's free shots, a squad of 30 that keeps 8 farming can always fire. A rush on the core is the dominant plan, and it needs a counter.
- **The Sporer is the one tier that flips it.** The strike wipes it once (seed 55, which every plan wins early). On the rest it takes the core and stands at the cap with 30 soldiers while 9,000-16,000 walkers live on. The spores have already left the core. Claim is best on 5 seeds and fort on 2. The tiers thread's reading: a strike makes the Sporer fruit, so the strike spreads it.
- **Three tiers make every plan worse** (paired with the same plan on plain).
  - Sporer: idle, farm, boom, claim and strike are each lower on 8/8.
  - Grudge: boom, claim, fort and strike lower on 8/8, and claim loses its squad on 3 seeds.
  - Sprawler: boom, claim and fort lower on 8/8. Idle is wiped on 4 seeds.
- **Three tiers are easier than plain.** Idle scores higher on 8/8 against the Sleeper, the Drifter and the Trunk, and so does random against the Drifter and the Trunk. Random wipes the Trunk on 2 seeds. The tiers thread has the same reading: the Drifter is too gentle.
- **Random's mould wipes per tier** (asked by the integration gate rule): Sporer 1, Sprawler 1, Trunk 2, the rest 0.
- **Dead time** (the census's stuck seconds, asked by the design review): no AI plan is ever stuck for more than 2 s a game on any tier. Random is stuck for 52-309 s, most against the Sleeper.
- Wary also ran (`main/wary.txt`). The tiers thread dropped it at 13:26Z (e359fac), because random fire herds it off every crystal, so it is left out of the matrix.

## Run 2: score v2, and the Sporer check on fresh seeds

On score v2 before free shots ([score2/analysis.txt](larder-world-tiers-2026-11-12/score2/analysis.txt)), the strike was best on 6, 8 and 6 of 8 seeds on plain, the Sleeper and the Swarmer. Against the Sporer, boom was best on 6. The claim that the Sporer flips the best plan was checked on fresh seeds 61-76, with measures named first ([confirm/MEASURES.txt](larder-world-tiers-2026-11-12/confirm/MEASURES.txt)):

- **C1, strike above boom on plain: fails.** It was 6 higher and 9 lower (p 0.61). The strike was a gamble: it wiped the mould on 3 of 16 seeds, and its median was a little under boom's.
- **C2, strike's lead over boom shrinks on the Sporer: 11-4, p 0.12.** It does not hold.
- **Not named in advance:** on the Sporer, boom beat the strike 13-2 (p 0.007), and the strike never wiped a Sporer in 16 seeds.

So under v2 the Sporer took away the strike's upside, but the strike was not yet the best plan on plain. Free shots (#106) made it the best plan, as the matrix shows.

- **Claim lost its squad under v2 before free shots:** 3 of 8 seeds on plain and 7 of 8 on the Sporer, between 625 and 741 s, with the bank at 0. Holders went into the network after 10:00 once the crystals had run dry, and every shot cost food. With free shots this is gone: claim loses no squad on plain.

## Run 1: W2b's score

[score1/analysis.txt](larder-world-tiers-2026-11-12/score1/analysis.txt). The Sporer was the hard tier: every AI style scored lower than on plain on 7 or 8 of 8 seeds, and the strike lost its squad down to 2-6 soldiers on 7 of 8. The Sleeper was easier: idle survived to the cap on 8/8. The strike took its core on 8/8, but 7-55 dormant walkers lived to the cap, so it never counted as a wipe. The Swarmer showed no measurable difference. The D3 predictions (the strike gains against the Swarmer and loses against the Sleeper) were not supported.

## The AI change: dry crystals

Score v2 turned on `Rules::home_dry`: a carrier on a crystal holding less than one frame's ask walks home and holds. The brain's `View` used to list every crystal with charge above 0, so it kept sending soldiers back to 0.8-charge crumbs. It now lists only crystals with at least one frame's ask (`View::of`). On dev seeds 1-4 the strike scored higher on 4/4 with the fix (two wipes, against none), and claim on 3/4. Boom was a hair lower on 4/4 (by 0.002-0.07).

## What this means for the game

- **A counter to the core rush is the next thing.** The rush wins seven tiers out of eight. The Larder thread's next PR is core defence, and the tiers thread is building a per-tier core: the Sprawler with no core, and the Grudge's core pull. The gate the tiers thread set: the strike is best on at most 2 tiers.
- **The Sporer already shows what a counter looks like.** The plan that wins elsewhere doesn't win against it, and holding crystals (claim, fort) does.
- **Shot craters (#113) change the baseline again.** Random wipes the mould on 11 of 32 race seeds with them, so this matrix is from before craters. The same harness re-runs it: `larderworld census seeds=… hands=idle,random,farm,boom,claim,fort,strike tier=NAME`, then `python3 matrix.py DIR`.

## Seeds used

Seeds 53-60 and 61-76 are now used. The tech-lead review (04:40Z) asked for a standing held-out seed block and one held-out map for every milestone. The integration gate rule (ROADMAP, 10:09Z) now plays seeds 5-36 and 201-232. The player AI's next confirmation runs use fresh seeds from 301 up, and the map Rim as its held-out map (the AI has never played a map).
