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
   Control on Scott's desktop (standing OK for tests, 11-11). Booked so
   far, each against main without it on the same planet:
   - the Lure (the Lab's steering, planets 1 to 3);
   - the Trunk's fort (Enemy's #473, 11-18): walkers on the Trunk's
     planet read one more `news` word a step (Params word 888, the last
     quarter of the cells' region), and `time_fort` runs once a second;
   - the raised walls (Look's view prep, 11-18): one multiply-add a fine
     cell in the view's prep each frame;
   - the tether (Enemy's #496, 11-18): 8 more cell reads for a breakaway
     walker stepping into a new cell (Params word 892), and the raid march
     (CPU, once a game second).

## The CPU engine

Scott asked on 11-17 whether the CPU twin of the slime engine is needed
at all. The planning thread measured one 60-second game at 23 s on the
CPU engine and 27 s on the software GPU (one run each; the CPU game uses
one core, so batches run about four times faster on it), and recommended:
new slime rules on the GPU only, today's CPU engine left as it is, its
removal decided at Milestone 2. That is on a card with him (19:10Z). If
he picks "retire it now", that becomes your first job after job 1.

## State (kept current by the lane)

**2026-11-18.** Branch `claude/project-thread-9y1l65`. Job 1 merged in
#449 (11-17 22:28Z), jobs 2 and 3 in #452 (11-18 03:38Z); CI's Windows
build is published. Job 4's first cut (the run bot) merged in #463
(11-18 ~09:40Z); the Run bot workflow was started on main by hand the
first time (the queue that landed #463 ran main's older queue file).

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

What jobs 2 and 3 put where:

- **The play log**: one file a round, `play-logs/<when>-<map>-<seed>.txt`
  (`playlog::file_name`), only in a real window and not on the menu. F9
  writes `note n=.. t=.. x=.. y=.. zoom=..` (`App::drop_note`). A typed
  note needs a text field in the HUD, which is Look's. Since #507
  (11-19) the head also names the loadout: `gun=` (the army round's
  cards), and in a run `turrets=` (their deck) and `bought=` (upgrades,
  key and level); a mid-round change of the gun's cards writes
  `gun t=.. CARDS` (`App::log_loadout`).
- **Sending the logs**: `play.ps1` `Send-PlayLogs`, after the game quits,
  to the branch `play-logs` (plumbing, own index, fixed author name);
  sent logs move to `play-logs/sent`. `play.cmd send-logs` lets git ask
  for a sign-in once.
- **The Windows build**: `.github/workflows/windows-build.yml` publishes
  to the prerelease `windows-build` on every new main (the queue
  dispatches it). `play.ps1` `Get-CiBuild` uses it only when no game file
  differs from HEAD. `slime_mould::build::root()` finds the maps from a
  build made elsewhere (`LARDER_ROOT`, set by `play.ps1`).
- **Testing play.ps1**: `scripts/play-selftest.ps1`, run by the Windows
  workflow on pull requests that touch the play scripts. Nothing else
  runs PowerShell; a cloud session cannot.

What job 4 put where (first cut, 11-18):

- **The bot**: `hand=bot` (`hands::bot_hand`), for films too. One squad;
  buys soldiers whenever it can; works the crystal with the most charge
  (a quiet near one until 30 soldiers), volleys at the heaviest slime in
  range, leaves a crystal that pays nothing for 30 s, takes the slime's
  home crystal once strong, steps back and heals when hurt. Its doc
  comment lists the five versions that lost and why. It prints a `BOT`
  line every 10 game seconds on stderr.
- **The run**: `runbot=SEED[:TO]` (`runbot.rs`): planets 1 to TO with the
  shop between (`runbot::shop`: the card the next planet asks for, then
  `WANTS`), a line a planet on stdout. A planet with no verdict a minute
  past its clock is reported as a soft lock.
- **CI**: `the_window_plays_planet_1_with_the_run_bot` (planet 1, about
  80 s on lavapipe; asserts a verdict and not under a minute, never a
  win). `.github/workflows/run-bot.yml` plays planets 1 to 3 for seeds 1
  to 3 after each landing (the queue dispatches it) and nightly, into the
  run's summary; it never fails on a loss.
- **Measured** (run seed 1, 11-18): planet 1 won at 2:48, 2 stars (mass
  9,300 to 110 in 28 s once 50 soldiers walked into the home crystal);
  planet 2 lost at the clock twice, the Sprawler's threads eating the
  9-soldier landing squad before it drained (1,471 food in 15 minutes),
  Tinder bought or not.
- **CI's first curve** (main af78a06, 11-18): 1 planet won of 9. Run seed
  1 as above; run seeds 2 and 3 lost planet 1 at the clock (27 and 4
  soldiers left).
- **Planet 1 on all three run seeds** (11-18, this branch): the home is
  taken once every other crystal is dry or gave nothing for 30 s (it
  counted as charged before, so seed 2 never went home with 29 soldiers,
  one under `BOT_ECON`), and only while the slime by it is under 200 a
  soldier. Seeds 1 and 2 win (2:48 and 5:16, 2 stars); seed 3 loses at
  the clock: its slime sits behind a band of set slime (the bulwark, grey
  in the film) that soaks the volley, so the hunt kills about 200 a
  half-minute while it grows 300, and a walk-in with 36 soldiers at 267
  a soldier was eaten to 2. Tried and reverted: a siege from a stand
  short of the home's heaviest slime (the home's slime is nearly all the
  slime, so it never ended, and seed 1 lost).

Planet 2 tries after #463 (11-18, reverted; run seed 1, `look=DIR
run=1:2 mode=squads hand=bot`):
- build up at the quiet crystal nearest the landing instead of the most
  charge for the walk: it drained crystal 3 by 0:50 but the slime took
  most of it (1,192 food); the squad never passed 12 and was worn out
  by 3:47 (a 421-walker hunter mini-boss at 1:18 and 3:32).
- pumps first on the biggest quiet crystals near the landing (a pump
  draws 1/150 of its crystal's start charge a second for ever): the
  Sprawler ate the first pump by 0:40, 1,632 food in 5 minutes; on
  planet 1 the same rule lost a won planet (3,897 food all game, the
  pumps eaten), so pumps before a big squad is a dead end for the bot.

- **Seed 3, later the same day**: the grey band round its home is not a
  bulwark or set slime (none on planet 1) but rock: the home sits in a
  basin with one gap. The volley and the hunt now aim only along lines of
  open ground (`clear` in `bot_hand`; seeds 1 and 2 still win, 2:49 and
  4:42), but seed 3 still loses at the clock with 34 soldiers: the hunt
  kills about 300 a half-minute while the home's slime grows about 350.
  Walking in at 250 a soldier (41 soldiers at 3:20) left 8 within 10 s.
  Not yet tried: film it to see whether the squad reaches the gap, and
  pumps on the basin's rim once the squad is 40.

- **Planet 2 after #468** (11-18, local): run seed 1 loses at the cap
  with 46k slime (its landing crystal is by the slime; 9 soldiers worn to
  1 by 3:00, 1.7k food all game). Run seed 2 reaches 30 soldiers by 2:30
  and ends with 14.5k slime: the home stays over 200 a soldier, and the
  two charged crystals left (2 and 5) give nothing while the slime holds
  them, so the bot swaps between them and heals its bank to 0. The home
  is now no pick or work until it can be taken (it was held as the work
  while the bot hunted, 15.9k slime at the end).

Job 5, branches (11-18): a thread cannot delete another's branch (the
proxy refused both `git push --delete` and the refs API), so on Scott's
card ("Let CI delete them", 13:17Z) `.github/workflows/branches.yml` runs
`scripts/branches.sh` after each landing (the queue dispatches it),
weekly and by hand: it deletes branches whose tip is in main and lists
the rest with the commits main has no copy of in the run's summary, for
Direction. Its first run (11-18 13:52Z, actions run 44317767827) deleted
91 (a session's shallow clone had found 18) and listed 56 not in main,
most of them 11-10 to 11-12 runner and census branches with one commit
each. Still open in job 5: films to an archive
branch with a size cap a pull request (gifs and mp4s are 1.2 GB of
main's 1.47 GB tree). The lint caps were already in.

**Next:** seed 3's planet 1 (above), then the bot on planet 2: clear a
held crystal by hunting the slime on it before draining it (land, back off to a quiet crystal, hold it
with the volley, build up), then planet 3; then whether the old run smoke
rounds (`the_window_plays_a_runs_planet`, RTS mode) go, measured against
the bot's coverage. After job 4: job 5's housekeeping.
