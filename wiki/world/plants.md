---
title: Plants
kind: mechanism
status: open
updated: 2026-11-15
sources:
  - Reports/bug-check-leftovers-2026-11-15/README.md
  - Reports/fire-2026-11-14/README.md
  - Reports/plants-2026-11-14/README.md
  - Reports/plants-look-2026-11-14/README.md
  - Reports/review-fixes-2026-11-15/README.md
code:
  - BigDish::ignite
  - BigParams::fire_damp
  - BigParams::fire_spread
  - BigParams::plant_grow
  - plants::moisture
---

# Plants

## Now

A per-cell plant field in four kinds (meadow, jungle, reeds, scrub;
`mould::plants`), separate from the old grazing grass. It grows toward a
capacity set by soil, kind and wet and spreads from its edges, 15 plant
steps a game second, on the CPU world and the GPU engine (bit-exact twin
on llvmpipe). Fire (`BigDish::ignite`, lit by scorch rounds and blasts)
spreads through plants, eats them and leaves char; wet ground, reeds and
slime with trail >= `fire_damp` 3 don't burn, so fire takes the slime's
edge, not its body (probe: 501 killed in 80 s while it grew 5,405 to
7,669). Char comes from the plants that burnt (`char_gain` 1), so a
jungle leaves darker ground than a meadow. How it is drawn: see *Look*.
Maps carry plants in the PNG's B channel (kind << 6 | density 1-63).

Fire round two (owner 19:18Z, 19:26Z): fire catches by chance, about a
cell a second on dry ground (was 5), goes out and smoulders where damp,
flickers and leaves embers; moisture (`plants::moisture`) comes from the
map's `moisture BASE SPREAD` line (levelgen, by biome), water within 8
cells and noise. Fire hurts soldiers (`Rules::fire_hurt` 25 hp/s at full
heat) ([page](https://claude.ai/artifact/7VV82Fp79TSyGViYef4SaV)).

## Look

Drawn per pixel in view.wgsl from one cubic B-spline of the plant
fields (`plant_px`), crisp at zoom 24 (11-14 ~22:00Z): each kind as
crowns on a jittered grid whose radius follows its density (meadow
tufts 0.45 cells apart, scrub 1.1, reed stems 0.4, jungle crowns 1.7),
muted green (hue 130-150; teal took the crystals' cyan), under the slime by the material's `vis`. Fire is light
on darkened ground through one continuous ember-red-orange-yellow ramp;
zoomed in, each burning crown takes the heat at its centre and burns
hotter while fresh (engine block 8 `PB_FRESH`, view only), ember grains
show the heat left on burnt ground, and char is near-black grey; a fresh
crown's top burns white-hot (luma ~240, over the slime's highlight; art
lead 11-15). Over water plants are drawn thinner and over deep water not
at all, so rivers stay blue as reeds spread over the shallows (look only;
[rivers under reeds](../../Reports/river-reeds-2026-11-15/README.md)).
Plants, char and soil detail are gone under any slime by
mid-film (`vis`, [report](../../Reports/review-fixes-2026-11-15/README.md)).
Each planet draws its own plant forms (`flora_form`, keyed by the map's
`flora` line, the biome): barrens dry thorn clumps, fen dark olive-brown moss hummocks with
yellow-green tips on the wettest ground (block 7, moisture),
valley long blades, jungle star-lobed canopy, highlands lichen rosettes,
caves pale fungus caps, core purple-magenta thorns and rings (hue 290-320, the only purple); a map with no
`flora` line keeps the meadow forms
([page](https://claude.ai/artifact/GfhdLAM5txddNwy9kT43e4))
([page](https://claude.ai/artifact/7ZFRbDuC4ahCxKVV6wDui5),
[report](../../Reports/plants-look-2026-11-14/README.md)). Before: a 3 x 3
Gaussian per cell, soft blobs ([page](https://claude.ai/artifact/5pChXx8qdHxa3qwwP27HdY)).

## Owner's word

- 2026-11-14 17:02Z: "No. This should be fully new from the old grass." (levelgen thread, asked whether plants reuse the engine grass); biomes roll their own patterns, some more likely than others; any map beatable by any strategy.
- 2026-11-14 19:18Z: "Fire should spread 5x slower. It shouldn't just be a wavefront... Sometimes it should burn out in areas... a hetergenous moisture field... It should hurt the player too." 19:26Z: "The fire should also look better... Moisture field should match where their are rivers/lakes, and change by biome." Then: plants "are mostly just green blobs right now. They need to look better."
- 2026-11-14 17:29Z, on the brainstorm: "Slime doesn't eat plants at all." Cover: "slime senses by smells so that is a little convoluted. Lets think about it." Slow ground: "We can test this, but I am guessing it might be annoying." Fire: "I like this but need to think hard about how hard fire hurts slime. You don't want to make it too easy to just wipe out the whole slime with fire." Blocks shots, thickets, slime-eating plants, fertile battlefields: "we can try it. Need to playtest to see if it is fun." Harvest and sowing: "No." 

## Open

- Plants are on by default (menu Plants, `plants=0` off); maps without
  their own get `plants::stand_in` until levelgen writes them, in the
  window and (since 2026-11-15, `plants::planted`) the campaign,
  larderworld and kitplans census tools. Fire kills count in
  `Game::killed` every frame on both worlds (`Game::fire_killed`;
  [bug-check leftovers](../../Reports/bug-check-leftovers-2026-11-15/README.md)).
- Cover (plants muffling a soldier's scent) awaits the owner.
- Plants' look (owner 19:26Z: "mostly just green blobs"): per-pixel crowns built, awaiting the graphics lead and the owner; llvmpipe frame +9.5 ms at zoom 12, 1070 not timed.
- Next: shots stopped by tall plants, thickets, slime-eating plants, fertile battlefields and slow ground (off), each a switch for playtest; the 1070's frame cost.

## Sources

- [plants report](../../Reports/plants-2026-11-14/README.md), *brainstorm*.
- [plants look](../../Reports/plants-look-2026-11-14/README.md), *crowns sized by the field, fire per crown*.
- [rivers under reeds](../../Reports/river-reeds-2026-11-15/README.md), *plants thinner over water, none over deep water*.
- [review fixes](../../Reports/review-fixes-2026-11-15/README.md), *plants under the slime, white-hot fire front*.
