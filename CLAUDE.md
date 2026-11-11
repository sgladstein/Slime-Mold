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
in `wiki/README.md`. Run `python3 wiki/lint.py` before committing (CI runs it).

## What the owner has ruled for this game

All rulings, quoted and timed: `wiki/decisions.md`.

- **Fun over fidelity.** The goal is interesting simulations that can be
  turned into a game, not a realistic slime mould (2026-11-10). Biology and
  Jones's papers are inspiration, not the bar; judge every change by play.
- **Everything on screen is the simulation, and looks like it** (2026-11-09,
  2026-11-10): no drawn-only effects (no painted pulse), no thresholded tube
  edge or fed/unfed colour line; the picture *is* the data.
- **An outcome is a spread, not a yes/no; every tool must deliver something
  you can see.**
- **Defaults on** unless there is a measured harm or a stated uncertainty.
- **Brainstorms start from the model**, not from the demo's tools (lamp,
  oats, salt, knife, water), which were built for the herding puzzle
  (2026-11-10). Describe the simulation neutrally in a brainstorm brief.

## How threads work

- **Each thread merges its own PRs** (2026-11-10): once both CI jobs are
  green on the exact head, with a merge commit. Docs-only PRs (`Reports/`,
  `handoff/`, `research/`, `wiki/`, `*.md`) skip the build jobs; skipped is
  green, but the wiki lint still runs.
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
3. Random and idle hands are the controls: a good dish is one a plan wins
   and random loses.
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

```
cargo test --release --locked
cargo clippy --all-targets --release --locked -- -D warnings
cargo build --release --example mouldprobe   # examples are not rebuilt by `cargo build`; build the one you run
python3 wiki/lint.py
```
