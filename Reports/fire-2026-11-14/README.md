# Fire, round two (2026-11-14)

The owner, 19:18Z: "Fire should spread 5x slower. It shouldn't just be a wavefront. It should stay burning in some areas, There should be more heterogeneity. It should be a little more challangeing to spread... Sometimes it should burn out in areas. You could maybe add a hetergenous moisture field that affects how fast it spreads, how long it burns, and the chance of it going out. It should hurt the player too". 19:26Z: "The fire should also look better... Moisture field should match where their are rivers/lakes, and change by biome. Discuss with the level generator thread."

## Built

- **Moisture** (`plants::moisture`): the map's `moisture BASE SPREAD` line, agreed with levelgen, which writes it per biome (Meadow 0.25 0.3, Barrens 0.0 0.2, Fen 0.5 0.3, River valley 0.3 0.3, Jungle 0.4 0.3, Highlands 0.1 0.2, Cave 0.35 0.2, Core 0.2 0.35). Maps without the line use 0.15 0.3. On top of the base: `spread` x three value noises (40, 12 and 4 cells), +0.4 falling to 0 over 8 cells from wet ground or deep water, jungle +0.15, scrub -0.15. Reeds and water are 1.
- **Fire** (`plants::fire_step`, engine.wgsl `fire`):
  - A cell catches by chance: 0.3 x the neighbours' heat x plants x burn x dryness.
  - It goes out with chance 0.05 x moisture² a step, leaving its plants.
  - It eats x (1 - 0.75 moisture) and its heat tops out at 1 - 0.4 moisture.
  - Its heat flickers x (1 - 0.35 x chance), and embers fade x 0.9 a step.
  - Eat went from 0.05 to 0.012.
  - The rolls are an integer hash of the cell and the fire step, so the GPU matches exactly.
- **Soldiers:** `UnitOut::heat` is the heat on the unit's cell. The GPU's unit out grew from 4 to 8 words. `Rules::fire_hurt` is 25 hp a second at full heat.
- **Look:** at first, the flame colour was the cell's own heat over the 3x3 halo, so the flicker and embers showed. After the graphics lead's review (20:10Z) and the owner's note (20:21Z, "the fire is blurry. The assumption should be that I am playing zoomed in"):
  - The flames are drawn per pixel in `shaded`, from the cubic B-spline of the heat.
  - They come in three crisp layers: red-orange from heat 0.25, orange from 0.5 and yellow over 0.8, each edge 0.03 of heat wide.
  - The 3x3 halo stays as a dim light spill.
  - Nearest-cell heat showed squares. Per-fine-cell heat in `prep` stair-stepped at zoom 24. A smoothstep from 0.2 to 0.3 still read soft.
- **Heat follows fuel:** a cell tops out at plants / 0.8, so the band dims behind the front. Embers fade x 0.95 a step.
- **Lighting by shots:** a blast lights 12% of its cells out to a cell past its crater, and a scorch round 40% of its disc. Before, every blast lit a full ring two cells past its crater. Since #288 that kept a squad's front on fire (gun thread and owner, 20:09-20:14Z).

## Measured

- **Meadow strip, front in cells at 10 s and 20 s** (`moisture_slows_fire_and_puts_it_out`):

  | Moisture | 10 s | 20 s |
  |---|---|---|
  | 0 | 11 | 23 |
  | 0.2 | 11 | 21 |
  | 0.4 | 6 | 12 |
  | 0.6 | 4 | 7 |
  | 0.8 | 0 | 0 |

  The first cut reached 50 and 100: a ribbon 9 cells deep moving 5 cells a second.
- **Twin test:** exact for plants, fire, char, kills (113 = 113 at the burn) and the heat under a soldier.
- **Film** (planet 2 seed 1, idle, three bombs): slime 25,000 → 31,095 by 60 s, and all 4 soldiers alive. In an earlier film a bomb landed beside the idle squad and the fire killed all 4 by 60 s.
- **Frame cost:** llvmpipe planet 2 frame p50 58.8 ms after the per-pixel flames. The flames' 16 heat reads run only where the halo glows.

## Rejected

- The first spread of 0.6 with dryness² was 2 cells a second dry and dead at moisture 0.6. It became 0.3 with linear dryness.
- Regrowth used to clear char from the embers' leftover 0.04 of plants, which wiped a burn in about 15 s. Now only plants above `fuel` clear it.

[Page](page/index.html); films from `raw/films.sh`.
