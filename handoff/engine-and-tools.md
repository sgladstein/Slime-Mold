# Engine and tools: the lane's handoff

Rewritten 2026-11-28 by the new Engine thread (account move, plan
[`Reports/plan-2026-11-27/README.md`](../Reports/plan-2026-11-27/README.md),
steps 3 and 4). The lane brief and the state log from 11-17 to 11-27 are
in [`engine-and-tools-2026-11-17-to-27.md`](engine-and-tools-2026-11-17-to-27.md):
read it for how a tool came to be, not for orders.

## What the lane owns

`src/gpu/mod.rs`, `larder_world/{turn,world,score,rules}.rs`, the window's
`main.rs`, `headless.rs`, `hands.rs`, `src/mould/playlog.rs`, CI
(`.github/workflows/`), `scripts/`, `play.ps1`, `wiki/lint.py`; by use, the
bots: `strat.rs` (the strategy and run bot), `league.rs`, `runbot.rs`.
Squad and arsenal writes the turret-design part of the bot in these files
(#887, 11-28).

## Milestone 2 tests this lane serves

1 (the run bot wipes every step out), 3 (the curve by step and pip), 4 (save
and resume of a star-map run), 5 (60 fps at 4K on the GTX 1070).

## State, 2026-11-28

- **Nightly Run bot** (`run-bot.yml`, planets 1 to 3, seeds 1 to 3): red
  since 11-25 on the soft-lock check, not a crash. Step 3 is the Trunk since
  #868; the bot goes broke with about 11 soldiers once every free crystal is
  drained, the slime keeps the home and one more crystal (17,000 walkers),
  and the bot's stand kills only what regrows: stall 2:01 to 3:44 (run
  46087963392; lavapipe repeat on 9a5ec6c: 1:40, won at 10:38).
- **Branch `claude/project-thread-8iu6no`** (this thread), open PR:
  - `push=SECS` dial (`strat.rs`, on at 40): after SECS with no harvest and
    no new 1% low, clear the slime's crystal with the fewest walkers until
    it is dry or the stall ends. First arms batch (f147c81a): the longest
    stall fell in 7 pairs, rose in none, 3 tied (p 0.008); planet 3 seed 4
    loss to win; pushes cost soldiers. Still over a minute on 5 of 10.
    Second batch (8543d53e): push=20 won 2 of 6 planet 3 games against 4
    of 6 for 40, so 40 stays. First runs by pip, planets 1 to 3, seeds 1
    to 3: Safe won 8 of 9 planets (the loss: a Safe run with no Safe or
    Fair way on took Hard; now the nearest pip); Hard lost step 2 (the
    Sprawler) on all three seeds, in 1:19 to 4:00 with 19 to 50 fallen.
  - The heart siege clears the live heart's own crystal (any crystal since
    #858), not crystal 0.
  - Star-map picks: `league=run:SEED[:TO[:SECS[:PIP]]]` takes that pip at
    each fork (`league::pick_pip`: that pip, else Fair, else the first); every
    league line carries `pip=`. Arms rows `runN.PIP` and `dials=A=1,B=2`.
  - The first Starfall/core of a run no longer says its words twice (corner
    alert held back while the run's Starfall note shows; `main.rs`).
  - Lint: a pull request changing `src/` with no `handoff/*.md` change fails
    unless a commit says `No-handoff: <why>` (`wiki/lint.py handoff_kept`).
- **PC strategy league** (`C:\Users\Scott\Code\slime-league`, detached HEAD,
  src tree `cb8d00f3`, the 11-22 build): paused 11-28 03:27Z with a `PAUSE`
  file (Scott allowed the Remote Control card). Two worker Pythons were
  running; delete `PAUSE` only after the clone is rebuilt on main.
- **Where long runs go** (Scott, 11-28 03:34Z): his PC when it is on and
  connected (Remote Control, the league clone), else GitHub's checks.

## Next, in order

1. run-bot.yml green or naming a real wall: its `--check` still fails on a
   stall over a minute, and stalls of 1 to 4 minutes remain on the Trunk
   with the push (the bot losing a fight, not a soft lock). Asked
   Direction 11-28 whether a stall becomes a named wall instead of a
   failure; their rule (11-22), so not changed here. Planet 3 seed 2 loses
   all 31 soldiers by 4:44 in every arm.
2. Whether the heart siege should stay at all (hearts are not the win,
   #858): an arms pair with the siege's heart block off.
3. The builder arm (pumps plus a turret each) now that #887 builds rolled
   designs.
4. run-bot.yml by pip (Safe, Fair, Hard); the curve on twelve steps (PC
   first, else GitHub), rows for Direction's test 3 page.
5. Rebuild the PC league on main, then delete `PAUSE`.
6. GTX 1070 retime: a Hard step 9 and the home world at 3840x2160, and a
   mid-planet save and resume of a star-map run (`scripts/pc-batch.ps1`;
   the `gpu-timing.lock` rule in project memory `gtx1070-batch`).
7. D8: list what stands on the CPU engine (`src/mould/big.rs`) for Direction.

## Tools, where to look

- The bot's dials: `strat.rs` `Plan` (each dial's doc says its measurements);
  `STRAT_DBG=1` prints the hunt's choice every 10 s.
- League jobs: `league=DEPTH:SEED[:SECS[:PIP]]`, `league=run:...` above;
  `scripts/run_bot_summary.py` (`--check`, `--pace`).
- Arms batches: `.github/workflows/arms.yml` from a PR changing the queue.
- The star map's bot check: `Reports/star-map-2026-11-26/` (`check.sh`,
  `order.py`).
- Cloud play on software Vulkan: `mesa-vulkan-drivers`,
  `VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`; a planet 3 game
  takes about 10 real minutes on 4 cores.
