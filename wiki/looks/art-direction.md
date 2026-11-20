---
title: Art direction for Larder World (the style guide)
kind: look
status: open
updated: 2026-11-19
sources:
  - Reports/art-review-2026-11-14/README.md
  - Reports/biome-ground-2026-11-15/README.md
  - Reports/bug-check-2026-11-15/README.md
  - Reports/crisp-terrain-2026-11-14/README.md
  - Reports/crystal-harvest-2026-11-14/README.md
  - Reports/dark-ground-2026-11-14/README.md
  - Reports/droplets-2026-11-18/README.md
  - Reports/fallen-soldier-2026-11-18/README.md
  - Reports/flinch-look-2026-11-17/README.md
  - Reports/ground-edges-2026-11-14/README.md
  - Reports/levelgen-review-2026-11-14/README.md
  - Reports/lure-rings-2026-11-18/README.md
  - Reports/mini-boss-look-2026-11-16/README.md
  - Reports/raised-walls-2026-11-18/README.md
  - Reports/river-look-2026-11-14/README.md
  - Reports/sim-use-2026-11-17/e2b-look.md
  - Reports/soil-materials-2026-11-14/README.md
  - Reports/soldier-skins-2026-11-19/README.md
  - Reports/squad-heads-2026-11-18/README.md
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
did. The first review (11-14) found the slime out of focus, cell stairs on hard shapes, an uncountable squad and a HUD over the fight; by 68870d0 those are fixed and the ground dark (#247); open: the tint discs, a pebbly frontier, flat grey spent crystals and the map editor ([review](../../Reports/art-review-2026-11-14/README.md), *After*).
Since 2026-11-17 the Look and interface lane holds this guide and its
ranked look jobs, in the owner's order.

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
   pump, the sapper ember crimson at your
   walls, the mortar (11-20, planet 4 on) moss green lit by an acid core,
   saturated where the plants are muted; the pinwheel (11-20, planet 3 on)
   deep cobalt with a sky-blue core, bluer and darker than the hunter's
   violet, its three arms the members' own; dark, mid and pale, parting by value as well as hue; not
   bronze (owner 11-16); its colour, dome and core light all from where its
   members are (once out on its errand, wherever they stand, however thick
   the slime round them), and a broken or spent one's look drains into the slime
   over its 1.5 s fade ([mini-boss look](../../Reports/mini-boss-look-2026-11-16/README.md),
   round 2; the hunter kept violet 11-19). A new colour needs a role nobody has.
   The ground follows this since 11-14 (slate, flat luma 38; [dark ground](../../Reports/dark-ground-2026-11-14/README.md)). Each soil has its own material from the soil field since 11-14 (sand ripples, clay cracks, ash speckle, dark water with a smooth edge and shore, raised rock with a lit rim), so planets read apart in grey; the ripples bend and fade out below zoom 3, and barren ash sits a clear step under rock ([soil materials](../../Reports/soil-materials-2026-11-14/README.md)). Water is a clear step darker than any ground, deep water darker again toward its middle, with a shallow margin, waterline and damp bank; bridges are plank decks; soft rock at full health is a mid-grey under thin slime ([river look](../../Reports/river-look-2026-11-14/README.md)). Play is assumed zoomed in (owner, 11-14 20:21Z): every boundary between grounds, soils, water and plant patches and kinds, is drawn crisp at the pixel up to 24 px a cell, and no texture shows the cell grid ([ground edges](../../Reports/ground-edges-2026-11-14/README.md)); a shortcut past the soil spline must cover all 4 x 4 cells it reads, or the shore and bridge shading steps at cell middles; the full test costs about 0.2 ms of GPU a frame on the 1070 ([bug check](../../Reports/bug-check-2026-11-15/README.md)). Value order since 11-15, on every planet: ground 41-51, rock 50-60 (hedges and rubble too), soft rock 60-72 darkening with damage, thin slime above rock with a shade floor on its far flank, crystal faces 110 and up, thick slime 140 and up; shallow water about 20 above deep; off the map is darker than cave rock; hedges are lumpy and grained, not tubes; the rock bevel is at most 5 px; below zoom 1 and on the minimap each cell is drawn from the shaded materials ([value order](../../Reports/value-order-2026-11-15/README.md)). Each planet's plain soil has its own hue at every zoom and its own close look from its biome (the map's `flora`): brown meadow loam, tan barrens hardpan, olive fen mud, warm valley silt, red-brown jungle humus, neutral highland gravel, blue-grey cave dust, the texture in from zoom 3 to 8, all kept in the ground's value band; thin film has an absolute floor of 95 before shading over any ground ([planet ground](../../Reports/biome-ground-2026-11-15/README.md)).
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
8. **Shapes the owner fixed**: crystals are hexagons; shots stop at the
   first slime (owner 11-11 15:50-16:48Z); soldiers wear their Armour:
   olive, slate, dark camo (11-19, [skins](../../Reports/soldier-skins-2026-11-19/README.md)).

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
- One owner: the Look and interface lane reviews every lane's visible
  change against this guide.

