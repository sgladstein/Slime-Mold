# A fed slime thickens (2026-11-15)

The owner, 11-15 08:34Z, on whether a slime left alone should keep getting
stronger through a level: "keep getting stronger by holding onto its
crystals and getting more of them (which increases how fast it
multiplies)." Page: https://claude.ai/artifact/96rvRmzz2kXB7NKe8C3U1d (source in `page/`,
open `index.html`).

## Before

Shape C on the CPU race world (`examples/foodprobe.rs`, seed 1,
`raw/foodprobe-plateau.txt`) eats 2.4-3x its upkeep from 22-23 crystals and
grows about 3,500-7,500 walkers every 30 s, limited by room at its edge and
by its slots (one child into slot k ^ m, a free agar cell beside it). It
fills its 50,000 slots at 3:00 (49,994). From then on its surplus only
piles up as energy (mean 1.58 at 3:00 and rising, against `birth_energy`
1), and it never gets stronger.

## The rule

`BigParams::thicken` (0 off; both engines): an ordinary walker whose
energy after its upkeep reaches `thicken` x `birth_energy` turns condensed
where it stands (`DENSE`) and pays `birth_cost`. It reuses condensing's
state and everything that already reads it: two lives (a shot, blast,
withering or starving breaks it back first), twice the bite and the hold
on a soldier's legs, twice the upkeep, two walkers' worth in `mass`, and
the bronze look. Unlike condensing, nothing is absorbed and births go on
(the slots-full stop is condensing's alone), so the mass grows past the
slots, up to twice. Above 1, a walker divides first whenever it can, so
only food the mould cannot turn into walkers becomes thickness.

**On in shape C at 2** (`shape_keys`, so the window's sandbox and the run
once it plays C's keys, the run thread's #336); the start menu's Thicken
option (0, 1.5, 2, 3).

GPU: `thicken` in tenths (0-255, `gpu::thick_tenths`) in `ccrowd`'s top
byte (bits 24-31; the condense radius keeps 16-23, now masked);
`condenses()` is the dense state for condensing or thickening, and
`ripens()` (condensing alone) gates ripeness, absorbing, the slots-full
birth stop and breakaway. No new Params words, no new passes; the
existing `ripen`, `masscount` and `dcount` passes run when either is on.
Twin: `the_world_matches_the_cpu_for_one_tick`'s dish "thicken" (a third
of the walkers set to 2.5 x a birth's energy): 863 thickened in the tick,
0 dense words off, energy L1 1.1e-7.

## Measured

CPU race world, scale 2, 25,000 walkers, C's keys plus `thicken` 2
(`examples/thickenprobe.rs seeds=1-2 scale=2 grow=420 keep=0,4
keys=thicken:2`, `raw/probe-on.txt`). Deciding measure, named before the
run: mass at 7:00 against the slots, and mass with every crystal against
4 at every line.

| Mass | 1:00 | 2:00 | 3:00 | 4:00 | 5:00 | 6:00 | 7:00 |
|---|---|---|---|---|---|---|---|
| every crystal, seed 1 | 32,860 | 45,181 | 61,204 | 71,592 | 80,983 | 86,984 | 90,739 |
| every crystal, seed 2 | 33,490 | 49,520 | 62,094 | 70,743 | 78,809 | 85,500 | 90,226 |
| 4 crystals, seed 1 | 27,595 | 21,340 | 22,768 | 24,674 | 26,897 | 28,749 | 30,338 |
| 4 crystals, seed 2 | 27,196 | 24,186 | 25,683 | 27,417 | 29,473 | 31,297 | 32,577 |

Past the slots on both seeds with every crystal (90,739 and 90,226 of
50,000), and higher with every crystal than with 4 at every line on both
seeds (14 of 14). With 4 crystals, walkers boxed in on the crystals
thicken too (7,119 and 5,460 at 7:00), as they can't divide.

Window (`larder-world look=DIR mode=squads hand=squads seed=1
[thicken=2]`, software Vulkan, `raw/window-on.txt`, `raw/window-off.txt`):
mass 64,154 at 6:00 against 49,950 (14,853 thick), soldiers 2 against 3.
One seed and a scripted hand, so not a balance reading.

## Not done

- One level: a body that holds everything long enough stops at twice
  its slots (mean energy still climbs past 3.6 at 7:00). More levels or a
  lower threshold deeper in the run are the next lever.
- It attacks nothing by itself; surges are next.
- 1070 frame cost: no new pass or word; `ripen`, `masscount` and `dcount`
  now run with thickening on, as they do for condensing (not timed).
