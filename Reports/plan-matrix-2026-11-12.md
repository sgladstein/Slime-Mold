# The plan-by-tier matrix after core defence (2026-11-12)

The re-run the design review and the tech lead asked for (14:13Z): twelve plans against eight mould tiers on main after #126 (core defence, commit 0249934), with the toolkit hands as rows, Holm over the whole matrix, the median paired difference beside each best cell, and a column called "best" only when its leader beats the runner-up after Holm. One command makes it: `python3 scripts/plan_matrix.py run OUT seeds=… tiers=…`, then `table OUT`. The table is [plan-matrix-2026-11-12/matrix.md](plan-matrix-2026-11-12/matrix.md) and the raw output is in [raw/](plan-matrix-2026-11-12/raw/). The [page](https://claude.ai/artifact/17cgbF9X4UdYMQANH91xVA) has the same table (source `plan-matrix-2026-11-12/page.html`, see [pages.md](pages.md)).

**It is incomplete on purpose.** The run was stopped at 17:10Z when the owner questioned hours-long censuses; the testing rule that followed (PR #150, 17:26Z) keeps strategy censuses for the player AI, but small, with the runner time named first. So it has one seed block, 201-216, not two. The Swarmer has no rows, the Sleeper has 11 paired seeds and the Grudge 14 with no toolkit rows. The other five tiers have all 16. The first block cost about 2 h on each of four machines.

The plans come from two harnesses on the same race world and seeds: `larderworld census` (idle, random, farm and the AI's boom, claim, fort, strike) and `kitplans` (randkit and the toolkit's loud, quiet, plague, siege). Per the integration rule, the kitplans columns are their own harness's numbers.

## Measures

M1-M3 come from the reviews' asks and the tiers thread's gate, set before the run; M4 was read afterwards.

- M1: per tier, the leader by median against the runner-up, paired by seed, sign test, Holm over all 125 tests (each plan against idle and random, plus each leader against its runner-up).
- M2: the tiers thread's gate, the strike best on at most 2 tiers.
- M3: random's mould wipes per tier (the integration gate's count).
- M4: dead time, the AI plans' stuck seconds.

## The matrix

Median `Game::score` (v2), then the ends: mould wiped / cap / squad wiped. The number after each tier is its paired seeds.

| Tier | idle | random | farm | boom | claim | fort | strike | randkit | loud | quiet | plague | siege | Leader over runner-up |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| plain (16) | −0.32 0/16/0 | −1.66 0/2/14 | +0.31 0/16/0 | +0.20 0/16/0 | +0.32 3/13/0 | +0.21 0/16/0 | +0.33 4/12/0 | −1.15 4/3/9 | +1.47 10/6/0 | +0.15 0/16/0 | +1.45 16/0/0 | +0.24 0/16/0 | loud over plague 6-10, tie |
| Sporer (16) | −0.41 0/15/1 | −1.73 0/2/14 | +0.21 0/16/0 | +0.12 0/16/0 | +0.14 0/16/0 | +0.11 0/16/0 | +0.10 0/16/0 | +0.95 8/2/6 | +0.15 6/10/0 | +0.09 0/16/0 | **+1.36 10/6/0** | +0.13 0/16/0 | **plague over farm 16-0, median +1.14, Holm p 0.004** |
| Sleeper (11) | −0.17 0/11/0 | −1.77 0/2/9 | +0.32 0/11/0 | +0.33 0/11/0 | +0.49 0/11/0 | +0.33 0/11/0 | +0.49 0/11/0 | −1.58 0/0/11 | +1.22 7/4/0 | +0.34 0/11/0 | +0.49 0/11/0 | +0.34 0/11/0 | loud over strike 10-1, +0.72, p 0.26, tie |
| Drifter (16) | −0.15 0/16/0 | +0.29 7/4/5 | +0.42 0/16/0 | +0.38 0/16/0 | +1.84 14/2/0 | +0.38 0/16/0 | +2.30 13/3/0 | +0.92 8/4/4 | +1.80 14/2/0 | +0.39 0/16/0 | +0.48 6/10/0 | +0.42 0/16/0 | strike over claim 13-3, +0.47, p 0.45, tie |
| Sprawler (16) | −0.76 0/8/8 | −1.77 0/1/15 | +0.46 7/9/0 | +0.10 4/12/0 | +0.21 5/11/0 | +0.10 5/11/0 | +1.97 9/7/0 | −0.36 6/2/8 | +1.39 11/5/0 | +0.09 0/16/0 | +1.50 16/0/0 | +0.24 2/14/0 | strike over plague 9-7, +0.53, p 1, tie |
| Trunk (16) | −0.28 0/16/0 | −0.67 5/3/8 | +0.33 1/15/0 | +0.26 2/14/0 | +1.79 9/7/0 | +0.27 2/14/0 | +2.20 13/3/0 | +0.49 7/6/3 | +1.54 16/0/0 | +0.25 0/16/0 | +1.40 14/2/0 | +0.30 0/16/0 | strike over claim 13-3, +0.37, p 0.45, tie |
| Grudge (14) | −0.33 0/14/0 | −1.75 0/2/12 | +0.15 0/14/0 | +0.11 0/14/0 | +0.13 0/14/0 | +0.11 0/14/0 | +0.16 1/13/0 | – | – | – | – | – | strike over farm 8-6, +0.01, p 1, tie |
| Swarmer | not run | | | | | | | | | | | | |

## What it says

- **Core defence took the strike's dominance away on the plain mould.** Before craters the strike scored +2.08 on plain and wiped it on 5 of 8 seeds (seeds 53-60, [the earlier report](larder-world-tiers-2026-11-12.md)). Now it scores +0.33 and wipes 4 of 16, level with claim and farm. On the Sporer, Sleeper and Grudge it is no better than farming.
- **M2, the strike gate, passes on significance but not on medians.** The strike leads by median on three tiers, the Drifter, the Sprawler and the Trunk, but no lead survives Holm. It is never significantly best.
- **One cell survives the correction: plague on the Sporer**, above farm on 16 of 16 seeds (median +1.14). Every other column is a tie between two or more plans.
- **The toolkit plans lead where the strike doesn't.** Loud leads on plain and the Sleeper, and plague on the Sporer; plague wipes plain and the Sprawler on 16 of 16. The strike's three tiers are the ones where the AI's own plans still win: there the strike and claim wipe the mould on 9-14 of 16.
- **M3, the Drifter and the Trunk are too soft.** Random wipes the Drifter on 7 of 16 and the Trunk on 5; on every other tier it wipes none. Those are also the strike's tiers, so the strike's lead there probably says more about the tiers than about the strike. I haven't looked at why.
- **M4, dead time:** no AI plan was stuck for a second on any tier; idle was stuck up to 30 s, random up to 806 s.
- **Plans still differ by tier,** which is what the matrix is for: the leader is loud, plague or the strike depending on the mould. But with 16 seeds most leads are ties.

## Seeds used

Seeds 201-216 are now used by this run as well as the integration gate. The player AI's held-out seeds (301 and up) and the map Rim are still unplayed.
