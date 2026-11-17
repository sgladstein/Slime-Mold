---
name: turret-growth
description: Turret tech tree vs turret cards thread (11-16): recommended both (tree = bodies, cards = barrel); first step turrets fire a shared turret deck
metadata:
  type: project
  modified: 2026-11-16T21:11:12.929Z
---

Thread "Expand turrets with tech tree or cards" (cmsg_01PcwPdssrGxiXYvyCG8WaUXXJRGjVCVFQkr7bCsGwnuPX), owner 2026-11-16 21:07Z, branch claude/project-thread-nvxdod. Page https://claude.ai/artifact/V16Dp6AyxrZ6KQ4KMPzbwr (source Reports/turret-growth-2026-11-16/page.html).

Proposal: tree unlocks turret bodies (gun turret, mortar over walls, sprayer field writer, fan herder) with a branch each; cards from the squads' bag fill the barrel; one deck per body set in the armory (option A). Owner picked BOTH (11-16 21:36Z card): tree = bodies, cards = barrel. Owner 21:41Z: tree in the run shop only, not in missions. Still open: deck per body vs per turret, shared bag, next body; defaults A, shared bag, mortar.

Today's turret: buildings.rs building_units, plain strike, Bullet::by None, cast 0; shop items turret + turret+ in run.rs ITEMS; Render upgrade. Gun thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXCpsFRiVSrGm8cVBDxSCd73) owns rounds.rs/shots.rs card code: hook agreed by message. Related: [[larder-buildings]], [[gun-system-review]], [[walls]].

Step 1 PR #400 (queued 11-16 ~22:00Z): Game::turret_deck, Cmd::Load(TURRET_DECK=255,..), film key tload=T,NAME, Building::next; turret = by None. Report Reports/turret-growth-2026-11-16/ (page v2 with 6 kill-box clips). Independent review: deck is free (no bag/slots) so turtling trade doesn't exist yet; review called mortar pointless, but owner 22:10Z: mortar stays, its point is the longest range in the game (22:11Z: no hard cap on Range cards; mortar just starts longest, retune only if play shows a problem); cast kills credited to soldiers; Pull pushes away; replace by-None with a shooter id. Step 2 = bag+slots+armory entry+shooter id. Decision card cmsg_01PcwPdssrGxiXYvyCG8WaUXXLZUD4gFVbBgxmWrBgXhmb: first body (mine / picky targeting [rec] / sprayer / mortar).

Step 2a PR #401 merged 11-16 23:18Z (Bullet/Shot::turret, Firer, cast kills to turret_killed, render_pay on casts, Pull toward the gun); gun-card thread told. Step 2b PR #402 merged 11-16 23:30Z: run shop 'Edit the turrets' cards' (Pick::EditTurrets) switches gun slots to turret rack 2 slots/budget 4 (+1 slot +2 budget per turret+ level), shared bag (Run::aside), landing loads Run::turret_cards. Next: bodies (mortar = longest range), tree in run shop, raid-on-base census, 1070 timing.

Owner card 11-17 00:47Z: first body = MORTAR. Built (step 3, branch claude/project-thread-nvxdod): Kind::Mortar, BuildRules mortar_* (cost 600, range 80, min 8, every 4 s, blast 3, flight 0.6 + d/35 s), Game::shells (buildings::Shell) land as a short ShotIn fired by the mortar; MORTAR command id 37, build page (1,2) key B, sprite/shader kind 7; run ITEMS 'mortar' needs 'turret'; film key mortar=X,Y,T; Fork film 5 shells/20 s killed 390. Mortar fires the one turret deck (per-body decks still open). PR #404 merged 11-17 01:45Z. Next body awaits the owner (sprayer, fan, picky turret, mine); 1070 timing and balance pass still to do.
