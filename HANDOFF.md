# Hand-off: where the mould game stood on 2026-11-17

> **History since 2026-11-17.** The front page is now [NOW.md](NOW.md), and
> [`CLAUDE.md`](CLAUDE.md) carries the rules. The owner approved a new plan
> that day ([copy](Reports/continuation-plan-2026-11-17.md)): one milestone,
> five lanes that each own files, everything on by default, no thread asks
> for a playtest. Read this page as what others were doing and recommended,
> not as orders; where it differs from `NOW.md` or `CLAUDE.md`, they hold.
> Open decisions now live in `NOW.md`. This file stays here, unedited beyond
> this note and the decisions list, only until the wiki lint stops needing
> links from it; then it moves under `Reports/`.

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

## Account move, 2026-11-17: read this first

On 2026-11-17 the owner moved to a new claude.ai account that sees **only
this repository**: no project memory, no project threads, no shared project
folder, and old `claude.ai/artifact/...` links and thread ids (`cmsg_...`)
will not open. Everything that lived only there is now copied in:

- [`handoff/project-memory-2026-11-17/`](handoff/project-memory-2026-11-17/MEMORY.md):
  every project memory note as it stood at the move (start at `MEMORY.md`,
  the index). The older 11-10 copy is [`handoff/project-memory/`](handoff/project-memory/README.md).
- [`handoff/project-files/`](handoff/project-files/): the shared folder's text
  files (the Scott-Desktop booking sheet, review briefs and prompts, a scratch
  run). Large media (crater A/B stills, sound WAVs) were not copied.
- `handoff/<area>.md`: per-area handoffs written by each working thread at
  the move, where they got to it before usage ran out.

### Project goal and direction

- **Fun over fidelity** (owner, 11-10): interesting simulations that become a
  game, not a realistic slime mould. Biology and Jones's papers are
  inspiration; judge every change by play.
- **Direction: RTS-type games** where the mould is integral to play and
  several strategies counter each other (11-10). The live game is the
  **roguelike run in squad mode** in Larder World (11-15): 8 generated
  planets, one slime per planet, shop between levels, pumps as income, rolled
  guns and gun cards. The campaign stays but is FROZEN with the RTS controls
  (D4, 11-15). The old dish game is paused.
- **Platform**: the real game is a native GPU app (wgpu); browser builds only
  for quick feel checks. Play is assumed ZOOMED IN (24 px/cell), so every
  graphic must be crisp at close zoom.

### How the owner wants work done (not obvious from code)

- **Merging**: each thread merges its own PRs by adding the label `queue`
  (`gh api repos/sgladstein/Slime-Mold/issues/N/labels -f 'labels[]=queue'`;
  the MCP `issue_write` labels call replaces other labels). Don't merge main
  in by hand to re-run CI. Details in `CLAUDE.md`.
- **Never assign sgladstein to a PR or request their review** (they get an
  email per PR; 11-14). Keep the attribution lines in PR bodies.
- **The repo is public** (made public 11-12 to fix Actions billing). Keep the
  owner's email and personal details out of anything pushed. A CI job that
  fails in 2-3 s with no log is billing: read the check-run annotation.
- **Build first, balance later** (11-12): a feature lands when it works and
  can be seen (tests plus a GIF or playable build). Censuses only answer a
  named strategy question, under about an hour without the owner's OK.
- **Defaults on** unless there is a measured harm or stated uncertainty.
- **Show, don't tell**: pictures and GIFs on an Artifact page (stills are not
  enough for look picks; give GIFs/video or a playable build); visual choices
  as blind A/B with the key off the page; page sources saved under
  `Reports/` and listed in `Reports/pages.md`.
- **Lean reading**: `CLAUDE.md` is the only file every thread reads; read
  only what the task needs.
- **Brainstorms start from the model**, never anchored on the old demo tools
  (lamp, oats, salt, knife, water).
- **Reviews** lead with the big picture (missed tests, approach, other
  directions), not only numbers. Two standing reviewers (game designer,
  technical lead) reviewed merges and milestones; their advice went straight
  to threads. When the owner was away, reviewers plus an integration thread
  decided and recorded it.
- **New threads** suggested by agents were pre-approved (11-12).
- **The owner plays by double-clicking `play.cmd`** (pulls main, builds
  `larder-world` and `larder-edit`, opens the menu). `play-new-slime.cmd`
  plays the new-slime rules. Point them at these, not at cargo.
