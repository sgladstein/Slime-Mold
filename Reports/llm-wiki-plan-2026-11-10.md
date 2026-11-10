# An LLM wiki for the mould game: verdict, plan and review (2026-11-10)

**Status: twelve pages in `wiki/` (seven, then oats, salt, the lesson and
puzzle dishes, sensors and the look after the check); the success check
below found the wiki ties reading the reports directly at seven pages.** Asked for by the owner (2026-11-10 16:40Z): review the repo,
judge whether Karpathy's LLM-wiki idea
(https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) fits it,
and if so plan the design, rules and workflows. He then asked for a
sub-agent review of later ideas and of the plan (16:48Z), and to start with
the first seven pages (16:49Z). The plan was written as a Claude Doc,
https://claude.ai/artifact/SjjCHoyQubEkRxgAhAPjrP; this file is its copy in
the repo. The rules and workflows themselves live in
[`wiki/README.md`](../wiki/README.md), which is the version to follow.

## Verdict

Yes, a small one. The repo holds about 157,000 words of Markdown in about 90
files, but most of that is the 11-07 brainstorm and the 11-08 game folder;
the core mould reports are 10 files and about 22,500 words. They are filed
by date and thread, not by subject, so the current truth about one thing
(the lamp turns up in 9 files, sensor reach in 16, the snip in 41, the storm
in 54) has to be re-assembled by every new session, and reports go stale in
place. The repo already has most of the gist's parts: `CLAUDE.md` is the
schema, `Reports/README.md` the index, `dead-ends.md` a concept page and
`HANDOFF.md` the overview. What was missing is one page per subject saying
what is true now, citing the reports behind it.

Not wanted: an Obsidian vault, a search engine or embeddings (an index is
enough at this size), a `log.md` (git and the commit messages already carry
it), or a second copy of the code.

## Design as built

`wiki/README.md` (rules and workflows), `wiki/index.md` (one line per page,
capped at 150 lines), `wiki/decisions.md` (owner rulings and decided choices,
quoted with times; open decisions stay only in `HANDOFF.md`), `wiki/lint.py`
(mechanical checks, run in CI), `wiki/_template.md`, and seven pages:
Keep It Alive, The Wild One, water and sleep, the lamp, the knife, the
census, and the decisions page. Each subject page has Now, Numbers (each row
linked to its report), Owner's word (dated), Tried and rejected, Open and
Sources, and frontmatter listing its sources and the code symbols it names.
`CLAUDE.md` gained three lines pointing at the wiki.

## The review, and what it changed

A second agent read what people have reported since the gist, then critiqued
the plan. Its sources were opened, not taken from search snippets.

