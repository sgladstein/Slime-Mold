# Flung droplets in the slime's material (2026-11-18)

Look A3 on the ranked list (`wiki/looks/art-direction.md`), the Look and
interface lane. Page: `page/index.html`, published at
https://claude.ai/artifact/9vsJuSofq14P5jShjiWyqe (listed in
`Reports/pages.md`).

## What was wrong

Every walker a shot kills flies out of the hit as a droplet (the spray,
[gun look](../gun-look-2026-11-15/README.md)), drawn since the bold spray
([gun feel](../gun-feel-2026-11-15/README.md)) as a white-hot to orange
teardrop inside a dark rim a pixel wide. Filmed in a planet 1 raid and at
the Fork at zoom 6 and 12, the rim made them cartoon chips, pale seeds in
an ink outline, beside a slime drawn with light: the 11-17 review's
"outlined shapes in a different style from the slime they come from".

Seen on the same films: **shots already fly as short bright comets**
(the gun look's streaks, 11-15); the look list's "dotted today" was out of
date, and A2's shot half is closed.

## What changed

`view.wgsl` only (`draw_specks` and the composite in `base`):

- **A lit bead.** Each droplet pixel stores how squarely the bead faces the
  key light (four bits: a sphere's normal from the droplet's axis), and the
  composite shades it with the body's light (`LIGHT`) and the dense body's
  tight wet glint (exponent 14 + 30 x 0.95). A gentler slope than the
  body's (`DROP_SLOPE` 0.8 against 1.25, floor `DROP_FLOOR` 0.55 against
  0.3): at the body's own curve a bead a few pixels across was mostly its
  dark limb, a brown pebble at zoom 6.
- **The slime's gold.** A shot's spray is the slime's gold (`DROP_GOLD`)
  while fast, glowing `DROP_GLOW` 0.4 of the way to a pale gold
  (`DROP_HEAT`), cooling to the deep orange (`DROP_COOL`) as it lands; a
  blast's throw stays gold. Heat now takes four bits (0 a throw, 1-15 a
  spray's heat).
- **A shadow, not a rim.** Where the rim was, the droplet's own shape falls
  `DROP_SHADOW` 0.9 head radii away from the light and takes `DROP_SHADE`
  0.5 of the light under it. With the shaded limb it keeps a droplet apart
  from the bright body, which is what the rim was for.

## Measured

One game drawn two ways (`LARDER_VIEW_WGSL`, the old shader; the game
lines are equal), 30 frames a game second, the middle 800 x 500 of the
picture. Changed is the share of it where any channel differs by more than
12 of 255.

| Scene | Shots, kills | Changed, mean | Changed, most | Frames that differ |
|---|---|---|---|---|
| Gun range (`range=7 hand=stand`), zoom 12, 2.5-4.5 s | 20, 167 by 5 s | 0.17% | 0.64% | 26 of 61 |
| The Fork held (`hand=hold@150,265 bank=8000 rounds=plain`), zoom 12, 30-32 s | 206, 1,267 by 32 s | 0.50% | 1.24% | 61 of 61 |
| The same, zoom 6 | as above | 0.16% | 0.38% | 61 of 61 |

### Tried and not kept

Stills of the Fork at 31 s, zoom 6 and 12, the shader swapped:

- **The body's own shade curve** (1.25, floor 0.3) with the 11-15
  white-hot a third of the way in and a glow of 0.6: pale cream seeds.
- **The same, warmer** (a fifth of the way, glow 0.3): grey-brown pebbles,
  darker than the chips over the bright body.
- **The gentler curve with the white-hot base and glow 0.45 or 0.6**:
  brighter, still cream rather than the slime.

## Tests

- `a_flung_droplet_is_a_lit_bead_with_a_shadow_and_no_rim`
  (`src/world/view.rs`): a fast spray droplet over a body at zoom 12. Its
  side toward the light outshines its far side by 15 in luma, its middle
  is gold (blue under 0.6 of red), the pixel just past its head on the lit
  side is at least 0.75 of the body (no rim), and the body beside its far
  side is under 0.8 (its shadow). Against the 11-15 shader it fails on the
  first: 241 against 243.

## Frame cost

Per droplet pixel, a normalize and a dot in `draw_specks`, and a second
distance test for the shadow over a box one shadow offset bigger; in the
composite, a `pow` a droplet pixel. Only pixels under a flying droplet pay.
Not timed.

## Not done here

- **Zoom 6 is quieter.** A droplet there is some four pixels; the
  outlined chips were brighter. If hits read too quiet in play,
  `DROP_GLOW` is the number to raise.
- **The hit's other half** (A2): the hole the shot leaves and how fast it
  closes.

## Files

- `page/`: the page, three clip pairs (H.264, 30 fps) with posters, and the
  close pair at twice the pixels.
- `raw/film.sh OUT OLD_WGSL` films the six clips; `raw/make_clips.py OUT
  PAGE` makes the clips, the counts and the close pair.
