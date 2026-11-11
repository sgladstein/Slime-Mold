# The species settings on the GPU (2026-11-11)

Larder World wants several mould species in one world (the owner, 11-11
14:06Z), and the creature thread built the rules for them on the CPU big
dish: species 0's share of the walkers (`species_share`), conversion that
weighs a square round the walker (`species_convert_r`), a second kind of
walker inside one mould (`mix_*`), and each species' own settings
(`species_sets`), which let a droplet species and a network species share
a dish. This report puts all four on the GPU engine (`gpu::Engine`, so the
sandbox and the Larder world both get them) and checks them against the
CPU. Agreed with the creature thread (a colleague's plan, not an owner
ruling); the tiers' life cycle (spores, dormancy, alarm, the strike) is the
next port.

Every one is off by default, so no dish changes. The gate: one GPU tick
matches one CPU tick on dishes that use them, planted bugs fail that test,
and the cost is quoted.

```
MOULDGPU_TEST_GPU=1 cargo test --release --locked --bin mouldgpu species_settings -- --nocapture
cargo run --release --bin mouldgpu -- species=2 s0.crowd=1 s0.graze=10 s1.crowd=0 s1.explore=60 explore_spread=1
```

## The controls

All are `set` keys of the sandbox and the engine (`gpu::set`), with the
CPU's names too:

- `share` (`species_share`): species 0's share of the walkers. Fixed for
  the dish's life, since it places the walkers; the GPU starts each walker
  as the CPU dish it was built from.
- `convert_r` (`species_convert_r`): conversion compares each species'
  trail summed over the (2r+1)-square round the walker. Live.
- `mix_share`, `mix_reach`, `mix_angle`, `mix_turn`, `mix_step`,
  `mix_deposit`: the second kind. The share is fixed for the dish's life
  (as the slots are); the multipliers are live.
- `sN.FIELD` for species N (0-3): that species' own `sensor_angle`, `turn`,
  `reach`, `step`, `laid`, `keep`, `crowd`, `blocked_lay`, `graze` or
  `explore`, in the global controls' units (degrees, `laid` per 100,000
  walkers). A field left unset is the global one, which still changes it
  live. Only with two species or more; with none set, the species use the
  globals as before.

## How it runs

- A walker's kind is bit 8 of its species word (`spec`), so no new buffer
  and no new binding. Conversion keeps the bit; a child copies the whole
  word, so it is its parent's species and kind.
- The deposit counts gain a block a species and kind. The second kind's
  walkers lay `mix_deposit` times as much, as the CPU's `lay` does, in every
  class (moving, sick, explorer, blocked, capped).
- Each species' set is in the uniform (`Params::sets`, four of them, 48
  bytes each; `PARAMS_BYTES` 432 to 672, the squad's words now at a fixed
  offset 416; 688 once the world look's hits row, #69, came in after the
  squad's). With sets on, the global explorer stage, blocked share and
  grazing weight are only switches (on if any species has one), and each
  walker reads its own species' values: reach, angles, step and scent reach
  in `sense`; crowd limit in `sense`, `walk`, `breed`, `born`, `jump` and
  `land`; deposit and blocked share in `laid`; keep in `trail`; grazing
  weight when it senses; explorer stage in `explores`. Only a grazing
  species eats grass, unless growth feeds on the grass (no crystals), when
  every walker eats, as on the CPU.
- Conversion's square: a new pass (`rows`, before `convert`) sums each
  species' trail after this tick's deposit along each cell's row, into the
  species fields' spare side, which the trail pass overwrites after; the
  convert pass sums the column. The CPU builds summed-area tables in f64;
  here they are f32 sums of at most 17 and 17 values.
- Respawn with crowd limits that differ by species: a walker of a species
  with no limit claims the cell it jumps to too, and every jump lands in
  `land`, so a limited walker later in the CPU's walker order loses the cell
  to an unlimited one, as on the CPU. (A first version moved unlimited
  jumpers in the `jump` pass; then a limited jumper could read a cell before
  or after an unlimited one left it, which depends on timing. The planted
  bug for the claim found it.)
