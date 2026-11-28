# The slime's level colours: review and options (2026-11-28)

The Look lane's first review of Milestone 2 (Direction's job 1, 11-28: the
slime's colours checked against `wiki/looks/art-direction.md`). The levels
(#864, 11-26: every crystal a level 0 to 7, its slime coloured by it in
`view.wgsl`'s `level_rgb`, orange through red and crimson to violet and
indigo) fail rule 3 and rule 5 on later steps.

Filmed on main 9a5ec6c, run seed 1, idle hand, 40 s after landing, at
1536 x 864 (his 4K at 250%), lavapipe:

- Step 10 (the Grudge): the tough crystals' slime is flat violet, plum and
  crimson, a matte carpet with no gloss or raised edge beside glossy orange;
  violet is the hunter mini-boss's colour, crimson the bite flash, plum the
  core world's plants. Up close (zoom 12) the seam between two levels is a
  soft band, which is right.
- Step 4 (the Trunk): levels 0 to 3 keep the gloss; the red-orange is close
  to the flush of slime under fire.
- The grain inside a big packed body is its relief (a debug glow showed the
  shading and highlights are there), the same at every level; on dark
  albedo it reads as fabric. Veins and flow in the thick body stay Look B's.

Three options were rendered by the game with `LARDER_VIEW_WGSL` (diffs
against main here, `option-A.diff` to `option-C.diff`; first renders, not
tuned): A, a warm ramp (gold, amber, deep orange, rust, brown, near-black)
with a faint heat of its own from level 2 (`LEVEL_HEAT`); B, today's ramp
lit a little in its own colour; C, one gold darkening to a crust with a
faint gold light. A first cut of A and C with a stronger glow (0.55 and
0.42 of the hue) lifted every level back to orange, so the levels no longer
read apart; the page shows 0.2 and 0.16. A glow gated on thickness
(`th` over 0.55 or 0.75) showed nothing: a packed body's drawn thickness
sits under that.

The question went to the owner on a card (11-28, recommended A); the page
(`index.html`, https://claude.ai/artifact/LZUZjAnjn1PYFteTyRvVMp) holds the
frames.

**His answer (11-28 05:37Z):** "Todays colors are still best. I like the vibrancy. All your new suggestions are too monotone." He could not see B's difference from today's in the stills. Today's ramp stays as it is; none of the three options was built.
