# E1a: crust, ground that remembers the slime (2026-11-17)

The simulation-use plan's E1a ([plan.md](plan.md), draft 3): the crust field
and its look, with no effect on play yet. Page:
https://claude.ai/artifact/7S91NqGRDSopYhhje3giS6 (copy in `page-e1a/`, open
`index.html`), with the Grudge's planet 7 film from step 0.

## The rule

`BigParams::crust_rise`, `crust_keep`, `crust_trail` (both engines; GPU keys
of the same names, and `crust=1` for the window's numbers). Each fine step,
after the trail spreads (and its soil and crater): an agar cell with a walker
on it and trail of `crust_trail` or more gains `crust_rise` up to 1; every
other cell keeps `crust_keep`. A shot cuts the trail on its cells, which
pauses the rise there but does not wipe the crust (the reviews: rifle craters
must not wipe it). `crust=1`: rise 1/1800 and keep 0.5^(1/3600) a step, so a
cell fills in a minute of steady living and halves in two minutes with none
at the window's 30 fine steps a second; trail 1. Off by default (no rule reads
it yet, and its look has not been through the owner's blind quiz).

On the GPU the field is a block of `news` after the plants' (`Engine::crust_at`;
every binding a stage has is taken), the Params row `CRUST_AT` (1184; the
uniform 1184 -> 1200 bytes), stepped in the trail pass (`crust_step`). The
view reads the same block (`View::set_crust`, its uniform 288 -> 304 bytes).

## Offline look first

`larder-world crustlook=T` (look= only) works the same rule out on the CPU
from the look's read-back once a game second and writes a crust map beside
each picture. Planet 3, run seed 1, idle (`crust-look/counts.txt`, the pairs
in `crust-look/`):

| Time | Over 0.05 | Over 0.3 | Of those bare now | Over 0.95 | Walkers' cells |
|---|---|---|---|---|---|
| 1:00 | 52,178 | 45,532 | 7% | 31,845 | 44,083 |
| 2:00 | 62,701 | 47,074 | 48% | 25,571 | 26,900 |
| 3:00 | 66,082 | 51,670 | 45% | 25,735 | 29,809 |
| 5:00 | 72,398 | 51,213 | 26% | 35,312 | 39,376 |
| 8:00 | 79,712 | 57,108 | 11% | 48,520 | 51,705 |

As the artificial-life review found on the CPU race world, it is one plate
over the slime's whole range, not a road map; it shows after the slime pulls
back (from 44,100 walkers' cells at 1:00 to 26,900 at 2:00 here). The
engine's own crust at 1:00: 51,830 / 45,489 / 29,453 (over 0.05 / 0.3 /
0.95); at 3:00 65,721 / 51,723 / 25,874, within 1% of the offline copy.

## Twin (llvmpipe)

`the_crust_matches_the_cpu` (rise 0.02, keep 0.995 a step, a strike at tick
30): 94,236 cells over 0.05, crust L1 0, 0 of 25,000 walkers off after four
ticks. Plants caught: GPU keep 1% lower, L1 0.024; GPU trail bar out of
reach, L1 0.10. The life-cycle twin's new dish "both, crust, growth on
crystals" (births, deaths, a strike, a wall): crust L1 0, 0 walkers off.

## The look

Satin patches on bare ground (the readability review's pick: value and hue
are taken, gloss is free): a two-octave value noise about a cell across, a
pixel in a patch where the noise is under 0.3 + 0.5 x crust, each edge a
pixel wide at any zoom, the ground inside lifted 12% and cooled, the rim
toward the light lit and the far rim shaded; past half a patch a pixel it
fades to the mean. Under the slime it hides as the soil's detail does.

- Rejected: the first mapping, 0.15 + 0.75 x crust. The noise rarely falls
  under 0.25, so crust under about 0.2 drew nothing, and the first zoom 12
  frame showed no change at all (a pixel diff found only walker noise).
- Seen (page): crisp patches at zoom 24 and 12, a fine glaze at zoom 6. It
  lies over the shallows too, where the slime spread over water.
- To do before it is on by default: the blind "name the ground" quiz
  (crust, char, rubble, barren, damp bank).

## Cost

One more uniform read a cell a step with the crust off. On: an atomic load
and store a cell a step in the trail pass; in the view, four reads and up to
six noise samples a crusted bare-ground pixel. 1070 timing to book with E1b.
