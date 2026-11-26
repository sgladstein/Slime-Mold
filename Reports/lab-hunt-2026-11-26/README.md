# Lab: the hunt, a slime that follows the squad's trail (2026-11-26)

Step A of the owner's order of 11-25 21:35Z ("a then c ... then b"), the
Lab's half: [the brief](../review-2026-11-25/lab-brief-slime-reach.md) asks
whether the slime's own rules can move its mass toward a squad faster than
the squad can leave, so that kiting costs soldiers, with no clock. Built on
the lab branch only (commit `ec603a74`, "LAB: the hunt"), not in the game.

## What was built

**The hunt** (`src/mould/larder_world/hunt.rs` at `ec603a74`), the brief's
third direction, a prey scent that leads, with its fourth, giving up, built
in. Once a game second the cells the soldiers stand on are kept for as long
as they are warm. The slime's field is the walking distance to the nearest
of those places with each place's age added as distance
(`Rules::hunt_lag`, 4 dish cells a second, out to `Rules::hunt_reach`, 80).
With the lag faster than the squad walks, a newer place always reads nearer
down the field, so a walker that meets the path goes along it to where the
squad is now. A place older than reach / lag (20 s) pulls nothing, so a
squad that gets far enough ahead leaves a cold trail and the body stops.

Calm walkers lean down the field by the engine key `hunt_pull` (as the
lure's pull) and, with `hunt_haste`, step farther on it. The field is built
off the frame as the lure's (`round_walk_from`, the lure's search with a
start distance a place) and written to the raid region's fourth quarter; the
pull and haste ride the wind row's two spare words. GPU only. Test:
`the_hunts_trail_leads_along_the_path` (raids.rs).

Cost: four `news` reads a step for each calm walker while a trail is
written (the lure's), and one search a game second on a worker thread. Not
timed on the GTX 1070.

## The rig

`larder-world league=2:SEED:300 plan=rush,cap=15,go=15,shop=squad,cards=0,kite=1.2`
with every scheduled threat off: `set=raid_every=0 set=fruit=0 set=wave=0
set=strikers=0 set=stand=0 set=last_stand=0 set=late_at=0 set=heart=0
set=star=0 set=boss_most=0`. Planet 2 (the Sprawler), run seeds 1 to 6, a
squad of 15 plain rifles, 5:00 each, on software Vulkan (lavapipe). The
strategy bot's kite walks the squad away while more than 30 walkers are
within 1.2 gun ranges. Measure named first: soldiers bitten to death by
5:00, paired by seed; walkers killed as the sign the slime came.

## Results

Soldiers bitten to death by 5:00 and walkers killed, seeds 1 to 6:

| Arm | Keys | Bitten | Walkers killed (sum) |
|---|---|---|---|
| Kiting, as main | | 0, 0, 0, 7, 0, 0 | 23,093 |
| Fast tubes | `trail_scale=0.02 step_by=1:2` | all 0 | 17,379 |
| Stronger soldier smell | `set=smell=4 set=smell_r=8` | all 0 | 19,724 |
| Hunt, pull 1 | `hunt_pull=1` | all 0 | 49,649 |
| Hunt, pull 10 | `hunt_pull=10` | all 0 | 61,300 |
| Hunt, pull 30 | `hunt_pull=30` | all 0 | 61,340 |
| Hunt 30 and fast tubes | `+ trail_scale=0.05 step_by=1.5:1` | all 0 | 99,054 |
| Hunt 30, haste 1 | `+ hunt_haste=1` | all 0 | 63,002 |
| Hunt 30, charge reach 15 | `+ set=panic_margin=15` | all 0 | 66,514 |
| Hunt 30, no flinch | `+ flinch=0` | all 0 | 65,601 |
| Hunt 30, tubes, Swarmer's charge | `+ panic=3 set=panic_margin=25` | all 0 | 89,083 |
| Swarmer's charge alone | `panic=3 set=panic_margin=25` | all 0 | 22,140 |
| 6 soldiers, as main | `cap=6,go=6` | all 0 | 8,107 |
| 6 soldiers, hunt 30 and tubes | | all 0 | 72,973 |
| Standing and fighting, as main | `kite=0` | 0, 1, 8, 23, 0, 0 | 40,063 |
| Standing, hunt 10 | `kite=0 hunt_pull=10` | 0, 3, 2, 34, 5, 0 | 136,139 |

Hunt 30 with fast tubes against main, kiting: more walkers killed on 6 of
6 seeds (sign test p 0.03), with 6 soldiers too (6 of 6). The slime at
5:00 is about the same size either way (17,422 to 20,483 against 16,457 to
22,903): what it throws at the squad it grows back from its crystals.

**No arm passed.** Kiting lost nobody in any hunting arm, nor with 6
soldiers, nor with the Swarmer's charge on top. The films say why.

## What it looks like

[before](before_all.mp4) and [after](after_all.mp4), the whole map, and
[before](before_near.mp4) and [after](after_near.mp4) at zoom 6 on the
squad: planet 2, run seed 1, 1:00 to 1:40 at five times speed, the same
kiting bot. As main the slime spreads threads to the crystals across the
map and leaves the squad to drain its crystal. With the hunt (pull 30 and
fast tubes) the body masses toward the squad and pours down the corridor
after it in a thick tongue, and it stops spreading to the far crystals
while it hunts. Stills at 1:20: [before](before-80s.jpg), [after](after-80s.jpg).

The tongue's head reaches the guns and dies there. 15 rifles, or 6, kill
whatever comes down one corridor as fast as it comes: 6 soldiers killed
73,000 walkers in five minutes and were never touched. Reach was half the
problem; the other half is that the body arrives one front at a time,
through the narrowest place, into the guns. The standing squad lost
soldiers only where it walked into the body.

## Recommendation

1. **Keep the hunt as the reach**, not as the threat. It makes the slime
   visibly come for the squad from its own fields, with no clock, and run
   cold when the squad gets away; on its own it does not cost soldiers.
2. **The next Lab step is arrival as a mass**: walkers that reach the end
   of the trail just outside the guns' reach hold there (the crowd packs
   them) until the front is deep, then come in on the alarm, so the body
   arrives wider than the guns can cut. This is the wave event's idea
   (`waves.rs`) driven by the trail instead of by the squad's kill rate,
   and the threat audit's case for merging the comeback events into one
   chase.
3. **Or a balance answer**: the plain rifle's kill rate on a corridor is
   the wall, which is the 11-25 review's second finding (the bot wins
   planets 1 to 3 with it). That is Direction's.

The owner said at 11-25 23:57Z, "the Slime heart makes the game even
easier. Lets turn it off for now"; the heart is another thread's (the
mini-boss rework), and this work left it alone.
