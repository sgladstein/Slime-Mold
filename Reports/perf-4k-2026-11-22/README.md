# The owner's frames at 4K (2026-11-22)

Performance thread ("Improve game performance"). Why the game ran at 20 to
30 frames a second in the owner's play while the GTX 1070 batches said 5 ms,
and what changed.

## What his play logs said

The `play-logs` branch keeps each second's frame p50 and p95. The median
second's p50, build 3072 (11-22, planets 1 to 4): 34 to 50 ms; its p95 99 to
130 ms. On 11-21 (builds 2880 to 2978) the p50 was 7.6 to 31 ms.

## What his PC measured

A Remote Control runner on Scott-Desktop, his play folder's built exe
(a54867a0), `bench=90 run=1:3 mode=squads hand=strat:rush,aim=6`. His screen
is **3840 x 2160 at 60 Hz, Windows scale 250%**. The GPU was not idle: his
own game sat open on the menu (95% GPU) until partway through b, and the
strategy league ran headless jobs.

| run | screen | frame p50 / p95 ms | GPU tail p50 ms |
|---|---|---|---|
| a | 1280 x 800 (his game open) | 14.8 / 68.4 | 13.6 |
| b | 3840 x 2160 | 41.2 / 66.0 | 40.0 |
| c | 3840 x 2160, zoom 6, view passes alone | 44.5 / 47.5 | 43.5 |
| d | planet 4, 3840 x 2160 | 42.8 / 47.8 | 41.7 |

Run c's view, each pass alone (median of 30 frames): base 38.0 ms of 41.8
(91%), prep 2.5, compose 0.75, marks 0.49. The simulation stays about 1 ms a
step. So the frame is the view's per-pixel cost times 8.3 million pixels: 8.1
times the 1280 x 800 the batches timed.

## What changed

- **#720**: the `pixels` crate uploaded a window-sized CPU buffer every frame
  (14.7 MB at 2560 x 1440, 33 MB at 4K) that the view then overwrote. The
  bench, off-screen, never paid it. Now `blit.rs` draws the view's buffer
  straight to the window. Each play-log second also gets a `frames` line:
  window px, zoom, and ms in step, sound, prep, swap-chain wait, record,
  present.
