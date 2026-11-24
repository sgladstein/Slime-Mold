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

- **Others' changes to the bot** (11-20): #539 (balance review) makes
  `runbot.rs` buy the Torch (`bring_shell`) before Tinder; before, it
  bought Tinder alone on every check, so expect the curve to move. If
  Direction adopts "held on" (a 1-star time-out) as an outcome, the
  bot's summary should count it apart from wins (the review's ask).

- **One bot** (11-20, NOW.md: "joined with the strategy bots rather than
  a second bot"): CI's run bot (`run-bot.yml`) plays
  `league=run:SEED:3 plan=PLAN`, the plan the first line of
  `scripts/run-bot-plan.txt`, which the strategy bot's thread sets once
  its plan-by-planet runs (#567) are measured on main
  (provisionally `rush,aim=6|rush,aim=6,cards=0|econ,go=4,aim=6,hold=8,grow=1.3,regroup=3,spare=1`).
  `scripts/run_bot_summary.py` writes a row a planet with its length
  against 8 to 12 minutes. First try (branch run 44777837761, plan
  `bot,want=torch.standard`): seed 1 won planet 1 at 4:08 then lost
  planet 2 at 3:14; seed 2 lost planet 1 at 7:25; seed 3 won planet 1 at
  5:42 then lost planet 2 at 2:29. Both wins are short of 8 minutes.
  Its own loop went once #567 landed (11-20): `runbot=SEED[:TO]` is now
  short for `league=run:SEED:TO` with `runbot::PLAN` (or `plan=`), and
  each PR's planet 1 test plays `league::play`. One bot is left; the
  `bot_hand` stays only for the gun lab. CI tests run on ubuntu-24.04
  since #572 (26.04's Mesa 26.0.8 hung the GPU tests on 11-20), with a
  40-minute limit (#568).

Job 5, branches (11-18): a thread cannot delete another's branch (the
proxy refused both `git push --delete` and the refs API), so on Scott's
card ("Let CI delete them", 13:17Z) `.github/workflows/branches.yml` runs
`scripts/branches.sh` after each landing (the queue dispatches it),
weekly and by hand: it deletes branches whose tip is in main and lists
the rest with the commits main has no copy of in the run's summary, for
Direction. Its first run (11-18 13:52Z, actions run 44317767827) deleted
91 (a session's shallow clone had found 18) and listed 56 not in main,
most of them 11-10 to 11-12 runner and census branches with one commit
each. Job 5's size cap is in (11-20): `wiki/lint.py` fails a pull request
that adds over 12 MB of pictures, films or sound outside `assets/`, or one
file over 3 MB, unless a commit line says `Big-media: <why>`; the 60 PRs
before it added at most 10.1 MB. Still open: moving the old films to an
archive branch. It shrinks the checkout, not the clone (history keeps them,
1.6 GB of .git on 11-20), and it breaks every report's links to its
films, so it waits for Direction to want it. The lint caps were already in.

Job 6 (11-20, no Squad and arsenal thread was running, so the coordinator
said go): the kit and the army's gun cards live on the library's `Game`
(`Game::kit`, `Game::rounds`, `larder_world/gear.rs`) and run at the end
of every `finish_step` (`Game::gear_turn`), so the library's tests, the
census and the bots cast cards and restock tools as the window does. The
window copies its selection into `Game::carry_full` each frame (selected
soldiers restock to the full carry). `Game::kit_out(&Run, RoundRules)`
sets a planet up from its run (tool prices, army round and budget, turret
deck); the caller still arms the gun. Kit or rounds calls that need the
game too go through `Game::with_kit`, `with_rounds` or `use_tool`. Two
small differences: a step a command finished runs its kit and cards
before the command, not after; the range's tools now cost what the run's
"tools+" level says. The run itself (`Run`) stays on the window's `App`
and the menu, since it lives between planets; `league::whole_run` drives
it for the bots.

After #595: #604 counts soldiers a falling star kills (`Game::fell_starfall`)
and the squad's own blasts' (`blasted_end`) in the league line; the bug
hunt's grid finding is fixed (a bot's coarser `mould_grid` ask is added up
from the summed grid and no longer moves the GPU's square, so bots' games
see play's grid; the league numbers shift from that commit on).

Job 7: `scripts/pc-batch.ps1 -Timing` is Milestone 1's batch (planets 1
to 3 of run 1 in the squad mode, `hand=strat:rush,aim=6`, each played 300
game s twice, each pass at 120 s, the view at zoom 6 and 12 on the larder
via `centre=larder,ZOOM`). It needs the PC to itself: the strategy
thread pauses its league worker (a PAUSE file in slime-league, or the
window in its `queue/pc-pause.txt`), and the script refuses while a league
game runs. Run `-DryRun` first, then `-Timing`; commit the m1-*.txt
files whole and write the report from them.

Job 7 ran 11-21 00:49-01:16Z on main 71af78c3
([report](../Reports/pc-batch-1070-m1-2026-11-21.md); raw on branch
`runner-out-1070-m1`): every frame under 7.2 ms p95 on planets 1 to 3, the
view's base pass 3.2-3.5 ms of it, planet 3's step frames 10 ms p95 from
the CPU half of a step; four fire twins need card tolerances (sent to
Enemy). Gotcha: the script's 2 GB free-RAM check failed at 2.00 GB with
Chrome open; Scott closed it.

Planet 3's CPU step (#624, 11-21): `bench=` now prints the CPU step by
part (`Game::costs`, filled by `Game::costed` only in the bench). The
once-a-second set-slime mask (`mark_bulwark`, Enemy's file) was 2.94 ms of
it on llvmpipe; a two-pass box sum made it 1.19 ms. The next spike is
`pump_maps` (paths, Squad and arsenal's), 4-6 ms longest, 0.09 ms mean:
reported to them.

The follow-up batch ran 11-21 04:13Z on c241fd7e (`-Followup -Timing`;
raw on branch `runner-out-1070-followup`; numbers in the M1 report's
follow-up section). Planet 3's step frames dropped from 10.1 to 6.8 ms p95,
all six twins pass on the card, and the view variants went to the Look lane.
The first try skipped timing on one 7% GPU-load sample, so #631 retries the
idle check.

Exit test 1 in CI (Direction, 11-21): `run-bot.yml` plays planets 1 to 3
as one run with the shop between; #639 adds `ended=` to the league line
(goal, held, squad, overrun, stuck, cleared, game, open) and a
`run_bot_summary.py --check` step that fails on a soft lock (no verdict by
the clock and a minute, no end line) or a planet under a minute. The plan
is the strategy thread's `bot,want=torch.standard|rush,aim=6,cards=0`
(11-21), kept equal to `runbot::PLAN` by a test; their #640 makes siege=3
the bot's default.

With #640's siege=3 the curve wins planets 1 to 3 on all three seeds
(every curve since e4db42d8, 11-21 06:18Z): planets 2:48 to 7:54, runs 12
to 17 minutes, every planet by its goal. Lengths went to Direction (all
short of 8 to 12; the bot's rush is a floor).

Pace by planet (Direction, 11-22): `run-bot.yml`'s `pace` job runs
`run_bot_summary.py --pace` over the three seeds (minutes, end, soldiers
lost, median by planet), and `scripts/play_log_summary.py` writes the
same from Scott's play logs on the `play-logs` branch. Reading them found
that a planet won in the step finished on the GPU wrote no `end` line to
the play log (21 of 30 logs); `log_end` in main.rs fixes it. The 11-22
table went to Direction: on 3d40c8eb the bot wins planets 1 to 3 in a
median 5:31, 4:47 and 4:51; Scott on 11-20 and 11-21 builds won planet 1
in 10:11 and 11:11 (2 lost, 2 quit) and planet 2 in 20:56 (2 lost).

The arms batch (#688, 11-22): `.github/workflows/arms.yml` plays
`scripts/arms-queue.txt` (`ARM DEPTH SEED SECS [GAME ARGS]`) from a pull
request that changes the queue, three shards with an 18 minute budget each,
rows in the job log (`ARMS arm=...`). First batch, planets 4 and 5 with the
heart or the strikers off: only seed 1 finished (planet 5 took 19 to 24 real
minutes for 12 game minutes on Mesa; about 90 runner minutes). Direction kept
the FIRST_PLANET rows; planet 5's size waits for Milestone 2's eight-planet
curve. Run planet 5 and later on Scott's PC, not here.

**2026-11-24, Milestone 2.** Save and resume A merged (#757:
`larder_world/snap.rs`, `gpu/larder_world/snap.rs` `snap_fields!`, the
window's `resume.rs`; `planet.sav` beside `run.txt`; checked on the 1070,
7.7 MB in about 50 ms). #765: the run bot's planet 1 stall (the siege's
attack-move stood in the body slime; it now closes in by plain moves),
Fireproof for fire decks, Direction's pause line, notices for bincode.
The curve by planet is `Reports/curve-2026-11-24` (two passes, 12 and 13
of 24 won); the third pass waits for Enemy's planet 4 and 7 changes
(Direction 06:48Z). `Run::played` (real seconds playing, saved in
run.txt, `App::count_played`) is for Look's run-time screens (test 2).
Open: Enemy's sense-pass timing on the 1070 when their branch comes.

**2026-11-24 afternoon.** The walker sort merged (#789, Milestone 2
test 5). `src/gpu/sort.wgsl` and `sort.rs` run a stable GPU counting sort
by 8x8 tile once a game second, in `GpuWorld::frame_start`. The key is
`sort=off|pack|tile` (`Settings::sort`), default tile. On the 1070 at 4K
on planet 8 peak, the GPU step went from 12.29 to 5.60 ms and the 50%
detail frame runs at 16.1/16.6 ms, against 19.7 ms. Tests that compare
slot by slot play with sort off: `twin_settings` in
`gpu/larder_world.rs` and the bomb twin in `mouldgpu`. A new per-slot
word in `engine.wgsl` must join `Sorter::new`'s list. #782 fixed the
league's clock (`Game::time()` already counts from the landing). Batch c
is in `Reports/curve-2026-11-24`: brace gave no gain.

The run bot's `storm` dial (default 0.5 from 11-24) lets the siege start
once the slime is at half its landing size and the squad holds 40.
Batch r numbers are in the field's doc. Planet 7 map B (squad stuck at
13, income about 2,100 in every arm) was the bot's crystal pick, not the
map: out of home it went for the biggest crystal even under the slime's
whole body, and gave a far crystal up on the road after 30 s. The pick
now takes crystals under `per` walkers a soldier first. A crystal's 30 s
clock waits while the squad closes in, and one the squad stalls short of
is left for ten minutes. On lavapipe, map B went from a loss at 4:45 to a
win at 5:09 (income 4,425). `size=F` (window argument: `league=`, `run=`,
the gun lab) multiplies a run level's start walkers, slots and surge food
(Enemy's planet 8 lever). #805 merged.

**2026-11-24 evening.** Batch t (the fourth curve pass, six maps, 54
games) is in `Reports/curve-2026-11-24`: 36 of 48 won, 21 of 24 on the
owner's maps. The new pick's cost isn't measured (p 0.55), so it stays.
Planet 8 at size 1.4 reads easier only because the storm scales with the
start size. Batch v (main 9d3f19aac, planet 8, size 1.0 against 1.4 with
`storm=0`, six maps) is queued on the PC after batch u (Enemy's heart-snipe
baseline, 63 games on c1180812d) and the Lab's fix2. Next: report batch u
to Enemy, then batch v; after Squad's #810 lands, add the builder arm
(`plan=bot ... ` then rush with `pumps=1,guard=1,hunters=1,body=grenade,blast=40`)
to every curve pass, CI's plan unchanged. The next 1070 timing batch also
times the HUD canvas at 4K, ui=125% (Look's ask).

**2026-11-24 evening.** #829 merged (`clock=480` default). Sent to
Enemy: y2 (planet 8 size stays 1.0; snipe breaks P6 0/0/2, P8 0/0/1), z4
(planet 4 siege at armour 15: 5 of 6 against 4, so P4 goes to 15 in
Enemy's D7). Sent to Squad: batch x (builder arm 17 of 24). This change
fixes the bot calling off far ghosts (stuck squad on P7 map B). On the
PC: y3 (P5 snipe and siege, as-is vs armour 15, main 0e01b47). Next: the
fifth curve pass once Enemy's D7 PR lands (main arm and builder arm, six
maps); the HUD canvas and landing-card timings in the next 1070 batch.

**2026-11-24 late.** Merged: #811 (curve batch t, dials `army=`,
`clock=`), #816 (`hand=origin2`: the D7 heart baseline's hand closes in
on the origin; the origin line prints `deepest N of M`), #826 (the origin
line prints its armour and size). Batch w: `clock=480` is the bot's
default (24 of 30 against 14). Batches u and u2 (Enemy's D7 baseline) are
sent; y1 (D7 arms on planets 4 and 5) and x (Squad's builder arm) are on
the PC; y2 (planets 6 and 8 on origin2, and batch v again) waits for
Enemy's #824. Bot batches on the PC wait while `gpu-timing.lock` exists.
1070 games are not repeatable: same-code arms split after 5 to 6 minutes.

**Next (11-22):** the next 1070 batch times the
view's `full` after the Look lane's #638 (plant crowns skipped under the
slime) and sends them the saving against 3.42 ms; `pump_maps` spikes
(4.4 ms) if Squad and arsenal hand them back.
