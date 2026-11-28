# Scents and trails: a review of the slime's smell fields (2026-11-28)

Scott's ask (11-28 07:19Z): "deeply examine the physics/behavior/simulation
of our scents and trails. Are they ideally designed? How could they be
better utilized or improved in the game." A review only; nothing here is
built into the game. The page is `index.html` (Artifact:
https://claude.ai/artifact/4pKJJThk9uQUDdyXpXiBx7).

Measured on main `351dcd5` (after #901; nothing in the fields changed by
`469c1d2`), planets 1 and 2, run seed 1, `hand=greedy`, headless on
lavapipe, with the scratch patch `scent-dump.patch` (not for merging)
writing the GPU trail, smell, alarm and walker counts and the CPU far smell,
lure and hunt maps. `analyse.py DIR MS` measures a dump and draws its maps;
`theory.py` gives each field's half-life and spread and a lone crystal's
settled profile. GPU timings are the Lab's from the GTX 1070
([lab-walker-sort-2026-11-24](../lab-walker-sort-2026-11-24/README.md));
none were taken here.

## The fields

Each GPU field is a 3x3 mean times a keep every engine step (30 a second at
scale 2); D = 1/3 cell² a step, so a keep k gives half-life ln 2 / (−ln k)
/ 30 s and spread length sqrt(D / −ln k) fine cells:

| Field | Keep | Half-life | Spread (fine cells) |
|---|---|---|---|
| Trail (plain) | 0.9 | 0.22 s | 1.8 |
| Trail (Drifter) | 0.93 | 0.32 s | 2.1 |
| Trail (Sprawler) | 0.8 | 0.10 s | 1.2 |
| Trail (Trunk) | 0.95 | 0.45 s | 2.5 |
| Food smell | 0.97 | 0.76 s | 3.3 |
| Alarm (alarm tiers) | 0.98 | 1.14 s | 4.1 |
| Scar (Grudge) | 0.9999 | 3.9 min | none (per cell) |

The food smell is read 36 fine cells ahead (`scent_offset` 18 x scale).
The walking-distance maps (far smell every 2 s; lure, hunt, wave, raid,
units every 1 s) are CPU Dijkstra on 2x2 blocks; a walker leans by weight x
(distance here − distance at its sensor), at most weight x reach.

## Findings

1. **Bug: trails on clay grow without limit when keep ≥ 1/1.05 = 0.952.**
   `trail_out = mean × keep × soil_keep` (`engine.wgsl` 2624 and 2644) with
   `clay_keep` 1.05 (`big.rs` 1704). Trunk 0.95 runs at 0.9975 on clay.
   The viewer's keep slider reaches 0.999. Planet 1 with `keep=0.99`:
   trail max 1.5e7 at 0:10, 5.6e14 at 0:25, 2.1e22 at 0:40, 80,941 cells
   over 1e4, the whole planet drawn as slime (`images/clay-blowup.jpg`).
   Fix: `min(keep × soil_keep, 0.998)` in both places; no shipped tier
   changes. Sent to the Enemy lane.
2. **One brief smell does every job.** Crystals, soldiers, lures, pumps,
   the larder, outposts and Bait share one field at keep 0.97. A settled
   crystal (radius 12, laying 20): 599 at the centre, 265 at the edge, 0.73
   at 30, 2.5e-3 at 48, 5.8e-5 at 60, 5e-13 at 120 fine cells. The squad is
   the loudest source: peaks 287 (planet 1, 17 soldiers, 1:00), 4,136
   (planet 2, 14 soldiers, 1:00), 5,253 (planet 2, 23 soldiers, 2:00);
   crystals 540-870 untouched, 85-96 touched.
3. **The far smell is already in the smell's exponent** (heat method;
   Varadhan 1967, Crane et al. 2013). Spearman of −ln(smell) against the
   far smell's walking distance per 2x2 block: planet 1 0.93, 0.87, 0.90
   (0:30, 1:00, 1:30); planet 2 0.79, 0.77, 0.69 (0:30, 1:00, 2:00).
   Off the body, ln-smell uphill within 45° of the far smell's downhill:
   planet 1 86%, 83%, 90%; planet 2 91%, 92%, 82%. Fitted slope on planet 1
   0.25-0.27 per fine cell (theory 0.30). Smallest smell 4e-20 (planet 1),
   6e-18 (planet 2); no subnormals. F2 shows smell ≥ 0.24: 24-29% of open
   cells on planet 1, 30-36% on planet 2.
4. **The walking-distance maps steer hardest and are unseen.** `LURE_PULL`
   10 leans about 60 on planet 2 (reach 6) and 180 on planets 1 and 4
   (reach 18); the hunt about 60 on planet 2 and 340 on planet 4. The hunt
   map covers 21% of planet 2 at 0:30 and 1:00, 60% at 2:00 (a disc 80
   dish cells round the squad's path, not a line). Shortest paths share
   gaps, so a hunt or wave arrives on one front (the Lab's hunt report).
5. **Cost.** On the 1070, planet 8, `trail` 1.155 ms and `scent` 0.823 ms
   of a 5.269 ms step (37%). Each cell re-reads each neighbour's deposit
   counters: about 63-81 reads a cell in `trail` (many atomic), about 45
   plus 12 atomic clears in `scent`. Fold-then-blur (a per-cell pass adds a
   cell's own deposit and clears its counters, then the blur reads 9 plain
   floats) keeps the same float operations in the same order; estimate
   about 1 ms a step saved on planet 8, untimed.
6. **Otherwise sound**: averages times keep < 1 fade (clay aside), edges
   absorb, walls average over open neighbours. A touched crystal keeps its
   15% smell for the rest of the planet (`reached` is never reset).
7. **Readability**: only the trail (the body) and the lure rings are on
   screen.

## Ideas, ranked

1. Cap clay's keep (Enemy, one line).
2. Fold, then blur (Enemy with Engine and tools; 1070 timing).
3. Flanking: the hunt, wave and units maps cost more under gun reach or
   recent deaths, so the slime routes round the kill zone (Enemy; Lab
   prototype first). Recommended first prototype.
4. Draw what the slime follows: hunt spoor, wave telegraph, alarm ripple
   (Look and interface).
5. Smell channels (food, prey, bait) with their own keeps, and a palate per
   tier; a long prey channel the player can wash, burn, wipe, mask or fake.
6. Read −ln(food smell) as the far smell, retiring the CPU map (after 5).
7. Smell on the wind (Lab first).
8. Reset a crystal's touched flag after time with no slime on it.
9. Walking-distance maps on the GPU by relaxation.
10. From the model: veins by flow (Tero et al. 2010), avoiding its own old
    slime (Reid et al. 2012), noise from shots, a squad stealth meter.

Scott chooses which to prototype first; nothing goes into the game without
his yes.