- **World detail** (his card, 08:51Z: "Half resolution on 4K, with a
  switch"): `View::set_scale` draws the world at half each way and spreads
  it back over the screen, nearest. The HUD stays at full size. Options,
  Screen tab: Auto (half above 2560 x 1600), Full, Half. On llvmpipe at
  3840 x 2160, zoom 12, the view's passes take 1000.9 ms at full and 255.0 at
  half (base 912.6 to 204.7): 3.9 times less.

Stills (1440 x 900 window under Xvfb, planet 1, a crop): `planet1-full.jpg`
and `planet1-half.jpg`. The world is a little softer at half. Soldiers,
rings, the squad's number and the HUD stand the same size.

## Still open

- Planet 3's once-a-second `mark_bulwark` / `set_cells` read-back waits for
  the GPU's queue: 3 to 10 ms a call, up to 76 ms with a 4K frame queued. It
  should shrink with the queue. Time it again at 4K after World detail lands.
- A game left on the menu keeps the card busy (the dish behind the menu
  runs at full size). At Auto it now draws at half too.

## Quality steps and the detail they kept (2026-11-24)

The owner approved a quality slider (03:15Z). World detail became Auto,
100%, 85%, 75%, 67%, 60%, 50%, 40% (#747; bilinear between whole ratios,
Auto under 2.4 million pixels, so 50% at 4K). His play logs from #725 on
(11-22 20:50Z to 11-23 17:33Z, planets 1 to 6, about 3,700 s at 50%) ran
60 frames in 99 to 100% of seconds.

Filming the steps at his 3840 x 2004 window (planet 2, zoom 7) showed that
below 100% the ground's plates and cracks faded and the slime's body went
to square blobs: the view's detail fades and `prep`'s fine cells read the
small picture's zoom (3.5 at 50%), as if zoomed out. They now read the
screen's own zoom (`look_zoom`, `View::grid`). 100% is byte-identical.
`detail-steps-2026-11-24.jpg`: 100%, then 50% and 67% before and after.

Cost on lavapipe at 3840 x 2004, 50%, zoom 8 (view passes, median of 5
frames): 198 to 224 ms; `prep` 10.3 to 43.6 (four times the fine cells,
as at 100%), base unchanged. On the 1070, `prep` at full 4K was 2.5 ms
(11-22), so expect about 2 ms more a frame; to be timed there.

## On the GTX 1070 at his window (2026-11-24)

A Remote Control runner on Scott-Desktop, the build bbebde5 (#747 and
#753), `bench=60 run=1:3 mode=squads hand=strat:rush,aim=6
screen=3840x2004 detail=D centre=larder,6 viewpasses=30`. The league was
paused, no game was open, and the GPU was otherwise idle. Frame p50 / p95, ms:

| detail | 100% | 75% | 67% | 60% | 50% | 40% |
|---|---|---|---|---|---|---|
| planet 3 | 26.6 / 27.8 | 18.2 / 19.3 | 15.7 / 16.8 | 13.9 / 15.0 | 11.4 / 12.5 | 9.4 / 10.4 |
| view, ms | 25.4 | 16.7 | 14.3 | 12.5 | 10.1 | 8.0 |

`prep` is 2.38 ms at every step, the cost #753 brought below 100%.

At the slime's peak, at 50% with `hand=idle` (his own peaks were planet 5
at 136k and planet 8 at 209k at landing, 11-22 and 11-23):

| planet | mould | frame p50 / p95 ms |
|---|---|---|
| 5 (219 s) | 132k | 14.5 / 15.5 |
| 8 (150 s) | 221k | 17.7 / 19.5 |
| 8 at 40% | 221k | 15.2 / 17.0 |
| 8, 1280 x 800 at 100% | 221k | 9.0 / 10.9 |

Planet 8 per frame: view 12.2 ms (base 8.8, prep 2.5, compose 0.7), and
the simulation 12.4 ms of GPU a step (sense 4.75, egain 1.44, trail
1.17, life 0.84, scent 0.82), 3.1 ms a frame at 15 steps a second. The
simulation's time is the same at 1280 x 800. The CPU step's
`time_bosses` takes up to 60 ms in a single call, about every 45 s: a
hitch the p95 misses.

What followed (Direction's picks): Auto now steps down toward 40% while
frames run long (#759, `autodetail.rs`), and the Enemy lane makes `sense`
cheaper and the boss turns shorter. A smaller planet 8 map is the fallback.
Planet 8 is timed again once both are in.

### Planet 8 again, 11-24 afternoon: a slower view found, and the walker sort

Same bench (planet 8, seed 1, `hand=idle`, 150 s, 3840 x 2004, the GTX
1070 otherwise idle; a lock file now keeps timing runs on his PC from
overlapping). Frame p50 / p95 ms, every frame and frames with a game step:

| build | detail | every frame | step frames | view base | GPU a step |
|---|---|---|---|---|---|
| main d30f7bdd | 50% | 19.4 / 21.2 | 20.3 / 21.3 | 10.46 | 12.37 |
| main d30f7bdd | 40% | 16.2 / 18.2 | 17.2 / 18.3 | 7.43 | |
| + #787 | 50% | 17.6 / 19.6 | 18.6 / 19.7 | 8.99 | |
| + #787 | 40% | 15.2 / 17.1 | 16.0 / 17.2 | 6.48 | |
| + #787 + sort (#789), `sort=off` | 40% | 15.2 / 17.1 | 16.1 / 17.2 | 6.41 | 12.29 |
| + #787 + sort (#789) | 40% | 13.6 / 14.1 | 13.9 / 16.4 | 6.43 | 5.60 |
| + #787 + sort (#789) | 50% | 16.1 / 16.6 | 16.4 / 18.9 | 8.95 | |
| + #787 + sort, Auto (median rule) | 50% to 126 s, then 40% | 16.1 / 20.6 | 16.5 / 20.4 | | |
| planet 5 (220 s), + #787 + sort, Auto | 50% | 14.5 / 15.4 | 15.1 / 15.6 | | |

- The Enemy lane's cheaper `sense` (#775) measured 4.75 to 4.69 ms: about
  nothing.
- The view's base pass had grown 1.5 ms on main. A bisect named the merge
  of the walkers' dashes off crystals (3acd82b, the Look lane) with #753:
  each alone was quick (8.6 and 8.9 ms), together 10.5, so an extra crystal
  read in `base`'s hot path. #787 decides it in `shaded` instead and wins
  it all back (planet 2 draws bit-identical).
- The GPU walker sort by tile (#789, the Engine lane) cuts the
  simulation's step from 12.3 to 5.6 ms (`sense` 4.63 to 0.92, `egain`
  1.46 to 0.28).
- Auto judged each second by its median frame, 16.1 ms at 50%, while the
  quarter of frames with a step ran 18.9 at their p95: it kept 50% with
  every fourth frame late. It then judged by each second's 90th
  percentile (#795), timed again with the sort: planet 8 at Auto p50 14.1,
  p95 16.7 over every frame and 18.8 over frames with a step, still at 50%
  for the first 90 s (about a frame in twenty late). It now judges by the
  95th (`autodetail::PCT`), which ignores up to three hitches a second.
