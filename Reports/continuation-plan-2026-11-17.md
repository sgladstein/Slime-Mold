# The plan for continuing the project (2026-11-17)

**What this is.** The plan the owner asked for on 2026-11-17 16:58Z ("review
the project, repo, handoffs, roadmap, everything else and develop a plan for
how to continue the project including how to split the work among threads.
They do not have to follow the recommendations from the handoff ... it
should form its own independent opinions and suggestions and new directions
and project workflows") and approved the same day at 19:07Z ("1) Yes"). It
was written by the planning thread as a Claude Doc
(https://claude.ai/artifact/WofUMAkZhp9EzaYYmRcfN9); this is a copy of that
doc as it stood at 19:15Z, addressed to the owner ("you"), kept here so it
outlives the page. The one question it lists as still open, the CPU twin
(D8), he answered by card at 19:21Z: GPU only for new rules, today's CPU
engine kept. Its one diagram (three milestones, two gates, the lab
alongside) is not copied.

**What holds now.** [`NOW.md`](../NOW.md) is the live front page and
[`CLAUDE.md`](../CLAUDE.md) carries the rules; the owner's answers are quoted
in [`wiki/decisions.md`](../wiki/decisions.md) and
[`wiki/decisions-game.md`](../wiki/decisions-game.md). The lane briefs
written from this plan are each lane's `handoff/<lane>.md`. Numbers here are
the planning thread's, one seed each where it says so; they are looks, not
measurements.

---

## The short version

Finish a small piece before widening the game: make planets 1 to 3 of the
run a 30-minute sitting you want to replay and like to look at, built by
five standing threads instead of fifteen. Whenever you have time to play,
the game itself tells you what is new, what is on or off and what is worth
trying, and no thread asks you for a playtest.

The reason is that building has outrun what your play time can cover. 445
pull requests merged in eight days. You have playtested many times and your
notes steered most of that work, but threads cannot see when you played or
on which build, and when you sit down to play nothing shows what is on, off
or new. 14 features sit behind command-line switches, and about 30 picks and
playtests are listed as waiting on you.

- **Keep:** the run in squad mode as the game, fun over fidelity, the
  picture is the simulation, build first and balance later, the merge queue,
  and twin tests for anything that is a default.
- **Change:** one milestone with exit tests, five lanes that each own files,
  a standing Look lane as art director, a one-screen front page in place of
  HANDOFF and ROADMAP, every feature on by default and listed inside the
  game, a record of each play session that threads can read, a script that
  plays whole planets, and a Windows build you download instead of compile.
- **Stop for now:** new gun card families, new mini-boss kinds, new turret
  bodies, planet polish, and everything already frozen.
- **New directions:** the look worked in order of where your eye is,
  starting with the fight round the squad, the origin at the slime's start
  crystal as the way a planet is won, steering the slime as the game's
  signature verb, and a decision on how long a run should be before the
  later planets are tuned.

The plan is built on your four answers of 11-17: a game you love to play
with release as a maybe, a slice first plus one lab thread, about five
standing lanes, and threads that decide small choices and ask you the big
ones. It was revised the same day after your three notes: you have
playtested many times, you cannot tell what is on or off when you play, and
the first draft gave the visuals no direction. You said go at 1:07 PM your
time on 11-17.

## Where the project stands

The run in squad mode sits on a strong engine and looks best from far out.
You have played it many times. What is missing is a record of that play that
threads can see, and a way for you to see what a build holds.

| Measure | Today |
|---|---|
| Age | First commit 11-07; its own repository since 11-10 |
| Pull requests | 445 merged, none open, no issues |
| Code | 107,600 lines of Rust and 8,300 of shader in `src`, 20,000 of probe tools, 712 tests |
| Records | About 270 reports, 7,305 files, 1.4 GB; a fresh clone downloads 1.2 GB |
| Speed on the GTX 1070 | Planet 1 at 4.6 ms a frame (p95), the core world at 12 ms |
| Cloud build | 2 min 13 s from clean; 120 game seconds film in 50 s with no GPU |

### What I saw

I built main at `d08583d` and filmed run seed 1 with the scripted hands on a
software GPU. Each is one seed, so these are looks, not measurements.

- **It looks like a game.** The HUD is one clean row, the crystals are
  crisp, and at map scale the slime reads as one living body with cords
  between its crystals. Up close in a fight it is weaker, and the section on
  the look below says where.
- **Planet 1, the rush hand** (drain crystals, buy soldiers): 11 soldiers at
  0:20, 21 at 1:00, 42 at 2:00, health 97%, nothing touching the squad. By
  4:00 every free crystal is drained dry, income is 0, and the slime (11,500
  walkers) sits in its rock bowl.
- **Planet 1, idle:** by 2:00 the slime has taken the big crystal and killed
  one of four soldiers, and by 5:00 all four are dead and the slime has
  doubled to 26,000 walkers. Idle loses, as it should.
- **Planet 3, the same rush hand:** the Trunk's cords cross the map, and at
  2:00 the squad is down to 1 soldier with 21 lost.

So the enemy presses when left alone and on planet 3, but on planet 1 the
rush you described on 11-15 still works. Between planets 1 and 3 the same
plan goes from untouched to wiped, and nobody has drawn that curve.

### Strengths

- The enemy is unlike other games' enemies: its body is its health, its food
  is on the map, and it reshapes the ground it fights on.
- The engine is fast and tested, with exact CPU and GPU twins in CI and a
  merge queue that tests what lands.
- You play it often and answer concretely, and threads act on your notes
  within hours. The decisions pages hold 73 dated entries of yours from
  11-14 to 11-17 alone.

### Risks, most serious first

1. **Your play is not visible to threads, and their work is not visible to
   you.** You have playtested many times, but nothing records when or on
   which build, so six of the nine area handoffs still list something as
   waiting on your playtest or pick, and the run's wiki page still says "not
   yet played by the owner". In the other direction, when you sit down to
   play nothing tells you what changed or what to try, and threads ask for
   playtests one at a time, at moments you often cannot give.
2. **Features land switched off and out of sight.** 14 switches are built
   but off, each waiting for a pick. They are set by command-line keys and
   none is on the Options page, so in play you cannot see what is on or off.
   Eight of them were added on 11-17, the day six others were deleted.
3. **The look has no owner and no target.** You asked on 11-14 for a thread
   to review all graphics and manage that work. The style guide it wrote is
   good, but look work has since been split across feature threads, and the
   part of the picture you watch most, the fight round the squad, is the
   least resolved.
4. **No script plays what you play.** The only run tests play 60 game
   seconds of planets 3 and 6, so a planet that breaks or stalls waits for
   you to find it. Planet clocks run from 15:00 to 55:30, so a whole run may
   take hours.
5. **The records are too heavy to steer by.** ROADMAP is 98 KB, the Larder
   wiki page is 59 KB against the wiki's own 12 KB cap, and the report index
   is 183 KB.
6. **Three taxes on every future change.** The run's state, the gun cards
   and the tools are held by the window program outside the game's rules, so
   library tests and replays play a different game. Every slime rule is
   written twice. About a fifth of the source is frozen games that still
   build and test on every change.
7. **Usage.** The old account ran out mid-task, cutting off the Trunk's
   cords and the Provoke and Still cards.

## Where I agree and where I diverge

I agree with the rulings about what the game is and how to judge it. I
diverge on what to build next and on how the work is run.

| Subject | Handoff and roadmap | My view | Why |
|---|---|---|---|
| The game | The run in squad mode; campaign and RTS control frozen | Agree | It is the game you play and judge by (11-15) |
| How work is judged | Fun over fidelity; the picture is the simulation; build first, balance later | Agree | They produced a game that looks alive, and numbers are cheap to retune |
| Next for guns | Finish Provoke and Still, then step E (device slot) and step F (luck cards) | Finish Provoke and Still, then pause new cards | More than a dozen card kinds landed in two days; how the gun looks and feels serves "guns are a draw" more than a wider catalogue |
| Next for the enemy | Trunk cords, Mortar and Pinwheel mini-bosses, the origin | The origin, plus pressure on planets 1 to 3 | The origin gives every planet a climax; more boss kinds can wait until you have said how the four built ones play |
| Next for turrets and planets | The next turret body; twists and rescue planets | Park both | Neither serves the first three planets |
| The rush brake (D7) | Held: "play first, no changes yet" | Your call stands; it is first on the list of things to try | My film today: the rush reaches 42 soldiers at 2:00 on planet 1 untouched, and the same hand is wiped on planet 3 |
| Built-but-off switches | Each waits for its own owner pick; 14 rows today | Features land on. At most three experiments are off at once, each listed in the game with a toggle, and each ends at your next play: on, one tier's trait, or deleted | You cannot test what you cannot see, and each switch doubles a code path |
| Look choices | Blind A/B pages with the key hidden | The Look lane decides small ones; a blind A/B only for a choice that sets the whole art direction | Two recent A/Bs showed you no visible difference (spray share, fen moat) |
| Graphics work | An art director thread, with look passes inside each feature thread | One standing Look and interface lane that owns the picture, the style guide and a ranked list of look jobs | You asked for this on 11-14 and called the visuals an important factor on 11-17; today no thread owns the look shader as a whole |
| Playtests | Each thread asks for one when its work is ready | No thread asks. The game shows what is new and what to try whenever you open it, and keeps a record of the session | You play often, but not on a thread's schedule, and threads cannot tell that you played |
| Reviewers | Two standing reviewers and an integration thread | One direction lane; outside reviews at milestones only | The 11-15 review itself found that reviews set more of the backlog than you did |
| Front pages | HANDOFF, then ROADMAP | One screen, `NOW.md`; archive both | Together they are 121 KB, part of HANDOFF is still the paused dish game, and 42 page links in them died with the old account |
| Records per change | A report, its index line, the pages list, a wiki page, a ROADMAP row | A report only for a measurement others will cite, a dead end, or a page for you | The rule is sound, but its cost is paid on every pull request |
| Slime rules on both engines | Every rule written for CPU and GPU with an exact twin | GPU first; write the CPU twin when a rule becomes a default (the open D8) | The game plays only on the GPU, and slime rules are your top ask |
| GTX 1070 timing | Booked per pull request | One batch per milestone | Planets 1 to 5 run at 4.6 to 5.9 ms p95 against a 16.7 ms frame |
| Win rule | 1% of the slime's peak mass | Kill the origin (your 11-16 idea) | The threshold moved from 25% to 2% to 1% and still ends in mop-up |
| Pull requests | Never assign you or request your review | Agree, and it needs restoring | The new project's default tells threads to do both, so the rule must go into the project instructions |

## Directions, in priority order

Each milestone ends in something you play, and nothing outside the current
milestone is built except by the lab thread.

*(Diagram in the doc: roadmap · 3 milestones, 2 gates, the lab alongside.)*

Only Milestone 1 has a date. The gate into Milestone 2 is the six exit tests
below, and Milestone 3 is drawn dashed because it waits on your choice to
release.

### Milestone 1: Three planets (about one week)

Planets 1 to 3 as one sitting of about 30 minutes. It is done when all of
these hold:

1. A scripted player (the run bot) lands, pumps, defends, wins and shops
   through planets 1 to 3 without a soft lock, each planet in 8 to 12
   minutes, and so can you.
2. The fight looks good in motion at zoom 6 to 12 on your PC: shots, hits
   and the slime's reaction are clear, and a clip of any minute is one you
   would show someone.
3. The rush is not the best plan. On the bot's paired seeds, drain-and-buy
   does not beat a plan that builds pumps and defends them. How to get there
   is your D7 call once you have played the new build; the brakes your
   rulings allow are prices, the slime contesting what you drain, and things
   worth buying.
4. After playing, you can say in a sentence what each of the three slimes
   did to you.
5. The starting rifle feels good to you, and one shop visit changes how the
   gun plays.
6. Everything built is on in the default game, and the game's own pages list
   what is new and the few experiments that are off.

### Milestone 2: A whole run (about two more weeks)

- **The origin.** Each planet is won by killing the body at the slime's
  start crystal. It replaces the 1% rule, gives the strike a target and
  gives the planet an ending.
- **A chosen run length.** Today's clocks add up to 3 h 43 min for eight
  planets, which follows your 11-14 note of "15/20 minutes to 1 hour+" a
  level. Play the first three planets, then decide whether that is the game
  or whether a run should fit one evening.
- **The curve.** The bot plays all eight planets and draws what nobody has:
  time to win, soldiers, income and losses by planet.
- **Save and resume**, which a run of that length needs.
- **The core world at 60 frames a second** on the GTX 1070. It is at 12 to
  14 ms a frame today.

### Milestone 3: Someone else can play it (only if you choose a release)

A downloadable build, a first five minutes that teaches, a sound pass, and
two or three outside players.

### The look, in order of where your eye is

I filmed the fight again at zoom 6 and 14 for this section. These are stills
from a software renderer, so motion on your PC and your own eye overrule
them.

**What holds up:** the map from far out, where cords linking the crystals
read at a glance, and the crystals, the ground and the HUD. The style
guide's rule of living light on dark ground is right and stays.

**What does not yet, ****in the order you set on 11-17****:**

1. **A. The fight round the squad.** The slime's flinch field draws as a
   lime haze in cell-sized blobs over ground and slime alike, and it is the
   largest thing on screen exactly where you aim. With that field off, the
   same frame is clean orange on dark. Tracers are thin dotted lines, and
   the flung droplets are outlined shapes in a different style from the
   slime they come from. The squad is part of this: at 42 soldiers it is one
   overlapping pile that you cannot count, and sprite A, which you chose, is
   lost in it.
2. **B. The slime itself.** The thick body is a bumpy glossy sheet with no
   veins or flow to be seen in a still, and loose specks read as glowing
   pebbles. The style guide still lists the pebble frontier as open.
3. **C. Planets and terrain.** Rock is flat grey slabs and a drained crystal
   is a flat grey hexagon, and those are the largest quiet shapes on most
   screens. Shadows exist but are too faint at play zoom to seat anything on
   the ground. The style guide still lists the flat spent crystals as open.
4. **D. Menus, shop and screens.** As far as I can see, landing on a planet,
   winning it and losing it are marked by cards and alert lines more than by
   the picture. I have not judged the shop and menus closely, and doing that
   is the first job when the lane gets here.

**What I would do, ****all four in that order.** A is Milestone 1's look
test. B starts once A passes your eye, whichever milestone that falls in,
and C and D follow the same way.

- **A. The fight round the squad.** Draw the flinch as a change in the
  slime's own colour, only where there is slime. Make a shot a short bright
  streak you can follow, and a hit the slime's own reaction: the hole it
  opens and the walkers it flings. Keep every effect no bigger than its
  damage, as you ruled on 11-14. Draw droplets in the slime's material.
  Space the squad so that 40 soldiers can be counted. Put camera shake and
  hit-stop (D6) in the game as a toggle to try.
- **B. The slime itself.** Veins and flow inside the thick body, loose
  specks that read as parts of one body, and a look for the origin worth the
  climax it is.
- **C. Planets and terrain.** Stronger grounding, materials for rock and
  spent crystals, and one visual trait per planet that only it has, drawn
  from its own fields.
- **D. Menus, shop and screens.** A landing, a win and a loss that happen in
  the picture, then the shop and menus, and a title screen if you release.

**How look work is run:**

- **One owner.** The Look and interface lane is the art director you asked
  for on 11-14. It owns the look shader and the style guide, keeps one
  ranked list of look jobs, and checks every other lane's visible change
  against the guide before it merges.
- **Judged in motion, on your PC.** A look change carries a short before and
  after clip in the game's list of what is new, and you see the real thing
  whenever you next play. Stills alone decide nothing, as the style guide
  already says.
- **The picture stays the simulation.** Every item above draws from a field
  or a game state, so none needs a painted effect.
- **Small choices are the lane's, the direction is yours.** The lane decides
  strengths and colours and you veto in play. You set the order on 11-17:
  all four are explored, the fight round the squad first, then the slime
  itself, then planets and terrain, then menus, shop and screens.

### The lab: one thread for new directions

The lab prototypes outside the milestone and shows you a playable clip,
never a switch in the main game.

Its first brief, if you agree, is **steering as the signature verb**. The
slime follows fields, and today the player's tools write almost only damage
into them. Lures that visibly draw, walls that funnel, bait and false trails
would let you herd the slime into a kill zone. You raised that idea for the
shove cards on 11-16, and it is the one thing no other game's enemy allows.

Later briefs: a second slime species that fights the first, the player's own
mould (parked today), and the forest floor.

### Parked until Milestone 1 passes

Gun steps E and F, the Mortar and Pinwheel mini-bosses, the next turret
body, twists and rescue planets, the Trunk's cords, further planet polish,
and everything already frozen (campaign, RTS control, player AI, the dish
games).

## How to split the work

Five standing lanes, as you chose: each is one long-lived thread that owns
named files. A lab thread and runners join on demand.

| Lane | Owns | First jobs |
|---|---|---|
| **Direction** | `NOW.md`, the game's lists of what is new and what to try, the record of your play sessions, `wiki/decisions*.md`, the tuning numbers (prices, clocks, rewards, slime sizes) | Sort the pending picks (table at the end); write the first list of things to try; turn your play notes into lane jobs within the hour |
| **Enemy** | The slime's rules (`big.rs`, `engine.wgsl`), `tiers.rs`, `bosses.rs`, `raids.rs`, `shapes.rs`, the ladder in `run.rs` | Make each of planets 1 to 3 do one nameable thing to you by minute 2; turn the 11-17 slime switches on or delete them; then the origin |
| **Squad and arsenal** | `squads.rs`, `steer.rs`, `paths.rs`, `cmd.rs`, the window's input and squad mode, `rounds.rs`, `guns.rs`, `bullets.rs`, `kit.rs`, `shots.rs`, turrets in `buildings.rs`, the gun sound | Finish Provoke and Still; whatever your play notes say gets in the way of control; shop offers that give planets 1 to 3 distinct builds |
| **Look and interface** | The look shader `view.wgsl`, the sprites and marks, `world/ui.rs`, `world/start/`, the window's menus and HUD, the style guide in `wiki/looks/` | The fight round the squad; a squad you can count at 40; the Experiments tab and the page of what is new; win and lose screens |
| **Engine and tools** | `gpu/mod.rs`, the window's `main.rs`, `headless.rs` and `hands.rs`, CI, scripts, the wiki lint | The list of switches in code and a build number; the run bot; the Windows build; the play record; move the run, cards and tools into the library; the lint cap on `NOW.md`; media off main |
| **Lab** (one at a time) | Its own example program or branch, nothing in the main game | The steering prototype |
| **On call** | World (planets, plants), the PC runner, Haiku runners | Only when a lane asks |

The rules that make the split hold:

1. **One owner per file.** A lane that needs a change in another lane's file
   sends a small pull request and tells the owner through the coordinator.
   ROADMAP's table of one owner per mechanic did this for 15 threads; per
   file is enough for five.
2. **Two shared shaders.** Enemy owns `engine.wgsl` and Look and interface
   owns `view.wgsl`. Other lanes change only their own passes there and say
   so in the pull request title. Free parameter words and bindings are
   listed in a comment at the top of `gpu/mod.rs`, not on the roadmap.
3. **Each lane keeps ****handoff/<lane>.md**** current** after every merged
   batch: state, open branch, next step. The 11-17 handoffs showed the
   format works, and it lets a thread be replaced without loss.
4. **One queue to you.** A lane answers you directly when you write in its
   thread. Questions a lane wants to raise go to Direction and onto the
   game's list of things to try, so no thread asks you for a playtest or a
   pick on its own schedule.
5. **Models.** Which model each lane runs is your call. To hold usage down I
   suggest the strongest model for Direction and for milestone reviews, a
   mid model for the four build lanes, and Haiku for runners as before.

## Workflow

You play when you have time, not when a thread asks. So the work is arranged
to be ready for you at any moment, and to learn from a session without a
report from you.

1. **Lanes build, and everything lands on.** A change is done when it has
   tests, is on in the default game, has a 10 to 20 second clip, and has one
   line in the game's list of what is new: what you will see and how to try
   it.
2. **You play whenever you like.** The start menu shows the build number,
   what is new since the build you last played, the few experiments that are
   off, each with a toggle, and at most five things worth trying, the most
   important first.
3. **The game keeps the record.** It already writes a play log every game
   second. The log gains the build number, the switches that were on and the
   planets played, and a key in play drops a note at the moment you notice
   something. As you agreed on 11-17, `play.cmd` sends the log to a branch
   of the repository when you quit. Git on that PC has to be signed in to
   GitHub for that, and the script says so when it is not.
4. **Direction sorts** the log and your notes within the hour into lane
   jobs, tuning changes or "not now", marks what you tried as tried, and
   updates both lists for next time.

The rules behind the loop, each with its reason:

- **Nobody asks you to playtest.** A thread never ends on "awaits the
  owner's playtest". Its work is on, in the game's list, and done. Before a
  thread calls something unplayed, it checks the play record.
- **One list of switches, in code.** Every switch is declared once with its
  name, default, owner lane, date and one line on what you will see. The
  Options page gains an Experiments tab drawn from that list, the HUD shows
  a small mark when anything is off its default, and a test fails on a
  switch that is not declared. So the game says what is on, not a table in
  ROADMAP.
- **On by default; off is the exception and expires.** Off is allowed for a
  measured harm or for one side of a real either-or. At most three are off
  at once, and each ends at your next play after it lands: on, one tier's
  trait, or deleted with its dead-ends line. The 14 that are off today are
  settled in the first two days, as in the table at the end.
- **The list of what is new is part of the change.** A pull request that
  changes what you see or do adds its line to one file the game reads, and
  the lint fails when it does not. The game remembers the last build played
  on that PC, so "new since you last played" is right whether that was
  yesterday or two weeks ago.
- **One front page.** `NOW.md` holds the milestone and its exit tests, each
  lane's top three jobs, and the queue of what waits on you. The lint fails
  it past 100 lines. HANDOFF and ROADMAP are archived under `Reports/`.
  `CLAUDE.md` stays the only file every thread reads, and points at
  `NOW.md`.
- **Threads decide what can be undone**, as you chose. A lane makes look and
  tuning choices, states them in the game's list of what is new, and
  reverses them if you object in play. You are asked only about what changes
  the game's direction or cannot be undone.
- **Lighter records.** A report is written for a measurement others will
  cite, a dead end, or a page for you. Other changes carry their numbers in
  the commit message, as `CLAUDE.md` already asks. The wiki's 2 KB cap on a
  page's Now becomes a failure, not a warning; the run page's Now is 8.2 KB
  today.
- **Media off main.** Films and stills move to an archive branch or a second
  repository, with a size cap per pull request in CI. A fresh clone is 1.2
  GB today and has grown about 140 MB a day, and every new thread pays that.
- **A build you download.** CI builds the Windows game on every green main
  and `play.cmd` fetches it, falling back to a local build. You stop
  compiling 107,000 lines on an 8 GB machine, and other people can play when
  you want that. CI already has a Windows job, for the sound device.
- **Whole planets in CI.** The run bot playing planets 1 to 3 replaces the
  per-feature smoke checks: idle loses, nothing soft-locks, no planet ends
  in under a minute.
- **Reviews at milestones.** One outside review and one bug sweep before
  each milestone check. Every finding names the exit test it serves, or goes
  on a later list.
- **GTX 1070 runs in one batch per milestone**, or sooner when a change adds
  a pass over every walker or cell.
- **Pull requests** merge by the `queue` label as now, never assign you or
  request your review, and have their branches deleted on merge. About 140
  merged thread branches are still on the remote.

## The first week

Nothing here waits on a day you play. The build is ready to play from day 2
and stays ready, and the milestone check happens at your first play after
the bot's tests pass.

1. **Today.** You answer the short list in the last section. The coordinator
   starts the five lanes, each with a brief written from this plan. The
   project instructions get the pull request rule back.
2. **Days 1 and 2: the build tells you what it holds.**
   - Engine and tools: the list of switches in code, the build number, the
     Windows build, and the run bot's first version (planet 1, to a win or a
     loss).
   - Look and interface: the Experiments tab and the page of what is new in
     the start menu, then the flinch haze.
   - Enemy: the 14 switches settled as in the table below, with far smell
     and charges round walls on by default, and one still of each of planets
     1 to 3 showing what it does to you.
   - Squad and arsenal: Provoke and Still filmed and merged.
   - Direction: `NOW.md` written, HANDOFF and ROADMAP archived, every
     pending pick sorted, and the first list of things to try.
