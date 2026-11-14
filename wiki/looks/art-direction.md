---
title: Art direction for Larder World (the style guide)
kind: look
status: open
updated: 2026-11-14
sources:
  - Reports/art-review-2026-11-14/README.md
  - Reports/crisp-terrain-2026-11-14/README.md
  - Reports/crystal-harvest-2026-11-14/README.md
  - Reports/dark-ground-2026-11-14/README.md
code:
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
   (`theme::BAD`) for warnings only. A new colour needs a role nobody has.
   The ground follows this since 11-14 (slate, flat luma 38; [dark ground](../../Reports/dark-ground-2026-11-14/README.md)).
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
   moment it opens (it refills in about 2 s). Check: a three-frame strip
   (fed, cut at +30 s, leaning) turned grey still shows the difference
   (game designer review, 11-14). The slime sharpness thread owns these.
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

- Look in motion: GIFs or a playable build at zoom 1, 6, 12 and 24, never
  stills alone (owner 11-12 14:28Z). A visual choice goes to the owner as a
  blind A/B with the key off the page.
- Quote the frame cost on llvmpipe as before/after; the 1070 timing waits
  for the owner's PC.
- A look lands when it works and can be seen (testing rule, 11-12 17:26Z):
  if a better look changes balance, retune the numbers, don't keep the
  uglier look.
- One owner per file region: the art director's thread agrees who edits
  `view.wgsl`, `marks.rs` and `draw.rs` when, so threads don't collide.

## Owner's word

- 2026-11-14 01:49Z: "Graphics need a big improvement. The crystals are pixelated. The slime looks very blurry/out of focus. The flashes of chaos bullets are really bad."
- 2026-11-14 01:50Z: "The bullets shouldn't just be flashy effects. Research how noita does their wand animations."
- 2026-11-14 02:24Z: "I want another thread to review all graphics in general and mange the current graphics work and start new work where it sees issues. We need to polish this game up"

## Open

- The review's ten issues and who has each: [review](../../Reports/art-review-2026-11-14/README.md), *Who takes what*.
- The 1070 cost of the new looks (waits for the PC).

## Sources

- [Art review 2026-11-14](../../Reports/art-review-2026-11-14/README.md).
