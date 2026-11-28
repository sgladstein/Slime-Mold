# The smell experiment, stage 0: the knowledge gate (2026-11-28)

The owner, 11-28 08:31Z: "I think it could be fun if 'the slime know only
what it can smell' but it will break a lot of the behaviors that we have
and might end up less fun. Lets test it out though." Built off primary
play on this branch (`claude/project-thread-04vjrt`), never merged. Design
and its reviews: the doc "Smell design for Plasmodia"
(https://claude.ai/artifact/UrcABXB8zVwLMCi2VZPD5A); the measured review
it starts from: `Reports/scent-review-2026-11-28/`.

## What was built

`src/mould/larder_world/know.rs`, `set=know_reach=40` (0, the default, is
today's game; negative is a placebo that reads the same and gates
nothing). Once a game second a soldier or building is smelled while it
stands within `know_reach` dish cells of walking from the slime's body
(every dish cell with 2+ walkers, `round_walk_from`, built off the frame a
second late), or within the guns' reach and the panic margin of it by
air. It is known for `know_memory` (10) seconds. The hunt's trail, raid
aims (pumps, buildings, thickest knot, last stand), waves' gather and
strike, and the mini-bosses' targets take only what is known. Lures are
always known; charges, the crystals' far smell and every keep-out-of-the-
guns check are as they were. The league line gains
`know=S/A/B/U:b/l/s/x:w` (smelled and alive soldier-seconds, smelled and
standing building-seconds, raids by aim, waves).

## Batches (the owner's PC, `run-batch.ps1`)

`queue.txt`/`queue2.txt`: rush (the CI run bot), kite (rush + kite=0.8),
turtle and econ plans, planets 2 and 4, run seeds 1-6, gate off and on:
96 games a batch, `league=PLANET:SEED:0`. Batch 1 (`rows.txt`, 41 min at
2 games at a time) had a flaw: buildings still going up were never
smelled, and the charges' unit field was gated, so the charges that wore
them went for the soldiers building them (planet 2 seed 1 on llvmpipe: a
turtle that won at 278 s lost at 127 s; one gate at a time showed only the
charges' changed the game to 140 s). Batch 2 (`rows2.txt`, 65 min at one
game at a time) is the fixed build (e97b779c). `analyse.py ROWS` pairs
them; outputs in `batch1-analysis.txt`, `batch2-analysis.txt`.

Deciding measures, named before the run: soldiers lost (`fell_end`) and
wins, paired off/on by planet, seed and plan, sign test. Kill: rush or
kite lose fewer soldiers with the gate on 5 of 6 seeds on a planet; a home
plan (turtle, econ) wins more or loses fewer buildings with it on; raids
sent fall by half.

Batch 2:

| | Planet 2 | Planet 4 |
|---|---|---|
| Soldiers lost, off -> on | 94 -> 85 | 1,561 -> 1,765 |
| Pairs on lost fewer / more | 5 / 4 (p 1.00) | 6 / 12 (p 0.24) |
| Wins | 16 -> 15 | 8 -> 9 |
| Buildings lost | 27 -> 27 | 41 -> 77 |
| Raids sent | 24 -> 16 | 456 -> 425 |
| Waves sent | 0 -> 0 | 31 -> 26 |
| Squad smelled (share of soldier-seconds, on) | 70% | 76% |
| Buildings smelled (share of building-seconds, on) | 61% | 46% |

No kill rule fired in either batch (best: rush on planet 2 lost fewer on
3 of 6 seeds, more on 0). Nothing survives a correction for 8 tests.

## Findings

1. The gate does not make the game easier, and changes little: the slime
   smells the squad 70-90% of the time anyway, because it is big and the
   squad fights in or beside it. Map-wide knowledge rarely did the work.
2. Reach 20 against 40 (planet 2 seed 1, turtle, llvmpipe): 76% against
   79% of soldier-seconds smelled; the gun-range sense by air covers most.
3. Where it moved, it leaned a little harder on planet 4 (12 of 18
   differing pairs lost more soldiers; the econ bot's undefended pumps lost
   more buildings), within noise.
4. The placebo (`know_reach=-40`) played the off game exactly (planet 2
   seed 1), so the reading itself does not perturb the game.
5. Rows from the PC differ from llvmpipe's for the same seed (float
   order); pairs are always within one machine.

So knowledge is not why the game is easy; the gate is safe but invisible.
Stage 1 of the design (a prey trail the squad lays, the slime follows and
the player sees) is the part that would change how a fight reads; it is a
new top-level smell feature and waits for the owner's answer (his 11-28
ruling: design before building in the scent lane).

`run-batch.ps1` writes an empty `wall=` for a few games (PowerShell 5 lost
`ExitTime`); the game's own `wall=` in the row is right.
