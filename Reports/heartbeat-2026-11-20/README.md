# The slime's heartbeat (2026-11-20)

The third rule from the thread "Make the slime a more interesting enemy",
after fruiting (`Reports/fruiting-2026-11-20/`) and waves (#555). The
owner, 11-20 05:46Z: once he is strong "there is just lots of shooting
through masses of uniform orange".

Page: https://claude.ai/artifact/KJMPgFozverA7Y5NVx3Zy6 (source in `page/`).

## What was built

`src/mould/larder_world/heart.rs` (`HeartRules`, `Game::time_heart`,
`Game::heart_site`), on on every run planet (`run::ENEMY_RULES`' `heart`),
GPU only (`World::set_heart`, `gpu::Engine::heart`, engine.wgsl's
`heart_beat`). Every 6 s a beat runs out from the heart (the home crystal,
`Game::core`, while 30 walkers stand on it, else the crystal the slime holds
with the most) as a ring 80 dish cells long. As it passes, a calm walker
leans away from the heart (25, in the readings' units; the lure's pull is
10) and steps up to 60% faster (`HEART_HASTE`); between beats it leans home.

## Measured

Software GPU, planet 1, run seed 1, greedy hand, one film each, camera on
the home body's edge (`centre=55,100,Z`), 0:40 to 0:54:

| | on | off |
|---|---|---|
| Slime's share of the frame, zoom 12 | 38% to 58%, peaks 6 s apart | 43% to 49% |
| Zoom 6 | 26% to 37% | 28% to 30% |
| Walkers killed by 0:54 | 1,242 | 707 |
| Slime at 0:54 | 11,536 | 12,089 |

A lean of 12 swung the zoom 4 frame's slime 1,193 to 1,370 (hundreds of
pixels); 25 swung it 1,124 to 1,453 and was kept.

On its own the beat feeds the guns: each swell carries slime into range.
The next step is to make the beat carry a threat (a surge on the beat, or
the balance thread's lunge for every slime), so the swell reaches the squad.

## Commands

```
larder-world look=DIR run=1:1 mode=squads hand=greedy set=heart=1 centre=55,100,12 at=40,40.5,..,54
python3 make_clips.py DIR OUT.mp4
```

Not checked: the GTX 1070's frame cost (a sine and a square root per calm
walker per step), planets 2 to 8, and how it reads against a fort.
