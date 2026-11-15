# Value order on the planets (2026-11-15)

The art lead's planet review (11-15 01:19Z, https://claude.ai/artifact/Kcy1AoquWJjJ3EciyA3KxP) asked for a grey value order on every planet. These are the targets in grey (luma):

- ground 20-45;
- rock 50-60;
- soft rock 60-72 at full health, darkening with damage;
- thin slime film at least 80;
- crystal faces 110 and up;
- thick slime 140 and up.

The review also asked for the items below, and this change does each of them in `src/world/view.wgsl`:

- **#1:** shade at every zoom (the far view and the minimap);
- **#7:** an off-map backdrop darker than cave rock;
- **#9:** shallow water at least 20 above deep;
- **#10:** hedges with a lumpy edge and grain, not tubes;
- **#11:** a crisp rock bevel at zoom 12-24.

Item **#3**, a ground micro-look per biome, is the next change, because it needs the biome passed to the view.

## Measure

The measure is the reviewer's own, `measure.py` on branch `claude/project-thread-0tcg97` at 1b37130b, in `Reports/levelgen-review-2026-11-15/raw/aesthetics/`. It uses the same views: planets 1-7, seeds 1-2, whole-map and identity views at 1 s and 60 s, a 1280x800 screen, the idle hand, on llvmpipe. Each cell below reads main 66dd92fe / this change.

| Planet, seed | film p10 | slime p50 | thick p90 | rock | soft rock | crystal | brightest ground p90 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 meadow, 1 | 71 / 70 | 97 / 93 | 149 / 149 | 98 / 56 | 123 / 66 | 109 / 131 | 81 / 74 |
| 1 meadow, 2 | 68 / 71 | 97 / 104 | 134 / 142 | - / - | 123 / 39 | 126 / 101 | 51 / 56 |
| 2 barrens, 1 | 86 / 93 | 127 / 141 | 182 / 194 | 98 / 56 | 123 / 59 | 121 / 114 | 59 / 60 |
| 2 barrens, 2 | 85 / 85 | 127 / 126 | 181 / 179 | 98 / 56 | - / - | 113 / 114 | 60 / 60 |
| 3 fen, 1 | 66 / 72 | 114 / 116 | 160 / 163 | 98 / 51 | 122 / 58 | 123 / 120 | 51 / 56 |
| 3 fen, 2 | 66 / 83 | 108 / 116 | 172 / 172 | 98 / 56 | 123 / 67 | 111 / 119 | 59 / 56 |
| 4 valley, 1 | 65 / 71 | 107 / 109 | 180 / 183 | 98 / 56 | 123 / 65 | 122 / 113 | 57 / 99 |
| 4 valley, 2 | 74 / 78 | 151 / 153 | 190 / 193 | 98 / 56 | - / - | 102 / 128 | 57 / 59 |
| 5 jungle, 1 | 49 / 56 | 114 / 113 | 190 / 191 | 98 / 41 | 123 / 51 | 116 / 112 | 87 / 64 |
| 5 jungle, 2 | 51 / 55 | 114 / 113 | 190 / 191 | 98 / 49 | 123 / 44 | 106 / 116 | 123 / 68 |
| 6 highlands, 1 | 63 / 67 | 111 / 114 | 193 / 192 | 98 / 56 | 107 / 59 | 106 / 120 | 51 / 51 |
| 6 highlands, 2 | 64 / 68 | 112 / 114 | 194 / 193 | 98 / 56 | 123 / 66 | 96 / 118 | 49 / 49 |
| 7 caves, 1 | 56 / 68 | 120 / 118 | 196 / 194 | 98 / 56 | 98 / 56 | 93 / 109 | 61 / 56 |
| 7 caves, 2 | 51 / 64 | 100 / 104 | 193 / 191 | 98 / 56 | 98 / 56 | 94 / 105 | 56 / 56 |

**How it moved:**

