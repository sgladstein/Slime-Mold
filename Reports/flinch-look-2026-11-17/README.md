# The flinch's look: a flush on the slime, not a wash over the fight (2026-11-17)

The Look and interface lane, the first piece of Look A (the fight round
the squad). The planning review of 11-17 saw, in frames at zoom 6 and 14,
that "the flinch field's haze draws as a lime wash in cell-sized blobs
over ground and slime alike, and is the largest thing on screen where the
player aims". This report finds where the wash came from, removes it, and
shows the alarm on the slime it acts on instead. Nothing here changes
play: each before and after film below is the same game to the walker
(all 180 game lines equal; at 20 s, 11,493 walkers, 103 shots, 1,782
killed). Page: `page/index.html`, published at
https://claude.ai/artifact/XqBjSyq4eqiTdzdgE6Wqxg (listed in `Reports/pages.md`).

## Where the lime came from

Two things drew the engine's alarm field (the germ fields' second block),
and the brief named the smaller one.

1. **The engine's own paint** (`src/gpu/engine.wgsl`, `draw`): the alarm
   added `(30, 110, 40) x (1 - exp(-alarm / 8))` to the colour the engine
   hands the view, on every open cell. It was made on 11-14 for the Alarm
   trail card, by the owner's word that a trail a bullet leaves is seen
   (08:40Z), and Bait replaced that card the same day (09:02Z). The paint
   stayed. It is nearly full from an alarm of 20. Since the flinch (11-15)
   every shot on planets 1 to 3 lays an alarm of 50 (`squad::SHOT_ALARM`)
   within twice its strike radius, so the paint was at full round every
   firing squad, all game. The view takes that colour as the slime's hue
   and as the soil's tint, so the slime went lime and the ground mint.
2. **The view's haze** (`src/world/view.wgsl`, `shaded`): a pale light over
   open ground and slime, at a quarter of its first strength since the
   alarm-look report (`ALARM_LOOK` 0.25, key `alarm_look=`).

The check that told them apart: with `alarm_look=0`, which draws no haze,
the films were still green. So the haze was the lesser part, and the
strength the owner was to pick for it could not have fixed the wash.

## What it is now

- The engine no longer paints the alarm (the crater's ember, the same
  pass, is unchanged).
- The haze and its strength are gone: `ALARM_LOOK`, `View::set_alarm_look`
  and the key `alarm_look=` are deleted, not left as a switch.
