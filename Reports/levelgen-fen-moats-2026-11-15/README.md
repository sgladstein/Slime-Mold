# Fen lakes and moats (2026-11-15)

The looks review ([report](../levelgen-review-2026-11-15/README.md), item 3)
found the biomes differ only in layout: with no nest or crystal in view, 4 of
8 could be named at zoom 12, and 6 of 8 fens had rolled only pools, which a
squad walks through, so they played like the barrens. After the front doors
([report](../levelgen-doors-2026-11-15/README.md)) every nest's rim was the
same grey rock on every biome too. This change is the map's half for the
fens; the ground's look is the drawing threads'.

**Outcome:** fens now roll lakes 3 to 1. The moat round a fen nest is built
and tested but off (`Recipe::moat`): in game the slime covers most of it by
30 s, and it then reads as little more than a thinner rim. Rock rim or moat
is the owner's pick, as a blind A/B on the page.

- Before: main with the front doors (PR #327). After: this change.
- Page: [Fen Lakes and Rims](https://claude.ai/artifact/R2ycNj8mmoUuffjfyCL4zo),
  copy in `page/`. The rim is a blind A/B there (rock rim or moat), as a
  visual choice, with paired films of seeds 4 and 7; the key is in
  [`raw/ab-key.txt`](raw/ab-key.txt).
- 20 fens (planet 3, seeds 1-20):
  `cargo run --release --example planets -- seeds=1-20 depths=3 out=DIR`
  ([main](../levelgen-doors-2026-11-15/raw/survey-after.txt),
  [lakes with rock rims](raw/survey-rock.txt),
  [lakes with moats](raw/survey-moat.txt)).
- In-game frames: `larder-world` on llvmpipe, `hand=idle`, seeds 1, 4, 7 and
  15 ([`raw/renders.sh`](raw/renders.sh), [`raw/renders-v2.sh`](raw/renders-v2.sh),
  [`raw/renders-rock.sh`](raw/renders-rock.sh),
  [`raw/renders-rock-films.sh`](raw/renders-rock-films.sh)); the page's pictures by
  [`raw/make_ab.py`](raw/make_ab.py), the moat's own before and after,
  first try and films by [`raw/make_pics.py`](raw/make_pics.py), layouts by
  [`raw/fc.py`](raw/fc.py). Render times in
  [`raw/render-timing.txt`](raw/render-timing.txt).

## What changed (`src/mould/levelgen.rs`)

- **Fens roll lakes 3 to 1** (they were 2 to 2): lakes on 8 of 20 fens
  before, 15 of 20 after.
- **A moat round each nest, off** (`Recipe::moat`, set on no biome). The
  bowl's rock rim becomes a thinner lip (0.3-1.3 of its thickness), with
  deep water outside it (0.6-1.6 of the thickness, wide where the lip is
  thin). The door crosses the moat on a bridge: a straight deck from 6 cells
  inside the water's nearest reach to 6 past its farthest, as the rivers'
  decks. The breach is a gap in both. Soldiers can't wade deep water; the
  slime crawls across it.

New test `moats_ring_the_nests_with_a_bridge` (fens built with the moat on).
The pin for fen seed 1 (pools before, lakes now) is re-pinned; fen seed 3
rolled lakes already and kept its pin. No other planet changed (the other
140 map pictures are byte-identical to main's).

## Measured (20 fens)

| | Main | This change (lakes, rock rims) | Lakes, moats on |
|---|---|---|---|
| Lakes | 8 | 15 | 15 |
| First-try passes | 18 | 18 | 19 |
| Median build | 96 ms | 102 ms | 104 ms |
| Median rock share | 5.5% | 5.5% | 4.5% |
| Median crystals | 13 | 13 | 13 |
| Door (`doors`) | 6-8 | 6-8 | 6-8 |
| Way round | 18-30, median 24 | 16-30, median 22 | 16-30, median 20 |

Build times are within the machine's noise (the meadow, unchanged, went
54 -> 62 ms in the same runs).

**In game, with the moat on** (the A/B's moat side), the moat shows as a
dark ring outside the lip where the slime hasn't reached it. The slime's start already spills past the bowl at 1 s
(through the door and breach), and by 30 s it has crossed most of the moat,
which then reads as a smooth, glassy channel of slime round the lip. The
bridge is under it.

## Tried first

**Deep water as the whole rim.** The slime's start covered the moat by 1 s,
so it never showed: the start fills every cell of its disc but rock. And
shots fly over deep water, so the moat had to reach past the gun's range of
the nest (146 map cells) to keep the door and breach the only ways in. The
`Reports/dead-ends.md` entry holds the condition.

## Not done here

- **The other biomes' rims.** A thorn hedge in the jungle would be soft rock,
  which the slime wears through in 20-30 s (`Reports/larder-world-roadmap-2026-11-11.md`),
  so the bowl would open on every side. A ring of boulders on the meadow
  would leave gaps a soldier walks through. Both need a rule of their own
  first.
- **The slime's start on water.** The start fills deep water inside its disc.
  That is the organic start's rule (`start_shape`), not the map's.
- Cave tunnels 5-15 wide, a landing crystal per biome, nests off river
  crossings.