- Not ported: `blocked_retry` (retry rounds for blocked walkers), which has
  never been on the GPU.

## The twin test

`the_species_settings_match_the_cpu_for_one_tick`, in CI on software
Vulkan: each dish is warmed 40 ticks on the CPU (without conversion: with
it, species 0 at 70% took every walker in the warm-up), then one GPU tick
from the CPU's state against one `BigDish::tick`. Compared: every walker's
place and heading, the trail, scent, species fields and grass, every age,
species and kind, and the species and kinds the GPU starts with (the
inject writes them again, so that is checked apart).

| Dish | Walkers | Off | Trail L1 | Species L1 | Joined another species | Second kind |
|---|---|---|---|---|---|---|
| species 0 at 70%, converting | 250,000 | 0 | 0 | 0 | 9,774 | 0 |
| converting over a square of 3, bands | 250,000 | 0 | 1.1e-13 | 1.3e-13 | 518 | 0 |
| hunting droplets converting over a square of 8 (the dead-end's dish) | 25,000 | 0 | 0 | 0 | 163 | 0 |
| three cyclic species converting over a square of 2, cap | 250,000 | 0 | 1.9e-8 | 2.0e-8 | 27,831 | 0 |
| mixed walkers | 250,000 | 0 | 0 | 0 | 0 | 75,581 |
| mixed walkers, one a cell, blocked lay and keep | 25,000 | 0 | 0 | 0 | 0 | 7,728 |
| mixed walkers, two species, cap, explorers, contagion | 250,000 | 0 | 1.2e-8 | 1.3e-8 | 0 | 75,581 |
| a droplet species and a network species | 25,000 | 0 | 5.7e-11 | 5.8e-11 | 0 | 0 |
| own reach, step, deposit and keep, scent at the own reach, respawn | 250,000 | 0 | 1.3e-8 | 1.3e-8 | 0 | 0 |
| crowd limits by species, respawn | 25,000 | 0 | 2.2e-12 | 2.7e-12 | 0 | 0 |
| crowd limits by species, heavy respawn | 25,000 | 44 | 0 | 0 | 0 | 0 |
| own settings, mixed, converting over a square of 4 | 25,000 | 0 | 0 | 0 | 31 | 7,728 |

Every dish: 0 ages, species and kinds off, scent L1 0, grass L1 0 ([`raw/twin.txt`](species-gpu-2026-11-11/raw/twin.txt)).

"0" for the fields is to the bit; the small differences are sums in a
different order (the cap, the square). The bounds: a thousandth of the
walkers off and an L1 of 1e-4, with every kind exact.

**Heavy respawn.** With a third of the walkers jumping a tick, 44 of 25,000
walkers end off. That is not this port: the GPU reads every jump's cell as
the jump pass began, where the CPU, jumping walkers in order, sees a cell an
earlier jumper left as free. One crowd limit for every walker has the same
gap; at the sandbox's respawn of 0.01 it is a walker or two. So that dish
checks the jumps where the gap cannot arise: of 7,484 jumpers, the 5,153
that land in a cell no jumper left all agree with the CPU, and 35 of them
are limited walkers that aimed at a cell an unlimited walker took first,
which is the claim rule at work.

**Growth.** A dish was added to the growth twin
(`growth_matches_the_cpu_for_one_tick`): hungry, two species, mixed
walkers, species 1 without a crowd limit and not grazing. Every slot's
place, life, energy and kind agrees, so a child keeps its parent's kind and
a parent breeds under its species' limit.

**The square's rows.** `the_squares_rows_sum_every_cell_of_the_row` runs
the `rows` pass alone on a CPU dish's trail (radius 8, two pieces of 256
cells a row) and checks every cell's row sum against the CPU's in f64: 0
of 655,360 sums off by more than 1e-5, the worst 3.9e-7. The twin test
sees a piece's edge only where a walker converts beside one, so it let a
tile loaded without its right margin through (round 3 below).

