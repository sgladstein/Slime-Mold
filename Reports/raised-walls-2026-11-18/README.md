# Raised walls and the Lure's rim: two look fixes (2026-11-18)

The Look and interface lane, after filming two other lanes' new features
in the game. Page: `page/index.html`, published at
https://claude.ai/artifact/EBAKa6mpRV9M31H5CoDiF1 (listed in `Reports/pages.md`).

## Set slime stands above the sheet

The Enemy lane's Trunk fort (#473; [trunk fort](../trunk-fort-2026-11-18/README.md))
made set slime a standing feature of planet 3: a ring with four gates
round every crystal the slime holds. Drawn as before, colour, gloss and
glow only, with the sheet's own height, the ring read as a rust stain or
a drawn target on the slime, not as a wall a soldier cannot cross. It now
stands `SET_RISE` (4) above the sheet (`view.wgsl`, `set_rise` in `prep`;
the dense body is at most 5), so the light gives it a lit flank and a
shaded one. Opacity, colour and the share's edge are as before.

Planet 3 of run seed 1, idle, the home ring setting from 12 to 18 s, the
Enemy lane's two cameras, one game drawn two ways (the game lines are
equal). "Changed" is the share of the picture (HUD strips cropped) where
any channel differs by more than 12 of 255, every tenth frame:

| Camera | Changed, mean | Changed, max |
|---|---|---|
| Zoom 6, the home crystal (`centre=60,71,6`) | 1.65% | 1.77% |
| Zoom 12, a gate (`centre=76,80,12`) | 4.86% | 5.22% |

### Tried and not kept

From stills of the same frame at 28 s, zoom 6 and 12, the shader swapped
with `LARDER_VIEW_WGSL`:

- **Rises 2 and 6.** 2 left the edge faint at zoom 6; 6 differed little
  from 4 and stands more than the body's own height.
- **A flat top** (the wall's height set to the land plus the body's 5 plus
  the rise, so its top has no bumps): looked the same as the added rise.
  The mottling on the wall's top is not its height; it is left (below).

## No ring on the Lure's rim

Filmed whole on planets 2 and 3, the Lure's rings ended in a dotted ring:
the game's reach (60 dish cells, 120 cells) is a multiple of the rings'
spacing (20), so the last ring fell on the field's edge, where the field
stops on its 2 x 2 blocks' steps. No ring is now drawn within half a ring
of the reach (`View::set_steer_reach`, the window passing the reach as
`time_lure_paths` builds it). The same frame at 40 s with the Lure from
30 s, before and after: 0.31%, 0.25% and 0.21% of the whole picture on
planets 1, 2 and 3; 0.24%, 0.32% and 0.11% at zoom 6 on the Lure.

## Tests

- `no_ring_is_drawn_on_the_lures_rim` (`src/world/view.rs`): a field
  reaching 40 cells; with no reach given the ring at 40 is drawn, with the
  reach it is not, and the ring at 20 is drawn either way. Planted with
  the old rule it fails ("a ring on the rim").
- The view's other 17 tests pass with the walls raised. None of them
  draws set slime: the walls are judged from the films.

## Frame cost

The walls: one multiply-add a fine cell in `prep`, which already read the
set share. The rim: one compare a ring pixel. Not timed.

## Not done here

- **The Lure's rings are octagons on open ground**, the field's
  8-neighbour walking distance; the Enemy lane will make the field round
  (11-18), and the Lure's clips in play wait for it.
- **The wall's top still reads soft**: pale flecks on it at zoom 12 that
  are not its height. List B in the art direction.
- **Cell steps on the walls' arcs at zoom 12**: the ring is laid in cells,
  and the share smooths over the 4 x 4 round.
