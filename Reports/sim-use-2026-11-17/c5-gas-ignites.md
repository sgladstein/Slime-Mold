# C5 Gas ignites (2026-11-17)

The simulation-use plan's C5 ([plan.md](plan.md), draft 3, Part C: "Gas
above `gas_burn` lights when fire or a blast reaches it and is used up:
sleep gas then a scorch round flashes across the cloud. Check a real
cloud's values first."). Page: https://claude.ai/artifact/LV3H1RgmX6EoBbYSrPtTkX (source
`page-c5/`).

**Status: built on both engines as `BigParams::gas_burn`, on at 0.2.**
Sleep gas and then any fire (a scorch round, a blast, a Torch, a fire
already burning nearby) now burns what sleeps in the cloud, damp body or
not. Without it a fire lit on the gassed body killed nothing.

## What was built

- `BigParams::gas_burn` (key `gas_burn=`; 0 off; default 0.2, the kit's
  `gas_sleep`, so the cloud burns where it puts walkers to sleep). Gas cloud
  over it is fuel:
  - **A light** (`ignite`: a burn, a blast, a Torch's light) lights every
    such cell in its reach, whatever the plants, the share lit or wet
    ground.
  - **Each fire step** a cell over it that burned the step before flashes,
    and one beside such cells catches by chance, half for each of its four
    neighbours that burned. A flash is fire 1, which kills what stands
    there (the fire kill, damp body or not), and uses the cell's cloud up.
    With no plants under it the flash is an ember the next step, so the
    fire runs across the cloud and goes out behind it.
- The first try caught every cell beside fire, which drew a sharp diamond
  across the cloud (one cell a step along each axis). The half chance per
  burning neighbour gives a round, ragged front instead.
- Both engines read only the step before's fire, so the order of cells
  does not matter. The CPU does it in `fire_tick` after the swap; the GPU
  does it in the `fire` pass after the char, on this tick's cloud
  (`germ_out`'s third block), and in `ignite` on the cloud between ticks
  (`germ_in`'s). The value rides in plant header word 43 and the dice are
  `GAS_DICE` on the fire step.
- **A real cloud** (the kit's sleep gas: `gas_r` 12, dose 1, half-life
  4 s; `examples/gasprobe.rs`): 1 s after it is laid on the body, it sums
  about 1,500 over about 2,240 fine cells over 0.2, and it has thinned
  below 0.2 everywhere 15 s on. So 0.2 burns the cloud while it still
  holds sleepers.

## Tests

| Test | Result |
|---|---|
| `gas_ignites_on_the_gpu_as_on_the_cpu` (flats map planted whole, the race world grown 12 s, the kit's sleep gas on the home, a fire of 2 cells lit at the cloud's edge 1 s later) | CPU: 355 killed by fire in 3 s against 0 with `gas_burn` 0; the cloud 371 at the light, 105 three seconds on against 220. Twin (llvmpipe): largest fire gap 0, cloud L1 0, living equal each second. Planted, a GPU whose gas never burns: fire gap 1, cloud L1 137, living off by 355 (fails) |
| `cargo test --release --locked` with 0.2 on by default | passes (591 lib, 41 window, 49 mouldgpu); clippy clean |

## Measure

`examples/gasprobe.rs`, raw lines in `c5-raw/`. Planet depth 2 under full
meadow, the default slime (shape C, `fire_dry` on) on the CPU race world
(scale 2), grown 60 s with the surges held. On a disc of the body chosen as
`dryprobe`'s, the kit's sleep gas; 1 s later a fire of 2 dish cells lit at
the cloud's east edge; watched 15 s. **Deciding measure, named before the
run:** walkers killed by fire in 15 s, `gas_burn` 0.2 against 0, paired by
seed.

| Seed | Asleep at the light | Killed, off | Killed, 0.2 | Cloud at the light (sum, cells over 0.2) | Cloud after 15 s, off / 0.2 |
|---|---|---|---|---|---|
| 1 | 1,969 | 0 | **1,271** | 1,509, 2,231 | 113 / 57 |
| 2 | 1,605 | 0 | **967** | 1,277, 1,855 | 95 / 45 |
| 3 | 1,985 | 0 | **1,181** | 1,508, 2,241 | 113 / 59 |
| 4 | 2,075 | 0 | **1,196** | 1,508, 2,244 | 112 / 59 |
| 5 | 1,928 | 0 | **1,224** | 1,512, 2,295 | 114 / 57 |
| 6 | 2,007 | 0 | **1,215** | 1,510, 2,247 | 113 / 58 |
| 7 | 22 | 0 | 0 | 922, 1,303 | 69 / 69 |
| 8 | 733 | 6 | **482** | 1,111, 1,635 | 82 / 40 |
| sum | | 6 | 7,536 | | |

- **Deciding:** more walkers burned on 7 of 8 seeds, none fewer, 1 tie
  (sign p 0.016). Off, the fire lit at the cloud's edge on the fed body
  went out at once (0 killed on 7 seeds). On, it flashed across the cloud
  in about 3 s and burned 58-66% of the sleepers.
- **Seed 7** lit nothing. Its disc held 22 walkers, as C4's seed 7 did,
  and the cloud by the light stayed whole.
- **The slime regrows the loss in seconds.** The living 15 s on were within
  0.1% either way, because shape C sits at its capacity and the freed slots
  are born again. The flash cuts a hole, not the body's size; what it buys
  is the ground and the time while the hole refills.
- **What it looks like** (the page, seed 1): the violet cloud over the
  body, a ragged ring of fire running from the light across it, a burnt
  hole behind it ringed by the grey sleepers at the cloud's thin rim that
  did not burn.

## Frame cost

Off (`gas_burn` 0, or no cloud laid: the GPU's `gas_keep` is 0 until a
cloud is laid): nothing beyond one test. On, a cell over `gas_burn` reads
its four neighbours' fire on a fire step, and `ignite` reads the cloud
once a cell. Not timed on the 1070.

## Left

- The combo's price in play: sleep gas and a scorch round now kill a whole
  cloud's sleepers. The gun thread's cards (Mist, Scorch, Torch) are told.
- 1070 timing with the rest of the plan's passes.
