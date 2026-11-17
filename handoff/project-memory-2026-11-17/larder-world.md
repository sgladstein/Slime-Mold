---
name: larder-world
description: Larder World (2026-11-11 14:06Z): owner moved Larder into a persistent GPU world with energy crystals that run out, several species; roadmap W0-W5, thread split
metadata:
  type: project
  modified: 2026-11-11T14:11:08.231Z
---

Owner rulings 2026-11-11 14:01-14:09Z: Larder moves into the big GPU dish as a *world* (not a dish); food becomes energy crystals that run out and force expansion; several mould species; longer persistent world, maybe a campaign; "put everything into the world version" (no more small-dish Larder tuning). Also asked for a Camera and UI thread (cmsg_01PcwPdssrGxiXYvyCG8WaUX1gdY88SxLjc8wMJ8HqTx9G) and a Player AI thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXNPXVd4rSwedN6ATVfJwAg7) for automated playtests and later AI soldier teams (two soldier teams vs several moulds). 14:09Z: also explore a "race" start (player and mould both start small and expand at once) beside the "landing" (world mostly mould).

Roadmap doc https://claude.ai/artifact/1N2NHKefAzKxUMZcr3ktjJ, repo copy Reports/larder-world-roadmap-2026-11-11.md (PR #62). Phases: W0 growth/starvation on GPU (GPU thread, phase 4; big dish has fixed walker count), W1 world + crystals + terrain (Larder thread), W2 Larder in the world + camera/UI, W3 species (creature thread settings), W4 persistence, W5 campaign. Interface: everything through `Cmd`; camera thread's `world` module (Camera, View, Controls -> UiCommand); units carry a team id; ~1024 marks incl. squares for buildings; visibility-mask hook for possible fog.

**Why:** owner wants the real game as a native GPU world, not the small dish. **How to apply:** new Larder work goes to the world version; open questions (world size, how a world ends, allies vs rivals, fog, campaign carry-over) are in the doc. Related: [[mould-shooter]], [[gpu-plan]], [[jenson-looks]].

Owner 14:14Z: not worried about early-leader balance; wants mould *difficulty tiers* and progression (easy moulds early, complex ones later; e.g. fast expanders that hold crystals badly), with player-side walls, upgrades and weapons making strategy depend on the mould mix. Draft ladder in the doc: Drifter, Sprawler, Trunk, Hunter, Rival pair, Converter, Mixes. The 80% early-leader number is a per-mould difficulty stat, not a gate.

Owner 14:54Z (answering the post-review order card in words; closed as "CPU world first"): not a reason to scale back; nothing set in stone. Test crystals that run out vs not. Run-out crystals pair with the *race* (mould starts in one spot, enough resources for a long game; idle -> mould grows stronger and kills you). The *landing* (mould over the whole map) probably wants crystals that don't run out. Walls first, Sporer + Sleeper tiers, player-owned species: all agreed. CPU world for small tests is fine; GPU dev/tests on the GTX 1070 via Remote Control in parallel.

Owner 15:24Z: walls must not seal a crystal for good; the mould damages or grows over them with a big slowdown. Built in PR #64: wall_hp default 400 (caged mould breaks out in 20-30 s; passing tubes never break one), crystal_flow 0.01 (a crystal drains as a whole; fixes the GPU thread's "eats only its roads" starvation). Both wait on W0 growth for a real calibration. Page https://claude.ai/artifact/ChBBfKkuMYaggBvXCKrxtN.

Playable since 2026-11-12 01:46Z (W2c, PR #86): `cargo run --release --bin larder-world -- map=fork` (maps fork, nest, rim, flats, passes, noise). It runs the W2b rules on the GPU world with the world UI. `mouldgpu`'s Q squad is the UI thread's stand-in with no game rules. The owner played it at 23:53Z thinking it was the game. Soil runs on the CPU only, so far.
