# Direction lane: handoff

The lane that keeps the project pointed at the milestone and is the one
queue to the owner. It owns `NOW.md`, `wiki/decisions*.md`, the content of
the game's "what is new" and "things to try" lists, the record of his play
sessions and the tuning numbers. The brief it was started from is at the
end.

## State (2026-11-18)

- **Done:** `NOW.md` and the lane rules in `CLAUDE.md` (#447); the 11-17
  decisions in both decisions pages, D8 included; the plan copied to
  `Reports/continuation-plan-2026-11-17.md`; the pending picks sorted
  (below); the lab's steering report carried to main (#450); the old front
  pages moved to `Reports/handoff-2026-11-17.md` and
  `Reports/roadmap-2026-11-17.md` with stubs left at the root;
  `news/things-to-try.txt` taken over.
- **The Lure is going in** (his card, 11-18 03:03Z: "Lure now, in the three
  planets"; row in `wiki/decisions-game.md`). The split is in `NOW.md` and
  went to Enemy, Squad and arsenal, and Look and interface through the
  coordinator, and to the Lab directly, at 03:05Z. The numbers are this
  lane's: pull 10 (the lab's 30 emptied planet 1's home colony in 45 s),
  shop unlock 150, the building 300 food and 150 health. When it lands:
  check its line in `news/whats-new.txt`, give it a line in
  `news/things-to-try.txt` in place of the mini-bosses, and retune after
  he plays. Nothing is open with the owner now.
- **Held for D7:** Enemy's idea that the Drifter also smells crystals being
  drained by hand. It is a rush brake, so it is not built, not even behind a
  switch. Enemy's planet 1 finding goes beside the rush on the list of
  things to try once its switch PR is on main: a hand building pumps is
  raided at 0:59 and both pumps are worn down by 1:30, while the
  hand-draining rush has 42 soldiers at health 90 at 2:00.
- **Next:** when Enemy's switch PR merges, note on the 11-17 01:13Z bulwark
  row of `wiki/decisions-game.md` what the lane settled (wall kept, trap
  deleted, fed slime fireproof as the Trunk's trait), as the lane's choice,
  not the owner's words. Prune and reword `news/whats-new.txt` as lanes add
  lines. When the owner plays, sort the log and his notes within the hour.
  When the run bot exists, the curve for planets 1 to 3.
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
| Haze strength | Settled by Look and interface, 11-17: there is no haze any more. The lime wash was the engine's alarm paint; the alarm now shows only on slime, as a warm orange flush in the slime's own colour, and bare ground shows nothing | closed when its PR merges |
| Flinch strength | Set it | Enemy |
| Mini-boss colour (Violet, Ember, Ink) | Decide | Look and interface |
| Fen moat or rock rims | Delete the moat: he saw no difference | Look and interface |
| Heave and Corral as one card | Decide; the plan would merge them | Squad and arsenal |
| The Mind bullets quiz, the crust "name the ground" quiz | Drop both: play shows whether they read | Squad and arsenal; Enemy |
| The door-grate bullets question | Answered 11-17 03:08Z: bullets do not pass doors | closed |
| Camera shake and hit-stop (D6) | One Experiments toggle; second on the list of things to try | Squad and arsenal, with Look and interface |
| Last stand | One deep tier's trait, or delete | Enemy |
| Lossy flow, lone step | Delete, each with its `Reports/dead-ends.md` line | Enemy |
| Reserve recruit (`Rules::reserve`) | Kept: it has no `set=` name and the run never uses it; it is the frozen campaign's mission rule (his 11-12 04:44Z no-soft-lock pick), so it is not a switch (Enemy, 11-17) | closed |
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
