# Review: pieces and behaviour (L3, L4, B2-B5)

> Saved by the one-body thread from the reviewer's final message (subagents could not write files). Paths under `raw/` are this folder's `raw/`; pictures named in the text are in `img/` (see the report's picture list). Unedited otherwise.

(I couldn't save `report.md`: the harness blocks report files from subagents, so this message is the report. The raw probe output is in `raw/pieces/`.)

## 1. Is Jones the right backend for pieces and behaviour? Extend it, don't replace it

**Jones walkers are good tissue but not an organism.** They draw the network, heal cuts and give local reflexes: the hole held under fire, the pseudopod to food. Two things stop them from being one creature:

- **Nothing belongs to a piece.** No quantity is shared by a hunk and nothing else, so "this hunk is hungry, so it hunts" has nowhere to live. `energy_flow` is the only coupling, and it spans the whole mould. That is why random play starved it all at once (9 of 16 seeds).
- **Every timescale is the walker's.** Walkers cross about 15 dish cells a second. So gaps close in 1-2 s, hunks reach food in 2 s, and any intent becomes motion within 1 s. B3's 10-30 s arc and "intent a few seconds ahead" can't happen while the body's edge moves as fast as the walkers inside it.

**Jones's own body results need a different regime from ours.** His amoeboid blob (2012) and shrinking blob (2013) are packed sheets of 10,000-15,000 particles, updated one at a time in random order. We run a sparse network at 3-5% occupancy, updated in parallel claims. Our port of his amoeboid rules failed for exactly this reason (`dead-ends.md`: 92-94% of walkers ended up off any tube, or about 180 still droplets). A GPU can't run his sequential update cheaply.

| Option | What it gets | Cost |
|---|---|---|
| Keep and tune | smoother tubes from density; one part with H | no new passes, but H triples the mould and loses rush; pieces still burst |
| **Extend with a body layer** | L3, B2, B3, B5, intent | about 0.05-0.15 ms a frame on the 1070 (under 10% of 1.37 ms at 100k walkers); 2-4 weeks; census reset |
| Replace with a flow or continuum backend | real streaming | months; loses every baseline and the mesh look |

## 2. What our tests missed (new probe)

I re-ran the alone-hunk probe on the Fork map with a per-second series: seeds 1-4, 6 cuts an arm (`cutprobe map=fork ring=8 cutw=4 alone=1 unfed=8 after=20 series=…`).

| Game seconds after the cut | 0 | 1 | 2 | 3 | 5 | 20 |
|---|---|---|---|---|---|---|
| Largest part, today | 1.00 | 0.42 | 0.34 | 0.33 | 0.26 | 0.46 |
| RMS radius in dish cells, today | 4.0 | 11.9 | 22.1 | 27.7 | 33.1 | 29.6 |
| Largest part, H | 1.00 | 0.53 | 0.34 | 0.31 | 0.33 | 0.29 |

- **A hunk doesn't straggle on the way to food. It bursts within 1 s, before it eats.** It starts as one lump 4 dish cells across. By 1 s it is 5-6 parts with three times the radius. Its first bite comes at 1-3 s. The walkers run out along the stale roads around them.
- **The deciding measure (largest part at 20 s) scores the wrong thing.** By 20 s the hunk has founded a colony on a new crystal, and its children are tagged too. So the number measures how that colony re-gathers, not whether the hunk moved as one. Measure at 1-3 s and at first food contact.
- **The hunks are tiny.** They held 13-42 walkers here, and the earlier map runs averaged 21-47. A lump that small is a few pixels, and one walker moves the share. Test pieces of 300-1,000 walkers, the size a player would cut off.
- **Every cohesion rule was applied to the whole mould,** so each paid its cost on the main network: `lone_upkeep` stopped the spread, `tether` split the body into hunks, `lone_step` cost about 25% of the spread. None was tried inside cut pieces only, which needs a piece id.
  - A boundary already works. A held ring gives one round lump (largest part 1.00, compactness 0.99).
  - One extra case: a hungry-gated `tether` 2 hunk (25 walkers) still burst (0.36 at 1 s). It's a single case, so it proves nothing, but 25 walkers can hardly satisfy tether 2.
- **"Largest part" bridges gaps of 2 dish cells,** so strings of specks count as connected and the measure can't see the specks the owner sees. Add a look measure, for example the share of walkers with at least 3 of their 8 neighbouring cells occupied.

## 3. What biology and the papers suggest

- **The body moves 20-140 times slower than its flow.** A migrating fragment moves at 0.17 µm/s, its cytoplasm streams at ±3-5 µm/s, and the contraction wave travels at 24 µm/s (Lewis et al. 2015, J R Soc Interface).
- **Mass piles up at the front.** Speed tracks front thickness (r 0.93); large plasmodia are a sheet in front with veins behind (Kuroda et al. 2015, J Exp Biol). The front swells before it advances, and that swelling is visible intent.
- **Signals ride the flow,** so the whole body acts as one sensor (Alim et al. 2017, PNAS; Alim et al. 2013, PNAS).
- **Jones's blob migrates because of a front-to-back difference in pull** (2012), and stays whole through birth and death gated by local density (2013).

## 4. Ranked directions

**1. Piece layer.**
- **Mechanism:** connected components on a 4x4-coarsened occupancy grid (128x80 or 256x160 cells), dilated by one coarse cell to match the census bridge, run every 8 ticks.
  - Per-piece sums: walkers, energy, alarm, food contact, centroid.
  - A small table is read back.
  - Ids stay stable by overlap, and a piece must stay apart for 1 s before it counts.
- **Why:** every direction below needs it, and so do sound and graphics.
- **Cost (estimate, not measured):** block union-find takes 0.4-0.7 ms on 0.1-0.2-megapixel images on a Quadro K2200 (Allegretti et al. 2019). The 1070 has about 3x its cores and bandwidth, so expect 0.05-0.1 ms a run, limited by dispatch overhead. Every 8 ticks that is under 0.02 ms a frame, plus about 0.02 ms a tick for per-walker atomics, like `esum`.
- **Risk:** piece ids flicker as streams break and rejoin.
- **First experiment:** on the CPU, `cutprobe`'s `label` already exists. Log the pieces in a grown race world for 300 s. Deciding measure: pieces of 50 or more walkers that last at least 2 s, per minute.

**2. Creep.**
- **Mechanism:** a walker's step is multiplied by 0.1-0.3 when it moves into a cell whose trail is below `tube`; on trail it moves at full speed.
- **Why:**
  - Trunks still refill a gap in about 2 s.
  - Fronts advance 2-4 dish cells a second, slower than soldiers (P1).
  - Followers pile up at the tip, so the tip swells before it moves. That is L4's intent, a few seconds ahead.
  - This differs from `lone_step`, which slowed everything outside a crowd, streams included.
- **Cost:** about 0 ms; the trail is already read.
- **Risk:** stale roads still count as tube, and every census baseline moves.
- **First experiment:** this needs a new field. Deciding measures: front speed, gap refill time (2 s or less), crystals at 300 s, and the look measure.

**3. Piece regimes (B2, B3).**
- **Mechanism:** walkers in a piece cut off from the core get:
  - `tether` 2, acting as a membrane;
  - high `energy_flow` inside the piece, so it lives or dies whole over 10-30 s. That was a harm for the whole mould and is the goal for a hunk.
  - a shared drift: the piece's summed food-scent gradient, plus a "kin scent" laid only by the main body. So a piece hunts or creeps home, which is `dead-ends.md`'s own re-test condition for body pull.

  The main body keeps today's rules.
- **Cost:** the kin field is about 0.03-0.05 ms a tick at 1024x640.
- **Risk:** it can look scripted, so ramp the switch in over 1 s with hysteresis.
- **First experiment:** hunks of 300 or more walkers. Deciding measures: largest part at 3 s and at food contact, and time to starve or rejoin when no food is near (target 10-30 s).

**4. Graded alarm (B4, B5).**
- **Mechanism:** each walker weighs the alarm by `lerp(flee, swarm, energy / hunger_at)`, plus a positive `scar_weight`.
- **Why:**
  - Hungry fringe walkers recoil and hold the hole open (102 against 29 dish cells with the flee).
  - Fed walkers at the core and crystals swarm the wound, which is the flee's own re-test condition.
  - The alarm fades in about 1 s while the scar lasts about 40 s, so the mould floods back 2-5 s after fire stops. The sequence reads as a duel: approach, recoil, surge.
- **Cost:** one multiply in `sense`.
- **Risk:** walker energy may be a noisy proxy for "near the core" at the frontier.
- **First experiment:** the census gate. Deciding measures:
  - the gate holds 6 of 6 under Holm;
  - random wipes the mould on no more than 4 of 16 seeds (today 2);
  - the fringe hole is at least 80 dish cells after 20 s of fire;
  - first touch on the squad comes no later than today.
- **Later:** give soldiers their own scent channel, weighted by hunger and recent alarm. Today a soldier smells like half a pile, so "comes for them" is just foraging.

**5. Density with `walker_mass` (L1).**
- **Mechanism:** 4-5x the walkers, each counting 1/4-1/5 of today's mass in eating, upkeep and damage.
- **Why:** at 20,000 walkers the mould already "reads much more like one body".
- **Cost:** well inside the 100,000-walker budget.
- **First experiment:** hold total mass within 10% of today; measure the look measure and the gate.

## 5. Emergent behaviour, a layer on top, and the ruling

Plain Jones gives B4's reflexes emergently, but not B3 or identity. A body layer doesn't break "everything on screen is the simulation" if:

- it reads only simulation state;
- it acts only through the walkers' sensing, steps, energy, births and deaths;
- it draws nothing: no outlines, no colour by piece.

Then every visible effect is still walkers moving, thinning and dying. Keep the modes as threshold functions of piece state, not a planner, and twin-test the layer on CPU and GPU.

## Correctness concerns

- With `alone=1` the removed mould's trail stays behind, so hunks run along dead roads.
- Rules applied to the whole mould grow different worlds from the same seed, so "paired" hunks are not the same hunk. With global `tether` 2, no hunk met the probe's cut rule at all (0 cuts in 4 seeds).
- The GPU cost figures in section 4 are estimates from the literature, not measurements.

Sources: [Allegretti et al. 2019, BUF](https://iris.unimore.it/bitstream/11380/1179642/2/2019__ICIAP_A_Block_Based_Union_Find_Algorithm_to_Label_Connected_Components_on_GPUs.pdf); [Lewis et al. 2015](https://www.math.ucdavis.edu/~guy/papers/lewis_RSI_2015_physarum.pdf); [Kuroda et al. 2015](https://cob.silverchair.com/jeb/article/218/23/3729/14430/Allometry-in-Physarum-plasmodium-during-free); [Alim et al. 2017](https://brennergroup.seas.harvard.edu/publications/mechanism-signal-propagation-physarum-polycephalum); [Alim et al. 2013](https://brennergroup.seas.harvard.edu/publications/random-network-peristalsis-physarum-polycephalum-organizes-fluid-flows); [Jones and Adamatzky 2013, shrinking blob](https://www.alphaxiv.org/abs/1303.4969); [Jones and Adamatzky 2012](https://www.alphaxiv.org/abs/1212.0023).