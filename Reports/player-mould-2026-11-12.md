# The player's own mould: do body forms counter each other? (2026-11-12)

The owner approved a player-owned mould species on 2026-11-11 at 14:54Z,
and on 2026-11-12 at 04:45Z chose its shape on a card: **gardeners**, where
soldiers grow the player's mould (feed it, lay its roads, cut it) and it
fights the enemy mould by conversion. The proposal, written from the model
rather than from the old demo's tools, is the Claude Doc
https://claude.ai/artifact/LBDQgb8x9RoH6bE11G8H3m; its text is copied in
[`player-mould-2026-11-12/proposal.md`](player-mould-2026-11-12/proposal.md).

This report is the probe the proposal named first: whether the model itself
gives body forms that beat one form and lose to another, so that a
player's choice of form is a strategy and not a stat. No game rule and no
default changed.

```
cargo build --release --example ownmould
./target/release/examples/ownmould census seeds=1-16 forms=net,hunk,scout,heavy threads=4
./target/release/examples/ownmould look=DIR seeds=2 forms=net,hunk secs=180 every=90
```

## Setup

- **World**: per seed, the race world (`larder_world::race_params`, scale
  2, 5,000 walkers growing in 50,000 slots, crystals that run dry, rock and
  wet ground) taken as a map with its left half mirrored onto the right.
  Two species start on mirrored home crystals, 2,500 walkers each.
- **Fight**: conversion over a square (`species_convert` 1.2,
  `species_convert_r` 4), species blind to each other's trail
  (`species_avoid` 0).
- **Forms** (`SpeciesSet`s from the race mould): `net` (the race mould),
  `hunk` (the Drifter droplets: crowd 1, sensor 90, turn 45, offset 9,
  deposit 5, keep 0.93, not grazing), `scout` (explorers, 60 steps),
  `heavy` (three times the deposit, half the step).
- **A game** of A and B is played with A on each side and A's measure is
  the mean, so the per-seed side bias cancels (on seed 1 the right side
  out-ate the left 0.641 to 0.359 with the same form on both).
- **Deciding measure, named before the run**: A's share of the crystal
  charge both species ate over 3 game minutes (each walker's `took`, by
  its species). A beats B on 14 or more of 16 seeds (sign test p 0.004,
  inside Bonferroni over 6 pairings).

## Looking first: two fixes before the census

1. **Raw trail makes deposit a stat.** Conversion compared trail, so
   `heavy`, laying three times as much, converted the race network whole:
   on seed 1 the network fell from 2,500 to 50 walkers by 60 s and 0 by
   3:00. A switch now weighs each species' trail over its own deposit
   (`BigParams::species_convert_per`, off by default, CPU only), so
   conversion counts walkers' presence. The census runs with it on.
   Test: `conversion_per_deposit_counts_presence` (raw trail leaves under a
   tenth of the light species, per deposit over nine tenths).
2. **Grazing hunks fill the slot cap.** With grazing on and growth on, the
   hunks ate the grass and bred to 49,744 walkers by 3:00 on seed 1. The
   census's hunks don't graze.

## Who eats more (the deciding measure)

| A vs B | A ahead (of 16) | p | Median A share | Mean walkers at 3:00 A / B | Crystals held A / B |
|---|---|---|---|---|---|
| net vs hunk | **14** | **0.004** | 0.570 | 4,324 / 1,345 | 6.0 / 2.0 |
| net vs scout | 11 | 0.21 | 0.598 | 4,635 / 4,209 | 5.8 / 5.5 |
| net vs heavy | 7 | 0.80 | 0.485 | 3,101 / 3,661 | 4.0 / 5.8 |
| hunk vs scout | 5 | 0.21 | 0.457 | 1,400 / 3,914 | 2.0 / 5.3 |
| hunk vs heavy | 6 | 0.45 | 0.471 | 1,296 / 2,036 | 1.9 / 3.4 |
| scout vs heavy | 4 | 0.08 | 0.455 | 2,460 / 3,549 | 3.1 / 6.0 |

Only net over hunk survives. Read as a ranking it is a ladder, not a cycle:
net and heavy, then scouts, then hunks. The proposal's pass bar (no form
beats all three others) is met only because few pairings reach
significance. Raw: [`raw/census-per1.txt`](player-mould-2026-11-12/raw/census-per1.txt).

## Who wins where they touch

Counting walkers that change species (alive before and after a tick)
showed that the census above mostly measured an expansion race: on seed 2
no pairing turned more than 125 walkers in 3 minutes. But where the bodies
did meet, the fight went the other way from the eating:

