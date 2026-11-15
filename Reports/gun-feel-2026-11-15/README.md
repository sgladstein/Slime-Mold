# Gun feel: a bolder spray, camera shake and hit-stop (2026-11-15)

Two asks the same evening:

- The owner on the spray share A/B (21:37Z): "I see no difference. Am I
  crazy?" ([spray A/B](../spray-share-ab-2026-11-15/README.md)).
- The whole-game review's D6, camera shake and hit-stop; the owner
  (21:41Z): "Let me see the video." The review thread asked for both off by
  default behind keys, about 10 s of each in normal play with heavy fire
  and blasts, next to the same moment with each off.

Page: [Gun Feel](https://claude.ai/artifact/AE3vzcKx9Ajzn5wi7cAwUr)
(archived in `page/`).

## The A/B: no difference to see

The owner was right. In the gun range clip, 25 of the 30 frames of A and
B were pixel-identical (median 1 pixel a frame differing by more than 40
grey levels); they differed only for the quarter second after a volley
landed (5 frames, up to 7,212 pixels). On the Fork about 2% of each frame
differed, short-lived and scattered. The share stays 1 (every kill flies).

Why: a flung walker's droplet added 6 calm walkers to its pixels' speck
word, and `base` lights a calm walker by `per = 0.95 / (1 + (dens /
0.35)^2)`, under 0.02 in the body. Over the slime a droplet all but
vanished; over bare ground it was a pale fleck for a quarter second.

## The bold spray (on)

- **Its own layer** (`view.wgsl` `draw_specks`): a third screen of the
  speck buffer (`View::specks` grew from 8 to 12 bytes a pixel), one word a
  pixel by `atomicMax` of a layer (2 the droplet, 1 its rim) and a heat.
  A head zoom / 3.2 pixels across the radius (was zoom / 4) and its tail,
  inside a dark rim zoom / 12 pixels wide (at least one), so it reads over
  the bright body and the dark ground alike.
- **Hot to cool**: the engine writes a shot's spray droplet's flight a tick
  below 0 (`fly`; the view's only reader of that word), so the view tells
  a killed walker blown out (white-hot `DROP_HOT` while fast, deep orange
  `DROP_COOL` as it slows to land) from a blast's living throw (the mould's
  gold, `DROP_GOLD`, with the same rim).
- **Further**: `Rules::spray_secs` 0.3 -> 0.45, `spray_reach` 5 -> 7 cells.

Measured against main (6d072828), zoom 12, 30 fps: gun range 2.5-4 s, 16
shots and 133 kills in both, 28 of 45 frames differ (up to 6,623 pixels);
Fork hold with plain rounds 30-31.5 s, 218 shots and 1,200 kills by 32 s
in both, 35 of 45 frames differ (up to 10,394 pixels). The engine change
is the sign of a word only the view reads, so the CPU/GPU twins are as
before.

## Camera shake and hit-stop (off)

`feel` (`src/bin/larder-world/main.rs`), after each step's blasts
(`Rounds::pops`, in `step` and `catch_up`). A blast's size is its kills
over 150 plus its throw over 400, up to 1 (a bomb of 6 kills and 80
thrown: 0.24). Plain hits do neither; in a fight every volley would.

- **Shake** (`shake=K`, 0 default): each blast adds `K x (0.12 + 0.6 x
  size)` to a trauma (up to 1) that falls 2.2 a second; the camera moves by
  trauma squared x 12 screen pixels on two sums of sines (`App::shake`),
  for the picture only: the HUD is its own layer and the cursor's aim reads
  the still camera.
- **Hit-stop** (`hitstop=K`, 0 default): the biggest blast of a step, at
  least size 0.2 and none in the last 0.6 s, holds the game and its notes
  (flashes, tracers, the burst's light) still for `K x (0.05 + 0.07 x
  size)` real seconds.
- **`real=1`** for films: past the first frame a headless sequence steps
  the window's way, a frame's real seconds at a time, so a hold shows as
  held frames.

Films: Fork, `hand=hold@150,265 bank=8000 rounds=bomb centre=158,268,12
real=1`, 30-40 s at 30 fps (eight soldiers, 180 bombs).

| | Shots by 40 s | Killed | Notes |
|---|---|---|---|
| Neither | 272 | 3,169 | |
| `shake=1` | 272 | 3,169 | camera moved in 85 of 300 frames; p90 2.2 px, 14 frames 4 px or more, at most 11 |
| `hitstop=1` | 265 by the clip's end (game 39 s) | 3,109 | 16 holds of 1-4 frames, 35 frames (1.2 s of 10) |

Offsets by phase correlation against the frame without shake. With
bombs on every shot, hit-stop slows the game about a tenth in a heavy
fight; that is the owner's call on the page.

## Files

- `page/index.html` and its four side-by-side clips (left off or main,
  right on or this branch; 1600 x 500, H.264).
- `raw/film.sh BIN OUTDIR NAME "ARGS" START N FPS`: a frame sequence by
  `larder-world look=` and its frames as `f000.png`...; clips by
  `ffmpeg -framerate 30 -i A/f%03d.png -framerate 30 -i B/f%03d.png
  -filter_complex "[0:v]scale=800:500[a];[1:v]scale=800:500[b];[a][b]hstack"`.
