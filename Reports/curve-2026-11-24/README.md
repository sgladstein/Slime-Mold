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
standing in the body slime round the origin. With the fix (#765: the stand
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

## Rerun on #769 with a raid-response arm and a raid-size arm

Planets 1 to 8 of the three maps on the GTX 1070 again (42 games,
[games-c.csv](games-c.csv)), on main after #765 (the planet 1 stand closes
in) and #769. Two extra arms on the later planets, paired by map:
`brace` (the bot's raid response, #774: a squad near a march toward it
steps back a range and fights it, `brace=1`) on planets 4 to 8, and
`raid` (`set=raid_march_size=1.0`, the Enemy lane's ask) on planet 4.

**A clock bug in the league, found here.** `Game::time()` already counts
from the landing (since 11-15), and the league (11-19) took the head start
off again, so its `secs`, the 720 s cap, the stall's grace and the
`at150/300/600` samples all ran short by the head start: about 11 s on
planet 4, 25 s on 5, 40 s on 6 and 87 s on 8 (0 on 1 to 3). `fell@` was
right. The times below add it back; the league is fixed in the same
change, so later batches need no correction.

Main arm (time to win corrected):

| Planet | Won / lost / open | Time to win (median) | Soldiers (peak) | Lost (all bites) |
|---|---|---|---|---|
| 1 | 3 / 0 / 0 | 4:40 | 32-39 | 3-9 |
| 2 | 3 / 0 / 0 | 5:57 | 44-60 | 2-4 |
| 3 | 3 / 0 / 0 | 5:36 | 35-55 | 21-47 |
| 4 | 2 / 1 / 0 | 4:50 (5:26, 4:13) | 54-62 | 0-58 |
| 5 | 3 / 0 / 0 | 4:15 | 55-95 | 0-23 |
| 6 | 3 / 0 / 0 | 5:31 | 73-78 | 3-23 |
| 7 | 0 / 2 / 1 | - | 14-77 | 17-68 |
| 8 | 1 / 1 / 1 | 5:24 | 64-84 | 0-103 |

18 of 24 won, against 13 of 24 on #756. Planet 1 won on all three maps
(the stall is gone on the 1070 too: longest stall 57 s), planet 4 won on
two. Planet 7 is the wall: origins of 2,674, 1,963 and 4,360 at most, and
no map won. Planet 8 map B ran out the clock with the origin at 2,409 of
2,699.

**`brace` (15 pairs, planets 4 to 8):** 12 pairs ended the same way; main
alone won planet 4 map A and planet 5 map A, `brace` alone won planet 8
map C. Sign test 2 against 1, p = 1.0: no gain, so it stays off (dial
`brace=0`).

**`raid_march_size=1.0` on planet 4:** 0 of 3 won, against main's 2 of 3.
On map C main saw no march and won at 4:13 with none lost; the raid arm
saw 6 marches and lost with 65 bitten. Three pairs give no p worth
quoting, but every pair went the same way.

## Fourth pass, batch t: six maps, the new crystal pick, planet 8 at size 1.4

Planets 1 to 8 on six maps (the owner's three plus run seeds 1, 2 and 3)
on the GTX 1070 at 7d7857e (#805: storm 0.5 by default, the takeable-first
crystal pick for planet 7 map B, `size=F`), two games at once, 54 games.
Origins are those before #806 (planets 5, 6, 8 at 1,500, 1,800, 2,400).
Every game's fields: [games-t.csv](games-t.csv).

| Planet | Slime | Won / lost / open | Time to win (median) | Soldiers (peak) | Lost | Income | Longest stall |
|---|---|---|---|---|---|---|---|
| 1 | drifter | 6 / 0 / 0 | 4:36 | 38 | 9 | 13,013 | 81 s |
| 2 | sprawler | 4 / 2 / 0 | 3:18 | 40 | 9 | 13,411 | 97 s |
| 3 | trunk | 6 / 0 / 0 | 6:04 | 39 | 10 | 16,053 | 93 s |
| 4 | swarmer | 3 / 3 / 0 | 5:42 | 51 | 55 | 26,529 | 87 s |
| 5 | sporer | 4 / 1 / 1 | 4:30 | 53 | 12 | 21,502 | 110 s |
| 6 | sleeper | 4 / 0 / 2 | 6:44 | 65 | 15 | 28,898 | 225 s |
| 7 | grudge | 3 / 1 / 2 | 8:39 | 40 | 27 | 16,143 | 104 s |
| 8 | dozer | 6 / 0 / 0 | 7:18 | 64 | 19 | 31,633 | 42 s |

36 of 48 won. On the owner's three maps that is 21 of 24 (the #769 pass
had 18 of 24): planet 7 is won on all three for the first time (A 11:09,
B 8:39, C 6:41), and map B, lost in every earlier arm with a squad of 13,
is won with 40. The run seeds are harder: they hold 9 of the 12 games not
won, among them planet 2 seeds 2 and 3, planet 7 seed 2 (lost at 3:00
with a squad of 21) and planet 6 seed 2 (open with 101 soldiers).

**Did the new pick cost anything?** Against the old pick's batches r and s
on the same planets and maps, 11 games ended differently: 4 better with
the new pick, 7 worse (planet 4 maps A and B among them). Sign test p =
0.55: not a measured harm. A software-Vulkan replay of planet 4 map A
with each pick ran identically to 1:40 and had 14,127 (old) and 13,299
(new) harvested by 2:30, where the 1070's game with the new pick had 731,
so that loss is a game going another way, not the pick. By 4:00 the old
pick was ahead there (income 24,377 against 18,435, 59 soldiers against
41), one replay. The pick stays; planet 4 is the place to watch it.

**Planet 8 at size 1.4 (Enemy's ask, `size=1.4`):** 6 of 6 won, at a
median 3:40 against 7:18 at size 1.0, with 1 lost against 19. That reads
backwards and is the bot, not the slime: its storm fires when the slime
falls to `storm` times *its own* start size, so a bigger slime is stormed
sooner, at a lower share of it. Batch v reruns the pair with the storm
off (`storm=0`) on main after #806.
