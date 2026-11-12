# The squad toolkit: six tools from the mould's model (2026-11-12)

Thread *Design squad tools from the mould's model*. The owner played the Larder
world on 2026-11-12 and said shooting "didn't feel satisfying", the slime
"mostly ignored my soldiers", and paying energy to shoot "isn't fun"
(04:13Z). The Larder thread proposed a toolkit built from the simulation's own
fields; the owner said "yes and maybe explosions, mines, etc. do we need
scortch when we have walls?" (04:25Z). After the proposal below, the owner
chose "All six together" on the card (04:31Z), and on the integration
thread's card "Drop it, scorch is the crater" (04:46Z). For the shots: "A
group of soldiers shooting the slime should start carving out a bigger and
bigger hole that doesn't refill while being shot at" (04:39Z, graphics A/B).

Everything here runs on the CPU race world (`larder_world::race_world`, 512 x
320, 5,000 walkers growing, 23 crystals). None of it is on the GPU yet; that
is step 2 (below).

## The model, neutrally

Walkers read the trail ahead with three sensors, turn toward the strongest,
step and lay trail. Trail spreads and fades. Scent from crystals, soldiers
and the larder pulls them. Each walker holds energy, pays upkeep, divides when
fed and starves when not. With the life cycle on, a walker can sleep
(dormant) and an alarm field wakes it. Each tool works on one of those terms.

## The tools (`mould::kit`)

| Tool | Term of the model | What it does |
|---|---|---|
| Blast | walkers, trail, alarm, a new crater field | kills within `blast_r` (6 dish cells), clears trail, lays alarm, scorches a crater for `crater_secs` (20 s) |
| Mine | the blast, triggered by walker count | laid unscented; goes off when `mine_trigger` (40) walkers stand within `mine_r` (5) |
| Det cord | blasts along a line | blasts of `cord_r` 2.5 every 2 dish cells, each with its crater |
| Germ | energy (new `sick_upkeep`) on the old contagion | sick walkers lay germ and pay 60 x upkeep for 6 s, then 40 s immune |
| Sleep gas | life cycle (dormant) | walkers within `gas_r` (12) sleep at once, deaf to scent for 20 s; alarm wakes them; sleepers don't eat or bite (`World::calm`) |
| Mask / reek | scent | a soldier's smell x 0 or x 4 (`Soldier::smell`) |

Engine additions, all off by default (the default big-dish hash
`869141a7bf398adf`, `bigdish preset=dense seconds=2 blob=0 piles=1`, is
unchanged): `BigParams::scorch_keep` (0.5) and `scorch_full` (300 fine
steps) with `BigDish::scorch_disc`, the one crater writer (a per-cell
countdown; trail x `1 - (1 - scorch_keep) x min(1, left / scorch_full)` each
fine step, so every crater fades out; a shot can write a short, faint one);
`BigDish::gas` and `gas_death`; `sick_upkeep`; `enable_life`,
`enable_alarm`, `enable_contagion`; `gassed_near`.

**Scorch against walls.** A wall stops walkers, most scent, the squad's
soldiers and its shots, and the mould gnaws it. Scorch only stops trail from
holding: walkers cross it but can't keep a tube there. That matters right
after a blast or a cut, so scorch is the crater, not a tool or building of
its own. The owner agreed (04:46Z) and the buildings thread dropped its
Scorcher.

## Tool by tool (`examples/kitprobe.rs`)

