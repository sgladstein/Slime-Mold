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

One line per subject; the page holds the detail. The owner's rulings:
[`wiki/decisions.md`](wiki/decisions.md).

**Live work: Larder World.** A squad against several mould species in a
persistent world on the GPU big dish, with energy crystals that drain
(owner, 11-11 14:06Z: "Put everything into the world version").

- [Larder World](wiki/world/larder.md): phases W0-W5; W1a's crystals, terrain and walls are on the CPU big dish; W0 [growth and starving](wiki/model/growth.md) is built on the CPU and the GPU (PR #73, after #63); next W1b, those fields on the GPU, then Larder in the world ([roadmap](Reports/larder-world-roadmap-2026-11-11.md)).
- [The mould tiers](wiki/world/mould-tiers.md): the difficulty ladder; Sporer, Sleeper and Swarmer built ([the life cycle](wiki/model/life-cycle.md)).
- [The player's own mould](wiki/world/own-mould.md): gardeners (owner, 11-12): soldiers grow a second species that fights the enemy by conversion; a probe found forms trade eating for fighting; the CPU build (`own_mould.rs`, `examples/garden.rs`) grows it by sowing but it does not yet shrink the enemy; contact is next.
- [The squad toolkit](wiki/world/squad-toolkit.md): blast and its crater, mine, det cord, germ, sleep gas, smell, on the CPU world; each tool shows in the mould; plans beat idle and random; with tools carried from the larder the best plan changes by tier (blasts on some, the germ on others).
- [The Larder player AI](wiki/world/player-ai.md): plays through the player's commands; `cargo run --release --bin larder -- 17 ai=lure`.
- [The world camera and UI](wiki/world/world-camera.md): pan, zoom, minimap, GPU marks, Larder's controls and the game UI (`world::ui`: fill a `Model`, handle `HudEvent`s); the engine is `slime_mould::gpu`; sandbox W, Q, M; view 0.3-0.4 ms, UI 0.1 ms a frame on the 1070.
- [The mould at scale](wiki/model/scale.md): GPU phases 1-3 done; phase 4 is W0. [The GPU sandbox](wiki/model/gpu-sandbox.md): `cargo run --release --bin mouldgpu`. [Outside looks and creature rules](wiki/model/outside-looks.md): species, droplets, trunks, contagion.
- [The field skirmish](wiki/rts/skirmish.md) and [Two Fevers](wiki/rts/two-fevers.md): playable RTS demos where one strategy still wins; [the RTS directions](wiki/rts/directions.md).
- How work is judged: [the census](wiki/method/census.md).

**Paused: the dish game** (`cargo run --release --bin mould`; owner, 11-10
20:55Z: unlikely to be the true games). Not tuned as a side effect of other
work; neither new dish has been played by the owner's hand.

- [Keep It Alive](wiki/dishes/keep-it-alive.md), [The Wild One](wiki/dishes/the-wild-one.md), [the lesson and puzzle dishes](wiki/dishes/puzzle-dishes.md).
- Tools: [oats](wiki/tools/oats.md), [the lamp](wiki/tools/lamp.md), [salt](wiki/tools/salt.md), [the knife](wiki/tools/knife.md); [water and sleep](wiki/model/water-and-sleep.md); [sensor angle and reach](wiki/model/sensors.md).
- [The simulation look](wiki/looks/simulation-look.md) (default; F7 the tube look) and [sound](wiki/looks/sound.md).

## Decisions the owner has not made

Ask each again; none has an answer to assume. Numbers are kept so older
references stay right.

1. ~~What to fix first after review 2.~~ Decided 11-10 18:56Z ("Trace it now"); the 20 s landing preview it proposes is not agreed.
2. **Grains or glints** for walkers in the tube look (only while F7 is kept). [`Reports/mould-looks-2026-11-10.md`](Reports/mould-looks-2026-11-10.md)
3. **Retire the tube look (F7)?** Ask before deleting. Same report.
4. **Food flow:** on or off; off by default, no verdict. [`Reports/mould-probe-2026-11-07.md`](Reports/mould-probe-2026-11-07.md), "Food that flows"
5. **The Organ and the Wanderer:** keep, change or drop.
6. **The GPU plan:** upload Jones 2011 before phase 4 (W0); hold the Pixel_Physics multi-core renderer port. ([page](wiki/model/scale.md)) The 1070 differs from the CPU in `sin`, `cos` and `sqrt` and fuses FMA (754246d; [report, *On the GTX 1070*](Reports/gpu-readout-2026-11-12.md)): cross-machine replay or lockstep needs integer or table maths. The CPU sim doesn't reproduce across Windows and Linux either: f32 `sin`, `cos` and `exp` come from the platform maths library (MSVC or glibc), so CPU replays, hashes and seed results differ between the two (the skirmish bank test failed on the PC for this, 2026-11-12). If cross-machine CPU replays or lockstep ever matter, the fix is the `libm` crate.
7. **Bloomfront's open questions** (fog of war from a held quantity; two keepers or three-to-four), now folded into Larder World's. ([`Reports/rts-design-2026-11-10.md`](Reports/rts-design-2026-11-10.md))
8. **Larder World's open questions:** world size, how a world ends, whether two soldier teams are allies or rivals, fog of war, what carries between campaign worlds. ([roadmap](Reports/larder-world-roadmap-2026-11-11.md)) For the campaign, how a mission ends and what carries (unlocked verbs only) are decided (owner, 11-12 04:44Z; [design](Reports/larder-campaign-2026-11-12.md)); world size, allies or rivals and fog stay open.
9. ~~**Cutting the mould off its crystals**~~ Decided 11-13 15:53Z: withering, at half the blind A/B/C's speed (`wither` 0.01 in shape C's keys; "B is the best but still too fast", "Lets just double it and call it good. I will have to playtest"). Asked the same minute (15:53-15:57Z, "if possible", an inspiration): a cut-off part acts like a scared animal, more reckless, and either runs back to the main body or goes straight for the soldiers, with no stronger attack. Built 11-13 (`panic` 2, on in shape C, default-on rule; `panic=0` turns it off): it charges a unit within 20 cells or else runs for a crystal with food, after a 1.5 s lean at its own pace; [films](https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP) await the owner's playtest. ([decisions](wiki/decisions-game.md); [report, *A cut-off part withers*](Reports/fed-mould-2026-11-13/README.md))

Decided and not yet built: **the forest floor from above** (owner's card,
11-10 16:04Z) as the first out-of-the-dish game. What it needs, in this repo
with no engine, is in §5 of
[`Reports/mould-real-world-2026-11-09.md`](Reports/mould-real-world-2026-11-09.md).

## Next steps, in order

How these sit against the GPU plan, the Pixel_Physics ports and the RTS,
with what each waits on and which thread owns it: [ROADMAP.md](ROADMAP.md).
The live work is ROADMAP's Larder World band (W0-W5) and the RTS demos; the
dish steps below are paused by the owner's 11-10 20:55Z ruling and kept
for if those dishes come back.

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
