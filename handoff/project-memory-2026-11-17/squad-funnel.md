---
name: squad-funnel
description: Squads funnel into a column through narrow gaps (Rules::funnel, funnel.rs, 11-15); PR #326 merged 11-15 05:04Z; awaits owner playtest
metadata:
  type: project
  modified: 2026-11-15T03:57:21.905Z
---

Owner 2026-11-15 03:00Z: "it gets awkward moving with large squads through narrow areas". Thread cmsg_01PcwPdssrGxiXYvyCG8WaUXAqmHgyg59ak7px8ogk2xwW, branch claude/project-thread-mr154j.

Built: src/mould/larder_world/funnel.rs (Column {on, trail, places} per squad in Game::columns). Squad looks formation width + gap ahead (guide); narrower ground there or at anchor -> anchor jumps to the front, places become tapered rows back along the anchor trail; head waits only for its nearest member; exits when wide again and all members have a straight walk to the anchor. place_at skips walk_line for column places (paths.rs one line; levelgen thread OK'd, it owns distance_map/map cache only).

Tests/harness: src/mould/larder_world/narrow_tests.rs (gaps(): bridge8, door6long, door8slant maps, saved in Reports/narrow-squads-2026-11-15/maps; measure_bridge/time_funnel ignored). Film helper Reports/narrow-squads-2026-11-15/raw/film.rs. Page https://claude.ai/artifact/Pa4SQzNxAAp9WSTrikKZfS.

Open: slanted approach still bunches at the mouth; 12-16 squads gain nothing and cross ~0.5 s slower. Related: [[squad-control]], [[levelgen]].
