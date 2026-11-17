# Element: step D of the gun-card plan, the fourth family; the Torch and heat in the sensors

2026-11-17. Step D of the [gun-system review](../gun-system-review-2026-11-16/README.md)
("bullet families by what they change in the simulation"), after the
[Mind bullets](../gun-mind-2026-11-16/README.md), the
[readers](../gun-readers-2026-11-17/README.md) and the
[route](../gun-route-2026-11-17/README.md). The plan's element family: fire,
water and plants, the ground the slime stands on. The owner, 03:07Z:
"Continue without my guesses. I am busy"; the game designer agreed the
order (08:43Z): "Torch, then Tinder, with Douse held", and added: "Give
walkers a small heat-avoid weight: read the fire field like a negative
scent, so fire becomes a herding tool ... Build Torch as planned, then try
the heat weight as a flag with one A/B film before Tinder."

Page: [Element Bullets](PAGE_URL) (source in `page/`): the clips with their
counts and the windows.

Status: **the Torch is built; heat in the sensors is built as a flag, off,
after one A/B film that did not show it herding; Tinder is next, in its own
change; Douse is held** (wet ground on the view is refreshed only with
walls and crystals, so a runtime Douse needs view plumbing first).

## The Torch

- **What it is** (`Card::Torch`, `Shell::Torch`): a payload with no kill,
  alarm or crater of its own that lights every cell with fuel within
  `torch_r` 6 dish cells (times `Wide`) at full heat (`squad::Burst::light`,
  `torch_light` 1, `BigDish::ignite`; on the GPU one more light in the
  burst's `Engine::ignite` call). The fire does the killing: a walker on a
  cell at `fire_kill` 0.5 heat or more dies, and the fire spreads through
  the plants by the ground's moisture. Living slime with trail of
  `fire_damp` 3 or more does not burn, so the torch eats an edge, not a
  body; thin grass, or grass the slime has worn, never gets hot enough to
  kill (its heat is capped by its fuel). Fire hurts soldiers too
  (`Rules::fire_hurt`).
- Cost 2, sold from planet 2 (`Card::tier` 2, `shells_at`), on the
  armory's Special bullets tab; loadouts `torch` and `torch-blast`; words
  "Sets plants within 6 cells on fire; the fire kills slime."; a flame
  rising out of grass for its icon, a saturated flame yellow for its
  bullet. The rounds line counts torches (`Rounds::cast[7]`) and, last,
  the walkers killed by fire; films' watch lines end with the cells
  burning near the point and the fire's kills.
- **Frame cost**: none while unused; a cast adds one light to the IGNITE
  pass a blast already runs (up to three lights a pass).

## Heat in the sensors (a flag, off)

`BigParams::fire_weight` (key `fire_weight=`, 0 off; GPU Params row
`FIRE_AT` 1232, Params 1232 to 1248 bytes): a sensor adds `fire_weight` x
the fire's heat at its cell, after the crust, on both engines. It reads
ground the player lights, not the soldiers, so it is not the flee from
gunfire ([dead ends](../dead-ends.md), whose entry now says so). Frame
cost off: none (the term is skipped at 0). On: one more read of the plants
block a sensor; not timed on the 1070.

**The A/B film** (`raw/film.sh`; planet 1:2, idle hand; three torches at
16 s on the grass beside a thin vein, world 122,165 / 122,175 / 122,185;
watched within 12 cells of 125,175 and from 12 to 24):

| Clip | Fire kills by 18, 20, 26 s | Walkers within 12 at 18, 22, 26 s | Cells burning within 24 at 17, 22, 26 s | Slime at 26 s |
|---|---|---|---|---|
| no torch | 0, 0, 0 | 39, 37, 0 | 0, 0, 0 | 24,903 |
| torch, heat weight 0 | 7, 12, 12 | 22, 34, 0 | 431, 221, 140 | 24,890 |
| torch, heat weight -5 | 10, 21, 25 | 23, 28, 48 | 435, 420, 297 | 24,877 |
| torch, heat weight -20 | 8, 14, 17 | 24, 24, 35 | 439, 362, 242 | 24,885 |

What the clips show: with no torch the vein down the grass patch's east
side draws back by 26 s on its own. With the torches and weight 0 the
fire burns the patch, the vein walks into the flames, and its damp trail
puts them out where it goes (12 walkers burn); it draws back as without
the fire, and the fire is mostly out by 26 s. With the weight the slime
stops at the fire's edge and rings it: the fire, no longer doused, burns
longer and wider, the vein stays beside it instead of drawing back (35-48
walkers within 12 cells at 26 s against 0), and more of it burns (17-25).
One scene, one seed: the weight changes what the slime does at a fire (it
outlines it instead of crossing it, and the fire lives longer), but it did
not keep the slime away; here it held the slime by the fire. It stays a
flag, off, for the designer to call; a herding test would put food beyond
a fire line and count who crosses.

## Tests

- `rounds::tests::a_torch_lights_the_plants_and_the_fire_kills` (CPU, the
  flats map planted in kind bands): the card, its burst, `Wide`, the
  `set=` keys; one torch on the slime in the thickest grass by the home
  lit 102 cells, killed none at its cast and 68 by fire in 4 s (out within
  a second: the ground under a body is damp); with no plants it lit and
  burned nothing.
- `gpu::larder_world::tests::a_torch_lights_the_plants_on_the_gpu_as_on_the_cpu`:
  the same, CPU against GPU, with heat out of the sensors and at -20: 102
  cells lit on both, the fire field equal (max difference 0), 68 killed by
  fire on both, the living equal each second (llvmpipe).
- `gpu::larder_world::tests::the_first_germ_off_a_planets_middle_infects_as_on_the_cpu`
  (the technical lead's review of #416): planet 1:2 (200 x 124 fine cells
  at scale 1), the first germ round on the walker farthest from the
  middle (113 cells off), with 1,033 walkers within 20 of the middle; the
  living sick match the CPU slot for slot every second, through the old
  timed seeding's time (32 s): 25 at the cast, 108 a second on, none 7 s
  on. With the old code (the seeding not turned off) it fails 21 s on:
  2,973 sick on the GPU, 2,008 of them within 40 cells of the middle, none
  on the CPU. The seeding is now off from the world's start
  (`GpuWorld::of`), not at the first germ, so a `sick=` key cannot set it
  off either; no other burst or round path places anything at the middle
  (they all take the cast's point).

## The paired windows in play

`raw/windows.sh`, table by `raw/windows_table.py` in `raw/windows.txt`
(planets 1-3, the readers' planet scenes) and `raw/windows-45.txt`
(planets 4 and 5). Each plays one game to T (25 and 40 s), then every
squad loads the variant's deck and the window runs 10 s. Plain is plain
bullets, torch is torch bullets, blast is blast bullets (cost 3, the kill
the torch is set against).

| Planet (biome, windows) | Kills: plain / torch / blast | Torch's more kills than plain |
|---|---|---|
| 1, meadow (6) | 230 / 350 / 258 | 6 of 6 |
| 2, barrens (2) | 117 / 123 / 119 | 2 of 2 |
| 3, fen (2; one with no kills on any) | 11 / 18 / 12 | 1, 1 tie |
| all (10) | 358 / 491 / 389 | 9, 1 tie (p = 0.004) |

- **The torch kills more than plain bullets** in 9 of the 9 windows with
  any kills (p = 0.004) and leaves less slime at the end in 8 of 9 (p =
  0.039); against the blast, which costs 3 to its 2, more kills in 7 of 9
  and less slime in 7 of 9 (p = 0.18 each). Only the first survives a
  Bonferroni correction over the four comparisons (p < 0.0125).
- **The ground decides it**: on the meadow it took half again as many as
  plain (350 against 230); on the barrens' sparse scrub and the fen's wet
  ground it is a plain bullet with a little fire (123 against 117, 18
  against 11). A torch is a meadow and jungle card, as the plants run.
- The squads' health and numbers did not change (8 soldiers left in every
  window; health 95-100 on average): the squad hands fight at range, out
  of their own fire.
- **Planets 4 and 5** (`raw/windows-45.txt`): in these scenes the squads
  had not met the slime by 50 s, so no variant killed anything; the
  torch's jungle (planet 5) is not measured.
- A balance note, not a gate (build first, balance later): at cost 2 the
  torch matches or passes the blast on meadows. The balance pass may raise
  its cost or shrink `torch_r`; neither is changed here.

## Raw

- `raw/windows.sh`, `raw/windows_table.py`, `raw/windows.txt`,
  `raw/windows-45.txt`: the windows.
- `raw/film.sh`, `raw/make_clips.py`, `raw/counts.json`: the clips.
