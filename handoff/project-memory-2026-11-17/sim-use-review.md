---
name: sim-use-review
description: 11-16/17 simulation-use review thread: plan draft 3 all built and merged; open owner picks on one blind page; next per-tier fire-shy key
metadata:
  type: project
  modified: 2026-11-17T15:10:00.000Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXMnf4G8wzvcSUjzucRhDDEx. Owner (Scotttt) 11-16 21:44Z: review how the game uses the slime simulation. Plan Reports/sim-use-2026-11-17/plan.md (draft 3); every report listed in that folder's README.md.

Owner rulings (wiki/decisions-game.md): fields over scripts only where not worse; E then C/B; bulwarks block shots and soldiers; fed slime fireproof, starving burns; plant-eating dropped; B tools = devices + gun cards; setting toughens under fire.

Built and merged (all of draft 3, through #432 11-17): scar on GPU, crust (E1a/E1b), setting + bulwark + look (E2a/E2b), C1 fire_dry ON, B1/B2 Provoke/Still, C4 fire_alarm OFF, C5 gas_burn ON, S1 unit_paths OFF, S2 far_smell OFF.

Open owner picks, all on ONE blind page (playtest pack 11-17 group 2, PR #434): https://claude.ai/artifact/QNErAVSGJgoLWhSHY9MJDF; key in pack-films.md (A=setting on; A=wall set_block; A=crust_weight +8; A=fire_fed OFF). Old cards for pull/block-trap/fire_fed withdrawn. Look card (glaze vs amber) still separate. S1/S2 blind pages: keys in s1/s2 reports; played via play-new-slime.cmd (far_smell=1, unit_paths=1; fire_alarm removed from preset by designer 14:38Z).

C4b (11-17, on PR #434 with the page): fire_alarm=50 is the Sprawler's does_to_you key (run planet 2), 0 elsewhere; run seed 1 planet 2 has nothing to burn (7 plain lights, 0 cells), so the trait needs fire cards' fuel; designer told it can move tiers. Later: 1070 timing for scar/crust/fields; +pull film needs a push.

Film tips: Reports/sim-use-2026-11-17/page-pack/film.sh is a worked set of larder-world headless runs (look=DIR run=SEED:3 hand=... at=...; frames -near/-all .png 1280x800); llvmpipe ~3x slower than real. Artifact watch limit (10) reached in this session.

**How to apply:** route picks through the pack page; see [[independent-reviews]], [[slime-enemy-review]].
