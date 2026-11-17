---
name: squad-toolkit
description: Squad toolkit thread (2026-11-12): CPU tools #104, carry #117, 32-seed confirm #141; GPU bursts + window tools page PR #159; next germ and gas on the GPU
metadata:
  type: project
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXJCxkP7EWH4x7TfirHGpHxJ. Owner: "All six together" (04:31Z), "Drop it, scorch is the crater" (04:46Z), fire carves a growing hole that doesn't refill (04:39Z).

History: six CPU tools in mould::kit (PR #104; report Reports/squad-toolkit-2026-11-12.md, page https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC, wiki/world/squad-toolkit.md). Round 2 carry rule (#117: carry_max 5, tool_weight 50). 32-seed confirm (#141, main 945e080, script Reports/squad-toolkit-2026-11-12/r2table.py): Holm over 152 pairs keeps loud>plague on Drifter 30/2, plague>loud on Sporer 30/2; random beats/ties every plan on Drifter and Trunk (Trunk still so on main with core pull, 24 seeds). Gas and cord never best.

PR #159 (20:10Z, head 42a1108): squad::Burst (dish cells, fine steps; rounds thread's fields) + World::burst; CPU squad::burst, GPU Engine::burst (strike row kcr/kcs at STRIKE_AT 800, hit pass atomicMax on binding 47 after #157). Kit generic over World; mine trigger x density / MINE_DENSITY 0.035. Window: E tools page (blast B, cord C, mine D; germ, gas greyed on GPU); larder restock 1 each, selected up to 5. 1070 twin booked in /mnt/project-files/scott-desktop/booking.md. Buildings thread's siren and the rounds thread (#147) will call World::burst.

Next: germ on GPU (GPU contagion lacks sick_upkeep; tech lead: fold sickness into the walker step; CPU germ pass ~9 ms/tick, check for early exit when few sick), gas on GPU, tools in the replay. Siege v2 vs one-body Colony and quiet vs campaign missions 2,3,7 are open ideas, but the owner's testing rule (17:26Z, CLAUDE.md) says build first and a census is no gate.
Related: [[larder-buildings]], [[mould-behaviours]], [[gun-rounds]].
Siege v2 inputs (one-body thread, 10:30Z): energy_flow=0.05 hunger_scent=8; an unheld gap refills in 2 s; a foodless hunk starves in 17-23 s (about 10 s with sharing); a hunk with a crystal in reach finds it in 2 s, so cut only where the far side has no crystal (examples/cutprobe.rs).
Integration rules: cross-thread tables come from `larderworld census`; final tools join larderworld as hands; GPU PRs that touch WGSL, Params or bindings run the 1070 twins (wiki/model/gpu-sandbox.md, *Twins on the 1070*) or merge saying "1070 twin pending".
