---
name: bug-sweep
description: 11-14 squad-mode bug sweep (#252, #262, #270 merged): stuck squad, shuffle, lost shots, keys Y/J, wheel zoom, follow drift, minimap; bug patterns to check in new code
metadata:
  type: project
  modified: 2026-11-14T11:10:33.498Z
---

Owner playtest 2026-11-14 08:01Z (squad mode): gun animation, formation shuffling, stuck unit, harvest shortcut, zoom. Report: Reports/bug-sweep-2026-11-14.md. Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXKV7ob5zFcdWnGkAo15FQJf.

- #252 (merged 09:04Z): squad anchor stays on ground (Game::slide), rank_squad matches places nearest-first, blob/ring don't turn, MAPS_MAX 48 walk maps.
- #262 (merged 11:21Z): App::catch_up takes notes of steps a command finished; SquadInput::aim sends Fire/Face between steps; squad keys Y split / J join (owner had agreed X/G at 04:18Z: two-line revert via squadmode::SPLIT/MERGE); card first, WASD letters moved (attack/mine R, gas F, sow U); wheel zooms, Ctrl+wheel switches; Camera::focus (inverse of centre_on) for the follow; minimap clicks in squad mode; card burn/build go through Cmd::SquadOrder; settle() runs kit_between.
- #270 (merged 13:02Z): sent squads check the whole line to dest else walk its cached map at the squad's pace (V-wall park, 65 -> 244 shots by 24 s); `ahead`/arrival measured from Game::shape_off (rest point via place_at), since a wedge's middle sits behind its anchor (wedges never arrived).

Bug patterns to check in any new window/squad code:
1. A Cmd finishes the GPU step in flight: never send commands every frame; take notes via catch_up.
2. Use Camera::focus, not centre(), with centre_on.
3. Input layers (squad keys, HUD card, controls): a key/click should reach the layer the screen shows; HUD/minimap hide the world.
4. Per-soldier orders to a moving squad are overwritten by squad_turn: use Cmd::SquadOrder.
5. Two commands on one card slot hide one (test every_command_card_page_has_its_own_slots_and_keys).
6. A squad's middle is not its anchor for a wedge/line: compare the anchor with Game::shape_off's rest point.

Left to the squad thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXRjKGDpWRiGwPp1xy9E2dBJ): digit keys renumber when a lower squad dies (live_squads order); Outpost is a trap in squad mode. PR #254's panel labels X/G must become Y/J.

**Why:** the owner asked for a sweep and then a general code review for these patterns (09:00Z).
**How to apply:** check these patterns when touching larder-world's window or squad mode; see [[squad-control]], [[slime-enemy]].
