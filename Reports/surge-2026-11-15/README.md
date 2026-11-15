# The body surges (2026-11-15)

The slime-enemy thread, after the owner's answers of 11-15 08:30-08:34Z.
Page: `page/index.html` (published as an Artifact, linked in
`Reports/pages.md`).

## Why

The owner, 11-15 08:30Z: some slimes "just hang at their home crystal,,
they don't spread or grow much. So their is no rush. They don't attach my
pumping crystals so no need to defend", and the enemy needs "more
interesting progression ... and increase in challenge ... especially when
your guns get stronger". The review that morning
(`Reports/slime-review-2026-11-15/README.md`) found that a body joined to
its crystals only follows its food. Only the soldiers' scent, the home
pull, panic (a cut-off part) and the tiers' specials draw it at the
player. A squad that stood at the gun's range from the body was safe as
long as it kept shooting.

## The rule

`BigParams::surge_every` (steps, 0 off), `surge_lean`, `surge_dash` and
`surge_haste`, on both engines, run on one clock for the whole dish. The
first surge comes a whole period after the start. In each surge, every
walker joined to the body (not panicking, not breaking away) that is
within `panic_r` of a unit (a soldier or a building, the same list a
panicking walker charges) heads for the nearest one. It creeps for
`surge_lean` steps at `panic_creep` of its pace, which is the tell: the
front facing the unit swells toward it. Then it lunges for `surge_dash`
steps at `surge_haste` times its pace, steering by its sensors alone, as
a panicking walker runs. The reach is panic's, the gun's range plus
`Rules::panic_margin` (29 at today's 24), so a squad in range of the
slime is always in the surge's reach. A range card moves both.

**On in shape C at 900** (every 30 s; lean 45 steps = 1.5 s, lunge 30
steps = 1 s at twice the pace). The start menu has a Surge option (0, 600,
900, 1800), and `surge=0` turns it off.

GPU: one new `Params` row, 944-959 (`SURGE_AT`: `surge_every`,
`surge_lean`, `surge_dash`, `surge_haste`), booked with the technical lead
and the integration thread. There are no new passes or bindings. The
surge sits in `panic_to`: a walker that is not panicking calls `surge_to`,
which searches the units only while the clock is in a surge.

## Measured

Window, Fork, squad mode, seed 1, software Vulkan (`raw/`):

- **Edge hand** (the squad walks to the body's edge by its home crystal
  and fires; `raw/edge-off.txt`, `raw/edge-on.txt`). Up to 30 s the two
  runs are the same. During the first lunge (31.5-32.5 s) the soldiers
  touching slime reach 5.3 on average, 13 at most, against 0.6 and 2
  without it. At 32.5 s 7 of 9 soldiers stand and at 34 s 6, against 9
  of 9. At 45 s both have 4. Without surges the squad is wiped at 51 s, as it walks into the
  body. With them it is wiped at 61.5-62 s by the second surge (touching
  28), from 3 soldiers at 60 s. One seed and one scripted hand, so this is
  not a balance reading. It shows the surge reaches and holds a squad
  that stands at the edge.
- **Scripted squads hand, four pumps** on the slime's arms' crystals
  (165,150), (347,150), (60,225), (450,225) (`raw/pump-*.txt`). Without
  surges C already wears all four down by 60 s, because they stand on
  crystals it grows over. With surges the (165,150) pump falls by 45 s
  (46 hp left at 45 s without), and the pumps feed 1,070 food against
  1,252. The squad, at (330,143), has 7 soldiers at 60 s against 8.
- **Scripted squads hand, no pumps** (`raw/squads-*.txt`, stopped at
  75 s). The squad stands in the clearing more than 29 cells from the
  body, so it is out of reach: squad health 65 against 71 at 75 s, from
  stray walkers. Out of reach is out of reach, so the surge does not
  replace the body growing toward the player.

Twins: the squad twin `the_squad_matches_the_cpu_for_one_frame` gets two
cases, "the body surges at the squad" (586 calm walkers surge) and "a
cut-off part charges and the body surges" (66 calm walkers surge, 520
stale ones charge). Both have 0 walkers off and 0 slots apart. All four
surge words are nonzero, and the clock is set so the frame's first step is
the lunge's first. A planted swap of lean and dash on the GPU fails it.
CPU unit test `the_body_surges_at_a_unit_in_reach`: 115 walkers in reach
close from a mean of 20.8 to 9.4 fine cells in 20 ticks, against 20.3
without the surge. With no unit in reach, and before the first surge,
the hash is unchanged.

