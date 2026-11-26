# Does any plan beat the plain rifle? (review step B, 2026-11-26)

Step B of the owner's order (11-25 21:35Z: "a then c ... then b"), from the
review of 11-25 (`Reports/review-2026-11-25/`, finding 2: the nightly run
bot wins planets 1 to 3 with a plain rifle and no buildings).

**Question, named before the run:** does any plan beat the plain rifle on
some planet and lose to it on another, so the plans counter the planets?
**Measure:** planet-maps won, and time to win, against `rifle` on the same
planet-map; whether the best plan differs between planets.

**How:** Arms run 45736141018 (PR #857), main at 95049373 (before #858),
eight shards of 35 minutes on GitHub's runners with lavapipe. Planets 2, 4
and 6 on the owner's three maps (A 1795038793, B 1795106014, C 1795119328),
the planets before taken as won and shopped by the plan
(`league=DEPTH:SEED:720`), 12 game minutes each. **The origin off**
(`set=origin=0`): his 11-26 00:08Z ruling makes hearts no longer the win,
so a planet is won by wiping the slime out; #858 (crystal hearts as a
mini-boss) landed after this batch, so these games have no hearts at all.
41 of 45 games played; four planet 6 games were skipped by the shard
budget. Raw rows: [arms-rows.txt](arms-rows.txt).

| Plan | What it does | P2 (A, B, C) | P4 (A, B, C) | P6 (A, B, C) | Won |
|---|---|---|---|---|---|
| rifle | the run bot: drain by hand, buy soldiers, hunt; no cards | **W 3:28, W 5:58, W 4:08** | W 10:32, L, L | open, open, open | 4 of 9 |
| econ | pumps first | W 4:26, L, W 8:12 | L, open, L | open, open, skipped | 2 of 8 |
| turtle | pumps, turrets, walls, a lure | W 3:18, W 4:01, L | open, L, L | open, open, skipped | 2 of 8 |
| lure | pumps, turrets, a lure | W 5:31, L, L | **W 11:06, W 7:55, W 9:30** | open, skipped, open | 4 of 8 |
| cards | the run bot buying gun cards | L, W 4:04, W 4:48 | W 7:55, L, W 7:20 | open, skipped, L | 4 of 8 |

"open" is no verdict by 12:00; "L" is the squad wiped.

## What it says

1. **Yes, the best plan changes with the planet.** On planet 2 (the
   Sprawler) the plain rifle is the only plan that wins all three maps;
   every building plan loses one. On planet 4 (the Swarmer) the lure plan
   (pumps, turrets and a lure drawing the slime into them) wins all three,
   while the rifle wins one and pumps or the full turtle win none. That is
   the counter shape the old demo bar asked for, on one planet pair.
2. **It is not significant.** Three maps a planet: the best possible sign
   test is 3 of 3 (p = 0.25). The planet 4 result is a lead to film and
   rerun on more maps, not a finding.
3. **Planet 6 can't be wiped out in 12 minutes by any plan.** With hearts
   off and the win at the last walker, no game on planet 6 finished. The
   rifle held 62 to 99 soldiers and left 3,858 to 48,002 walkers; the
   building plans let the slime grow to 37,000 to 189,000. On the new win
   rule (#858), late planets may run long or stall: this is the first thing
   the curve (exit test 3) must check.
4. **On planet 6 the building plans barely dent the slime.** The lure and
   turtle plans on map A ended above the slime they landed against (177,396
   and 162,691 walkers against 142,730), and econ on map B too (106,211
   against 95,491), while the rifle cut it to 3,858 to 48,002. On planet 4
   the lure plan's turrets were the ones that finished the job; the full
   turtle, which waits behind walls, did not. Building is a counter when it
   pulls the slime into the guns, not when it waits.
5. **Close calls.** The rifle lost planet 4 on map C with 186 walkers left,
   so its 1 of 3 there overstates the gap a little.

## Next

- Film planet 4, map B, lure against rifle, at zoom 6 to show the counter.
- Rerun planets 4 and 6 on six more maps with #858's crystal hearts, and
  with the cap at 20 game minutes, to see whether the late planets end.
