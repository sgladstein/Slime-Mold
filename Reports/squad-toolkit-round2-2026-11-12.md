# The squad toolkit, round 2: carrying tools (2026-11-12)

Thread *Design squad tools from the mould's model*. Round 1
(`squad-toolkit-2026-11-12.md`) ended with every plan beating idle and random
play but one plan, **loud** (a runner carrying blasts to the mould's densest
block), best on all four tiers, so the plans did not counter each other
(gate 2 failed). The owner was away overnight and asked the threads to settle
questions together (04:48Z). The game designer review set the order: add a
counter from the game, not a weaker blast, re-run the plan-by-tier matrix
after each, and stop once loud loses on some tier.

Everything here is the CPU race world (`larder_world::race_world`), 8 paired
seeds, cap 600 s, score v2 (`Game::score`), the round 1 hands
(`examples/kitplans.rs`). Raw output sits beside round 1's, in
`squad-toolkit-2026-11-12/raw/` (`carry*`, `probe-*-keep01-full30.txt`).

## Counter 1: tools take carry space

`KitRules::carry_max`, `carry_each`, `tool_weight`, `restock_r`;
`Kit::restock`, `Kit::held`; `Soldier::load` (harvest fills `carry - load`).
A soldier picks tools up at the larder (within `restock_r`, 6 dish cells),
each takes `tool_weight` of its 300 carry, and a use spends one held by a
soldier in reach. Tools are still paid from the bank when used. Smell needs
none. In the plans a runner takes `carry_max`, everyone else `carry_each` (1),
and an empty runner walks home first.

Deciding measure, named before the run: per tier, the best plan by mean and
each plan against it, paired (gate 2); and gate 1 as in round 1.

Mean score (mould wiped / cap / squad wiped):

| Setting | Tier | loud | plague | farm | best |
|---|---|---|---|---|---|
| round 1 (no carry) | none | +1.76 (8/0/0) | +0.84 | +0.20 | loud |
| carry 2, weight 100 | none | +0.27 (0/8/0) | +0.88 (4/4/0) | +0.20 | plague, 8/8 over loud |
| | Swarmer | +0.18 | +0.96 (4/4/0) | +0.18 | plague 8/8 |
| | Sleeper | +0.27 | +0.47 (0/8/0) | +0.26 | plague 8/8 |
| | Sporer | -0.37 (0/5/3) | +0.82 (3/5/0) | +0.13 | plague 8/8 |
| carry 2, weight 50 | Swarmer / Sleeper / Sporer | +0.19 / +0.25 / -0.42 | +1.09 / +0.48 / +0.84 | | plague |
| **carry 5, weight 50** | none | **+1.03 (5/3/0)** | +1.02 (5/3/0) | +0.20 | tie, 4/4 |
| | Swarmer | +0.63 (2/6/0) | **+1.16 (5/3/0)** | +0.18 | plague, 7/1 |
| | Sleeper | **+0.83 (3/5/0)** | +0.48 (0/8/0) | +0.26 | loud, 6/2 |
| | Sporer | -0.43 (1/3/4) | **+1.06 (5/3/0)** | +0.10 | plague, 7/1 |

At carry 2 loud fell to the farm hand's level everywhere and plague took
every column: the rule had moved the dominance, not broken it. Halving the
weight changed nothing (within 0.05 on every cell), so the cost is the trips
home, not the lost harvest. A runner holding 5 is the setting kept
(`KitRules` default now: `carry_max` 5, `tool_weight` 50): loud is best on the
Sleeper, plague on the Swarmer and the Sporer, and they tie on tier none.
The Sporer punishes loud hardest (4 squad wipes): each blast scatters spores.

