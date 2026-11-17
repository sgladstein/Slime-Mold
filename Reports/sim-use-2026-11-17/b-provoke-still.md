# B1 Provoke and B2 Still (2026-11-17)

The simulation-use plan's B1 and B2 ([plan.md](plan.md), draft 3, Part B:
"its signals"). The owner ruled at 01:13Z that these tools are both carried
devices and gun cards, the gun thread making the cards. Page: https://claude.ai/artifact/6PUddP5ThBHR6AQnEZmwip
(source `page-b/`).

## What was built

Two new Mind statuses, on the same word and passes as Blind, Sour and Spin
(`BigDish::mind`, the engine's `mindlay`), so any `squad::Burst` with
`mind` 4 or 5 lays them on either engine.

- **Provoke** (`big::MIND_DEAF` = 4). A deaf walker hears no food news while
  the status lasts: not its own eating, not the news spread from its
  neighbours (`hear_food`; the shader's `ngather` and `nhear`). It still
  passes on what it last heard. Its news ages, so once it is older than the
  wither lag (5 s for shape C) the walker believes it is cut off: it turns the
  cut-off grey, panics (charging the nearest unit within `panic_r`, or else
  running for a crystal), may wither, and burns (C1's `fire_dry`). Nothing new
  draws it; the grey is the news the view already reads.
- **Still** (`big::MIND_STILL` = 5). A still walker neither panics, surges,
  raids nor breaks away (`panic_haste` reads it on the CPU; `panic_to` on the
  GPU returns calm before any of them), and takes no crystal: its share of the
  cell is spilled (the crystal loses it, as for any walker on the cell, so the
  two engines stay exact) and it does not count as having eaten. It keeps
  walking the network and paying its upkeep. Unlike sleep gas it moves and
  wakes for nothing. A lump's member keeps its leash. "Calm" in the plan's
  first draft was renamed Still, as `calm` is a pass.
- **The devices** (`Tool::Provoke`, `Tool::Still`; `KitRules::mind_r` 12 dish
  cells, Provoke 12 s, Still 10 s, 100 food each; first guesses, untuned). On
  the window's tools page (E): Provoke on X, Still on Z, thrown like the gas
  with the throw ghost (alarm red, cool blue). Runs leave them out
  (`run_hidden`) until the gun thread's cards. The gun thread is told the two
  kinds exist.

## Tests

| Test | Result |
|---|---|
| `provoked_walkers_hear_nothing_and_panic_and_still_ones_neither_panic_nor_eat` (CPU, small fed dish with crystals, lag 5, lean 3) | deaf: no walker's news moves in 10 steps; 1.00 panicking against 0.31 plain; 10 steps after it runs out 0.24. Still: a stale body 1.00 panicking plain, 0 still; on a fed body one step, the crystals lose exactly what the plain dish's do, energy 502.7 against 510.2 with 377 plain walkers eating; 5 steps on no still walker has news newer than the stamp |
| `the_world_matches_the_cpu_for_one_tick`, new dish "mind: deaf and still walkers" (llvmpipe; every fifth walker deaf, still, still, deaf run out, none; the panic dish with the surge on) | 0 walkers off, energy L1 0, 0 news off, 0 Mind words off. Without the statuses 407 deaf walkers' news, 415 still walkers' energy and 2,066 of their moves differ, so a GPU ignoring either fails. Planted: still's spill removed from the shader, energy L1 3.6e-4 (fails); deaf ignored in `nhear`, news off (fails) |
| `provoke_and_still_stamp_their_status_for_their_seconds` (kit, CPU race world) | each stamps its kind on every walker in `mind_r` to the right step, is paid, kills nothing |
| `the_window_uses_the_tools` (window, llvmpipe) | the tools page lists Provoke and Still; Provoke used beside a soldier stamps a status on the GPU |

## Measure

`examples/mindprobe.rs`, raw output in `b-raw/`. Shape C at its 25,000
walkers on the CPU race world (scale 2), grown 63 s with the surges held.
`panic_r` 29 dish cells (the game's). A unit (as a standing soldier is to the
slime; it does not shoot) stands on open ground 8 dish cells clear of the
slime with the most slime within 20 dish cells, crystals 20 or more away;
the disc of 12 dish cells is centred on that slime. **Deciding measures,
named before the run:** walkers that come within 3 dish cells of the unit in
15 s, each counted once, against the same state without the status.

| Seed | Disc | Provoke: plain | Provoke: provoked (from the disc) | Disc dead in 15 s | Surge: plain (from the disc) | Surge: disc still (from the disc) |
|---|---|---|---|---|---|---|
| 1 | 783 | 0 | **136** (124) | 88 | 149 (69) | **130** (0) |
| 2 | 577 | 0 | **183** (168) | 77 | 255 (115) | **248** (21) |
| 3 | 1,119 | 0 | **147** (147) | 143 | 226 (101) | **184** (0) |
| 4 | 1,036 | 0 | **142** (141) | 124 | 175 (100) | **147** (7) |
| 5 | 863 | 0 | **145** (145) | 112 | 253 (104) | **232** (1) |
| 6 | 244 | 0 | **106** (92) | 39 | 96 (36) | **91** (0) |
| 7 | 756 | 0 | **151** (151) | 83 | 261 (123) | **257** (6) |
| 8 | 1,134 | 0 | **137** (135) | 161 | 177 (86) | **144** (2) |
| mean | | 0 | 143 | 103 | 199 | 179 |

(The still disc is centred on the walker nearest the unit, so it is not the
provoke disc; its own walker counts were 741 to 980.)

- **Provoke** (deciding): the provoked part reached the unit on 8 of 8
  seeds where the plain body never did (sign p 0.008). On seed 1 it went grey at 5 s and
  charged after the 1.5 s lean, and 4-16% of the disc died on the way (the
  wither, 0 plain).
- **Still** (deciding): fewer walkers reached the unit in a surge on 8 of 8
  (p 0.008), but only 10% fewer (199 to 179): the stilled disc sent 0-21
  walkers against 36-123, and the slime behind it, outside the disc but in
  the surge's reach, came round it.
- Both survive a Bonferroni correction over the three tests (0.05 / 3 =
  0.017); the third (hold, below) is a tie on every seed.
- A first try put the unit inside the slime (beyond the disc's edge toward
  the dish's middle, which the race world's body covers): the provoked part
  panicked (1,818 of 1,818 by 5 s) but could not push through the body
  between, and the unit drew the same walkers either way (104 against 106).
  A charge needs open ground to the soldier.

### Hold: does a held part starve?

Raw lines in `b-raw/mindprobe-hold-*.txt` (`only=hold`). The surges held,
a disc stamped still every 10 s for 30 s; living walkers counted (the
disc's by slot; the race world's crystals are fed on throughout).

- **On the spot, clear of crystals** (the first run, `b-raw/mindprobe-seeds*.txt`):
  no change at all on 8 of 8 seeds. No walker there eats, panics or surges,
  so a still walker ticks exactly as a plain one.
- **On the crystal nearest the spot** (1,491 to 1,811 walkers): none of the
  disc died, still or plain, on 8 of 8 seeds, and the slime ate as much (the
  still walkers' shares are spilled, so the crystal loses them anyway). A
  still part neither starves nor goes grey in 30 s: shape C's upkeep is
  0.0002 a step, so a walker lives minutes on what it holds; food flows to it
  through the body (`energy_flow`); and its news is kept fresh by its
  neighbours and by its own children, who are born without the status and
  eat.
- **The whole body still**: it grew less on 8 of 8 seeds (p 0.008), 32,712
  living at the end against 37,279 (12% fewer); 118 to 296 walkers went stale
  (0 or 1 plain).

So Still's cost, as built, barely shows in a fight's length: it stops the
charge and the bite of what it hits, and a body held still for long grows
less. If it should starve visibly, the levers are an upkeep while still, or
the status passing to a still walker's children; neither is built.

## Frame cost

Off (no status laid): nothing; `mind_of` returns at once while the Mind row
is off. With statuses on the GPU: one more read of the walker's Mind word in
`panic_to` and in the life pass (the word `sense` already reads), and
`ngather`'s read moved, not added. On the CPU the same. Not timed on the 1070.

## Left

- The gun thread's cards for the two kinds (told).
- Tuning the devices (reach, length, price) once played; whether Still
  should starve visibly (above).
- 1070 timing with the rest of the plan's passes.
