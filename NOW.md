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

1. **The origin wins a planet.** On every planet the slime's heart sits on its start crystal from the landing, and killing it wins (the last walker still wins as a fallback). The run bot wins planets 1 to 8 that way with no soft lock, and draining the heart's crystal is a second way in.
2. **A run of at least two hours** (his answer, 11-24 03:14Z): longer planets or twice as many planets, the exact length not settled yet. The run is built so either can be dialled, and his own run lands at two hours or more.
3. **The curve.** The run bot plays all eight planets on his maps, and one page shows time to win, soldiers, income and losses by planet. No planet is a wall for every main plan, and later planets cost more than earlier ones.
4. **Save and resume.** He can quit between planets, or mid-planet, and pick the same run up on the next start.
5. **60 frames a second on his PC**: frame p95 under 16.7 ms on the GTX 1070 at his own screen (3840 x 2160, World detail as shipped) through planets 1 to 8, the Sporer's big slimes included.
6. **Everything built is on**, and the game's pages list what is new and the few experiments that are off (kept from Milestone 1).

The rush (D7): he picked a heart that gets tougher by planet (11-24 14:52Z); no other rush brake unasked. Milestone 3, a
release, only if he chooses one.

**Where it stands (11-24, 12:30Z).** The origin is on main (#699), at 700, 500 and 1,000 on planets 1 to 3 and half size on 4 to 8 (#756), its gather capped at twice its size (#769).
- **Curve (test 3).** Third pass (#782): 18 of 24 won, P7 on no map (the wall), medians flat at 4:40 to 5:24. Planet 7's stall is the bot's: its siege waits for every crystal but the home to run dry, and P7 has about 30. Engine's `storm=F` dial (siege once the slime is down to F of its landing size with 20 soldiers) is on the PC at 0.35 and 0.5; Enemy's P5, P6, P8 curve follows.
- **Rush (D7).** A heart snipe of 8 to 15 soldiers won P2 to P4 in 1:10 to 2:32; his card (14:52Z): a heart tougher by planet, Enemy building it.
- **4K (test 5).** On the 1070 at his 3840-wide window, Auto detail, with the walker sort (#789): p95 12.7 ms on P3, 15.1 on P5, planets 1 to 7 pass with room. P8 at its peak is 14.1 at 40% but 16.8 on Auto, its 50% retries over the line; Engine makes Auto retry only with headroom, then test 5 is marked.
- **Save and resume (test 4): passes** (#757; 1070 check 07:07Z, 7.7 MB in about 50 ms).
- **What each planet does** is on screen: P2 Sprawler threads, P6 Sleeper, P7 Grudge scar, P8 the Dozer (#784, #788), each with its landing card and HUD tell (#785, #792).

## Each lane's top jobs

| Lane | Test | Top jobs, in order |
|---|---|---|
| Direction | all | This page and the rulings. His two-hour-run card, then D8. Sort each play session within the hour. The curve page when test 3 holds. |
| Enemy | 1, 3 | The heart tougher by planet (D7, his card 14:52Z). Planet 7 beatable with the storm dial, then P5, P6, P8 sized so later planets cost more. Lap two (Lab's pairings) if he picks 12 or 16 planets. Wanderer feeding after the Lab's P7 check. |
| Squad and arsenal | 1, 3 | Fever and Backdraft, then the Sprayer turret body, then the Hunter. |
| Look and interface | 1, 5 | HUD: run screens that follow `run::PLANETS`, then Squad's new tools readable in fight and shop, the HUD's draw cost at 4K, the Experiments list (test 6). Main: Look B, C, D from his notes. |
| Engine and tools | 1, 3, 5 | The storm dial picked on P7, the GPU walker sort timed on P5 and P8, then the fourth curve pass. |
| Lab | none | Bait against drain on planet 7 (#793's open trade), then lap two's planet 9 as a rig. |

**Unparked now:** gun steps E and F, the next turret body, twists and rescue
planets, more planet polish. Still parked: the player's own mould (D12), and
everything frozen (campaign, RTS control, player AI, the dish games).

## What waits on the owner

Nobody asks him to playtest, and a lane's question goes to Direction, which
asks at most once a session with a recommendation.

| What | State |
|---|---|
| Longer planets or more of them | His 11-24 answer: at least two hours, by either; "we don't need to decide exactly now". The dials are on main (#749: `run::PLANETS`, `run::STRETCH`). Costed and asked 05:10Z: both roads, starting at 12 planets at 1.3x (recommended, about 2.5 days), 16 planets (2 days), 8 at 1.8x (1 day), or not yet. |
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
