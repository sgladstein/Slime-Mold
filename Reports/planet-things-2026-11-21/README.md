# What each slime does by minute 2 (2026-11-21)

Direction's ask for Milestone 1's exit test 4: after playing, the owner can
say in a sentence what each of the three slimes did to him. The Enemy lane
checked that each of planets 1 to 3 does one nameable thing by about
minute 2 on main after the threat pacing (#606, and #644 moving fruiting to
planet 2), and filmed one clip a planet at zoom 6.
Page: https://claude.ai/artifact/7HaiejeHqdUjhiSjEfrJqK (copy: `index.html`
and the three clips here).

How: `larder-world run=SEED:PLANET mode=squads hand=greedy` (planet 1:
`hand=pumps`) on lavapipe, run seeds 1 to 3, logs to 3:00 (planet 1 to
9:00); clips from `look=` frames a game second (planet 3 a half second),
joined with ffmpeg at 8 frames a second, 960 x 600.

| Planet | By about 2:00 | Seeds 1-3 | Clip |
|---|---|---|---|
| 1, Drifter | smells your pumps, streams to them and eats them (a pump at 194 health worn to 0 by 1:10); surges from 0:29 | surges by 0:58 on 3 of 3; first raid 1:22 to 4:14 | `planet1-pumps.mp4`, seed 1, pumps hand, 0:26-1:15 |
| 2, Sprawler | fast threads find the squad from far off; a mini-boss forms; a pod (fruiting) | mini-boss by 1:25 on 2 of 3, pod by 1:30 on 2 of 3, strikers (0.6) on none by 3:00 | `planet2-hunted.mp4`, seed 2, 1:18-2:00 |
| 3, Trunk | cut under 60% of its peak, its edge condenses into strikers that charge | strikers by 1:30 on 2 of 3, mini-boss by 1:40 on 3 of 3 (a Pinwheel on seed 3); waves and the late lunge later | `planet3-strikers.mp4`, seed 1, 1:26-1:46 |

Before #644, planet 1's fruiting never fired in these runs: its trigger
(a fifth of the slime's mass killed in 30 s) was out of reach of a
40-soldier greedy squad against 10,000 to 15,000 walkers (10 to 13% at
best). At 0.12 it fruited at 1:30 and burst at 2:00 on seeds 1 and 3. Not
landed: #644 moved fruiting to planet 2 after a burst wiped the owner's
squad, and planet 1's pump raid already names it.

Found on the way: a burst threw its walkers 600 fine cells, wider than a
planet 1 map, so they came down round the squad (the owner's 21 of 28 dead
in 3 s). PR #648 holds the throw short of the guns' reach from the nearest
soldier; planet 2 run seed 3, soldiers at 3:00: 8 before, 23 after.

Seen while filming, not followed up: on planet 2 seed 2 the hunter sat on a
rock edge from 1:44, 50 to 100 cells from the squad, and never reached it.
