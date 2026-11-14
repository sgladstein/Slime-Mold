# Plan-by-tier matrix with no food rule (2026-11-14)

The approved tier run (owner OK 01:05Z), judged against the designer's D3 table.
Code: `2371bf0` (#224 census-size C, #220 rations off, #213 cut/take and bank
spending, #218 2 s craters). Measures were named and committed before the run:
[raw/MEASURES.txt](plan-matrix-tiers-2026-11-14/raw/MEASURES.txt).

The setup was `larderworld census` at census size: shape C, scale 1, 6,250
walkers and 4x crystal charge. The race world ran to a 900 s cap on seeds
301-308 with nine hands. There was one plain column and seven tier columns,
72 games each. It took 10.7 min per column on 4 threads here, 1 h 26 min in
all, with no runners.

Tables per tier: [matrix.md](plan-matrix-tiers-2026-11-14/matrix.md), written
by [tab.py](plan-matrix-tiers-2026-11-14/tab.py) from `raw/`.

## Result

**The deciding measure fails.** On plain C no plan wipes the mould on any of
the 8 seeds, and every AI plan reaches the cap. Idle and random lose 8/8.
Strike no longer loses: with no food rule its squad survives on every seed.

| tier | best by median | runner-up (paired) | mould wiped by take / strike / cut | D3 intended | D3 should lose |
|---|---|---|---|---|---|
| plain C | take +0.20 | cut 6-1 | 0 / 0 / 0 | cut | strike (ties boom, +0.155) |
| Drifter | strike +0.34 | take 4-4 | 1 / 1 / 0 | strike | boom (loses to strike 1-7) |
| Sprawler | take +0.27 | cut 8-0 | 2 / 3 / 0 | fort | strike (mean +0.92, wipes 3) |
| Trunk | take +0.30 | cut 8-0 | 1 / 2 / 0 | cut or claim | strike (loses to take 1-7) |
| Sporer | take +0.46 | strike 7-1 | 3 / 3 / 0 | claim untaken first | strike (second) |
| Sleeper | take +0.49 | cut 8-0 | 0 / 0 / 0 | bait-then-strike | claim slowly (loses to strike 0-8) |
| Swarmer | take +1.34 | cut 7-1 | 4 / 2 / 3 | fort or kill zone | repeated strikes |
| Grudge | take +0.37 | cut 8-0 | 2 / 0 / 0 | boom or claim spread | fort (ties boom) |

Here is how the run did against each D3 gate.
- **No plan best on more than half the tiers: fails.** Take is best by median
  on 7 of 8.
- **Every plan best somewhere: fails.** Boom, claim, fort and boom with
  turrets are never best. They never send soldiers at the mould, so their
  most is a stalemate: about +0.15, with the mould regrowing to 12,500.
- **Idle loses everywhere: passes.** Idle is the lowest AI-plan score on
  every tier, at -0.33 to -0.73.
- **Each counter beats random on at least 6 of 8: passes.** Every AI plan
  beats random 8-0 on every tier.

**Holm:** nothing survives. The family has 120 tests (per tier, every plan vs
idle and vs random, plus best vs runner-up). With 8 seeds the smallest sign
p is 0.0078, and that is above 0.05/120. The paired counts above are records,
not proofs.

## What it says

1. **Walking in is now the only way to win, and take is the best way to walk
   in.** Take holds the cut points with 40 soldiers and 6-8 turrets, then
   claims the crystals it cuts off. It is the only plan that wipes the mould
   on Swarmer half the time (4/8), and it gets Sleeper down to a median of 53
   walkers. On plain C it leaves a median 8,456 of 12,500.
2. **Plain C is the hardest dish.** It is harder than every tier: no plan
   wipes it, while Sporer, Swarmer, Sprawler, Grudge and Drifter are each
   wiped somewhere. Tiers that the ladder means as harder (Sporer, Grudge)
   play easier than plain C for an attacking hand.
3. **The passive plans can't be anyone's counter.** Fort, boom and claim never
   attack, and a cap ends at a stalemate score. So D3's fort/boom/claim
   counters need either an attacking variant of those plans or a score that
   pays for holding ground.
4. **Banks still pile up.** Boom and fort end with 17-34k unspent, since only
   `spend=` hands buy past `buy_to`. Spending the bank on turrets alone
   (boom+tur) changes nothing (+0.155 vs +0.154).

## Tuning notes (not vetoes)

- AI thread: the next try is take as the base for every plan, varying what it
  holds (fort = turret ring at the cut, boom = economy then take at 5 min,
  claim = take from the far crystals). Spending the bank should be the
  default, not a knob.
- Designer: by D3, take is dominant. Score v2 pays only for a wipe or a share
  at the cap, so plans that hold ground all score about +0.15.
- Tiers: Sleeper and Swarmer fall to cut-and-take; plain C is stiffer than
  the ladder's top tiers.
- The owner is moving to squad control without the larder (11-14 ~04:15Z),
  so these commands will change. This matrix is the baseline to compare the
  squad version against.
