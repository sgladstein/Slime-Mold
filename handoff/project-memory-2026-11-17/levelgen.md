---
name: levelgen
description: Procedural planets for the roguelike run (11-14/15): thread, interface, owner's answers, review, map fixes, doors, fen lakes + moat A/B, caves, per-biome openings, next map items
metadata:
  type: project
  modified: 2026-11-15T15:35:00.000Z
---

Thread "Generate roguelike levels procedurally" (cmsg_01PcwPdssrGxiXYvyCG8WaUXHpvAsm955EuZKKdhjnHSNE, branch claude/project-thread-0tcg97), from 2026-11-14 15:22Z. 

**Interface** (run thread cmsg_01PcwPdssrGxiXYvyCG8WaUXTX9SyKMGKZP4ciXzGUAHZu): `levelgen::level(seed, depth) -> Map`, `planet(seed, depth)`; pure, no platform maths, pinned fingerprints (`planets_are_pinned`, `every_dial_is_pinned`). Levelgen owns ground, crystals, nests, landing, soft rock, map size; run thread owns tier, walkers, goal, clock, rules.

**Owner's answers 11-14:** levels 15 min-1 h+; 60% area to 6x at the core; set biome order; twists, rare rescue planets later; open early, mazes late (Reports/levelgen-2026-11-14/biomes-plan.md). No view.wgsl edits from this thread.

**Merged:** #277-#312 (dials, walking maps, plants, arenas, GROW_CAP 6).

**Pathing:** ours = distance_map, MapBuild, map_for/pump_maps/trim_maps/stale_maps (larder_world/paths.rs). Since 11-16 05:2xZ walk, elbow_room, squad_turn, guide, rank_squad, place, funnel.rs belong to the squad movement thread (cmsg_01PcwPdssrGxiXYvyCG8WaUX4uE2KRVhL3PF2znGuNZZ1c); it asks before map_for changes shape; its steer.rs (#392) has each squad follow one map (Squad::map); check map_for changes with move_tests.rs (MOVE=planet1,planet8). MAP_NEAR 4 -> 12 (PR #382 merged 11-16 08:17Z: 200 soldiers at 6x thrashed the 34-map cache, 47% -> 19% lookups unserved).

**3-agent review** (11-15, page Kcy1AoquWJjJ3EciyA3KxP, Reports/levelgen-review-2026-11-15/): looks are the owner's #1 concern; drawing items went to graphics/ground threads, clock/breather to the run thread.

Windows pin run (PC runner) still to ask.

**Breather planet:** owner 11-15 02:17Z chose "Wait for your playtest" (no smaller rich planet yet; sizes stay AREA).

**Map fixes** #323 merged 11-15 (queue clashes are appended lines in Reports/README.md, pages.md: keep both, main's first): nests off edges, hedges off water, decks, fords.

**Front doors + back roads** #327 merged 11-15 (Reports/levelgen-doors-2026-11-15/): rock bowl, door 7-9 dish wide, breach, road via `route`; `check` holds DOOR_MAX 10 / ROAD_MIN 16; nests >= 470 apart; first-try 149 -> 143/160.

**Fen lakes + moat** PR #329 merged 11-15 07:22Z ( report Reports/levelgen-fen-moats-2026-11-15/, page R2ycNj8mmoUuffjfyCL4zo): fens roll lakes 3:1 (8 -> 15 of 20). `Recipe::moat` (rock lip + deep-water moat + bridge at the door) built but OFF: the slime's start fills deep water and crawls across, so by 30 s it reads as a thinner rim. Owner asked to pick A or B on a blind A/B (key raw/ab-key.txt).

Owner 07:45Z couldn't see the A/B difference; moat stays off.

**Caves** #337 merged 11-15: lobed caverns, tunnels 8-25 dish (`girth`), arenas.

**Openings** #345 merged 11-15 13:02Z (Reports/levelgen-openings-2026-11-15/, page WUT2XYmq9hBux7kqLw8uAw): `Opening::of(biome)` sets landing crystals per biome; meadow unchanged; nearest crystal 44-60 -> 40-94 dish; first-try 143 -> 141 (highlands 15 -> 13, tuning note).

**Rivers off nests** #352 merged 11-15 14:14Z (Reports/levelgen-rivers-2026-11-15/, page X37QSmDaK192GKjKai3UPC): rivers shift along route to keep RIVER_OFF 220 from nests, fords/bridge CROSSING_OFF 320; 2nd river with no room dropped. Valley nests by river 19 -> 0/40.

**Cave gates off pools** PR #355 merged 11-15 15:33Z (Reports/levelgen-gates-2026-11-15/, page VQ7tvbpMgYVTQrgoLy5KKD): wet gate moves along its tunnel (`Edge::way`) or stands on a sill (water let out, clay kept); gates in water 40 -> 1/164. All review map items done. Sill look at zoom 6 awaits owner's eyes.

**Next (ours):** twists, rescue planets.

Related: [[larder-maps]], [[map-editor]], [[roguelike-run]], [[independent-reviews]].
