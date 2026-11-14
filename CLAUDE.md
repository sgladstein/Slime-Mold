# Working in this repo

**Read only what your task needs**; this file is the
only thing every thread reads. A runner thread reads nothing else.

- Game or simulation work: `HANDOFF.md`, then `wiki/index.md` and the page
  for the dish, tool or mechanism you touch.
- Planning, or changing a step's state: `ROADMAP.md`.
- Before trying an idea: grep `Reports/dead-ends.md` and `Reports/README.md`
  for it; don't read them whole.
- File layout and tools: `README.md`, when you need them.

**The wiki** (`wiki/`): a change that adds a report, changes a default
or records an owner decision updates those pages in the same commit; rules
in `wiki/README.md`. Run `python3 wiki/lint.py --base=origin/main` before committing (CI runs it).

## What the owner has ruled for this game

All rulings, quoted and timed: `wiki/decisions.md` (standing) and
`wiki/decisions-game.md` (per game).

- **Fun over fidelity.** The goal is interesting simulations that can be
  turned into a game, not a realistic slime mould (2026-11-10). Biology and
  Jones's papers are inspiration, not the bar; judge every change by play.
- **Everything on screen is the simulation, and looks like it** (2026-11-09,
  2026-11-10): no drawn-only effects (no painted pulse), no thresholded tube
  edge or fed/unfed colour line; the picture *is* the data.
- **An outcome is a spread, not a yes/no; every tool must deliver something
  you can see.**
- **Defaults on** unless there is a measured harm or a stated uncertainty.
- **Build first, balance later** (2026-11-12 17:26Z; [rule](Reports/testing-practice-2026-11-12.md)).
  A feature or look lands when it works and can be seen (tests plus a GIF or
  a playable build); a census is not its gate. **Balance never blocks looks
  or features**: retune the numbers instead, and never hold one area's work
  for balance in another. A census only answers a strategy question (player
  AI, a shared default every thread measures against, dominance before a
  milestone or playtest, mission difficulty, an owner question); name it and
  its runner time first, under about an hour without the owner's OK.
- **Brainstorms start from the model**, not from the demo's tools (lamp,
  oats, salt, knife, water), which were built for the herding puzzle
  (2026-11-10). Describe the simulation neutrally in a brainstorm brief.

## How threads work

- **Each thread merges its own PRs through the queue** (2026-11-10; queue
  2026-11-14). Add the label `queue` to the PR:
  `gh api repos/sgladstein/Slime-Mold/issues/N/labels -f 'labels[]=queue'`
  (MCP `issue_write` `labels` replaces the PR's other labels; GraphQL, so
  `gh pr merge --auto`, is blocked here). Don't merge main in by
  hand to re-run CI: `.github/workflows/merge-queue.yml` merges up to five
  queued PRs onto current main with merge commits, runs CI once on that
  candidate and fast-forwards main when it is green, so what lands is what
  was tested (11-13: #165 and #174 were each green alone and together broke
  main). A conflict or a red PR gets its label removed and a comment: fix
  it and label it again. Merging by hand still works but makes the queue
  rebuild. Docs-only changes (`Reports/`, `handoff/`, `research/`, `wiki/`,
  `*.md`) skip the build jobs; the wiki lint still runs.
- **Long runs may use up to 4 Haiku runner threads** (2026-11-10), asked for
  through the coordinator. Runners make no decisions: they run exactly the
  commands given and send the raw output back.
- **GPU work targets a GTX 1070** on the owner's home desktop, which is often
  not the machine they are at. GPU runs go through Remote Control when that
  PC is on with the Claude app open; the owner gave a standing OK for tests
  there (2026-11-11).

## Method

1. Look before you measure: render the dish (`mouldprobe -- look=DIR`,
   `mouldlooks`) and look at it.
2. Name the deciding measure before a run. Compare paired seeds, quote
   differences as paired counts with a sign-test p, and say which survive a
   multiple-comparison correction.
3. Every feature gets cheap smoke checks: idle doesn't win, the game doesn't
   end in under a minute, nothing soft-locks. Random and idle hands are the
   controls when a census does run; a result there is a tuning note, not a
   veto.
4. Quote the frame cost of anything that runs every step.
5. Before retrying an idea, grep `Reports/dead-ends.md` for the mechanism.

## Showing the owner

Pictures and GIFs go on an Artifact page linked in the thread, with the
counts beside each picture, never as chat attachments. A visual choice goes
up as a blind A/B with the key kept off the page. Save each page's HTML and
images under `Reports/<report-folder>/` and list it in `Reports/pages.md`
in the same change, so the page outlives claude.ai.

## Records

- A new report gets its line in `Reports/README.md` in the same commit.
- A tried-and-reverted mechanism gets a `Reports/dead-ends.md` entry with the
  condition its rejection depends on.
- Research, harnesses, raw census output and page sources live in this repo,
  not in the project's shared folder (owner, 2026-11-10).
- Commit messages carry the measurement: before, after, what was rejected.
- Source comments record *why*, including approaches tried and reverted.
  Keep them.

## Commands

The owner plays by double-clicking `play.cmd` (runs `play.ps1`: pull main,
build `larder-world` and `larder-edit`, open the menu). Point them at it
rather than at cargo commands.

```
cargo test --release --locked
cargo clippy --all-targets --release --locked -- -D warnings
cargo build --release --example mouldprobe   # examples are not rebuilt by `cargo build`; build the one you run
python3 wiki/lint.py --base=origin/main
```