- **Rock** went from 98 to 41-56.
- **Soft rock** went from 98-123 to 39-67. The 39-51 values are damaged soft rock, which darkens as it should.
- **Crystals** are at 110 or above on 11 of 14 planet-seeds, up from 6 of 14. They are still under on meadow 2 (101), caves 1 (109) and caves 2 (105).
- **The film p10** now beats rock and soft rock on every planet-seed. Highlands 2 is narrow, 68 against 66.

**Why the film p10 is low.** The review's film p10 counts the veins' cast shadow on the ground beside them. Over pixels that are actually slime-coloured (R-B > 45 at 60 s), the film p10/p25 went as below; 10 of 14 are now at 80 or above:

| Planet, seed | Slime-coloured film p10/p25, before -> after |
| --- | --- |
| 1-1 | 82/92 -> 83/94 |
| 1-2 | 82/91 -> 81/95 |
| 2-1 | 97/115 -> 100/120 |
| 2-2 | 90/107 -> 95/110 |
| 3-1 | 79/101 -> 87/105 |
| 3-2 | 83/100 -> 87/104 |
| 4-1 | 76/95 -> 80/93 |
| 4-2 | 108/134 -> 107/134 |
| 5-1 | 71/100 -> 78/100 |
| 5-2 | 68/91 -> 72/94 |
| 6-1 | 70/86 -> 74/93 |
| 6-2 | 76/107 -> 81/108 |
| 7-1 | 74/102 -> 84/103 |
| 7-2 | 68/88 -> 77/92 |

**Water and ground** (cell-class medians, after): shallow water is 38 and deep water 15-16, a 22 gap; shallow was 22 before. Bare ground is 41-51, and plant ground about 50. Valley 1's "brightest ground p90" of 99 is its small clay class (2.8k px, p50 49), whose top tenth picks up bright pixels at its edges; every other ground class there stays at 56 or below.

## What changed

- **Rock, hedge (wall) and rubble albedo** are lowered to the rock band.
- **Soft rock** keeps its health darkening.
- **Crystal faces:** the charged colour is brighter, `(38,142,165)`, and the face light is compressed while charged, so shaded faces stay above 110.
- **Thin film:**
  - The film reaches full cover faster (`exp(-40 th)`, was `-14`).
  - The gel is lifted to about 100 luma.
  - The film gets a shade floor. The low end was the far flank of the frontier veins, shaded down to about 0.4; lifting the gel alone did not move p10.
- **Shallow water** is `WET_RGB` `(14,33,41)`, was `(5,14,22)`; deep water is unchanged.
- **#1, far view.** Below zoom 1, the boxed view and the minimap now draw from a per-cell `far_cell`, which uses the shaded view's materials: soil, water depth, plants, hard terrain and crystals. They used the old flat palette before. See `far-view.png` (planet 8 at zoom 0.86).
- **#7, backdrop.** Off the map is `OFF_MAP_RGB` `(12,13,16)`, darker than cave rock (about 55), with a 1.5-cell fade.
- **#10, hedges:**
  - Hedges get their own albedo share with a crisp joint to rock.
  - The edge is lumpy, from value noise on turned grids. Grids on the cell axes showed squares at zoom 24.
  - Grain is laid over them.
  - The bevel tilt, specular and lit rim are mostly removed on hedges, so they no longer read as tubes.
- **#11, bevel.** The rock bevel is at most 5 px wide at any zoom, and the cast shadow edge is crisp at the pixel.

## Pictures

- `grey-near.png`: identity views at zoom 12, 60 s, in grey, before and after for all 14.
- `grey-all.png`: the whole-map views, the same way.
- `hedge-z12-*.gif`: meadow 1 at cells 162,120, zoom 12, 2-8 s.
- `water-z12-*.gif`: valley 1 at 335,62.
- `caves-z12-*.gif`: caves 1 at 456,124.
- `index.html` is the page (https://claude.ai/artifact/GgsaFe2YaUfMfm6AFnjNMb).

## Cost

Every change is a constant or a few ALU operations in passes that already run. The far view samples one cell per pixel, where the boxed view sampled up to nine before. No new texture reads were added at zoom 1 and above.