The race world grows for 60 s with the squad on Hold Fire (it can't die), then
the tool is used at a point picked from the mould and followed. 8 paired
seeds, two-sided sign test. Raw: `squad-toolkit-2026-11-12/raw/probe-*.txt`.
The deciding measure for each was named in the example's header before the
run.

| Test | Deciding measure | Tool | Without the new part | None | Pairs |
|---|---|---|---|---|---|
| blast | walkers in the disc at 10 s | 82 (crater) | 188 (no crater) | 370 | crater lower 8/8, p 0.008 |
| fire, 4 soldiers 20 s | empty cells within 12 dish cells at 20 s | 1,174 (crater at each hit) | 1,037 (plain fire) | 931 | higher 6/1, p 0.125 (n.s.) |
| germ | living at 30 s | 796 | 3,590 (steering only) | 3,625 | lower than both 8/8, p 0.008 |
| gas | crystal charge eaten in 20 s | 1 | | 1,090 | lower 8/8, p 0.008 |
| gas + a blast 10 cells off at 5 s | asleep at 10 s | 188 (with noise) | 812 (quiet) | | lower 8/8, p 0.008 |
| det cord | trail on the cut at 10 s | 13 (crater) | 144 (no crater) | 122 | lower 8/8, p 0.008 |
| mines, a ring of 4 at the larder | walkers within 12 cells of the larder at 20 s | 304 | | 407 | lower 6/0, p 0.031; gone by 60 s (5/1, p 0.22) |

Six deciding comparisons; the five at p 0.008 survive Holm (smallest
threshold 0.05/6 = 0.0083). The fire test does not, and the mines' p 0.031
does not.

What this says:

- **The crater is what makes a hole stay a hole.** Without it the blast's
  hole half refills in 10 s (188 walkers back of 370) and the cord's cut
  carries more trail than before at 10 s. With it the hole holds 82 walkers
  at 10 s and 99 at 20 s, and the cut stays open for about 15 s, then fades.
  Walkers, not only trail, stay out: the walker pictures on the page show
  empty ground inside the crater.
- **Sustained fire carves more with a crater per hit, but it's not proven.**
  1,174 against 1,037 empty cells after 20 s of four soldiers firing (6/1).
  Only 32 shots were fired in 20 s at the default `shot_every` and price, and
  a hit crater was half a blast's (`strike` x 2 radius, 10 s). The Larder
  thread owns the shot's crater size; this is the writer it calls.
- **The germ needs the energy cost.** Steering-only contagion does nothing
  measurable at 30 s (as `dead-ends.md` found); with `sick_upkeep` 60 it
  kills about three quarters of the mould in 10 s and leaves hollow tubes. It
  is strong, maybe too strong (below).
- **Sleep gas works and noise ends it.** A gassed crystal is not eaten; a
  blast 10 cells off wakes three quarters of the sleepers within 5 s.
- **The det cord does not starve anything yet.** Walkers beyond the cut were
  no fewer at 30 s (higher 6/2, p 0.29), because food doesn't flow through
  the mould (`energy_flow` 0). That is the one-body thread's work.
- **Mines are weak.** A ring at the larder thinned the mould there at 20 s
  (6/0) but not by 60 s. Three mines on the straight line from the core to
  the larder killed a mean 6 walkers in 60 s: the mould's road isn't straight.

**Frame cost** (CPU, one thread, 512 x 320, from the plans' ms a tick): the
crater and gas are a pass over cells and walkers only while they exist, lost
in the noise (8-9 ms a tick for farm and siege alike). The germ is not: a
running germ field doubles to triples the tick (17-31 ms against 8-15 ms),
the same germ spread contagion always had.

## The four plans (`examples/kitplans.rs`)

Each plan farms like the farm hand and adds a tool pair. **Loud**: a reeking
decoy at the larder, four mines round it, a blast on any thick mould in reach,
and a runner carrying a blast to the mould's densest block every 3 s.
**Quiet**: every soldier masked, Hold Fire, gas any mould that reaches a
harvester. **Plague**: a runner carries the germ to the densest block whenever
no wave is running and the last wave's immunity is over. **Siege**: a det cord
across the mould nearest the larder every 20 s. Controls: idle, random, and
random with tools (`randkit`). 8 seeds, cap 600 s, score v2 (`Game::score`).
Tier none ran here; Swarmer, Sleeper and Sporer on three runner threads
(their per-game lines are in the runners' thread replies; the tables are in
`raw/`).

Mean score (ends: mould wiped / cap / squad wiped):

| Hand | none | Swarmer | Sleeper | Sporer |
|---|---|---|---|---|
| idle | -0.69 (0/5/3) | -0.52 (0/6/2) | -0.23 (0/8/0) | -0.83 (0/4/4) |
| random | -1.63 (0/0/8) | -1.71 (0/0/8) | -1.67 (0/0/8) | -1.72 (0/0/8) |
| randkit | -1.54 (0/0/8) | -1.64 (0/0/8) | -1.62 (0/0/8) | -1.63 (0/0/8) |
| farm | +0.20 | +0.02 (0/7/1) | +0.27 | +0.13 |
| **loud** | **+1.76 (8/0/0)** | **+1.96 (8/0/0)** | **+1.44 (6/2/0)** | **+1.18 (6/2/0)** |
| quiet | +0.13 | +0.13 | +0.27 | +0.09 |
| plague | +0.84 (3/5/0) | +0.92 (4/4/0) | +0.47 (0/8/0) | +0.74 (2/6/0) |
| siege | +0.20 | +0.00 (0/7/1) | +0.27 | +0.12 |

**Gate 1 (each plan beats idle, random and randkit, per tier): every pair is
8/0 or 7/1 in the plan's favour**, 60 comparisons. At 8 seeds the smallest
sign-test p is 0.0078, so none survives Holm over 15 a tier; 16 seeds would
be needed to call it.

**Gate 2 (the best plan changes between tiers): fails.** Loud is best in all
four columns, and every other plan is below it on 8 of 8 seeds in three
columns (Sporer: 6 of 8, p 0.29). Its weak spots are real but small: on the
Sleeper and the Sporer it wipes 6 of 8, not 8 of 8. On the Sporer twice the
blasts didn't keep up: the squad ended at one soldier with a mould of 14-20k
(spores reseed what blasts clear). On the Sleeper the germ loses its wipes
(0 of 8, against 2-4 elsewhere): a dormant crust neither moves nor catches
much germ.

A first try at a costlier loud (blast price 120, reach 15 dish cells, the gun's
range) still wiped 4 of 4 on tier none, and a germ at `sick_upkeep` 20 still
left 2-17 walkers on 3 of 4 (`raw/plans-none-reach15.txt`). Price alone
doesn't break it.

## What this means

The six tools each do what they say, on screen and in the counts. Plans built
on them beat idle and random play. But "carry explosives to the body" beats
every mould we have, because no tier answers a blast. Per the owner's ruling
that difficulty comes from mould types, not from scaling tools back (11-11
14:14Z), the fix is a mould that answers, not a weaker blast. Candidates, from
the model:

1. **A mould that scatters when struck** (the Sporer's `spore_burst` on
   alarm, already in the engine): blasts reseed it. The Sporer column already
   shows the start of this.
2. **Habituation** (review 2's candidate, and the design reviewer's note
   04:35Z): a tier whose sensors weigh the alarm left by the tool used most
   recently, so it avoids blasts after a few and swarms gas.
3. **A crust that ignores germ** (the Sleeper already blunts plague).

The loud plan should then lose to one of them while plague or quiet wins
there. That is the next census, on 16 seeds and two maps.

## Step 2: in the game

Agreed with the integration thread (04:38Z): the tools move into
`squad::Frame` as `bursts: Vec<Burst { at, r, kill, alarm, alarm_r, crater,
crater_steps, sleep, germ }>` on the CPU and GPU, with the scorch countdown on
binding 46 after the soil row. The buildings thread's Siren will be a burst
with only alarm. Merge order on `larder_world.rs` is the integration thread's:
Larder, controls 1, buildings 1, then this.

## Pictures

[Page](https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC) (source in `squad-toolkit-2026-11-12/page/`): walker pictures
(one dot a living walker, crater cells tinted from the scorch field) and trail
pictures for blast, fire, cord and gas on seed 1, and the whole world for the
germ, with the counts beside each.
