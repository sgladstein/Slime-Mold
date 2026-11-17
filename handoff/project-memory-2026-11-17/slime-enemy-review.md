---
name: slime-enemy-review
description: Second slime-enemy thread (11-15/16): surges, depth, haze, flinch merged (A/Bs await owner); 11-16 bronze (thicken + condense) removed, specks from ordinary slime
metadata:
  type: project
  modified: 2026-11-16T10:00:00.000Z
---

Thread "Improve the slime enemy" (cmsg_01PcwPdssrGxiXYvyCG8WaUXVjzMVR9jPQsKucgP9GF9Hs, branch claude/project-thread-6rmmc6), started 2026-11-15 08:03Z by the owner: "a deep review of the current state and then ask me questions". Review page https://claude.ai/artifact/3wF8KBUfBUp7oBTntvPBJi, Reports/slime-review-2026-11-15/ (#333).

**Owner's answers (11-15 08:30-08:34Z):** won by rushing the biggest crystal to 40 soldiers then attacking ("not what I want to play"); slimes sat at home, no rush, no need to defend; wants challenge "especially when your guns get stronger"; slime should "keep getting stronger by holding onto its crystals and getting more of them"; planet slime size "should vary and be more later on".

**Thickening (#338)** made C stronger by holding crystals (mass up to 2x slots); removed from C on 11-16 (below). Reports/thicken-2026-11-15/.

**Surges #343, depth #347, food-driven #350 (merged; 1070 p95 5.35 vs 5.34):** calm walkers within panic_r of a unit creep then lunge; `Game::time_surges` meter = crystal food eaten, surge at `Rules::surge_food` after `surge_min` 10 s or at `surge_every` (C 60 s). Run: surge_food 200..75 x walkers, longest wait 80..30 s, lunge 1.0-1.8 s, thicken 2-1.3. Reports/surge-2026-11-15/.

**Alarm haze #354 (merged; 1070 base 6.22->6.38 ms):** W27 square glow fixed (`alarm_at` weights open cells). W46 `ALARM_LOOK` (0.25, 0.25), key `alarm_look=G[,S]`. Blind A/B https://claude.ai/artifact/XwB5KRtcCniXSBJVipGSyp (A slime only, B tenth, C quarter, D full) awaits owner's letter.

**Flinch (PR #356 merged 15:33Z; 1070: frame unchanged, step +0.05 ms; docs PR #357):** `BigParams::flinch`/`Settings::flinch` key `flinch`, folded into alarm at world build (`big::flinch_alarm`) only for a slime with no alarm of its own (Swarmer/Sporer/Sleeper/Grudge keep theirs). Shape C 0.9 (sandbox, planets 1-3), menu Flinch. Planet 1 seeds 1-5: no steady change in who wins. Blind A/B https://claude.ai/artifact/NiqQovqWtGqbzBX9YsM1bS (key Reports/flinch-2026-11-15/raw/ab-key.txt: A 0.95, B 0, C 0.8, D 0.9) awaits owner's letter.

**D2 breakaway (#370):** condense+roam on planets 2-8; replaced 11-16. Reports/breakaway-run-2026-11-15/.

**Owner 11-16 04:58Z (via mini-boss thread): "Remove the bronze, keep the specks (specks and mini-boss are separate but both happen. Specks more often early game)."** This thread: thicken out of shape C, condense out of run + menu, breakaway (`roam`) now from ordinary walkers (`specks` mode, roam>0 && condense==0, both engines, twin dish "specks"); a speck gives up after `SPECK_TRIP` 600 steps; `run::enemy_keys` ROAM/ROAM_SHARE curve highest early. Mini-boss thread (claude/project-thread-g8thj2) owns lumps, dense bits 25-27: keep LUMP_BITS when writing ripeness or landing; second to merge rebases. Loss to flag: thickening's "stronger by holding crystals" is gone; offer a replacement. Reports/specks-2026-11-16/, page https://claude.ai/artifact/PjaBjAmY1FC2qzZb7fDbzQ. PR #381 merged 11-16 06:40Z (befdb5d0); mini-boss #385 rebases over it; chance on (step+slot)%roam, crowd counted only then; share 0.05 planets 1-2 -> 0.02 core; roam_share default 0.05; +7%/+10% mass at 5:00 (planets 2/1). 1070: step +0.05/+0.07 ms planets 1/2; core +0.56 -> +0.30 after #388 (masscount only with condense, dclear/dcount with condense|thicken|lumps). #390: come-back twin + CPU dense_cells gated on condense|thicken.

**How to apply:** the owner has answered (above), so build: surplus food into strength, surges, depth progression; run-path fixes belong to the roguelike thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXTX9SyKMGKZP4ciXzGUAHZu). Measure run slimes via the menu path, not bare `run=`.

Related: [[slime-enemy]], [[roguelike-run]], [[condense]], [[pump-economy]].