| Suggestion | Evidence | Taken? |
|---|---|---|
| Keep the always-loaded `CLAUDE.md` addition to a few lines; rules in a page read when needed | Measured: Gloaguen et al. 2026, "Evaluating AGENTS.md" (https://www.emergentmind.com/papers/2602.11988): LLM-written context files lowered success about 0.5-2% and raised cost over 20%; human-written ones gained up to about 4% at a cost | Yes |
| Scripted lint for anything checkable, run in CI (CI skips docs-only PRs) | Anecdotal: https://mpt.solutions/karpathys-llm-wiki-works-heres-what-broke-when-i-ran-it-at-real-scale/ (a drift lint caught 9 stale files), https://www.rdworldonline.com/is-karpathys-viral-llm-wiki-helpful-mostly-yes-one-month-in/ | Yes: a CI job; also fails Numbers rows with no source and owner quotes with no date |
| A change is not done while a page still describes the old state | This repo has a case: The Wild One report's status and *What it is* still describe walkers-at-the-bell scoring; only its later *Review 2 fixes* section and `level.rs` score food eaten | Yes, rule 9; the page states the food-share score |
| Each Numbers row carries its dish, hand, seeds, paired count, p | Anecdotal: summaries drop qualifiers (https://effortlessacademic.com/andrej-karpathys-llm-wiki-doesnt-work-for-academics/) | Yes |
| Size to the 10 core reports; a page only when it adds a synthesis | Opinion | Yes: 7 pages, candidates listed in the index |
| One home for open decisions | The plan had two lists | Yes: `HANDOFF.md` |
| Let reports grow by dated sections rather than freeze | That is how they grow today (the probe report has 11 commits) | Yes, rule 1 reworded |
| Cut `log.md`; don't shorten `Reports/README.md` yet | Git already logs; the README entries are the most-read summaries | Yes |
| Leave `handoff/project-memory/` frozen | It is a snapshot of the old account | Yes |
| Strengthen the success check | 12 paired questions would pass a real 70/30 advantage less than half the time | Yes, below; not run |

Also from the review, not acted on: a pattern of rules injected at edit time
being followed 8 of 8 against 0 of 8 for rules kept on wiki pages (a small,
self-reported test in the gist thread); the lint is this repo's version of
that.

## Success check (run 2026-11-10, seven pages)

25 questions were written from past questions (the hand-off's next steps,
review 2's checks) with an answer key, before the next pages. Each was
answered once by a fresh agent in a copy of the repo, three ways: **wiki**
(the repo as it is, wiki included), **reports** (the same repo with `wiki/`
removed), and **summaries** (`HANDOFF.md`, `README.md` and
`Reports/README.md` only). A fourth agent scored the three answer sets
against the key under shuffled labels (C correct, P partial, W wrong,
N not found) and counted numbers that contradict the key. Deciding measure,
named before the run: wrong numbers quoted. Files are in
[`llm-wiki-plan-2026-11-10/check/`](llm-wiki-plan-2026-11-10/check/scores.tsv)
(`blind-key.txt` maps X/Y/Z to arms).

| Arm | Correct | Partial | Wrong | Not found | Wrong numbers | Files read | Tokens | Time |
|---|---|---|---|---|---|---|---|---|
| wiki | 22 | 3 | 0 | 0 | 0 | 10 | 69,666 | 41 s |
| reports | 25 | 0 | 0 | 0 | 0 | 7 | 76,758 | 57 s |
| summaries | 4 | 11 | 1 | 9 | 1 | 3 | 56,221 | 26 s |

Paired by question (correct 2, partial 1, else 0), two-sided sign test, three
comparisons (Bonferroni bar 0.017): wiki beats summaries 18-0 (p 0.000008),
reports beat summaries 21-0 (p 0.000001), and **wiki against reports is 0-3
with 22 ties (p 0.25)**. Only the first two survive.

What it says. At this size the wiki **ties reading the reports directly**:
no wrong numbers either way, about 9% fewer tokens and 29% less time, and
three answers thinner because a page had dropped a detail its report keeps
(why a shorter reach senses less, that The Wild One's lead moved by up to
half at the bell, and the warden with and without the snip, 4-4 and 3-5).
Those three details were added to their pages in the same change. The
hand-off and report index alone are not enough (9 of 25 not found). The
plan's bar ("if the control ties the wiki, it is not paying for itself") is
not met yet: the corpus is about ten reports, small enough for one agent to
read the right three in under a minute. One run per arm, so agent-to-agent
variance is not measured. Re-run the same questions, plus new ones on the
new pages, when the wiki doubles (rule in `wiki/README.md`, *Growth*); if
the tie holds then, shrink the wiki to the index, decisions and lint.

## Growth (owner, 17:24Z)

"I agree with the plan, but note that this repo is brand new so it will grow
significantly." The wiki now grows by triggers in `wiki/README.md`, *Growth*:
lint fails on any new report no page cites; pages past about 300 lines split;
the index past 150 lines splits per folder; search is added past about 100
pages or when sessions miss pages; the success check runs while the wiki is
small and again after it doubles. External sources (papers, outside
simulations) get their own reference cards in `research/sources/` (PR #8,
another thread); wiki pages list card paths in `sources:` like reports.

## Open for the owner

- ~~Keep the lint in CI?~~ Owner, 17:25Z: "lint can stay in the CI for now."
- ~~What becomes of `HANDOFF.md`?~~ Owner, 17:26Z, "Sure" to the
  recommendation: it stays the hand-written front page, its status section
  becomes one linked line per subject, and it keeps the open decisions and
  next steps. Lint fails when a dish, tool or mechanism page is not linked
  from it.
