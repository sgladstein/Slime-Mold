---
name: slime-enemy
description: Slime-enemy thread: #164 headless; #174 C default; #183 C shares food; #196 C withers (0.01) when cut off; cut-off part panics (panic 2, on in C; #200 merged; #202 sliver check, no size floor; #203 creep; #204 dash)
metadata:
  type: project
  modified: 2026-11-13T19:01:13.886Z
---

Thread "Design the ideal slime enemy" (cmsg_01PcwPdssrGxiXYvyCG8WaUXQsjsMSgmPdKaKPdMkvKWhg, branch claude/project-thread-0hct5s). Full detail lives in Reports/fed-mould-2026-11-13/README.md and wiki/model/growth.md; this note keeps the owner's words and what to do next.

**Owner rulings.**
- 11-12 20:21Z "remove the king rule ... Headless" (#164): no feature may rely on taking a core.
- 11-12 22:16-22:22Z: C is the best shape and the default (#174). "walking directly into the enemy should be very deadly" (Rules::drain 2 a walker, whole). Cut-the-network rule parked until feeding is visible.
- 11-13 02:39Z: shared food, "Definitely film 1" (#183, energy_flow 0.05 in C keys).
- 11-13 15:51-15:53Z: withering "B is the best but still too fast" then "double it and call it good. I will have to playtest" (#196, wither 0.01 in C keys).
- 11-13 15:57Z inspiration ("if possible"): a cut-off part should "act more like a scared animal. Either run away back to the main body or go right for the soliders. It should be more reckless", NO stronger attack. Built as `panic` 2 in C keys (#200).
- 11-14 01:08Z on the C4 stalemate (#213): "I have easily beat the game in all my playtests so I am not too concerned about the slime winning your automated test". So don't make C easier for the AI: collapse threshold (Rules::broken 0.03) and "size follows food held" parked until a playtest says how. Finding (examples/foodprobe.rs, #215 merged): a crystal feeds ~9,400 walkers and C eats 3x upkeep at its cap, so on 1 crystal of 23 it holds 50k for 2 min, 10k by 6 min; 4 kept: 47k at 6 min.

**Panic, as merged.** A walker whose food news is older than the lag (5 s) leans at its own pace for 1.5 s (the tell), then runs at 2x toward the nearest squad unit within panic_r 20 dish cells, else the nearest crystal with >2% left. It calms when it hears news. GPU Params 460 panic_r2, 484 panic; twins exact. Films page https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP (v7, labelled, 4 scenes; source Reports/fed-mould-2026-11-13/panic-page/). Staging: `larder-world look=DIR hand=farm seed=1 cut=150,365,240,24 cutsquad=365,256,150,4` (charge: shot down, one soldier lost), ring r32 with `cutsquad=365,264,150,4` (last stand: a 163-walker piece kills all 4 in 2 s). Check that the binary has current main before filming (a pre-#198 build gave a false "squad falls to 1").

**Tech lead check, 11-13 (#202, 202819a).** `larderworld slivers=1`: scared walkers are mostly alone and deal 4-6% of the squad's bites; losses 26/24 vs 25/25, so there is no size floor.

**Designer's wind-up, 11-13.** #203 (abb8535) adds `panic_creep` 0.5 (GPU word 488): the lean creeps at half pace; a walker going straight moves 15 dish cells/s against a soldier's 10. Then `panic_dash` 15 steps (GPU word 492, u32): a charge at a unit bolts 0.5 s and then creeps again. Retreat probe `examples/retreatprobe.rs`: pulling back 0.5 s late at 24 cells is clear on 5/6 seeds with the dash, against 0/6 without; standing is still caught on 5/6. Merged #204 (c56aadc) 11-13 ~20:08Z; films are scene 4 of the page (v7). Smoke unchanged (idle -0.333 4/4).

**Reach follows the guns, 11-14.** Designer's review of #249 (range 15 -> 24): #264 keeps panic_r at gun_range() + Rules::panic_margin 5 once a game second (World::set_panic_r, both engines); #267 reads gun_range so Range cards move it (29 / 41 / 53). Smoke unchanged.

**Open.**
- Owner playtest of withering plus panic.
- Next only if play asks: slower lean, shorter panic_r, size floor.
- 1070 frame cost of the news passes once timing is allowed.
- HANDOFF open decision 9 (flow_loss) stays off.

Related: [[one-body]], [[larder-world-merge-order]], [[larder-world-playtest]].
