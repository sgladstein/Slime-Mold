---
name: larder-buildings
description: Larder world buildings (2026-11-12): turret, lure, pump, outpost, wall built (PR #112); guns at crystals win because food doesn't bind; siren merged #160
metadata:
  type: project
  modified: 2026-11-12T07:32:14.292Z
---

Owner (Scotttt, 2026-11-12 04:23Z) started the buildings thread. Owner picked "All six in one go" (04:45Z), then dropped the Scorcher (04:46Z: "scorch is the crater"). Soldiers build over time (controls thread uses `Game::raise`).

Built, PR #112 (2026-11-12 07:32Z): `mould::larder_world::buildings`. Turret, lure (8x over 6 cells, toggle), pump, outpost (`Game::drop_offs`) and wall are still units in `squad::Frame`, so the CPU/GPU twin holds by construction. `squad::UNITS_MAX` 96. Window build page on Q. Hands in examples/larderworld.rs: ring, turtle, killzone, guard, pumps, army; random also builds.

Finding: turtle and guard (turrets at crystals) top the race world on every tier, and army (the same food spent on soldiers) ties them. Hands end with 20-35k food unspent, so this is an economy problem. The Larder thread takes it as its PR 3, with these hands in its gate. Nest map: ring best, killzone wiped. Fork: no hand ahead.

Re-run on 9f5e4a4 (craters, 8 s builds, core defence), PR #152 merged 17:31Z: army (soldiers) best on race (22/32), ring/guard best on nest (army below idle): best plan changes with map. Owner rule 17:26Z (CLAUDE.md #150): build first, balance later; no census gates for features; runs over ~1 h need owner OK. Siren merged #160 (2026-11-13 01:21Z, 46b01be): calls with the alarm burst every 4 s; Swarmer floods to it 8/8 seeds, plain mould ignores it. Command id SIREN 27.

**Why:** owner bar: buildings use the mould's behaviours, and several strategies must counter each other. **How to apply:** for building balance, look at the economy first (army control). Report: Reports/larder-buildings-2026-11-12.md. See [[larder-world-playtest]], [[soldier-controls]].
