# A squad you can count: every head above every body (2026-11-18)

Look A3 (`wiki/looks/art-direction.md`), from the lane brief: "Space the
squad so 40 soldiers can be counted; sprite A stays (Scott's pick)." Page:
`page/index.html`, published at https://claude.ai/artifact/F58mCnsvBbT2CMUk2Rhz31
(listed in `Reports/pages.md`).

## Where soldiers stand

Planet 1, run seed 1, greedy hand (a local print of each living soldier's
nearest neighbour, in fine cells; the planet's scale is 2, so a dish cell
is two fine cells):

| Time | Soldiers | Doing | Nearest neighbour: least, quarter, median |
|---|---|---|---|
| 60 s | 21 | at the first crystal | 3.99, 4.02, 4.14 |
| 90 s | 35 | draining it (`Clear`) | 2.57, 3.03, 3.30 |
| 120 s | 42 | walking to the next | 3.68, 3.99, 4.11 |

Elbow room asks 1.6 touch radii between centres, 4.8 fine cells
(`Rules::elbow`). The steering's push (`steer_all`) is a spring, the
overlap times `SPREAD` 5 a second, that lets go a twentieth short of
elbow room, so a squad pulled toward one point stands where the pull and
the push balance: a median of 0.86 of elbow room at 60 and 120 s, and
0.69 at 90 s draining the crystal (14 of the 15 pairs closer than 3.5
fine cells had its `Clear` order, all on standable ground). The drawn figure is 1.15 touch radii,
6.9 fine cells across (`soldier_r`, the 11-14 art review's size for zoom
6), so at 3.3 apart each soldier's shoulders lie over its neighbours'
middles, and whichever was drawn later hid the other's head.

## The look's part: heads on top

Every living soldier's helmet is drawn a second time, above every body
(`Marks::soldier_head`, `HEAD_ONLY` in `view.wgsl`), with a ring of its own
figure a 1.5-pixel outline wide round it. Its pixels are the whole figure's
own, so a soldier standing alone looks exactly as it did (tested to 2 of
255); in a crowd each soldier keeps its white dome and the ring parts it
from the shoulders under it. Seen from above, heads are the highest thing,
so this is also the right order.

A helmet showing at its soldier's own place (the middle of its helmet
white), the same game drawn before and after, 90 s:

| Zoom | Soldiers on screen | Before | After |
|---|---|---|---|
| 6 | 35 | 25 | 35 |
| 12 | 31 | 20 | 31 |

## The standing part, for Squad and arsenal

How close soldiers stand is the steering's (`steer.rs`, Squad and
arsenal's file). With the heads on top a squad of 35 at 0.69 of elbow room
can be counted, but it still reads as one heap of shoulders on the crystal.
Holding elbow room when a squad stands, draining or at rest, would spread
it to the spacing the 11-14 review chose (1.6 touch radii, where the
figures' helmets stand well apart). Sent to them with these films.

## Films

Built from main at 008bfd1 with this change, the before from 008bfd1
itself (same game; main to 667a05b changed no code). Planet 1, run seed
1, greedy hand.

| Scene | Zoom | Soldiers standing | Picture changed (mean, most) |
|---|---|---|---|
| Draining the first crystal, 86 to 92 s | 6 | 33 at 86 s, 36 at 92 s | 0.50%, 0.63% |
| The same | 12 | the same | 1.54%, 1.99% |
| Walking to the next, 118 to 124 s | 6 | 42, 43 | 0.55%, 0.71% |
| The same | 12 | the same | 0.76%, 1.60% |

Changed is the share of the clip's picture where any channel differs by
more than 12 of 255 between the two drawings of a frame.

## Cost

One more mark a living soldier, whose box is a radius across (a quarter of
a figure's): the whole figure's shading runs again there, plus eight taps
of the helmet for the ring. At zoom 6 about 400 pixels a soldier, 16,000
for 40; not timed on the GTX 1070 (the software renderer is not for
timing).

## Files

- `page/`: the page and its clips (H.264, 30 fps, the middle 960 x 600 at
  its own pixels) with posters.
- `raw/film.sh OUT OLD_GAME` films them; `raw/make_clips.py OUT PAGE` makes
  the clips and prints the soldiers standing and how much of the picture
  changed.
