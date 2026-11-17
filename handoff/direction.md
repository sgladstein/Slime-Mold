# Direction lane: handoff

The lane that keeps the project pointed at the milestone and is the one
queue to the owner. It owns `NOW.md`, `wiki/decisions*.md`, the content of
the game's "what is new" and "things to try" lists, the record of his play
sessions and the tuning numbers. The brief it was started from is at the
end.

## State (2026-11-17)

- **Done:** `NOW.md` written; `CLAUDE.md` points at it and carries the lane
  rules; the 11-17 decisions are in `wiki/decisions.md` and
  `wiki/decisions-game.md`, D8 included; the plan is copied to
  `Reports/continuation-plan-2026-11-17.md`; `HANDOFF.md` and `ROADMAP.md`
  are marked as history; the pending picks are sorted (below); the first
  list of things to try is in `NOW.md`.
- **Open branch:** `claude/project-thread-5ho8zd` (this change).
- **Next:** move `HANDOFF.md` and `ROADMAP.md` under `Reports/` once Engine
  and tools retires the lint rule that needs links from `HANDOFF.md` (its
  job 5); both decisions pages list `HANDOFF.md` in `sources:`, so change
  them in the same commit. Move the list of things to try into the game's
  file when Engine and tools lands it. When the owner plays, sort the log
  and his notes within the hour. When the run bot exists, the curve for
  planets 1 to 3.
- **No play record yet.** Until the play log carries a build number, his
  play shows only in what he writes; his notes are the dated rows of
  `wiki/decisions-game.md`.

## The pending picks, sorted (2026-11-17)

Everything that was listed as waiting on the owner on 11-17 (HANDOFF,
*Loose ends at the move*; ROADMAP, *Switches that are off* and *Playtest
pack 11-17*), with who settles it now under his card "Threads decide small,
ask big". None of these is asked of him; he vetoes in play.

| Was waiting on him | Now | Who |
|---|---|---|
| Far smell (`far_smell`) and charges round walls (`unit_paths`); `play-new-slime.cmd` | On by default on the run's planets; delete the second script | Enemy |
| Setting, block or trap (`set_block`, `set_hold`), crust and its pull, fed slime fireproof (`fire_fed`): four blind films | Pick one configuration and give it to one deeper tier as its trait; the films exist if wanted | Enemy |
| The set slime's look (`setlook`: glaze or amber) | Decide | Look and interface |
| Haze strength and flinch strength | Redraw the haze (look A); set the flinch | Look and interface; Enemy |
| Mini-boss colour (Violet, Ember, Ink) | Decide | Look and interface |
| Fen moat or rock rims | Delete the moat: he saw no difference | Look and interface |
| Heave and Corral as one card | Decide; the plan would merge them | Squad and arsenal |
| The Mind bullets quiz, the crust "name the ground" quiz | Drop both: play shows whether they read | Squad and arsenal; Enemy |
| The door-grate bullets question | Answered 11-17 03:08Z: bullets do not pass doors | closed |
| Camera shake and hit-stop (D6) | One Experiments toggle; second on the list of things to try | Squad and arsenal, with Look and interface |
| Last stand | One deep tier's trait, or delete | Enemy |
| Reserve recruit, lossy flow, lone step | Delete, each with its `Reports/dead-ends.md` line | Enemy |
| Crystals that refill | Stays off: it belongs to his D7 answer | Direction holds it |
| Specks amount, mortar strength, the size of what each planet does to you | Tuning notes from his next play | Direction |
| Run pacing, shop prices, the breather planet | His next play and the bot's curve | Direction |
| Sound levels, plants | His next play | Direction notes it; no lane job until he says |
| The next turret body, one deck per body | Parked with the turret work | parked |
| GTX 1070 timings (homing, Mind bullets, Grudge memory, plants, mini-boss lumps, crust and setting) | One batch before the Milestone 1 check | Engine and tools |
| Dish-era decisions 2 to 8 | Closed 11-17 as not needed while paused | closed |

What still waits on him is in `NOW.md`, *What waits on the owner*.

## The brief (planning thread, 2026-11-17)

