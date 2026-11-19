# Squad auto-fire among loose walkers (2026-11-18)

Squad and arsenal. Scott, 11-18 17:01Z: among loose walkers he lets go of the
fire button, "but the soliders seem to shoot much slower or not all of them
are shooting". Page: https://claude.ai/artifact/EdTkz7B7MwwHV7FbRtgGE3
(source in `page/`).

## Why

- The commanded squad's fire mode was read as defend whatever the player
  chose (`Game::target`), so his "free" (log, 70.8 s and 346 s) did nothing,
  and its attack-move stood to fight (`Game::engaged`) slime it would not
  shoot.
- Defend fired only when the engine's aim, the thickest block in the gun's
  whole 24-cell reach, lay within `Rules::defend` 7.5 cells. A thicker clump
  further off hid walkers on the squad; touched, it fired past them at the
  clump.
- His planet-1 log of 11-18 15:32Z (play-logs branch): button held 328 s,
  23.9 shots and 143.5 kills a second; on its own 117 s, 4.5 shots and 10.9
  kills a second. Planet 2 (15:42Z): 22.9 and 85.7 held, 12.3 and 55.9 on its
  own. The lead rate (2x) applies both ways, so rate was not the cause.

## Fix

`Game::stance_of` (the commanded squad keeps hold, defend or free; attack-move
fires freely) and `Game::sight` (a defending soldier with nothing to guard, no
crystal to clear and off the core reads only 7.5 cells plus half a block's
diagonal, so each soldier aims at the thickest slime near itself). Test:
`the_controlled_squad_fires_by_its_mode_and_defends_what_is_near`.

## Measure

Planet 1, run seed 1795015831, `mode=squads`, a measuring hand (not
committed) that buys to eight soldiers, takes command, holds them at
(340, 75) and either sets defend, free, or volleys every frame at the
thickest block a member reads. Raid 1 reaches them at 68 s. At 150 s:

| | shots | killed | health |
|---|---:|---:|---:|
| old, defend or free (identical) | 804 | 3,058 | 60% |
| old, volley | 1,632 | 3,117 | 41% |
| new, volley | 1,700 | 3,032 | 49% |
| new, defend | 1,208 | 3,166 | 83% |
| new, free | 1,613 | 3,204 | 75% |

One scene and seed: kills are close because the raid brings a fixed number;
the difference is the bites taken. Clips: before and after (defend) at zoom 6,
64-90 s, and zoom 12, 68-80 s, on llvmpipe, frames joined by
`make_clips.py`.

## Spreading the volley (tried, not kept)

He asked for it (17:40Z: "if that improves my killing sure"). Five read
probes round the volley point (one at it, four on a ring 4 dish cells out),
each member firing at one of their finds in turn, only finds it can see. Kills
paired against the single point on main after #495 (the raid ramp had moved
the first raid later, so the loose scene is the early walkers), same build
with `set=volley_spread=0`:

| post | point | spread | change |
|---|---:|---:|---:|
| main body's edge (170,105) | 3,103 | 3,491 | +12.5% |
| (166,109) | 3,437 | 3,995 | +16.2% |
| (174,101) | 2,866 | 3,157 | +10.2% |
| stream past a rock (204,158) | 2,498 | 2,398 | -4.0% |
| (200,162) | 2,832 | 2,573 | -9.1% |
| (208,154) | 1,838 | 1,797 | -2.2% |
| loose walkers, the start (340,75), 150 s | 1,496 | 1,397 | -6.6% |
| (336,79) | 1,401 | 1,375 | -1.9% |
| (344,71) | 1,697 | 1,389 | -18.1% |

Among loose walkers the squad kept 10 to 14 points more health, but killed
fewer. Keeping only finds half as thick as the thickest changed nothing clear.
The finds held 0 to 6 walkers a block in every scene, so the game has nothing
to tell the edge where it wins from the rest. Not kept: `Reports/dead-ends.md`.
