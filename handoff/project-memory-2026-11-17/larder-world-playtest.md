---
name: larder-world-playtest
description: Owner's Larder World playtests and fixes: free shots #106, crater #113, core defence #126, wade+cone 3b, economy #170, sink #181, wading counts whole walkers #197
metadata:
  type: project
  modified: 2026-11-12T13:31:13.933Z
---

Owner (Scotttt) played `larder-world` on 2026-11-12, relayed 04:13Z: 1) not one cohesive organism (one body thread); 2) shooting "visually it didn't impact much"; 3) "the slime mostly ignored my soldiers unless they walked into the slime"; 4) "energy required for shooting isn't fun".

Shots bar (04:38-04:39Z): drawn hits rejected ("Both of those look like stamps"); "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at".

PR #106 (merged): free shots (Rules::price 0), scent (smell 0.5, smell_r 2). The flee rule made the hole but let random win by holding the core (12/16 wipes); now opt-in (`flee=W`), dead-end written.

PR 2 = #113 crater (`Rules::crater` 10 s, walker stop, GPU binding 47), merged 12:13Z. #122 (BuildRules::raise 8) is a census reset: quote a gate's baseline.

PR 3 = #126 core defence (opened 13:30Z): `Rules::core_pull` 8 (a soldier within 2 core_r lays lure-strength smell over lure_r, so the body comes for it; through UnitIn, both engines), `Core{kind,secs,pull}`, `CoreKind::Fixed|Headless`, `Game::with_tier`, `Tier::core()` (None; the tiers thread fills Sprawler Headless, Trunk Regrow, Drifter Migrate, Grudge pull). Census (baseline 089167b, pull 0): random wipes 18/64 -> 5/64; every plan over random after Holm on 5-36 and 201-232 (none held on 201-232 at pull 0). Merged 14:52Z (9f5e4a4); Fork/Rim no harm; twin gap is chaos (ignored check). Unmet: strike beats farm 22-10/21-10 and wall 26-6 (food doesn't bind) -> PR 4 plus tier cores.

PR 3b (owner 14:41-14:43Z, after video clips of the looks): look 3 = cone crater is the pick (race_params scorch_cone, full 150, keep 0; race_settings carries them); soldiers slowed in thick slime = `Rules::wade` 6 (mine); no standing on crystals = controls thread. Commit 90ca407; gate census arms wade6 / wade0 / flat on 4 race files (runners 1-4). Look-mode `hand=stand`, `centre=X,Y`. Clips page https://claude.ai/artifact/HzgDT3XPLX59Ba621tVH2H.

PR 4 economy (#170): rations + overrun. Gardeners' Feed (#119) is already a ~33 food/s sink; don't drain twice.

Census: `larderworld census hands=... seeds=... cap=900 threads=4`; runners send raw files by git, never retyped.

**Why:** the owner's bar is that the mould is integral to play and every tool's effect is visible. **How to apply:** judge by what the owner sees, then the census gate (plans beat random and idle on both blocks). See [[mould-behaviours]], [[squad-toolkit]], [[tech-lead-review]].

**Shots sink in, PR #181, merged 11-13 00:05Z at c00227a (the owner's 23:14Z "Yes")**: a hit's blast is centred on the last slime within `Rules::sink` (1) strike radii on along the shot's line, not on the first slime cell (`squad::sink`, GPU `Params::shot_sink` in the spare word after the tend row). Shape C: standing still on free fire 12.6 -> 20.3 kills a shot, walking in 42.0 -> 47.9. First built as `bite`; renamed because "bite" in this game means the mould's damage to soldiers (`Rules::drain`). Page https://claude.ai/artifact/4iJTF66gpSNoJxnLgUtyiG; report Reports/larder-shot-sink-2026-11-12.md; 1070 twin booked behind the timing jobs. Still open: the owner's crater-length pick (`crater=2` vs `scorch_keep=0.5`).

Later work (11-13 #197 wading whole, #198 aims that see, the mass audit): [[larder-aim-and-mass]].
