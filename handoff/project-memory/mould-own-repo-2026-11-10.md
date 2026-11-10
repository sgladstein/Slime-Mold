---
name: mould-own-repo-2026-11-10
description: Mould game moving to its own repo sgladstein/Slime-Mold (Scott 11-10): plan, what goes, what stays, status of push and Pixel_Physics removal
metadata:
  type: project
  modified: 2026-11-10T14:27:24.897Z
---
Thread "Move the mould game to its own repo" cmsg_016bYH3Ybq4z6xkGqHujo4HW6cW77Bfhu8KKiLAnGgzRt9 (Scott 11-10 14:06Z): move the mould game to its own repo AND put every mould research report, tool, script, page in that repo, not in the project folder.

NAME: card said Slime_Mould (14:10Z) but Scott CREATED sgladstein/Slime-Mold (private, empty; also a stray empty Slime-Mold- , not ours to delete). Push waits for PR #1063 (sim-true look) to land; then run scratch reextract.sh (re-filters latest main, lays the scaffold commits back on; verified identical on unchanged main). Scott tapped REMOVE FROM PIXEL_PHYSICS AFTER THE MOVE (14:28Z).

PLAN / BUILT (local, session scratch): git filter-repo of Pixel_Physics main ca6b08f6 on src/mould, bins mould+organ, examples mouldprobe/mouldlooks/organprobe/mould_world_sketch, Reports/mould-* = 59 commits from 11-07. Crate `slime-mould` (lib `slime_mould`, `pub mod mould` kept so paths match). Engine deps were only hud.rs+hud_crisp.rs (font, render::put inlined) and audio/wav.rs: COPIED, no shared build. rust-version 1.87 must stay (clippy chunks_exact_to_as_chunks fires without it). mould_world_sketch needs engine+mould: kept as research/engine-sketches/*.rs.txt. mouldhand promoted to examples/. Brought in: new-game-brainstorm-2026-11-07 text, 61 MB prototype pictures into Reports/mould-game-2026-11-08 (pages now one folder each with images, + 9 sensor angle, 10 food flow, 11 bait patches), mould-scratch-2026-11-10 tarball contents, review extras; Reports/pages.md indexes all 22 mould Artifact pages. Jones papers NOT committed (copyright), cited in PAPERS.md. Repo pack ~100 MB. 63 tests pass, clippy clean.

PIXEL_PHYSICS REMOVAL (branch claude/project-thread-ydga9b, approved by Scott's tap, PR after the Slime-Mold push): deletes the above, lib.rs line, arcade Mould row (1-7 keys), README arcade line kept same line count (README TOC regen churns 275 lines otherwise), Reports/README + instruments.md pointer rows (instruments first column must not backtick a missing example: docscheck). dead-ends.md mould entries LEFT in Pixel_Physics (history; copied too). Seep's own mould (src/seep) is separate and stays.

CUT-OVER: mould threads keep landing on Pixel_Physics main until told; at cut-over re-extract from latest main. Mould thread, review thread answered (nothing unlanded); looks thread (sim-true look) not yet. After cut-over all mould work goes to Slime-Mold. Related: [[mould-keep-alive-2026-11-10]] [[mould-looks-2026-11-10]] [[mould-review-2026-11-10]].

DONE 11-10: Slime-Mold main pushed 15:04Z (70888aa, CI green; dd38b6e CI runs push only on main). Pixel_Physics removal PR #1064 MERGED 15:35Z as c3588b3f: src/mould gone there. ALL MOULD WORK NOW GOES TO sgladstein/Slime-Mold (crate slime_mould; add_repo sgladstein/Slime-Mold). Review thread's 2nd review = Slime-Mold PR #1 (given to the merge desk). Stray empty repo Slime-Mold- is Scott's to delete. Jones papers stay only in project folder game-design/mould-depth-2026-11-09/papers/.
SLIME-MOLD MERGE DESK (Scott 11-10 15:56Z cmsg_016bYH3Ybq4z6xkGqHujo4HWAQ2bStxAHXfbMfZGCin9ey: "You are managing all merged into the new repo for now"): THIS thread (cmsg_016bYH3Ybq4z6xkGqHujo4HW6cW77Bfhu8KKiLAnGgzRt9) merges every Slime-Mold PR, not the Pixel_Physics Merge desk. Rule kept: merge on green CI (2 jobs: test+clippy, sound) on the exact head, merge_method merge, expectedHeadSha. PR #1 (review 2) merged 15:57Z as e242d2d.
