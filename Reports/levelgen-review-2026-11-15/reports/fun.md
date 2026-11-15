# Fun review: procedural planets (4494ad4f)

Saved by the levelgen thread from the reviewer's hand-back (the harness
blocked the reviewer from writing it). Pictures, data and scripts are in
`/home/user/review-fun/` (`pics/`, `maps/*.tsv`, `runs/`, `frames/`, `tools/`).

**What I ran**
- 64 planets (seeds 1-8, depths 1-8), drawn in false colour.
- Map measures on 4-cell bins (`tools/*.py`):
  - walks;
  - the widest route to each nest's gun range;
  - the least wall between home and the nests, for soldiers and for the slime;
  - the "direct way": the widest route at most 15% longer than the shortest;
  - sight lines;
  - crystal energy along the route.
- The same measures on the six designed maps, as a control.
- Play:
  - the rush hand on planet 1 seed 1 (1:1) for 20 s;
  - take_claim for 150 s at 1x area on meadow seeds 1 and 2 and on seed 1 of valley, highlands and caves;
  - frames of 4:1.

Widths are in dish cells; gun range is 24, and bin rounding reads widths 1-2 cells low. **[M]** = measured, **[J]** = judgement, **[C]** = read in the code.

## Big picture
1. **The approach suits looks and reachability, not play.** Regions plus a spanning tree are a sound skeleton, but `check` asks only "can you reach it?". The planets come out as two kinds of play, not seven [M]:
   - **Open:** planets 1-3 and most jungles and cores. A route at least 16 wide reaches every nest, but holding a line takes 40-110 cells of wall (800-2,200 food against a 2,000 start).
   - **Funnels:** planets 4, 6 and 7. Every route to every nest passes a gap 4-14 wide.

   Only **4 of 112 nests** have what the owner asked for: a narrow direct way (at most 10 wide) and a wide way round (at least 16). The designed maps are each built for one plan too. Generate a front door and a back road for each nest on purpose, and score them.
2. **What the tests missed.**
   - In the report, no planet was played past 2 minutes by any hand.
   - Nothing checks that every strategy can win.
   - Nobody measured whether the late planets are mazes. The walk is 0.99-1.27x the straight line (median), and 1.51x at worst [M].
   - The game cap is 900 s on every planet (`rules.rs:490`; the run changes only `drain`, `run.rs:417`) [C]. The owner's "15/20 minutes to 1 hour+" isn't wired up.
3. **From outside.**
   - Age of Empires II gives each map an identity and a start built for that map. Here the first crystal is 12-26 cells from the landing on 64 of 64 planets [M].
   - Togelius et al. (2010), on generating StarCraft maps, use choke width and the paths between bases as goals, not pass/fail gates. An expressive-range plot (widest route against least wall) would show the empty corner where the owner's maps should be.
   - In They Are Billions, natural chokes make walls cheap.
   - Against the Storm lets you choose the next settlement by its modifiers, and Noita's Holy Mountain is a breather between biomes. The run has neither.

## Every feature
- **Open ground, outcrops, pillars:** open, with no choke. The 6x pillar cores' widest routes are still only 6-8 [M].
- **Ridges and passes:** the real chokes. The least wall is 5-15 on 7 of 8 valleys [M].
- **Rocky rooms:** widest route 4-14, least wall 6-28. On 6:6, soft rock shuts both nests [M].
- **Caves:** every route is 4-6 wide on 8 of 8 seeds, and mean sight is 15-17, under the gun range of 24 [M]. Range upgrades are wasted there [J].
- **Core arenas:** the nests' rings still funnel them. Only 3 of 8 cores give every nest a route at least 16 wide [M].
- **Pools:** shallow, so soldiers walk through them. 6 of 8 fens have only pools, so for soldiers they play as meadows [M].
- **Lakes and rivers:** the only water that bends a route. The slime crosses deep water at 0.2 speed (`big.rs:1285`), and on 4:1 it was on the squad's bank by 150 s. A moat that stops you, not it [J].
- **Fords, bridges, `cut_fords`:** the ford is the valley's widest route (6-14), and bridges are 3-4 cells wide. Core 8:5 has four parallel rivers and a widest route of 4: the same obstacle four times [M].
- **Streams:** 5-10 of them tangle a 6x map (8:2) without shaping any route [C/J].
- **Soils:** they never change a walk [C], but they decide when the slime arrives. Meadow seed 1 (sand) met the slime early: 530 killed, 1,500 harvested. Seed 2 (clay and pools) never met it: 6,600 harvested [M, one run each].
- **Soft rock:** a jungle has 216-250 pieces, but walking round them costs at most 1.43x the walk (median 1.04) [M]. The gaps in the nests' rings make natural kill boxes [J].
- **Plants:** fire fuel only, with no cover and no food [C].
- **Crystals:**
  - From planet 1 to planet 8, energy per 1x area falls from 4 to 1, and the crystal count from 18 to 9 [M].
  - 10% of the energy sits in the home third, which pulls the squad out [J, good].
