---
name: units-read
description: Units/buildings readability (#227 merged 11-14 04:10Z): elbow room rule, outlined soldier sprite, thin rings, hurt-only health, dead pose, larder sprite, building states
metadata:
  type: project
  modified: 2026-11-14T04:10:48.097Z
---

Thread "Make soldiers and buildings read clearly" (cmsg_01PcwPdssrGxiXYvyCG8WaUXXftESW48xjfbBSn6gxVhcn), art review issue 5. PR #227 merged 55331767 (2026-11-14). Page https://claude.ai/artifact/FQ1y7vhrqpgGZGn9LFw64F, report Reports/units-read-2026-11-14/.

- Rules::elbow 1.6 touch radii (Game::elbow_room, paths.rs, before each walk); Game::slot ring max(1.5, elbow*touch*1.05). Census baseline break. Engine twins in mouldgpu (seed 3 crater + core pull) set elbow 0.
- view.wgsl: sprite colours are 0-255 (an outline in 0-1 came out black). Soldier outline/rim, building ghost cream plan + seam, scorch when hurt. Atlas 2048x160, larder = kind 5 (tiles 14-15), Game::overrun_share.
- Marks::selected (thin cream ring), Marks::health (only under 95%).

- Harvest look PR #238 (merged 11-14 08:08Z): Game::last_takes (Take) shared with crystals thread; Marks::soldier_working (carry bits 9-16, take 17-24), Marks::building_pulse (larder flash). Glow must stay inside the body: one mark claims each pixel, so a wide glow bites neighbours.
- Turret look PR #258 (merged 11-14 12:51Z): base + head tile 16 (atlas 17 tiles, 2176 wide), Marks::turret (facing in colour2, state bits 25-26), turret looks in App.turrets (headless.rs turret_fired/turn_turrets), turret=X,Y film arg. Turret shots are free (Rules::price 0), so the dry lamp never shows.

**How to apply:** new unit/building looks go through marks.rs + sprites/ (node render.js regenerates the atlas). Related: [[art-direction]], [[world-look]].
- Building looks PR #271 (merged 11-14 13:48Z): all buildings 1 touch radius; lure scent light (Light lift<0 lit by look byte 3); pump turns hose to crystal (Marks::building_facing, Building::drew); outpost mini larder flares on banks (step_notes detects pack emptied; larder flare no longer from harvested); siren rings on real call; soldier flinch per 3 hp (Marks::soldier_bitten, colour2 word). Window notes BuildingLook/blooks. Film args lure=/pump=/outpost=/siren=. Page VL9ogWubo1dD6anisCLZVa.
- Siren alarm PR #272 (merged 11-14 14:27Z): view bindings 25-26 = engine germ[0]/[1] (alarm = 2nd block), View::set_alarm / set_alarm_side(done%2); pale haze + slope front in shaded, 1% early-out (+3.8% llvmpipe). Not red (slime-look thread: red = bolting/cut-off). Page MFM16Vc9siMWV4gYyNC3UQ.