| A vs B (8 seeds) | Seeds with contact | Contacts won by A / B | Turned to A / B, mean a game |
|---|---|---|---|
| net vs heavy | 7 | 0 / 7 (the 8th: 1 vs 0) | 673 / 2,048 |
| net vs hunk | 7 | 0 / 7 | 36 / 399 |
| hunk vs heavy | 6 | 5 / 1 | 110 / 6 |

A seed is won by the side more walkers turned to. 7 of 7 is p 0.016, just
inside Bonferroni over the three pairings. So the slow, dense forms win
fights and the fast network wins the economy: **the network out-eats the
hunks (14 of 16) and loses every contact to them (7 of 7).** That is two
axes, not one ladder, and it is the opening a gardener needs: whether to
spread your mould to eat or bunch it to fight is a choice with a cost.
Raw: [`raw/turned.txt`](player-mould-2026-11-12/raw/turned.txt).

## The fight's settings (4 seeds, net vs heavy)

| Settings | Turned to net / heavy, a game | Net's median share |
|---|---|---|
| convert 1.2, r 4, blind (census) | 673 / 2,048 (8 seeds) | 0.485 |
| convert 1.0, r 4, blind | 283 / 861 | 0.594 |
| convert 1.05, r 8, blind | 584 / 1,225 | 0.545 |
| convert 1.2, r 4, attracted (avoid -0.5) | 449 / 2,077 | 0.316 |
| convert 1.0, r 8, attracted (avoid -0.5) | 953 / 2,366 | 0.318 |
| convert 1.0, r 4, attracted (avoid -1) | 598 / 1,958 | 0.278 |

Attraction (a species drawn to the other's trail) brings the bodies into
contact and the dense form then takes the economy too (net's share falls
to about 0.3). Four seeds each: direction only. Raw:
[`raw/sweep.txt`](player-mould-2026-11-12/raw/sweep.txt).

## What this means for the gardeners build

- Form is not a counter cycle by itself; it is a trade between eating and
  fighting. The soldiers are what can make it a choice: Road spreads your
  mould to eat, Feed and Cut bunch it to fight.
- The fight needs contact. With the species blind to each other it is rare;
  the enemy tiers can be made to seek the player's mould (attraction) where
  a level wants a fight.
- Cost: `species_convert_per` adds one division a species a walker to
  conversion (not timed; conversion over a square of radius 8 costs 1.3 ms
  a frame on the 1070 at 100,000 walkers, [species on the
  GPU](species-gpu-2026-11-11.md)). It is CPU only; the GPU port is the
  GPU thread's when the game turns it on.

## Not done

- Crust and fliers: the life-cycle rates are global, not per species, so
  those forms need per-species life settings first.
- The one body thread's cohesion rules (energy flow, hunger scent, tether)
  change every form; the matrix should be rerun when they land.
- Pictures: [`player-mould-2026-11-12/pics/`](player-mould-2026-11-12/pics/),
  seed 2 at 0:01, 1:30 and 3:00 for each pairing, counts in `looks.txt`.

---

# Gardeners in the Larder world: the first CPU build (2026-11-12)

The build the owner's card chose, on the CPU world, behind its own world
type so the one-mould game is untouched.

```
cargo build --release --example garden
./target/release/examples/garden census seeds=1-32 hands=idle,random,farm,feed,sow,sow2 threads=4
./target/release/examples/garden look=DIR seeds=1 hands=farm,sow2 every=150
```

## What was built

- **`src/mould/own_mould.rs`**: `Garden`, a `larder_world::World` that holds
  two species on one `BigDish`. The enemy is species 0 and starts on
  crystal 0. The player's mould is species 3 (`OWN`, the slot agreed with
  the integration thread: enemy 0, new tiers 1 and 2, player 3) and starts
  on the crystal farthest from it, which is also the squad's landing (the
  larder). 2,500 walkers a side, growth on, blind to each other
  (`species_avoid` 0), conversion 1.2 over radius 4, `species_convert_per`
  on.
- **Shots, smell and bite tell species apart**: a shot stops at the first
  enemy slime and kills only enemy walkers; soldiers read and are bitten by
  the enemy only. The player's mould is harmless to the squad.
- **Tasks** (one a soldier, worked in a disc of radius 3 around it):
  **Road** lays the player's species trail (30 a second), **Feed** spends
  the bank (20 a second, while it lasts) as crystal charge spread thin on
  the disc's ground, **Cut** kills the player's walkers in the disc and
  clears its trail.
