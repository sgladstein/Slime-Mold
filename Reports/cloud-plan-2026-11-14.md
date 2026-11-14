# Keeping Larder World moving without playtests or the PC (2026-11-14)

Asked by the owner (Scotttt, 2026-11-14 00:28Z): "I cannot playtest right
now and my gpu is busy. Start a new thread to review what every other
thread was working on and come up with a plan for how to proceed and keep
making progress." Written against `main` at `c56aadc` (#204), with no PR
open, from every thread's latest messages, the project memory, `ROADMAP.md`
and `HANDOFF.md`. This is a plan, not a measurement: no number here is new.

## The big picture first

1. **The game has outrun its playtests.** About 80 PRs have merged since
   the last combined playtest page (L-PT, 2026-11-12 11:10Z): shape C,
   shared food, withering, panic, bullets with pierce and split, rounds,
   your own mould, the siren, buildings anywhere, rations, aims that see,
   wading. Owner playtests ran on 11-12 and in short clips on 11-13, so most
   of these have not been played together. The owner's bar (11-10) is that the mould is
   integral and that several strategies counter each other. Only play can
   judge that. So the most useful cloud work now is to **make the next
   playtest short and decisive**, and to build things that do not need the
   owner's feel to judge.
2. **Everything is behind a command-line flag.** `mission=`, `own=1`,
   `rounds=`, `panic=`, `wither=`, `bullet=`, `energy_flow=` and the
   shifting keys (E, W, Y, C, H, K, O) mean the owner has to remember a
   list before playing. A native game needs a front door: a start menu, a
   mission list, toggles and a key card. Nobody owns this yet.
3. **The 1070 debt is growing.** None of 11-12 night's or 11-13's GPU
   changes has been timed on the card, and the 1070 twins for #181 and #198
   never ran (`/mnt/project-files/scott-desktop/booking.md`: four timing
   jobs, two twin jobs). CI's software Vulkan still proves correctness, so
   nothing is blocked. But when the PC frees up, the queue should be a single
   prepared batch, not six hand-relayed jobs.
4. **Several pieces the campaign waits on have landed.** Missions 4-8 need
   the Trunk, cut pieces that starve, turrets, the Sleeper and strike,
   pumps, lures and a rival pair. All but the rival pair exist on `main`.
5. **The two biggest files keep growing.** `src/mould/larder_world.rs` has
   3,636 lines and `src/bin/larder-world.rs` has 3,390. Almost every thread
   edits them, which is where the merge collisions of 11-12 and 11-13 came
   from. A quiet stretch with few threads working is the cheapest time to split them.

## What can move now, in the cloud

