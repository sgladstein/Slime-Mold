# S2 A far smell (2026-11-17)

The simulation-use plan's S2 ([plan.md](plan.md), draft 3, Part S: "A far
smell that only hungry walkers read, for specks and for a hungry body to
reach"; the 11-15 whole-game review's W4, specks steering straight, and W7,
surplus walkers starving at home). Page:
https://claude.ai/artifact/NgZ29e1vfagrbnuuJ1AZST (source `page-s2/`, blind;
the key is below).

**Status: built on both engines and in the game as `BigParams::far_smell`
(key `far_smell=`), off until the owner's blind film.** At 1, the slime on
planet 3 reached more crystals in 2:30 on 7 of 8 seeds and fewer on none
(59 against 47, sign p 0.016), and ended bigger on all 8 (up to 45%). The
look stays the slime's: a network, with the body shifting toward the next
untouched crystal as it goes hungry.

## What was built

- **The field** (`BigDish::food_field`, `big::food_field_for`): each
  2-cell block's walking distance to the nearest untouched crystal, by the
  units' search (S1's `units_field_for`), sources the crystals' centres
  (`BigDish::fresh_crystals`).
- **Untouched** means at least `FAR_FRESH` (0.99) of the charge left. The
  first look used `ROAM_FRESH` (0.8): the slime eats slowly, so the home
  crystal and the next stayed sources for minutes, and at weight 5 the
  whole body pooled round the nearest and stopped reaching out (planet 3,
  seed 1). With 0.99, a crystal leaves the sources as soon as the slime
  eats from it, and the smell moves on to the next.
- **A hungry walker** (hunger at least `FAR_HUNGER`, 0.5: one body's shared
  food, 1 - energy / `hunger_at`) adds `far_smell` x its hunger x how much
  nearer each sensor's block is than its own to that sensor's reading, a
  block the field never reached reading `FAR_WALL` (16) farther. Shape C
  shares its food, so the body goes hungry together: at 30 s on planet 3
  about four in five walkers were over 0.5 (energy deciles 0.32 up to the
  sixth), so this is the body leaning, not a few scouts.
- **A speck** (`roam`) walks down the field instead of steering straight
  at its crystal, and lands at the first untouched crystal it reaches.
  Tracking one speck showed why specks give up today: they stand jammed in
  the crowded body (one walker a cell), not lost behind rock, so the field
  changed their landings little: over the 8 seeds 104,513 sent, 44,857
  landed and 41,291 gave up with it, against 95,003, 40,662 and 37,994
  without (more were sent, as the bigger body has more ripe walkers).
- **The GPU**: the field rides in the units' region after the units'
  blocks (blocks are a quarter of the cells, so both fit),
  `Engine::write_food_field`; the weight in one body's row's last word
  (`Params::far`, spare since `fire_weight`'s delete). The S1 upload now
  writes a word a block too (it wrote a word a fine cell).
- **The game** (`Game::time_food_paths`): with the slime reading it
  (`World::far_smell`), every 2 game seconds it sets the field started 2 s
  before and starts the next from the untouched crystals on a worker
  thread (the GPU world reads them from its crystal sums), joining it if
  late, so replays don't depend on the thread's speed.

## Tests

| Test | Result |
|---|---|
| `the_far_smell_runs_on_the_gpu_as_on_the_cpu` (llvmpipe; a grown dish with crystals and shared food, a quarter of the walkers set on their way as specks, the field set on both, one tick) | 23 untouched crystals, 3,089 specks; 0 walkers and 0 speck words off the CPU's; without the field 1,496 walkers move elsewhere, so a GPU that ignored it fails |
| `the_game_sets_the_far_smell_every_two_seconds` | after 4.5 s: 8,496 of 10,240 blocks reached, 0 at the first untouched crystal, all within 8; off, none |
| `walking_distances_are_set_at_a_fixed_step_however_slow_the_thread` (the technical lead's ask, 11-17 13:50Z) | the same seed with every worker sleeping 300 ms: the same units field, far smell and dish hash every game second for 6 s |
| `all_off_switches_on_together_play_on_both_engines` | `far_smell` 1 with the other off switches |
| `cargo test --release --locked` with the GPU twins on llvmpipe, clippy | pass (597 lib, 41 window, 51 mouldgpu); clippy clean |

## Measure

`examples/farprobe.rs`, raw lines in `s2-raw/`. Planet 3
(`levelgen::level`), the default slime (shape C) with the run's planet-3
keys (`run::enemy_keys`: surges, specks), on the CPU race world (scale 2,
25,000 walkers), no squad; the field rebuilt every 2 s and set 2 s later,
as the game does; 150 s. **Deciding measure, named before the run:**
crystals the slime had eaten from (under `FAR_FRESH`) at 2:30, `far_smell`
1 against 0, paired by seed. (A first try counted crystals eaten under
`ROAM_FRESH`; none were in 90 s, so that measure was dropped before the 8
seeds.)

| Seed | Crystals | Reached, 0 | Reached, 1 | Living, 0 | Living, 1 |
|---|---|---|---|---|---|
| 1 | 10 | 6 | **7** | 23,280 | 24,428 |
| 2 | 9 | 5 | **7** | 22,839 | 25,977 |
| 3 | 17 | 6 | **7** | 30,850 | 36,598 |
| 4 | 9 | 3 | **5** | 18,542 | 24,011 |
| 5 | 17 | 7 | **8** | 38,092 | 38,487 |
| 6 | 14 | 2 | **3** | 15,256 | 15,922 |
| 7 | 17 | 10 | **14** | 32,336 | 46,842 |
| 8 | 13 | 8 | 8 | 35,792 | 36,495 |
| sum | 106 | 47 | 59 | 216,987 | 248,760 |

- **Deciding:** more crystals reached on 7 seeds, fewer on none, 1 tie
  (sign p 0.016).
- **Bigger:** the slime ended bigger on 8 of 8 (sign p 0.008), by 1% to
  45%: more crystals feed more walkers.
- **What it looks like** (the page, seed 1): both slimes lay the same kind
  of network. With the far smell the body shifts as a lobe toward the
  next untouched crystal, eats from it, and lays tubes on; it reached the
  crystal on the east rim that the other never did.
- **The page's key** (kept off the page): **A is `far_smell` 1, B is 0**
  (a coin toss). Its film is seed 1 (`farprobe seeds=1 depths=3 secs=150
  smells=0,1 look=DIR`, `page-s2/film.py DIR far1 far0`).

## Frame cost

Off: nothing (the game builds no field; one test a walker). On: the build
runs on a worker thread every 2 s, as S1's (8-10 ms at the 6x core world
with 200 sources, `examples/raidpaths.rs`); the frame copies the ground and
starts the thread (0.24-0.29 ms there) and uploads a word a block (961 KB
at 6x). A hungry walker reads three more field words a step. Not timed on
the 1070.

## Left

- The owner's blind film decides `far_smell`; then planets 1 and 2 and a
  squad on the planet (the plan's seconds to first contact and pump health
  at 3:00) if it goes on.
- Specks still give up jammed in the body (two in five of those sent); that
  is the crowd, not the way, and is the slime-enemy thread's to tune.
- Pumped crystals leave the sources too (their charge drops), so the far
  smell never leads to a pump; the 11-15 review's W1 (pumps don't draw the
  slime) is a separate question.
- 1070 timing with the rest of the plan's passes.
