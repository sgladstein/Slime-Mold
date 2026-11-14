# Starving mould on screen (plan C8, 2026-11-14)

The cloud plan's C8 ([plan](../cloud-plan-2026-11-14.md)): "a panicking
walker's lean and a withering piece should read on screen, drawn from
walker and field state (no drawn-only effect)". Before this, a cut-off
piece of shape C looked like any other until it fell apart.

**Page:** [Starving Mould on Screen](https://claude.ai/artifact/4ZvXqmGhvx7NkaBvbQ2pgd)
(source in `page/`).

## What the view draws now

The view binds the engine's food news (binding 48, the step each walker
last heard of food) and its step counter, read-only (`View::set_news`),
and reads them the way the engine's `panic_to` and withering do:

- **Specks by phase.** A walker whose news is older than `wither_lag`
  (150 steps, 5 s) is cut off. Its speck is amber while it leans (the
  next `panic_lean` steps, 1.5 s, creeping at half pace) and red once it
  bolts. Calm walkers keep the warm speck. Cut-off specks grow with the
  zoom (2 x 2 pixels at zoom 6, at most 4 x 4): at one pixel, 195 walkers
  of a cut piece did not read on the first film. The first try added the
  colour as light, as the calm speck does; on the bright body no pixel of
  a bolting piece came out red (0 pixels with red 100 over green), so the
  speck's colour now replaces the body's (up to 75% amber, 85% red).
- **The ashen fade.** `draw_specks` counts the cut-off walkers a world
  cell (a view-owned buffer, cleared each frame); `prep` sums them over
  the same 5 x 5 cells as the body and fades the mould's hue towards ash
  (and its gloss down) by the share cut off. A piece greys as its walkers
  go stale and thins as they wither; a fed piece keeps its colour.

Nothing is painted: both come from the number the simulation uses to
decide who withers and who panics.

## Films (llvmpipe, Fork, seed 1, farm hand)

- The last stand (`cut=150,365,240,32 cutsquad=365,264,150,4
  centre=365,238,6`), with and without: the walker counts inside the ring
  match to the walker every second (184 at the cut, 195 at +6 s, 109 at
  +7 s, 18 at +9 s, 0 at +14 s), so only the drawing differs. The piece
  greys from about +5.5 s, its specks go amber, then red at the soldiers
  by +7.4 s. A slow-motion GIF covers 2:34 to 2:39 in 0.1 s frames.
- The bolt scene (`cut=150,386,239,22,8,16 centre=386,232,5`): the piece
  inside keeps 219 to 265 walkers through 2:57 and never greys; its news
  stays fresh, so the screen correctly shows a piece that is not starving.

Logs (picture paths left out) and the film scripts are in `raw/`
(`mkfilm.py` and `frame.py` from the Scared Mould page; `mkslow.py` for
the slow clip).

## Cost

`larder-world bench=20 hand=farm seed=1` on llvmpipe, alternating builds:
frame p50 30.37 and 31.58 ms without, 31.20 and 31.86 ms with; GPU tail
p50 23.77 and 23.82 against 23.86 and 23.85. Within run-to-run noise. Each
walker adds one read of its news and, if cut off, one atomic count; `prep`
loads a second 20 x 20 tile. The 1070 timing waits for the PC.

## Wiring

`View::set_news(Some((&e.news, &e.state)), s.wither_lag, s.panic_lean)`
after `set_alive` when the engine withers. The window's call
(`src/bin/larder-world.rs`) was held out of the first PR while the
integration thread split that file (freeze 00:36Z-02:40Z, 11-14); the
films were made with it patched in locally.
