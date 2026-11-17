---
name: game-integration
description: Larder World game integration thread (2026-11-12): one owner per shared mechanic, merge order on larder_world.rs/squad.rs, GPU Params queue, overnight tie-breaker, morning playtest page
metadata:
  type: project
  modified: 2026-11-12T05:10:46.761Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXVvB9idhrH7XKQHoSEmCnSB owns the overlaps between the Larder World threads and the merge order (started 04:35Z on the tech lead review's suggestion). Plan: ROADMAP.md section "Larder World: shared rules and merge order" (merged #94, #98).

- Scent: everything through squad::UnitIn.smell. Harvest: UnitIn.take (pump too).
- Area effects: one `bursts` list in squad::Frame (toolkit). Scorch: one crater field (toolkit; GPU binding next free, likely 47, since one body took 46 + BODY_AT 848, PARAMS 864; bindings are contiguous, first to merge takes next). Hole under fire: Larder thread (candidate alarm_weight -1; one body handed it over 05:08Z).
- Buildings: one Building type; larder+outpost = depot; controls builds over time. UNITS_MAX 64->96.
- Merge order: Larder PR1 -> controls 1 -> GPU Game::step split -> buildings 1 -> toolkit -> controls 2 -> buildings 2 -> own mould. No slower-clock PR. mould_grid on GpuWorld rides in GPU thread's readout.
- Counts written against Game::density (walkers per open cell), measured at two scales.
- Owner rulings: Scorcher dropped (04:46Z card); soldiers build; gardeners own mould; campaign carry unlocked verbs only.
- Overnight (owner 04:48Z): no owner questions; integration thread settles Larder questions with the two reviewers and records them in ROADMAP.md.
- Species slots (05:51Z): 0 enemy main mould, 3 player own mould (own_mould::OWN), 1-2 extra enemy tier species; MAX_SPECIES 4.
- L-PT: this thread owns a morning "what changed, what to try" page on one combined build; scheduled task at 11:00Z.

**Why:** many threads edit the same rules file and GPU Params. **How to apply:** new Larder game mechanics check this map first. Related: [[larder-world-merge-order]], [[tech-lead-review]].
- Testing (owner 17:26Z, CLAUDE.md #150): build first, balance later; census never gates features/looks/controls; only strategy questions; >1 h needs owner OK. Tie-breaks follow it. PC booking: /mnt/project-files/scott-desktop/booking.md (this thread keeps it; runner can't write it).

Thread ids for direct notes (11-12): player AI cmsg_01PcwPdssrGxiXYvyCG8WaUXNPXVd4rSwedN6ATVfJwAg7; fight-feel/Larder rules cmsg_01PcwPdssrGxiXYvyCG8WaUXPPgmhkdJZ1Mr7xtNTLiPVf (owns PR 4/#170); shapes cmsg_01PcwPdssrGxiXYvyCG8WaUXQsjsMSgmPdKaKPdMkvKWhg. Census baseline breaks are listed in ROADMAP ("Baseline breaks, 11-12 evening").
- 11-14: split landed #210 (1e96f11): larder_world.rs -> larder_world/{world,rules,paths,cmd,raising,shots,turn,score}; bin -> src/bin/larder-world/{main,args,input,draw,events,headless,bench,hands}. PC jobs are one script scripts/pc-batch.ps1 (-DryRun first, twins, -Timing only on owner's go). Playtest pack https://claude.ai/artifact/TwAqFC2yHJAVT2MHzf9TRv sent 01:15Z. Owner 01:08Z: beats the game easily by hand; AI stalemate is an AI weakness, slime shared-default changes parked. Crater pick 2 s (01:05Z, Larder thread sets it).
- 11-14 02:33-03:00Z: #220 rations off (Rules::ration 0, owner 01:49Z; `ration=1` restores). #224 D1: larderworld census defaults to shape C at census size (scale 1, 6,250 walkers, 4x crystal_charge; shape=c full, shape=a old); shape_keys now in lib larder_world::shapes. Tier run pinned to 2371bf0.

Playtest branch 11-14 (owner played main+open PRs) is retired: all those PRs merged; #246 (08:08Z) merged the pack update, bench centre=X,Y,ZOOM and pc-batch view timing (opening/zoom 6/zoom 24).
1070 batch 11-14 09:13Z at 004b7cb: every frame <2.6 ms p95; 4 twin fails were card drift/chaos -> #261 (merged 11:11Z): tol(exact, card)/ON_CARD in twin tests (card slack only on real GPU, llvmpipe exact), report Reports/pc-batch-1070-2026-11-14.md, HANDOFF decision 6. PC queue empty.
