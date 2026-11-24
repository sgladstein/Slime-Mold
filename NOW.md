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

The rush (D7) stays his: **add no rush brake unasked.** Milestone 3, a
release, only if he chooses one.

**Where it stands (11-24).** The origin is on main (#699, 11-24 03:29Z);
Engine's siege #707 is queued: sizes 700, 500 and 1,000 on planets 1 to 3; the
siege wins those on seeds 1 to 3 in under 10 min; a 44-soldier rush still
wins planet 1 at 1:34 and planet 2 at 1:47 (for D7). Planets 4 to 8 have no
origin sizes yet. His 4K frames were 40 ms (the view's per-pixel cost,
`Reports/perf-4k-2026-11-22/`); World detail Half and #720 cut it, not yet
timed on the 1070. Save and resume works between planets (#696); quitting
mid-planet relands the planet fresh, and the pause menu says so (#748).
A true mid-planet snapshot follows the siege curve.

## Each lane's top jobs

| Lane | Test | Top jobs, in order |
|---|---|---|
| Direction | all | This page and the rulings. Cost the two roads to a two-hour run (longer planets, or 16) with Enemy and Engine. Sort each play session into lane jobs within the hour. Post the curve page when Engine's table lands. |
| Enemy | 1, 3 | Land the origin (#699) on, then give planets 4 to 8 their origin sizes, so the siege wins each under 12 min. Planet 5's size (`tier_size`), which waited for this milestone. Each of planets 4 to 8 does one nameable thing to the player. |
| Squad and arsenal | 1, 3 | The Tinder brush that burns its own squad on planet 3 (rounds.rs). Gun steps E and F, now unparked. The dart (the Lure's second form). Hold fire and formations follow his play notes. |
| Look and interface | 1, 5 | The origin's look and the HUD's goal line, so he knows from the first minute what to kill. The view's cost at 4K with Engine (test 5). Then Look B, C, D from his notes. |
| Engine and tools | 1, 3, 4, 5 | Save and resume (test 4). The run bot's siege on all eight planets, its stall gate counting origin losses as progress, and the curve table (test 3). A 1070 batch at his 3840 x 2160 screen (test 5). |
| Lab | none | The Sprawler's body as threads: packed walkers that flow along its trail while the walker count holds (Enemy's patch, `/mnt/project-files/enemy/lanes-prototype-2026-11-22.patch`). Waits for its brief. |

**Unparked now:** gun steps E and F, the next turret body, twists and rescue
planets, more planet polish. Still parked: the player's own mould (D12), and
everything frozen (campaign, RTS control, player AI, the dish games).

## What waits on the owner

Nobody asks him to playtest, and a lane's question goes to Direction, which
asks at most once a session with a recommendation.

| What | State |
|---|---|
| Longer planets or more of them | His 11-24 answer: a run of at least two hours, by either; "we don't need to decide exactly now". Direction brings both, costed, once the origin plays. |
| Whether to remove today's CPU slime engine (D8) | Decided at this milestone (his card, 11-17 19:21Z). Engine lists what stands on it first; asked next. |
| D7 the rush brake | His, in play. The origin's rush numbers are in `Reports/origin-design-2026-11-22/`. |
| "Automatically delete head branches" in the repository settings | His switch to turn on (asked 11-17 19:14Z). |

## Things to try at the next play

At most five, most important first; offers, not requests. The game shows them
beside "What is new": [`news/things-to-try.txt`](news/things-to-try.txt).
Today: the origin, the harder planets from 2, the soldier price past 30,
formations as blob and ring with a spread, and starfall's pickup.
What each build holds is in
[`news/whats-new.txt`](news/whats-new.txt).

## Where things are

- Rulings, quoted and timed: [`wiki/decisions.md`](wiki/decisions.md), [`wiki/decisions-game.md`](wiki/decisions-game.md).
- What is true now, by subject: [`wiki/index.md`](wiki/index.md). Tried and rejected: `Reports/dead-ends.md`.
- Before 11-17: [`Reports/handoff-2026-11-17.md`](Reports/handoff-2026-11-17.md) and [`Reports/roadmap-2026-11-17.md`](Reports/roadmap-2026-11-17.md), history only.