- **Nests:** the 280-cell spacing is absolute (`levelgen.rs:833`). At 6x the nearest pair is 0.14-0.19 of the map's width on 5 of 8 seeds, and on 8:1 all three nests share 7% of the map [M].
- **Size:** the walk to the far nest is 14 s on planet 1 and 60 s on the core [M]. The slog is not the walk. It is the fixed 40-soldier cap, the 15-minute clock and the thinner crystals [J].
- **Fallback map:** not examined.

## Variety
The score is the distance between two biomes over 10 play features, divided by the spread between seeds of one biome; 1.0 means no further apart than two seeds of one biome [M; the features are my choice].
- **Most alike:** barrens and fen, 1.01; fen and valley, 1.11; meadow and barrens, 1.13.
- **Barrens, fen and meadow** differ only on the slime's side.
- **The core** sits near highlands (1.06) and jungle (1.11), so the climax is a remix.
- **Caves** stand apart from every biome (1.96-2.40).
- **Seed to seed,** jungle and caves vary least; core and fen vary most.

## Progression
Each planet: area, nests, the run's slime as a multiple of planet 1's, walk to the far nest, then difficulty / novelty [J].

| Planet | Area | Nests | Slime (x planet 1) | Walk to far nest | Difficulty / novelty |
|---|---|---|---|---|---|
| 1 meadow | 0.6 | 1 | 1.0 | 14 s | low / high |
| 2 barrens | 0.8 | 1 | 1.6 | 18 s | low / low |
| 3 fen | 1.0 | 1 | 2.3 | 21 s | low / low |
| 4 valley | 1.4 | 2 | 3.7 | 27 s | **spike** / high |
| 5 jungle | 2.0 | 2 | 6.0 | 32 s | up / mid |
| 6 highlands | 2.8 | 2 | 9.3 | 40 s | up / low |
| 7 caves | 4.0 | 2 | 14.7 | 56 s | up / high |
| 8 core | 6.0 | 3 | 24 | 60 s | up / low |

The slime grows about 1.6x every planet, with no breather. Players are likely bored on 2-3, swamped on 4 (twice the nests and the first funnel) and slogging on 7. The climax is new only in its scale.

## Every strategy can win
- **Gun builds** need an open approach. Planets 4, 6 and 7 have none on any of 8 seeds, and the core lacks one on 5 of 8 [M].
- **Turtle builds** need a cheap choke. Planets 1-3 have none, so walling a line costs about 4-8x more than on a funnel planet [M].
- **Snipers** lose the caves.
- **Artillery and jumping** aren't built yet, so they can't be tested.

## Play
- **Rush hand on 1:1:** no soldiers left by 20 s, after 707 kills. That comes from the slime's tuning, not the map.
- **take_claim at 150 s.** I named harvest as the deciding measure before the runs.

  | Planet | Harvest | Soldiers left | Kills |
  |---|---|---|---|
  | meadow seed 1 | 1,500 | 11 | 530 |
  | meadow seed 2 | 6,600 | 24 | 0 |
  | valley | 5,100 | 18 | 1,411 |
  | highlands | 3,900 | 17 | 741 |
  | caves | 2,700 | 4 | 3,049 |

  The two meadow seeds differ more than the biomes do, so one run each can't tell the biomes apart. The caves' result is confounded: 25k walkers on 41% open ground, a density the run's `open_scale` avoids.

## Questions only the owner's play can answer
1. Is a moat that stops only you fun or unfair?
2. Do planets 2-3 feel like the tutorial again?
3. Is 15 minutes enough at 6x, and is a minute's walk to the front fine?
4. Are the 4-wide cave tunnels tense or tedious?
5. Do the jungle's hedges read as a maze or as clutter?
6. Is the same opening on every planet a comfort or sameness?

## Findings, most serious first
1. **CONFIRMED: almost no nest has both an open way and a choke** (4 of 112).
   - Fix: make `check` require one of each per nest, measured as in `tools/localchoke.py`. On funnel planets, widen one tunnel or pass to 16. On open planets, add a ridge with a pass across the direct way.
   - First experiment: generate 50 seeds per biome and count the nests that pass.
2. **CONFIRMED: the clock is 900 s at every planet size.** Fix: scale `cap` and `TWO_STARS` by area in the run.
3. **CONFIRMED: the core's nests cluster,** because the 280-cell spacing doesn't scale with the map. Fix: space nests 0.3 of the map's width apart.
4. **CONFIRMED: every planet opens the same way.** Fix: set the landing crystal's distance and size per biome.
5. **CONFIRMED: fens with only pools play as meadows** (6 of 8). Fix: weight lakes 3:1 over pools.
6. **CONFIRMED: crystal density halves with depth, and this is documented nowhere.** Fix: scale `big` crystals with area, or name it as a difficulty dial.
7. **SUSPECTED: the caves are a slog at 4x.** Fix: tunnels 5-15 wide and a cavern arena or two.
8. **SUSPECTED: plants matter only when something burns.** Fix: give cover a play effect (the plants thread's call).
9. **SUSPECTED: there is no breather on planets 4-8.** Fix: a smaller "rich" twist planet at planet 6.
