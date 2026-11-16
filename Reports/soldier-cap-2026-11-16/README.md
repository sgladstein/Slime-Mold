# The soldier cap: 40 to 200 (2026-11-16)

Owner, 04:50Z: "Start a thread to evaluate increasing the solider limit.
What is the realistic maximum we could have". After the first findings the
owner picked, on the card, **"Flat 200, the realistic max"** (05:06Z), and
wrote: "Maybe scale later in when releasing but I want to test the max"
(05:06Z).

## What held the cap at 40

| Limit | Before | What happened past it | Now |
|---|---|---|---|
| `Rules::max_squad` | 40 (the M2 design number, 11-11) | `Game::buy` refuses | 200 |
| `squad::UNITS_MAX`, the squad frame's unit slots (CPU and GPU) | 96, shared by soldiers, the home and up to 40 buildings (`BuildRules::max`) | units past it are cut from the frame silently: a soldier past about 55 would not bite, harvest or aim | 256 (200 + 1 + 40 = 241) |
| `squad::SHOTS_MAX`, bullet legs in flight | 256 | the newest bullet is dropped (`Game::bullets_dropped`) | 1024 |
| `marks::MAX_MARKS`, marks a frame | 2048 | the newest marks are dropped; the dead are drawn first and stay all level, so the living went first | 4096 |
| `SQUADS_MAX` | 16 | no 17th squad | unchanged; number keys 1-9 |

`engine.wgsl` spells the frame's sizes out (`Squad`'s arrays, the sleep
gas's `GAS_CALM` and `GAS_SLOTS`, which follow `UNITS_MAX`); a new test,
`squad::tests::the_shader_holds_the_frame`, fails if they drift apart.

The cap binds early: the owner on the old hand drain had "a full squad of 40
within the first 30 seconds" (11-15, [pump economy](../pump-economy-2026-11-15/README.md)),
and the greedy hand after pumps took over still had 22 by 1:30 on one seed.

## Measured: the soldiers' turn on the CPU

**Deciding measure (named before the run):** the game step's CPU time
outside the world's frame (`World::frame`, the GPU's work in the window),
mean and p95 ms a step, at 40 to 240 soldiers. Steps run 15 a second, so a
step lands on about one frame in four at 60 fps (16.7 ms).

`examples/soldiercap.rs` plays a run planet (`levelgen::level`) on the CPU
big dish in squad mode with the window's squad-mode rules, buys every
soldier at once, splits them over 8 or 16 squads and sends them on
(`fight=1`: every squad fights at the slime's thickest blocks; else half
fight at crystals and half clear them, a new pick every 15 s). It wraps the
world to time its frames apart. The run used `UNITS_MAX` 512 and
`SHOTS_MAX` 2048, raised locally, so no soldier was cut from the frame.
Seed 1, 40 game seconds, cloud container (4 cores). Raw output:
[cpu-sweep.txt](cpu-sweep.txt).

| Soldiers | Planet 1 (400 x 248), walking | Planet 1, fighting (16 squads) | Planet 8 (1256 x 784, 6x), walking |
|---|---|---|---|
| 40 | 0.29 / 0.65 | 0.89 / 3.58 | 6.65 / 10.2 |
| 80-100 | 0.42 / 1.01 (80) | 1.35 / 3.94 (100) | 7.44 / 10.4 (80) |
| 120 | 0.55 / 1.24 | | 9.10 / 13.2 |
| 160 | 0.91 / 3.42 | | 9.64 / 13.7 |
| 200-240 | 1.24 / 3.26 (240) | 2.21 / 4.72 (200) | 12.1 / 17.0 (240) |

(mean / p95 ms a step.) A soldier costs about 0.005-0.01 ms a step on
planet 1 and 0.03 ms on planet 8. Most of planet 8's cost is there at 40
soldiers: the walking maps (`MapBuild`, a fixed `MAP_POPS` a step, the
levelgen thread's code), not the soldiers. Squads share maps (`MAP_NEAR`),
so more soldiers in the same squads ask for few more maps. On planet 8 no
squad reached the slime in 40 s (0 shots), so the fight's cost there is not
measured; planet 1's fight added about 0.6-1 ms over walking at every size.

Bullets: 200 riflemen fighting had at most 129 bullets in flight (40: 32,
100: 78). Buckshot fires four pellets a shot and a scattergun up to six
shots, so a big buckshot army would pass the old 256; 1024 holds a few
hundred buckshot pellets.

**Realistic maximum: about 200.** Past about 250 the soldiers' own turn on
a big planet (12-17 ms a step at 240) starts to cost frames by itself.
Beyond that the turn's O(n) scans per soldier (`Game::members`, elbow room's
pairs) would want a grid or a cache.

