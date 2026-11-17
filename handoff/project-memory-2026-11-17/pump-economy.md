---
name: pump-economy
description: Pumps are the main income (owner 11-15 03:07Z); PR #322: hand drain x0.2, pump draw by crystal size, upgrades, rig look, income-only flow readout
metadata:
  type: project
  modified: 2026-11-15T03:41:04.846Z
---

Owner (Scotttt, 2026-11-15 03:07Z): hand draining was too easy (40 soldiers in 30 s); pumps become the main income. Pumps snap to and centre on the crystal and look like a structure built round it, are upgradable, pay more on bigger crystals, and are never locked in the run. The flow readout shows income only and never goes negative; costs come straight out of the bank.

PR #322 (branch claude/project-thread-y6524a, merged 11-15 04:20Z; awaits owner playtest):
- `Rules::hand` 0.2: drain share with no larder.
- `BuildRules::pump_share` 1/150 of the crystal's start food per second (floor `pump_rate` 5).
- Upgrades `pump_up_cost` 400 then x level; 1.5x draw and 1.25x yield per level, up to `PUMP_LEVELS` 3.
- `Game::income` drives the HUD rate.
- `Marks::pump_rig` / view.wgsl `pump_rig` (building kind 6).
- Run shop item "pump" is now "pump+" (Pump power).

Smoke (Fork seed 1, greedy opening): 27 soldiers at 0:30 before, 11 after.

Pumps never drain (owner 11-15 17:14Z, PR #358): pump pays while crystal >1% charge (`Game::pump_flows`, `PUMP_DRY`), takes no dish charge; only slime eating the crystal stops it. Hand drain still empties crystals.

Open lever: guarding a big crystal still pays a lot (1/120 of charge left per second).

Report: Reports/pump-economy-2026-11-15/. Page: https://claude.ai/artifact/6jYuCSBxzV1XsXNbTvYfJ4. Roguelike thread was told. See [[larder-buildings]], [[roguelike-run]].