## Owner's word

- 2026-11-14 01:49Z: "Graphics need a big improvement. The crystals are pixelated. The slime looks very blurry/out of focus. The flashes of chaos bullets are really bad."
- 2026-11-14 01:50Z: "The bullets shouldn't just be flashy effects. Research how noita does their wand animations."
- 2026-11-14 02:24Z: "I want another thread to review all graphics ... We need to polish this game up"
- 2026-11-17 17:39Z: "we should explore all your suggestions, priority in the order that you listed A->B->C->D"
- 2026-11-19 00:08Z: "multiple looks could eventually apply to different slimes"

## Look jobs, ranked

In the owner's order; A is Milestone 1's look test (`NOW.md`, exit test 2).

**A. The fight round the squad**

1. Done (11-17, 18): the flinch on the slime alone ([flinch look](../../Reports/flinch-look-2026-11-17/README.md)); the Lure's rings ([lure rings](../../Reports/lure-rings-2026-11-18/README.md)); flushed and set slime apart ([settled](../../Reports/sim-use-2026-11-17/e2b-look.md)); droplets of slime ([droplets](../../Reports/droplets-2026-11-18/README.md)); the fallen in blood ([fallen](../../Reports/fallen-soldier-2026-11-18/README.md)).
2. A hit is the slime's own reaction: the hole and how it closes (shots fly as comets).
3. Done (11-18): a squad countable by its heads ([heads](../../Reports/squad-heads-2026-11-18/README.md)).
4. Done (11-18): no light past its damage; biting slime crimson ([bite](../../Reports/biting-slime-2026-11-18/README.md)).
5. Done (11-18): status slime glows rose ([mind](../../Reports/mind-status-2026-11-18/README.md)).
6. Checked (11-18): fire at zoom 12 has no cell steps.

**B. The slime itself**, a look a slime (owner, 11-19): Sprawler dashes (done, `SlimeLook`); veins and flow inside the thick body; set slime as a hard material (raised 11-18, [raised walls](../../Reports/raised-walls-2026-11-18/README.md)); loose specks that read as one body, not pebbles; the origin's look once Enemy builds it.

**C. Planets and terrain**: stronger grounding (faint shadows at play zoom; 11-20: each crystal casts its hexagon swept away from the light, `GEM_CAST` 0.55 of its radius times its height share, at most 8 cells, `GEM_SHADOW` 0.6 dark at its foot, on the slime as on the ground; rock's cast `TER_CAST` 1.6 cells, `TER_SHADE` 0.55); done 11-20: rock as stone (#534), planet 1's meadow in patches by the water (#535), a spent crystal a cracked husk of frosted glass (its six faces kept, rays and chords from a blow at its middle, coming in as the charge goes; `GEM_HUSK`, `GEM_SHARD`, `GEM_RAYS`, `GEM_SPENT` in `view.wgsl`; value about 70, between rock and a live face); one visual trait a planet that only it has (planet 1 done; 11-20, planets 2 and 3: the slime's trail on bare land, as pale dust tracks on the barrens, `DUST*`, and a wet slick with a cool sheen and glints in the fen, `SLICK_*`, both in `shaded` from `trail_lin`).

**D. Menus, shop and screens**: win and lose screens with the planet's numbers (done 11-18, [ends](../../Reports/end-screens-2026-11-18/README.md)); a landing, a win and a loss that happen in the picture (the landing done 11-20: the soldiers drop in one by one over their shadows, `draw::landing`; a dust ring at their feet tried and dropped as a drawn pulse; the end done 11-20: the world plays on for 2.4 s with the clock held and the camera drawing back before the card, `ending.rs`); then the shop and the menus. Shop, 11-20: the counter's offers as shelf rows with their one-line effect (`shelf_row` in `start/shop.rs`; bends shop B's half-size tiles while every offer fits on one page).

Also open: the 1070 cost of the new looks (one batch); the first review's open issues ([review](../../Reports/art-review-2026-11-14/README.md), *Who takes what*).

## Sources

- [Art review 2026-11-14](../../Reports/art-review-2026-11-14/README.md).
- [Planet art review 2026-11-14](../../Reports/levelgen-review-2026-11-14/README.md).
