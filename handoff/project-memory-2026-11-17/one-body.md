---
name: one-body
description: One-body thread (2026-11-12): cohesion/hunger rules merged CPU #103 (off); GPU port #108 (binding 46, BODY_AT 848); lead energy_flow 0.05 + hunger_scent 8; page awaits owner A/B
metadata:
  type: project
  modified: 2026-11-12T17:15:00.000Z
---

Thread "Make the mould one cohesive body" (cmsg_01PcwPdssrGxiXYvyCG8WaUXQsjsMSgmPdKaKPdMkvKWhg), from the owner's 04:13Z playtest.

- New BigParams (CPU, off): hunger_scent/hunger_at, body_pull/body_r/body_lone, tether/tether_hold, lone_upkeep/lone_r. Probe: examples/cutprobe.rs (ring=6 cutw=4 wound= half= unfed=; CUTPROBE_FILM=1 for frames). Report Reports/one-body-2026-11-12.md.
- Lead: energy_flow 0.05 + hunger_scent 8 = one body on more seeds, 2.5x size (8-seed census, 300 s). tether -> separate hunks; lone_upkeep 5+ -> one part but no spread.
- Cuts refill in 2 s; only a held gap makes a cut-off piece (one lump, starves ~20 s).
- Page https://claude.ai/artifact/E9MSpcgBj7F8jZX6i6LA5q asks owner: which looks like one creature (blind A/B key in ab-key.md).
- GPU (#108): esum/egain passes, binding 46; 1070 +0.23 ms/frame.
- Larder census (10:15Z, PR #121): H keeps farm/wall gate (16-0, 15-1) but rush stops winning (6-10, 9-7), every hand lower, mould 2.8-2.9x. Recorded: H OFF by default pending owner's A/B; next candidate energy_flow alone, re-run after the Larder crater PR.
- Matched size (13:00Z, PR #127): crystal_eat 0.004 holds H at today's size but random starves the whole mould 9/16 (cliff to 0.0045). Hunger sets size (~3x at any flow); energy_flow 0.01 alone = today's scores, no look gain. Proposed: off by default, H as a 'Colony' tier, next try distance-capped sharing on the crater baseline.
- Owner 14:36Z: both look like 'individual specks', wants films/playtest. Game-look films via `larder-world look=DIR hand=idle at=2,4,.. screen=1280x800 KEYS` (llvmpipe works). lone_step (CPU+GPU, LONE_AT 880, PARAMS 896, PR #134, off): thicker tubes, hunk holds (0.72 vs 0.46) but frontier still specks, spread -25%. Density start, flow_loss 0.3, body_pull on hunks: no.
- Owner 16:11-16:28Z: specks = density (15-20% of cells smooth; films on page). Jones mesh forms at 4 s then coarsens to strands by 30 s. Owner asked for a design doc 'The ideal slime enemy' (Claude Doc https://claude.ai/artifact/6Jb8uKgtnHrgP7vy7cqKyZ, goals L1-L6/B1-B6/P1-P5 + measures) then a deep review: 5 agents (look/density, network, pieces, play, backend) answering 'is Jones a good backend?'; brief in scratchpad review/brief.md. walker_mass factor agreed with Larder (off till reset).
- Ownership (integration thread): closed ground is the toolkit's crater field / Larder's hole under fire; GPU rows queue after the toolkit's crater rows. Sound/graphics want a piece id once pieces exist.
- Review done 17:00Z (5 agents, all: keep Jones, extend). Big find: GPU Engine::made drops one-walker-a-cell when max(walkers,capacity) > agar/2; race world capacity = 10x walkers, so every >~7k-walker run (all 25k films, 1070 bench A) had no crowd limit -> ropes. With capacity=50000 the 25k mould is a continuous sheet / lasting mesh, no specks (page v10). Doc tab 'How we get there' has 6 tests. Owner 17:00Z (testing-review thread): 'We were developing bad looking slimes because more runners killed the player too easily (instead of rebalancing)' -> look first, rebalance numbers after; census only catches breakage.
- 17:35Z questions page https://claude.ai/artifact/7LyE6uCco3xQNLukrcJFBz (PR #154): king rule films + 5 front shapes. Lead look: 25k walkers capacity=50000 sensor_angle=22.5 sensor_offset=5 (sheet->mesh->trunks, continuous). Owner asked: drop king rule? which front shape? Waiting on answers.

**Why:** owner bar: the mould is integral and reads as one organism. **How to apply:** don't turn defaults on without the owner's answer; send key changes to the tiers, body-forms, integration, Larder and graphics threads. See [[larder-world-playtest]], [[growth]].
