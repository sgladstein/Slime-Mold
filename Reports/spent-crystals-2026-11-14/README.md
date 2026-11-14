# Spent crystals read as worn stone (2026-11-14)

**Ask.** The art director, 11-14 08:15Z, after #228: at 94% taken (farm
hand, 1:00, zoom 6) "a dry crystal is a large flat grey hexagon that reads
like a UI block. It should read as worn, cracked, dull rock, lower and
darker than a live one and with some texture from its depth field, so it
doesn't compete with live cyan."

**Page:** https://claude.ai/artifact/4oi2DgiPNcX7rzRQHa6aoo (copy: `index.html` here).

## What changed (`view.wgsl`, the crystal block of `shaded`)

- Spent stone is darker and matte. The albedo is (66,66,64) x
  (0.6 + 0.3 depth), against (84,88,92) flat before. The highlight falls to
  8% with the charge gone (it was 25%).
- It is lower: `GEM_STUB` goes from 0.15 to 0.10 of a full crystal's
  height.
- It steps in three hexagonal terraces from the crystal's depth field. Each
  step's riser, two pixels wide, tilts outward, so the light catches one
  side and shades the other, like a worn step pyramid. The steps come in as
  the charge goes, so a live crystal stays clean glass.
- Rejected: four dark seams on a flat face. They read as a target, not as
  stone.

## Pictures

Fork map, farm hand, the crystals by the larder from 30 to 65 s as the squad
harvests them dry: before (main) and after at zoom 6, and after at zoom 12.

## Cost

A few operations on crystal pixels only; not measurable on llvmpipe.
`view::tests` pass (a drained crystal still draws greyer and lower).
