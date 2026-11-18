# The Lure in play, with its round pull (2026-11-18)

The Look and interface lane's films of the Lure in the game, owed since its
rings (#458; [lure rings](../lure-rings-2026-11-18/README.md)) and held for
the Enemy lane's round field (#478, merged 14:40Z). Page: `page/index.html`,
published at https://claude.ai/artifact/QHdZD8LkMJ9E5ZaTDv3Aw2 (listed in
`Reports/pages.md`). Films only: nothing in the game changed here.

## Scenes

Run seed 1, `mode=squads bank=20000`: three rendering turrets (`turret2`)
from 0:20 round a Lure built at 0:30, eight soldiers holding beside them,
`watch=` counting the slime's walkers within 30 cells of the Lure and from
30 to 60 each second. Zoom 6 from 36 to 42 s and zoom 12 from 40 to 46 s,
30 frames a game second; the whole planet at 40 s. Built from main at
8c4fb6c with the droplets (#482) on top.

| Planet | Lure | Within 30 cells | 30 to 60 cells |
|---|---|---|---|
| 1 | (275, 95) | 91 at 36 s, 198 at 40 s, 147 at 46 s | 225, 226, 230 |
| 2 | (100, 168) | 365, 537, 628 | 678, 940, 1,016 |
| 3 | (357, 182) | 513, 404, 314 | 464, 386, 471 |

The handoff's earlier scouts (octagonal field, 11-18 morning) had 400 to
500 within 30 cells at 40 s on each planet; with the round field planet 1
has 198 at 40 s. Not a census: one seed, and the turrets kill what comes.

## Seen

- **Round rings on open ground** that bend where the walk is longer (planet
  2's rock), and no ring at the pull's edge (#477).
- **The slime streams in along them** from the pull's edge on all three
  planets, and the turrets thin it as it comes.
- **Faint over the body at zoom 12**: the rings are hairlines, plain on bare
  ground and quiet over the bright slime. Left as it is: they are a sign,
  not the fight.
- **The scent light is never lit**: the steering field is drawn the whole
  time a Lure stands (`Engine::lured`), and the green scent light shows
  only where no field is drawn. Its code (`draw.rs` Lure light,
  `view.wgsl` lift below 0) stays while Squad and arsenal asked to keep it.

## Files

- `page/`: the page, six clips (H.264, 30 fps, the middle 960 x 600 at its
  own pixels) with posters, and the three whole planets.
- `raw/film.sh OUT` films them; `raw/make_clips.py OUT PAGE` makes the
  clips and prints the counts.
