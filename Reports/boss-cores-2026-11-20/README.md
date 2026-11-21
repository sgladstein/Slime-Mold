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

## Collecting feels like collecting (2026-11-21)

The coordinator's ask after the readable-cores change: make the pickup feel
good, the sound and the moment the money comes in. Until now taking points
made no sound, and an emptied drop just shrank away, with its points shown
only on the planet's end screen.

- **Coin ticks**: every 0.15 s while soldiers take, a short glassy coin
  whose note climbs two octaves of C major pentatonic (E5 to E7) with the
  share of the drop taken (`AlertKind::Coin`, `WorldListener::coins`). The
  recorded film's loudest tone in 600 to 2,800 Hz climbs 657, 780, 880,
  1,173, 1,567 and 1,760 Hz over the drain, 5 to 10 times the band's
  median over the gunfire.
- **Paid**: when the squad empties a drop (`Starfall::cashed`) it rings
  C6, E6 and G6, quick, and a held C7 for a core (`AlertKind::Cashed`);
  the drop vanishes at once and its `+N` rises 34 pixels in gold and fades
  over 1.6 s (`CASHED_SECS`).
- **Counted**: the HUD's resource row shows the shop points found on the
  planet (gold, the shop icon), with `+N` beside them while they come in.
- The drop's points and the `+N` are drawn after the soldiers: at zoom 12 a
  squad standing on a drop hid them.

The synth has 12 tone voices, not 8, so a run of coins never takes the slot
of a soldier's fall. 25 coin ticks and 1 drop emptied in the film.

- `pickup-z12.mp4`, `pickup-z6.mp4`: real time with sound, 0:11.5 to 0:17.6.
  Planet 1, run 1, a held squad walked onto ore staged by its side:
  `larder-world look=DIR run=1:1 mode=squads hand=hold@340,70,8,376,149
  at=11.5,11.6,...,17.5 centre=376,149,12 sound=out.wav set=star_first=6
  set=star_last=7 set=star_near=0 set=star_far=4 set=star_warn=3
  set=star_gap=8 set=star_hurt=0`.
- `6-pickup-z12.jpg`: 0:14.1 (`+23` while draining, 23 at the top), 0:15.9,
  0:16.3 and 0:16.7 (`+40` rising and fading after it paid).
- `7-pickup-z6.jpg`: 0:14.1 and 0:16.2 at zoom 6.

## Shop money by planet, after the planet pacing (2026-11-21)

The coordinator's question once the pacing (#606) moved mini-bosses to
planet 2: does planet 1 still earn enough from starfall and raid cores to
buy something meaningful? Named measure: shop points a planet pays, split
into the win's `reward` and what the squad took from ore and cores, against
the shop's prices (cheapest buys 80 to 150: a common card 80, Longer
barrels, Supply drop, Quick hands 100, Armour, Faster trigger 120, Bigger
landing 150; Turrets and the Torch-and-Tinder pair 200).

**Planet 1**, six games in the cloud: the strategy bot's branch merged with
main (#610), `league=1:SEED:600 plan=rush,aim=6,drops=2` (the bot walks to
ore or a core while at most 2 walkers a soldier are near it), seeds 1 to 6;
rows in `income/league-rows.txt`, script `income/run1.sh`. Seeds 7 and 8
were cut: lavapipe played these at a fifth of real time.

| seed | end | falls landed | cores | ore taken | core taken | slime ate |
|---|---|---|---|---|---|---|
| 1 | won 3:07, 2 stars | 1 | 1 | 0 | 0 | 44 |
| 2 | held 10:00 | 3 | 2 | 36 | 0 | 244 |
| 3 | won 4:58, 2 stars | 1 | 1 | 0 | 0 | 116 |
| 4 | lost 3:48 | 2 | 0 | 0 | 0 | 80 |
| 5 | held 10:00 | 3 | 1 | 0 | 0 | 120 |
| 6 | held 10:00 | 3 | 2 | 0 | 0 | 247 |

The squad took 36 points in 6 games and the slime ate 851. The two wins
paid only `reward(1, 2)`, 250: two of the cheap buys, so planet 1's shop
still buys something, but from the win alone. Starfall and cores add
nothing a player can count on there:

- A fall lands by slime where the grid shows some (`fall_spot`), and walkers
  eat ore at ore's `eat` 1 a second each; the bot never found one with 2 or
  fewer walkers a soldier round it.
- Planet 1's cores are now only fruit and raid cores (no mini-bosses), 1 or
  2 a game, and a core slides home into thick slime by design.
- Falls are drawn between 0:50 and 10:00, so a planet won in 3 to 5 minutes
  sees 1 of its 3 or more.

**Planets 2 and 3**, from the bot league on the owner's PC (its 14-0-c and
14-0-d jobs, before #606, which changed nothing past planet 1 for drops;
those bots did not chase drops): planet 2, 55 wins of 132, `reward` 279 on
average, ore 6.7 and cores 12.6 a win; planet 3, 48 of 130, `reward` 286,
ore 15.1 and cores 33.0 a win, the slime eating 156 and 225. One cloud
planet 2 game with chasing (seed 1, won 6:18 with 3 stars): 58 core points
on 300. **Planet 4** was not measured: the cloud plays it far too slowly
and the league has no planet 4 rows with drops.

So on every planet the win pays nine tenths or more of the money, and the
pickups are a small extra the slime mostly eats.
