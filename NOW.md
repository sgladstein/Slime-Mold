# Now

The front page: what is being built, who builds it, and what waits on the
owner. Kept by the Direction lane (last change 2026-11-28), which owns it; send it
changes, don't edit it. At most 100 lines. The rules are in
[`CLAUDE.md`](CLAUDE.md), each lane's state in `handoff/<lane>.md`, the
reasons in [the plan](Reports/continuation-plan-2026-11-17.md) the owner
approved on 11-17 ([decisions](wiki/decisions.md)).

## Milestone 1, Three planets: done (his word, 11-24 03:09Z)

"Milestone 1 is complete." Planets 1 to 3 play as one sitting; on 11-22 he
won all eight planets in about 77 minutes and called it too easy, so #721
and the 11-22 evening changes raised planets 2 to 8 partway. The record is
in [`handoff/direction.md`](handoff/direction.md) and
[`wiki/decisions-game.md`](wiki/decisions-game.md).

## Milestone 2: A whole run (re-anchored 11-27 on main 5cd97ab)

The roguelike run in squad mode, twelve steps on the star map, as a run he
wants to finish and replay. It is done when all six hold:

1. **Every planet ends with the slime wiped out** (his ruling, 11-26 00:08Z: hearts stay, "but they are not the win condition"). A heart forms on any charged crystal the slime keeps packed for 2 minutes, a mini-boss to break before the crystal is yours (#858). The run bot wins all twelve steps on Safe, Fair and Hard picks by wiping the slime out, with no soft lock.
2. **A run of at least two hours** (his answer, 11-24 03:14Z), by twelve steps picked on a star map with forks, contracts and heat (his pick, 11-25 23:45Z; #860). His own run lands at two hours or more.
3. **The curve.** The run bot plays the twelve steps, and one page shows time to win, soldiers, income and losses by step and pip. No step is a wall for every main plan, later steps cost more than earlier ones, and Hard costs more than Fair, Fair more than Safe.
4. **Save and resume.** He can quit between planets, or mid-planet, and pick the same run up, its star-map route and turret designs included.
5. **60 frames a second on his PC**: frame p95 under 16.7 ms on the GTX 1070 at his own screen (3840 x 2160, World detail as shipped) through the twelve steps, a Hard step 9 and the home world included.
6. **Everything built is on**, and the game's pages list what is new and the few experiments that are off.

Milestone 3, a release, only if he chooses one.

**Where it stands (11-27).** From 11-25 to 11-27 his other project built, on his own chat rulings (all in [`wiki/decisions-game.md`](wiki/decisions-game.md)): the star map (#860), hearts as a crystal mini-boss (#858), slime levels and toughness by planet (#862, #864), stronger and smarter mini-bosses (#865), a faster early climb (#868), harder Hard forks (#870), harder from step 2 (#873), turrets that fire their loadout's gun (#871, #875) and are bought like guns (#876), and the HUD's slime bar (#872). Review: [doc](https://claude.ai/artifact/6dmCXQ5GC3H1w8LN4ZWGrE); plan: [doc](https://claude.ai/artifact/57RBPNiFdCkUk4DZwN28nG), kept in the repo as `Reports/plan-2026-11-27/` (#882). The work moves to a new account (11-27 23:36Z): start from [`handoff/account-move-2026-11-27.md`](handoff/account-move-2026-11-27.md). New lane threads started there 11-28 03:14Z with the jobs below.
- **Tests 1 and 3: not measurable yet.** The run bot plays only the Fair planet, builds no turret designs and still aims its siege at hearts; the nightly Run bot was red on 11-25 and 11-27. Last curve pass: 11-24, eight planets.
- **Test 2: close.** His 11-27 run took about 55 minutes for steps 1 to 6, so about 1 h 50 m for twelve, before the raises since.
- **Test 4: holds**, recheck on his PC with a star-map run. **Test 5: held 11-24** (P8 p95 16.4 ms), recheck: levels, bigger raids and Hard home-world sizes came since. **Test 6: holds.**
- **Difficulty (his card, 11-27 23:55Z, "Five lanes, no freeze"):** lanes keep raising difficulty on his notes; each of his run logs gets one retune pass from Direction with Enemy.
- **First the core problem: too easy** (his word, 11-28 03:31Z). A stronger slime and better turrets come before any economy balance; the plan's step 6 (energy sink) is dropped, and food, soldier and turret costs are tuned from his playtests. Each lane asks him after its first review when its task is unclear or forks (03:27Z).

## Each lane's top jobs

| Lane | Test | Top jobs, in order |
|---|---|---|
| Direction | all | This page and the rulings, including asks he makes in any thread. One retune pass from each of his run logs. The curve page when test 3 can be measured. |
| Enemy | 1, 3 | Pressure from the slime itself first: rerun the Lab's hunt on today's slime and build what it shows. Then the rescue planet as a star-map node (below). The Grudge readout is merged (#886). |
| Squad and arsenal | 1, 3 | Turret designs in the run bot (with Engine, #887). Better turrets, worth building against a stronger slime. No economy retune: food, soldier and turret costs follow his playtests (his word, 11-28 03:31Z). |
| Look and interface | 1, 5 | The star map and slime colours checked against the art direction; the rescue node's badge and screens; then his A to D order. |
| Engine and tools | 1, 3, 4, 5 | The run bot up to today's game: picks by pip, builds turret designs, no heart siege, the builder arm; then the curve on twelve steps. The 1070 retime and a star-map save check. The PC league stopped or rebuilt on main. |
| Lab | none | Paused until the hunt rerun needs its rig (his card, 11-27 23:55Z). |

**The rescue planet** (Direction's call, 11-24, moved 11-27): in about one
run in three, one planet offered at the step that meets the Trunk is a
Rescue planet, marked on its card; its goal is to walk two dug-in squads
home, and wiping out the slime also wins it. Hearts work there as anywhere.

Kept for later: the slime spreading between planets and planets that
remember you (his 11-25 "keep 2 and 5"). Still parked: the player's own
mould (D12), and everything frozen (campaign, RTS control, player AI, the
dish games).

## What waits on the owner

Nobody asks him to playtest, and a lane's question goes to Direction, which
asks at most once a session with a recommendation.

| What | State |
|---|---|
| The PC strategy league | Still running on an 11-22 build; stop it or let Engine rebuild it on main. |
| "Automatically delete head branches" in the repository settings | His switch to turn on (about 40 stale branches). |

## Things to try at the next play

At most five, most important first; offers, not requests. The game shows them
beside "What is new": [`news/things-to-try.txt`](news/things-to-try.txt).
Today: hearts as a mini-boss, picking Safe, Fair or Hard on the star map,
each slime's landing card, quitting mid-planet, and turrets bought like guns.
What each build holds is in
[`news/whats-new.txt`](news/whats-new.txt).

## Where things are

- Rulings, quoted and timed: [`wiki/decisions.md`](wiki/decisions.md), [`wiki/decisions-game.md`](wiki/decisions-game.md).
- What is true now, by subject: [`wiki/index.md`](wiki/index.md). Tried and rejected: `Reports/dead-ends.md`.
- Before 11-17: [`Reports/handoff-2026-11-17.md`](Reports/handoff-2026-11-17.md) and [`Reports/roadmap-2026-11-17.md`](Reports/roadmap-2026-11-17.md), history only.
