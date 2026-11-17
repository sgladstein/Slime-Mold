---
name: walls
description: Walls thread 11-16: rubble as ground (#384), doors + Thicken merged #395; turrets fire over walls, rifles don't (owner 19:50Z, merged #398); eat-through times
metadata:
  type: project
  modified: 2026-11-16T15:12:49.139Z
---

Thread cmsg_01PcwPdssrGxiXYvyCG8WaUX2vD9H66GxiPwtE8UF7tPuY, branch claude/project-thread-fynzif. Owner 2026-11-16 06:06Z: how soldiers get through own walls, 2-4x thick walls, slime must not go over walls, only eat through with the real sim.

Found: sim already blocks walkers at walls and bites them (BigDish::bite_walls, wall_hp 400/cell, bite 1/bump -> rubble = ground). "Going over" was the view: rubble drawn as low wall. Fixed #384 (merged). Builders-past-wall fix #394 (merged 13:48Z).

Measured (planet:1:7, `hand=pen`, `set=wall_width=W`): slime in 90 s after contact at 2 cells, ~290 s at 4, 8 cells lost 24% by 8:00. Report Reports/walls-2026-11-16/, page https://claude.ai/artifact/TQvqcvpKwK1EBFiqrhPW11.

Owner picks (cards): 14:24Z Doors; 14:25Z Thicken a standing wall. Built in src/mould/larder_world/doors.rs:
- Door = Kind::Wall with Building::door; cells are world wall (slime bites, shots blocked); Game::doors mask + Game::ground (open||door) used by standable, standable_cells, walk shut check, steer want, squad anchor/dest (agreed with levelgen + squad-movement threads). Door drawn along a standing wall snaps on (door_host), takes level/home, cut at once; across a gap raised by builders. Cmd::Door, window DOOR=36 build page (1,3) key G. Look: Ground::Door=6, timber grate in view.wgsl (slate blue read as water).
- Thicken: Game::upgrade on wall -> thicken: band of wall_width on home side, WALL_LEVELS 4 (2/4/6/8 cells), each at wall_price; doors in it follow; wall card U; Game::wall_of maps a door to its wall.
- Film `hand=door` (pen ring + door + sortie + thicken), Reports/walls-2026-11-16/door_clip.py.
- World trait gained `wall_cell(i)` (no default; every wrapper must pass it on).

Owner 19:50Z: turrets fire over walls, lobbed fire over walls, soldiers' guns do not. Built, merged PR #398 (11-16 21:27Z): UnitIn::over / ShotIn::over / Bullet::over, squad::sees_over + over_wall (standing wall cell, not rubble), Game::lob_line; GPU squad::OVER bit 31 in sight word and shot count, shader over_wall = ground WALL bit. BuildRules::turret_over (set=turret_over=0 = old rule). Thrown tools already went over (Kit::use_tool checks reach + open only). Film hand=turretring: turret ring held 646/646 to 3:00, 5,546 kills vs 0 (ring 335) without. Balance note: turret ring now turtles vs default slime. Owner 11-17 03:08Z: bullets do NOT pass through doors ("No"; recorded PR #406). Mortar card not built.

Related: [[wall-sides]], [[larder-buildings]], [[defense-payoff]], [[squad-movement]], [[levelgen]].
