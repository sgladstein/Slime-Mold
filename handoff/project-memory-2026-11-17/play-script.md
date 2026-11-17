---
name: play-script
description: Owner plays Larder World by double-clicking play.cmd at the repo root (PR #245, merged 2026-11-14)
metadata:
  type: project
---

play.cmd -> play.ps1 (merged #245, 2026-11-14): pulls main (skips on local changes/offline), builds larder-world + larder-edit release, opens the start menu; extra args pass to the game.
**Why:** owner asked why playing needed four commands (07:43Z).
**How to apply:** tell the owner "double-click play.cmd" instead of cargo commands. First Windows run was untested in the cloud; fix it if the owner reports trouble.

**Gotcha (11-14 19:51Z):** play.ps1 skips the pull when any *tracked* file is modified. A replay (`larder-world-replay-flats-1.txt`) had been committed in #192, and the game overwrote it on play, so play.cmd stopped updating. #293 untracks it and gitignores `/larder-world-replay-*.txt`. Never commit files the game writes beside itself; owner's clone needed one `git checkout -- <file>` (PC runner).

**Playtest pack 11-17 (#433 merged 14:59Z):** `play-new-slime.cmd` = play.cmd + `far_smell=1 set=unit_paths=1` (off switches with no measured harm; command-line keys go over each run planet's). `fire_alarm` kept OUT (designer 14:38Z: a per-tier trait for a fire-shy Sprawler, never global; globally Torch/Tinder burned 0 against 119). ROADMAP "Playtest pack 11-17" groups the rest. The simulation-use thread is building one blind pick page (setting+look, block/trap, crust sign, fire_fed, plus a Heave/Corral merge line). After the owner plays and picks, each row gets a default or a delete.
