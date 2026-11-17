# Slime enemy: handoff at the account move (2026-11-17)

The slime-enemy thread made the run's default slime (shape C) and its
behaviours. Model page: [`wiki/model/growth.md`](../wiki/model/growth.md);
C's keys and the reason for each are in the comment in
`src/mould/larder_world/shapes.rs`; tier keys in `src/mould/tiers.rs`.

## State

All of this thread's work is merged. No open PRs. The branch
`claude/project-thread-6rmmc6` holds only this file.

Merged behaviours, all on in C: shared food (`energy_flow` 0.05), withering
when cut off (`wither` 0.01), panic (`panic` 2), food-driven surges
(`surge` 1800), flinch (`flinch` 0.9), alarm haze, germ resistance
(`germ_resist` 10), specks (small breakaway creatures from ordinary slime;
`Reports/specks-2026-11-16/`). Bronze (thicken, condense in play) was removed
on the owner's call, 11-16. Specks' frame cost was trimmed and re-timed on
the 1070 (#388, #390): skipped dense passes clear on their first frame back
(test `the_dense_passes_start_clean_when_they_come_back` in
`src/bin/mouldgpu.rs`).

## Owner picks still pending

- **Haze strength**, blind A/B: page source `Reports/alarm-look-2026-11-15/page/`,
  key `Reports/alarm-look-2026-11-15/raw/ab-key.txt` (keep it off the page).
- **Flinch strength**, blind A/B: `Reports/flinch-2026-11-15/page/`, key
  `Reports/flinch-2026-11-15/raw/ab-key.txt`.
- **Specks playtest**: the owner plays a run (`play.cmd`) and says whether
  specks are too many, too few or right. Rate table in
  `Reports/specks-2026-11-16/README.md`.

The old claude.ai links to these pages will not open from the new account;
open the saved `index.html` in each `page/` folder.

## In progress when usage ran out: the Trunk's cords

The game designer's ask (11-17): on run planets the slime is a sheet, so a
thin cut starves under 2% of it; the Trunk tier (planet 3 of every run,
`LADDER[2]`) should grow real cords, **10% or more of the slime behind a
neck in most counts**, through its keys or wider crystal spacing. This is a
tier-identity fix, and it is the re-test condition for a Cut card
(`Reports/gun-cut-count-2026-11-17/`, PR #435).

Measure: `larder-world look=DIR run=SEED:3 hand=idle necks=60,120,180
at=60,120,180` (the cut count from #435; the neck-4 share). About 6 min a
run on a cloud CPU with four in parallel. Results so far (neck-4 % at 60 /
120 / 180 s):

| Keys added to the Trunk | Seed 1 | Seed 2 | Seed 3 | Walkers |
|---|---|---|---|---|
| none (today's Trunk) | 2.2 / 4.7 / 2.3 | 2.2 / 0.4 / 0.6 | 6.8 / 0.8 / 0.4 | 26-44k |
| `upkeep=0.0006` | 0 / 0 / 4.8 | 0 / 1.9 / 2.6 | | 9-15k |
| `sensor_offset=27 keep=0.97` | 2.2 / 0 / 0.6 | 1.8 / 1.6 / 2.7 | | 20-43k |
| `upkeep=0.0006 fed_scent=0` | 0 / 0 / 1.4 | 0 / 0 / 0 | | 6-12k |
| `upkeep=0.0006 food=0` | 0 / 0.9 / 0 | 0 / 0 / 0 | | 6-12k |

None reaches 10%. `upkeep=0.0006` does *look* like trunks (thick cords
between crystals, a blob on each), but the count stays low.

Why: every body sits on or touches a crystal with charge, and crystals are
barely eaten in 3 min (0.98 of their charge left at 60 s). A cut between two
crystals leaves both halves fed, so nothing starves. Only mass away from
any charged crystal counts: arms in transit to far crystals, or bodies on
crystals already drained. Thinner or wider bodies don't make such mass.

Next step, in order:

1. Try faster crystal eating on the Trunk so held crystals run dry while the
   body stays on them, fed through its cords (then a cut starves the dry
   piece). Check the idle smoke (idle must not win, the game must not end
   under a minute).
2. Or space the Trunk planet's crystals further apart (or fewer of them) so
   long arms are in transit: that is `src/mould/levelgen.rs`
   (`level(seed, depth)`), via a per-tier option.
3. Film the Trunk planet at 1, 2 and 3 min before and after (the designer
   said that is enough to judge), put it on a page saved under `Reports/`,
   and report the neck-4 counts.
