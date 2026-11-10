# Working in this repo

The mould game moved here from Pixel_Physics on 2026-11-10. Pixel_Physics's
`CLAUDE.md` holds the project's long-learned method; the parts that apply
here are kept below, short. Read `README.md` for the layout and
`Reports/README.md` for what has already been tried.

## What the owner has ruled for this game

- **Fun over fidelity.** Real slime mould biology and Jones's papers are
  inspiration, not the bar. Judge every change by what it does for play.
- **Everything on screen is the simulation, and looks like it** (2026-11-09,
  2026-11-10): no drawn-only effects (no painted pulse), no thresholded tube
  edge or fed/unfed colour line; the picture *is* the data.
- **An outcome is a spread, not a yes/no; every tool must deliver something
  you can see.**
- **Defaults on** unless there is a measured harm or a stated uncertainty.

## Method

1. Look before you measure: render the dish (`mouldprobe look=DIR`,
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
images under `Reports/<report-folder>/` in the same change, so the page
outlives claude.ai.

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
cargo test --release
cargo clippy --all-targets --release --locked -- -D warnings
cargo build --release --example mouldprobe   # examples are not rebuilt by `cargo build`; build the one you run
```
