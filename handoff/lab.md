# Lane brief: Lab

Model and effort: Fable, high. Read `00-all-lanes.md` in this folder first.

## Your job

One prototype at a time, outside the milestone, in your own example
program or branch. Nothing of yours goes into the main game or behind a
switch there. You show Scott a playable clip and a recommendation: build
it into the game, change it, or drop it. He chose "Slice first, plus one
lab thread" on 11-17; you are that thread.

## First prototype: steering the slime

The idea: the slime follows fields, and today the player's tools write
almost only damage into them. Let the player herd it instead: lures that
visibly draw it, walls and ground that funnel it, bait and false trails,
so a good player moves the slime into a kill zone. No other game's enemy
allows this, so it could be the game's signature verb.

Scott's words that point here, 11-16 05:11Z: "There should be ways to
modify the pushback. I could image a gun that lets you corral and push the
slime onto areas. Into a kill zone". He did not answer the question of
whether steering should be your first prototype, so it goes ahead on the
plan's recommendation; if he names another, switch.

How to work:
- Start from the model, not from the old demo tools (his ruling, 11-10):
  what can walkers and fields respond to? Scent, food, alarm, trail,
  ground speed, walls. Describe the simulation neutrally before listing
  tools.
- Grep `Reports/dead-ends.md` for lure, bait, siren and scent before
  building: the siren, the lure and the outpost were taken out of the run
  shop on 11-15, and the reasons matter.
- Build on the GPU engine and test on the software GPU.
- Deliver within about three days: a clip of 30 to 60 seconds on an
  Artifact page of a squad herding a slime into a kill zone on one of
  planets 1 to 3, the counts beside it, and your recommendation.

## Later briefs, for after this one

A second slime species that fights the first; the player's own mould
(parked, D12); the forest floor.

## State (kept current by the lab thread)

- 2026-11-17 22:30Z: **the steering prototype is delivered.** Report
  [`Reports/lab-steering-2026-11-17/`](../Reports/lab-steering-2026-11-17/README.md),
  page "Steering the Slime" (in `Reports/pages.md`). Recommendation: build
  it into the game, changed (its own field slot and a per-tier weight, the
  Lure back in the run shop, pull 10 not 30). Whether and when is
  Direction's to place; nothing of it is in the main game.
- Open branch: `claude/project-thread-4tc75h` holds the prototype's code
  (commits "LAB: steering field prototype" and "LAB: the lure's core
  glow"; module `src/mould/larder_world/lab.rs`). It changes
  `engine.wgsl` and `view.wgsl` in place and borrows the far smell's slot
  (`far_smell=-W` is the pull), so it must not merge. The rig and every
  scene's places are in the report.
- The lab session can push only that branch, so the lab's docs (this file,
  reports, pages) reach main through Direction: the files go in the
  project's shared folder under `lab-docs/` and Direction merges them
  docs-only.
- 2026-11-18 03:03Z: Scott: "Lure now, in the three planets". Enemy ports
  the lure field from commit `dafbc22` (own slot, weight 10), Squad and
  arsenal returns the Lure to the run shop, Look picks the ring colour. The
  lab answers their questions and keeps the branch; the pusher, per-tier
  weights and steering by bullets stay in the lab.
- 2026-11-18: part two, steering by bullets, delivered as a section of the
  same report and page: no bullet steering card (no version beat plain
  fire); the gun's job is to carry a lure dart. Lead: a strong lure that
  drags a colony off its crystal shrinks it without kills.
- 2026-11-18: part three, the dart (Direction's pick), on the same page
  (`#dart`): a dart fired where the squad aims, as a lure or a pusher; at
  pull 10 a dart every 10 s takes a crystal off a colony in 30 to 45 s and
  leaves the slime 22% smaller by 2:00, at the cost of kills and food.
  Recommendation: build it as the Lure's second form, with a pusher
  setting. Code commit `9df3f0a` (`LAB_DARTS`), not for main.
- 2026-11-18: part four, after the colony leaves (`#after`): it neither
  starves nor comes for the squad; most of it dies in the turrets, and
  another colony resettles the crystal within one to two minutes once the
  darts stop. With darts the squad lives to 5:00, without it is overrun.
  Without turrets the lure loses to a raid. **Steering is closed as a lab
  subject** (Direction, 11-18 03:31Z); the dart waits for Scott to have
  played the Lure building.
- 2026-11-18: **the second species is delivered.** Report
  [`Reports/lab-species-2026-11-18/`](../Reports/lab-species-2026-11-18/README.md),
  page "Two Slimes" (in `Reports/pages.md`). Described from the model
  first, then built on the lab branch (code commits `849670d`, `a96e78c`,
  not for main): the garden's green mould (`own=1`) as a hunter that
  converts the orange one way, at an edge (`LAB_CONVERT`), and takes no
  crystal share (`LAB_HUNTER`), so it lives on what it converts. Planet 1:
  at edge 0.8 it starves off its crystal, then a front crosses the orange
  body and eats it by 5:00; 0.6 and 0.7 win sooner; 0.9 and 1.0 live beside
  it. Conversion alone stalls. Recommendation: keep the starving hunter,
  not for the game yet: start it as a small seed, make it a second enemy
  strain, and try leading the slime into it with the Lure.
- 2026-11-18: part two of the second species, leading the slime into the
  hunter with the lure (pull 10, laid in the hunter's ground, page
  `#lure`): at edge 1.0 the hunter is twice the size and the orange 36%
  smaller by 7:00 (8,247 / 6,651 against 4,260 / 10,345), but green and
  orange together are no fewer; on run seed 2 the hunter starved by 2:00
  either way. The second species is closed in the lab (Direction 08:49Z:
  "Then the forest floor"); it waits as a second enemy strain on NOW.md's
  Lab row.
- 2026-11-18: **the forest floor is delivered.** Report
  [`Reports/lab-forest-2026-11-18/`](../Reports/lab-forest-2026-11-18/README.md),
  page "Forest Floor" (in `Reports/pages.md`). Described from the model
  first, then built on the lab branch (code commit `a9b9e61`, key
  `LAB_FLOOR=1`, not for main): rain that runs to the rivers, pools and
  dries, on the GPU plant pass; the slime, soldiers, fire and the view read
  it. Planet 1: one shower pools 6% of the cells for about half a minute;
  the slime ends a tenth smaller; few soldiers stand in mud. Rot (logs into
  food) not built. Recommendation: change, then build (wet planets, a
  player verb on it, a look pass).
- 2026-11-18: part two of the forest floor, the rot (page `#rot`, code
  `47150aa`): litter rots into food where rain soaks it and smells like a
  crystal; after a shower the slime forages out and the squad shoots it
  (2,020 killed by 2:00 against 1,476), the slime no bigger.
- Next step: none. Direction (11-18 14:14Z): stop and wait for a brief; the
  dart and the wet-planet trait go to Scott together after he has played
  the Lure.