## Measured: the GTX 1070 (11-16, after #377 merged)

`larder-world bench=60 hand=greedy map=planet:P:1 mode=squads
set=start_bank=1000000 set=max_squad=N passes=30` on Scott-Desktop, the
cap branch at ceaf153 against main at e305994, PC to itself (Edge closed,
GPU 0% before each run), each run once, not interleaved; the card's clock
dipped at the start of three runs (p1 n100 835 MHz, p8 n40 797, p8 n200
1012; `1070/runner-log.txt`). Raw: [1070/](1070/). The greedy hand buys
the whole army in the first second and drains the biggest crystal.

| Run | Every frame p50 / p95 / max | Frames with a step p50 / p95 | CPU step p50 / p95 | GPU step, every pass |
|---|---|---|---|---|
| main, planet 1, 40 | 4.89 / 5.72 / 9.16 | 5.54 / 5.82 | 0.95 / 1.08 | 0.885 |
| cap, planet 1, 40 | 4.87 / 5.72 / 8.68 | 5.56 / 5.85 | 0.99 / 1.09 | 0.887 |
| cap, planet 1, 100 | 5.34 / 6.89 / 10.54 | 6.32 / 7.13 | 1.32 / 2.10 | 0.930 |
| cap, planet 1, 200 | 5.87 / 8.33 / 11.36 | 7.60 / 9.06 | 2.09 / 3.31 | 1.266 |
| main, planet 8, 40 | 5.83 / 7.13 / 18.56 | 6.49 / 7.83 | 1.31 / 2.69 | 3.371 |
| cap, planet 8, 40 | 5.88 / 7.18 / 18.81 | 6.56 / 7.88 | 1.32 / 2.69 | 3.391 |
| cap, planet 8, 100 | 5.97 / 9.80 / 40.33 | 8.74 / 12.10 | 2.94 / 6.66 | 3.352 |
| cap, planet 8, 200 | 6.14 / 13.65 / 58.24 | 12.17 / 14.86 | 6.32 / 9.35 | 3.587 |

(ms.) **The bigger buffers cost nothing:** main and the cap branch at 40
soldiers are within 0.05 ms everywhere. **The GPU barely grows:** the
per-unit passes at 200 soldiers are smell 0.16 ms, harvest 0.13 and units
0.07-0.13 a step (0.03-0.05 each at 40), so the whole GPU step grows 0.4 ms
on planet 1 and 0.2 ms on planet 8. **The CPU step is what grows**, about
0.03 ms a soldier on planet 8 (1.3 -> 6.3 ms p50 from 40 to 200), as on the
cloud. Planet 1 at 200 stays well inside 60 fps (a step's frame p95 9.1
ms). Planet 8 at 200 holds 60 fps at the median, but a step's frame p95 is
14.9 ms, close to 16.7, and two runs had single long frames (40 ms at 100,
58 ms at 200, in the record and GPU tail, not the CPU step: record max 29
ms, tail max 32 ms at 200), not seen at 40; once each, unexplained.

### The long frame, repeated (11-16 06:07Z)

The technical lead asked whether the 58 ms frame repeats. Six more runs on
planet 8, PC to itself (raw: `1070/rep-*.txt`, `runner-log-rep*.txt`):
ceaf153 at 200 and main at 40, twice each, then 313ce4f, which prints the
three longest frames with their game time, at 200 and 40, once plain and
once with `viewpasses=30`, alternating.

| Run | Every frame p50 / p95 / max | GPU tail max | After 1 s: CPU step max |
|---|---|---|---|
| cap 200, 1 / 2 | 6.04 / 13.50 / 24.1; 6.05 / 13.52 / 24.2 | 12.6; 12.3 | 13.6; 13.6 |
| main 40, 1 / 2 | 5.84 / 7.09 / 14.4; 5.89 / 7.15 / 14.5 | 6.0; 6.1 | 7.9; 7.8 |
| 313ce4f 200, plain / viewpasses | 6.09 / 13.49 / 20.5; 6.13 / 13.56 / 32.0 | 10.5; 30.5 | 13.8; 13.7 |
| 313ce4f 40, plain / viewpasses | 5.88 / 7.17 / 18.9; 5.93 / 7.21 / 18.7 | 10.5; 10.4 | 7.9; 8.0 |