Free shots (`Rules::price` 0, the Larder thread's #106 default), tier none,
carry 2: the plans don't move (plague +0.89, loud +0.26); random rises from
-1.64 to -0.65 and once wipes the mould (`carry2-price0-none.txt`).

## The tiers thread's new moulds

On a scratch merge of this branch with the tiers branch (cf62e8e: Drifter,
Sprawler at explore 200, Trunk, and the two habituation tiers Wary and
Grudge), carry 5, weight 50 (`carry5-w50-new-tiers.txt`):

| Tier | loud | plague | farm | random | best |
|---|---|---|---|---|---|
| Sprawler | +0.59 | **+1.09** (5/3/0) | +0.34 | -0.95 | plague, 7/1 |
| Trunk | **+1.48** (8/0/0) | +1.08 | +0.31 | -0.28 | loud, 6/2 |
| Drifter | **+1.63** (7/1/0) | +0.57 | +0.36 | +0.08 | loud, 8/0 |
| Wary | **+1.79** (8/0/0) | +0.54 | +0.25 | +0.95 | loud, 8/0 |
| Grudge | +1.35 | **+1.36** (6/2/0) | +0.11 | -1.59 (0/0/8) | tie, 3/5 |

So across nine tiers the best plan changes: **loud** on Sleeper, Trunk,
Drifter and Wary; **plague** on Swarmer, Sporer and Sprawler; a tie on none
and Grudge. Gate 2 passes by the mean. It is not yet significant: the
plan-vs-plan pairs are 6/2 to 8/0 at 8 seeds, and only the 8/0 pairs (p
0.008) would survive a correction; 16 seeds are needed.

Two flags for the tiers thread, sent: at explore 400 random wiped the
Sprawler 7 of 8 (fixed at explore 200, random -0.95 now); on the Wary,
random (+0.95) beats farm, quiet and siege 8 of 8.

Quiet (gas) and siege (det cord) are never best: the game has two working
strategies here, not four. Gas protects a harvester but kills nothing; the
cord starves nothing until food flows (`energy_flow`, the one-body thread).

## Answers to the germ

Plague led every tier at carry 2, so two mould traits were tried on the germ
probe (`kitprobe test=germ`, 4 seeds, living walkers at 30 s; none 3,625, germ
796 in round 1's 8 seeds):

- **Amputation by steering** (the designer's first pick): healthy walkers add
  `germ_weight` x germ at their sensors (-5, -20) and sick ones lay no trail
  (`sick_trail` 0), so the body should pull away from a sick part. Living at
  30 s: 565 and 524, no better than the plain germ (dead end; the wave
  crosses the body in a few seconds, faster than a walker turns). Removed.
- **Resistance** (`BigParams::germ_resist`, kept, off at 1): a walker catches
  only where germ is over `sick_catch` x `germ_resist`. Catch 0.5 / 1 / 2:
  ever sick 2,101 / 1,768 / 1,238 of 2,303, living at 60 s 962 / 1,627 /
  2,672: a wave takes the thick parts and the mould grows back. In the plans
  (x10, carry 5): tier none plague +1.02 -> +0.47 and loud is best (7/1);
  Swarmer plague +1.16 -> +0.85, still best, tied with loud 4/4
  (`carry5-catch2-none.txt`, `carry5-resist10-none-swarmer.txt`).

With carry 5 the ladder already splits, so resistance stays a knob for the
tiers thread to name (`germ_resist`) if a campaign mould needs one.

## The shot crater's strength

For the Larder thread's shot craters (keep 0.1, full 30 asked), the blast and
cord probes at those values against round 1's defaults (keep 0.5, full 300),
8 seeds, means: blast, walkers back in the disc 98 / 98 / 94 at 5 / 10 / 20 s
against 77 / 82 / 99 (none 146 / 188 / 235 without a crater); cord, trail on
the cut 1 / 2 / 67 at 5 / 10 / 20 s against 11 / 13 / 137. No harm, so one
shared strength; the Larder thread's #113 sets the defaults.

## Frame cost

The carry rule is a pass over soldiers at each frame of the plan (8-40
soldiers): nothing measurable (loud 7-12 ms a tick, as in round 1).
`germ_resist` is a multiply in the catch test. The germ pass is still 2-3x
the tick while a wave runs.

## Next

- Confirm on main after #113 (shot craters on), 16 seeds, the nine tiers.
- Give quiet and siege a tier where they win (gas against a mould that
  floods harvesters; the cord once food flows).
- Step 2: the tools as bursts writing binding 47's block 0 on the GPU.
