---
name: gun-look
description: Gun look pass 11-15 (owner: guns a draw, "really satisfying"): muzzle flash by gun kind, recoil, gliding tracers, hit spray of killed slime; branch msj8l1
metadata:
  type: project
---

Owner 2026-11-15 08:30Z: "The gun graphics need work. They should really be one of the draws of the game. They need to be really satisfying." Gun look thread cmsg_01PcwPdssrGxiXYvyCG8WaUXNMGAy5sBZGmGbjVvVaNv7S, branch claude/project-thread-msj8l1, report Reports/gun-look-2026-11-15/.

Built: `Shape::Flash` muzzle flash (view.wgsl `flash_at`; jets by GunKind: rifle 3 prongs, carbine short/wide, long lance, scatter fan of 5), recoil `KICK`, tracers glide between steps (`App::between`; headless sub-step frames via `App::sub`), hit light waits for `Tracer::delay` (0.9 x f^2 peak; a volley's pellets share one hit's light: summed at 1.4 they lit the soldier white), bullet light fades in over 3 body radii out of the barrel.

The spray (`squad::Frame::spray`, `Rules::spray_secs` 0.3, `spray_reach` 5; `spray=0` off): walkers a shot kills fly back toward the shooter as flung spores with a DOOM mark (GPU spec bit 14, CPU `BigDish::doomed`), pace/flight by slot (`big::spray_pace`, `spray_timer`), lay nothing, die on landing. Kills identical (twin test cases; seed-1 films kill 1,177 at 30 s both). Flung walkers draw as streaks: `fly` writes flight-per-tick into walkers.w.

Rejected: live knock-back splash (-22% kills, barely visible; dead-ends.md). Screen shake not built (offered for bombs only). Alarm haze over bomb fights is the buildings-look thread's. Page https://claude.ai/artifact/MNvwqnxE9nqKr8hmfx26ce (source Reports/gun-look-2026-11-15/page/). Bench llvmpipe frame p50 62.1 -> 64.7 ms; 1070 untimed. Related: [[gun-rounds]], [[view-cost]].

Review follow-ups (11-15, PR #342 MERGED 13:25Z): `view::pick_lights` uploads only on-screen lights, brightest past MAX_LIGHTS 64. `Rules::spray_share` (default 1, `spray=SECS,SHARE`; GPU reads it from the spray row's last word >> 8) with blind A/B page https://claude.ai/artifact/KmajibGgb5iWouC1fqyFHF (all vs a quarter; key Reports/spray-share-ab-2026-11-15/raw/ab-key.txt; awaits owner pick). Params: surge row 944, spray row 960, PARAMS_BYTES 976. W18 (body saturation) measured false, not built (Reports/body-saturation-2026-11-15/; diag shader raw/make_sat.py). W27 square glow = alarm field, handed to slime-enemy thread.

Gun feel (11-15 evening, PR #369 MERGED 22:23Z): owner saw no difference in the share A/B (21:37Z) -> share stays 1; bold spray: droplets in a third speck screen (2*shalf, atomicMax layer|heat), rim, hot->cool, engine writes spray flight word < 0; spray 0.45 s / 7 cells. D6 shake (`shake=K`) and hit-stop (`hitstop=K`) on blasts only, OFF, `feel` in main.rs; `real=1` headless films. Page https://claude.ai/artifact/AE3vzcKx9Ajzn5wi7cAwUr; awaiting owner's pick on shake/hit-stop (would add an Options switch).
