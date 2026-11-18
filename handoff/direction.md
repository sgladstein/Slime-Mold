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
- **The Lure is in the game** (his card, 11-18 03:03Z: "Lure now, in the
  three planets"; row in `wiki/decisions-game.md`): Enemy's field (#457),
  Look's rings (#458), Squad's building (#461, #464). This lane's numbers:
  pull 10; on the Build page from planet 1 with nothing to unlock (a shop
  unlock would have kept it off planet 1, where the rush is tested;
  `run_lock` never locks it); 300 food and 150 health; Tinder 100 and the
  shop's sure Torch at most 100 (before planet 2 a run has 200 to 300
  points, and Tinder without a fire is a wasted buy). It has its line in
  `news/things-to-try.txt`, second after the rush. Retune after he plays.
  Nothing is open with the owner now.
- **From the lab's part two** (11-18, `Reports/lab-steering-2026-11-17/`):
  no bullet steering card. A tuning lead for the pull: a strong lure that
  drags a colony off its crystal shrinks it without kills (10,386 alive
  against 11,460 by 2:00, 1,565 killed against 10,533), so a high pull is
  a second way to win a crystal, not only a kill zone. Keep the game's
  pull at 10 until he has played it.
- **The dart waits** (the lab's part three, 11-18): a lure fired where the
  squad aims, at pull 10, takes a crystal off a colony in 30 to 45 s and
  leaves the planet's slime 22% smaller by 2:00, for fewer kills (about
  3,500 against 8,612) and less food. The lab says build it as the Lure's
  second form. Not placed: it is more than his card covered and it changes
  how a crystal is won, so it waits until he has played the Lure building.
  Then put it to him once, with the clips, or give it to Squad if his play
  notes ask for it. Its price and cooldown are this lane's.
- **Held for D7:** Enemy's idea that the Drifter also smells crystals being
  drained by hand. It is a rush brake, so it is not built, not even behind a
  switch. Enemy's planet 1 finding is beside the rush on the list of things
  to try: a hand building pumps is raided at 0:59 and both pumps are worn
  down by 1:30, while the hand-draining rush has 42 soldiers at health 90
  at 2:00. `crystal_regrow` stays off for the same reason.
- **Also held for D7:** the harvest reach. 40 soldiers draining one crystal
  stand in an overlapping pile, because only soldiers within the rim plus
  2 cells harvest (Squad, 11-18); tidy rings need a longer reach, which is
  the rush economy.
- **The switch list** has two rows since #460 (the set slime's look
  deleted, amber stays): `crystal_regrow` (D7's) and `feel` (shake and
  hit-stop, his to judge in play). Lines in `news/whats-new.txt` should not
  name a row count, which goes stale (fixed 11-18 after Look's #471 showed
  the file in the game).
- **The run bot's first cut** (#463, Engine and tools, 11-18; run seed 1,
  a measurement, nothing changed for it): planet 1 is won at 2:48 with 2
  stars by the rush (drain the biggest crystal to 50 soldiers, walk into
  the home crystal: 9,300 walkers to 110 in 28 s), far under exit test 1's
  8 to 12 minutes; planet 2 is lost at the 15:00 clock with or without
  Tinder (the Sprawler's threads eat the 9-soldier landing squad before it
  drains). Planet 1's time is the rush, which is D7's and his. Planet 2 is
  this lane's tuning once the bot plays it on seeds 1 to 3 (the Run bot
  workflow's summary): if a played planet 2 is still lost, the landing
  squad or the Sprawler's size is the number to move. Then post the curve.
- **Next:** check each lane's lines in `news/whats-new.txt` as they land
  and keep `news/things-to-try.txt` to five (refreshed 11-18 for Provoke
  and Still, the Trunk's walls and the flinch look). When the owner plays,
  sort the log (the `play-logs` branch, since #452) and his notes within
  the hour. When the run bot exists, the curve for planets 1 to 3.
- **The play record** exists since #452: a log a round with the build
  number and F9 notes, pushed by `play.cmd` to the `play-logs` branch. No
  session of his is on it yet; his notes so far are the dated rows of
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
