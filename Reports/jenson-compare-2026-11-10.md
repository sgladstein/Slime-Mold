# Why Jenson's mould looks different from ours (2026-11-10)

Research with small tests, no game change. The owner asked, 22:48Z: "can we
dig more into the research? How other people have implemented these
simulations? ... Can you identify those differences from the gifs that I
sent you earlier and what may be different between how sage implements the
simulation and how we're simulating". Page:
https://claude.ai/artifact/HL7jZ4qGhr6f1HpUDmNEj2 (source and pictures in
[`jenson-compare-2026-11-10/`](jenson-compare-2026-11-10/)). It follows
[`jenson-physarum-2026-11-10.md`](jenson-physarum-2026-11-10.md), which read
Jenson's post and GIFs, and compares them with the phase 2 big dish
(`mould::big`, [`gpu-plan-2026-11-10.md`](gpu-plan-2026-11-10.md)).

## Summary

Jenson runs the same step we do. The look comes from how it is run and
drawn. Tested on our 1024 x 640 big dish (no food, spread start):

- **Dropping the one-walker-a-cell limit alone breaks our mesh.** At 100,000
  walkers and our numbers, loops at 10 s fall from 151.5 (median, 144-160)
  to 0 (0-2): fewer on 8 of 8 seeds, sign test p 0.008, survives Bonferroni
  over 4. The walkers fold into a few thick ropes. Jenson's rule needs other
  settings: of 12 sensor-angle, turn and reach arms with no limit, only
  sensor 45, turn 22.5, reach 18 kept a mesh (47 loops at 10 s, straight-
  sided polygons; the others 0-15).
- **Half the look is drawing.** The game's curve (1 - e^(-trail/6)) is nearly
  full at a trail of 15; no-limit tubes peak near 100-200. The same dish drawn
  with a soft curve (trail over its 99.9th percentile, to the power 0.6)
  shows veins 0.4-0.6% of the dish wide; drawn by walkers a cell, 0.1%. The
  game's curve shows 1.4-1.6%. Jenson's network_2 last frame: 0.3-0.4%.