- **The hook in `larder_world.rs`**: one trait method,
  `World::tend(soldiers, bank, ticks) -> spent`, default 0, called once a
  frame before the larder unit. Every existing world keeps the default, so
  the one-mould census is unchanged (all `larder_world` tests pass).
- Tests: `the_garden_starts_two_moulds`,
  `tasks_act_on_the_players_mould_only`,
  `soldiers_leave_the_players_mould_alone`.

**Frame cost**: one 300 s game (4,500 frames, one thread) takes 68 s
farming and 118 s sowing on seed 1, 15 and 26 ms a frame. The gap is the
player's mould (1,227 against 24,508 walkers at the end), not `tend`, which
touches 8 discs of 29 cells.

## The hands

- **idle** gives no orders. **random** gives one random soldier a random
  move and a random task every 5 s.
- **farm**: the scripted farm, soldier k clears the k-th nearest crystal,
  no tasks.
- **road**, **feed**, **grow** (first census): farm, plus soldier 0 roads
  the player's mould to an unclaimed crystal, feeds it standing on the
  larder, or both.
- **sow**, **sow2** (second census): farm, plus soldier 0 (and 1) walks
  from the larder to the nearest unclaimed crystal and back on Feed while
  the bank is over 600 (else Road), so the charge lands thin along a path.

## Measures, named before each run

1. The player's **share** of the crystal charge gone: the squad's harvest,
   plus the player's mould's part of what the moulds ate (split by their
   walkers' `took`).
2. Added before the second census, after the first showed that the share is
   the squad's harvest whatever the gardeners do: the **balance**, the
   player's walkers less the enemy's at the 300 s cap.

Pairs by seed against idle, random and farm, two-sided sign test, Holm
over every comparison.

## Results (32 seeds, second census)

| Hand | Median share | Player's walkers | Enemy walkers | Balance vs farm (higher / lower) | Share vs farm |
|---|---|---|---|---|---|
| idle | 0.418 | 4,038 | 6,762 | 23 / 9 | 0 / 32 |
| random | 0.761 | 18,700 | 10,876 | 29 / 3 | 0 / 32 |
| farm | 0.946 | 1,723 | 5,369 | n/a | n/a |
| feed | 0.936 | 2,052 | 5,864 | 18 / 14, p 0.60 | 13 / 19, p 0.38 |
| sow | 0.953 | 18,629 | 7,211 | **31 / 1** | 21 / 11, p 0.11 |
| sow2 | 0.952 | 23,023 | 7,281 | **32 / 0** | 23 / 9, p 0.02 |

Walkers are means at the cap. Every planned hand beats idle and random on
share 32/32 (Holm). Raw:
[`garden/census1.txt`](player-mould-2026-11-12/garden/census1.txt) (first
census: road, feed and grow, each within 0.005 of farm on share; random
and farm moved a little in the second census, after the hands' code
changed between the runs; idle is identical), [`garden/census2.txt`](player-mould-2026-11-12/garden/census2.txt).

What the numbers say:

1. **Gardening grows the player's mould, and you can see it.** Sowing
   thinly along a walk gives 13 times farm's mould (32 of 32 seeds for
   sow2), and the player's species holds half the map by 2:30 on seed 1.
   Feeding in one spot does nothing (18 / 14): the disc piles charge faster
   than the walkers there eat it. Farming alone starves the player's mould
   (1,723, under idle's 4,038), because the squad clears its crystals.
2. **The player's mould doesn't beat the enemy.** The enemy is *bigger*
   under sowing than under farming (fewer on only 5 and 4 of 32 seeds),
   since the squad shot it less while two soldiers sowed. The share gain is
   small (sow2 23 / 9, p 0.02, not inside Holm over these comparisons).
   With the species blind to each other, the two bodies meet along one
   front and hold it (pictures), so conversion barely runs. This is the
   form probe's finding again: **the fight needs contact**.

## Not done, next

- Contact: make the enemy seek the player's mould (`species_avoid` below 0,
  per side), so a big player mould is a threat it must answer, and see if
  the network-vs-dense trade then makes Road against Cut a choice.
- A per-side walker reserve, so a big player mould cannot fill the enemy's
  birth slots (the tech lead's point); stopping births by species.
- Cut was never used by a planned hand; random cut 5,899 walkers and lost.
- The GPU port, after the toolkit's and one body's rows.
