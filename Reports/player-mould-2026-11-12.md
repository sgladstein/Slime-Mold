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
   front and hold it (pictures). *Corrected after #132:* conversion does
   run there, and hard: counted per walker, a sown game turns about 7,200
   enemy walkers to the player and 7,900 back (32 seeds, hunt 0 below), a
   churn that nets to nearly nothing. So the fight is not missing contact;
   it is symmetric where the bodies touch.

## Each side's walkers (after #119)

The tech lead's review asked for a walker reserve before contact: in one
shared pool of 50,000 slots a sown player mould reached 32,000, so a later
fight could be won by filling the enemy's birth slots, which would mean
nothing on the GPU, where the pool is a different size.
`BigParams::species_cap` (empty is off, CPU only) caps each species' living
walkers: a parent divides only while its species held fewer than its cap
when the birth pass began. The garden gives each side half the slots,
25,000, and a taken enemy core sets the enemy's cap to 0, so the core rule
now stops the enemy's births and not the player's (before, the garden
stopped none). Tests: `species_cap_holds_one_side`,
`taken_core_stops_only_the_enemys_births`.

Rerun on the same 32 seeds
([`garden/census3-cap.txt`](player-mould-2026-11-12/garden/census3-cap.txt)):
idle and farm are identical (neither side nears 25,000); sow2's mould ends
at 21,987 against 23,023 uncapped, the enemy at 7,268 against 7,281. So
the shared pool was not starving the enemy here; the cap matters once the
moulds fight. Balance and share pairings are unchanged in direction and
significance (sow2 vs farm balance 32/0; vs random 25/7, p 0.002).

## Craters in the garden (after #113)

The garden's shots now lay the crater the one-mould game's do
(`Frame::crater`, `Rules::crater` 10 s), and an enemy walker standing in a
crater stops a shot. The crater fades every species' trail, the player's
too; shots stop at enemy slime, so it lies on the enemy's side. Test:
`shots_lay_the_crater_on_the_enemy`. Census rerun with craters
([`garden/census4-crater.txt`](player-mould-2026-11-12/garden/census4-crater.txt)):
sow2's mould 23,189 against the enemy's 6,971 (6,971 is the lowest enemy
of any census); share 0.955; balance against farm 32/0, against random
28/4. Idle is unchanged, as it never shoots.

## The front: does each action move the line? (designer's review)

Two moulds holding a line is an RTS front, so before making the enemy hunt,
the designer asked how far each action moves it. `garden front`: every arm
plays sow2 to 2:30, when the moulds meet, then the squad farms and soldiers
0 and 1 act for 40 s at the player's cell nearest the enemy's core.

**Deciding measure, named before the run:** the player's cells (own trail
over 1 and over the enemy's) 20 s after the branch, less the none arm's on
the same seed; second, the enemy's cells the same way. Sign test, Holm over
the eight +20 s tests. 32 seeds (one seed ties every arm: both soldiers
dead by 2:30).

| Arm (two soldiers, 20 s) | Player's cells vs none | Enemy's cells vs none |
|---|---|---|
| sow (Feed at the front) | **+609, 25 / 6, holds** | -87, 13 / 18 |
| shoot (Free fire at the front) | +267, 19 / 12 | **-396, 7 / 24, holds** |
| road (Road 15 cells past the front) | **+886, 27 / 4, holds** | -255, 8 / 23, p 0.011 (just outside Holm) |
| cut (Cut at the front, a check) | +122, 17 / 14 | +1, 16 / 15 |

With craters ([`garden/front-crater.txt`](player-mould-2026-11-12/garden/front-crater.txt)).
Means at the branch: player 23,000 cells, enemy 13,153. Each action moves
the line its own way: **a road pushes the player's ground forward, sowing
thickens it, and fire burns the enemy's back.** Without craters
([`garden/front.txt`](player-mould-2026-11-12/garden/front.txt)) shooting
took nothing (-37, 13 / 17): the crater is what makes fire hold ground.
All three last to +40 s (with craters: road +755 player cells 29 / 2, sow
+1,191 26 / 5, fire -579 enemy cells 7 / 24). The cut check moved nothing: one
soldier's disc of radius 3 is too small to see against 23,000 cells, so
Cut needs a longer blade to matter. The moves are 2 to 4 per cent of the
map in 20 s: visible at the tip where the soldiers stand (pictures on the
page), not a sweep.

## Cut as the toolkit's det cord (designer's order)

