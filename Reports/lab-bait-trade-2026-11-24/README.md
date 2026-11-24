# Lab: bait or drain, the trade on planet 7 (2026-11-24)

The Lab lane's fourteenth job, on the lab branch `claude/project-thread-2o4cic`
only (feeding `14ba8f8f`, the `drain@` hand `8573ff93`, the batch
`raw/pc-trade.ps1` at `eaf9f35b`), not in the game. Brief from Direction
2026-11-24 12:08Z, after [crystal bait](../lab-crystal-bait-2026-11-24/README.md):
settle the trade on planet 7, bait against drain against off on the
wanderer, six paired seeds, deciding measure squad wipes and soldiers at
the boss's end; if drain beats bait, the trait comes off Enemy's list.

## How

Planet 7, run seeds 1 to 6, the squad of 8 (bank 3,000) on the crystal by
its landing, 2:30 a game. One wanderer placed at 0:20 on the slime about
280 units away (`boss=20,X,Y`, `boss_kind=2`), the same spot in all three
arms, no other mini-boss (`boss_first=9999`). Arms: **off** (hold, no
feeding), **bait** (hold, the crystal left charged, `LAB_FEED=1.0`),
**drain** (`drain@`: drain that crystal and buy soldiers, `LAB_FEED=1.0`).
Run by Engine on the GTX 1070, 18 games in 7.6 minutes (`raw/pc-output.txt`).
A placed wanderer is spent at 1:57 (its life), so "the boss's end" is
1:57 or when it breaks.

## Results

| Seed | Off | Bait | Drain |
|---|---|---|---|
| 1 | 1 soldier left; boss grew 475 → 1,777 | 8; boss 464 → 137 | 13; boss → 48 |
| 2 | 8; boss 327 | 8; boss 343 | **wiped 1:32**, 8 to the boss (449) |
| 3 | 8; boss 638, never came | 8; 642 | 13; 642 |
| 4 | 8; boss grew 534 → 1,097 | 8; **broke 1:09** | 13; **broke 1:12** |
| 5 | 8; nothing came (0 shots) | 8 | 13 |
| 6 | 8; boss grew 132 → 847 | 8; **broke 0:50** | 13; **broke 0:50** |

**The deciding measure.** Wipes: off 0, bait 0, drain 1 of 6. Soldiers at
the boss's end: off 41, bait 48, drain 65; drain had 5 more than bait in
5 of 6 seeds (13 against 8, from 615 to 939 food drained) and 8 fewer in
one. Seed 2's wipe is not the trade at work: the wanderer never hunted
(every boss line reads Go or Sit). In the drain arm it sat on its first
crystal at 0:45, took in slime (248 → 698) and set off for the next fresh
crystal, (883,40), 50 units past the squad, and went through the squad on
the way; in the bait arm it never got off its first crystal (about 310
members on Go all game). The arms had parted before the squad's crystal
mattered.

**What feeding does.** The fed wanderer starves at every crystal on its
way, not at the squad's: seed 1, off it took in slime at each crystal and
grew to 1,819 by the time it was 62 from the squad; fed, it lost walkers at
each (464 → 185 → 145) and never came near. Off, it grew in 3 of 6 seeds
(to 847-1,777); fed (bait or drain), it ended at 10 to 137 in those three
and broke in two. Whether the squad drains its own crystal made no
difference to that.

## Recommendation

**Drain beats bait as a player choice: the trade is not there.** The
crystals that starve a wanderer are the ones on its way, not the one the
squad drains, so draining paid 5 soldiers and cost no bait in 5 of 6
seeds; the one drain wipe came by another road. By Direction's rule, the
"leave a crystal charged as bait" framing and its card come off.

What is left is a slime rule, not a counter: fed wanderers stay small
instead of swelling at each crystal (3 of 6 seeds here). If Enemy wants
wanderers that grow less, it is a cheap way; as a player tool it is weak.

## Raw

`raw/pc-trade.ps1` (the 18 games; PowerShell 5.1 needs `Continue`),
`raw/pc-output.txt` (the PC's printed lines as Engine relayed them; the
boss lines at 15 s for seeds 1 and 2 are quoted above). The placed spots
were picked from each map's 0:01 picture (orange within 220 to 320 units
of the squad).
