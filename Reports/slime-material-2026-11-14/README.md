# Slime material, bright lone walkers, a detailed middle (2026-11-14)

The graphics thread, after the owner's playtest of the sharp slime
(11-14 04:50-04:58Z, relayed by the art director): "The edges of a large
section of mold looks really good now. The middle still looks blurry."
"single individual walkers should be brighter". "They don't actually always
look dim. Not sure why they do sometimes". Also the art director's material
brief (it "still reads as sand against the olive ground": glossy raised
veins, a translucent thin film, saturated gold, flow you can see) and the
game designer's note on #212 (a cut piece snapped from gold to ash at once).

Page: https://claude.ai/artifact/JXq4Sz5Pg1jn5sHHzM9EMv (`page/`).

## What changed (`src/world/view.wgsl`)

- **Lone walkers.** On main a lone walker was a bump 3 cells tall and a
  cell wide (the sharp count makes one walker read nearly as a body),
  whose far side shaded to the floor, plus 0.048 of white on one pixel: it
  drew darker than bare ground (the "sometimes": cut-off walkers had large
  amber specks, calm ones did not). Now:
  - every speck grows with the zoom (zoom / 3 pixels, up to 4, corners off);
  - a calm walker's light is a share of the way to a pale gold,
    0.95 / (1 + (d / 0.35)^2) a walker, where d is the walkers a cell round
    it from the wide count (`prep`'s `wide`, now kept in the material's
    byte that held the undrawn wound, `Mat::body`): about 0.6 alone, under
    0.1 in the body, so the body keeps a calm moving grain;
  - the shading slope is capped at 0.7, so a bead's far side is brown,
    not black;
  - the colour's thickness counts walkers too (`max(th, 0.4 near)`): an
    explorer lays little trail and was a dark film bead.
- **The middle.** The dense body keeps 40% of the sharp count
  (`dense * 0.6`), so the walkers' clumps and channels show at the fine
  grid.
- **Material.** Hue pushed 45% further from grey (less as the piece goes
  stale); body brightness 0.95-1.25 by thickness; thin film is the soil
  tinted by the hue (gel), opacity smoothstep(0.08, 0.55, th); height
  5 (1 - e^(-2.6 th)) / (1 - e^(-2.6)) (was 4 th^0.65); highlight exponent
  16 + 64 gloss, strength gloss (0.5 + 0.9 gloss).
- **Ramps.** A walker's grey, amber and red are smooth ramps of its news
  age in fifteenths: grey from lag - 2/3 lean to lag + 4/3 lean (-1 s to
  +2 s at 5 s and 1.5 s), amber over a third of the lean each side of the
  lag, red over a third each side of the lean's end. The fine grid counts
  a walker 15.

## Measurements

- GPU test `a_lone_walker_is_bright_and_the_body_stays_calm` (zoom 6): a
  lone walker 170.8 luminance against bare ground 55.6; on main 42.9
  against 54.6 (fails). A cell of the body's pixels spreads less than 0.6
  of the lone walker's lift.
- Slime pixels at 2:31, zoom 6 (saturation above 70): median saturation 83
  main, 119 now; luminance p50 113 and 110, p90 149 and 175; pixels above
  230 luminance 1 and 1,716. Ground 55.
- Cut ring with four soldiers, 2:36-2:38 (179, 135, 52 walkers inside at
  2:36, 2:37, 2:38), mean saturation of the piece every 0.1 s: main 67 67
  67 66 66 66 66 64 63 56 52 48 46 42; now 86 82 79 78 76 75 74 68 65 57 52
  47 45 42. The fade starts a second earlier and has no step.
- Same simulation in both builds: the `t` lines and cut counts in `raw/`
  match.
- Frame cost, llvmpipe, `larder-world bench=20 hand=farm seed=1`, builds
  alternated: frame p50 40.5 and 41.6 ms on main, 42.5 and 42.9 ms now
  (+4%). The 1070 timing waits for the PC.

## Tried and changed

- Speck light falling linearly with the wide count (0.7 to 0.05 over wide
  0.5-0.9): the body's middle was a starfield; the wide count in a medium
  body is no higher than round a lone walker. Light by the square of the
  walkers a cell instead.
- Speck light dimmed by the mould's height: a lone walker's own bump
  dimmed it.
- Colour thickness `max(th, 0.6 near)`: every body fully opaque, the film
  gone; 0.4 kept it.
- Height 5 th^0.75: the dense body's trail swings showed as mottling.

## Next

The ground goes darker and cooler (crystals thread), which supplies the
rest of the grey contrast (rule 2). The 1070 timing when the PC is free.

Commands: `raw/fin.sh` (films, bench, GIFs via `raw/mkgif2.py`).
