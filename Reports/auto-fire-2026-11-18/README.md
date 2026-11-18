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

Not done: the volley still puts every bullet on the cursor point. Spreading it
over the thickest blocks near the cursor is the next step if he asks.
