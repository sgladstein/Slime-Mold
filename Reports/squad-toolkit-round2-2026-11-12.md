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

## Confirmed at 32 seeds, with shot craters (main 945e080)

The tech lead's run (12:30Z): main after #113 (shots lay the crater, keep
0.1, full 30), carry 5 at weight 50, 32 fresh seeds (5-36), eight tiers
(none, Swarmer, Sleeper, Sporer, Sprawler, Trunk, Drifter, Grudge; Wary was
removed by the tiers thread), with idle, random and randkit (random play with
the tools) beside the five plans. Run on this container and four runner
threads in 8-seed chunks. Raw lines `tier hand seed score end` in
`squad-toolkit-2026-11-12/raw/r2-32/`; the table is
`squad-toolkit-2026-11-12/r2-32-table.txt`, made by
`squad-toolkit-2026-11-12/r2table.py`. This is the kitplans harness, not
`larderworld census`.

Deciding measure, named before the run: gate 2, the best plan by mean
against each other plan, paired sign test, with Holm over all 152 pairs in
the table (gate 1 and gate 2 together; at m 152 the first pair needs p under
0.00033). Gate 1: each plan against idle, random and randkit.

Mean score, with (mould wiped / cap / squad wiped):

| Tier | loud | plague | farm | random | randkit | best | best vs the other of loud and plague |
|---|---|---|---|---|---|---|---|
| none | +0.86 (16/16/0) | **+1.34** (27/5/0) | +0.24 | -0.04 (7/12/13) | +0.44 (16/7/9) | plague | 24/8, p 0.007, med +0.37 |
| Swarmer | +0.76 (11/21/0) | **+1.28** (26/6/0) | +0.20 | -1.47 (0/1/31) | -1.39 | plague | 22/10, p 0.05, med +0.61 |
| Sleeper | **+0.87** (15/17/0) | +0.48 (0/32/0) | +0.27 | -1.20 | -1.27 | loud | 24/8, p 0.007, med +0.05 |
| Sporer | +0.21 (3/29/0) | **+1.06** (19/13/0) | +0.12 | -1.43 | -0.52 | plague | **30/2, holds**, med +1.17 |
| Sprawler | +0.93 (18/14/0) | **+1.37** (29/3/0) | +0.35 | -0.58 (8/2/22) | +0.10 (14/4/14) | plague | 25/7, p 0.002, med +0.15 |
| Trunk | **+1.37** (28/4/0) | +1.16 (23/9/0) | +0.30 | +0.73 (17/9/6) | +0.51 (17/6/9) | loud | 22/10, p 0.05, med +0.09 |
| Drifter | **+1.65** (29/3/0) | +0.50 (3/29/0) | +0.36 | +1.34 (21/11/0) | +1.01 (19/10/3) | loud | **30/2, holds**, med +1.32 |
| Grudge | +1.13 (18/14/0) | **+1.46** (28/4/0) | +0.18 | -1.49 (0/1/31) | -1.48 | plague | 19/13, p 0.38, med +0.12 |

Gate 2: the best plan still changes with the tier, and after Holm the split
holds on two tiers, one each way: **loud beats plague on the Drifter (30/2)
and plague beats loud on the Sporer (30/2)**. Everywhere the best plan beats
farm, quiet and siege (31/1 or 32/0, all hold). On the other six tiers the
loud-plague pair is 19/13 to 25/7 and does not survive the correction; the
Sleeper's (24/8) has a median difference of only +0.05, a near tie. Against
the 8-seed table, tier none and the Grudge went from ties to plague, and the
Sleeper's loud lead shrank; Trunk, Drifter, Swarmer, Sporer and Sprawler
kept their leader.

Gate 1 fails on four tiers, and that is the main news. The plans beat idle
everywhere (32/0), but random play wipes the mould often on tier none (7 of
32; randkit 16), the Sprawler (8; randkit 14), the Trunk (17) and the Drifter
(21). On the Drifter random beats farm, quiet, plague and siege (31/1 to
26/6, all hold) and ties loud (18/14); on the Trunk loud and plague
tie random (18/14, 16/16). So on those two tiers no plan beats random, and
"a good dish is one a plan wins and random loses" is not met: the Drifter
hunts the squad and the Trunk sends its mass down a few tubes, and wandering
soldiers firing everywhere meet them anyway. That belongs to the tiers
thread (sent), not to the tools. But 945e080 is before the core pull
(#126, `Rules::core_pull` 8), the Larder thread's answer to random play
parking on the core, and before the tiers thread's drifting Drifter (#133);
with both, `larderworld census` has random wiping the Drifter 3 of 48. The
Trunk and Drifter rows are being re-run on main (`raw/r2-32-main/`). On the Swarmer, Sleeper, Sporer and Grudge
random and randkit are squad-wiped 27-31 of 32 and every plan beats them.

Quiet (gas) and siege (det cord) are at the farm hand's level on every tier
(+0.07 to +0.37, 0 wipes), as at 8 seeds.

## Next

- Give quiet and siege a goal where they win: quiet scored on the campaign
  missions (2, 3 and 7 in `examples/campaign.rs`), siege against the one-body
  Colony tier (cut where the far side has no crystal).
- Random wipes the Drifter and the Trunk: the tiers thread's call.
- Step 2: the tools as bursts writing binding 47's block 0 on the GPU.
