---
name: mini-boss
description: Mini-boss slimes thread (11-16): owner rulings 04:58Z + 05:23Z; hunter/wanderer/sapper/leech built on "lumps" (round 3 #391 merged; leech ring + crowd 3 #396 merged); look thread separate
metadata:
  type: project
  modified: 2026-11-16T10:26:22.932Z
---

Thread "Explore mini-boss slimes" (cmsg_01PcwPdssrGxiXYvyCG8WaUX4bin1vJKXY3SgDC7CP5V1t, branch claude/project-thread-g8thj2), started by the owner 2026-11-16 03:41Z. Report Reports/mini-boss-2026-11-16/ (rounds 1-3). Pages: round 1 4hyVm1cPgWBYssTEnitdBV, round 2 AEQ89ydrEj4qQva7CehQZg, round 3 (kinds) RX8Jv6iQaZzyMvKJ7EHh2z.

**Owner's answers, 2026-11-16 04:58Z**: 1) prototype "all of them" (every kind on the page); 2) "Remove the bronze, keep the specks (specks and mini-boss are separate but both happen. Specks more often early game)"; 3) mix of seekers and stay-put; 4) "both": visible cause and chance; chance rises through the run; spawn "where there is crowding or a big part gets cut off", never a random spot; 5) full bosses held, but the idea is an origin entity at the slime's start crystal on EVERY planet that you must kill to win (routed to run progression); 6) squads target mini-bosses first: yes.

**Owner 05:23Z**: "They should be significantly stronger than normal slime too and vary quite a bit in strength from boss to boss". **05:21Z**: wants them to look cool; look thread "Make mini-boss slimes look cool" (cmsg_01PcwPdssrGxiXYvyCG8WaUX8BMLnYUy7fAXhh635oaWM9, branch w9vv1m, its PR #386) owns the view; reads lump bits 25-27, hurt bits 28-31 and `Game::lump_looks()` u16 (bits 0-2 kind 1-4, 3 not gathering, 4-7 grade, 8 fading, 9-15 fade progress).

Built: engine `big::Lump` (4 lumps), GPU twin exact. PR #385 (merged 08:43Z): hunter, strength by grade, boss-first aim, review fixes. Round 3 (PR #391, merged 11-16 11:14Z): `BossKind` drawn by weights `boss_hunter/wanderer/sapper/leech` (1 each) among kinds with a target, `boss_kind=N` forces; hunter lunge; wanderer roots on far fresh crystals and `grow_lump`s (members kept); sapper waits for a hole right across a wall, goes through, then hunts; leech sits on a pump, its pay goes to `stolen`, refunded if Broken, rig not worn. Members never starve (both engines); fade 1.5 s. PR #396 (merged 11-16 20:48Z): the leech rings its pump (`Lump::ring`, `boss_ring` 1.5 x crystal radius, look thread's suggestion) and `LUMP_CROWD` 1 -> 3 (a leech stalled 28 cells short in thick slime at 1). Headless `wall=`, `set=pump_hp=`, staged buildings raised (walls lay cells). 1070 timing not done.

**How to apply:** Next: Mortar, Pinwheel (engine work), origin entity (run progression thread). Lumps don't path-find: aims pull straight. Slime engine rules shared with [[slime-enemy-review]] (keep LUMP_BITS/KEPT in dense writes); raids with [[defense-payoff]]; targeting with [[gun-rounds]]; walls with [[walls]].
