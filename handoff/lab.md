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
- 2026-11-18: **cohesion, one body, is delivered** (brief from Direction
  15:58Z, from Scott's play note that raids read as separate walkers).
  Report [`Reports/lab-body-2026-11-18/`](../Reports/lab-body-2026-11-18/README.md),
  page "One Body" (in `Reports/pages.md`), code `22f468a` (keys
  `LAB_TETHER`, `LAB_WAIT`, `LAB_BODY_LOG`). Marchers read no trail; a
  tether on marchers alone cuts loose walkers to a third and keeps
  breakaways a blob, but the raid sends fewer walkers out (kills 799
  against 1,212). Recommendation: tether 2 with the raid's walker count
  retuned, for Direction to place.
- 2026-11-18: part two, tighter tethers at Scott's ask (page `#tighter`,
  code `73b6357`): any tether of 2 or more keeps raid 1 at home (raid kills
  41 against 379); tighter adds nothing. Recommendation now: tether 2 for
  roaming and breakaway walkers only; a muster so a raid leaves as a body
  is the next step if Direction wants it. Numbers sent to Enemy.
- 2026-11-19: the lab branch's tree restarted from main (`d267da1`, a
  merge that keeps the old prototypes' history; they are no longer in
  the tip). **Trunk veins delivered** (brief from Direction 09:30Z):
  report [`Reports/lab-veins-2026-11-19/`](../Reports/lab-veins-2026-11-19/README.md),
  page "Trunk Veins", code `042b8a9` (`LAB_VEIN`, `LAB_ALIGN`; the flow
  block after the plants', not the dense region's fourth quarter, which
  is the fort's mask). No veins from walker motion: the body is packed.
  Recommendation: draw the shared food's flow instead (untested).
- 2026-11-24: **the Sprawler's body as threads is delivered** (brief from
  Direction 03:14Z, Milestone 2's first lab job). Report
  [`Reports/lab-threads-2026-11-24/`](../Reports/lab-threads-2026-11-24/README.md),
  page "Sprawler Threads" (in `Reports/pages.md`). The lab's branch is now
  `claude/project-thread-2o4cic` (from main 11-24; code `283617eb`, keys
  `LAB_LANES`, `LAB_SHUTTLE`, `LAB_CAPS`, `LAB_HOLD`, `LAB_DUMP`). Lanes and
  a shuttle together turn planet 2's packed disc into spokes at the same
  walker count; recommendation: build it for the Sprawler, changed. Enemy
  builds it on Direction's say.
- 2026-11-24: **scar tissue for the Grudge, planet 7** (brief from
  Direction 04:37Z: the weakest of planets 4 to 8's nameable things; Enemy
  named planet 7). Report
  [`Reports/lab-scar-tissue-2026-11-24/`](../Reports/lab-scar-tissue-2026-11-24/README.md),
  page "Grudge Scar Tissue". Code `4c76a347` (`LAB_TOUGH=P:S0`). A hit on
  scarred slime shrugs off; recommendation: build it as the Grudge's thing
  at P 0.5.
- 2026-11-24: **it grows in its sleep, for the Sleeper, planet 6** (brief
  from Direction 05:16Z: Enemy's runner-up, free to farm before it wakes).
  Report [`Reports/lab-sleeper-2026-11-24/`](../Reports/lab-sleeper-2026-11-24/README.md),
  page "Sleeper Dreams". Code `8cbe4757` and `3508d9d4` (`LAB_DREAM=P`,
  `LAB_DUMP_LIFE`, hand `wake@T,X,Y`). A sleeping walker buds a sleeping
  child off the scent; recommendation: build it as the Sleeper's thing at
  P 0.003.
- 2026-11-24: **a lap-two pairing for planet 8, the Swarmer that naps**
  (brief from Direction 06:31Z; Enemy: planet 8 is weakest, it only
  replays 5 to 7). Report
  [`Reports/lab-pairing-2026-11-24/`](../Reports/lab-pairing-2026-11-24/README.md),
  page "Napping Swarmer". Engine keys only (`raw/nap.txt` on `run=1:4`).
  It sleeps until shot, charges, sleeps it off; the report lists the trait
  keys that fight. Recommendation: planet 8 as this pairing, Enemy tunes.
- 2026-11-24: **lap two's seven pairings, planets 9 to 15** (brief from
  Direction 06:53Z, for the run-length card). Report
  [`Reports/lab-lap-two-2026-11-24/`](../Reports/lab-lap-two-2026-11-24/README.md),
  page "Lap Two Pairings". Keys only (`raw/pairs.txt`); each smoke-tested
  2:00 alone and paired; the seven are a first draft for Enemy.
- 2026-11-24: **sorting the walkers by place** (brief from Direction
  08:07Z, test 5). Report
  [`Reports/lab-walker-sort-2026-11-24/`](../Reports/lab-walker-sort-2026-11-24/README.md),
  page "Walker Sort". Code `247b6730` (`LAB_SORT`, `LAB_SORT_LOG`),
  `a99924b4` (`LAB_SORT_PACK`). 1070: step 9.46 to 5.27 ms; seven pairs
  play the same. Open: Engine's pack-only 1070 run, to add to the report.
- 2026-11-24: **feeding the march, planet 4's late raids** (brief from
  Direction 09:14Z). Report
  [`Reports/lab-march-counter-2026-11-24/`](../Reports/lab-march-counter-2026-11-24/README.md),
  page "March Counter". Code `14ba8f8f` (`LAB_FEED`, `LAB_HUNGER`,
  `LAB_MARCH_LOG`). No march wall in the rig (a still squad of 8 lost no
  one to a march, mini-bosses off); feeding starved a wanderer on seed 2.
  Open: a rig where the march is the wall, from Direction.
