# Working in this repo

**Read only what your task needs**; this file is the only thing every thread
reads. A runner thread reads nothing else.

- Every lane: `NOW.md` (the milestone and its exit tests, each lane's top
  jobs, what waits on the owner), then `handoff/<your-lane>.md`.
- Game or simulation work: `wiki/index.md` and the page for the mechanism
  you touch.
- Before trying an idea: grep `Reports/dead-ends.md` and `Reports/README.md`
  for it; don't read them whole.
- File layout and tools: `README.md`, when you need them.
- The old front pages (`Reports/handoff-2026-11-17.md`,
  `Reports/roadmap-2026-11-17.md`) and the area handoffs in `handoff/` (every
  file there that is not a lane's) describe the work before 2026-11-17. Read
  them as what others were doing, not as orders: where they differ from this
  file or `NOW.md`, these two hold. The reasons are in
  `Reports/continuation-plan-2026-11-17.md`.

**The wiki** (`wiki/`): a change that adds a report, changes a default
or records an owner decision updates those pages in the same commit; rules
in `wiki/README.md`. Run `python3 wiki/lint.py --base=origin/main` before committing (CI runs it).

## What we are building

The game is the roguelike run in squad mode. Work goes one milestone at a
time, and `NOW.md` names it and its exit tests. Nothing outside the milestone
is built, except by the lab thread in its own example program or branch.

## What the owner has ruled

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
- **Build first, balance later** (2026-11-12 17:26Z; [rule](Reports/testing-practice-2026-11-12.md)).
  A feature or look lands when it works and can be seen; a census is not its
  gate. **Balance never blocks looks or features**: retune the numbers
  instead. A census only answers a named strategy question; name it and its
  runner time first, under about an hour without the owner's OK.
- **Brainstorms start from the model**, not from the demo's tools (lamp,
  oats, salt, knife, water), which were built for the herding puzzle
  (2026-11-10). Describe the simulation neutrally in a brainstorm brief.
