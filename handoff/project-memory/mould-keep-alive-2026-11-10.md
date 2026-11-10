---
name: mould-keep-alive-2026-11-10
description: Mould Keep It Alive endless dish + water and sleep (11-10): PR #1059 to merge desk, page, numbers, what's next (wild second mould, finer tubes, Fable review)
metadata:
  type: project
  modified: 2026-11-10T08:55:36.841Z
---
Thread cmsg_016bYH3Ybq4z6xkGqHujo4HWKXbN7RGrsGS3sjZN1gbMSb. Follows [[mould-depth-2026-11-09]], [[mould-review-2026-11-10]], rule [[mould-everything-simulated]]. Looks thread [[mould-looks-2026-11-10]] draws water/sleep on top.

SCOTT ORDER (11-10 03:07Z, "Agreed"): 1 endless dish, 2 water+sleep, 3 wild second mould (steals, then fights). Bars: >=3 tools, random loses to plan; wild mould: player's hand changes which mould ends bigger (paired seeds); quote frame cost. Then the Fable review thread cmsg_016bYH3Ybq4z6xkGqHujo4HW8wgxAHEW1fvrMLaVGqw4vA reviews ALL of it (Scott 04:08Z).

LANDED: Keep It Alive went to main inside looks PR #1057. Water+sleep = PR #1059 (branch claude/project-thread-cek3ly, head 747dfeff7, sent to merge desk 11-10 ~09:10Z). Page https://claude.ai/artifact/5KN1DQDHhqV6faXFwx737Y. Report Reports/mould-keep-alive-2026-11-10.md.

DESIGN: Params::wet (Dish::wet per cell, base 0.5); lamp dries; <0.25 sleeps (no move/eat/upkeep, crust_life 20 s); >0.7 wakes; Tool::Water 2 drops, 1 back per 40 s (WATER_STOCK/WATER_REGROW_SECONDS); on via Level Tools::water (alive only). ENDLESS_MIN 20; score = Dish::awake_at (last step >=20 awake), HUD "Asleep m:ss"; cap 900 s.
NUMBERS 16 seeds: idle 1:08-3:48, random 1:04-3:29, oats-only Route 2:47-5:26, probe hand `sleeper` (lamp+oats+water) 2:51-10:42 med 4:41; sleeper beats each 15-1 p .0005. Stars 4:15/5:30/6:40 unchanged.
PROBE: mouldprobe level=4 (wet from level; wet=0/1 overrides), water= water_regrow=, LEVELTRACE (asleep count), SLEEPMARK=1 overlay. Census script /tmp scratch a16.py (session-only).
GOTCHA: `cargo build --release --examples` can take >30 min here; build only the needed examples. Force-push is denied; after a merge, rebase onto main is a fast-forward if the old head is in main.
Window bar still shows "3 None"; water = key 3 until looks thread draws the tile.
WILD ONE (step 3) BUILT 11-10: PR #1061 MERGED 13:15Z, page https://claude.ai/artifact/QDgynHJg8dPRKuhZf54sf9, report Reports/mould-wild-one-2026-11-10.md. Dish::rival = whole 2nd Dish; shared pile table (Pile::bite) = stealing; fight 0.02; WILD_REPEL 0.6; WILD_START 600. 16 seeds: idle 0 stars x16, random 0 x14, warden (all 4 tools) bigger 16/16 mostly 2 stars; lamp-only and salt-only win 8-0, snip-only useless (cut heals). +1.4 ms/frame. Looks thread told render.rs::wild is theirs.
FINER TUBES 11-10: sensor_offset 9->8/7/6 measured, KEPT AT 9 (planned hands break: Beat the Ruler 4/8 at 8, 0/8 at 7; Alive median 6:21->3:41 at 6; Wild share .67-1->.49-.63). Page https://claude.ai/artifact/5JT3D1sv7CwSRmYfkR6VTW, dead-end + report section, PR #1062 merged 14:07Z. Thin look = drawing job. Sent all to Fable review thread 11-10 ~14:00Z; awaiting its findings.
REPO MOVE (Scott 11-10 14:06Z): mould game moving to sgladstein/Slime_Mould, thread cmsg_016bYH3Ybq4z6xkGqHujo4HW6cW77Bfhu8KKiLAnGgzRt9; keep landing on Pixel_Physics until it announces the freeze, then mould work goes there. My scratch (page sources, run5.sh, w5.py, census outputs) archived at /mnt/project-files/game-design/mould-scratch-2026-11-10/.
CUT-OVER DONE 11-10 15:06Z: mould now lives in sgladstein/Slime-Mold (private, branch main, crate `slime_mould`, imports slime_mould::mould::...; history through PP #1063; my scratch under Reports/mould-scratch-2026-11-10/). ALL mould work = branch + PR in Slime-Mold (add_repo sgladstein/Slime-Mold); MERGES: the "Move the mould game to its own repo" thread cmsg_016bYH3Ybq4z6xkGqHujo4HW6cW77Bfhu8KKiLAnGgzRt9 merges Slime-Mold PRs for now (Scott 11-10 15:56Z); send it PR number + head SHA. CI ~5 min. PP #1064 deletes mould from Pixel_Physics.
