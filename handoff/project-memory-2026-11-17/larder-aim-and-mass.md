---
name: larder-aim-and-mass
description: Larder thread 11-13: #197 wading counts whole walkers, #198 aims only at slime a unit can see (both merged), #199 walker-count audit (all merged)
metadata:
  type: project
---

**#197 merged 11-13 16:59Z (c387f41)**, owner 15:55Z "why are they not way slowing down": wade used Rules::mass (0.2 on shape C), so half speed took 30 walkers; now `touching / wade`. Walking in was already deadly since #174; the sink page's clips were cut on a pre-#174 build (check the binary has current main before filming). `Rules::arming` (blast kept off a soldier's own touch circle) DROPPED: owner 16:15Z, a soldier rushed while standing still must be able to shoot what reaches it (dead-end + patch in Reports/larder-wading-2026-11-13/raw/). Smoke on C: strike 0-4 worse paired (tuning note). Page https://claude.ai/artifact/DKoWuDZja5GGeMTUuJFLMR.

**#198 aims that see** (merged 17:28Z, 6e9008a; page v2 has the aim clip): the aim (densest 4x4 block in sight) had no wall check and Game::target dropped blocked shots, so a squad by a wall fired 47 shots in 26 s; now `squad::sees` (integer line, both engines, WGSL `sees`), 198 shots. CPU cost +~10% on squad::read (under 0.1% of a tick). Test a_step_in_halves now gasses at touching >= 3. 1070 twin booked in booking.md, HELD (owner 11-14 00:28Z: GPU busy, no PC jobs until they say it's free); it also covers #181's twin, which never ran.

**Mass audit, PR #199, merged 17:46Z (a17cdc5)** (integration thread's ask; findings sent to integration to pass on; design review notes recorded: wall-fort hand in the next balance census, cover-seeking tier idea): every walker-count reader commented whole / by mass / presence; test walkers_count_whole_on_soldiers_and_by_mass_on_the_larder_and_buildings (Later test world has `touch: Option<u32>`). By mass only: larder steal, outpost steal, building wear. Finding for the player AI thread: View::mass reads (dare, HELD, wall_alarm) are whole walkers against 5k-mould numbers, so on C they trip at ~1/5 the thickness. Sound kill weight and view BODY_N scale by density by their own laws.

**Owner picked 11-14 01:05Z (card): "Shorter craters, 2 s"**: done in #218 (merged 11-14 ~02:10Z, 1815863): `Rules::crater` 2.0 in src/mould/larder_world/rules.rs (after the C9 split); hole refill on C median 0.5-0.9 s vs 1.3-3.9 s; report Reports/larder-crater-2s-2026-11-14.md. Found: a soldier under a far burn order fired nothing (volley squad on C died without a shot): fix #221 merged 11-14 02:44Z (88eb193; Game::target falls through to own aim when the burn point is out of reach). The `volley` look hand's walk is stale for C (squad never reaches its stand point; starves from 1 s).

**Why:** the owner judges by play; the mould must be deadly to walk into and soldiers must shoot what they face. **How to apply:** read the code before measuring (owner 15:56Z); see [[larder-world-playtest]], [[slime-enemy]].
