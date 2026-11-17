# Lane brief: Engine and tools

Model and effort: Fable, high; extra high for job 6. Read
`00-all-lanes.md` in this folder first.

## Your job

Make the game easy to play, test and change. You own `src/gpu/mod.rs`,
`larder_world/{turn,world,score,rules}.rs`, the window's `main.rs`,
`headless.rs`, `hands.rs`, `src/mould/playlog.rs`, CI, `scripts/`,
`play.ps1` and `wiki/lint.py`. For Milestone 1 your targets are exit
tests 1 and 6: a bot that plays planets 1 to 3, and a game that says
itself what is on.

Start from `handoff/integration.md` and
`Reports/whole-game-review-2026-11-15/engineering-health.md` as history.

## First jobs, in order

1. **The build tells Scott what it holds** (days 1 and 2; other lanes
   depend on it).
   - One list of switches in code: each declared once with its name,
     default, owner lane, date and one line on what the player will see.
     A test fails on a switch that is not declared. The 14 rows of
     ROADMAP's "Switches that are off" are the starting content; Enemy is
     settling them.
   - A build number, shown on the menu and written in the play log.
   - One file the game reads for "what is new", one line per change, and
     a lint that fails a pull request which changes what the player sees
     or does without adding a line. The game remembers the last build
     played on that PC.
   - Look and interface draws the pages from these; agree the data shape
     with that lane on day 1.
2. **The play record.** `playlog.rs` already writes a line each game
   second. Add the build number, the switches that were on and the planet,
   and a key in play that drops a note. `play.cmd` pushes the logs to a
   branch when the game quits (Scott, 17:39Z: "yes play logs is a great
   idea"). The repository is public and git on his PC must be signed in
   for the push, so the script must say plainly when it could not send,
   and never block play.
3. **A Windows build from CI** on every green main, fetched by `play.cmd`
   with a fall-back to a local build. CI already has a Windows job.
4. **The run bot.** A scripted hand that plays planet 1 to a win or a
   loss, then planets 1 to 3 with the shop between. Today no hand wins a
   planet and the only run tests play 60 game seconds. In CI it replaces
   the per-feature smoke checks: idle loses, nothing soft-locks, no planet
   ends in under a minute.
5. **Housekeeping Scott approved (19:07Z).**
   - Old branches: delete only remote branches whose tip is already in
     main. List any with unmerged commits to Direction and leave them;
     `claude/project-thread-vx9t3p-wip` holds unmerged card work. Scott
     turns on "delete branches on merge" himself.
   - Films and stills move to an archive branch, with a size cap per pull
     request in CI. The clone is 1.2 GB today.
   - Lint: cap `NOW.md` at 100 lines, make the 2 KB cap on a wiki page's
     Now a failure, and retire the rule that needs links from
     `HANDOFF.md` so Direction can archive it.
6. **Move the run, the gun cards (`Rounds`) and the tools (`Kit`) out of
   the window's `App` into the library's game**, so tests, replays and
   the bot play the game Scott plays. Agree the timing with Squad and
   arsenal.
7. **One GTX 1070 batch** before the milestone check, through Remote
   Control on Scott's desktop (standing OK for tests, 11-11).

## The CPU engine

Scott asked on 11-17 whether the CPU twin of the slime engine is needed
at all. The planning thread measured one 60-second game at 23 s on the
CPU engine and 27 s on the software GPU (one run each; the CPU game uses
one core, so batches run about four times faster on it), and recommended:
new slime rules on the GPU only, today's CPU engine left as it is, its
removal decided at Milestone 2. That is on a card with him (19:10Z). If
he picks "retire it now", that becomes your first job after job 1.

## State (kept current by the lane)

**2026-11-17.** Branch `claude/project-thread-9y1l65`. Job 1 is built and in
its pull request; nothing of the lane is merged yet.

What job 1 put where:

- **The switch list**: `src/build/switches.rs`, `SWITCHES`. A row is a name,
  its choices (each a set of the game's own keys), the default, the lane,
  the date, what the player sees and why it is off. `try=NAME[:CHOICE]` on
  the command line and the Options page's Experiments tab turn the same
  rows (`src/bin/larder-world/experiments.rs`). Tests in
  `src/build/switches/tests.rs`: `no_switch_is_undeclared` fails on an
  engine or rule key that the default game leaves at 0 on every planet and
  tier and that no row declares; `mouldgpu`'s all-switches test plays
  `switches::all_on()`. The count rule (at most three) is held off by
  `SETTLING = true` until Enemy has settled the 11-17 rows: set it false in
  the change that gets the list down to three.
- **The build number**: `build.rs` stamps the commit count, hash and date;
  `slime_mould::build::build()` reads them. 0 on a shallow clone (cloud
  sessions), where the commit shows instead. In the window's title and the
  play log's header (`build=`, with `switches=`, `keys=`, `planet=`).
- **What is new**: `news/whats-new.txt` and `news/things-to-try.txt`,
  compiled in (`src/build/news.rs`). `Memory` keeps the last build played
  and the lines seen in the settings file. `wiki/lint.py --base` fails a
  change to the game that adds no line, unless a commit message says
  `Nothing-new: <why>`.
- **Housekeeping already done** (from job 5): the lint's `HANDOFF.md` link
  rule is retired, `NOW.md` is capped at 100 lines, a page's Now over 2 KB
  fails (the 12 pages over on 11-17 fail only if they grow: `NOW_OVER`).

Not done from job 1: the menu's corner does not show the build yet and the
"what is new" and "things to try" pages are not drawn. Those are Look and
interface's, from the data above.

**Next:** job 2, the play record (a note key in play; `play.cmd` pushes
the logs to a branch), then job 3, a Windows build from CI.
