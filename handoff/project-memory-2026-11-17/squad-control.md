---
name: squad-control
description: squad mode (mode=squads) history; Guards renamed Lookouts and Guard order on F (command card rebuild 11-15)
metadata:
  type: project
  modified: 2026-11-14T04:10:26.229Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXRjKGDpWRiGwPp1xy9E2dBJ, branch claude/project-thread-vakljr. Owner 04:08Z: "revamp potentially the whole feel of the game. the unit control should be squad based from the ground up. All soldiers in a squad stick together ... really good and intuitive system to split and combine squads and to switch control between them ... when controlling a squad your mouse is the aim for the guns and they all fire in unison at the mouse. maybe whoever you are actively controlling is stronger (and that should be visible)."

04:10Z asked 7 feel questions (move WASD vs RTS, fire hold-LMB volley, idle squads keep orders, split X / box / merge G, boost kind, formation, new soldiers). Answers (04:18-04:28Z): 1c WASD steer + right-click send; 2a hold LMB = unison volley at cursor; 3a idle squads keep last order + auto-fire; 4 split/merge plan approved (1-9/wheel switch, X split, box pulls out, G merge, no size cap); 5a active squad fires faster + hits harder, visible in bullets/holes + ring; 6 blob default, other formations selectable; 7 new soldiers join the active squad, spawn beside it after a short call-in (10a); 8 REMOVE THE LARDER: crystals are "drain or guard" per crystal (card 04:27Z): drain = fast into bank, crystal gone for both; guard = slow trickle while a squad stands on it; owner 04:28Z "guarding has to be more valuable in the long run". Owner wants complexity, not a land-grab race. 9 slime's goal: hold as many crystals as possible to grow; lose = all soldiers dead. 11a WASD freed, old letter hotkeys move. 12a camera follows active squad, free pan, Space snaps back.

Code entry points: Game::begin_step (src/mould/larder_world/turn.rs) does walk/target/shots per soldier; Game::target and walk/goal_raw/slot in cmd.rs/paths.rs; window input src/bin/larder-world/input.rs, selection + Ctrl-digit groups in src/world/controls.rs. Shots cost Rules::price (0 now). Interacts with #227 soldier spacing thread, player AI, campaign missions (same Cmd path). Related: [[soldier-controls]], [[gun-rounds]].

PR #240 (11-14): squad mode. Not built yet: a call-in delay, a squad list in the HUD, the card's letter keys moved off WASD/X/G, player AI for this mode. Page: https://claude.ai/artifact/JVSTNATVeyXGivTGoT9HU7

PR #254 merged 11-14 11:53Z: squad tiles in the bottom middle panel (owner 08:22Z/08:48Z: "replace the bottom middle menu each squad should be a tile"); Split/Combine on its header (keys Y/J from bug sweep #262); stable keys (digit = squad id+1 while alive); no Outpost without the larder. UI-scale thread owns the bar's layout (#250); squad thread owns tile content.

PR #276 (merged 11-14 16:41Z): owner 15:06Z notes + 15:49Z answers. Guards while draining/building near slime (cover.rs; some=1/3, more=1/2, never; key I); guards cost 0.6 x share (owner: 7.5% "too small to notice"); heal after 3 s rest, full in 10 s by held crystal, 60 s away, 0.25 food/pt; Shift+right click queues squad orders; Combine and Link only while squads touch; a link is commanded as one (can't pick a squad inside it), whole link gets the lead boost. Procedural levels = separate thread; roguelike separate; campaign kept.

PR #330 (merged 11-15 08:25Z): owner 07:41Z "Healing should be a specific action" -> Cmd::Heal (key L, Heal button), nothing heals unasked; owner 07:57Z "squad mode is default mode" -> window defaults to squads (no mode=), missions stay RTS (larder; owner 08:09Z "campaign can stay the old mode"), look=/bench stay RTS unless mode=squads.

Guards review (11-15, thread cmsg_01PcwPdssrGxiXYvyCG8WaUXAjpdUFZKCE99TBg7S7Mwpo, PR #361 merged 11-15 18:52Z): owner 18:00Z "doesn't seem to work well, improve or remove". Guards posted out in the slime film died (5 dead vs 0 over 4 seeds); now posted at the work's edge (kept, improved). Film hand `hand=guardwork` (GW_WORK, GW_COVER env). Page https://claude.ai/artifact/XXkb98zpvyfuuStHjK3otP. Button/key I unchanged.

Command card rebuild (11-15, thread cmsg_01PcwPdssrGxiXYvyCG8WaUXSKhb5KyUn5oDvjXdvVLiGR, branch claude/project-thread-q1xt81): owner picks Card 2 (5x2, one cycling Fire button, Z/X/C kept), Burn a line removed, Guard order on F (Order::Guard; zone guard_r 8 / GUARD_RIM 3), the squad's "Guards" renamed **Lookouts** (key I, now on the card with Heal L), click a building with no squad selects it (owner 21:37Z: demolish refunds half, calling off a part-built one refunds all; recorded #366). Page https://claude.ai/artifact/9XmKNPSBSncdjAVQcfdjaR.
Rename (11-15 20:50Z, command card rebuild, branch claude/project-thread-q1xt81): the guards-while-working button is now "Lookouts" on the command card (1,4), key I; "Guard" is a new Order::Guard on F.