Each line names its owner and the smoke checks it needs (CLAUDE.md: idle
doesn't win, no game under a minute, no soft lock, frame cost). None is
gated on a census.

| # | Work | Owner | Why now |
| --- | --- | --- | --- |
| C1 | **Playtest pack**: one page "what changed since 11-12, what to try" on one `main` build. List each merged change with its key or flag, a 10-20 s film made headless (`larder-world look=DIR ...`, check the binary is current main first), and the count beside it. End with a 15-minute play script and the picks to make while playing (below). Archive under `Reports/playtest-pack-2026-11-14/` and list it in `Reports/pages.md` | integration thread (it owns L-PT) | the next playtest is the bottleneck. This turns it into one sitting |
| C2 | **Front door**: a start menu in `larder-world` (sandbox, campaign mission list, an options page for the flags above with today's defaults shown, and a key card on F1 or `?`). Flags keep working. Command ids go in the `commands!` table | **new thread** | no owner judgment needed, and it makes C1 playable without a flag list |
| C3 | **Campaign missions 4-7** (Cut the cord / Trunk, Kill zone / Swarmer + turret, The nest / Sleeper + strike, Spore season / Sporer + pump), each with two winning plans. Mission 8 (rival pair) after D4 | campaign thread | its pieces are on `main`. "Mission difficulty" is a named census reason, so keep each mission's check under an hour |
| C4 | **Strategy check on today's default** (shape C, panic, withering, bullets): a small plan-by-tier matrix (seeds 301-308, idle and random rows, about 45 min of runner time). It answers one named question: *does any plan dominate before the next playtest, and does anything beat walking in now that it is deadly?* Add the designer's wall-fort hand and a headless strike target (cut the body off its crystals, now that cut pieces wither and panic) | player AI thread | "dominance before a playtest" is a named census reason. The designer writes the counter table it is judged against first (D3) |
| C5 | **GPU correctness ports**: germ and gas on the GPU (toolkit tools and the gas and germ rounds, skipped on the GPU today). Twin-tested on software Vulkan, merged as "1070 twin pending" | squad toolkit thread, with the gun rounds thread | these are the only toolkit verbs the native game can't play |
| C6 | **One PC batch**: a single script that runs the booked jobs in order when the PC frees up: the twins for #198 (which also covers #181) and #183, then the timing jobs (crater countdown and 25k dish, own mould 5k/20k, shapes A/C/D, bullets 45 vs 0, plus panic and wither on C). Each prints the machine line and writes one output file. Leave the timing part off until the owner says go | integration thread writes it, tech lead reviews it | when the PC frees up, the runner gets one command, not six relays |
| C7 | **Sounds for what is new**: bullets in flight, a panic charge's tell, withering, the siren, toolkit bursts, and a fainter voice for each cut-off piece (the one-body piece id exists now). Record WAVs onto the Larder Sound page; levels wait for the owner's ears | sound thread | built from counts the game already has |
| C8 | **Make the new states visible from the data**: a panicking walker's lean and a withering piece should read on screen, drawn from walker and field state (no drawn-only effect). The ground's look from WM3's fields comes after | graphics thread | the playtest can't judge panic or withering if the player can't see them |
| C9 | **Split the two big files** by subsystem (economy, combat, buildings, missions; window input, HUD, recording), with no behaviour change and every test unchanged. Also delete the stray `src/bin/mouldgpu.rs.orig`. Run it as one PR in a merge-freeze window the integration thread books | integration thread | the cheapest time is now, while few threads are working |
| C10 | **Map editor (WM5)**: paint ground kinds, place crystals and starts, save to the WM1 map format, play. The ROADMAP lists its owner as "to be chosen" | **new thread** | designed maps are part of the plan (owner, 11-11 19:29Z), and the missions need more maps |

Order: C1 and C2 first (C1 can ship its page before C2 lands and point at
it later), C6 at any time, and C9 in a window when C3, C5 and C10 have no
branch open on those files. The rest run in parallel.

## What waits for a playtest

These need the owner's hands or ears. Pile them into C1's page, not into
separate pings:

- Withering plus panic (slime-enemy thread; [films](https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP)).
- Crater length, `crater=2` or `scorch_keep=0.5` (fight-feel thread; the
  owner was answering it there at 2026-11-14 00:26Z).
- Bullets and flight cards: the look and the feel (gun rounds thread).
- Your own mould (`own=1`), and the sound levels (never heard live).
- Whether shape C still draws as specks on the 1070 (owner, 11-12 14:28Z).
- Mission difficulty for 1-3, and 4-7 once built.

## What waits for the PC

- Every 1070 timing (C6's timing half), only after the owner's explicit go
  (standing since 11-12 ~21:00Z: compute is allowed, timing is not).
- The 1070 twins for #181, #183 and #198 (compute, allowed whenever the PC
  is on and quiet).
- Any GPU census at 25k walkers.

## Picks the reviewers and integration can settle now

Under the owner's overnight rule (11-12 04:48Z), the game designer, the
technical lead and the integration thread (tie-breaker) decide these, record
them in `ROADMAP.md` or `wiki/decisions-game.md`, and the thread goes ahead:

- **D1. Census harness default.** Should the census baseline move to
  shape C (the window's default since #174)? ROADMAP calls this the "census
  reset held". Recommended: yes, as one PR, with each thread re-running its
  baseline once. Decided by the tech lead and integration.
- **D2. flow_loss.** It stays off; withering is the owner's pick for a
  cut-off piece (11-13 15:53Z). Recommended: mark it parked in HANDOFF and
  the wiki, not deleted. Decided by the designer and integration.
- **D3. The counter table C4 is judged against.** Which plan should beat
  which tier on C: for example, walls and guns against the Swarmer, cutting
  against the Trunk, a strike against the Sleeper. Written by the designer
  before C4 runs.
- **D4. Mission 8's rival pair.** Two enemy species in species slots 1-2
  now, or swap mission 8 for one that uses what exists. Decided by the
  designer and the tech lead.
- **D5. The front door's scope (C2).** Menu, options and key card first;
  persistence and saves later. Decided by the designer.
- **D6. The freeze window for C9.** Decided by integration with the tech
  lead.

The crater-length pick is a look, so it stays the owner's (CLAUDE.md:
a visual choice is a blind A/B for the owner).

## Messages and threads

New threads (the owner pre-approved threads that agents suggest, 11-12 04:32Z):

1. **Add a start menu to Larder World** (C2).
2. **Build the Larder World map editor** (C10).

Messages to existing threads:

- Integration: C1 (playtest pack), C6 (PC batch), C9 (file split and
  freeze), D1, D2, D6.
- Campaign and onboarding: C3; D4 with the reviewers.
- Player AI: C4, after D3.
- Squad toolkit and gun rounds: C5.
- Sound: C7.
- Graphics: C8.
- Game designer review: D2, D3, D4, D5; review C1's page before it goes out.
- Technical lead review: D1, D4, D6; review C6's script before the runner
  gets it.
- Slime enemy and fight feel: no new build. Give C1 your films and keys, and
  answer the owner's crater question in your own thread.
- GPU tiers port and the PC runner: nothing until the PC frees up. The GPU
  thread's timing job goes into C6's batch.