3. **Days 3 to 6: the fight and the three planets.** Look and interface
   works down the look list: shots and hits first, then the squad. The bot
   plays planets 1 to 3 on paired seeds and Direction posts the curve. Lanes
   act on whatever you noted, whenever you played. The lab shows its first
   steering clip.
4. **Day 7, or your first play after it: the milestone check** against the
   six exit tests. The bot answers the ones it can, and you answer the rest
   in play. If they hold, Milestone 2 starts. If not, the misses are next
   week's jobs.

## Decisions that are yours

You answered these on 11-17. Three things stay open, and everything else in
the queue the lanes settle.

### Decided on 11-17

- **Go.** The five lanes and the lab start on this plan (1:07 PM your time,
  "1) Yes"). Their briefs are in the project's shared files under
  `lane-briefs/`.
- **Windows only for now (D9)**, and **your own mould stays parked (D12)**.
- **The dish-era decisions on HANDOFF (2 to 8) are closed** as not needed
  while paused.
- **Never assign you to a pull request or request your review.** The
  coordinator has added it to the project instructions.
- **Old branches whose work is on main are deleted, and branches are deleted
  on merge.** The repository switch "Automatically delete head branches" is
  yours to turn on.
- **Films and stills live on an archive branch** of this repository.
- **The look is explored in your order:** the fight round the squad, the
  slime itself, planets and terrain, then menus, shop and screens.
