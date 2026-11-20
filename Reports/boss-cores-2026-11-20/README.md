# Boss cores that re-form, and raids' cores (2026-11-20)

The helper thread beside Starfall (`Reports/starfall-2026-11-20/`), building
the rest of the design the owner picked 11-20 19:06Z ("I like starfall with
boss cores"): walkers stream back to a core, and if they get there first the
boss re-forms; a beaten raid drops one too; a planet's cores hold a fixed
pot. Code: `src/mould/larder_world/cores.rs`; rules in `StarRules`.

## The first count, and the change it made

Six games, the miniboss review's census (`Reports/miniboss-review-2026-11-20/tools/census.sh`:
the owner's run seeds 1795038793, 1795106014 and 1795119328, planets 2 and 3,
`hand=strat:econ,go=4,aim=6`, to 420 s; three of the planet-2 games end
early with the squad wiped, as before this change).

| | cores dropped | from bosses | from raids | re-formed | core points taken |
|---|---|---|---|---|---|
| Core stays where the boss broke | 3 | 2 | 1 | 0 | 0.8 |
| Core runs home (`core_drift` 8, `core_drift_r` 60) | 6 | 5 | 1 | 1 | 75 |
| Runs home, cores eaten at `core_eat` 0.3 (ore's 1) | 5 | 4 | 1 | 1 | 11 |

With the core left where the boss broke, it lay inside the guns' reach (15 to
22 units from the nearest soldier) and 0 to 4 walkers lived to reach it in
its 15 s; a held squad never fetched it either. So a core now slides to the
nearest thick slime within 60 dish cells, and the squad has to reach in for
it. The second row is not paired with the first: the Starfall thread raised
the slime's `eat` from 0.25 to 1.0 between them, and that moves every game.
Neither bot goes for drops; the points are cores that slid past soldiers.

**A core is eaten slower than ore.** At ore's `eat` of 1, a core that
landed in thick slime (27 walkers on it) was half eaten in 2 s, and the
wanderer broken at 4:58 on run 1795106014 planet 2 re-formed 3 s later,
too fast for any answer. At `core_eat` 0.3 the same wanderer re-forms 8 s
after it breaks (third row), and a thin trickle of walkers can no longer
rebuild a boss inside a core's 15 s.

## The film

Planet 2, run 1795106014, the strategy bot (`hand=strat:econ,go=4,aim=6`),
the third row's rules: a wanderer of 544 breaks at 4:58 on the squad's
crystal, the squad down to one soldier; its core (worth 56) sits on the
crystal in thick slime; walkers pour onto it; the wanderer forms again
there at 5:06, a leech beside it.

- `core-z6.mp4`, `core-z12.mp4`: 4:57 to 5:14, a frame each half second.
- `1-boss-breaks.jpg` (4:58), `2-slime-eats-core.jpg` (5:03),
  `3-boss-re-forms.jpg` (5:06), zoom 12.

Re-run: `larder-world look=DIR run=1795106014:2 mode=squads hand=strat:econ,go=4,aim=6 at=297,297.5,...,314 centre=217,169,12`.

A staged film (`hand=hold@230,200 boss=20,330,205`) showed the slide best: a
wanderer broken 16 units from a held squad at 0:42, its core at the slime's
edge 52 units off by 0:46. At ore's eat rate it re-formed there at 0:57; at
`core_eat` 0.3 the thin slime there eats only 4 of 43 before the core cools.

## Readable cores (after the owner's play, 11-20 23:17Z)

"It isn't clear how to collect the miniboss hearts. Why do they have
circles that spread across the whole map?" The circles were the Lure
building's field rings (`View::set_steer`): a core and ore are lures
(`LureMark`), and the window drew the field's rings, every 20 cells out to
120, whenever any lure stood. They now show only while a Lure (or siren)
building stands. Collecting now reads: an alert when ore lands or a core
drops, saying to stand a squad on it; a thin ring at the soldiers' reach;
the points larger, with a shadow; a gold bar of what the squad took, and
`+N` once soldiers are in the ring.

- `4-collect-ring-z12.jpg`: a wanderer's core at 0:55 (run 1795106014
  planet 2, `hand=hold@262,181 boss=20,330,205`), its ring, 29 left.
- `5-no-map-rings.jpg`: the whole map at 0:46 of the earlier film, a core
  sliding home and no rings across the map.
