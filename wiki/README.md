# The wiki: rules and workflows

The wiki is one page per subject (a dish, a tool, a mechanism, a method)
saying what is true **now**, with every number linked to the report it came
from. Reports in `Reports/` stay the record of what was tried and measured,
in the order it happened; the wiki is the summary a new session reads first.
It follows Karpathy's LLM-wiki pattern
(https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f), cut to
this repo's size: about ten core reports, so an index is enough and there is
no search engine. The plan and its review: [`Reports/llm-wiki-plan-2026-11-10.md`](../Reports/llm-wiki-plan-2026-11-10.md).

Start at [index.md](index.md). Copy [_template.md](_template.md) for a new page.

## Rules

1. **Reports keep their history.** A later finding is a new report, or a new
   dated section appended to the old one. Never rewrite a past section's
   numbers; correct it with a dated note. Pages cite sections by name.
2. **Short pages; the numbers live in the reports** (owner's card,
   2026-11-11 15:28Z, after the second success check tied). A page has no
   Numbers table: its Now gives the headline result with its conditions in a
   sentence and links the report section that holds the table. A number that
   is in no report does not go in the wiki.
3. **The wiki holds no new measurement.** A question that needs a run gets a
   report first; the wiki then cites it. A synthesis of existing reports may
   become a page.
4. **The code wins.** Pages name the symbol and its default rather than
   restate code; `code:` in the frontmatter lists them, and lint fails when
   one disappears from `src/` or `examples/`.
5. **The owner's words are quoted, with the date and time.** Never firmer
   than he said. Open decisions live only in `HANDOFF.md`; decided ones move
   to [decisions.md](decisions.md) (standing rulings) or
   [decisions-game.md](decisions-game.md) (choices about a game) with the quote.
6. **Extend, don't create.** Check the index first. Write a page only when
   someone has a synthesis to add that a second report would otherwise
   repeat; one page per entity is not a goal.
7. **Never overwrite silently.** When a newer report disagrees, keep both,
   dated, and say in **Now** which holds and why.
8. **Rewrite Now; don't append to it.** History stays in
   the reports and git.
9. **Same commit.** A change that adds a report, changes a default or records
   an owner decision updates the affected pages and the index in that commit.
   A change is not done while a page's Now still describes the old state.
10. **Pictures stay on Artifact pages.** Pages link the archived copies listed
    in [`Reports/pages.md`](../Reports/pages.md).
11. **Status** (frontmatter `status:`): `current` (settled), `open` (work
    under way), `decided`, `paused`, `retired`. **Paused** marks a subject the
    owner has stepped back from (the dishes and their tools, 2026-11-10
    20:55Z): don't tune it as a side effect of other work, but a thread that
    is working on that subject directly may change it and its page (owner,
    2026-11-11 15:17Z). Lint fails on any other value.
12. **Fun over fidelity.** Biology and Jones's papers are inspiration; never a
    reason a mechanism must stay.

## Growth

The repo is new and will grow a lot; the wiki is built to grow with it, by
these triggers rather than by a plan for its final size:

- **Every new report is ingested.** Lint fails on a report no page cites
  (reports from before the wiki are listed in `lint.py`). A report that
  touches no existing subject gets a new page or a line under an existing one.
- **A page past 12 KB (about 3,000 tokens) splits** into a subject page and a sub-page
  (for example a dish's tuning history), linked both ways.
- **The index past 150 lines splits** into one index per folder
  (`dishes/index.md`, ...), with `index.md` listing the folders. Lint enforces
  the cap.
- **Past about 100 pages, or once sessions start missing pages that exist**,
  add search: a grep-based `wiki/find.py` first, a local search tool such as
  qmd (named in Karpathy's gist) only if that is not enough.
- **New folders** (`looks/`, `prototypes/`, a folder per new game such as the
  forest floor) are added when their first page is.
- **The success check** (`Reports/llm-wiki-plan-2026-11-10.md`) runs while the
  wiki is small, so its worth is known before it is big, and again after it
  doubles. Run 2 (2026-11-11, 21 pages): still a tie with reading the
  reports (3-2, 33 ties; 13% fewer tokens), so its shrink rule applies.

## Workflows

**Outside sources** (papers, other people's simulations and code) have
reference cards in `research/sources/`, one per source. A page that draws on
one lists the card's path in `sources:` next to the reports; lint checks that
it exists.

**Ingest** (in the same PR as a report or a default change): read the
report; list the pages it touches; rewrite each page's Now, add the report to
Sources, bump `updated`; link any new `dead-ends.md` entry; update the
page's index line; run `python3 wiki/lint.py`.

**Notes behind a report** (a sub-agent's full write-up, a brainstorm, a
red-team review, raw counts) are kept whole in a folder beside the report,
`Reports/<report>/`, and are **not ingested one by one**. The parent report
(`Reports/<report>.md`, or the folder's `README.md`) summarises them and
links every note, and pages cite the parent. A page cites a note directly
only when it uses a number or a quote that only the note holds. So every
note is one hop from the wiki; lint fails on a note no parent or page
reaches, and on a folder of notes with no parent (owner's question,
2026-11-10 18:48Z: "Do the agents full notes get ingested into the wiki?").

**Query**: `HANDOFF.md`, then [index.md](index.md), then the page; open a
report only when the page does not answer.

**Lint** (CI runs it on every PR, with `--base`; run `python3 wiki/lint.py
--base=origin/main` before pushing): with `--base` it fails a page whose
source report the PR changed while the page did not change (ingest it, or
bump `updated` once you have checked it is still true). It also fails on broken links, pages missing from the index,
code symbols that no longer exist, required frontmatter missing, and an owner quote with no date. It warns when a
page is older than one of its sources (by git date, or by UTC time when
`updated` carries one, such as `2026-11-11T15:40Z`; use the time on a busy
day, since a date misses a source changed later the same day), when a page
links a report its `sources:` do not list (so the age check cannot see it),
and lists reports no page cites yet. Then read the warned pages for the part a script cannot do:
a page whose Now a newer report overturned, or two pages that disagree.

**Owner decision**: quote it with its time in [decisions.md](decisions.md)
or [decisions-game.md](decisions-game.md),
remove it from `HANDOFF.md`'s open list, update the pages it changes.

**Hand-off**: `HANDOFF.md` is the hand-written front page: one linked line
per subject, the open decisions, the next steps in order. Update a line when
its page's Now changes; lint fails when a dish, tool or mechanism page is not
linked from it. At a hand-off, run lint, then re-read each line against its
page.
