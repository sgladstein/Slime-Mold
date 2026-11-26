# Now

The front page: what is being built, who builds it, and what waits on the
owner. Kept by the Direction lane (last change 2026-11-24), which owns it; send it
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

## Milestone 2: A whole run (about two weeks from 11-24)

The roguelike run in squad mode, all eight planets, as a run he wants to
finish and replay. It is done when all six hold:

1. **Every planet ends with the slime wiped out** (his ruling, 11-26 00:08Z: hearts stay, "but they are not the win condition"). A heart forms on any charged crystal the slime keeps packed for 2 minutes, as a mini-boss to break before the crystal is yours (#858). The run bot wins planets 1 to 8 by wiping the slime out, with no soft lock, and its time to win is measured again on that rule.
2. **A run of at least two hours** (his answer, 11-24 03:14Z), reached by about 12 planets picked on a star map with forks and contracts (his pick, 11-25 23:45Z), the harder forks paying more. His own run lands at two hours or more.
3. **The curve.** The run bot plays all eight planets on his maps, and one page shows time to win, soldiers, income and losses by planet. No planet is a wall for every main plan, and later planets cost more than earlier ones.
4. **Save and resume.** He can quit between planets, or mid-planet, and pick the same run up on the next start.
5. **60 frames a second on his PC**: frame p95 under 16.7 ms on the GTX 1070 at his own screen (3840 x 2160, World detail as shipped) through planets 1 to 8, the Sporer's big slimes included.
6. **Everything built is on**, and the game's pages list what is new and the few experiments that are off (kept from Milestone 1).

The rush (D7): a heart tougher by planet (11-24 14:52Z) answered the heart snipe, which no longer wins a planet since 11-26; no other rush brake unasked. Milestone 3, a
release, only if he chooses one.

**Since 11-26.** Hearts are no longer the win (#858); the step-B strategy batch (five plans, planets 2, 4 and 6, hearts off) and the star map with forks and contracts (his pick, 11-25 23:45Z, [brief](Reports/review-2026-11-25/route-choice-brief.md)) are under way.

**Where it stands (11-24, 20:10Z).** The origin is on main (#699), at 700, 500 and 1,000 on planets 1 to 3 and half size on 4 to 8 (#756), its gather capped at twice its size (#769).
- **Curve (test 3).** Fourth pass (bot with storm and crystal-pick fixes): 36 of 48 on six maps, 21 of 24 on his three, P7 won on all three of his maps. Medians P1 4:36, P2 3:18, P3 6:04, P4 5:42, P5 4:30, P6 6:44, P7 8:39, P8 7:18. The fifth pass (main and builder arms, with D7) runs next; planet 5 is the one to watch.
- **Rush (D7).** His card (14:52Z): the heart gets tougher by planet. Enemy's #836: a snipe needs 16 soldiers on P2 and 32 from P3 on, and the siege still wins.
- **4K (test 5): passes** with #820 (Auto retries 50% only with room for it). On the 1070 at his 3840-wide window, Auto detail, frame p95: P3 12.7 ms, P5 15.2, P8 at its peak 16.4 and 16.3 in two runs (it steps to 40% once and stays). Left over: the landing frame's step (about 440 ms) and one 38 ms CPU step at 2:14 on P8.
- **Save and resume (test 4): passes** (#757; 1070 check 07:07Z, 7.7 MB in about 50 ms).
- **What each planet does** is on screen: P2 Sprawler threads, P6 Sleeper, P7 Grudge scar, P8 the Dozer (#784, #788), each with its landing card and HUD tell (#785, #792).

## Each lane's top jobs

| Lane | Test | Top jobs, in order |
|---|---|---|
| Direction | all | This page and the rulings. His two-hour-run card, then D8. Sort each play session within the hour. The curve page when test 3 holds. |
| Enemy | 1, 3 | The heart tougher by planet (D7, his card 14:52Z). Then the Lab's planet twists, Wind and Cold snap, on about one planet in three. P5, P6, P8 sized so later planets cost more. Lap two (Lab's sizes, rigs and `strikers_settle` patch) if he picks 12 or 16 planets. |
| Squad and arsenal | 1, 3 | The gun row's card look B, fused shots (his 15:51Z pick), with Look HUD. Whatever the curve's builder arm shows broken in the kit. |
| Look and interface | 1, 5 | HUD: first-sight hints for each threat and trait, then what the twists need on screen. Main: Look B, C from his notes. |
| Engine and tools | 1, 3 | The storm dials (army, clock), then the fifth curve pass with the builder arm once D7 lands; the curve page follows. |
| Lab | none | A rescue planet: a goal other than the heart, from the model. Night stays on its branch. |

**Unparked now:** gun steps E and F, the next turret body, twists and rescue
planets, more planet polish. Still parked: the player's own mould (D12), and
everything frozen (campaign, RTS control, player AI, the dish games).

## What waits on the owner

Nobody asks him to playtest, and a lane's question goes to Direction, which
asks at most once a session with a recommendation.

| What | State |
|---|---|
| Longer planets or more of them | Answered 11-25 23:45Z by his pick of a star map with forks and contracts (about 12 steps, harder forks pay more; [brief](Reports/review-2026-11-25/route-choice-brief.md)), so the run reaches two hours by more planets, chosen. `run::STRETCH` stays 1 until he plays it. |
| Whether to remove today's CPU slime engine (D8) | Decided at this milestone (his card, 11-17 19:21Z). Engine lists what stands on it first; asked next. |
| "Automatically delete head branches" in the repository settings | His switch to turn on (asked 11-17 19:14Z). |

## Things to try at the next play

At most five, most important first; offers, not requests. The game shows them
beside "What is new": [`news/things-to-try.txt`](news/things-to-try.txt).
Today: the heart, the harder planets from 2, each slime's landing card and
number, quitting mid-planet, and the Sprayer and Hunter.
What each build holds is in
[`news/whats-new.txt`](news/whats-new.txt).

## Where things are

- Rulings, quoted and timed: [`wiki/decisions.md`](wiki/decisions.md), [`wiki/decisions-game.md`](wiki/decisions-game.md).
- What is true now, by subject: [`wiki/index.md`](wiki/index.md). Tried and rejected: `Reports/dead-ends.md`.
- Before 11-17: [`Reports/handoff-2026-11-17.md`](Reports/handoff-2026-11-17.md) and [`Reports/roadmap-2026-11-17.md`](Reports/roadmap-2026-11-17.md), history only.
