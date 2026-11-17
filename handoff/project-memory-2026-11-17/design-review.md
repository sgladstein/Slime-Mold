---
name: design-review
description: Game designer review thread (2026-11-12): ongoing big-picture review of all Larder world threads; first report's findings and what awaits the owner's OK
metadata:
  type: project
  modified: 2026-11-12T04:31:40.877Z
---

Owner (Scotttt, 04:29Z) asked for two review threads with different perspectives: game designer (cmsg_01PcwPdssrGxiXYvyCG8WaUX2dsdkBBhuYywD19AnT6kXG) and technical lead (cmsg_01PcwPdssrGxiXYvyCG8WaUX7L1qrgkAvCdykPj6C5Q5yB). 04:30Z: "These threads should continue to review as the other threads finish each task." The coordinator messages the reviewer when a thread merges a PR or finishes a milestone. Reviews change no code; suggestions go to threads (message_thread) only after the owner agrees.

First report (04:40Z): goal unreadable, verb overlaps, fix feel before verbs, no-soft-lock rule, campaign as onboarding, own species unowned.

**How to apply:** check the owner's answer on the card in this thread before messaging threads. See [[larder-world-playtest]], [[squad-toolkit]], [[larder-buildings]], [[soldier-controls]].

Owner 04:35Z: "Send all and start new threads, except your recommended to hold off on tools. I want to work on tools and feel in parallel" (no hold on tools). Done: notes sent to every thread; overlaps to the game integration thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXVvB9idhrH7XKQHoSEmCnSB, owns overlaps and merge order). New threads: campaign and onboarding (cmsg_01PcwPdssrGxiXYvyCG8WaUXN6dqSoXu4VfzYG7x8SKVu3), player's own species (cmsg_01PcwPdssrGxiXYvyCG8WaUXBmPwzNsk24pDonTtxjxzQ3). Larder thread adds a "stuck" census measure and a free-shots check on the fort (ring) style.

Owner 04:43Z (going to bed): "Always send your advice directly to the threads. You don't need my approval." Standing rule: review notes go straight to threads; post to the owner only what needs their eye (their choices), briefly.

Owner 04:48Z overnight rule: "When there is a question have the threads work together to come up with the best answer and move forward with it." Design questions: the design review calls first, with the tech lead review and integration thread. 04:49Z calls: campaign goes ahead on its three recommendations (verbs-only carry, graded cap, first-time hints; persistence as a later mode, revisitable); controls: soldiers build over time; toolkit: owner had already picked "All six together" (04:31Z), which stands; blast, crater, mine tested first within it.

Owner 17:00Z (cmsg_01PcwPdssrGxiXYvyCG8WaUXB5uQMwuRYCuXDSiZB3DHRE) questioned heavy census gating: hours of beats-random tests for each feature (even gun code), reviewers holding implementation for balance elsewhere, ugly slimes kept because a better look killed the player (should rebalance instead). How to apply for this review: build and look first, rebalance numbers rather than reject a better look; ask for a census only for strategy questions (does a plan/tier counter exist), small and late; never hold a feature for balance in another area.
Owner adopted the testing rule 17:26Z (PR #150): build first, balance later; no census gates on features, looks or controls; balance never blocks work; censuses only for strategy questions, runner time named first, over ~1 h needs owner OK. Earlier review asks for standing matrices / per-PR gates are withdrawn.
Cloud plan #205 (11-14 00:3xZ) calls: D2 park flow_loss; D3 counter table sent to player AI (C cut, Drifter strike, Sprawler fort, Trunk cut, Sporer claim first, Sleeper bait+strike, Swarmer kill zone, Grudge keep moving; pass = no plan best on >half, miss = tuning note); D4 mission 8 = own mould vs Grudge, rival pair to chapter 2 after per-species GPU counter PR; D5 menu+options+key card first. Next: review integration's C1 playtest pack page before it goes out.
Owner 11-14 01:08Z: "I have easily beat the game in all my playtests" -> AI census stalemates measure the AI, not the slime; never make the mould easier from an AI result; slime may need to be harder. Ask the owner how they won.
Card notes go to 'Rethink the gun card system' (cmsg_01PcwPdssrGxiXYvyCG8WaUXCpsFRiVSrGm8cVBDxSCd73); gun look thread (…NMGAy5sBZ) owns only shot looks.
