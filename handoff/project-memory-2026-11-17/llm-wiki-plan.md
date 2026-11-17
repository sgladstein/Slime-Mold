---
name: llm-wiki-plan
description: Status of the Slime-Mold LLM wiki (wiki/ folder): what is built, where the plan lives, what the owner still has to decide
metadata:
  type: project
  modified: 2026-11-10T16:56:40.436Z
---

2026-11-10: owner (Scotttt) approved building an LLM wiki for Slime-Mold, starting with seven pages (16:49Z). PR #9 (branch claude/project-thread-cmvrom) adds `wiki/` with Keep It Alive, The Wild One, water and sleep, lamp, knife, census, decisions.md, plus `wiki/README.md` (rules and workflows), `wiki/index.md`, and `wiki/lint.py`, which runs in CI as a step of the "what changed" job. Plan and review: `Reports/llm-wiki-plan-2026-11-10.md`; the Claude Doc copy is https://claude.ai/artifact/SjjCHoyQubEkRxgAhAPjrP.

**Why:** knowledge is filed by date across about 10 core reports, so each session re-assembles the truth about one subject.
**How to apply:** read `wiki/index.md` before touching a dish, tool or mechanism. Ingest new reports into pages in the same commit. Open decisions live only in HANDOFF.md. Owner 17:24Z: repo will grow a lot, so growth rules are in wiki/README.md (lint fails on uncited new reports; NOT_YET_INGESTED lists pre-wiki ones). Owner 17:25Z: lint stays in CI. Owner 17:26Z: HANDOFF.md stays the hand-written front page with one linked line per subject; lint fails if a dish/tool/mechanism page is not linked from it. Outside-source cards live in research/sources/ (GPU thread, PR #8) and are cited in page `sources:`. Success check run 2026-11-10 at 7 pages (files in Reports/llm-wiki-plan-2026-11-10/check/): wiki tied reading reports directly (0 wrong numbers each, 22 vs 25 correct, p 0.25, 9% fewer tokens); summaries alone failed. Re-run the same questions when the wiki doubles; if it still ties, shrink the wiki to index + decisions + lint. PR #13 adds oats, salt, puzzle dishes, sensors, the look (12 pages); only rts-design is still uncited.

2026-11-11 15:20Z wiki upkeep check (PR #66): fixed Larder World drift in shooter.md/HANDOFF/index; lint now compares `updated` by UTC time when given (e.g. 2026-11-11T15:40Z) and warns on body-linked reports missing from `sources:`. Six structure suggestions put to the owner (world/ folder, mould-tier ladder page, split scale.md, `paused` status for old dishes, slimmer HANDOFF, index line cap); none decided yet.
2026-11-11: success check run 2 (21 pages, 38 Qs) tied again (wiki 36C/2P vs reports 35C/3P; 13% fewer tokens). Owner card 15:28Z: shrink to short pages with NO Numbers tables (index, decisions, lint, short Now pages linking reports). Page caps are bytes (12 KB page, 2 KB Now), `paused` status exists, CI runs lint --base (page must change when its source report changes).
