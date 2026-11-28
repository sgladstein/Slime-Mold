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

- **Merged #889 (11-28 05:50Z)**: `push=40` (after 40 s with no harvest and
  no new 1% low, clear the slime's thinnest crystal; arms batch 1: longest
  stall fell in 7 pairs, rose in 0, p 0.008; batch 2: push=20 won 2 of 6
  planet 3 games against 4 for 40); the heart siege clears the live heart's
  own crystal; `league=run:SEED[:TO[:SECS[:PIP]]]` and `pick_pip` (that
  pip, else the nearest); every league line has `pip=`; arms rows
  `runN.PIP` and `dials=`; run-bot.yml plays seeds 1 to 3 times Safe, Fair,
  Hard, a stall over a minute is a WALL line (Direction's word), soft
  locks fail; one ore notice; the handoff lint. First runs by pip: Safe
  won 8 of 9 planets, Hard lost step 2 (Sprawler) on all three seeds, a
  finding for the curve page (Direction: Scott wants it harder).
- **The curve kit** (#898, #904 merged; arms PR open on
  `claude/project-thread-8iu6no`). `scripts/pc-curve.ps1` (the owner's PC:
  its own clone `slime-curve`, one planet a game `league=STEP:SEED:SECS:PIP`,
  arms from `$ArmDials`: main, builder, kite, kitecards, kitebuild; default
  main, kitecards, kitebuild; waits while he plays or `gpu-timing.lock`
  exists, resumes from its rows, `-Push` to branch `curve-out`),
  `.github/workflows/curve.yml` (the GitHub fallback, by hand only: a whole
  curve is about 280 games, tens of runner hours, so name the part and its
  time first), `scripts/curve_table.py` (the table by arm, step and pip, and
  the paired checks). First PC rows (c596e7c6, seed 1, main/builder): every
  Safe step won, Hard step 2 lost at 77 s, Fair 3 to 5 lost to the hunt
  (#895; `hunt_pull=0` kept 44 alive). Direction 06:51Z: a bot result, not
  a retune signal; the curve needs an arm with a real build, hence the
  kite arms. kitebuild still built no pump on steps 3 and 4: the builder
  dials' pump wait is the next thing to look at if it stays bare.
- **The scott arm** (Scott's own build from his 11-28 play logs: carbine,
  Split, Rate, Range, torch needle; Armour, Heavier shots, trigger, barrels,
  Fireproof): step 5 Fair full clock won seeds 1-3 where main lost all
  three; the curve's default arms are now main, scott, kitecards. Next:
  the scott arm across the curve, then whether it should be the run bot's
  plan (Direction's call; scripts/run-bot-plan.txt).
- **PC strategy league** (`C:\Users\Scott\Code\slime-league`, detached HEAD,
  src tree `cb8d00f3`, the 11-22 build): paused 11-28 03:27Z with a `PAUSE`
  file (Scott allowed the Remote Control card). Two worker Pythons were
  running; delete `PAUSE` only after the clone is rebuilt on main.
- **Where long runs go** (Scott, 11-28 03:34Z): his PC when it is on and
  connected (Remote Control, the league clone), else GitHub's checks.

## Next, in order

1. The curve: on the owner's PC when it is on and connected (Remote Control
   in C:\Users\Scott\Code: `scripts\pc-curve.ps1 -DryRun`, then `-Push`),
   else a named part on GitHub (`curve.yml`). Rows to Direction for the
   test 3 page (`curve_table.py`).
2. Whether the heart siege should stay at all (hearts are not the win,
   #858): an arms pair with the siege's heart block off.
3. Rebuild the PC league on main, then delete `PAUSE`.
4. GTX 1070 retime: a Hard step 9 and the home world at 3840x2160, and a
   mid-planet save and resume of a star-map run (`scripts/pc-batch.ps1`;
   the `gpu-timing.lock` rule in project memory `gtx1070-batch`).
5. D8 is decided (Scott 11-28 05:38Z, "Keep it, no new twins"): nothing.

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
