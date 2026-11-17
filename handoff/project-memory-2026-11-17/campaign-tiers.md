---
name: campaign-tiers
description: Campaign mould tiers thread (2026-11-12): PR #115 tiers, PR #133 (Drifter drifts, Sprawler headless, Grudge pull 16, Wary removed) merged a56f644, #180 Drifter hunger 16 merged a1a7156; next scar GPU port
metadata:
  type: project
  modified: 2026-11-12T07:55:10.666Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUX8PXYHJ6DnXY2AcpsnyCzwM, branch claude/project-thread-dum4s1, PR #115 merged 2026-11-12 10:06Z (6ef3e91).
Built: Tier::{Drifter, Sprawler (explore_steps 200; 400 starved it), Trunk} on CPU+GPU; Tier::{Wary, Grudge} on BigParams::scar_* (CPU only; GPU refuses until scar = channel 1 of binding 47, Params row after 880).
Pre-crater census (held-out seeds 201-216, #106 rules): Grudge hardest (all plans lower than plain 16/16), strike best on every tier, kit loud plan wipes all tiers. Held-out block 201-216 is this thread's; dev seeds 1-4.
On merge: tell integration thread and campaign thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXN6dqSoXu4VfzYG7x8SKVu3) directly.
Related: [[mould-behaviours]], [[squad-toolkit]], [[game-integration]].
Per-tier core (designer 07:55Z; Larder agreed 07:56Z), follow-up after #113: Tier::core() -> Option<Core{kind: Fixed|Headless|Regrow{starve_secs}|Migrate{every_secs}, secs, pull}>; read once at Game::new/with_tier; End::Core still fires except Headless (wipe only); pull lays alarm like shots (GPU shout path, no new binding). Build Sprawler Headless + Grudge pull first. Gate: strike best on at most 2 tiers; other plans win Sprawler, Trunk, Grudge; report random's mould-wipe count per tier.
Gate rule (integration, ROADMAP #120, 10:09Z): gates play seeds 5-36 and 201-232, report random's wipe count, quote the census provenance: line; keep #116's window tests green. Player AI thread drives the post-crater plan-by-tier matrix from one harness (tiers by name).
13:50Z: Wary removed (e359fac, pushed, not PR'd). Core values tested on scratch merge /tmp/claude-0/wt-core (Larder branch + mine): Sprawler Headless, Grudge pull 16 (GRUDGE_PULL const); dev 1-4: Sprawler strike ~ farm/wall (tied), random loses squad 2/4; Grudge random squad wiped 4/4. Waiting for Larder PR #126 to merge, then apply tiers.rs core() on my branch, PR, held-out census 201-232.
17:16Z (integration): any PR touching WGSL, a GPU Params row or a binding runs the twin tests on the 1070 first (book in booking.md, send via coordinator to PC runner); body quotes the 1070 machine line + pass count, or 'pending 1070 twin' if PC busy. Baseline 27/29; commands in wiki/model/gpu-sandbox.md 'Twins on the 1070'. Applies to the scar GPU port. Trunk 702c59b held-out rows 201-224 in /tmp/claude-0/c6 (225-232 from runner D pending).

17:45Z: #133 merged (a56f644) under build-first rule (#150): census is records, not a gate. Trunk random wipes 9/24 (201-224) - tuning target for Regrow. Next: scar GPU port (1070 twin rule), tier look page.
20:53Z: owner ruled headless default (#164, 'remove the king rule'); Trunk Regrow and Drifter Migrate DROPPED (my call, told designer + integration). Trunk softness = later numbers retune.

00:33Z 11-13: #180 merged (a1a7156): Drifter hunger 8->16 (First contact: random best 0.31->0.21 on 2ade1d4). Merge rule: green head must contain current main.
04:28Z 11-13 (#194): spare GPU Params words listed in ROADMAP GPU section (byte offsets 188,228,232,236,360,364,460,484,488,492,732,828 + one _p per species Set). Claim in ROADMAP in own PR, rename in WGSL + write_params; new 16-byte row only via integration. Use for scar GPU port.
07:11Z 11-14: building wear for Grudge/Swarmer = dead end (#243): take wins via army not turrets. Backlog, first lever AFTER owner's playtest (designer): Grudge scar/pull retune so the body converges on a squad fighting in one spot.
