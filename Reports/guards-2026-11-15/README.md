# Guards while working: posts at the work's edge (2026-11-15)

Owner, 2026-11-15 18:00Z: "Have someone review the guards while harvesting
option. It doesn't seem to work well. Either improve it or remove it."

The option (`src/mould/larder_world/cover.rs`, PR #276; key **I**, the
**Guards** button on the squad panel): a squad draining or building with
slime near posts a share of itself (none, some a third, more a half) on the
slime's side while the rest work faster to make up most of it.

Page (film with counts): https://claude.ai/artifact/XXkb98zpvyfuuStHjK3otP,
source in `page/`.

## What it did

Each guard took its own bearing toward the slime and stood up to 8 cells past
the work's edge, stepping back only until the slime under its post was
"thin" (4 walkers within 2 cells). On shape C a thin film reaches far round
the body, so the posts were usually the full 8 cells out: 14-22 cells from
the crystal's middle, alone in the film, outside the squad's ring. They were
bitten there, wandered with the film, and died; the workers were bitten about
as often as with no guards at all. With pumps now the main income (#322) the
option also has little work to cover: a pump went up in 2-6 s with 8
builders (`raw/pump-*.log`), so guards stood for at most a few seconds.

## Measure

Deciding measure, named before the run: the squad's summed health at 60 s
and soldiers lost, beside slime killed and the crystal drained, so a guard
that saves nothing but kills more shows as a trade.

`larder-world mode=squads seed=S look=DIR hand=guardwork at=60` with
`GW_WORK=drain` and `GW_COVER=none|some|more` (`raw/gw.sh`): eight soldiers
on the Fork, sent at 1 s to drain by hand the big crystal with the most
slime within 30 cells. The new hand `guardwork` prints a `GW` line a game
second. "Edge" is the same run with guards posted at the work's edge
(`raw/gw0.sh`, the build with `out` 0, equal to the change here). llvmpipe,
4 seeds, about 25 min of runner time.

| seed | guards | alive | health | slime killed | guard-seconds | workers bitten (s) | guards bitten (s) |
|---|---|---|---|---|---|---|---|
| 1 | off | 8 | 629 | 2627 | 0 | 49 | 0 |
| 1 | out (was) | 7 | 396 | 3262 | 156 | 56 | 57 |
| 1 | out, more | 8 | 351 | 3394 | 236 | 55 | 89 |
| 1 | edge (now) | 7 | 483 | 3413 | 155 | 46 | 48 |
| 2 | off | 8 | 712 | 952 | 0 | 45 | 0 |
| 2 | out (was) | 8 | 689 | 1004 | 162 | 20 | 33 |
| 2 | out, more | 8 | 698 | 1036 | 212 | 12 | 29 |
| 2 | edge (now) | 8 | 708 | 1041 | 177 | 12 | 23 |
| 3 | off | 8 | 614 | 1551 | 0 | 61 | 0 |
| 3 | out (was) | 7 | 477 | 3571 | 165 | 44 | 39 |
| 3 | out, more | 7 | 393 | 3954 | 224 | 34 | 55 |
| 3 | edge (now) | 8 | 511 | 3106 | 168 | 50 | 42 |
| 4 | off | 8 | 507 | 4003 | 0 | 74 | 0 |
| 4 | out (was) | 5 | 219 | 5265 | 155 | 53 | 75 |
| 4 | out, more | 6 | 185 | 5871 | 233 | 65 | 112 |
| 4 | edge (now) | 8 | 518 | 4905 | 159 | 48 | 31 |

Health is the sum over the living, of 800. The crystal drained the same in
every case (1869-1876 food, seed 3 about 1500; `raw/*.log`), so the guards'
work cost (22.5% for 3 of 8) did not show in 60 s on these crystals.

Paired over 4 seeds (sign test, two-sided p 0.125 at 4 of 4; none survive a
correction, 4 seeds being a look, not a census):
- **Out (was) vs off:** health lower 4 of 4 (summed 1781 vs 2462), 5 dead vs
  0; slime killed higher 4 of 4.
- **Edge (now) vs out (was):** health higher 4 of 4 (2220 vs 1781), 1 dead
  vs 5; guards bitten fewer 3 of 4.
- **Edge (now) vs off:** slime killed higher 4 of 4 (12465 vs 9133), workers
  bitten fewer 4 of 4 (156 vs 229 s), health lower 3 of 4 (2220 vs 2462),
  1 dead vs 0.

So posted at the edge the guards do the job the option names: they take
bites the workers would have taken and kill a third more slime, at a small
health cost, and the squad stays one body. Out in the film they spent
soldiers.

## Change

Guards stand at the work's edge on their bearing toward the slime (the
crystal's rim plus `RIM`, or 2 cells from a building), the nearest standable
ground if that spot is not. `CoverRules::out` and `CoverRules::thin` are
gone. Everything else is as it was: who goes on guard, the share, the work
boost, workers firing half as often, the I key and button. Same cost a
step (less: the 8-step search for a thin post is gone).

Removing the option instead was weighed: it is the owner's own ask (11-14
15:06Z) and, at the edge, it now gives a visible trade (more kills, fewer
bites on the workers, a little health), so it stays.

## Film

`page/guards-compare.mp4`: seed 4, drain by hand, 2-60 s at 0.5 s, the near
view: off, out (was), edge (now) side by side, counts over each. At 52 s
(`page/guards-52s.jpg`) the old guards have walked down into the film and 3
soldiers are dead; the edge squad has all 8.
