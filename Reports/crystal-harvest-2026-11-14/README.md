# Crystals show their live charge while harvested (2026-11-14)

**Ask.** The owner's playtest (11-14 04:53Z: "harvesting crystals could look
way better"), passed on by the art director: a crystal should visibly lose
charge where it is taken, with a bright working spot where each taker
touches it, and a dry crystal should read as spent grey rock
([style guide](../../wiki/looks/art-direction.md): crystals are the only cyan,
charge visible as glow).

**Page:** https://claude.ai/artifact/WGmUSQDUZxYdfF1rtSco18 (copy: `index.html`
here, with its GIFs).

## What was built

On top of the slime thread's exact hexagons ([sharp slime](../sharp-slime-2026-11-14/README.md)):

- `View::set_charge` binds the engine's own per-cell `charge` buffer
  (binding 21) and each crystal cell's full charge (`gems[k].w`), so the
  view reads charge cell by cell every frame. Before, the view had one share
  per crystal, refreshed with the ground codes about once a game second.
- A new `drain` pass over the crystal cells only (`gem_list`, binding 24)
  keeps each cell's last charge and its heat (binding 22): the share of full
  charge lost per game second, held as `max(rate, heat * exp(-dt / 0.35 s))`.
  Game time comes in through `View::set_time`, so a paused game holds still.
  Whoever takes the charge, soldier or slime, makes the same heat.
- `gem_state` is a cubic B-spline of share and heat over the crystal's cells,
  so a crystal taken row by row sinks smoothly, not in cell steps.
- In `shaded`: the crystal stands `0.15 + 0.85 x share` tall, its colour goes
  from cyan to grey stone with its share, its depth glow scales with share,
  and heat adds a white-cyan glow on the cells being drained plus a short
  halo spilling just outside the edge. A dry crystal is a low grey stub.

## What it costs

Larder World bench, 8 game seconds, farm hand (a squad harvesting),
1280 x 800, llvmpipe: GPU tail p50 57.6 / 60.6 ms with the live charge,
58.9 / 60.9 ms with its reads switched off. No measurable cost; the drain
pass touches only crystal cells. 1070 timing waits until the PC is free.

## Checks

- `view::tests::a_crystal_shows_its_live_charge_and_where_it_is_taken`: on
  llvmpipe, a crystal cell whose charge drops glows brighter than its
  untouched neighbour, and a drained crystal renders greyer and lower than a
  full one.
- GIFs: zoom 12 before (main) and after, zoom 6 after; seed 1, farm hand,
  3 to 20 s every 0.5 s, `centre=280,119`.

## Not yet

The soldier's side of harvesting (pose, a carried glowing load) is the
units thread's; it will publish each take (`Game::last_takes`) so its
effects and this glow read the same numbers.
