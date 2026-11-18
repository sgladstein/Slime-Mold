# What planets 1 to 3 do to you by minute 2 (2026-11-18)

The Enemy lane's job 2 from the plan of 11-17: planets 1 to 3 each do one
nameable thing to the player by minute 2, with a clip a planet (Milestone
1's exit test 4: after playing them the owner can say in a sentence what
each slime did to him). No rush brake is added: that is D7, which the owner
holds.

Page: https://claude.ai/artifact/NUbrTLNawdjzSHCHxyWeRV (source `page/`; open `index.html`).
Films: `film.sh`; clips from their frames: `make_clips.py`.

All films: run seed 1, the software GPU, one film each, the game as on main
after #473 (the Trunk's fort) with #478 (the round lure field). `hand=bot`
is Engine and tools' run bot: it drains the richest crystal, buys soldiers
and volleys at the heaviest slime, and builds no pumps. `hand=pumps` builds
two pumps and holds beside the first.

## The three things

| Planet | Slime | What it does to you | Seen by 2:00 |
|---|---|---|---|
| 1 | Drifter | Smells your pumps and comes to eat them | Only if you build pumps |
| 2 | Sprawler | Its threads smell your soldiers from far off and swarm them | Yes |
| 3 | Trunk | Walls the crystals it holds, and hardens where you shoot it | Yes |

### Planet 1: it eats your pumps

`hand=pumps` (K1-pumps). The slime streams out of its bowl to the pump
nearest it from the start; a raid of 1,945 walkers leaves at 0:59 for the
far pump.

| Time | Pump at (374, 25) | Pump at (290, 50) |
|---|---|---|
| 0:40 | 200 of 200 | 144 |
| 1:00 | 200 | 71 |
| 1:05 | 189 | 46 |
| 1:10 | worn down | 14 |
| 1:15 | worn down | worn down |

The pumps paid 1,607 food in all. With the pumps gone, raid 2 (1:35) goes
for the squad of 4 holding by the first: health 98 at 1:00, 53 at 1:40.

Under the bot, which builds no pump, planet 1 does nothing to the squad by
2:00 (J1-bot): 14 soldiers at 0:30, 26 at 1:00, 50 at 2:00 at health 93,
with two raids of about 1,650 walkers (0:59, 1:35) that it shoots down, and
the slime in its bowl down to 9,508 walkers (`page/planet1-bot-2min.jpg`).
That is the gap in this exit test. Direction holds the first candidate,
"the Drifter also smells crystals drained by hand", for when the owner
rules on D7, and it is not built before (Direction, 11-17 22:28Z).

### Planet 2: it hunts your squad

`hand=bot` (K2-bot, J2-bot). The Sprawler's fast threads find the squad
from across the map and swarm it.

| Time | Soldiers | Health (mean) | Walkers killed | Slime |
|---|---|---|---|---|
| 0:20 | 10 | 80 | 1,581 | 22,067 |
| 0:40 | 11 | 51 | 4,806 | 21,030 |
| 1:00 | 11 | 35 | 6,620 | 20,717 |
| 1:30 | 10 | 41 | 8,657 | 22,098 |
| 2:00 | 6 | 36 | 12,953 | 21,698 |

The squad kills 13,000 walkers by 2:00 and the slime is no smaller: it
spends its threads on the squad and grows them back. Raids at 0:59 and
1:40 go for the soldiers. By 2:30 three soldiers are left (J2-bot).

### Planet 3: it walls its crystals and hardens where shot

`hand=bot` (K3-bot, J3-bot). The home ring stands from 0:15 (the fort,
[report](../trunk-fort-2026-11-18/README.md)); the bot drains the crystals by
the landing site, then goes for the second big crystal and its ring.

| Time | Soldiers | Health | Walkers set | Wall cells | Killed | Slime |
|---|---|---|---|---|---|---|
| 0:30 | 12 | 99 | 1,597 | 1,540 | 114 | 43,423 |
| 1:00 | 15 | 96 | 1,784 | 1,731 | 706 | 43,440 |
| 1:30 | 16 | 58 | 7,508 | 5,354 | 4,965 | 28,430 |
| 1:45 | 16 | 61 | 11,851 | 7,211 | 8,885 | 20,701 |
| 2:00 | 19 | 68 | 12,140 | 5,461 | 9,488 | 17,192 |

From 1:15 the slime the squad shoots sets into amber round it, and by 1:40
the squad is inside the second crystal's ring. Raid 1 at 1:12 goes for the
soldiers.

## Reading

- Planets 2 and 3 each do their thing by 2:00 to the bot, and they are
  different things: planet 2 wears the squad down from everywhere, planet 3
  holds its ground and turns the squad's own fire into walls.
- Planet 1 does its thing only to a player who builds pumps. A player who
  drains by hand and buys soldiers, the owner's own way (his 11-15 play
  notes), meets a slime that stays home. Closing that is D7's.
- The bot is one scripted player; the owner plays differently. Each table
  is one film.

## Not checked

- Other run seeds.
- Planet 2 with Tinder and a Torch, which the shop before it now pushes.
