# The hunt on today's slime: rerun and built in (2026-11-28)

Enemy lane, the plan's step 5 (`Reports/plan-2026-11-27/README.md`: "Rerun
the hunt on today's tougher slime, under an hour; if a 2- to 3-hit tongue
costs soldiers, build it in"). The owner, 11-28 03:31Z (Squad thread): "I
didn't build turrets because the game was too easy to beat without them
... trying to make the slime enemy stronger."

## The rig

The Lab's, unchanged ([lab-hunt-2026-11-26](../lab-hunt-2026-11-26/README.md)),
on main after #891 with the Lab's hunt (`ec603a74`) ported:
`larder-world league=2:SEED:300 plan=rush,cap=15,go=15,shop=squad,cards=0,kite=K`
with every scheduled threat off (`raw/run.sh`). Step 2 of today's twelve
(curve place 1.96: the Sprawler, 1.75 blows a walker, levels on), run seeds
1 to 6, 5:00, lavapipe. Measure named first: soldiers bitten to death by
5:00 (`bitten_end`), paired by seed. One line a game in
`raw/league-lines.txt`.

## Results

Soldiers bitten by 5:00, seeds 1 to 6 (alive at 5:00 in brackets where a
squad was wiped or nearly):

| Arm | Keys | Bitten |
|---|---|---|
| Kiting, as main | `kite=1.2` | 0, 0, 0, 0, 0, 0 |
| Kiting, hunt 10 | `hunt_pull=10` | 0, 0, 7, 11 (0 alive), 0, 0 |
| Kiting, hunt 30 | `hunt_pull=30` | 0, 0, 4, 11 (0 alive), 0, 0 |
| Kiting, hunt 30 and the Lab's fast tubes | `+ trail_scale=0.05 step_by=1.5:1` | 11 (0), 0, 14 (2), 11 (0), 14 (0), 2 |
| Standing, as main | `kite=0` | 0, 0, 0, 0, 0, 1 |
| Standing, hunt 10 | `kite=0 hunt_pull=10` | 18, 0, 5, 18 (0), 21 (5), 20 (0) |

On 11-26 (one-blow slime, planet 2) no kiting arm lost a soldier, with
or without the hunt. On today's slime main still costs a moving or a
standing squad nothing at step 2 (12 games, 1 soldier). The hunt costs a
standing squad soldiers on 5 of 6 seeds (sign test p 0.03 one-sided); a
kiting one on 2 of 6 at pull 10 or 30, and 5 of 6 with the fast tubes,
which wiped three squads. Seed 4 wipes in every hunting arm.

What changed is the slime, not the hunt: the tongue that died at the guns
in one blow now takes 1.75 or more (and a level's more), so it reaches
them.

## Built

`hunt_pull` by planet (`run::HUNT_PULL`, curve places 1-8: 0, 10, 18, 26,
30, 30, 30, 30): off on the landing world, about 10 at step 2, 14 at step
3, 27 at step 6, full from step 8, and a Hard pick about four steps on.
The fast tubes are the whole slime's walk and were not taken. The Lab's
haste key (a longer step on the trail) was dropped: it changed nothing in
the Lab's rig and would have been a key left off. GPU only, as the lure.

Cost: four reads of the trail field a step for each calm walker on it,
and one walking-distance search a game second on a worker thread. Not
timed: the 1070 batch is Engine's.

Still: [step 2, run seed 1, greedy, 2:00](step2-greedy-120s.jpg), the
Sprawler's body streaming from its crystal down to the squad.

The start and strength are on the owner's card of 11-28 ~04:56Z (mild at
step 2 to full by step 6 recommended; from step 3; full from step 2; or
not yet); this table is the recommendation and changes with his answer.