- 2026-11-24: **a charged crystal as bait for a mini-boss** (brief from
  Direction 10:34Z). Report
  [`Reports/lab-crystal-bait-2026-11-24/`](../Reports/lab-crystal-bait-2026-11-24/README.md),
  page "Crystal Bait". Code `8573ff93` (the `drain@X,Y` hand), with
  `LAB_FEED` and `set=boss_kind=2`. Wiped 2 of 6 off, 0 of 6 bait; the
  wanderer broke in 3 of 6 bait. Open: the trade itself (stage the boss).
- 2026-11-24: **planet 9's rig** (Direction 12:08Z): `LAB_LAP2=1`
  lets `run=SEED:9` to `:15` through (code `9115723b`);
  `Reports/lab-lap-two-2026-11-24/raw/rig9.sh` runs the Drifter with the
  Sporer's bursts. Lap two's sizes wait on Direction.
- 2026-11-24: **bait or drain on planet 7** (Direction 12:08Z). Report
  [`Reports/lab-bait-trade-2026-11-24/`](../Reports/lab-bait-trade-2026-11-24/README.md),
  no page. 18 games on the 1070 (`raw/pc-trade.ps1`). Drain beats bait as
  a player choice; feeding keeps wanderers small.
- 2026-11-24: **lap two's first sizes and rigs** (Direction 13:09Z), a
  section in the lap-two report: `LAB_LAP2_SIZE` (code `bd836d45`), planet
  9 at 0.7 of the size rule (run bot on the 1070 against planet 8),
  `raw/rig.sh` for 9 to 15, smoke tests (12 and 15 end under 1:00 for
  greedy; 10 and 14's pump smell unseen).
- 2026-11-24: **lap two's smoke checks, round two** (Direction 14:26Z),
  a section in the lap-two report. Strikers wipe 12 and 15 (the oversized
  start trips their 60% trigger); first number `strikers_below` 0.45 on
  lap two. Pump smell 40 takes 14's edge pump in 45 s; it is redundant on
  10. Set walkers are counted; paired 15's first size is 0.5.
- 2026-11-24: **planet 10's trait and the strikers' settled peak**
  (Direction 15:39Z), a section in the lap-two report. Planet 10 takes the
  Drifter's spore bursts (10 to 40 times the spores in flight). The rule key
  `strikers_settle` (code `e7b72d88`) lets 12 and 15 live past 1:00 with no
  `strikers_below` patch, and planets 2 to 8 play as on main. Enemy's patch
  is `raw/strikers-settle.patch`.
- 2026-11-24: **planet twists** (Direction 17:00Z, run variety). Report
  [`Reports/lab-twists-2026-11-24/`](../Reports/lab-twists-2026-11-24/README.md),
  page "Planet Twists". `LAB_TWIST` (code `4f4f3856`): cold, night, wind.
  Wind and the cold snap recommended, with card lines.
- 2026-11-24: **the rescue planet** (Direction 17:24Z, a goal other than
  the heart). Report
  [`Reports/lab-rescue-2026-11-24/`](../Reports/lab-rescue-2026-11-24/README.md),
  page "Rescue Planet". `LAB_RESCUE` and the bot hand `rescue@X,Y[,T]`
  (code `a7c788e2`). Winnable and loses visibly on planet 1, but too short
  there; timing on planets 3 to 5 needs the GTX 1070.
- Next step: add Engine's pack-only run when it comes; then wait for
  Direction's next brief.