**Planted bugs.** 28 bugs planted one at a time
([`bugs.py`](species-gpu-2026-11-11/bugs.py),
[`bugs.sh`](species-gpu-2026-11-11/bugs.sh)), each run against the twin
tests: the second kind's multipliers dropped, its scent reach scaled, its
full deposit, its counts in the first kind's block, conversion or a birth
dropping the kind; each species taking the global crowd limit, keep,
deposit, grazing weight, explorer stage, blocked share or scent reach; a
species' scent reach from the global reach; every species eating grass, or
growth on grass feeding only the grazers; the square's rows or columns a
cell short, the rows' tile a cell off or loaded without its right margin,
conversion by the cell, the rows pass not run, its rows read before the
tick's deposit; a step, a birth by the global crowd limit; an unlimited
jumper that claims nothing, or lands only by the claim; the GPU starting
every walker as species k % n of the first kind.

| Round | What changed | Caught |
|---|---|---|
| 1 ([raw](species-gpu-2026-11-11/raw/bugs-round1.txt)) | first test | 22 of 25 |
| 2 ([raw](species-gpu-2026-11-11/raw/bugs-round2.txt)) | explorer stage 60 with a spread, a bug that never planted fixed, the heavy-respawn dish | 26 of 26 |
| 3 ([raw](species-gpu-2026-11-11/raw/bugs-round3.txt)) | the rows pass by workgroup tiles, two bugs for its tile | 27 of 28 |
| 4 ([raw](species-gpu-2026-11-11/raw/bugs-round4.txt)) | the row sums checked directly; the four rows bugs again, and the wall fix undone | 5 of 5 |

Round 1 missed the explorer stage (every network walker had finished a
20-step stage by the twin tick), a bug mistyped so it never planted, and
the jump claim (too rare at respawn 0.01). Round 3 missed the tile without
its right margin, which only the cells within 8 of every 256th column read.

## A wall fix found on the way