- **Respawn kept a fine, living mesh.** Moving 1% of walkers a step to a
  random spot (Jenson's later work respawns every particle about every 1,000
  steps, per Bleuje's port) held 196 loops at 10 s against 43.5 (more on 8 of 8 seeds, p 0.008, survives Bonferroni over 4), and gave a haze over 44%
  of the dish (Jenson's: 41%; ours without: 15%).
- **More walkers mostly smooth the grain.** 4 million walkers (6 a cell)
  still organised everywhere within 1 s (171 loops) and coarsened (60 at
  5 s). The turn rule only compares sensors, so density matters little.
- **Not reproduced:** the feathery fronts that grow into the haze, lasting
  trunks inside a fine mesh, and contagion. By the post's page order,
  "the gif below" after the contagion paragraph is ef_9 (the web), so its
  fibrous circle inside a straight lattice is probably the infected region.

## What the clips show that ours does not

Measured on the pictures the same way for Jenson's frames and our grey
stills: brightness over 110 or 150 of 255 is tube, width is 2 x area / edge
over the dish's equivalent diameter, haze is the share of dish pixels between
20 and 80 ([`picmeasure.py`](jenson-compare-2026-11-10/picmeasure.py); soft-curve and walker-count stills drawn from `style=raw` dumps by [`tone.py`](jenson-compare-2026-11-10/tone.py)).

| Difference | Jenson | Ours (the 4x Mould page) | Likely cause | Tested |
| --- | --- | --- | --- | --- |
| Thin veins vs wide tubes | network_2 last frame: 1.5-1.9 px, 0.3-0.4% of the disc, bright cores with a glow | 4.8 dish cells at 10 s (100k, seed 1), about 1.9% of the dish, flat and grainy | One walker a cell makes a tube as wide as its crowd; the game's curve saturates the flanks | yes |
| Haze | 41% of the disc dim grey | 1-2% strays, dark ground | A free share of millions is a fog; respawn | yes |
| Finer over time | 3-5 loops to 180-340 in 9 s | 164 at 10 s to 92 at 30 s | Every run here coarsens; respawn held the mesh; in Jenson's later performance a stylus spawns particles and changes settings live ([2020 paper](../research/sources/jenson-kuksenok-2020.md)) | yes |
| Polygons vs bubbles | Straight-sided cells, three-way joints | Round holes | Settings: no limit, turn 22.5, reach 18 gave polygons | yes |
| Feathery fronts | Branching tips grow into the haze, then close | A bag with tendrils (blob start), or all at once (spread start) | A guess: a network capturing a haze too thin to organise itself; the 2020 paper names a "critical particulate density" with a tipping point | no |
| Trunks and capillaries | The first ring stays thick inside the fine mesh | All tubes alike | The limit caps how bright a busy tube gets; with none, busy tubes do show brighter here | partly |
| Events | explosion: a bright burst over a faint net; ef_9: fibres inside a lattice | Nothing travels | Contagion; the lattice looks grid-locked | no |

## The tests

`examples/bigdish.rs` with `preset=dense blob=0 piles=1 food_deposit=0`
(no food, spread start) plus each change; deposit scaled by 100,000 /
walkers so every arm lays the same trail over the dish (with no food, the
turn rule compares sensors only, so the scale of the trail changes the
picture, not the walk). Stills are seed 1. New in the example: `style=`
(`amber`, `grey`, `count`, `raw`) and `knee=`; new in `BigParams`:
`respawn` (default 0, today's rule).

| Arm | Seeds | Loops at 10 s | Other | Cost |
| --- | --- | --- | --- | --- |
| 100k, our numbers, limit on | 8 | 151.5 (144-160) | tube area 12,620 dish cells | 20.3 ms a tick (seed 1, four threads for the fields) |
| 100k, our numbers, limit off | 8 | 0 (0-2); fewer on 8 of 8, p 0.008 | tube area 3,510 | 14.1 ms |
| 100k, limit off, 12 angle / turn / reach arms | 1 | 47 at sensor 45 / turn 22.5 / reach 18; 0-15 for the rest | | 16-20 ms (one thread, four at once) |
| 1M, limit off, 45 / 22.5 / 18 | 8 | 43.5 (38-52) (seed 1: 70 at 5 s, 42 at 10 s) | soft-curve width 0.4-0.6%, haze 15% | 159.6 ms (one thread, four runs at once) |
| same, respawn 0.01 a step | 8 | 196 (187-208); more on 8 of 8, p 0.008 (seed 1: 184 at 5 s, 200 at 10 s) | width 0.5-0.7%, haze 44% | 165.6 ms (+6, 4%) |
| 4M (6 a cell), limit off, 45 / 22.5 / 18 | 1 | 171 at 1 s, 118 at 2 s, 60 at 5 s | width 0.4-0.7% | 547 ms (two threads) |
| 4M, our numbers, limit off | 1 | 46 at 2 s, 8 at 5 s | | 515 ms |

Drawing the 1M dish at 10 s three ways, same 42 loops: game curve width
1.4-1.6% and haze 8%; soft curve 0.4-0.6% and 15%; walkers a cell 0.1% and 7%.

Paired counts ([`tables.txt`](jenson-compare-2026-11-10/tables.txt)),
loops and width at 10 s as the deciding measures, named before the runs,
Bonferroni over 4 (p < 0.0125). The limit pair: loops fewer with no limit on
8 of 8 (p 0.0078), width (engine's trail cut) wider on 8 of 8, 5.74 against
5.20 dish cells (p 0.0078); both survive. The respawn pair: loops more on 8
of 8 (p 0.0078) and width wider on 8 of 8, 7.88 against 5.83 (p 0.0078);
both survive. The wider respawn width is the haze lifting the trail around
the tubes over the cut; on the picture the tubes are 0.5-0.7% against
0.4-0.6% of the dish. Six respawn seeds were run first; six of six gives at
best p 0.031, which cannot pass the correction, so seeds 7 and 8 were added
(both went the same way).

## How other people build the step

Read from each project's code on 2026-11-10 (cloned into a scratch folder,
not kept here). Jenson's own code is not published.

- **Jenson, physarum (2019 post):** 5-10 million particles on a GTX 1070,
  one-a-cell "usually ignored"; sensor distance, size and angle, turn, step,
  deposit size, diffuse size and decay all configurable, numbers not given;
  3x3 mean and multiplicative decay; contagion. Grey on black.
- **Jenson, 36 Points, read through Bleuje/interactive-physarum (CC BY-NC-SA,
  ideas only):** 1280 x 736 with 5.8 million particles (6 a pixel), no limit,
  wraps. Each particle senses the trail where it stands (s, scaled and
  clamped to 0-1) and sets sensor distance, sensor angle, turn and step as
  base + amplitude x s^exponent (credited to Jenson in the code). Deposit is
  0.003 x sqrt(particles on the pixel, at most 100), counted with atomicAdd;
  3x3 mean then keep 0.75. Every particle respawns at a random spot every
  1,000 steps (credited to Jenson). Drawn from particle counts a pixel
  through a steep curve, not from the trail.
- **Lague/Slime-Simulation (GPL-3.0, ideas only):** preset A is 250,000
  agents on 1920 x 1080; sensors sum a 3x3 area 35 px out at 30 degrees;
  the turn is a random share of about 14 degrees a step; step 0.4 px; trail
  capped at 1; the blur blends 6% a step toward the 3x3 mean and the fade
  subtracts 0.004 a step; species follow their own trail and avoid others'.
- **PolyPhy / MCPM (MIT):** compares straight ahead with one random side
  direction and picks by chance, weighted by trail^exponent; sensing
  distance and step may be drawn at random each step; the deposit that
  guides agents keeps 0.9 a step, a separate trace of where agents went
  keeps 0.96; both drawn, through a log curve.
- **Ghassaei/gpu-io physarum (MIT):** 0.35 particles a pixel; "Fibers"
  preset sensors 18 px out at 5.5 degrees, turn 45, step 2; keep 0.9.
- **Barradeau/physarum, three.js (public domain):** up to 262,144 particles;
  the one-a-cell check is commented out; keep 0.9; wraps.

Every GPU version drops the one-a-cell limit. Ours is the only one with
walls, food and a limit.

## What it means for the GPU mould

- Jenson's rule (no limit) needs Jenson-style settings; with ours it gives
  ropes. A smaller turn with longer sensors gave polygons.
- A softer brightness curve, or walker counts, shows the same data as thin
  veins with ghost tubes. Both draw the simulation straight, so they fit the
  owner's rule on the picture.
- Respawn is the cheapest thing that kept a fine mesh: one keyed draw a
  walker a step. It is off by default in the big dish because the chosen
  nose rule was measured without it and nothing in the game uses it yet.
- Not tried: feathery fronts into a haze, contagion, settings set by the
  trail a walker stands in (36 Points), a square-root deposit (Bleuje),
  area sensors (Lague), chance-weighted steering (MCPM).

## Correction (2026-11-11, independent review)

A fresh reviewer checked this report's numbers against its raw output and
re-ran `analyze.py` (its tables reproduce exactly; the sign tests and the
Bonferroni over 4 are right). What it found, kept here rather than
rewriting the sections above:

- **One seed, no raw:** the 12 angle / turn / reach arms ("47 at sensor 45 /
  turn 22.5 / reach 18; 0-15 for the rest"), the two 4M arms (171 / 118 / 60
  and 46 / 8) and "density matters little" are one seed each, with no raw
  file kept. Read them as a screen, not findings. "Jenson's rule needs
  other settings" picked the best of those 12 single-seed arms; it was
  tested on 8 seeds only at 1M (43.5 loops), never at 100k.
- **Costs:** the 20.3 and 14.1 ms a tick for the 100k arms came from an
  unsaved four-thread run; the kept raw (`raw/crowd1-s1.txt`,
  `raw/crowd0-s1.txt`, one thread) gives 25.91 and 18.67 ms.
- **Loops are two different measures:** Jenson's are dark regions under a
  brightness cut in a GIF frame; ours are holes of at least 16 fine cells
  where the trail is under 1. "3-5 loops to 180-340" beside "164 at 10 s"
  compares them as one scale; it is only a rough guide.
- The haze figure (41% of Jenson's disc) reproduces on the original GIF's
  last frame (42%), not on the saved JPEG (33%).

The GPU sandbox's re-check of these dishes on 8 seeds is in
[`gpu-sandbox-2026-11-11.md`](gpu-sandbox-2026-11-11.md), *Independent
review*.