`Garden::cord` lays the squad toolkit's det cord (`KitRules`: a blast of
radius 2.5 dish cells every 2, a crater on each, alarm) on the garden, and
it hits both moulds alike (test `a_cord_cuts_both_moulds`). The front
probe's **cord** arm: once soldier 0 is within reach of the line, one cord
16 dish cells long, square to the way to the enemy's core and 6 past the
player's front, paid from the bank (48).

**Deciding measure, named before the run:** the enemy's cells 20 s after
the branch, cord less none, paired; second, the player's cells. Shoot
reruns beside it as the reference, identical to the earlier run seed for
seed. 32 seeds ([`garden/front-cord.txt`](player-mould-2026-11-12/garden/front-cord.txt);
seeds 28-32 of the cord arm in
[`garden/front-cord-tail.txt`](player-mould-2026-11-12/garden/front-cord-tail.txt),
as the first run hit its time limit).

| Arm | Enemy's cells vs none, +20 s | +40 s | Player's cells vs none, +20 s |
|---|---|---|---|
| cord (one, 16 long) | -289, 10 / 21, p 0.07 | -236, 11 / 20, p 0.15 | +357, 20 / 11, p 0.15 |
| shoot (two soldiers, 20 s) | **-354, 7 / 24, p 0.003** | **-667, 7 / 24, p 0.003** | +331, 19 / 12, p 0.28 |

Cord against shoot on the enemy at +20 s: 17 / 14, p 0.72. **One cord takes
about what 20 s of fire does at first, but does not hold**, and the
ground grows back by +40 s, where fire's keeps widening. It killed few
walkers (0, 53, 0 and 132 on seeds 1-4): at 2,500 walkers a side the line
is thin, so the cord works by wiping trail and laying craters, not by
killing. It cost the player's mould nothing seen, as it lands past the
player's front. As the cut tool it is no better than the gun here; it
needs a mould whose line is dense, or a cord that severs a supply tube.

## The enemy hunts the player's mould (designer's order)

`BigParams::species_avoid_of` (CPU only, empty is off) gives one species
its own trail-avoid weight; `hunt=H` sets the enemy's to -H, so it turns
toward the player's trail. The garden now counts every walker that
changes side (`Garden::turned_own`, `turned_enemy`).

**Deciding measure, named before the run:** the enemy's walkers at the cap,
hunt 1 against hunt 0, paired per seed; second, net turned (to the player
less to the enemy). Hands idle, farm and sow2, 32 seeds
([`garden/census5-turned-hunt0.txt`](player-mould-2026-11-12/garden/census5-turned-hunt0.txt),
[`garden/census5-turned-hunt1.txt`](player-mould-2026-11-12/garden/census5-turned-hunt1.txt)).
Holm over the twelve tests.

| Hand | Enemy walkers, hunt 1 lower | Player's walkers, hunt 1 higher | Net turned to the player, hunt 1 higher | Mean net turned, hunt 0 → 1 |
|---|---|---|---|---|
| idle | 20 / 10, p 0.10 | 22 / 8, p 0.016 | **27 / 3, holds** | -332 → +1,209 |
| farm | 23 / 9, p 0.020 | **29 / 3, holds** | **30 / 2, holds** | +236 → +1,181 |
| sow2 | 20 / 12, p 0.22 | 20 / 11, p 0.15 | **29 / 3, holds** | -751 → +626 |

**A hunting enemy feeds the player.** Walking into the player's trail, the
enemy's walkers are outnumbered where they arrive and convert, so net
turning flips to the player's side under every hand; sow2's end share does
not move (16 / 16), as the gain is small against 23,000 walkers. With
this conversion rule (a walker joins a species whose trail outweighs its own 1.2 to 1 nearby) attraction is
not a threat the player must answer; it is a gift. Hunt stays off. Making
the enemy a threat needs a rule where the attacker has an edge, such as
conversion that favours the moving or the fed walker, or a hunter that
attacks in mass rather than trickling in.

## The front hand (built under the testing rule of 17:26Z)

`garden` hand **front**: sow2 throughout (soldiers 0 and 1 sow the bank
thin); once the moulds touch (a cell over 1 in both trails, read once a
game second), soldiers 2 and 3 are a front team. Every 20 s they read the
line (the player's cell nearest the enemy's core), count each side's cells
within 12 dish cells, and pick: **fire** where the enemy holds more,
**sow** where the player holds more and the bank is over 600, else a
**road** 15 cells into the enemy. `GARDEN_TRACE=1` prints each pick. A
first version sent sow2's own sowers to the front: fire won nearly every
read, the bank piled up unspent (16,529 at 5:00, seed 1) and the player's
mould ended at 6,129 against sow2's 17,062.

