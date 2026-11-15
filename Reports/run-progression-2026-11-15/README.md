# Run and enemy progression: first pass (2026-11-15)

Thread "Build run and enemy progression", from the whole-game review
(`Reports/whole-game-review-2026-11-15/`, W3, W6, W9, W13, W14, W17, W30).
The owner pre-approved the items every reviewer agreed on (08:35Z: "Anything
where everyone agrees, just start building"). D3 (what rises from planet to
planet) and D7 (the 40-soldier brake) wait for the owner.

Page: https://claude.ai/artifact/3UZ3fiFNWsRYgpH67dp7x4; its source is
`index.html` here, with the stills in `stills/` and the raw t lines in `raw/`.

## What was built

1. **The stuck rule no longer ends a healthy run** (W17). `Game::count_stuck`
   counts a second as stuck only when, besides no shot, no food taken and the
   bank under a soldier's price: no soldier walked half a dish cell, no pump
   stands, nothing is being built, and not (shots are free and a crystal has
   charge). In squad mode shots are free, so a planet with charge left never
   ends stuck; the clock judges a squad that stands still. From `STUCK_WARN`
   (10 s) the goal line warns "Nothing is happening: walk, harvest, pump or
   fight, or this ends in N s" (`Tracker::progress_line`). Before: a run ended
   at 1:37 with four soldiers at full health and both pumps standing (the
   review's GD capture), and the chores thread's window test ended planet 6 in
   squad mode "Stuck" at 0:34 before the squad had met the slime.
   Tests: `a_walking_squad_or_a_standing_pump_is_not_stuck`,
   `stuck_warns_before_it_ends`; the old `stuck_seconds_count_a_squad_that_can_do_nothing`
   now prices shots (price 10), the case the rule is for.
2. **The shrink goal counts mass** (W30). `World::mass`: living walkers with a
   condensed or thickened walker counted twice. CPU: `BigDish`'s living plus
   its dense walkers. GPU: the `count` pass adds the enemy's condensed walkers
   to `wstat[8]`, read in the same packed readout (`WSTAT_BYTES` 32 -> 48, the
   mould grid moves from word 8 to 12), so no read-back. Checked against the
   headless count, which reads every slot back: planet 1 idle, 14,427 and
   17,611 at 30 and 60 s, the readout equal to the walker. Before, with
   thickening on in C (#338), a body that thickened read as shrinking.
3. **A play log** (W13): the window writes `larder-world-log-MAP-SEED.txt`
   (gitignored) beside the replay: a header (map, seed, mode, mission, tier),
   a line a game second (walkers, mass, soldiers, squads, bank, income,
   pumped, shots, kills, pumps, turrets, walls, other buildings, charge left,
   stuck seconds, the window's frame p50 and p95 over the second), each
   command at its game second, and the end. `cargo run --release --example
   playlog -- FILE` prints time to 10/20/30/40 soldiers, squads at once and the
   first split, buildings, shots and kills, the slime's peak, kills a second
   and the slime's mass minute by minute, frame times and commands by kind
   (`mould::playlog`, test `a_log_reads_back_what_it_wrote`). Frame cost: one
   short line appended a game second.

## Measured: idle by 5:00 (W9)

Run seed 1, planets 1-3, `hand=idle`, llvmpipe, main 40f34e48 plus this
branch (`smoke.sh` re-runs it; raw lines `raw/smoke-idle-d*.txt`). The
deciding measure, named before the run: at 5:00 the idle squad has lost
soldiers and the slime has more walkers than at 1:00.

| Planet (tier) | Soldiers 0:00 -> 5:00 | Walkers 1:00 -> 5:00 | Verdict |
|---|---|---|---|
| 1 (Drifter) | 4 -> 2 | 15,233 -> 25,226 | clearly losing |
| 2 (Sprawler) | 4 -> 1 | 25,276 -> 46,124 | clearly losing |
| 3 (Trunk) | 4 -> 3 | 46,434 (falls to 28,554 at 2:00) -> 41,173 | clearly losing |

No planet ends by 5:00 (the clock is 15:00 or more), so "idle loses" holds
as "clearly losing", the review's wording; it no longer holds that idle is
safe (the review's 17f2e5b8 capture had planet 1 at 1,515 walkers and the
squad untouched at 5:00, before #336 and #338).

## Measured: the squad's kills against the slime, by depth (W6, W14)

Run seed 1, planets 1-6 (the six rungs of `LADDER`), one seed each, no
census. Idle squad, and `hand=stand` (attack-move to the core: the squad
stops where the slime comes in range and fires) carrying the depth's average
rifle (`gun=rifle`, `GunKind::average_at`). Mass from the headless dense
count. `depths.sh` re-runs it (about 1 s of wall time a game second on
planets 1-4, 3 s on planet 6). Planet 7 was cut short (about 8 s a game
second here), planet 8 not run.

| Planet | Tier | Mass at landing | Idle mass 1:00 | Idle mass 2:00 | Squad's mass 2:00 | Kills 0-2:00 | Kills/s | Soldiers at 2:00 (squad) |
|---|---|---|---|---|---|---|---|---|
| 1 | Drifter | 13,119 | 17,611 | 20,774 | 20,789 | 1,941 | 16.2 | 1 of 4 |
| 2 | Sprawler | 23,076 | 26,848 | 35,577 | 35,326 | 1,633 | 13.6 | 3 |
| 3 | Trunk | 43,304 | 48,527 | 32,725 | 32,260 | 1,356 | 11.3 | 3 |
| 4 | Swarmer | 53,574 | 62,205 | 51,055 | 50,173 | 3,790 | 31.6 | 4 |
| 5 | Sporer | 88,105 | 84,978 | 73,937 | 72,194 | 818 | 6.8 | 4 |
| 6 | Sleeper | 144,317 | 142,833 | 133,833 | 140,044 | 4,781 | 39.8 | 4 |

**Reading.** A four-soldier squad with the depth's average rifle kills 7 to
40 walkers a second. The slime's own swing over the same minute is ten times
that: planet 1 grows 53 walkers a second from 1:00 to 2:00, planet 2 145 a
second, and planets 3-6 shed thousands a minute on their own. So at 2:00 the
squad's slime is within 5% of the idle one on every planet (-2.4% to +4.6%);
the kills don't show in the mass. At planet 1's rate, cutting 98% of 20,774
would take about 21 minutes with no regrowth (the clock is 15:00); planet 6
about 55 minutes (clock 31:30). Mass at landing grows 11x from planet 1 to 6
while the average rifle's kill rate grows about 2.5x (and depends as much on
where the squad stands). This is the review's W6 "damage threshold" seen
from the other side: the planet is won by income and army size (the owner's
40-soldier rush), not by the gun, and deeper guns don't change that.

**What it suggests, held for the owner (D3, D7).** Whatever rises per planet
should be pressure the player feels (the review's per-tier "does to you"),
and the guns' growth and the slime's growth need one curve between them.
The slime thread's `run::enemy_keys(depth)` (surges, thickening by depth) is
the first such curve; this table is the baseline to measure it against.

## Stills

One still a tier on its own planet (`stills/idle-dD-120s-all.jpg`, whole map
at 2:00, idle) and the squad at work (`stills/stand-dD-120s-near.jpg`).
Planet 1 idle at 5:00: `stills/idle-d1-300s-all.jpg`.

## Not done here

- Planet 7 and 8 rows (slow on llvmpipe; a 1070 job if wanted).
- Kills from a bought loadout (cards, rolled guns); only the average rifle.
- D3 and D7: owner decisions.
