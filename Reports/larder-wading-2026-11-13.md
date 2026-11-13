# Wading counts every walker (`Rules::wade`), 2026-11-13

The Larder thread. Soldiers in thick slime now slow down on shape C: the slowdown counts every walker on a soldier whole, as the bite already did. Raw files, the page's source and its clips are in [larder-wading-2026-11-13/](larder-wading-2026-11-13/); its [raw README](larder-wading-2026-11-13/raw/README.md) names each file. Clips page: https://claude.ai/artifact/DKoWuDZja5GGeMTUuJFLMR.

**Why.** The owner, 15:55Z, after the sink page's clips: "I have said this before, but 1) this is really hard to judge when the soliders are just running into the slime, 2) why are they not way slowing down. I thought we implemented that, 3) Running into the thickest part of the enemy should be very deadly". Then (15:56Z): "why do you measure it instead of checking the code to see if it was implemented. I was shown a video at one point where it worked".

**What the code said.** Wading (PR 3b, #143) is on: a soldier walks at `speed / (1 + x²)`, `x = touching × mass / wade`. `Rules::mass` (#164) is 5,000 over the start walkers at scale 2, so 0.2 for shape C's 25,000. Half speed took 30 walkers on a soldier, not 6. The video the owner remembered was the 5,000-walker mould, mass 1. The bite (`Rules::drain`) stopped using mass in #174; wading kept it.

**The change.** `x = touching / wade`. Every walker counts whole at any mass; `wade` 6 stays.

| Walkers on a soldier | 2 | 6 | 12 | 20 | 26 |
|---|---|---|---|---|---|
| Speed before on C (counted at 0.2) | 100% | 96% | 86% | 69% | 57% |
| Speed now | 90% | 50% | 20% | 8% | 5% |

At mass 1 (the 5,000-walker mould, the census's default) nothing changes.

## What was wrong with the last clips

The sink page's rush clip (PR #181) showed four soldiers walking into C's body at 6-14 s and standing there to 20 s with none lost. That binary was built before #174 merged into the branch, so the bite was still 1 × mass, 0.2 health a second a walker. On today's main the same hand (`hand=rush`, Fork, seed 1, llvmpipe) loses all four by 10 s with the old wading, with 16 to 28 walkers on each soldier from 6 s ([raw](larder-wading-2026-11-13/raw/preview-r-a0-w30-b45.txt)). With the new wading it loses them by 12 s, slowed short of the core ([raw](larder-wading-2026-11-13/raw/preview-r-w6.txt)). So point 3 was already true on main. The owner was shown stale clips.

## The clips (shape C, the Fork, seed 1, llvmpipe)

Two new look hands, both buying the squad to eight at once with 400 food kept: `hand=push` walks the eight into the body from 3 s (`Order::Attack`, the rush's order). `hand=hold@X,Y` stands them at (X, Y) on free fire.

**Walking in** (`hand=push`, `wade=30` for the old rule on C, against the default):

| | 8 s | 10 s | 12 s | Wiped |
|---|---|---|---|---|
| Before (walkers at 0.2) | 8 soldiers, health 92, 3.5 on each | 8, 65, 12.9 | 3, 24, 19.0 | 13 s |
| Now (walkers whole) | 8, 93, 2.4 | 8, 75, 7.5 | 8, 46, 17.0 | 15 s |

- Deadly either way: both squads die within 5-7 s of the first walkers reaching them.
- Slowed: by 12 s the squad has walked 104 world units from the larder, against 128. That is why it lives 2 s longer: it never gets as deep.
- Raw: [`clip-push-before.txt`](larder-wading-2026-11-13/raw/clip-push-before.txt), [`clip-push-now.txt`](larder-wading-2026-11-13/raw/clip-push-now.txt); to the wipe, [`preview-pl-w30.txt`](larder-wading-2026-11-13/raw/preview-pl-w30.txt), [`preview-pl-w6.txt`](larder-wading-2026-11-13/raw/preview-pl-w6.txt).

**Standing at the edge** (`hand=hold@318,148`, the mouth of the right-hand channel, against `sink=0`). This answers the owner's point 1: a squad that never walks in, to judge the sink.

| 4-30 s | Shots | Killed | A shot | Health at 30 s |
|---|---|---|---|---|
| Blast on the first slime (`sink=0`) | 58 | 271 | 4.7 | 44 |
| Blast sunk one radius in (default) | 47 | 261 | 5.6 | 50 |

- One seed and one stand, a picture, not a census.
- **The squad fires rarely**: about 50 shots in 26 s from eight soldiers that could fire about 200. A soldier's aim is the densest block of walkers within its sight (`squad` frame, `UnitOut::target`, both engines), found with no wall check. `Game::target` then drops the shot when a wall or rock blocks the line (`clear_line`). The densest slime in sight was beyond the right-hand wall, so the slime in front went unshot. Fixed in #198 (*Aims that see*, below).

## Tried and dropped: a soldier's blast kept off its own touch circle

Built and measured, not merged ([patch](larder-wading-2026-11-13/raw/arming-not-merged.patch); `Rules::arming`). A soldier's bullet could not strike until it had flown past its own touch circle and one blast radius (4 dish cells). The first leg flew that much further, and the engines traced only the part past it, so the bullet was still drawn from the soldier.

- **Why tried.** From the stale clips I read that a soldier in the slime blasted its own feet clear, and its crater kept the walkers off.
- **Why dropped.** On today's main walkers sit on soldiers in the body anyway (16-28 each, with or without it). And the owner, 16:15Z: "will this make it so a solider can never kill a walker close enough to it. This could be a problem in a different situations where I solider is standing still and gets rushed by walkers".
- **What it measured.** These runs all had arming and new wading together, against neither, so its own share is not separated.
  - Walking in (`hand=push`): wiped at 14 s with it, 13 s with neither, 15 s with wading alone.
  - Four soldiers idle at the larder: dead by about 21 s with it, about 23 s with neither.
  - Eight standing at the larder on free fire: health 21 at 60 s with it, 19 with neither.
  - The CPU thick probe (`examples/thickprobe.rs`, race world, 240 s of growth, then the squad attack-moved into C's thickest part, seeds 1-4): 40 soldiers lost 18, 27, 0 and 15 with it, against 4, 14, 0 and 15 with neither; first deaths at about 2 s against 16-23 s. 10 soldiers lost 4, 3, 2 and 3 against 3, 2, 5 and 3. Raw in [`raw/thickprobe/`](larder-wading-2026-11-13/raw/thickprobe/) (`new` is arming with wading, `old` neither).
- The probe now has a `wade=` key, and no rations or overrun. A squad held at the larder for 240 s otherwise lost the round before the order on seeds 1, 2 and 4.

## Smoke games (wading)

CPU census on the race world, shape C's keys without withering, seeds 5-8, cap 600 s, five hands, the new rule (`wade=6`) against the old count on C (`wade=30`):

```
larderworld census seeds=5-8 walkers=25000 upkeep=0.0002 keys=capacity:50000+energy_flow:0.05 hands=idle,random,farm,strike,rush cap=600 threads=4 wade=6|30
```

| Hand | Now (`wade=6`) | Before (`wade=30`) |
|---|---|---|
| idle | wiped on 2 seeds (586, 599 s), held to the cap on 2 at -0.333; never wins | the same games |
| random | wiped 4/4 at 69-114 s | wiped 4/4 at 72-114 s |
| farm | cap 4/4, +0.073 to +0.096 | cap 3/4 (+0.071 to +0.083), wiped once at 595 s |
| strike | cap 2/4 (+0.063, +0.073), wiped at 446 and 516 s | cap 3/4 (+0.040 to +0.079), wiped at 468 s |
| rush | wiped 3/4 at 121-215 s, cap once at -0.333 | wiped 3/4 at 137-214 s, cap once at -0.333 |

- **The checks pass.** Idle never wins (below 0 on every seed). The shortest game is 69 s (random). Every game ends in a wipe or at the cap. Farm beats idle and random on 4 of 4 seeds in both arms.
- **Paired by seed, now against before:** farm 3 higher, 1 lower; strike 0 higher, 4 lower (p 0.125; nothing survives Holm over five hands); rush 1-2 and a tie; random within 0.01. Idle soldiers never walk into the slime, so idle plays the same games. Strike walks its squad into the body, and the slowdown costs it; that is a tuning note for the balance pass, not a reason to hold the rule (build first, balance later).
- One binary for both arms, built from this PR's code before it was rebased onto #196 (which only turns withering on in the window). The provenance lines read 23601fc2b+dirty and 30c2fe515+dirty because the census reads the checkout's HEAD as it runs, and the rebase landed between the arms. 15 and 13 minutes on 4 threads. Raw: [`smoke-c-wade6.txt`](larder-wading-2026-11-13/raw/smoke-c-wade6.txt), [`smoke-c-wade30.txt`](larder-wading-2026-11-13/raw/smoke-c-wade30.txt).

## Cost (wading)

None. One multiply fewer in the walk.

## Aims that see (#198)

**The rule.** A unit's aim is the densest block within its sight **that it can see**: every cell on the line from the unit's cell to the block's centre cell is agar (`squad::sees`, `squad::block_cell`; WGSL `sees` in the `units` and `units1` passes). The line is whole-cell steps, `max(|dx|, |dy|)` of them, each offset truncated toward zero, in integers so both engines walk the same cells. It is walked only for a block that would beat the best so far, which picks the same block as walking every one. Soldiers, turrets and the garden's read all use it.

**The stand again** (`hand=hold@318,148`, 4-30 s, sink 1):

| | Shots | Killed | A shot | Health at 30 s |
|---|---|---|---|---|
| Before: aims through walls | 47 | 261 | 5.6 | 50 |
| Now: aims it can see | 198 | 486 | 2.5 | 78 |

- The squad fires at its full rate (8 soldiers, one shot a second each) and kills almost twice as many. It shoots the slime that comes at it, which is thinner, so fewer a shot. It ends healthier because that slime never reaches it.
- The thick body in the channel is out of range from there (about 43 world units against 30). So this stand still does not test the sink: with sink 0 it fires 195 and kills 512 (2.6 a shot). Raw: [`clip-hold-seen.txt`](larder-wading-2026-11-13/raw/clip-hold-seen.txt), [`clip-hold-seen-sink0.txt`](larder-wading-2026-11-13/raw/clip-hold-seen-sink0.txt).
- A stand nearer the channel (`hold@322,166`; the squad stopped at 289.5, 138, on the V's arm and by the larder) gave 4.7 kills a shot with sink 0 and 5.3 with sink 1 (182 and 190 shots). Raw: [`preview-near-s0.txt`](larder-wading-2026-11-13/raw/preview-near-s0.txt), [`preview-near-s1.txt`](larder-wading-2026-11-13/raw/preview-near-s1.txt).

**Tests.** `a_unit_aims_only_at_slime_it_can_see` (squad.rs) walls off the middle of a unit's line to its aim. The aim must move to a block it sees, and come back when the wall goes. The GPU's squad twins pass on llvmpipe, `the_squad_matches_the_cpu_for_one_frame` to the bit. `the_squad_frame_holds_its_cap_on_the_gpu` now expects no aim from the six units the clamp puts in the world's rock border (they see nothing); it still checks that every unit on agar past 64 aims.

**Cost.** On the CPU, `squad::read` for 40 units on the edge of C's body (shape C at 240 s, seeds 1 and 2, the same dish at either commit since the squad held fire) took 111-120 µs a frame before and 120-139 µs after (four timings each): about a tenth more, a quarter of a microsecond a unit, under 0.1% of a 22 ms CPU tick. The line is walked only for a block that beats the best so far. The probe is [`raw/aimcost.rs`](larder-wading-2026-11-13/raw/aimcost.rs) (copy it into `examples/` to run it), output in [`raw/aim-cost/`](larder-wading-2026-11-13/raw/aim-cost/). The GPU's `units` pass walks the same lines; its 1070 timing waits for the owner's go on timing runs, and the twin there checks only that it passes.

**Smoke games** (the wading smoke's command at the default `wade`, so the same settings hash as its `wade=6` arm; [`smoke-c-aim.txt`](larder-wading-2026-11-13/raw/smoke-c-aim.txt)):

| Hand | Aims that see | Aims through walls (`wade=6` above) |
|---|---|---|
| idle | cap 3/4 at -0.333, wiped once at 586 s; 792-1,309 shots | cap 2/4, wiped at 586 and 599 s; 122-1,309 shots |
| random | wiped 4/4 at 70-114 s | wiped 4/4 at 69-114 s |
| farm | cap 4/4, +0.074 to +0.089; 23,661-33,107 killed | cap 4/4, +0.073 to +0.096; 20,900-28,911 killed |
| strike | cap 2/4 (+0.068, +0.074), wiped at 436 and 526 s | cap 2/4, wiped at 446 and 516 s |
| rush | wiped 3/4 at 122-215 s, cap once | the same |

- **The checks pass.** Idle never wins (-0.333 at best). The shortest game is 70 s. Every game ends in a wipe or at the cap. Farm beats idle and random on 4 of 4 seeds.
- Idle's squad stands at the larder on Defend, and on seed 5 it fired 792 shots against 122 and held to the cap. Farm kills more (seeds 5 and 7: 33,107 and 31,849 against 26,322 and 28,911). Every other score moves by 0.017 or less.

**A test that leaned on the old aim.** `a_step_in_halves_plays_as_a_whole_one` gasses the mould round the first soldier touched and needs a gassed walker by a soldier afterwards. With aims that see, the first touch on seed 5 was one walker on an attack-moving soldier, which walked on from the five walkers the gas slept. The test now gasses at three walkers on a soldier: it sleeps 12, and 4 are still on a soldier at the first read.