Model and effort: Fable, high. Read `00-all-lanes.md` in this folder first.

### Your job

Keep the project pointed at Milestone 1 and keep Scott's attention cheap.
You own `NOW.md`, the content of the game's "what is new" and "things to
try" lists, the record of Scott's play sessions, `wiki/decisions*.md`, and
the tuning numbers (prices, clocks, rewards, slime sizes). You are the one
queue to Scott: other lanes send you questions, and you ask him at most
once a session, each question with a recommendation. He answers numbered
lists and decision cards quickly.

### First jobs, in order

1. **One docs pull request that resets the front door.**
   - `NOW.md`, at most 100 lines: the milestone and its six exit tests,
     each lane's top three jobs, and the queue of what waits on Scott.
   - `CLAUDE.md`: point threads at `NOW.md` instead of `HANDOFF.md` and
     `ROADMAP.md`, and carry the lane rules from `00-all-lanes.md` (lanes
     and owned files, nobody asks for a playtest, everything lands on,
     lighter records). Keep Scott's rulings that the plan keeps: fun over
     fidelity, the picture is the simulation, build first and balance
     later, brainstorms start from the model.
   - `wiki/decisions.md` and `wiki/decisions-game.md`: record the 11-17
     decisions below with their quotes and times.
   - `HANDOFF.md` and `ROADMAP.md` move under `Reports/` as history once
     Engine and tools has changed the wiki lint that requires links from
     `HANDOFF.md`. Agree the order with that lane through the coordinator.
2. **Sort every pending pick** as in the plan's table "What I would let
   the lanes settle", and tell each lane what it now decides. Close the
   dish-era decisions 2 to 8 on HANDOFF as "not needed while paused"
   (Scott, 19:07Z, "5)Yes").
3. **Write the first "things to try" list**, at most five items, most
   important first. First item: the rush on planet 1 on the new build
   (D7). Second: camera shake and hit-stop (D6) as an Experiments toggle.
4. **When Scott plays**, sort the play log and his notes within the hour
   into lane jobs, tuning changes or "not now", mark what he tried as
   tried, and update both lists.
5. **When the run bot exists** (Engine and tools), have it play planets 1
   to 3 on paired seeds and post the curve: time to win, soldiers, income
   and losses by planet.

### Decisions of 2026-11-17 to record (Scott's words, UTC)

- 17:12-17:13, four cards: "About 5 standing lanes"; "Threads decide
  small, ask big"; "A game I love to play; release maybe"; "Slice first,
  plus one lab thread".
- 17:25: "I have playtested many times. the threads don't always know that
  and I often cannot playtest when a specific thread asks. But the game is
  not unplayed." On features landing switched off: "this is a problem.
  There is a lot of work going on in parallel and when I have time to
  playtest, I don't know what is on/off and what needs to be tested." And:
  "I didn't see any specific directions about improving the visuals, but
  that is an important factor".
- 17:39: "For the visuals, I am happy to start with your recommendations,
  but we should explore all your suggestions, priority in the order that
  you listed A->B->C->D" (A the fight round the squad, B the slime itself,
  C planets and terrain, D menus, shop and screens).
- 17:39, on `play.cmd` sending play logs to the public repository: "yes
  play logs is a great idea".
- 19:07, to ten numbered questions: go on the plan ("1) Yes"); Windows
  only for now, D9 ("3) Yes"); his own mould stays parked, D12 ("4)
  Sure"); close the dish-era decisions ("5)Yes"); never assign him to a
  pull request or request his review ("6)Yes"); delete old merged
  branches and delete branches on merge ("7)Yes"); films and stills on an
  archive branch ("8) Yes"); models as quoted in `00-all-lanes.md`. On
  D8 he asked "is there any reason to even have the CPU twin?" and at
  19:21 chose on a card "GPU only for new rules, keep old CPU": new slime
  rules on the GPU engine only, today's CPU engine unchanged, its removal
  decided at Milestone 2. Record that and close D8. The lab's first prototype
  (steering) had no answer and goes ahead on the plan's recommendation.

### Not yours

Building features. If a lane is blocked on another lane, you unblock it by
deciding or by asking Scott once; you do not write the code.
