---
name: mould-review-2-2026-11-10
description: Second independent review of the mould build (Keep It Alive, water and sleep, The Wild One): findings, page, PR in the new Slime-Mold repo
metadata:
  type: project
  modified: 2026-11-10T15:12:47.668Z
---

Second independent review of the mould game, asked by Scott 2026-11-10 04:08Z ("After you finish all your work, have the fable thread review it"). Review only, no build. Same thread as [[mould-review-2026-11-10]]; reviews [[mould-keep-alive-2026-11-10]].

- Page: https://claude.ai/artifact/DnqCHgesBQYWDJBHjE7PHr
- Report: `Reports/mould-review-2-2026-11-10.md` in **sgladstein/Slime-Mold, PR #1, merged 2026-11-10 as e242d2d**. Later Slime-Mold PRs go to the repo-move thread (cmsg_016bYH3Ybq4z6xkGqHujo4HW6cW77Bfhu8KKiLAnGgzRt9), not the Merge desk: that thread relayed this as Scott's ruling "for now" (2026-11-10, not seen first-hand here). The mould game lives in that repo since 2026-11-10 [[mould-own-repo-2026-11-10]]; clone at `/home/user/slime-mold` in this session.
- Method: about 270 scripted runs at Pixel_Physics main `b3bd36ac1`, the mould thread's hands made worse on purpose (aim error, slow looking, letting go early), paired seeds 1-8, sign tests. About 14 comparisons, so nothing passes the strict bar (0.004); an 8-0 is p 0.008.

**Verdict: both dishes work and every reported number reproduces, but neither is yet a game a person can get better at.**

Keep It Alive (dish 4):
- Water is the mechanism (bake without water vs oat-saver 1-7).
- Not flattered by precision: the clumsiest hand beats the oat-saver 8-0.
- Lives fall in two humps: of 32 water-hand runs 17 died by 5:05, 14 lived past 7:20; the same seed flips humps between hands. The 5:30 star was earned once in 32. What decides the hump was NOT measured (needs a per-run trace).

The Wild One (dish 5):
- First star is the opening salt wall: letting go at 30 s keeps it on 8 of 8. A salt ring round the wild nest is a second one-move star (5 of 8).
- Stars 2 and 3 need aim within about 2 cells (3 cells off: 1 star on 7 of 8).
- The snip delivers nothing (3 cuts through the nest: 0 stars on 8).
- The bell rings mid-crash: 4 of 7 piles run dry at about 2:10-2:20, the player's mould loses 28-54% in the last 10 s, lead moves by up to half.

Two-mould step (`Dish::step`, `Pile::bite`, `fight_losses`, `wild_left`): read in full, no bug; invariant check clean with a positive control. Four small things in the report.

Recommendations, ranked: 1 Wild One scores share of food eaten (or bell at about 2:15); 2 make seconds 30-150 matter at a person's aim; 3 Keep It Alive: trace what decides long games, then give that choice to the player; 4 make a cut-off wild piece visibly starve or drop the snip from that dish; 5 star bars follow.

Not done: hand play, mouse feel, sound, frame cost, finer tubes (PR #1062).

**How to apply:** findings went to the mould thread (cmsg_016bYH3Ybq4z6xkGqHujo4HWKXbN7RGrsGS3sjZN1gbMSb), which owns the fixes. Awaiting Scott's read as of 2026-11-10.