- **Play logs:** `play.cmd` sends them to a branch when you quit.
- **Models:** Fable for the main threads, Opus or Sonnet for simpler tasks,
  Haiku for runners, at medium to extra high effort by task.

**Still open:**

- **The CPU twin (D8).** You asked whether it is needed at all if rules
  start on the GPU. My answer is in the thread: new slime rules on the GPU
  only, today's CPU engine left as it is, and its removal decided at
  Milestone 2. The card in the thread has the choices.
- **The lab's first prototype.** It starts on steering the slime, my
  recommendation, because that question had no answer. Say so if you would
  rather it start elsewhere.
- **Later, in play:** the rush brake (D7), which you are holding until you
  have played more, and the origin as the win rule and the run's length at
  Milestone 2.

### What I would let the lanes settle

These are the picks and playtests waiting on you today, with what I would do
with each under "threads decide small, ask big".

| Waiting on you | From | What I would do |
|---|---|---|
| Far smell and charges round walls (`play-new-slime.cmd`) | Simulation-use | On by default now, you veto in play; delete the second script |
| Setting, block or trap, crust pull, fed slime fireproof (four blind films) | Simulation-use | Enemy picks one configuration and gives it to one deeper tier as its trait. The films exist if you would rather watch them |
| Set slime's look, glaze or amber | Simulation-use | Look and interface decides |
| Heave and Corral as one card | Gun cards | Squad and arsenal decides; I would merge them |
| The Mind bullets quiz and the crust "name the ground" quiz | Gun cards, simulation-use | Drop both quizzes; play shows whether they read |
| Haze strength, flinch strength | Slime enemy | Look and interface redraws the haze; Enemy sets the flinch |
| Mini-boss colour (Violet, Ember, Ink) | Mini-boss look | Look and interface decides |
| Fen moat or rock rims | Planets | Delete the moat; you could not see the difference |
| Camera shake and hit-stop (D6) | Gun feel | An Experiments toggle, first on the list of things to try |
| Specks amount, mortar strength, the size of what each planet does to you | Slime, turrets, run | Tuning notes from your next play |
| Run pacing, shop prices, the breather planet | Run | Your next play and the bot's curve |
| Sound levels, plants | Sound, plants | Your next play |
| The next turret body, one deck per body | Turrets | Parked with the turret work |
| Last stand, reserve recruit, lossy flow, lone step | Raids, Larder, one body | Delete reserve, lossy flow and lone step; last stand becomes a deep tier's trait or goes |
| Crystals that refill | Larder | Stays off; it belongs to the D7 answer |
| GTX 1070 timings (homing, Mind bullets, Grudge memory, plants, mini-boss lumps, crust and setting) | Several | One batch before the Milestone 1 check |

### What this plan does not know

- I have not played the game. I read the code and records and filmed
  scripted hands on a software GPU, one seed each, so my view of the look
  comes from stills and not from motion on your PC. The first draft also
  read thin records as little play. You have played it many times, and what
  you saw beyond the 73 recorded entries is not in this plan.
- Whether a 40-soldier squad still wins planet 1 is your report from 11-15;
  no scripted hand attacks in squad mode, so I could not film it.
- The one-week and two-week spans are estimates from last week's pace, not
  commitments.

