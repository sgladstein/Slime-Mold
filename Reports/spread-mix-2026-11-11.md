# Spread from one spot, and mixed walkers in one mould (2026-11-11)

Research with looks and small runs, no game change. The owner, 13:58Z:
"if these different types of molds are started just in one location how
well do they spread across the map, what happens if we have a mixed
population of molds ... what if some of the walkers had different
properties than other ones? What kind of behavior would that create". The
Larder thread asked for mixes with a clear strength and a clear weakness,
named by difficulty tier, for its persistent world on the big dish. Page:
https://claude.ai/artifact/Mp6XiXvfqo4gxc1gWaMjRz (source in
[`spread-mix-2026-11-11/`](spread-mix-2026-11-11/)).

These are looks, with 3 seeds each and no paired test: no deciding measure
was named in advance, so nothing here is a measured win. All runs have no
food, on the 1024 x 640 big dish.

## Spread from one spot

Every type starts in a disc at the dish centre.
- Network types: 1M walkers in an 80-cell disc.
- Droplet types: 30k walkers in a 200-cell disc.

Packed one walker to a cell, droplet walkers jam and don't move at all
(`raw/drop-packed.txt`).

| Type | What it does | Numbers (3 seeds) |
|---|---|---|
| Jenson network (`net`) | Closes into **one thin ring** that widens slowly | Tube 1832-1939 cells at 20 s (0.4% of the dish); ring radius 64 → 96 cells from 2 to 20 s (seed 1) |
| Chance turn (`chance_exp` 4) | The same ring, thinner and slower | Tube 1198-1202 cells at 20 s; radius 61 → 115 |
| Explorers (`explore_steps` 900, spread 1) | **Fills the dish in about 10 s** with a coarse mesh, then thins out | Tube 13828-14013 cells at 10 s, 8827-9322 at 20 s; 34-38 loops |
| Droplets (`blocked_deposit` 1) | Stay where they formed | 90th-percentile walker distance 185 → 180-182 in 30 s |
| Grazing droplets (`graze_weight` 10) | Creep outward | r90 185 → 192-199 in 30 s |
| Beads (SO 27) | Pull inward | r90 172 → 166 |

So, with no food, only explorers spread across the map. Everything else
holds its ground or curls into a ring. Raw: `raw/spread-net.txt` (runner)
and `raw/drop.txt`.

## Mixed walkers (`mix_share`, `mix_reach`, `mix_angle`, `mix_turn`, `mix_step`, `mix_deposit`)

A share of the walkers is a second kind with its own multipliers. Both
kinds lay and read **one shared trail**, so this is one mould, not two
species. The `kinds` style draws where each kind's walkers stand: the first
kind amber, the second teal. The shared trail is dim grey.

The main finding is that **the kinds sort themselves**. In every mix where
the second kind differs in how it moves, the two kinds end up in different
places.

| Mix | Second kind | What it does | Numbers (3 seeds, median distance from the start at 20 s) |
|---|---|---|---|
| **Outriders** (`scouts`) | 10%; reach x3, step x2, deposit x0.3 | Within 2 s the scouts race to the dish wall and ride it as a faint halo. The core stays as nested rings in the middle, with some scouts as an inner ring | Scouts 77-79, core 85-87 (median hides the split: part at the wall, part inside); 6-7% of scouts off the trail against 0% of the core |
| **Splitter** (`wanderers`) | 20%; turn x4, sensor angle x2 | The ordinary walkers fly out to the wall as a big ring. The wanderers hold a tight ring at the centre. At 20 s on seed 1 they had become one cord across the dish | Ordinary 138-372, wanderers 29-49: apart on 3 of 3 |
| **Roamer** (`twoscale`) | 50%; reach x3 | The long-reach half rides the wall; the short half balls up into a knot that **crawls across the dish** and joins the wall by 20 s | Both kinds 251-329 at 20 s against 13-28 at 2 s |
| Builders | 30%; step x0.5, deposit x3 | Looks like the plain network ring | Both kinds 70-77 (seed 3 drifted to 129-158) |
| **Grazing outriders** (`grazescouts`) | Grazing droplets plus 20%; reach x3, deposit x0.3 | The droplets stay as bodies. The scouts leave them and haze out over the **whole dish** | r90 299-303 at 30 s against 192-199 for plain grazers (3 of 3); 25% of scouts off the trail |
| Bridged droplets (`dropscouts`) | Droplets plus 20%; reach x3, deposit x0.3 | Scouts collect at the edges of the droplets and link neighbours into chains | r90 174-176 against 180-182 (pulls in slightly) |
| Fast droplets (`dropfast`) | Droplets plus 20%; step x2, deposit x0.3 | Like plain droplets | r90 180-182 |

Raw: `raw/mix-net.txt` (runner, lines at 2/10/20 s) and `raw/drop.txt`.
Script: [`spread-mix-2026-11-11/run.sh`](spread-mix-2026-11-11/run.sh).

Cost: the two kinds add one multiply per sensed walker, which doesn't show
in the tick times (105-120 ms a tick at 1M walkers on a runner, the same
as plain).

## What this means for play

Different walkers in one mould don't blend. They split into parts with
jobs: outriders on the edge, a core that holds, a cord, a crawling knot.
That gives a mould a body plan, which is closer to a creature than one
uniform mesh. For a difficulty ladder:
- **Outriders** expand to the map edge in seconds, but the scouts are faint
  (deposit x0.3), so they are easy to cut. The core holds.
- **Grazing outriders** do the same on a droplet body.
- **Splitter** sends most of its mass out and keeps a dense core.
- **Roamer** moves its body.

The next step is a paired test on a dish with food, to see whether these
body plans change who gets to a crystal first and who keeps it.

## Commands

```
cargo build --release --example bigdish
bash Reports/spread-mix-2026-11-11/run.sh spread-net|spread-drop|mix-net|mix-drop [JOBS] [OUT]
SEEDS=1 SECS=20 bash Reports/spread-mix-2026-11-11/run.sh mix-net 4 /tmp/out   # stills in /tmp/out/<mix>-s1/
```