- **Slime under alarm flushes warm** (`flush_at`, `flush_mat` in
  `view.wgsl`, applied in `prep`, once a fine cell): its colour is
  multiplied toward `(1, 0.76, 0.58)` and gains a glow of `(24, 4, 0)`, by
  `1 - exp(-alarm / 75)` (half the alarm that reads full, a siren's 150),
  times the open share of the sample (no wall bleeds in, as before) and
  the slime's thickness from 0.004 to 0.05. One shot's alarm of 50 is a
  flush of 0.49. Gold slime at (255, 197, 78) goes to (255, 161, 54) at
  zoom 6, and (252, 193, 77) to (255, 158, 52) at zoom 2.
- **Bare ground shows no alarm.** The tools that lay one show by their own
  light (below).
- A mini-boss's skin and the set slime are laid over the flush, so they
  keep their own colours.

The flush is orange, not red: the crater's ember stays the hottest thing
in the fight, and red stays for danger (the style guide's rule 3).

## What the films show

Planet 1, run seed 1, `hand=squads`, eight soldiers, 1280 x 800, 30 frames
a game second, on llvmpipe. "Before" is main at 41bb764. Green is the
share of the picture (HUD strips cropped) clearly greener than it is red
or blue; hot is the share where red is at least 1.9 times green and blue.
Mean over the 180 frames, and the largest frame.

| Film | Green, mean | Green, max | Hot, mean | Hot, max |
|---|---|---|---|---|
| Before, zoom 6 (14.5 to 20.5 s, camera on the squad) | 4.7% | 6.9% | 0.04% | 0.09% |
| After, zoom 6 | 0.4% | 0.4% | 0.64% | 1.55% |
| Before, zoom 12 (15.5 to 21.5 s, `centre=126,152,12`) | 19.3% | 32.6% | 0.04% | 0.14% |
| After, zoom 12 | 0.9% | 1.2% | 2.98% | 6.93% |

The green left after the change is the fen-coloured water and moss of the
map, not the alarm. Counts by `raw/make_page.py`; films by `raw/film.sh`;
each film's game lines and the four bench runs are in `raw/logs/`.

**The Swarmer's planet**, where the alarm is the tier's own, lasts longer
(keep 0.98) and pulls the slime in: planet 4, seed 1, the squad's frame at
20, 40 and 60 s (stills, same game both sides: 50,376 walkers, 401 shots,
4,328 killed at 40 s). Green before 17.9%, 12.8%, 7.9% of the picture;
after 1.5%, 2.0%, 1.8% (the planet's own moss), hot 1.2%, 0.7%, 0.0%. The
flush is on the stream that floods to the wound and nowhere else; the two
big bodies stay gold. This is the planet where, by the alarm-look report,
the haze at full strength washed the frame mint.

## The tools that lay an alarm: the Flare and the siren

The owner's word of 11-14 08:40Z on the scorch and alarm trails was "They
should be visible though" "100%", and the green on bare ground was built
from it. Two tools lay an alarm today, and both must still be seen where
there is no slime to flush.

- **The Flare** (`Card::Flare`) had no other sign: its strike has no
  radius, so no burst light, and what showed was the haze over its whole
  reach of 16 dish cells, far larger than anything it did. A burning flare
  is now an ember (`Marks::streak`) and a hot red light (`Light`, strength
  `FLARE_LIGHT` 2.5; at 1 it did not read on the dark ground at zoom 6)
  for as long as it burns (`Rounds::burning`), guttering out over its last
  `FLARE_GUTTER` 0.5 s, reaching half its alarm's radius
  (`src/bin/larder-world/draw.rs`). Planet 1 seed 1, a free flare on bare
  ground at 16 s (`boom=16,165,168,flare`), the frame at 18 s: green 15.9%
  of the picture before, 0.3% after, hot 1.2%; same game both sides
  (11,765 walkers, 95 shots, 1,602 killed at 19 s).
- **The siren** keeps its red lamp and light for the call
  (`SIREN_CALL`), and the slime its call reaches flushes. The front that
  spread over bare ground is gone with the haze. The siren is held out of
  the run today.

Every other way the player lays an alarm already shows where it lands:
a shot, a Blast, a Bomb and the kit's blast by their hit light and their
crater, and fire by the fire (`rounds.rs`, `kit.rs`, `buildings.rs`: every
`Burst` with an alarm). So an alarm is seen as the thing that lays it and
the slime it acts on, not as the field on empty ground. `wiki/decisions-game.md`'s row for
11-14 08:39 says the alarm is drawn on bare ground from its field; that
row is Direction's to amend.

## Tried and not kept

Frames of the same fight before the pick (planet 1 seed 1, zoom 6, at 16
and 20 s), each candidate a view shader swapped in with `LARDER_VIEW_WGSL`.
They were judged from those frames; only the one kept was then filmed in
motion.

- **Blanch** (the alarmed slime toward white): the front line washed out
  exactly where the squad aims, and it sat too close to the muzzle and hit
  lights, which are the brightest things by the value order.
- **Dim** (the alarmed slime darker): read as dirty or starving slime, and
  the starve ramp already owns "greyer and darker".

Both have a line in `Reports/dead-ends.md`. The flush was first built per
pixel in `shaded`, where the haze was, and moved to `prep`: the slime's hue
is sampled once a fine cell there already, and the alarm is no finer than
a cell.

## Test

`slime_under_alarm_flushes_hot_and_bare_ground_shows_nothing`
(`src/world/view.rs`, replacing the two haze tests): two gold pieces, an
alarm disc of radius 10 round one, at zoom 6 and zoom 2. The alarmed
piece's green falls by more than 20 and its blue falls with its red kept;
the other piece and the bare ground 4 cells from the slime, inside the
alarm, change by at most 2; the other germ side shows nothing. Planted:
with `flush_mat` off the slime stays (255, 197, 78); without the slime
gate bare ground goes (40, 41, 39) to (61, 36, 26).

## Frame cost

llvmpipe, 4 cores, `bench=45 fps=15 run=1:1 mode=squads hand=squads
screen=1280x800 viewpasses=30`, two runs each, taken in turn (ms a frame,
the view's passes; the game line at 45 s is the same in all four: 11,081
walkers, 330 shots, 4,122 killed):

| Pass | Before (two runs) | After (two runs) |
|---|---|---|
| base | 90.0, 87.2 | 79.5, 84.7 |
| prep | 5.42, 5.80 | 3.93, 5.86 |
| marks | 1.72, 1.28 | 1.10, 1.66 |
| compose | 4.64, 3.93 | 3.43, 5.02 |
| whole frame, p50 | 140.3, 134.9 | 134.6, 130.4 |

Runs of the same build differ by up to 10% (marks and compose, which did
not change, move as much as anything), so the reading is: no dearer. The
base pass has lost the haze's samples at every pixel and `prep` has gained
one alarm sample a fine cell; neither shows above the spread. The engine's
`draw` lost one branch (the step's drawing 1.99, 1.92 ms before; 1.86,
1.84 after).

The 1070 timing goes in the milestone's one batch.

## Not done here

The rest of Look A: shots as short bright streaks, droplets in the
slime's material, a squad of 40 you can count, and the check that no
effect is bigger than its damage. The list is in
`wiki/looks/art-direction.md`, *Look jobs, ranked*.