Reading `evict` for the port: a wall built on the GPU moved every slot
under it, the empty ones too (growth). An empty slot keeps the place its
walker died at and no count, so the GPU took 1 from a count that did not
hold it (which can wrap a cell's count to 2^32 - 1 and close it for good)
and added a walker nobody was to the cell it found; the CPU's
`build_wall` skips them. The wall twin test had no growth. It gains a dish
with growth on (hungry, no crystals, warmed 150 ticks so walkers starve
under the wall): before the fix, 195 empty slots under the wall moved, 349
cells' counts were off and 154 cells held two walkers under a limit of
one; after it, 0, 0 and 0. The Larder world builds walls with growth on, so
this reached it.

## Cost

On the GTX 1070 (`cost.sh` run there by the PC's session, the GPU
otherwise idle; base 6b907d8, main with W2b, against this branch at
68e8bff; [`raw/cost-1070.txt`](species-gpu-2026-11-11/raw/cost-1070.txt)),
the sandbox's default dish (100,000 walkers), a frame of two ticks,
headless, 5 s a run, three rounds alternating. GPU ms of ticks a frame,
the three rounds' range:

| Pair | Without | With |
|---|---|---|
| plain dish, main against this branch | 1.47-1.53 | 1.47-1.48 |
| two species, main against this branch | 2.08-2.09 | 2.13-2.14 |
| two species, without and with own sets (equal to the globals) | 2.13-2.15 | 2.14 |
| the outside rules forced on, the second kind off against on with multipliers of 1 | 1.47-1.48 | 1.59-1.60 |
| the same, against the second kind with its multipliers | 1.47-1.48 | 1.67 |
| two species converting, conversion by the cell against the square of radius 8 | 2.34-2.35 | 3.64-3.68 |

- Off, the port costs nothing (plain dish). Two species pay 0.05 ms more
  a frame (2%, higher on 3 of 3 rounds): the per-species lookups on the
  outside path. Own sets cost nothing on top.
- The second kind costs 0.12 ms with multipliers of 1 (its second block of
  counts, which the trail pass reads) and 0.19 ms with them, which also
  changes the dish. Forcing the outside rules on costs nothing measurable
  here (1.47-1.48 against the mix-off dish's 1.47-1.48).
- The square of radius 8 adds 1.3 ms a frame. Each tick alone, the rows
  pass takes 0.36 ms and the convert pass's column sums 0.45 against 0.13
  by the cell; the trail pass is 0.32 either way. That is about one and a
  half trail passes a tick, a third of a 2.3 ms frame, so a world that uses
  the square should keep it small or take the change below.

On the software GPU (Mesa's llvmpipe, a CPU, so these are not GPU
numbers), the same runs earlier
([`cost.sh`](species-gpu-2026-11-11/cost.sh)). GPU ms of ticks a frame:

| Pair | Without | With |
|---|---|---|
| plain dish, main (296ea40) against this branch | 27.3-27.4 | 26.2-26.8 |
| two species, main against this branch | 56.9-64.2 | 58.1-64.2 |
| two species, without and with own sets (equal to the globals) | 57.3-63.1 | 59.3-62.8 |
| the outside rules forced on, the second kind off against on with multipliers of 1 | 35.0-41.2 | 35.8-38.5 |
| the same, against the second kind with its multipliers | 35.0-41.2 | 37.5-43.7 |
| two species converting, conversion by the cell against the square of radius 8 | 59.4-60.8 | 113.8-114.8 |

So the port costs nothing measurable where it is off, and own sets and the
second kind cost nothing measurable on top of the outside rules. The
second kind on the default dish does switch the outside rules on (they are
compiled out when off): 25.8-26.8 to 35.7-40.3 ms
([`raw/cost.txt`](species-gpu-2026-11-11/raw/cost.txt), *mix*), the outside
rules' known price, not the mix's
([`raw/cost-mix.txt`](species-gpu-2026-11-11/raw/cost-mix.txt)).

The square is dear: radius 8 adds 54 ms a frame here, about one trail pass a
tick. Each tick submitted alone, the rows pass takes 24-25 ms against the
trail pass's 18, and the convert pass's column sums 4.5-5.0 against 1.4-1.9
([`raw/cost-square.txt`](species-gpu-2026-11-11/raw/cost-square.txt)). The
rows pass reads each cell's trail after this tick's deposit, which means
the deposit counts, as the trail pass does. A first version summed the 17
cells of each row from the fields one thread a cell and added 74-79 ms
([`raw/cost.txt`](species-gpu-2026-11-11/raw/cost.txt), *square*); the pass
now loads a row's 256 cells and their margins into workgroup memory once.
The convert pass sums the columns only at walkers' cells. The 1070 puts
the square at 1.3 ms a frame (above); if a world needs it, the next step
is to write the deposited trail once (rows) and let the trail pass read
it, instead of both working it out.

## Files

- `src/gpu/engine.wgsl`: the `Set` struct and species block, `per`,
  `mixes`, `kind_of`, `crowd_of` and the other per-species lookups, `eats`,
  `laid_kind`, `weigh` and the `rows` pass;
  `evict`: the wall fix.
- `src/gpu/mod.rs`: `Settings` fields and `sN.FIELD` keys, `SpeciesKnobs`,
  `species_sets`, `spec_words`, the species block in `write_params`, the
  `rows` pass in `tick_passes`, `Engine::run` (passes on their own, for a
  test of one pass).
- `src/bin/mouldgpu.rs`: the twin test, the row sums' test, the growth
  dish, the wall test's growth dish, the headless `species:` line.
- [`species-gpu-2026-11-11/`](species-gpu-2026-11-11/): `bugs.py`, `bugs.sh`,
  `cost.sh`, `raw/` (bugs by round, cost, `twin.txt` the tests' output).
