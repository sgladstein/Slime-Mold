# Hand-off: where the mould game stands

The single starting point for whoever picks up the mould game. It says where
the game stands in one line per subject, what the owner still has to decide,
and what to do next. It is written by hand, not generated: the facts behind
each line live on the [wiki](wiki/index.md) pages it links, so update a line
here when its page's **Now** changes, and keep the detail there (owner,
2026-11-10 17:26Z; [`wiki/decisions.md`](wiki/decisions.md)). It began as the
hand-off at the 2026-11-10 account move; everything it points to is in this
repository.

**Read first:** this page, then [ROADMAP.md](ROADMAP.md) (every plan in one
order), then [`CLAUDE.md`](CLAUDE.md) (how to work here),
then the [wiki page](wiki/index.md) for whatever you touch, then its
reports ([`Reports/README.md`](Reports/README.md) indexes them all).

## Where the game stands

`cargo run --release --bin mould` is the game. Everything on screen is the
simulation; the owner's rulings are in [`wiki/decisions.md`](wiki/decisions.md).
**Neither new dish has been played by the owner's hand yet.**

- [Keep It Alive](wiki/dishes/keep-it-alive.md): the endless dish works, but
  which of two humps a game lands in (dead by about 5:00, or past 7:20) is
  luck so far: traced, it turns on whether the mould sleeps near the next
  landing, which shows only 5 s ahead.
- [The Wild One](wiki/dishes/the-wild-one.md): scored by your share of the
  food eaten; the first star is the opening salt wall, the rest is aim.
- [The lesson and the three puzzle dishes](wiki/dishes/puzzle-dishes.md)
  (Behind the Wall, Beat the Ruler, Running Dry): pass their bar at 45
  degrees; review 1 would keep them as a side pack.
- Tools: [oats](wiki/tools/oats.md) lead, [the lamp](wiki/tools/lamp.md)
  pushes and dries, [salt](wiki/tools/salt.md) walls off (it is The Wild
  One's first star), [the knife](wiki/tools/knife.md) moves nothing in The
  Wild One; [water and sleep](wiki/model/water-and-sleep.md) carry mould
  through famine in Keep It Alive.
- [The simulation look](wiki/looks/simulation-look.md) is the default, F7
  the old tube look.
- [Sensor angle and reach](wiki/model/sensors.md): 45 degrees by default,
  22.5 on Keep It Alive and The Wild One, reach 9. Food flow is off; no
  page yet.
- Scale: [the mould at scale](wiki/model/scale.md). The dish holds a few dozen
  loops where Jenson's GPU version holds hundreds. GPU plan phase 1 is under
  way on the CPU: a late census is safe; fixed-point trail and scent change
  play (the fields stay floats); moving all walkers at once cuts fights
  where two moulds meet. Later gates use engine measures, not today's dishes.
  The scaling rule is chosen (today's numbers in fine cells, food smelled at
  today's reach) and the big dish runs offline on the CPU (`mould::big`):
  at 1024 x 640, 32,000 walkers make about 60 loops and 100,000 about 90-160,
  against today's 3 ([page](https://claude.ai/artifact/WMfUwTj9Dfdwp9MCfy8MHs)).
  Next: the GTX 1070 timing, then the owner's scale gate.
- How it is judged: [the census](wiki/method/census.md).

## Decisions the owner has not made

Ask each again; none has an answer to assume.

1. ~~What to fix first after review 2.~~ **Decided** 11-10 18:56Z (card:
   "Trace it now"); the trace is done
   ([`Reports/mould-keep-alive-luck-2026-11-10.md`](Reports/mould-keep-alive-luck-2026-11-10.md)),
   see [`wiki/decisions.md`](wiki/decisions.md). The 20 s landing preview it
   proposes changes the game and is not yet agreed. Kept as a numbered line
   so the later numbers stay the same.
2. **Grains or glints** for walkers in the tube look. Only matters while F7
   is kept. [`Reports/mould-looks-2026-11-10.md`](Reports/mould-looks-2026-11-10.md)
3. **Retire the tube look (F7)?** The owner chose to keep a key back; ask
   before deleting. Same report.
4. **Food flow:** on or off. It is real, off by default, and has no verdict
   from the owner's hand. [`Reports/mould-probe-2026-11-07.md`](Reports/mould-probe-2026-11-07.md), "Food that flows"
5. **The Organ and the Wanderer:** keep, change or drop. Neither has a
   verdict.
6. **The GPU plan** ([`Reports/gpu-plan-2026-11-10.md`](Reports/gpu-plan-2026-11-10.md),
   [page](wiki/model/scale.md)), now started (phase 1): integer or floats is
   measured: fixed-point trail and scent changed play, so the recommendation
   is now floats for the fields (integer positions still possible); upload
   Jones 2011 before phase 4; hold the Pixel_Physics multi-core renderer port
   until the phase 2 gate. Phase 2's gate is next: after the GTX 1070 timing
   (when the owner is at Scott-Desktop), pick the target scale and look
   (32,000 or 100,000 walkers on 1024 x 640), or stop.
7. **Bloomfront, the RTS** ([`Reports/rts-design-2026-11-10.md`](Reports/rts-design-2026-11-10.md),
   *Risks and open questions*): should it wait for the queued work or take
   over The Wild One's next steps (its Stage 0 answers the same mid-game
   question); is a fog of war allowed if it is drawn from a held quantity;
   two keepers, or three-to-four. Since the brainstorm
   ([`brainstorm/`](Reports/rts-design-2026-11-10/brainstorm/README.md)) and
   its [red team](Reports/rts-design-2026-11-10/red-team/README.md): which
   direction to probe first. The red team recommends a paired-clone verb
   census (one verb in one of two cloned dishes, 16 seeds) before Bloomfront's
   Stage 0, whose gates would pass without testing the opening; then one
   contagion strain on the CPU. The census has since run, with a retest on a
   23-pile field after the owner said 7 piles limit the options
   ([`field/`](Reports/rts-design-2026-11-10/field/README.md)): a salt line
   across the rival's road passes on the field, only the lamp moves share on
   The Wild One, Taut's pull wins on the field, and the pattern counters form
   a cycle on one field layout of two. Next: one contagion strain, and
   whether any verb can overturn a 60 s lead. Two Fevers is still the
   strongest direction. The food store sits at a median of 0, so any "pay
   from the store" rule needs its own currency first. Page:
   [the RTS directions](wiki/rts/directions.md).

Decided and not yet built: **the forest floor from above** (owner's card,
11-10 16:04Z) as the first out-of-the-dish game. What it needs, in this repo
with no engine, is in §5 of
[`Reports/mould-real-world-2026-11-09.md`](Reports/mould-real-world-2026-11-09.md).

## Next steps, in order

How these sit against the GPU plan, the Pixel_Physics ports and the RTS,
with what each waits on and which thread owns it: [ROADMAP.md](ROADMAP.md).

1. **Keep It Alive: give the player the choice that decides the hump.**
   Traced: the first pile landing on mostly-asleep mould decides it, by
   whether a sleeper is within water reach. Next: a 20 s landing preview and a
   hand that uses it, on fresh seeds
   ([`Reports/mould-keep-alive-luck-2026-11-10.md`](Reports/mould-keep-alive-luck-2026-11-10.md)).
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
  PRs that change only `Reports/`, `handoff/`, `research/`, `wiki/` or `*.md` skip the
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