**The 58 ms frame did not come back:** a one-off on the PC. The longest
frames are the first game second's (pipelines and buffers first touched)
and, in one run, three frames at 40.7 s whose GPU tail was 21-31 ms, seen
once in six runs. What holds in every run: at 200 soldiers the worst CPU
step is about 14 ms (8 ms at 40), so the frames that carry it run 19-20 ms,
a dropped frame on planet 8 when the step is at its worst. The p95 at 200
(13.5 ms) repeats to 0.1 ms.

## Measured: the CPU step by part (planet 8)

`perf record` of `soldiercap depth=8` at 40 and 200 soldiers (cloud CPU,
line tables and frame pointers; raw: [perf-planet8.txt](perf-planet8.txt)),
the soldiers' turn split by its biggest parts, ms a step (shares turned
into ms over the run's step time, so about +-0.3):

| Part | 40 soldiers | 200 soldiers |
|---|---|---|
| Walking maps (`MapBuild::advance`) | 4.9 | 6.0 |
| Walking (`walk_line`, `place_at`, `goal_ground`, `standable_cells`; elbow room and distances, inlined, in `hypotf`) | 1.6 | 5.0 |
| Squads (`rank_squad`, `members`, `guide`) | 0.3 | 0.6 |
| Whole turn | 6.4 | 11.4 |

No shots in this run (the squads had not reached the slime in 30 s); on
planet 1 a fight added 0.6-1 ms over walking at every size (CPU table
above). The walking-map build is the biggest block and mostly fixed (a
`MAP_POPS` budget a step while any map builds; the levelgen thread's code);
the part that grows with soldiers is each soldier's walk. The squad
movement rework thread (11-16) is rewriting that walk and has taken
`members` and `rank_squad`'s sort and elbow room's all-pairs loop.

## Smoke: how fast the greedy hand grows an army (the game designer's ask)

The game designer (11-16): the cap of 40 was the snowball's only brake, so
at 200 a greedy hand might reach 100+ soldiers by 4:00. `soldiercap
greedy=1` plays the window's `greedy` hand (buy whenever the bank can pay,
every soldier drains the biggest crystal, no pumps) from the normal bank,
cap 200, planets 1 and 3, seeds 1-2, 6 game minutes, on the CPU big dish
(its slime is not the window's shape C, so this is a tuning note only).
Raw: [greedy.txt](greedy.txt).

| Planet, seed | 10 | 20 | 40 | 80 | At 6:00 |
|---|---|---|---|---|---|
| 1, 1 | 0:12 | 0:58 | 1:58 | never | 45 |
| 1, 2 | 0:09 | 1:11 | never | never | 29 |
| 3, 1 | 0:20 | 1:13 | never | never | 29 |
| 3, 2 | 0:20 | 1:08 | never | never | 31 |

Hand draining alone does not snowball past the old cap: each run stopped
growing by about 3:00, when the crystals the hand drains stopped paying.
Pumps are the main income since 11-15, and a hand that builds them was not
run. The owner's ruling on a brake stands (whole-game review D7, 11-15
21:41Z: "play first, no rush brake yet"), so no brake was added; the
designer's two options (a cap on soldiers draining one crystal, or a
rising soldier price) wait for the owner's play.

## Not measured yet

- **Balance.** The slime, the shop and the AI were tuned at 40. The player
  AI (`larder_world_ai`) buys to the cap whenever its bank holds `spend`
  soldiers' worth, so at 200 a rich bank goes on soldiers before turrets
  (its test `a_rich_hand_spends_its_bank` now pins the old cap of 40). Build first,
  balance later (owner, 11-12): the cap is in for the owner to play.
- **Reading 200 soldiers on screen** at zoom 6 (the art guide): the owner's
  playtest, and the art lead's if the look breaks.
- **A pumping army** (pumps plus buying), the snowball's real income.

## Tests of the new sizes

`squad::tests::the_shader_holds_the_frame` checks `engine.wgsl`'s sizes.
On both engines (`mouldgpu`, llvmpipe here; the 1070 in the PC batch):
`the_squad_frame_holds_its_cap_on_the_gpu` fills all 256 unit slots and
every unit's outputs match the CPU's; `the_squad_frame_holds_every_shot_on_the_gpu`
(11-16) fires 1,024 shots in one frame, every hit and kill matches, and the
shots past the old 256 hit and kill (its planted check: a shader or buffer
still sized for 256 drops them or reads them from the units).

## Reproduce

```
cargo run --release --example soldiercap -- depth=1 n=40,80,120,160,240 secs=40
cargo run --release --example soldiercap -- depth=1 fight=1 squads=16 n=40,100,200 secs=40
cargo run --release --example soldiercap -- depth=8 n=40,80,120,160,240 secs=40
cargo run --release --example soldiercap -- greedy=1 depth=1 seed=1 n=200 secs=360
```
