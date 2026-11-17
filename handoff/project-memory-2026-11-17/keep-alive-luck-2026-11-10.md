---
name: keep-alive-luck-2026-11-10
description: What decides Keep It Alive's short vs long hump (traced 2026-11-10, PR #17) and the proposed next test
metadata:
  type: project
  modified: 2026-11-10T18:31:06.837Z
---

Traced 48 games (PR #17, merged 2026-11-10; Reports/mould-keep-alive-luck-2026-11-10.md): the first pile landing on mostly-asleep mould decides the game. Sleeper within 45 cells: long 13 / short 2; none: long 7 / short 26 (Fisher p 0.00003, exploratory, cut named after looking). Luck because the landing ring shows only 5 s ahead. `PILETRACE=1` in mouldprobe gives the per-pile lines.

Next (a game change, waits on owner's go): show the next landing 20 s ahead, a hand that sleeps the mould toward it, lives vs today's `sleeper` on fresh seeds 17-32.

**How to apply:** don't re-trace; confirm on fresh seeds or test the preview. Owner card "trace now or wait until played" was still unanswered when this was done. Related: [[handoff-triage-2026-11-10]].
