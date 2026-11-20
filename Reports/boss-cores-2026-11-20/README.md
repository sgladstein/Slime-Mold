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

With the core left where the boss broke, it lay inside the guns' reach (15 to
22 units from the nearest soldier) and 0 to 4 walkers lived to reach it in
its 15 s; a held squad never fetched it either. So a core now slides to the
nearest thick slime within 60 dish cells, and the squad has to reach in for
it. The second row is not paired with the first: the Starfall thread raised
the slime's `eat` from 0.25 to 1.0 between them, and that moves every game.
Neither bot goes for drops; the 75 points are cores that slid past soldiers.

## The film

Planet 2, run 1795106014, a held squad (`hand=hold@230,200`), a wanderer
staged at 0:20 (`boss=20,330,205`): it breaks on the squad's fire at 0:42
16 units out; its core (worth 43) slides 52 units to the slime's edge by
0:46; walkers eat it; the wanderer forms again there at 0:57 with 70
walkers and sets off again at 1:03.

- `core-z6.mp4`, `core-z12.mp4`: 0:41 to 1:04, a frame each half second.
- `1-core-runs-home.jpg` (0:45), `2-slime-eats-it.jpg` (0:52),
  `3-boss-re-forms.jpg` (0:57), zoom 12.

Re-run: `larder-world look=DIR run=1795106014:2 mode=squads hand=hold@230,200 boss=20,330,205 at=41,41.5,...,64 centre=290,182,12`.
