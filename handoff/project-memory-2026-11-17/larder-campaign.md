---
name: larder-campaign
description: Larder campaign thread: missions 1-8 built (#226, #239, #256 queued 11-14); next a tuning pass after range 24; census tools and plan hands
metadata:
  type: project
  modified: 2026-11-14T07:15:55.340Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXN6dqSoXu4VfzYG7x8SKVu3 (branch claude/project-thread-sjil14). Design approved by owner 2026-11-12 04:44Z; full history in Reports/larder-campaign-2026-11-12.md (one section per census version) and each mission file's comment.

**State 11-14 07:15Z.** Missions 1-7 in Reports/larder-campaign-2026-11-12/missions/. 4 Cut the cord (Trunk, shrink 0.2, rations on: teaches rations), 5 Kill zone (Swarmer, retake 3 60 1 2 3), 6 The nest (Sleeper, rations off, shrink 0.5), 7 Spore season (Sporer, clear 1 60 5: keep spores off the landing crystal 1:00). Two winning plans each (census v13-v16, seeds 1-4); idle/random lose. Elbow room (#227) forced bigger strikes: 14 soldiers (m4), 12 (m6). Spore season: pump helps but doesn't decide (tuning note).

**Tools.** examples/campaign.rs: hands = AI styles (larder_world_ai Style knobs joined by `+`), `PLAN+STYLE` with plans from `mould::plans` (ring, pumps, guard, turtle, zone, cut; moved from larderworld.rs), `gaps+STYLE`, `keys=K:V+K:V` (CPU tier keys). campaign::Mission::list for the start menu (agreed with start-menu thread). Goal::Take(f, secs) held, counts after TAKE_FROM 0.1 of charge gone. Ignored test missions_4_to_7_plans_still_win (~2.5 min).

**Rulings/context.** Larder stays in this game; squads/no-larder is a separate mode (`mode=squads`, owner 04:49Z). Owner beats the game easily, so AI losing a mission is a tuning note, not a blocker. Merge via `queue` label (MCP issue_write labels).

**Mission 8 built 11-14 (PR #256, queued).** Two moulds: own mould (own_mould::garden_on_map / GpuWorld::garden_on_map: enemy at map's mould starts, own on the crystal nearest the landing, sides set where placed) vs Swarmer (stand-in; `tier grudge` once scar keys are on the GPU) on the Passes; mission line `own`, goal `outgrow 0.6 60` (World::own_living; GPU can't split take share). Plans `sow`, `bait` (lure on clean ground behind enemy) in mould::plans. Census CPU seeds 1-4: sow+boom 3/4, bait+boom 3/4, boom 3/4, idle 0/4, random 2/4 (a standing Feed task spends the start bank early: tuning note; levers start_bank, Tend::feed, CONVERT). Next: tuning pass over all 8 after range 24 (#249). Designer's m8 notes (08:49Z): boom winning 3/4 is the real issue, so the goal must need your mould (crystals held, or share plus a size floor e.g. own >= 40% of the enemy's peak); cap the total feed rate into your mould rather than cutting start_bank (random to <=1/4); first hint points at bait (lure) since it grows your mould soonest; keep the start on the crystal by the landing. The rival pair moves to chapter 2. Designer wants the larder-overrun loss, if ever replaced, to be "mould holds every crystal you took".

Related: [[larder-world-playtest]], [[larder-maps]], [[mould-behaviours]], [[player-mould]].
