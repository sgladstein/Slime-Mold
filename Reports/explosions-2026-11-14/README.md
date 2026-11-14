# Bomb rounds: explosions that throw the mould (2026-11-14)

The owner, 2026-11-14 01:59Z: "Do we have explosions yet. That should also
be a thing", and 01:59:50Z: "as part of the flashes of chaos gun system".
This builds step 3 of [the Noita spell look](../noita-spell-look-2026-11-14.md):
a blast throws the world's own matter. Noita's splash takes the pixels round
it into a particle simulation, and they land back as cells. Here the matter is
the mould's walkers.

**Films page:** https://claude.ai/artifact/Q177REGwpKfP1vyKeZtmRr (source in
[`page/`](page/index.html); made by [`raw/run.sh`](raw/run.sh) and
[`raw/clip.py`](raw/clip.py); t lines in `raw/`).

## What was built

- **`Card::Bomb`**, a payload of the rounds' own (`src/mould/rounds.rs`).
  It kills everything within `bomb_r` 4 dish cells, lays a crater of
  `bomb_crater_r` 4 for `bomb_crater_secs` 6 s, and lays alarm to 8 cells as a
  blast does. Then it throws every living walker on the ground out to
  `bomb_ring` 2.5 times the kill (10 cells). It costs `bomb_price` 60, a
  blast's price. Loadouts `bomb`, `cluster` (scatter+bomb) and `blowout`
  (wide+bomb) are in `rounds=` and the H cycle.
- **The throw** (`squad::Burst::fling`, `fling_steps`; `BigDish::fling` on the
  CPU, the `fling` pass in `engine.wgsl` and `Engine::fling` on the GPU).
  A thrown walker becomes a spore in flight, marked flung (`BigDish::flung`,
  bit 13 `FLUNG` of the engine's `spec` word). It heads atan2(dy, dx) out
  from the centre. Its timer is max(1, floor(steps x (R² - d²) / (R² - r²))),
  so the nearest fly furthest. It flies straight at `fling_speed` 1.5 x
  `step`, never homes, and lays `fling_deposit` 3 x `deposit` as it flies.
  It lands where its timer runs out, on a free cell, with no rest. A wall
  stops it and turns it as a spore. At the race world's 30 fine steps a
  second, `bomb_secs` 0.5 gives 15 fine steps: the inner edge flies about
  22 fine cells (11 dish cells).
- **Seen in flight.** A flung walker's fourth word is 1 while it flies, and
  the view draws it as a hot speck a pixel wider each side per 4 of zoom
  (`draw_specks` in `world/view.wgsl`). The streak is the trail field, the
  same data the slime is drawn from.
- **Words used:** Params word 233 (`fling_st`, was `_p3`) and 231
  (`fling_dep`, was `_p1`); gas header word 3 (walkers flung, a read-back
  per throw); pass `fling` (index 52). A flung walker counts 65536 in the
  spores' deposit block, so spores and flung lay their own rates from one
  counter.
- **`larder-world boom=T,X,Y`** sets the squad's round off there at T, free
  (`Rounds::cast_at`), and **`bank=F`** sets the larder's starting food:
  a staged scene for films. The headless `rounds t` line now counts the
  walkers thrown.

## Checks

- **Engine twin** (`mouldgpu`, `an_explosion_flings_the_same_walkers_as_the_cpu`):
  on four dishes (plain, no crowd limit, dormancy and alarm, growth on
  crystals with a wall across the ring), the same walkers are thrown on
  both (202, 1,530, 205 and 175), with every state, timer, mark, count and
  heading the same. Then each tick of the flight from the CPU's state: 0
  walkers off at 0.001, trail L1 within 1e-6, and by the end at most 1
  still in flight on either. A GPU speed of 2 against the CPU's 1.5 parts
  all 202 walkers (the planted bug).
- **Game twin** (`bomb_rounds_fling_the_same_mould_on_the_engine_as_on_the_cpu`):
  the squad attacks with bomb rounds on the CPU world for 30 s. At each of
  the 22 casts, the engine is loaded with the dish as it stood before and
  the same bursts go through `World::burst` on the GPU world. They throw
  the same walkers, and every slot's state, timer and mark matches. Two
  frames on, at most 1 mark is off.
- **CPU** (`a_bomb_flings_its_rim_and_the_flung_live_on`, seed 3, 3,000
  walkers): a bomb on the mould's middle kills 175 and throws 954. 940 have
  landed as walkers 0.9 s later, 909 further out than they started and 870
  beyond the ring. The living count is 3,000 then 2,825, exactly the kills.
- **Frame cost:** the throw runs once a burst, not once a step: one pass
  over the slots and one 4-byte read-back. In flight a flung walker costs
  what a spore costs in `fly`. The view adds one branch a walker. The
  1070 twin and timing wait for the PC (C6 batch).

## Smoke checks

The window headless, seed 1, Fork, shape C, to 90 s, one run each
(`raw/smoke-HAND.txt` for bomb rounds, `raw/smoke-HAND-plain.txt` and
`-blast.txt` for the others). Nothing soft-locked. Idle doesn't win.

| hand | plain shots | blast rounds | bomb rounds |
|---|---|---|---|
| rush | squad wiped at 80 s | wiped at 36 s | wiped at 21 s |
| idle | squad alive at 90 s, mould 24,055 | wiped at 62 s | wiped at 59 s |
| farm | 17 soldiers at 90 s, mould 16,496 | 4 soldiers, mould 18,473 | 4 soldiers, mould 18,134 |

A tuning note, not a gate (build first, balance later): every round,
blast too, ends these three hands sooner than plain shots on this seed. The
rush with bombs is the one under a minute. The rush walks into the body
and fires at close range, so a bomb's ring reaches the squad and throws
mould onto it. Things to try in a balance pass: a smaller `bomb_ring`, or
no throw toward the shooter. One seed, so nothing here is a measured
difference.

## In the films

On one shot into C's body at 0:20 (zoom 6): blast kills 334 and leaves a
round hole that stays. Bomb kills 171 and throws 678: a bright ring bursts
out, sparks fly, the ring lands round the hole and the body fills back in
within about a second. Cluster makes three small holes and rings (161
killed, 716 thrown). Blowout makes one big hole and a thick wave (375
killed, 1,503 thrown). In play, eight soldiers on free fire at the body's
thin edge, 0:00-0:27: blast rounds kill 225 in 37 casts and bomb rounds 111
in 46, throwing 609. The mould is 24,995 against 25,144.

So a bomb is a weaker killer than a blast, and it moves mould rather than
removing it. That is the trade the Noita report planned: it cuts the body
and spreads it. No balance is claimed (build first, balance later).

## Not done

- The impact light: the gun rounds thread lights each pop by payload
  (`card_look`, PR #219). The bomb's arm goes in once #219 is on main.
- 1070 twin and timing: pending, the PC is busy.
