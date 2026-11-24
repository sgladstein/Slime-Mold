# The run bot's curve by planet, 2026-11-24

Milestone 2's test 3 (Direction, 11-24): time to win, soldiers, income and
losses by planet on the owner's three maps, with the run bot's siege on
the origin (#707). Engine and tools lane.

**How:** `larder-world league=D:MAP:720 plan=bot,want=torch.standard|rush,aim=6,cards=0`
(the planets before D taken as won with two stars and shopped by the
plan), on main 59a27bc, on the owner's GTX 1070, two games at once: 39
games in 34 minutes, 0.2 to 6.6 real minutes a game. Maps (the owner's
runs): A 1795038793, B 1795106014, C 1795119328. "now" is today's
`run::ORIGIN_SIZE`; "half" (planets 4 to 8, for the Enemy lane) is
`set=origin_size=` 1200, 1500, 1800, 2100, 2400. Every game's fields are in
[games.csv](games.csv); the raw league lines are on the owner's PC in
`C:\Users\Scott\Code\pc-batch-out\curve-20261124\out`.

## Today's origin, planets 1 to 8

Medians over the three maps; "time to win" over the won games only.
Soldiers are the peak alive; lost is soldiers fallen by the end.

| Planet | Slime | Won / lost / open | Time to win | Soldiers (peak) | Lost | Income | Longest stall |
|---|---|---|---|---|---|---|---|
| 1 | drifter | 1 / 0 / 2 | 5:18 | 36 | 5 | 11,096 | 537 s |
| 2 | sprawler | 3 / 0 / 0 | 5:33 | 51 | 7 | 19,358 | 35 s |
| 3 | trunk | 1 / 2 / 0 | 3:52 | 38 | 35 | 12,361 | 76 s |
| 4 | swarmer | 0 / 3 / 0 | - | 61 | 65 | 28,203 | 76 s |
| 5 | sporer | 2 / 1 / 0 | 7:46 | 71 | 29 | 32,827 | 77 s |
| 6 | sleeper | 3 / 0 / 0 | 8:50 | 76 | 32 | 42,254 | 38 s |
| 7 | grudge | 0 / 2 / 1 | - | 42 | 49 | 18,248 | 80 s |
| 8 | sporer (A), grudge (B, C) | 1 / 1 / 1 | 3:57 | 78 | 60 | 38,776 | 39 s |

12 of 24 won. Lost games are the squad's last soldier falling ("open" is
no verdict by 12:00).

## Half the origin, planets 4 to 8

| Planet | Won / lost / open | Time to win | Soldiers (peak) | Lost | Income |
|---|---|---|---|---|---|
| 4 | 3 / 0 / 0 | 8:07 | 59 | 31 | 27,374 |
| 5 | 2 / 0 / 1 | 3:19 | 69 | 36 | 31,895 |
| 6 | 3 / 0 / 0 | 4:43 | 75 | 3 | 36,641 |
| 7 | 1 / 1 / 1 | 4:27 | 46 | 45 | 18,650 |
| 8 | 1 / 0 / 2 | 3:57 | 78 | 50 | 38,697 |

10 of 15 won against 6 of 15 at today's size on the same planets and
maps (paired: 5 games won only at half, 1 only at today's size, planet 5
map A; two-sided sign test p = 0.22, so three maps don't settle it).

## Rerun on #756 (Enemy's origin sizes), planets 1 to 8

The same 24 games on main 70e4811 (#756 caps the origin's gather), on the
owner's GTX 1070, 25 minutes; every game's fields in [games-b.csv](games-b.csv).
Losses by cause are `bitten`, `burned`, `starfall`, `blasted` at the end.

| Planet | Slime | Won / lost / open | Time to win | Soldiers (peak) | Lost | Of which bitten | Income | Longest stall |
|---|---|---|---|---|---|---|---|---|
| 1 | drifter | 2 / 0 / 1 | 6:07 | 36 | 3 | all | 11,660 | 163 s |
| 2 | sprawler | 3 / 0 / 0 | 5:35 | 51 | 7 | all but 1 (starfall) | 19,691 | 32 s |
| 3 | trunk | 2 / 0 / 1 | 6:14 | 38 | 21 | all but 3 (burned) | 12,361 | 22 s |
| 4 | swarmer | 0 / 3 / 0 | - | 61 | 64 | all | 28,273 | 51 s |
| 5 | sporer | 2 / 0 / 1 | 3:43 | 69 | 20 | all | 31,672 | 17 s |
| 6 | sleeper | 3 / 0 / 0 | 4:46 | 74 | 3 | all | 37,203 | 9 s |
| 7 | grudge | 0 / 2 / 1 | - | 50 | 35 | all but 3 (burned) | 19,391 | 60 s |
| 8 | sporer (A), grudge (B, C) | 1 / 0 / 2 | 3:57 | 76 | 53 | all | 38,705 | 33 s |

13 of 24 won, against 12 of 24 before. Planet 7's origin now reads 2,668
and 2,494 members on maps A and B but still 13,699 on map C. On planet 8,
maps B and C, the origin was never touched (3,465/3,465, 2,416/2,416). On
planet 4 every loss is a bite and they come in bursts late in the game:
map A 28 soldiers in the 30 s after 5:00, map B 32 in the 30 s after 9:30,
map C 31 between 8:00 and 9:00. `fell@` here still counts from the head
start (this build is before #757).

**The planet 1 stalls** (map A here and before, map B here: a squad of
26 or 36 with a flat bank beside the origin) are the siege's attack-move
standing in the body slime round the origin. With the fix (#760: the stand
closes in by plain moves when the origin stops falling), planet 1 on
software Vulkan: map A won at 4:23 (stall 48 s, 9 fallen), map B 4:42
(27 s, 6), map C 3:59 (22 s, 5).

## Read with care

- **Planet 1, map A is a bot stall**: 26 soldiers and a bank flat at 259
  from 3:00 to 12:00 beside an origin of 741/763 (stall 537 s). The run
  bot's fault, not the planet's; the Engine lane's to fix.
- **Planet 7's origin on maps B and C** reads 79,940 and 66,426 members
  (61,875 at half), far over `ORIGIN_SIZE`: sent to the Enemy lane.
- **`origin=fell@T`** runs ahead of the planet's `secs` on later planets
  (planet 8 map A: won at 237 s, `fell@401`), so the two count from
  different zeros; sent to the Enemy lane.
- The bot buys no buildings or tools (`turrets=0 walls=0`, `tools=0/0/0/0`)
  and plays the rifle: these are the plan's numbers, not a player's.
