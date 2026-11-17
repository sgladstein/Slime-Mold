# Handoff: run and enemy progression (2026-11-17)

Area: the roguelike run's progression, i.e. what each planet's slime does to
you, the win goal, the stuck rule, and the play log. ROADMAP row **N1**, with
N3 held.

## State (all merged unless noted)

- **#351**: `count_stuck` no longer ends a healthy run. The shrink goal counts
  the slime's mass (`World::mass`). The window writes a play log
  (`larder-world-log-MAP-SEED.txt`), which `examples/playlog.rs` reads. Report:
  `Reports/run-progression-2026-11-15/README.md`.
- **#359**: the win goal is 1% of peak mass (`WIN_SHARE` 0.01; owner 11-15).
- **#372**: `run::does_to_you(tier)` gives each LADDER tier one thing it does
  to you (owner's D3, "test it out"), shown on the planet card and in the
  landing alert. Report and films: `Reports/run-progression-2026-11-15/does-to-you.md`
  and `does.html`. The same PR took the outpost, lure and siren out of the run
  shop (D10).
- **#434** (another thread): the Sprawler is fire-shy, and its planet card
  says "Bring Tinder".
- **#436, OPEN** (branch `claude/project-thread-uwavsa`): `run::bring(tier)`.
  When the planet ahead asks for a card you don't own, that card takes one of
  the shop's card slots. Before this, Tinder was in 23 of 200 planet-2 shops.
  Test: `the_shop_before_the_sprawler_offers_tinder`. CI was green. Next step:
  merge it through the `queue` label (#434 has merged).

## Pitfalls

- `wake_alarm` and `spore_alarm` are thresholds: a higher value means less
  waking or bursting.
- A louder `alarm_weight` pulls the slime to the wounds, not the squad.

## Waiting on the owner

- The sizes of the `does_to_you` rows (one number each), to be judged in play.
- D7, a brake on the 40-soldier rush: "play first, no changes yet".