- **Never `cargo fmt` the whole tree** (it reflows other threads' files).
- **Command ids** for `larder-world` live in one `commands!` table in
  `src/bin/larder-world.rs`; new ids append at its end.
- **Ownership locks** that held at the move: `paths.rs` distance map / map
  cache belonged to the levelgen thread; soldier movement over it (walk,
  elbow_room, squad_turn, guide, funnel) to the squad movement thread.

### Machines

- **Cloud sessions have no GPU.** `apt install mesa-vulkan-drivers` gives
  llvmpipe Vulkan for checking wgpu code (never for timing).
- **Scott-Desktop**: the owner's Windows PC with a GTX 1070, repo clone at
  `C:\Users\Scott\Code\Slime Mold`. GPU timing runs go through Remote Control
  when the PC is on with the Claude app open; standing OK for tests (11-11),
  fully available including timing (11-14). Booking and run rules:
  [`handoff/project-files/scott-desktop/booking.md`](handoff/project-files/scott-desktop/booking.md).
  Timing runs alone; warm the card first; close Edge before timing; GPU-idle
  check under 10%. Disk: "clear build folders and stay on C"; check free
  space first; never delete the owner's files. Every frame was under 2.6 ms
  p95 on the 1070 (11-14 batch).
- **Windows vs Linux**: f32 `sin`/`cos`/`exp` come from the platform libm, so
  CPU replays and seed results differ across OSes (the skirmish
  `eating_fills_the_bank` test failed on Windows; fix 1a681d7). The GPU
  differs again (FMA, atomics order). Use the `libm` crate if lockstep ever
  matters.
- **Runner threads**: long runs could use up to 4 Haiku runner threads that
  only run given commands and send raw output back.

### Owner rulings for Larder World and the run (condensed; full list in `wiki/decisions*.md` and the memory copy)

- Soldiers must NOT need feeding; no upkeep or maintenance costs at all
  (upkeep code removed, #298). Pumps are the main income.
- Squad mode is the default mode; healing is a specific order (costs the same
  energy as building), not automatic. New soldiers join the active squad;
  tight blob is the default formation; linked squads keep their own guns.
- Larder system removed in squad mode: a taken crystal is immediately yours.
  Crystals pay by drain or guard; guarding is worth more long-run. The owner
  wants complexity.
- Walking into thick slime is VERY deadly; soldiers stand and shoot into
  slime, are heavily slowed in thick slime, can't stand on crystals. Shots
  stop at the first slime. Crater = 2 s default.
- Shape C is the default slime. Difficulty comes from mould types (a
  progression ladder), not from balance cuts; don't make the slime easier for
  the player AI. Slime aims to control crystals and grow; cut-off parts
  wither (and panic). Slime can damage and overgrow walls slowly.
- Bronze (thicken/condense) is removed from play (keys stay); breakaway
  specks come from ordinary slime, more often early, on from planet 2.
- Mini-bosses: all kinds, no bronze, spawn at crowding or cut-off, chance
  rises through the run, much stronger and varied.
- One new threat per slime per planet (D3, tested). No starting payload card
  (D5). Shake/hit-stop: show video first (D6). No rush brake yet (D7). Drop
  outpost from the run shop, hold lure/siren (D10).
- Raids are the #1 defense payoff; a kill box pays only through an upgrade.
- Refunds: demolish refunds half; cancelling a partial build refunds 100%.
  One click resumes a half-built building.
- Turrets fire over walls, rifles don't.
- UI: the HUD is ONE row (gun cards included); NO scrolling; UI scale
  applies to every screen; the home-crystal hover overlay must not block
  shooting.
- Graphics: shaders for slime and effects; crisp hexagon crystals; sharp
  slime; visuals follow the simulation (no drawn-only effects). Designed maps
  are part of the plan; ground is drawn from the simulation's fields.
- Guns: squad owns rolled guns and rolled bullets; cards cost, guns have a
  budget, bullets plus stacking modifiers; simple card text.
- The roguelike run is SEPARATE from the campaign (don't erase the
  campaign); maps grow bigger and harder through the run.

### Loose ends at the move (2026-11-17)

- **PR #436** must be labelled `queue` after **#434** merges.
- **Provoke and Still cards**: built on branch
  `claude/project-thread-vx9t3p-wip`, tests pass, not filmed, no PR yet.
  Next: film them, open a PR, then gun-system step E.
- **Owner picks still open** (the "New Slime Picks" page; its source should
  be under `Reports/` via `Reports/pages.md`): 4 blind A/B films, Heave/Corral
  merge yes/no, set slime block-vs-trap, fireproof fed slime; the door-grate
  bullets question; Mind-bullet clip guesses; the crust "name the ground"
  quiz; a `play-new-slime.cmd` playtest. Also older: haze and flinch A/B,
  mini-boss look Violet/Ember/Ink, fen moat A/B, plants and condense
  playtests, sound levels.
- **Three GTX 1070 timing runs queued**: homing, Mind bullets, Grudge memory.
  Plants also still lack a 1070 timing.
- The decisions list below (Decisions the owner has not made) still holds.

## Where the game stands

One line per subject; the page holds the detail. The owner's rulings:
[`wiki/decisions.md`](wiki/decisions.md).

**Live work: the roguelike run in squad mode** (owner, 11-15 08:30Z: the
game they play and judge by). Eight generated planets toward the slime's
core world, one slime a planet, a shop between, squad control, pumps as
income, rolled guns and cards. Their three asks: an enemy that progresses and
presses, guns that are a main draw, menus that don't get in the way. The
whole-game review (11-15, [report](Reports/whole-game-review-2026-11-15/README.md))
ranks what to do; ROADMAP's *Now* band is cut around it.

- [The roguelike run](wiki/world/run.md): 8 planets, shop, dev mode, gun range; since #336 the slime is shape C under the tier and grows (planet 1 idle 22,410 walkers at 4:00, planet 3 wears pumps down by 3:00); the drain-and-buy rush still reaches 40 soldiers by 2:00 and no thread yet owns the enemy's progression by planet.
- [Larder World](wiki/world/larder.md): the world the run is played in (crystals, terrain, walls, GPU big dish); W1a and W0 [growth and starving](wiki/model/growth.md) built; the larder itself is RTS-only (missions), squad mode has none.
- [Planets](wiki/world/planets.md): the roguelike run's levels, generated from a seed and a depth (`mould::levelgen`), eight biomes in a set order to the core world; all at today's map size until maps carry their own.
- [Plants](wiki/world/plants.md): a new plant field in four kinds, separate from the old grass (owner, 11-14), on CPU and GPU and drawn; fire through plants, lit by scorch rounds and blasts, stopped by water, reeds and the slime's body; maps carry plants in B.
- [The mould tiers](wiki/world/mould-tiers.md): the difficulty ladder; Sporer, Sleeper and Swarmer built ([the life cycle](wiki/model/life-cycle.md)).
- [The player's own mould](wiki/world/own-mould.md): gardeners (owner, 11-12): soldiers grow a second species that fights the enemy by conversion; a probe found forms trade eating for fighting; the CPU build (`own_mould.rs`, `examples/garden.rs`) grows it by sowing but it does not yet shrink the enemy; contact is next.
- [Squad mode](wiki/world/squad-mode.md): steer one squad, the others run AUTO; split, join, link, guards, queues; the default mode since #330, healing is an order. [The squad economy](wiki/world/economy.md): pumps are the main income (#322), no upkeep. [Guns and gun cards](wiki/world/guns.md): rolled guns with a budget, rolled bullets, the gun range.
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

**Closed 2026-11-17.** The owner closed 2 to 8 as not needed while those
games are paused (19:07Z, "5)Yes"; none was answered, so ask again only if
its game comes back), and answered D9 and D12 the same minute
([decisions](wiki/decisions-game.md)) and D8 at 19:21Z
([decisions](wiki/decisions.md)). What is still open (D7 the rush brake) is
in [NOW.md](NOW.md), *What waits on the owner*. The
list below is kept as it stood, so older references to its numbers stay
right.

1. ~~What to fix first after review 2.~~ Decided 11-10 18:56Z ("Trace it now"); the 20 s landing preview it proposes is not agreed.
2. **Grains or glints** for walkers in the tube look (only while F7 is kept). [`Reports/mould-looks-2026-11-10.md`](Reports/mould-looks-2026-11-10.md)
3. **Retire the tube look (F7)?** Ask before deleting. Same report.
4. **Food flow:** on or off; off by default, no verdict. [`Reports/mould-probe-2026-11-07.md`](Reports/mould-probe-2026-11-07.md), "Food that flows"
5. **The Organ and the Wanderer:** keep, change or drop.
6. **The GPU plan:** upload Jones 2011 before phase 4 (W0); hold the Pixel_Physics multi-core renderer port. ([page](wiki/model/scale.md)) The 1070 differs from the CPU in `sin`, `cos` and `sqrt` and fuses FMA (754246d; [report, *On the GTX 1070*](Reports/gpu-readout-2026-11-12.md)): cross-machine replay or lockstep needs integer or table maths. The CPU sim doesn't reproduce across Windows and Linux either: f32 `sin`, `cos` and `exp` come from the platform maths library (MSVC or glibc), so CPU replays, hashes and seed results differ between the two (the skirmish bank test failed on the PC for this, 2026-11-12). If cross-machine CPU replays or lockstep ever matter, the fix is the `libm` crate. The card also orders atomics and float sums its own way (2026-11-14 batch at 004b7cb, [report](Reports/pc-batch-1070-2026-11-14.md)): births race so a garden's sides came out (988, 1068) against the CPU's (992, 1064), and one blast-flung walker parted at tick 3. So short twins are exact on llvmpipe (CI's bug detectors) and get measured slack only on a card (`tol`, `ON_CARD` in the twin tests).
7. **Bloomfront's open questions** (fog of war from a held quantity; two keepers or three-to-four), now folded into Larder World's. ([`Reports/rts-design-2026-11-10.md`](Reports/rts-design-2026-11-10.md))
8. **Larder World's open questions:** world size, how a world ends, whether two soldier teams are allies or rivals, fog of war, what carries between campaign worlds. ([roadmap](Reports/larder-world-roadmap-2026-11-11.md)) For the campaign, how a mission ends and what carries (unlocked verbs only) are decided (owner, 11-12 04:44Z; [design](Reports/larder-campaign-2026-11-12.md)); world size, allies or rivals and fog stay open.
9. ~~**Cutting the mould off its crystals**~~ Decided 11-13 15:53Z: withering, at half the blind A/B/C's speed (`wither` 0.01 in shape C's keys; "B is the best but still too fast", "Lets just double it and call it good. I will have to playtest"). Asked the same minute (15:53-15:57Z, "if possible", an inspiration): a cut-off part acts like a scared animal, more reckless, and either runs back to the main body or goes straight for the soldiers, with no stronger attack. Built 11-13 (`panic` 2, on in shape C, default-on rule; `panic=0` turns it off): it charges a unit within 20 cells or else runs for a crystal with food, after a 1.5 s lean that creeps at half its pace (`panic_creep` 0.5, the game designer's review: a lean at its own pace already outran a soldier), and a charge at a unit gives up after a 0.5 s dash (`panic_dash` 15) so a squad that pulls back in time gets clear; [films](https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP) await the owner's playtest. ([decisions](wiki/decisions-game.md); [report, *A cut-off part withers*](Reports/fed-mould-2026-11-13/README.md))

10. **From the whole-game review (11-15)** ([report](Reports/whole-game-review-2026-11-15/README.md), *Owner decisions*): D1-D5, D7 and D10 answered 21:41Z ([decisions](wiki/decisions-game.md)). On 11-17: D9 (Windows only for now) and D12 (the own mould stays parked) answered yes; D8 answered by card at 19:21Z: new slime rules on the GPU engine only, today's CPU engine kept, its removal decided at Milestone 2; D6 (camera shake and hit-stop) goes into the game as an Experiments toggle to try.

Decided and not yet built: **the forest floor from above** (owner's card,
11-10 16:04Z) as the first out-of-the-dish game. What it needs, in this repo
with no engine, is in §5 of
[`Reports/mould-real-world-2026-11-09.md`](Reports/mould-real-world-2026-11-09.md).

**Settled without the owner (cloud plan, 11-14 00:35Z; reviewers with the
integration thread, under the owner's 11-12 04:48Z rule; [plan](Reports/cloud-plan-2026-11-14.md),
[ROADMAP](ROADMAP.md) *Cloud plan picks*):** D1 the census harness moves to
shape C's rules at a census-sized map; D2 `flow_loss` parked (off, kept;
it cost about 30% of C's size and still died at once); D3 the counter
table C4 is judged against (designer); D4 mission 8 becomes "Your own mould" against
the Grudge on the Passes, the rival pair moves to chapter 2; D5 the start menu's
first scope; D6 the file split goes first, in a short freeze.

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
- **Merging.** Label the PR `queue`; `.github/workflows/merge-queue.yml`
  merges it onto current main (with up to four other queued PRs) as a merge
  commit, runs CI (the tests as four parallel parts: lib, window, window
  smoke, rest with clippy; and "sound device builds (organ)") on that
  candidate, and fast-forwards main when green. GitHub's own
  merge queue is for organization-owned repos only (2026-11-14). PRs that
  change only `Reports/`, `handoff/`, `research/`, `wiki/` or `*.md` skip the
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
