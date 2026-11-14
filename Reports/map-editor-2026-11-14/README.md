# Larder World map editor (WM5, 2026-11-14)

Plan item C10 of [`cloud-plan-2026-11-14.md`](../cloud-plan-2026-11-14.md),
ROADMAP row WM5: paint ground kinds, place crystals and starts, save to the
WM1 map files, play. Built in the cloud while the owner could not playtest
and the 1070 was busy.

## What it is

`cargo run --release --bin larder-edit [-- map=fork name=my-fork]`

- A window of its own (`src/bin/larder-edit.rs`), not a mode of
  `larder-world`: a start menu is being built at the game's front door in
  parallel (plan C2), and `larder-world.rs` was frozen for the file split
  (C9) while this was built. The editor needs only the map files, so the
  two never share an entry point; the menu can start `larder-edit` for an
  "Edit maps" item.
- The editing is `slime_mould::mould::map_edit` (`Editor`, `Brush`, `Tool`,
  `Mirror`, `draw`), tested without a window (9 tests).
- **Ground**: 1 plain, 2 sand, 3 clay, 4 barren, 5 rock, 6 wet, as the G
  byte the loader reads. Soil brushes clear rock and keep wet; rock and wet
  keep the soil. `[` `]` size the brush (1-160 map cells).
- **Items**: C crystal (a hexagon, the engine's `in_pile` footprint; `[` `]`
  size it, 8-120 cells; charge 384 a cell), M mould start (several allowed;
  the loader shares the walkers), L the squad's landing (one), W wall (drag;
  8 wide, 400 health, `build_wall`'s width and `wall_hp`), E erase the
  nearest item within 24 cells.
- **Mirror** (X): off, left-right, or a half turn through the centre, for
  fair maps. The landing is never mirrored.
- Ctrl+Z / Ctrl+Y undo and redo (40 steps), Ctrl+S save, N a blank map (twice
  if unsaved), F fit, wheel zoom, right or middle drag pan.
- **Enter saves and plays**: it starts `larder-world map=DIR/NAME` beside the
  editor (the built binary next to it, or `cargo run` if that is not built),
  passing on any other `key=value` given to the editor (`seed=3`,
  `tier=sleeper`). The editor stays open. It refuses to play a map with no
  crystal, no mould start or no landing, or with a start or the landing on
  rock; the panel lists what is missing.
- Saves go to `maps/NAME` in the repo by default (`dir=` to change). The
  designed maps in `Reports/larder-maps-2026-11-11/maps/` are opened
  read-only unless `dir=` names that folder. Closing the window saves a
  changed map.
- `map=noise seed=N` opens the race world as a map to edit (`Map::of_dish`).

**The picture is the file.** Each pixel is the map cell under it in the
world view's own ground colours (`view.wgsl` on flat land), crystals as the
engine's hexagons, so a map looks as it will at the start of a round.

## Checks (cloud, llvmpipe Vulkan)

- `cargo test --lib map_edit`: 9 pass, among them an edited map through
  `Map::save` and `Map::load` comes back equal, and every panel string has a
  glyph in the crisp font.
- **The real window** under Xvfb, driven by xdotool
  ([`raw/window-run.sh`](raw/window-run.sh)): opened the Nest, painted a clay
  stroke and a rock stroke, placed a crystal, dragged a wall, erased the
  crystal, undid the erase, saved (files in
  [`raw/nest-edit.*`](raw/)). Enter then started `larder-world` on
  `nest-edit`: its window opened on the edited map with the new clay band and
  wall (`page/played.jpg`, 0:24 of the round, 4 soldiers, 25.2k mould).
- **Headless film** (`larder-edit demo=DIR`): a mirrored map drawn from blank
  in 13 steps (sand belt, clay pockets by the starts, rock ridges, wet
  hollows, barren, three crystal pairs, two starts, the landing, two walls),
  saved as [`raw/demo-map/demo.*`](raw/demo-map/); the panel said "ready to
  play". Played with `larder-world map=DIR/demo look=DIR hand=rush at=2,20,45`:
  25,000 walkers from two starts, each in its clay pocket; the rush squad of 4
  lost all 4 by 20 s, killing 1,158 walkers with 45 shots; crystals 99.4% at
  45 s.
- Frame cost: the editor redraws on input only, a full CPU repaint of the
  1280 x 800 frame (not timed; it does no simulation). Nothing runs every
  game step.

## Not done

- A start-menu entry (plan C2's thread can add it: run `larder-edit`).
- Mission files (`Reports/larder-campaign-2026-11-12/missions/`) are not
  edited here; a mission names its map, so a map made here can be named in one.
- Crystal charge is fixed at 384 a cell in the window (the `Editor` has
  `charge`; no key yet), and walls take the default width and health.
- Height (R byte) stays parked, as the loader ignores it.

## Page

https://claude.ai/artifact/NAJ89BJNhtkszVmFBWHutK, copy in [`page/`](page/index.html).
