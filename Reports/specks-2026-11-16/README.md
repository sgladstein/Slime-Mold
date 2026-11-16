# Specks: ordinary slime breaks away, the bronze goes (2026-11-16)

The owner, 2026-11-16 04:58Z, answering the mini-boss thread's questions
(answer 2): "Remove the bronze, keep the specks (specks and mini-boss are
separate but both happen. Specks more often early game)"
(`wiki/decisions-game.md`). The coordinator passed it to the slime-enemy
thread, which owns thickening and breakaway; condensing came from the
condense thread.

Page for the owner (planet 1 with and without specks, whole-map
time-lapse to 4:00; stills of planet 1 at 3:00 and planet 2 at 5:00):
https://claude.ai/artifact/PjaBjAmY1FC2qzZb7fDbzQ (source `page/`; clips by `raw/make_clips.py` from the films of
`raw/film.sh`).

## What changed

- **No bronze in play.** Shape C no longer thickens (`thicken` 2 is gone
  from `SHAPES`), the run no longer condenses from planet 2 (`condense`
  300 and the `thicken` curve are gone from `run::enemy_keys`), and the
  start menu's Condense and Thicken options are gone (`ENGINE_OPTS`). The
  engine keeps both rules, so `thicken=K` and `condense=N` still film
  them. `Reports/dead-ends.md` has the entry and what went with them.
- **Breakaway from ordinary slime** (`BigParams::roam` with `condense` 0,
  both engines). An ordinary walker gets a chance every `roam` steps, on
  the steps where (step + slot) is a multiple of `roam`, so a few of the
  body's walkers have their chance each step and never all at once. It
  takes the chance at `roam_share` if it then stands ripe as condensing
  reckons it: on a crystal it took from, or with `condense_crowd` (20)
  walkers within `condense_r` (2). It then runs for the nearest crystal
  with four fifths of its charge left at least `roam_far` (30) cells off,
  as a breakaway runs, and stays ordinary (one life, its own bite). It
  lands within `ROAM_LAND` of the crystal's centre, or when the crystal
  runs dry, or gives up where it stands after `SPECK_TRIP` (600 steps,
  20 s). With `condense` on, condensed walkers break away as before.
- **The run's curve** (`run::enemy_keys`, `ROAM` and `ROAM_SHARE`): every
  planet sends specks, a chance every 300 steps (10 s), taken at 0.05 on
  planets 1-2, 0.04 on 3-4, 0.03 on 5-6 and 0.02 on 7-8. Planet 1 sends
  them too (D2 had kept planet 1 plain), since the owner wants specks most
  early; one number to change if the first planet should stay plain.
- `BigParams::roam_share`'s default is 0.05 (was 0.5, set for condensed
  pieces), so the start menu's Breakaway, which sets `roam` alone, sends
  a twentieth in the sandbox too.
- The headless readout counts walkers on their way ("N breaking away").

## How many specks

Measured in the window's headless run (`raw/arm.sh`;
[`raw/runs/`](raw/runs/README.md) has the readouts and which build made
each). Planet 2 is run seed 2 (the
Sprawler), planet 1 run seed 1; the squad hand, one game each. "Away" is
walkers on their way at that moment.

| Build | Planet | Share | Away at 0:20 / 0:30 / 1:00 / 1:30 / 2:00 | Walkers at 2:00 |
|---|---|---|---|---|
| first (no give-up) | 2 | 0.5 | 6,743 / 9,498 / 12,300 / 12,174 / 11,493 | 16,098 |
| first | 2 | 0.1 | 1,247 / 2,275 / 4,447 / 6,215 / 7,231 | 15,680 |
| give-up after 20 s | 2 | 0.5 | 6,743 / 9,453 / 5,560 / 4,824 / 4,575 | 16,355 |
| give-up | 2 | 0.1 | 1,247 / 2,275 / 1,783 / 1,910 / 1,772 | 15,834 |
| give-up | 2 | 0.05 | 596 / 1,063 / 966 / 1,200 / 1,078 | 15,985 |
| give-up | 1 | 0.05 | 385 / 648 / 794 / 711 / 1,024 | 13,497 |
| **final** | 2 | 0.05 | 1,155 / 1,150 / 961 / 1,230 / 972 | 15,952 |
| **final** | 1 | 0.05 | 862 / 744 / 915 / 806 / 1,071 | 14,158 |
| none (`roam=0`) | 2 | | | 15,255 |

- The first try, half of each chance, sent most of the slime away at
  once: 12,300 of 17,406 walkers on their way at 1:00 on planet 2. Many
  never landed: a crowd at the crystal or a wall kept them outside the
  landing circle, and a walker on its way neither withers, panics, raids
  nor surges. The give-up cut that to about 5,000; a twentieth of the
  chances brings it to about 1,000, 6% of the body.
