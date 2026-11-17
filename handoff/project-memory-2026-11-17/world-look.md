---
name: world-look
description: Larder world graphics: view shades the world from sim fields, sprite soldiers and buildings (#118), shot holes from living walkers (#93); open follow-ups listed
metadata:
  type: project
  modified: 2026-11-11T16:24:54.781Z
---

Owner 2026-11-11 15:50-15:59Z: shaders for most graphics; test whether a shader soldier reads, else SVG / CC0 / image generator; the visuals must follow the simulation (slime especially); include shooting, damage to slime, environment, walls, and crystals.

Built (draft PR #69, branch claude/project-thread-4s8zqp, stacked on camera PR #63): engine `look` field (u32 a cell: thickness, walkers, wound, scent), `Engine::set_hits` (shots cut trail, wound fades), `Engine::block` (walls); `world::look` ground field (byte0 kind 0 agar 1 wet 2 rock 3 wall 4 rubble 5 crystal, byte1 value = wall health share / crystal charge share, byte2 crystal depth); `View::set_look`; `Marks::soldier` shader figure; sandbox key L. Report Reports/world-look-2026-11-11.md, page https://claude.ai/artifact/J7K7W5hSvxutqZmhN91yVd with a blind soldier A/B (key in the report).

Larder thread's field names (PR #64 BigDish): agar, rock, wet, wall, wall_health (full = p.wall_hp), charge (full = p.crystal_charge), pile_of; rubble is just wet ground (no mask).

Cost on the GTX 1070 at 1080p: prep pass (cell height/material once a frame) cut the look view 1.6-1.8 -> 0.8-1.2 ms (plain 0.3). The 1070 has two speed states (plain 0.29 vs 0.37 ms): time alternating runs and compare matched ones. Ground (owner 19:29Z 'just black'): bare agar now soil + engine tints; rock/wet in test world; ground byte 3 = elevation, kept 0 until the sim uses height (Larder thread condition). CC0 sites (kenney.nl, opengameart) are blocked by the cloud proxy; fetch on the owner's PC if needed.

Owner 16:48Z: picked A (SVG sprite) for soldiers -> atlas src/world/sprites/ (soldier.svg, render.js, sprites.png); shots stop at the first slime (`world::look::first_hit`, Larder thread calls it; walls pass, rock stops); crystals hexagonal (sim footprint changed in PR #64 6456f6b).

**How to apply:** new world visuals go through these fields, not drawn-only layers. Related: [[larder-world]], [[scott-desktop]].

Merge order agreed with the Larder thread (18:59Z): #63, GPU W0 (binding 32 energy), Larder W1b (bindings 33-41, agar read_write, Engine::build_wall), then #69 rebased by this thread: hits/look move to 44/45 (W1b+W2a take 42 squad, 43 kills; GPU thread from 46), block() calls build_wall. Shots: "slime" = look thickness >= 26/255, fresh read each frame (owner 17:37Z complaint; 98/120 shots crossed slime at 60/255).

PR #69 merged 2026-11-11 21:49Z (a741810). Still open: view reads its own ground buffer (world::look), not the engine's ground word (binding 34); the maps thread may take that with soil kinds. draw_marks now splits each mark over 16 workgroups (1070 cost untimed). The 'mould from one spot' question (examples/worldspread.rs) was handed to the creature rules thread at the owner's request (21:36Z).

Shot damage (owner playtest 2026-11-12 04:13Z): blind A/B (X = living-walker thickness + per-death char, Y = old burn); owner 04:38Z "Neither. Both of those look like stamps". PR #93 merged 06:12Z (264ddec): no wound drawn, body() = trail x living walkers (prep tile, +0.26 ms z1 on 1070), View::set_body_density, larder-world holes=1. Finding: hole closes in ~0.2 s (walkers flow back), regrowth halves losses; lasting damage is the Larder thread's rule side. Buildings thread will draw plain marks; this thread swaps in sprites later (ui::Model carries buildings list).
Design review (04:35Z, owner approved sending): coming effects to draw from data: toolkit blast crater (trail fades fast ~20 s), one-body thread's cut-off hunk that starves; also show alarm field and hunger subtly (mould intent: hunting, fleeing, starving).
Follow-up (04:56Z): once the Larder PR with `Game::density` (start walkers per OPEN fine cell) merges, feed View::set_body_density from it; BODY_DENSITY is calibrated per ALL cells (5000/(512*320)), so recalibrate by the open fraction when switching.
Buildings (PR #118 merged 2026-11-12 ~10:07Z, e203c57): Shape::Building / Marks::building, atlas 1536x160 tiles 4-11 (src/world/sprites/buildings.svg); rises as built, darkens when hurt, lamp lit while Site::working. Walls stay rects. Next: after crater PR #113, check the lasting hole reads at zoom 6/16.
Crater look (10:15Z): #113 head renders 4 equal red-brown discs with a dark rim (hand=rush 15 s). The view has no switch; the stamp comes from the rule (uniform disc, keep 0.1 cuts in one cell -> rim, thin film inside = dim hue). Asked the Larder thread for a kill-shaped, gradual crater; re-render when they have a new head. Worktree /home/user/wt113.
Update 10:11Z: Larder thread tried kill-shaped (hole shrank) and gradual drain (still round); ships the disc, puts an aim-spread variant behind a switch as an owner A/B. Re-render that variant's head when it exists.
Crater red-brown (10:45Z): it is the Fork map's clay disc round the mould start (CLAY_RGB 104,54,38), not a wound or film; proved by debug render. Sent A/B/C/off renders of fbc3e7d to /mnt/project-files/crater-ab/. Offered to soften CLAY_RGB if wanted.
#113 merged 12:14Z (945e080), binding 47 = countdown+strength. Decided not to draw the crater field (owner's stamp ruling); revisit only if the owner's blind pick on the crater page asks for it.
1070 re-time 15:49Z (memory free, GPU shared, default screen): body tile +0.14 ms z1 (0.579->0.715), +0.03 z2, 0 z4; main with buildings = tile; +23% vs +22% before. In report via PR #138 (merged).
PC booking (15:55Z): /mnt/project-files/scott-desktop/booking.md; graphics re-time is #2 in queue after GPU thread; my 15:49Z re-time doesn't count (GPU 80-85% busy). Re-run at 1920x1080 alone with the machine line (larder-world machine(), e516f4f; worldlook lacks it so print the same via powershell+nvidia-smi).
Clean 1070 re-time 17:10Z (PC idle, via PC runner): z1 1.187->1.444 (+0.26), z2 +0.05, z4 0 = morning numbers; main w/ buildings = tile. PR #144. PC jobs now go via coordinator to the standing PC runner thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXSwyPUwNtHT7L2TgFCr59ga), not own RC sessions.
Testing rule (owner, PR #150, 17:29Z): build first, balance later; looks land on tests plus a GIF or playable build; balance never blocks a look (retune numbers instead).
Siren sprite PR #161 merged 2026-11-13 02:32Z: atlas 1792x160, tiles 12-13, kind 4; lamp gradients fixed (were centred at bbox corner). Rule now: PR head must contain current main before merge.

C8 (11-14): View::set_news(news, state, wither_lag, panic_lean) shows cut-off walkers (amber lean, red bolt, ash fade); view side PR #212. The larder-world call `v.set_news(e.withers.then_some((&e.news, &e.state)), ws.wither_lag, ws.panic_lean)` after set_alive (ws = &game.world.s) waits for the larder-world split freeze; graphics thread adds it after.
