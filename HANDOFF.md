# Hand-off: the mould game at the account move (2026-11-10)

On 2026-11-10 the owner moved all slime-mould work to a new account and
project. This page is the single starting point for whoever picks it up
there. It says where the game stands, what the owner still has to decide,
and what to do next. Everything it points to is in this repository; nothing
lives only in the old project.

**Read first:** this page, then [`CLAUDE.md`](CLAUDE.md) (how to work here),
then the report for whatever you touch ([`Reports/README.md`](Reports/README.md)
indexes them all).

## Where the game stands

`cargo run --release --bin mould` is the game. Everything on screen is the
simulation (the owner's ruling, 11-09 and 11-10: no drawn-only pulse, no
thresholded tube edge, no fed/unfed colour line; the picture is the data).

- **Dishes.** An opening lesson, then five challenge dishes: Behind the Wall,
  Beat the Ruler, Running Dry, **Keep It Alive** (the endless dish: oats land
  one after another, farther each time, and you keep the mould alive with
  lamp, oats and water) and **The Wild One** (a wild violet mould steals food
  and fights yours). Free dishes and the Wanderer and Organ prototypes are
  there too.
- **Tools.** Lamp (pushes the mould away and dries the ground), oats, salt,
  the knife (snip) and water (wet ground lets mould sleep through hunger
  instead of dying).
- **Look.** The from-the-simulation look is the default (owner 11-10 14:49Z:
  "much better"): trail as brightness, hunger blended, a dot per walker. F7
  brings back the old tube look for comparison.
- **Model defaults.** Sensor angle 45 degrees, sensor reach 9 (finer tubes at
  reach 6 were tried and not kept), food flow off (the T panel turns it on).
  Keep It Alive and some dishes run at 22.5 degrees, which branches more;
  `Params::settle_seconds` (off) eases 22.5 to 45.
- **Scoring.** The Wild One scores your share of the food eaten over the
  whole game (PR #6), because counting mould left at the bell let the last
  ten seconds decide the stars. Keep It Alive scores how long it lives.
- **How good is it as a game?** Review 2 found both new dishes work and
  their numbers reproduce, but neither is yet a game you get better at by
  playing: Keep It Alive's lives fall in two humps (dead by about 5:00 or
  past 7:20) largely by luck, and The Wild One was an opening salt wall then
  an aim test. PR #6 answers the scoring half of that for The Wild One.

## Decisions the owner has not made

Ask each again; none has an answer to assume.

1. **What to fix first after review 2.** Asked on a card 11-10 15:12Z, never
   answered. The Wild One's scoring has since been done (PR #6), so what is
   left of the question is whether to trace Keep It Alive's luck next or wait
   until the owner has played both dishes.
   [`Reports/mould-review-thread-handoff-2026-11-10.md`](Reports/mould-review-thread-handoff-2026-11-10.md)
2. **Grains or glints** for walkers in the tube look. Only matters while F7
   is kept. [`Reports/mould-looks-2026-11-10.md`](Reports/mould-looks-2026-11-10.md)
3. **Retire the tube look (F7)?** The owner chose to keep a key back; ask
   before deleting. Same report.
4. **Food flow:** on or off. It is real, off by default, and has no verdict
   from the owner's hand. [`Reports/mould-probe-2026-11-07.md`](Reports/mould-probe-2026-11-07.md), "Food that flows"
5. **The Organ and the Wanderer:** keep, change or drop. Neither has a
   verdict.

Decided and not yet built: **the forest floor from above** (owner's card,
11-10 16:04Z) as the first out-of-the-dish game. What it needs, in this repo
with no engine, is in §5 of
[`Reports/mould-real-world-2026-11-09.md`](Reports/mould-real-world-2026-11-09.md).

## Next steps, in order

1. **Keep It Alive: trace individual games** to find what puts one in the
   short hump or the long one, then give that choice to the player. Review 2's
   untested guess: whether sleeping mould is within reach when a far pile lands
   (about the twelfth pile, gap past 140 cells).
2. **The Wild One: make the middle of the game matter.** Late food on the seam
   was tried and added luck, not play (`Reports/dead-ends.md`). Separate which
   of lamp, snip and oats needs the precision before trying again.
3. **The snip:** make a cut-off piece visibly starve, or take the snip off
   The Wild One. Three cuts through the wild nest change nothing on 8 of 8
   seeds.
4. **Star bars last**, re-checked on 16 or more seeds once the above have
   changed what a game is.
5. **The forest floor from above** (decided, above).
6. **Later, if those play well** (owner-agreed order for depth): melting salt,
   spores, a rival the player steers, same-kind moulds joining. Draw anything
   new in the simulation look's way, from a held quantity through one fixed
   ramp.

Each report's own "next steps" section has the reasons and numbers:
[`Reports/mould-wild-one-2026-11-10.md`](Reports/mould-wild-one-2026-11-10.md),
[`Reports/mould-review-thread-handoff-2026-11-10.md`](Reports/mould-review-thread-handoff-2026-11-10.md),
[`Reports/mould-looks-2026-11-10.md`](Reports/mould-looks-2026-11-10.md),
[`Reports/mould-keep-alive-2026-11-10.md`](Reports/mould-keep-alive-2026-11-10.md).

## How the work has been done

- **Judge by play and by paired counts.** Every dish is measured as a
  scripted hand against doing nothing and against random on the same seeds
  (`examples/mouldprobe.rs`), with a two-sided sign test on the pairs. A
  difference is only called when it passes; say how many comparisons were
  made. The owner's play overrules a table.
- **Show, don't describe.** Pictures go on an Artifact page linked in the
  thread, never as chat attachments, with the event counts beside each image.
  Every page made so far is listed in [`Reports/pages.md`](Reports/pages.md)
  with its archived source.
- **Merging.** Merge a PR only when both CI jobs ("test + clippy" and "sound
  device builds (organ)") are green on its exact head, with a merge commit.
  PRs that change only `Reports/`, `handoff/`, `research/` or `*.md` skip the
  two build jobs (PR #5), so they show as skipped, which is green.
- **Reaching the repo from a session.** Add `sgladstein/Slime-Mold` to the
  session (`add_repo`); the crate is `slime-mould`, imported as
  `slime_mould::mould::...`.
- **Pixel_Physics is a separate project now.** The mould code was removed
  from it (its PR #1064); the history of every file came across here. Seep,
  the gas-eating mould in Pixel_Physics, is a different game and stayed.

## What the old project held, and where it is now

- **Project memory notes** about the mould game: copied as they stood at the
  move into [`handoff/project-memory/`](handoff/project-memory/README.md).
  Their thread ids (`cmsg_...`) and claude.ai artifact links point into the
  old account and will not open from the new one; every page's source is
  archived in this repo (see `Reports/pages.md`).
- **Research papers:** the two Jones papers are in
  `Reports/mould-game-2026-11-08/depth-2026-11-09/papers/`, kept by the
  owner's choice; they are the publishers' copyright and for reading only.
- **Project folder files** (briefs, prototype pictures, scratch runs): all in
  `Reports/`. Nothing mould-related was left only in the old project folder.
