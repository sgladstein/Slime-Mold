---
name: soldier-controls
description: Soldier controls thread (2026-11-12): RTS control scheme for larder-world proposed (queue, attack-move, auto-harvest, larder building, builders); awaiting owner's pick on who builds
metadata:
  type: project
  modified: 2026-11-12T04:24:20.279Z
---

Owner asked 2026-11-12 04:22Z for a thread to improve soldier controls/commands in `larder-world` (how to order harvest, build, etc.). Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXHqKQJaegWQfzHMZT2JhV4v.

Proposed 04:30Z: right click is context (crystal harvest, mould attack-move, wall repair, ground move); Shift queues orders; dry crystal -> harvester moves to the nearest safe crystal; larder selectable (B buys, right click rally point); V wall is a ghost that selected soldiers walk to and build over time; A+click attack-move; H hold, S stop; line formation on group moves.

Owner picked (card, 04:41Z): **soldiers build over time**, every building kind incl. walls. Split: buildings thread owns Building {kind, at, hp, built} and Cmd::Build; this thread owns Order::Build (soldiers raise `built`), queue, attack-move, Rules::seek, Game::rally, larder selection. Integration thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXVvB9idhrH7XKQHoSEmCnSB) owns merge order: Larder -> controls part 1 -> buildings 1 -> toolkit -> controls part 2 -> buildings 2.

Part 1 merged 2026-11-12 05:58Z (PR #102): queue, Order::Fight, Rules::seek (safe pick, on everywhere after designer + tech-lead review), Game::rally, double-click select, MAX_ZOOM 24. Report Reports/soldier-controls-2026-11-12.md.

Code facts: rules in src/mould/larder_world.rs (Order/Cmd from larder.rs, Game::apply, goal, walk, step harvest block); window src/bin/larder-world.rs (order_at, hud_events, command card around line 645); controls in src/world/controls.rs. Larder thread edits Rules::price, smell and shots in the same file: coordinate.

**How to apply:** part 2 waits on the buildings thread's Building PR. Related: [[world-ui]], [[larder-world-playtest]].

Overnight 2026-11-12 04:48Z: owner went to bed; threads decide open questions together and move forward (no owner answers tonight).

Part 2 merged 2026-11-12 11:01Z (PR #122): Order::Build, BuildRules::raise 8 s (wall per 10 dish cells), idle soldiers take up ghosts, carriers to nearest drop_off, census hands use send_builder. Open from the proposal: right click on mould = attack-move, line formation, per-soldier toggles. Zoom: graphics thread suggests capping at 16 (24 looks blocky); kept 24, owner's call.

Part 3 (2026-11-12, PR #153, merged 21:06Z): Rules::crystal_solid (on) means soldiers can't stand on a crystal with charge left (owner 14:43Z). The solid disc is r*sqrt(charge/start). Goals move to the rim (Game::rim), and reach counts from the rim (Game::reach). Census stopped early under the build-first rule (17:26Z). Page https://claude.ai/artifact/6KXwjXXSjXjCYE4F31vKJ3. The Larder thread owns the thick-slime slowdown.
GPU twin tests (flee, crater, core) in src/bin/mouldgpu.rs run with crystal_solid: false. With it on, seed 3's chaos parted the moulds by more than 8%.

Part 4 (owner playtest 2026-11-12 22:06Z, PR #169): build anywhere (`BuildRules::anywhere` on), `Cmd::Send` sends nearest idle (else busy, which resumes its order), ghosts faint with bar + seconds left (`Game::raise_left`), wall ghosts drawn, builders stand at edge. Look `hand=far`, `centre=X,Y,ZOOM`. Page https://claude.ai/artifact/JX94pzk5N7T9Ec7EGMWbGa. No census: AI may now pick far sites.
