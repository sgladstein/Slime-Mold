# Planets 4 to 8: the slime's trail on each biome's ground (2026-11-24)

Look lane, Look C (art direction: "one visual trait a planet that only it
has"; planets 1 to 3 had theirs, 4 to 8 none). Page:
https://claude.ai/artifact/Tx18bLio6eXj9ntMgxJAic (source `index.html`
here, clips beside it).

**What.** In `shaded`, after the barrens' dust and the fen's slick, each
later biome shows the trail (`trail_lin`, from `DUST_TRAIL` 0.02 to 0.2)
on its bare ground, fading as the trail does:

- valley (planet 4): a scum of yellow flecks on the river (`SCUM_RGB`,
  `SCUM`), on water only;
- jungle (5): the plants it runs under wilt olive-yellow (`WILT_RGB`,
  `WILT`, on the plants' grey);
- highlands (6): its dried residue fills the gaps between the gravel's
  stones (`GRAVEL_FILL`, `GRAVEL_FILL_K`; `gravel()` now shared with
  `plain_look`);
- caves (7): pale-green phosphor pins (`CAVE_GLOW`);
- core (8): the purple flora glows along it (`CORE_FLORA_GLOW`).

Masks use `water_share` (bilinear), not a per-cell water test, which
stepped the scum at a deep pool's cells.

**Tried and dropped:** gravel polished (a lift of 0.12 and glints),
unseen at zoom 6; the caves' phosphor as a wash under the pins, a green
halo round the body; the wilt as the plants' colour scaled, which made dark
greens brighter green; the core glow at (60, 14, 72), a magenta halo.

**Cost:** on planets 4 to 8 only, on bare pixels where the trail is over
0.02: a `trail_lin` and a `water_share` (four reads each), a noise, and on
the highlands a 3 x 3 cell search. Timed on the 1070 by the 4K thread
before it queued (see the PR).

Filmed on llvmpipe, seed 1, idle hand, zoom 6, 20 frames from 1:50
(planet 4 from 0:55).
