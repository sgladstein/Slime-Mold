---
title: Art direction for Larder World (the style guide)
kind: look
status: open
updated: 2026-11-18
sources:
  - Reports/art-review-2026-11-14/README.md
  - Reports/biome-ground-2026-11-15/README.md
  - Reports/bug-check-2026-11-15/README.md
  - Reports/crisp-terrain-2026-11-14/README.md
  - Reports/crystal-harvest-2026-11-14/README.md
  - Reports/dark-ground-2026-11-14/README.md
  - Reports/flinch-look-2026-11-17/README.md
  - Reports/ground-edges-2026-11-14/README.md
  - Reports/levelgen-review-2026-11-14/README.md
  - Reports/lure-rings-2026-11-18/README.md
  - Reports/mini-boss-look-2026-11-16/README.md
  - Reports/river-look-2026-11-14/README.md
  - Reports/soil-materials-2026-11-14/README.md
  - Reports/value-order-2026-11-15/README.md
code:
  - LumpLook
  - theme
---

# Art direction for Larder World (the style guide)

## Now

One look for the whole game, which every graphics thread follows: **living
light on dark ground**. The slime is the brightest, warmest, most detailed
thing on screen; the ground is dark and quiet; hard shapes (rock, walls,
crystals) are crisp; your units are high-contrast and countable; light
(muzzle, impact, crystal charge) only ever shows something the simulation
did. The first review (11-14) found the slime out of focus, cell staircases on every hard shape, a squad you can't count and a HUD that hides the fight; by 68870d0 those are fixed, and the ground is dark and cool since #247; what is open is the tint discs, a frontier that reads as pebbles, spent crystals as flat grey blocks and the map editor ([review](../../Reports/art-review-2026-11-14/README.md), *After*).
Since 2026-11-17 the Look and interface lane holds this guide and the list
under *Look jobs, ranked*, in the owner's order: the fight round the squad,
then the slime, planets and terrain, menus and screens.

## The rules

1. **The picture is the data.** Every pixel comes from a field or a game
   state; no painted pulse, no stamps, no outline or threshold on a graded
   quantity (owner 11-09/11-10, [decisions](../decisions.md)). An effect is
   light on a real change, lasting as long as the change: Noita looks the
   way it does because the world changes, the glow only shows it.
2. **Value order, darkest to brightest**: ground; rock and walls; thin slime
   film; crystals; thick slime veins; your units; light (muzzle, impact,
   burning). A thing that matters more is never darker than one that
   matters less. Check by turning a frame grey: the order must still read.