Frame cost (GTX 1070 at ca54aa2a, `larder-world bench=20 mode=squads
hand=squads seed=1`; raw `claude/project-thread-26drse` cbc79fd7,
`runner-out/surge-pr343-ca54aa2a/`): with `surge=0` every frame is p50
4.60 and p95 5.21 ms (GPU tail p95 4.40). With the clock in a surge
almost all the time (`surge=76 surge_lean=45 surge_dash=30`) it is p50
4.60 and p95 5.16 (GPU tail p95 4.40). No cost shows. The units are
searched only during a surge (2.5 s in 30). Both twins pass on the 1070.

## By depth (run planets)

`run::enemy_keys(depth)`, applied in `run_shape` after shape C's keys and
before the tier's and the menu's moved options. The run thread agreed the
curve and asked for it to sit on the shop's clock line.

| Planet | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| Surge every (s) | 40 | 40 | 30 | 30 | 25 | 20 | 20 | 15 |
| Lunge (s) | 1.0 | 1.0 | 1.0 | 1.2 | 1.4 | 1.5 | 1.5 | 1.8 |
| Thicken at (births' energy) | 2.0 | 2.0 | 1.8 | 1.7 | 1.6 | 1.5 | 1.4 | 1.3 |

Planet 1 is gentler than the sandbox's C (30 s, 1 s, 2.0) and the core
world fiercer. Test `run::tests::the_slime_gets_fiercer_deeper`. (PR #347.
Since food-driven surges, below, the period's ratios became the food a
surge needs and twice the period the longest wait.)

## Food-driven (later on 11-15)

Agreed with the game designer after #343: a fixed beat ignored what the
owner asked for (11-15 08:34Z, the slime should "keep getting stronger by
holding onto its crystals and getting more of them").

**The rule.** The game keeps a meter (`Game::time_surges`, each frame on
both worlds): what the slime ate this frame is the crystals' charge gone
since the last frame, less what the units took (pumps and soldiers), as
food (`Rules::charge` a food). A surge starts on the next step once the
meter holds `Rules::surge_food` and `Rules::surge_min` game seconds have
passed since the last surge (or the landing), or once the world's
`surge_every` has passed, eaten or not, so it is the longest wait. The
meter then empties. The engine gets the step through `BigParams::surge_at`
(`World::set_surge_at`), and the lean and lunge run once from it.
`SURGE_WAIT` (u32::MAX) holds the body calm before the game's first surge.
`surge_every` 0 is still off, and `set=surge_food=0` gives the fixed clock
back.

**On:** 150 food, a floor of 10 s, and shape C's `surge` raised from 900 to
1800 (a longest wait of 60 s). The menu's Surge option is now the longest
wait (0, 900, 1800, 3600).

GPU: no new row. `surge_lean` and `surge_dash` share the row's second word
(low and high half, each at most `SURGE_MAX` 0xFFFF), and the third word
is `surge_at`.

### Measured

Window, Fork, squad mode, seed 1, software Vulkan, to 3:00 (`raw/food-*`,
`raw/eating-*`).

What the slime eats, with no soldier near (idle hand, fixed clock; food a
second over each span):

| 0-10 s | 10-20 | 20-30 | 30-40 | 40-50 | 50-60 | 60-75 | 75-90 | 90-120 | 120-150 | 150-180 |
|---|---|---|---|---|---|---|---|---|---|---|
| 2.3 | 2.8 | 3.7 | 4.7 | 5.9 | 6.9 | 8.4 | 9.7 | 9.8 | 10.4 | 11.2 |

It eats more as it spreads over its crystals, so the surges come faster as
the game goes on:

| Run | Surge starts (s) | In 3:00 |
|---|---|---|
| Fixed clock (`surge=900 set=surge_food=0`) | 30, 60, 90, 120, 150, 180 | 6 |
| Food, idle hand | 42.7, 64.5, 81.3, 96.7, 112.1, 127.3, 142.1, 156.3, 169.9 | 9 |
| Food, four pumps on its arms' crystals | 46.9, 69.1, 84.9, 100.3, 115.7, 131.1, 146.5, 161.9, 177.2 | 9 |

The gaps shrink from 22 s to 14 s. The pumps take enough that each surge
comes 3-7 s later. Its eating barely moves with the surges (1,466 food
eaten by 3:00 against 1,474 on the clock).

Edge hand (the squad at the body's edge firing): the first surge comes at
43.7 s, against 31.5 on the clock, on a squad 13 s more worn (mean health
44 at 30 s), and wipes it at 46 s. On the clock it was wiped at 62 s,
and at 51 s without surges. One seed and one scripted hand, so it is a
tuning note, not a balance reading.

Twins: the squad twin gets a third surge case, "the game times the
surge". `surge_at` puts the frame's first step at the lunge's first, while
the clock alone (a period of 1,000,000 steps) would be calm. It gives 586
calm walkers surging, 0 walkers off and 0 slots apart. A planted GPU bug
that ignores `surge_at` fails it, and so does one that swaps the packed
lean and lunge. CPU tests: `the_body_surges_at_a_unit_in_reach` gains the
phases step by step (the clock's beat, then the game's once from its
step, held by `SURGE_WAIT`, off with `surge_every` 0).
`the_slime_surges_when_it_has_eaten` (race world) checks three things.
With `surge_food` 0 the world keeps its clock. Fed (a meter that fills in
a second), 14 surges come exactly 5.0 s apart at the floor. Starved (a
meter it can't fill), they come 20.0 s apart at the ceiling. The slime
there eats 0.21 food a second.

By planet (`run::surge_food`, `enemy_keys`): the food a surge needs is a
share of the sandbox's 150 times the level's walkers (a bigger slime
eats more), and the longest wait is twice the old beat:

| Planet | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| Food a surge (x walkers) | 200 | 200 | 150 | 150 | 125 | 100 | 100 | 75 |
| Longest wait (s) | 80 | 80 | 60 | 60 | 50 | 40 | 40 | 30 |
| Lunge (s) | 1.0 | 1.0 | 1.0 | 1.2 | 1.4 | 1.5 | 1.5 | 1.8 |
| Thicken at | 2.0 | 2.0 | 1.8 | 1.7 | 1.6 | 1.5 | 1.4 | 1.3 |

The shop's clock line says "it surges as it feeds, at least every N s".

Frame cost and twins (GTX 1070 at 0bb20d4d, PR #350, `larder-world
bench=20 mode=squads hand=squads seed=1`; raw `claude/project-thread-26drse`
a9f33a00, `runner-out/surge-pr350-0bb20d4d/`): both twins pass (the squad
twin's game-timed case included). Every frame p50 4.73 and p95 5.35 ms with
food-driven surges, 4.73 and 5.34 with `set=surge_food=0`; GPU tail p95
4.53 both. No cost shows. Neither run surged in its 20 s (identical t 20 s
lines), so the meter's per-step cost is what was timed, not a surge's.

## Not done

- The tell is the creep alone. A brighter front while it gathers (more
  trail laid, so the look shows it) would read better; to try if the
  owner can't see it coming. The game designer prefers the slime swelling
  or brightening as the meter fills over a HUD meter.
- The meter counts net eating: with `crystal_regrow` on (off in the game)
  regrowth hides some of it, and the player's own mould's eating counts
  too (`own=1`, off by default).
- One surge for the whole body. Per-crystal surges, or a surge strength
  that grows with the thick share, are the next levers if a single beat
  reads as mechanical.
