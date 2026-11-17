---
title: Plants
kind: mechanism
status: open
updated: 2026-11-17
sources:
  - Reports/bug-check-leftovers-2026-11-15/README.md
  - Reports/fire-2026-11-14/README.md
  - Reports/gun-element-2026-11-17/README.md
  - Reports/gun-tinder-2026-11-17/README.md
  - Reports/plants-2026-11-14/README.md
  - Reports/plants-look-2026-11-14/README.md
  - Reports/review-fixes-2026-11-15/README.md
  - Reports/sim-use-2026-11-17/c1-dry-burns.md
  - Reports/sim-use-2026-11-17/c4-fire-alarm.md
  - Reports/sim-use-2026-11-17/e1b-c2-crust-effects.md
code:
  - BigDish::ignite
  - BigDish::lay_fuel
  - BigParams::crust_fuel
  - BigParams::fire_alarm
  - BigParams::fire_damp
  - BigParams::fire_dry
  - BigParams::fire_fed
  - BigParams::fire_spread
  - BigParams::fire_weight
  - BigParams::plant_grow
  - BigParams::plant_rot
  - plants::moisture
---

# Plants

## Now

A per-cell plant field in four kinds (meadow, jungle, reeds, scrub;
`mould::plants`), separate from the old grazing grass. It grows toward a
capacity set by soil, kind and wet and spreads from its edges, 15 plant
steps a game second, on the CPU world and the GPU engine (bit-exact twin
on llvmpipe). Fire (`BigDish::ignite`, lit by scorch rounds, blasts and
the Torch, a gun card that lights every plant within 6 cells) spreads
through plants, eats them and leaves char; wet ground, reeds and
slime with trail >= `fire_damp` 3 don't burn, so fire takes the slime's
edge, not its body (probe: 501 killed in 80 s while it grew 5,405 to
7,669). Char comes from the plants that burnt (`char_gain` 1), so a
jungle leaves darker ground than a meadow. How it is drawn: see *Look*.
Maps carry plants in the PNG's B channel (kind << 6 | density 1-63).
With the crust on (`crust=1`, 2026-11-17), crust is fuel too, worth 3
plants a unit (`crust_fuel`), and char of 0.3 or more is barren
(`char_barren`: no births, x3 upkeep): a 4-cell light on ground the slime
left ran across the whole clearing in a minute (4,382 cells burning) and
stopped at the living body, where at fuel 0 it went out in 20 s
([report](../../Reports/sim-use-2026-11-17/e1b-c2-crust-effects.md)).
**Starving slime burns** (C1, 2026-11-17, the owner's 01:13Z ruling;
`BigParams::fire_dry`, on): the trail no longer wets slime whose food news
is older than the wither lag, nor set slime, on both engines. A cut part
that went stale burnt whole in 20 s on full meadow (6 of 6 seeds; 0-6%
before), and a fed disc burnt exactly as before. But fed slime was never
wholly fireproof: fire through its thin trail took 24-65% of a fed disc in
20 s; `BigParams::fire_fed` (off, the owner's card) makes fed slime wet
whatever its trail, and took that to 0
([report](../../Reports/sim-use-2026-11-17/c1-dry-burns.md)).
**Fire lays alarm** (C4, 2026-11-17; `BigParams::fire_alarm`, off): each
fire step a burning agar cell adds `fire_alarm` x its fire to the alarm
field the flinch reads, on both engines. At 5 nothing changed (8 seeds); at
50 and 200 the slime drew back from the flames, the fire, undoused, ate
holes in the body ringed by slime, and killed 13% and 26% more. The same
behaviour as `fire_weight`, so off with it until the designer's crossing
test ([report](../../Reports/sim-use-2026-11-17/c4-fire-alarm.md)). The two routes are one decision (technical lead, 11-17 12:05Z): one blind
A/B film of each alone, the owner keeps one and the other goes to
`Reports/dead-ends.md`; if they look alike the alarm route wins, as it adds
no sensor read ([ROADMAP](../../ROADMAP.md), switches that are off).

Fire round two (owner 19:18Z, 19:26Z): fire catches by chance, about a
cell a second on dry ground (was 5), goes out and smoulders where damp,
flickers and leaves embers; moisture (`plants::moisture`) comes from the
map's `moisture BASE SPREAD` line (levelgen, by biome), water within 8
cells and noise. Fire hurts soldiers (`Rules::fire_hurt` 25 hp/s at full
heat) ([page](https://claude.ai/artifact/7VV82Fp79TSyGViYef4SaV)).

**Laid fuel** (2026-11-17, the Tinder gun card): `BigDish::lay_fuel`
(GPU: the IGNITE pass in lay mode) plants open, dry ground up to a level,
even where nothing grows; bare ground turns scrub. Plants over their
ground's capacity wither back to it by `plant_rot` (0.0002 a tick, about
40 s from full on barren ground), so a laid line is gone in under a
minute; with nothing laid the rule is the old one. A fire runs along the
line on ground that would not carry one: on the barrens a torch and tinder
gun killed 159 in its windows against the torch's 123
([tinder](../../Reports/gun-tinder-2026-11-17/README.md)).

Walkers do not sense fire. A flag puts heat in their sensors
(`fire_weight`, under 0 they turn from it; 0 off, both engines): in its one
A/B film the slime ringed a fire instead of walking into it, and more of
it burned, since its damp trail no longer put the fire out
([element](../../Reports/gun-element-2026-11-17/README.md)). The game
designer's crossing test (a lit tinder line across a stream of walkers
going to food, planet 1:3, one seed, 30 s) kept it off: with no line 214
walkers crossed, with the line 10 at weight 0 and 57 at -5. The weight
held the stream at the fire's edge (4 crossed while it burned, against
10) and fewer burned (44 killed against 119), but the stream crossed the
char as soon as the fire died, and it did not flow round the line; the
simulation-use thread's `fire_alarm` at 50 did the same (31 crossed, none
while it burned) ([tinder](../../Reports/gun-tinder-2026-11-17/README.md)).

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
- [tinder](../../Reports/gun-tinder-2026-11-17/README.md), *laid fuel, the crossing test*.
- [rivers under reeds](../../Reports/river-reeds-2026-11-15/README.md), *plants thinner over water, none over deep water*.
- [review fixes](../../Reports/review-fixes-2026-11-15/README.md), *plants under the slime, white-hot fire front*.
