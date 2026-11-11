# World camera and UI (2026-11-11)

**Built and timed on the GTX 1070 (0.32-0.42 ms a frame at 1920 x 1080); not yet played.** The owner asked for a
camera and UI thread for the Larder world (2026-11-11 14:06Z, "We should do
a camera UI thread too as you suggested"). Larder is moving from the 256 x
160 CPU dish onto the GPU as a world bigger than the screen ("A longer
persistent world", 14:06Z). The Larder, GPU and creature threads own the
rules, the economy and the species. This thread owns seeing the world and
giving orders in it. Page: https://claude.ai/artifact/X4b5Nmwgkd11paVF8RS1mj (copy in
[`world-camera-2026-11-11/`](world-camera-2026-11-11/), open `page.html`).

## What was built

1. **The GPU engine is a library** (`slime_mould::gpu`, PR #63): the shader
   is `src/gpu/engine.wgsl`, and `Engine`, `Settings` and the rest are
   public, so the Larder world and the sandbox share one engine. Code
   changed only in `pub` and paths. At scale 2, 25,000 walkers, 2 s, seed 1,
   the picture is bit for bit main's.
2. **`slime_mould::world`**, which has no game rules:
   - `Camera`: the world point at the screen's corner, plus a zoom. The
     wheel zooms at the cursor in whole steps from 1 to 8 (each cell an
     equal block), and in steps of 1.25 below 1 down to the fit. Middle drag,
     the screen edge (6 px, 1,100 px/s) and the arrow keys pan. The view is
     clamped to the world, and a world smaller than the screen is centred.
   - `View`: three compute passes after the engine's draw. `base` resamples
     the engine's frame through the camera; zoomed out, each pixel is the
     mean of up to 4 x 4 cells under it, so thin tubes fade rather than
     flicker. It also draws the minimap: bottom right, the world's shape, at
     most 240 px, hidden when the whole world is on screen. The camera's
     view on it is a white outline. `draw_marks` gives each mark a workgroup,
     and each mark claims the pixels it covers with `atomicMax` of its index,
     so a later mark always draws over an earlier one. `compose` colours the
     claimed pixels and lays the screen-space overlay (the HUD) on top.
     There is a fog-of-war hook (a world-sized 0-1 field the view darkens by)
     that is unused, because the owner has not decided on fog.
   - `Marks`: disc, ring, rect, outline, bar, 5x7 glyph (labels beside
     things), and dashed lines. Sizes are in world cells or screen pixels,
     up to 2,048 a frame (the Larder thread's budget is about 1,000).
   - `Controls`: Larder's controls ported to world cells. Left click picks
     within 10 px at any zoom. A left drag box-selects the local team only
     (Shift adds). A right click gives an order. F + drag draws a burn line.
     Ctrl+1-9 sets a group and 1-9 recalls it (a second press within 0.5 s
     centres the camera on it). On the minimap, a left click moves the camera
     and a right click orders there. Other keys pass through to the game.
     All of this comes out as `UiCommand`s, which the game maps to its own
     commands.
3. **The sandbox draws through it** (`mouldgpu`). The window no longer has to
   fit the dish. W switches to a 2048 x 1280 world with 4x the walkers, M
   toggles the minimap, Q brings out a stand-in squad (markers that walk to
   orders in two teams, with no rules), and middle drag pans (infect is on I
   only now). The panel is screen-space and shows the view's GPU time.
   Headless, `view=WxH cam=X,Y,ZOOM squad=1` writes the frame through the
   camera and times the view pass.

## Checked

| Check | Result |
|---|---|
| Today's 1024 x 640 dish through the camera at zoom 1 against the engine's own frame (scale 4, 3 s, seed 1) | 0 of 573,440 pixels differ (rows clear of the panel) |
| Each screen pixel shows the cell the camera puts there (GPU test, zoom 1 and 3, a 100 px screen so the row stride is in play) | pass on lavapipe |
| Minimap, marks over marks, overlay on top (GPU test) | pass on lavapipe |
| Camera and controls (15 tests: inverse transforms, zoom keeps the point under the cursor, clamp, edge scroll, team-only selection, pick at three zooms, burn, groups, minimap orders, middle pan) | pass |
| View pass on lavapipe, 1280 x 800, 49 marks | 15.5-15.8 ms at zoom 1 and 4, 17.2 ms at the fit, 26.9 ms at 0.75. **A CPU emulating a GPU: not a frame cost** |

## On the GTX 1070 (11-11)

Run by the Larder thread through Remote Control on Scott-Desktop, branch at
`57f63b2`, release build, NVIDIA driver 582.66:
`cargo run --release --bin mouldgpu -- look=out seconds=10 scale=8 walkers=400000 view=1920x1080 squad=1 cam=...`.
The dish is 2048 x 1280 with 400,000 walkers and seed 1. Each view time is the
median of 21 frames.

| Camera | View pass, GPU | Rest of the frame (2 ticks + draw), GPU | Wall a frame, median (worst) |
|---|---|---|---|
| zoom 1, centre | 0.333 ms | 7.826 + 0.250 ms | 8.263 (10.665) ms |
| zoom 4 | 0.315 ms | 7.832 + 0.250 ms | 8.241 (10.327) ms |
| asked 0.75, clamped to the fit 0.84 (1080 / 1280) | 0.415 ms | 7.827 + 0.250 ms | 8.228 (10.345) ms |

So the camera, minimap, 49 marks and overlay cost about 0.3-0.4 ms of a
frame at full HD, or 4-5% of the frame. Zoomed out costs most because of the
box mean. At 400,000 walkers the ticks dominate: `sense` alone is 2.35 ms a
tick. The 8 ms bar is the GPU thread's, at 100,000 walkers on the 1024 x 640
dish. Dish at 10 s: tube 6589.7, loops 466, width 0.84, piles 7/7, strays
0.347.

## Open

- The owner's hands: pan, zoom, the minimap and the squad (Q, W) in the
  sandbox.
- The Larder world binary, which the Larder thread builds on these pieces.
  It will need a world wider than today's dish layout, not the same dish at
  a finer scale. `W` only scales today's layout up, which is enough for the
  camera.
