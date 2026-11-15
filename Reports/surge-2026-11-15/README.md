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

Frame cost: the units are searched only during a surge (2.5 s in 30). The
1070 timing is not done yet. It would compare a bench with the clock
always in a surge (`surge=76 surge_lean=45 surge_dash=30`) against
`surge=0`.

## Not done

- The tell is the creep alone. A brighter front while it gathers (more
  trail laid, so the look shows it) would read better; to try if the
  owner can't see it coming.
- Progression by planet depth: shorter periods, longer lunges and lower
  `thicken` on deeper planets, with the run thread (it owns `run.rs`).
- One shared clock. Per-crystal clocks, or a surge strength that grows
  with the thick share, are the next levers if a single beat reads as
  mechanical.