3. **One hue a role.** Ground cool and low in saturation (slate, dark loam);
   rock neutral grey; walls warm grey whose brightness is their health; the
   slime warm gold to amber, grey as it starves (#212's ramp), each enemy
   species its own hue; crystals the only cyan (`theme::ENERGY` family);
   your team cream and olive with one accent colour; danger red
   (`theme::BAD`) for warnings only; plants muted green (the fen's moss
   olive-brown, the caves' fungi mauve-grey), the core planet's flora
   purple-magenta (hue 290-320), the only purple on the map
   ([review fixes](../../Reports/review-fixes-2026-11-15/README.md)); a
   mini-boss its members' own skin, by kind (`LumpLook`): the hunter
   violet (skin hue about 265, core 280), the wanderer pale bone, the
   leech near-black ink whose core turns the charge's cyan as it drinks a
   pump (the cyan means charge there too), the sapper ember crimson at your
   walls; dark, mid and pale, so they part by value as well as hue; not
   bronze (owner 11-16); its colour, dome and core light all from where its
   members are (once out on its errand, wherever they stand, however thick
   the slime round them), and a broken or spent one's look drains into the slime
   over its 1.5 s fade ([mini-boss look](../../Reports/mini-boss-look-2026-11-16/README.md),
   round 2; the owner's A/B pick for the hunter open). A new colour needs a role nobody has.
   The ground follows this since 11-14 (slate, flat luma 38; [dark ground](../../Reports/dark-ground-2026-11-14/README.md)). Each soil has its own material from the soil field since 11-14 (sand ripples, clay cracks, ash speckle, dark water with a smooth edge and shore, raised rock with a lit rim), so planets read apart in grey; the ripples bend and fade out below zoom 3, and barren ash sits a clear step under rock ([soil materials](../../Reports/soil-materials-2026-11-14/README.md)). Water is a clear step darker than any ground, deep water darker again toward its middle, with a shallow margin, waterline and damp bank; bridges are plank decks; soft rock at full health is a mid-grey under thin slime ([river look](../../Reports/river-look-2026-11-14/README.md)). Play is assumed zoomed in (owner, 11-14 20:21Z): every boundary between grounds, soils, water and plant patches and kinds, is drawn crisp at the pixel up to 24 px a cell, and no texture shows the cell grid ([ground edges](../../Reports/ground-edges-2026-11-14/README.md)); a shortcut past the soil spline must cover all 4 x 4 cells it reads, or the shore and bridge shading steps at cell middles; the full test costs about 0.2 ms of GPU a frame on the 1070 ([bug check](../../Reports/bug-check-2026-11-15/README.md)). Shots scatter a little round the aim since 11-15 (the rifle 0.5 cells, the long gun 0.3), so a wide or tight bullet card shows on every gun (same report). Value order since 11-15, on every planet: ground 41-51, rock 50-60 (hedges and rubble too), soft rock 60-72 darkening with damage, thin slime above rock with a shade floor on its far flank, crystal faces 110 and up, thick slime 140 and up; shallow water about 20 above deep; off the map is darker than cave rock; hedges are lumpy and grained, not tubes; the rock bevel is at most 5 px; below zoom 1 and on the minimap each cell is drawn from the shaded materials ([value order](../../Reports/value-order-2026-11-15/README.md)). Each planet's plain soil has its own hue at every zoom and its own close look from its biome (the map's `flora`): brown meadow loam, tan barrens hardpan, olive fen mud, warm valley silt, red-brown jungle humus, neutral highland gravel, blue-grey cave dust, the texture in from zoom 3 to 8, all kept in the ground's value band; thin film has an absolute floor of 95 before shading over any ground ([planet ground](../../Reports/biome-ground-2026-11-15/README.md)).
4. **Sharp at every zoom, 1 to 24.** Hard shapes have exact edges worked
   out at the screen pixel and antialiased over one pixel; no cell stair is
   visible at any zoom. The slime's edge is where its walker density falls
   off, soft over at most two pixels at zoom 6 and closer; no blur wider
   than that anywhere in the world view. Rock, walls and rubble meet it
   ([crisp terrain](../../Reports/crisp-terrain-2026-11-14/README.md); crystals show live charge and glow where taken, [crystal harvest](../../Reports/crystal-harvest-2026-11-14/README.md)).
5. **The slime looks alive.** Thick parts raised and glossy (one key light,
   top left, a specular highlight), thin film translucent over the ground,
   grain only where walkers really are sparse (the frontier), and motion
   you can see in a two-second clip.
   The states play hangs on read at zoom 6 within a second, all from the
   data: a cut-off piece withering loses light and cools as its energy
   drops while the fed body stays brightest; a panic lean shows as a
   brighter, denser front facing its target; a shot's hole is clear the
   moment it opens (it refills in about 2 s); slime under the alarm (backing
   off from fire, or called by a siren) flushes warm orange, deepest where
   shots land now, and bare ground shows no alarm (`flush_at`;
   [flinch look](../../Reports/flinch-look-2026-11-17/README.md)). Check: a three-frame strip
   (fed, cut at +30 s, leaning) turned grey still shows the difference
   (game designer review, 11-14).
6. **Units read at zoom 6.** Every soldier countable (figures never draw
   exactly on top of one another), its facing visible, selection a thin
   ring, health shown when hurt; the dead look dead; buildings show their
   state (rising, hurt, working) in their sprite (soldier sprite A's style,
   owner 11-11 16:48Z).
7. **The HUD serves the fight.** In play the HUD covers at most about 15% of
   the screen, collapses when nothing is selected, and never sits over what
   the camera opens on. Its look stays the `theme` set in `ui.rs`.
8. **Shapes the owner fixed**: crystals are hexagons; soldiers are sprite A;
   shots stop at the first slime (owner 11-11 15:50-16:48Z).

## How a look is judged and shipped

- Look in motion, at zoom 6 and 12, never stills alone (owner 11-12
  14:28Z): a before and an after clip of 5 to 10 s on an Artifact page, and
  one line for the game's "what is new" list. Small choices are the lane's
  and the owner vetoes in play; a blind A/B with the key off the page is
  kept for a choice that sets the whole art direction (11-17).
- Quote the frame cost on llvmpipe, before and after.
- A look lands when it works and can be seen (testing rule, 11-12 17:26Z):
  if a better look changes balance, retune the numbers, don't keep the
  uglier look.
- One owner: the Look and interface lane (`CLAUDE.md`, *Lanes*) reviews
  every lane's visible change against this guide.

## Owner's word

- 2026-11-14 01:49Z: "Graphics need a big improvement. The crystals are pixelated. The slime looks very blurry/out of focus. The flashes of chaos bullets are really bad."
- 2026-11-14 01:50Z: "The bullets shouldn't just be flashy effects. Research how noita does their wand animations."
- 2026-11-14 02:24Z: "I want another thread to review all graphics in general and mange the current graphics work and start new work where it sees issues. We need to polish this game up"
- 2026-11-17 17:39Z: "we should explore all your suggestions, priority in the order that you listed A->B->C->D"

## Look jobs, ranked

In the owner's order. A is Milestone 1's look test (`NOW.md`, exit test 2);
B starts once A passes his eye.

**A. The fight round the squad**

1. Done: the flinch on the slime alone, no lime wash (11-17, [flinch look](../../Reports/flinch-look-2026-11-17/README.md)); the Lure's pull as cream rings (11-18, [lure rings](../../Reports/lure-rings-2026-11-18/README.md)).
2. A shot is a short bright streak the eye can follow (thin dotted tracers today), and a hit is the slime's own reaction: the hole and the walkers it flings.
3. Flung droplets in the slime's material (outlined shapes in another style today).
4. A squad of 40 you can count (42 are one pile today); the spacing with Squad and arsenal.
5. No effect bigger than its damage: check the muzzle light, hit light and crater ember.
6. A Mind status (blind, sour, spin, provoke, still) reads on the slime it holds, not only as the hit's rose flash.
7. Fire on slime and on set slime with no cell-sized squares at zoom 12.
8. Flushed slime beside amber set slime (planet 3): film both, part them by hue if they blur.

**B. The slime itself**: veins and flow inside the thick body; loose specks that read as parts of one body, not glowing pebbles; a look for the origin when Enemy builds it.

**C. Planets and terrain**: stronger grounding (shadows are faint at play zoom); materials for rock and spent crystals, flat grey today; one visual trait a planet that only it has.

**D. Menus, shop and screens**: win and lose screens with the planet's numbers first; a landing, a win and a loss that happen in the picture; then the shop and the menus.

Also open: the 1070 cost of the new looks (one batch before the milestone check); the first review's remaining issues ([review](../../Reports/art-review-2026-11-14/README.md), *Who takes what*).

## Sources

- [Art review 2026-11-14](../../Reports/art-review-2026-11-14/README.md).
- [Planet art review 2026-11-14](../../Reports/levelgen-review-2026-11-14/README.md).
- [Flinch look 2026-11-17](../../Reports/flinch-look-2026-11-17/README.md).