Under the owner's new rule a census is not the gate; the feature lands on
a GIF ([page](https://claude.ai/artifact/Rv4ZqMJ1hQDnJKiVcAXihx), source
[`front-hand/`](player-mould-2026-11-12/front-hand/)) and smoke checks.
One small strategy check, named before the run (runner time about 20 min):
the moulds' balance at the cap, front against sow2, 8 seeds
([`garden/census6-front.txt`](player-mould-2026-11-12/garden/census6-front.txt)).

| Hand (8 seeds, 5:00) | Player's walkers | Enemy walkers | Soldiers at end |
|---|---|---|---|
| idle | 3,807 | 6,720 | 4.0 |
| random | 15,494 | 12,157 | 0.5 |
| sow2 | 21,332 | 7,888 | 8.0 |
| front | 21,632 | 6,501 | 8.0 |

Front against sow2: balance higher on 6 of 8 (p 0.29), enemy smaller on 6
of 8 (mean -1,387). Front beats idle and random on balance 8 / 0. Smoke:
every game ran to the cap, front lost no soldier, idle is last. A lean,
not a result; a tuning note.

## Scale: what these numbers do and don't carry

Every garden number is the CPU dish at scale 2 with 2,500 walkers a side,
on a 50,000-slot pool. Cells, radii and walker counts will not carry to the
GPU dish: there the mould is many times denser, so a disc or cord of the
same radius meets far more walkers, and a front that is 2-4 % of the map
here may be a different share there. What should carry is the direction
and the ranking (road pushes, sow thickens, fire burns back; a hunting
enemy is fed to the player under this conversion rule). The GPU port will
be measured at 20,000 walkers in all (10,000 a side) and at 5,000 (tech
lead's sizes; the final count is the owner's).

## Not done, next

- An attacker's edge, so a hunting enemy is a threat (above): conversion
  that favours the fed or massed walker.
- The front hand's thresholds (12-cell read, 20 s, fire when the enemy holds more) are first guesses; tune by play.
- Cut's reach (radius 3) is too small to see; one det cord takes about what fire does but does not hold.
- Cut was never used by a planned hand; random cut 5,899 walkers and lost.
- The GPU port, after the toolkit's and one body's rows.

## The GPU port and the window (own=1)

The garden now runs on the GPU engine and plays in the larder-world window
(`larder-world own=1`; [page](https://claude.ai/artifact/SLmH8k3C797UCo3949RcMn), seed 1 at 3:00: own 4,522, enemy 4,153). Off by default: without `own=1` no new pass runs and
the plain game is hash-identical (all twin tests unchanged).

- **Engine.** World bit `GARDEN = 32`; the player's mould is species 3 on the
  per-species trail (`sp_in`), the enemy species 0. Shots stop at the
  enemy's trail only (`enemy_trail` = trail minus species 3), so fire flies
  through green and stops at the first amber cell; strike skips species 3
  walkers; cut keeps species 3's trail; soldiers' slowdown counts enemy
  walkers only (`enemy_occ` = occupancy minus the per-cell own count). Eat
  runs over every cell in a garden (the GPU's pile-only eat could not feed
  the player's mould). New passes `tend` (Road and Feed entries, written in a
  tail after the squad path, `squad::TEND`), `ownclear` and `owncount`.
  Conversion uses the engine's own `convert` (1.2 at radius 4). No new
  Params row or binding (BINDINGS stays 48).
- **World.** `GpuWorld::garden` (refuses a grazing world, with a test: the
  own count shares the graze counter), `garden_settings`, `counts_by_side`,
  `World::tend` building the same Road/Feed discs as the CPU. Cut is not
  wired on the GPU yet; it becomes the det cord through `World::burst` (#159).
- **Window.** Sow (E) and Road (Y) on the command card toggle the task on the
  selected soldiers; the top bar shows the own count and labels the enemy.
  `hand=sow` plays it (soldiers 0-1 sow or road between the larder and a
  crystal, the rest farm). Tests: GPU garden starts as the CPU garden; shots
  spare the player's mould (a line through green hits amber); Road and Feed
  as the CPU; the window smoke test runs every command for 60 s.
- **Not yet on the GPU: `species_cap`.** Until it lands, both moulds draw
  births from one shared pool, so the enemy can crowd the player's mould out
  of it. A garden that loses ground late is that, not balance.
- **Cost.** On llvmpipe (software Vulkan, not a timing) `tend`, `ownclear`
  and `owncount` cost 0.2-0.5 ms a step each. 1070 timing at 5k and 20k
  walkers in all, with the garden's all-cells eat as its own number, is
  booked on the PC; the twin tests with `own` on and off are booked too.
