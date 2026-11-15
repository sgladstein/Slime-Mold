# The alarm field's look: the square glow and the mint wash (2026-11-15)

The slime-enemy thread, taking two items from the whole-game review
(`Reports/whole-game-review-2026-11-15/README.md`): **W27**, "A blast's
glow is clipped to a square", which the gun-look thread traced to the
alarm field and handed on, and **W46**, "The alarm field washes the screen
mint ... Draw it only on the slime, or at a tenth." Both are the view's
haze for the engine's alarm field (`src/world/view.wgsl`, `shaded`), which
a siren's call and every hit lay once the field is on: the Swarmer,
Sporer and Sleeper planets (4-6) run it all game, and a siren turns it on
anywhere. Nothing here changes play: the five films below ran the same
game to the walker (52,556 walkers, 368 shots, 2,785 killed at 42 s).
Page: `page/index.html`, published at
https://claude.ai/artifact/XwB5KRtcCniXSBJVipGSyp (listed in
`Reports/pages.md`).

## W27: the square edge is the walls

**Cause.** A dump of the engine's alarm field (the Fork, `rounds=wide`,
seed 1, at 10 s: peak 83.7, 11,439 cells over 1) is right: it is a
plateau that stops at the Fork's walls, which hold none (the engine's
`trail` pass zeroes every cell off the agar). The view sampled it
bilinearly over all four cells round a pixel, walls included, so the
plateau fell to 0 across the last half-cell before every wall face, and
the haze's front term, which brightens the field's slope, drew that fall
as a bright straight line down each wall. The Fork's stem walls are about
75 cells apart, the review's "straight vertical sides about 80 cells
apart".

**Fix.** `alarm_at` weights only open cells (not rock, wall or the
world's rim, the engine's agar), and returns the open share with the
value; a slope sample that lies wholly in a wall takes the pixel's own
alarm (`alarm_or`), so a wall face has no slope; and the haze fades out
over a wall's first cell by the open share. Holes the hits cut in the
field (the engine zeroes it in each strike) keep their fronts, as those
are the field.

**Test.** `the_alarm_haze_has_no_front_at_a_wall_and_its_strength_is_set`
(`src/world/view.rs`): a plateau of 150 against a wall two cells thick,
the ground's lift (summed RGB, full strength) in the middle of the
plateau and by the face. Planted back to plain bilinear it fails as the
bug looked: 58 in the middle, **349 and 345** half and a fifth of a cell
from the face. Fixed: 58, 52 and 42 (the wall's own drawn edge covers a
little of the last fifth).

**Film.** The Fork, `rounds=wide`, seed 1, centre 280,265 at zoom 6, 10
and 15 s (`page/img/wall-before-*.png`, `wall-after-*.png`, 180 x 180
screen pixels at 2x): the pale line down both faces of the stem's right
wall is gone.

## W46: the wash, and four strengths

`View::set_alarm_look(ground, slime)` scales the haze on bare ground and
on the slime (blended by the film there, the same `under` the soil
detail uses); `larder-world alarm_look=G[,S]` sets it for films.
**Default `ALARM_LOOK` (0.25, 0.25), a quarter**, from (1, 1).

Measured on two scenes, each option against the same frames with no haze
(`alarm_look=0`), over every frame, the lift summed over RGB (so 60 is 20
a channel), the HUD's top strip and bottom bar left out
(`raw/make_page.py`):

| Option | Planet 4 squad fight: mean lift | pixels over 60 | max | Siren call: mean lift | pixels over 60 | max |
|---|---|---|---|---|---|---|
| Full (today, 1) | 57.6 | 27.7% | 356 | 25.1 | 11.6% | 322 |
| A quarter (0.25) | 15.5 | 12.5% | 91 | 6.3 | 1.1% | 82 |
| A tenth (0.1) | 6.3 | 0.0% | 37 | 2.5 | 0.0% | 33 |
| Only on the slime (0, 1) | 0.8 | 0.5% | 248 | 0.0 | 0.0% | 0 |

- Planet 4: `run=1:4` (the Swarmer, alarm keep 0.98), `hand=squads`,
  39-42 s every 0.125 s, the camera on the squad, 640 x 400.
- Siren: the Fork, `hand=idle`, `siren=90,160` on bare ground,
  `centre=100,165,5`, 19.9-22 s every 0.1 s (a call at 20 s).

**What the films show.** Full strength washes the fire zone pale mint,
plants and soil with it. A quarter keeps a soft haze where the fire is
and the siren's call reads as a clear ring. A tenth barely shows, the
siren's ring faint. Only on the slime removes the wash, but the haze on
the slime hardly shows (the slime is bright already) and a siren's call
on bare ground disappears, which breaks "every tool must deliver
something you can see" for the siren; the siren's lamp and horns still
flare red with their light.

**Pick.** A quarter, as the default the build plays, put to the owner as
a blind A/B of the four (key `raw/ab-key.txt`, off the page). The knob
stays, so any of them is one constant.

## Frame cost

The haze takes five samples, each now four ground reads more than
before, only where the field is over 1% of full; where it draws nothing
(strength 0 there) it samples nothing. llvmpipe, planet 4, the squad
fight in view (`bench=45 fps=15 run=1:4 mode=squads hand=squads
screen=1280x800 centre=380,300,6 viewpasses=30`, the view's passes each
timed alone, median of 30 frames at 45 s; raw `raw/bench-*.txt`):

| Build | `base` pass, ms | GPU tail p50 / p95, ms |
|---|---|---|
| Before (plain bilinear, full) | 107.2 | 156.0 / 221.3 |
| After, full | 118.9 | 165.1 / 191.1 |
| After, a quarter (the default) | 118.9 | 168.3 / 197.7 |
| After, no haze (`alarm_look=0`) | 124.5 | 163.3 / 192.9 |

The run with no haze at all, which samples nothing, came out slowest, so
llvmpipe's spread between runs (about 10%) is bigger than the haze's
cost here; the earlier run with the window's own camera, where the fight
was not in view, gave 123.86 against 123.87 ms. The deciding numbers are
the 1070's, booked through the PC runner once the PR is up.

## Not done

- W19, the flinch (alarm keep about 0.9 and weight -1 on every tier):
  next, as its own A/B.
- The haze's colour and its fill/front split are unchanged; the knob only
  scales them.

## Files

- `src/world/view.wgsl`: `alarm_open`, `alarm_at`, `alarm_or`, the haze in
  `shaded`; `V` grew `alarm_ground`, `alarm_slime` and two pads (44
  words).
- `src/world/view.rs`: `ALARM_LOOK`, `set_alarm_look`, `PARAMS_BYTES`
  176, the test.
- `src/bin/larder-world/main.rs`: `alarm_look=`.
- `raw/make_page.py` builds the clips and the table; `raw/ab-key.txt`.
