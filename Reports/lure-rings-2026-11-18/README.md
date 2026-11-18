# The Lure's rings: the pull drawn as cream hairlines (2026-11-18)

The Look and interface lane's part of the Lure (the owner's card, 11-18
03:03Z: "Lure now, in the three planets"; `NOW.md`). The lab's prototype
drew the steering field in pink, with a haze over the field and a bright
core at the lure ([lab report](../lab-steering-2026-11-17/README.md)). The
pink is the core planet's flora's hue, and the haze and core sat on the
kill zone the Lure is built to make. This report picks the look, builds it
into the view, and films it. Nothing here changes play, and nothing draws
until the Enemy lane's field calls `View::set_steer`. Page:
`page/index.html`, published at
https://claude.ai/artifact/4QyfFzQNXQuAPfvz83WqTe (listed in
`Reports/pages.md`).

## What it is

- **Lines only** (`view.wgsl`, `steer_px` and the block in `shaded`): a
  hairline where the walking distance to the nearest lure is a multiple of
  20 cells (10 dish cells on planet 1), one to two pixels wide at any zoom,
  bilinear over the field's 2 x 2 blocks, so the lines bend round rock and
  walls and stop where the field does. No line at distance 0: the Lure
  building marks the middle.
- **Pale cream**, `(250, 242, 220)`, the team's hue (the art direction's
  rule 3: "your team cream and olive"; the Lure is the player's tool).
  Strength 0.42 at the lure, falling as `0.25 + 0.75 exp(-d / 110)`, and
  30% of that over slime thicker than a film.
- **The rings stand still.** They redraw once a game second with the field.
  A ring that drifted toward the lure would be a drawn pulse, not the field
  (the owner's rule: no drawn-only effects).
- **The Lure's scent light is not lit while the rings are drawn**
  (`shaded`, the light loop). The scent is still laid and smelt; F2 shows
  it.

The field's form was agreed with Enemy (11-18 03:10Z): a word a 2 x 2
block in the engine's `news`, the walking distance in cells + 1, 0 where
the field does not reach; `View::set_steer(at)` takes the word it starts
at, 0 hidden.

## What the films show

Planet 1, run seed 1, the lab's scene: a Lure at dish cell (275, 95) built
at 30 s, three turrets round it, the squad held to the east, pull 10,
1280 x 800, 30 frames a game second, on llvmpipe. The build is the lab's
prototype (`claude/project-thread-4tc75h`) merged onto the flinch look,
since the game's own field is not on main yet; the three films of a zoom
differ only in the view shader (`LARDER_VIEW_WGSL`), so they are one game
(all 180 game lines equal).

"Changed" is the share of the picture (HUD strips cropped) where any
channel differs by more than 12 of 255 from the same frame of the film
with no sign of the Lure at all; "slime" is that share among the slime's
own pixels. Mean over the 180 frames, and the largest frame.

| Film | Changed, mean | Changed, max | Slime, mean | Slime, max |
|---|---|---|---|---|
| Lab's look, zoom 6 (34 to 40 s, `centre=275,95,6`) | 23.3% | 23.4% | 28.9% | 39.3% |
| Now, zoom 6 | 2.1% | 2.1% | 1.9% | 2.1% |
| Lab's look, zoom 12 (37 to 43 s, `centre=268,88,12`) | 65.7% | 66.1% | 57.5% | 64.7% |
| Now, zoom 12 | 1.1% | 1.1% | 1.0% | 1.2% |

The lab's look here is its pink rings, haze and core with the Lure's green
scent light (strength 1.3) on top, as the lab's films had it. With both
gone, the orange at the Lure in the new films is the flinch's flush from
the turrets' shots ([flinch look](../flinch-look-2026-11-17/README.md)),
which the green pool covered. Counts by `raw/make_page.py`; films by
`raw/film.sh`; each film's game lines are in `raw/logs/`.

## Tried and not kept

Each judged from frames of the same scene at zoom 6, a view shader
swapped in with `LARDER_VIEW_WGSL`; only the one kept was filmed in motion.

- **The lab's pink** `(255, 110, 215)`: not tried further. Hue 290 to 320
  is the core planet's flora, the only purple on the map (rule 3).
- **The Lure's green** `(150, 235, 80)` and a **pale lime-white**
  `(215, 245, 170)`: the green lines were lost on the meadow's own green,
  and the lime-white sat between the two (`raw/hues.jpg`: green, cream,
  lime-white, left to right, each still with the lab's fill and core).
- **The lab's fill and core** (a haze of 7% over the whole field and a
  glow of 90% at the lure): a wash over the fight, the thing the flinch
  look removed a day earlier.
- **The Lure's scent light kept, in cream at strength 0.7**: it read as a
  pale mist round the Lure, the haze the alarm lost. In its own green at
  1.3 it turned the slime at the Lure lime.

They share one entry in `Reports/dead-ends.md`.

## Test

`the_lures_field_draws_as_cream_rings_and_its_scent_light_goes`
(`src/world/view.rs`): a field round a lure, reaching 30 cells, at zoom 6
and zoom 2. The ring 20 cells out is lit cream east and north; half way to
it, at the lure's own block and where the next ring would fall past the
reach nothing shows; on slime the ring is under half as strong; the scent
light is lit without the field and not with it; `set_steer(0)` draws the
picture with no field. Planted wrong it fails five ways (llvmpipe, zoom 6):
with the lines at 0 the ring stays (40, 41, 40); with a ring at distance 0
the lure's block goes (43, 44, 43) to (113, 112, 104); with the scent
light left on the two pictures differ; without the slime's 30% the slime
lifts 136 against the ground's 247; with unreached blocks read as distance
0 the ground past the reach goes (40, 41, 40) to (66, 66, 63).

## Frame cost

llvmpipe, 4 cores, the films' scene with the Lure built at 5 s
(`bench=30 fps=15 ... lure=275,95,5 centre=275,95,6 screen=1280x800
viewpasses=30`), the rings drawn against the same shader with the rings'
block compiled out, three runs each, taken in turn (ms a frame; the game
line at 30 s is the same in all six):

| Pass | Rings drawn (three runs) | Rings out (three runs) |
|---|---|---|
| base | 91.4, 103.5, 91.6 | 89.6, 91.9, 107.6 |
| prep | 11.1, 16.0, 12.3 | 10.1, 13.4, 17.0 |
| whole frame, p50 | 146.5, 159.3, 154.8 | 146.8, 148.4, 150.3 |

The rings add four reads of `news` a screen pixel to the base pass. Runs
of one build differ by more than the two sides do (`prep`, which is the
same code on both sides, moves by half between runs), so the reading is:
no cost that shows. The 1070 timing goes in the milestone's one batch.

## Not done here

- Nothing calls `View::set_steer` yet: Enemy's pull request adds the call
  beside `set_crust` in `main.rs` and passes 0 while no lure stands.
- The Lure comes off `run_hidden` (`draw.rs`) when Squad and arsenal's
  building lands, and the Lure's "what is new" line goes in with it.
- The scent light's own code (`draw.rs`, the Lure's `Light` with lift
  below 0, and the shader's branch) stays for a game with no steering
  field; it can go once the Lure never stands without one.
