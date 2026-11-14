# Take-specialty matrix (2026-11-14)

This is the designer's follow-up to the [tier matrix](plan-matrix-tiers-2026-11-14.md).
Each plan is now take plus one specialty, so a plan that is "best somewhere"
says which specialty wins on which tier. It is the last run before the
owner's playtest.

Code: `b550625`, on main `30bf05e`. The specialties:
- `take_fort`: take plus 8 walls.
- `take_boom`: take from 10:00, once 36 soldiers stand.
- `take_claim`: take with 16 turrets and 4 soldiers kept farming.
- `take_strike`: take, then the core once 2 crystals are taken (new knob
  `core_after`).

The world is census-size C, race map, cap 900 s, seeds 301-308, with 8 hands
on the plain column and 7 tier columns. It took 58 min on 4 threads.
Measures were named before the run:
[raw/MEASURES.txt](plan-matrix-take-2026-11-14/raw/MEASURES.txt). Tables:
[matrix.md](plan-matrix-take-2026-11-14/matrix.md), from
[tab.py](plan-matrix-take-2026-11-14/tab.py).

**The engine changed since the tier matrix.** #227 (art review) added elbow
room (`Rules::elbow` 1.6), which spreads a squad instead of stacking it on
one point. Plain take is much stronger now. It wipes Swarmer and Grudge 8/8
(2/8 and 4/8 before), and its plain C median rose from +0.20 to +0.30. This
is likely because of elbow room, but that is not isolated. Compare within
this run, not with #234.

## Result

| column | best by median | take_claim vs take | wipes: take_claim / take / take_fort | strike vs take |
|---|---|---|---|---|
| plain C | take_claim +2.14 | 8-0 | 6 / 1 / 2 | 0-8 |
| Drifter | take_claim +1.89 | 8-0 | 5 / 2 / 0 | 0-8 |
| Sprawler | take_claim +0.38 | 6-2 | 3 / 1 / 0 | 0-8 |
| Trunk | take_claim +2.04 | 6-2 | 5 / 3 / 2 | 0-8 |
| Sporer | take_claim +2.23 | 6-2 | 7 / 5 / 3 | 0-8 |
| Sleeper | take +0.49 (take_claim ties) | 3-2 | 0 / 0 / 0 | 0-8 |
| Swarmer | take_claim +2.28 | 5-3 | 8 / 8 / 8 | 0-8 |
| Grudge | take_claim +2.27 | 6-2 | 8 / 8 / 8 | 0-8 |

Against the measures named before the run:
- **Plain C wiped on at least 4 of 8: passes.** Take_claim wipes it on 6 of
  8, while it failed on every plan in #234.
- **Idle and random lose everywhere: pass.**
- **Strike loses: passes.** It now loses to take 0-8 on every column.
- **Specialties matter (one beats take on at least 6 of 8, on at least 2
  columns): passes.** Take_claim does so on 6 columns.
- **No one specialty best on more than half the columns: fails.** Take_claim
  is the best specialty on all 8.
- **Holm** (32 specialty-vs-take tests): none survive. The smallest p is
  0.0078, the floor for 8 seeds.

The other specialties lose to plain take nearly everywhere.
- **take_boom** (late start) loses 0-8 or 1-7 on every column but Sleeper.
- **take_strike** (to the core after 2) loses 0-8 or 1-7 on 7 columns. The
  core walk still throws the squad away.
- **take_fort** ties take on Swarmer and Grudge and loses elsewhere (1-6 to
  4-3).

## What it says

1. **Turrets do the holding, and soldiers do the cutting.** Take_claim puts
   36 of 40 soldiers on the cut and leaves the taken crystals to 16 turrets.
   That is the first plan that beats plain C.
2. **The book has one good line, not a set of counters.** Every specialty
   that adds something other than more turrets and more cutters is worse.
   Under score v2 this AI finds no tier where walls, a later start or a core
   strike pays.
3. **Plain C is still the stiffest column except Sprawler**, where take_claim
   wipes only 3/8. As the designer asked, this is a note for the tiers ladder
   with no retune until the owner has played the missions.

As agreed with the designer, this is the last census until the owner's
playtest.
