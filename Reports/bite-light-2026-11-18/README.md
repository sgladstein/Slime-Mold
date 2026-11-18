# A bite lights only what it bites (2026-11-18)

Look A4 (`wiki/looks/art-direction.md`): no effect bigger than its damage
(the owner, 11-14 20:22Z: "only the size of the damage. The same for all
affects"). Page: `page/index.html`, published at
https://claude.ai/artifact/Wk5DevY7VGNnCAN21hAPcb (listed in
`Reports/pages.md`).

## Every light the fight makes, against its damage

From `src/bin/larder-world/draw.rs`, in dish cells (the planet's scale
turns them into fine cells):

| Light | Reach | Its damage | |
|---|---|---|---|
| A shot's hit | the hit's radius (`tr.r`) | the same | equal |
| A round in flight | its strike's disc (`Bullet::r`) | the same | equal |
| A burst | the blast's radius (`burst.r`) | the same | equal |
| A flare | half its alarm's reach | none (a signal) | its own rule, smaller |
| Muzzle flash, a waiting round's ember | 1 (`MUZZLE_LIGHT`) | none | a point light |
| **A bite** | **3.5**, centred a touch out | the soldier's touch, 1.5 (`Rules::touch`) | **2.3 times as far** |

Lights fall off as (1 - d²/r²)², so nothing shows past the reach. Only the
bite's was bigger than its damage: centred a touch out on the bitten side,
it lit the slime out to 5 cells from the soldier, 3.3 touches, a disc 5.4
times the area of the bite. A soldier wading in is bitten many times a
second (`step_notes` sets the flinch from the real damage), so a squad in
the slime stood in a flickering orange pool that also tinted the figures
and drowned the slime's own light, the hits included.

## Now

The bite light reaches the soldier's touch (`g.rules.touch * sc`), still
centred a touch out on the side the slime bit from, at the same colour and
strength. One number changed; nothing else.

## Films

Built from the heads-on-top branch (Look A3, PR 494) with this change, the
before from that branch without it: the same game.

| Scene | Zoom | Soldiers standing | Walkers touching a soldier (mean) | Picture changed (mean, most) |
|---|---|---|---|---|
| The Fork held at 150,265, plain rounds, 41 to 45 s | 12 | 7 at 41 s, 5 at 45 s | 5.4 | 2.34%, 6.80% |
| The same | 6 | the same | the same | 0.51%, 1.48% |
| The Fork, the first wave, 23 to 27 s | 12 | 8, 7 | 3.3 | 1.50%, 4.54% |
| The same | 6 | the same | the same | 0.34%, 1.04% |
| Planet 2, run seed 1, greedy, 86 to 92 s (not on the page) | 6 and 12 | 20, 20 | 0.2 to 2.0 | 0.07% and 0.21% |

The two Fork windows are where the hold is most bitten (a scan every 2 s
from 10 to 50 s: 3.9 walkers a soldier at 24 s and 6.7 at 42 s, 1.1 at most
everywhere else). On planet 2 the greedy squad stands at the slime's edge,
so few bites and little change. Changed is the share of the clip's picture
where any channel differs by more than 12 of 255 between the two drawings
of a frame; most of it is the old light's wash (about four in five changed
pixels are darker after, at zoom 12).

## Seen

- **Each bite is a small hot glow at the soldier's edge**, on its side,
  gone in a moment; at zoom 6 it is about a figure's radius across.
- **The squad keeps its olive** while the slime is on it, and the slime
  round it keeps its gold, so shots' hits and bursts read against it.

## Cost

None: the same light, smaller (fewer pixels inside its reach).

## Files

- `page/`: the page and its clips (H.264, 30 fps, the middle 960 x 600 at
  its own pixels) with posters from each pair's most changed frame.
- `raw/film.sh OUT OLD_GAME` films them; `raw/make_clips.py OUT PAGE` makes
  the clips and prints the soldiers standing, the walkers touching and how
  much of the picture changed.
