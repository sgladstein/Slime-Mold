# Planet review fixes: plants under the slime, fire as the brightest light (2026-11-15)

These are the art lead's drawing items from the three-reviewer planet review (https://claude.ai/artifact/Kcy1AoquWJjJ3EciyA3KxP) that belong to the plants thread:

- **#8:** clay cracks and plant crowns showed through the slime.
- **#11:** the fire was no brighter than the slime and close to its hue. Rule 2 puts light at the top of the value order.
- **#3:** only 4 of 8 biomes could be named at zoom 12. The plants' side of that is a plant form per biome.

Page: https://claude.ai/artifact/GfhdLAM5txddNwy9kT43e4 (`page/`, `raw/films.sh`, `raw/make_page.py`; the numbers come from `raw/measure.py`).

## What changed (src/world/view.wgsl)

- **Under the slime:** the material's `vis` is now `1 - smoothstep(0.004, 0.05, th)` on the colour's thickness, which counts walkers. It used to be `1 - 0.95 x opaque` (opaque only from th 0.05). The pixel also uses `under`, the same curve on the mould's own film height. Plants, char and soil detail (`soil_detail`'s `bare`) all take both, so the ground's own detail is gone by the thin film's middle and only the slime's texture reads where it lies.
  - Before this, a film of explorers left every plant crown whole under it: its height (`film_th`) is next to none, and its opacity starts only at th 0.05.
- **Fire core:** a freshly caught crown burns white-hot at its top. Its colour is mixed toward `FLAME_CORE` (255,242,205), luma about 240, by sqrt(freshness) x (0.5 + 0.5 dome), where freshness is the engine's `PB_FRESH`, 1 on catching and x 0.85 each fire step.
  - With freshness taken linearly, the front's luma p99 was 186.
  - The far view's smooth flame takes the same core.
  - The body keeps its red-orange ramp, and the embers stay dull red.
- **Level pins:** the item text's new `flora` line changes the levelgen fingerprints (`PINNED`, `PINNED_BIG`); the ground and plants are the same.
- **Camera:** `Camera::clamp` no longer panics when the view is the world's size to within rounding. The clamp's bounds crossed by an ulp on planet 5 seed 1 at 960x600 (min -29.661352 > max -29.661354), and the headless film crashed.

- **A flora per biome:** each map now has a `flora N` line, written by the level generator from its biome (`levelgen::Biome` in order: 0 meadow, 1 barrens, 2 fen, 3 valley, 4 jungle, 5 highlands, 6 caves, 7 core). It goes to `map::Items::flora`, then `BigDish::flora_of`, then `View::set_plants`, then the view's `P.flora`.
  - `flora_form(flora, kind)` picks each kind's form: grid spacing, radius, stretch, lobes, relief, colour, a shape (lobed dome, blade, thorn star, cap with a dark eye, or ring) and clumping into patches. Clumping is a slow noise at each crown's centre, and a denser stand fills more of the ground between.
  - Only the look changes. The plants' kinds and amounts are the simulation's.
  - A map without the line draws flora 0.
  - Each flora's forms:

    | Flora | Plants up close |
    |---|---|
    | Meadow | Fine tufts and round trees, as before |
    | Barrens | Thorn bushes in patches, dry grass blades, palm-like thorned crowns at oases |
    | Fen | Puffy moss cushions in patches, low swamp shrubs |
    | River valley | Long grass, blade by blade |
    | Jungle | A broadleaf canopy of big crowns (2.3 cells apart) over ferns |
    | Highlands | Lichen rosettes and cushion plants in patches, firs |
    | Caves | Fungi in a dark mauve-grey: caps with a dark eye, giant caps for the jungle kind, stalks for reeds |
    | Core | Alien growth in dark wine: coral rings and thorned crowns |

  - No flora uses cyan, which belongs to the crystals, or gold or red-orange, which belong to the slime and the fire.
  - Rejected for the fen: tight 0.35-cell cushions read as meadow. For the valley: thin blades (3 x 0.6) barely showed at zoom 12.

## Measured

Planet 1 seed 1, zoom 12, 960x600. The fire is a bomb into the meadow at 3 s, filmed at 20 s, and its pixels are those that changed from the no-bomb frame and are warm. The slime is planet 2's thick body at 24 s.

| | luma p10 / p50 / p90 / p99 | hue p10 / p50 / p90 |
|---|---|---|
| Fire before | 56 / 77 / 124 / 155 | 12 / 15 / 22 |
| Fire after | 57 / 89 / 153 / 214 | 12 / 16 / 24 |
| Thick slime | 92 / 131 / 197 / 240 | 34 / 35 / 43 |

The fresh front's white cores now pass luma 200 against the slime highlight's 240, and the body stays at hue 12-24, away from the slime's gold at 34-43.