- The final build starts sooner (540 away at 0:10 on planet 2, none
  before) because a walker no longer has to stay ripe 10 s before its
  first chance.

## What it does over five minutes

The same two games to 5:00, with the run's specks and with `roam=0`:

| Planet | Arm | Walkers 1:00 / 2:00 / 3:00 / 4:00 / 5:00 | Soldiers 1:00 / 2:00 | Food eaten by 5:00 | Away at 5:00 |
|---|---|---|---|---|---|
| 2 | none | 16,975 / 15,255 / 16,952 / 20,553 / 25,716 | 3 / 0 | 911 | |
| 2 | specks | 17,029 / 15,952 / 17,773 / 21,608 / 27,428 | 5 / 0 | 967 | 1,680 |
| 1 | none | 10,472 / 11,171 / 15,372 / 19,553 / 23,900 | 8 / 1 | 1,494 | |
| 1 | specks | 11,289 / 14,158 / 18,493 / 22,788 / 26,239 | 6 / 0 | 1,575 | 2,481 |

- With specks the slime is 7% heavier at 5:00 on planet 2 and 10% on
  planet 1, and eats 5-6% more. One seed a planet, so a direction, not a
  size.
- On planet 1 (a sheet of slime) the specks are easy to see: at 3:00 one
  stream crosses the river from the top body to the bottom one, and
  another runs to the crystal on the right, where a new colony has grown
  that the plain game does not have yet (page). On planet 2 (a mesh) the
  specks look like the slime's own exploring streams; in a near view of
  one channel, frames with and without were hard to tell apart.
- On planet 1 the squad hand lost its last soldier by 1:45 with specks,
  by 3:00 without; planet 2's by 1:15 and 1:30. The scripted squad dies
  on both planets either way; it is a test hand, not the owner's play.
- Smoke: no crash or stall in any run; every game lasted past 1:00 with
  soldiers alive; the slime grows on every run, so idle cannot win.

## Frame cost

Not timed on the GTX 1070 yet. From the D2 timing
(`Reports/breakaway-run-2026-11-15/README.md`, *Frame cost*): the dense
passes already ran on every planet, for shape C's thickening, and still
do (`Engine::condenses` is on with `roam` alone). Condensing made the
`ripen` pass count every walker's crowd each step, 0.405 ms on the core
world at 94,367 walkers against 0.090 ms with thickening alone. Specks
count a walker's crowd only on its chance step, a 300th of the walkers a
step, so `ripen` should cost about what thickening alone did. Without
condensing's absorbing, the body keeps more walkers to sense, breed and
draw. So the game should cost what the table's arm without condensing
cost: no change on planet 1, planet 2's step 0.95 ms against main's 0.98,
and the core world's step 6.28 ms against main's 6.18 (0.1 ms dearer, for
a third more walkers), the frame unchanged within the runs' spread. A
1070 timing goes through the PC runner after the merge.

## Tests

- `big::tests::ordinary_slime_breaks_away_for_a_far_crystal` (CPU): off
  changes nothing; on, 3,340 walkers left in 600 steps of a small dish,
  145 landed, none condensed, 940 left a second time; a walker one step
  short of `SPECK_TRIP` keeps going and then gives up.
- GPU twin, dish "specks" in `the_world_matches_the_cpu_for_one_tick`
  (llvmpipe): 0 dense words off, 0 condensed, 3,608 on their way, 55 left
  and 123 landed in the tick; a sixteenth of the planted breakaways sit
  one step from giving up. The first build's twin caught a planted GPU
  landing bug.
- `run::tests::the_slime_gets_fiercer_deeper`: specks no more often
  deeper, every planet has `roam`, no planet condenses or thickens, planet
  1's rate over twice the core world's, and shape C has no `thicken`.

## What went with the bronze

Thickening was this thread's answer to the owner's 11-15 08:34Z "keep
getting stronger by holding onto its crystals and getting more of them":
a fed slime with no room grew past its slots, up to twice
(`Reports/thicken-2026-11-15/README.md`). Without it, C stops at its slots
again. Specks give some of that back by reaching fresh crystals sooner
(the table above), and the mini-boss thread's bosses are the strength
that grows deeper. If the owner wants the held-crystal strength back
without the bronze, one option is a slot cap that grows with the
crystals the slime holds.

## Open

- The owner's playtest: whether the specks read on planet 1 and planet 2,
  and whether planet 1 should send them.
- A look that tells a speck from an explorer, if planet 2's mesh hides
  them: the view could draw a walker on its way from its state (the
  dense word's breakaway bit), which would be the simulation and not a
  painted effect. Not built; it touches the slime look's lines.
- The 1070 timing.
- The mini-boss thread (lumps in the dense word's bits 25-27) rebases
  over this or this over it, whichever merges second; specks write a
  walker's word only when it leaves, lands or gives up.
