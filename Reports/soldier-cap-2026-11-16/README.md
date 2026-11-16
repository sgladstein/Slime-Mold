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

## Not measured yet

- **The GTX 1070.** The GPU's units pass runs a workgroup a unit (parallel),
  but its smell and harvest passes walk the units in order in one workgroup
  (for the CPU's order, `engine.wgsl`), so they grow with the army. The
  frame's upload and read-back are now the bigger buffers' size every step
  (about 48 KB up and 28 KB back, from about 14 and 8). Booked: `bench` at
  40, 100 and 200 soldiers (`set=max_squad=N`, a new rule key, with
  `hand=rush` buying to it).
- **Balance.** The slime, the shop and the AI were tuned at 40. The player
  AI (`larder_world_ai`) buys to the cap whenever its bank holds `spend`
  soldiers' worth, so at 200 a rich bank goes on soldiers before turrets
  (its test `a_rich_hand_spends_its_bank` now pins the old cap of 40). Build first,
  balance later (owner, 11-12): the cap is in for the owner to play.
- **Reading 200 soldiers on screen.** The owner's playtest.

## Reproduce

```
cargo run --release --example soldiercap -- depth=1 n=40,80,120,160,240 secs=40
cargo run --release --example soldiercap -- depth=1 fight=1 squads=16 n=40,100,200 secs=40
cargo run --release --example soldiercap -- depth=8 n=40,80,120,160,240 secs=40
```