- **Nobody asks the owner to playtest** (2026-11-17 17:25Z: "I have
  playtested many times ... I often cannot playtest when a specific thread
  asks. But the game is not unplayed."). He plays often, on his own time.
  Never end work on "awaits the owner's playtest": land it on, say what he
  will see, and move on. His play notes are in `wiki/decisions-game.md`.
- **Everything lands on by default** (2026-11-17). Off is allowed only for a
  measured harm or for one side of a real either-or, at most three at once
  across all lanes, and each ends at his next play: on, one tier's trait, or
  deleted with its `Reports/dead-ends.md` line. A switch that exists only as
  a command-line key is not allowed: every off-by-default key gets a row in
  `src/build/switches.rs`, which the game's Experiments tab shows and
  `try=NAME` turns on from the command line.
- **Every change he can see or feel adds one line** to
  `news/whats-new.txt`, the list the game shows him: `DATE | LANE | what you
  will see | how to try it`, newest at the bottom. The lint fails a change
  to `src/`, `assets/` or the play scripts that adds none, unless a commit
  message carries `Nothing-new: <why>`. Direction owns the wording and
  `news/things-to-try.txt` (at most five lines; ask Direction for one).
- **Decide what can be undone; ask only what cannot** (2026-11-17, card:
  "Threads decide small, ask big"). Look, tuning and implementation choices
  are yours: make them, state them in the "what is new" line, reverse them if
  he objects in play. A question that changes the game's direction goes to
  the Direction lane, not straight to the owner.
- **The look is everyone's job and one lane's call.** He called the visuals
  "an important factor" (2026-11-17). Check a visible change against
  `wiki/looks/art-direction.md` and include a short clip at zoom 6 and 12 in
  the pull request (a page only for a big look, below);
  Look and interface reviews it. His order for look work (2026-11-17): A the
  fight round the squad, B the slime itself, C planets and terrain, D menus,
  shop and screens. An effect is never bigger than its damage (2026-11-14).

## Lanes

Five standing lanes and a lab, each one long-lived thread (2026-11-17).

| Lane | Owns |
|---|---|
| Direction | `NOW.md`, the content of the game's "what is new" and "things to try" lists, the record of the owner's play sessions, `wiki/decisions*.md`, the tuning numbers (prices, clocks, rewards, slime sizes) |
| Enemy | the slime's rules (`src/mould/big.rs`, `src/gpu/engine.wgsl`), `tiers.rs`, `larder_world/{bosses,raids,shapes,bulwark}.rs`, the ladder in `run.rs` |
| Squad and arsenal | `larder_world/{squads,steer,paths,funnel,cover,cmd,shots,buildings,doors,raising}.rs`, the window's `input.rs` and `squadmode.rs`, `rounds.rs`, `guns.rs`, `bullets.rs`, `kit.rs`, the gun sound |
| Look and interface | `src/world/view.wgsl`, `marks.rs`, `sprites/`, `ui.rs`, `start.rs`, `start/`, the window's `menu.rs` and `draw.rs`, `wiki/looks/` |
| Engine and tools | `src/gpu/mod.rs`, `larder_world/{turn,world,score,rules}.rs`, the window's `main.rs`, `headless.rs`, `hands.rs`, `playlog.rs`, CI, `scripts/`, `play.ps1`, `wiki/lint.py` |
| Lab | its own example program or branch; nothing in the main game |

- **One owner per file.** Need a change in another lane's file? Send a small
  pull request and tell the owner through the coordinator. In the two shared
  shaders (`engine.wgsl`: Enemy; `view.wgsl`: Look and interface), other
  lanes change only their own passes and say so in the pull request title. A
  file no lane lists is changed by the lane whose work needs it.
- **Keep `handoff/<your-lane>.md` current** after every merged batch: state,
  open branch, next step. It is what lets a thread be replaced without loss.
- **New slime rules are built on the GPU engine only** and tested on the
  software GPU: no new CPU twin and no new twin test (owner's card,
  2026-11-17 19:21Z). Today's CPU engine (`src/mould/big.rs`) stays as it is
  for the tests and the census that stand on it; whether to remove it is
  decided at Milestone 2.

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
- **Never assign the owner to a pull request or request his review**
  (2026-11-14, kept 2026-11-17). Delete your branch when its PR merges.
- **Models** (2026-11-17): Fable for the main threads, Opus or Sonnet for
  simpler tasks, Haiku for pure runners, at medium to extra high effort by
  the task's complexity and importance.
- **Long runs may use up to 4 Haiku runner threads** (2026-11-10), asked for
  through the coordinator. Runners make no decisions: they run exactly the
  commands given and send the raw output back.
- **Cloud sessions have no GPU**, but the game runs headless on software
  Vulkan (for checking and filming, never for timing):
  `apt-get update && apt-get install -y mesa-vulkan-drivers`, then
  `export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json`. The filming
  kit is in `Reports/whole-game-review-2026-11-15/kit.md`.
- **GPU timing targets a GTX 1070** on the owner's home desktop, through
  Remote Control when that PC is on with the Claude app open (standing OK
  for tests, 2026-11-11). Timings go in one batch per milestone; ask Engine
  and tools.

## Method

1. Look before you measure: film the planet
   (`larder-world look=DIR run=1:PLANET mode=squads hand=greedy at=20,60,120`)
   and look at it.
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

**Pages for big looks only** (his card, 2026-11-19 00:03Z). A small look
lands with its "what is new" line and a still in the pull request, and no
page. A bigger look, one that changes how the slime, the fight or a planet
reads, keeps a before-and-after page. Threads still film to check their own
work. Rejected: a page for every change, and no pages at all.

Pictures and clips go on an Artifact page linked in the thread, with the
counts beside each picture, never as chat attachments. A look is judged in
motion, never from stills alone. Small visual choices are the lane's; a
blind A/B with the key kept off the page is only for a choice that sets the
whole art direction. Save each page's HTML and images under
`Reports/<report-folder>/` and list it in `Reports/pages.md` in the same
change, so the page outlives claude.ai.

## Records

- Write a report only for a measurement others will cite, a dead end, or a
  page for the owner; it gets its line in `Reports/README.md` in the same
  commit. Otherwise the commit message carries the numbers: before, after,
  what was rejected.
- A tried-and-reverted mechanism gets a `Reports/dead-ends.md` entry with the
  condition its rejection depends on.
- Research, harnesses, raw census output and page sources live in this repo,
  not in the project's shared folder (owner, 2026-11-10).
- Source comments record *why*, including approaches tried and reverted.
  Keep them.

## Commands

The owner plays by double-clicking `play.cmd` (runs `play.ps1`: pull main,
build `larder-world` and `larder-edit`, open the menu). Point them at it
rather than at cargo commands.

```
cargo test --release --locked
cargo clippy --all-targets --release --locked -- -D warnings
cargo build --release --locked --bin larder-world   # about 2 min 15 s from clean in the cloud
cargo build --release --example mouldprobe   # examples are not rebuilt by `cargo build`; build the one you run
python3 wiki/lint.py --base=origin/main
```
