# Build brief draft: a star map with forks and contracts

Step C of the owner's order (11-25 21:35Z: "i really like the idea of c and
I think there are a lot of different directions that we could take it").
The five directions are on the [route-choice page](route-choice.html)
(https://claude.ai/artifact/85hVGbUEAL2pBYA4xovKts). This brief covers directions 1 and 4,
**the owner's pick (11-25 23:45Z)**: "I choose 1 + 4 and each of those
planet options should have clearly different difficulties with greater
rewards on the harder planets. Keep 2 and 5 as potential future directions
(recorded in the repo)." Directions 2 and 5 are kept in
[future-route-directions.md](future-route-directions.md).

## What the player sees

- After landing and after each shop, a **star map**: about 12 steps from
  the landing world to the home world, with two or three planets to choose
  from at each step and paths that cross. Each planet node shows its slime
  species and landing-card line, its biome, any twist, its goal (the heart,
  or the rescue), its reward and its contracts.
- **Contracts** (1 or 2 per planet, optional): rescue the dug-in squad,
  lose at most N soldiers, take a crystal without losing one, break the
  mini-boss, or win under par. Each pays points or a rare card. **Heat** is a
  toggle on the node: turn on a twist, a bigger heart or a sooner first raid
  for a reward multiplier.
- The route taken is drawn on the Past runs page.

## Each fork is a clear difficulty, paid by its reward

His rule: the planets offered at one step differ clearly in difficulty, and
the harder one pays more. A step offers up to three, each marked on its node
and on the landing card with one to three pips:

| Pip | Difficulty | What makes it so | Reward |
|---|---|---|---|
| 1, Safe | the curve a step **below** this depth (`curve_depth` - 1) | smaller slime and heart, no twist, fewer late threats | points x1 |
| 2, Fair | the curve **at** this depth | as today | points x1.5 |
| 3, Hard | the curve **1.5 steps above** this depth | bigger slime and heart, a twist always, and the harder slime species of its band | points x2.25 and a rare card or gun offered in the next shop |

- The difficulty comes from the tuned per-depth tables read at a shifted
  curve depth, so it is one dial a node, not new numbers. The slime species
  can differ between nodes as well: the pool for a band offers the easier
  and harder species of that part of the run.
- The reward is shown as the exact points before the pick, and the shop's
  rare offers say which node paid for them.
- **Check that the difficulty is clear**: the run bot plays all three nodes
  of a step on paired seeds (steps 2, 6 and 10, six seeds). Hard must cost
  more than Fair, and Fair more than Safe, in soldiers lost and time to win
  on at least 5 of 6 seeds each. If it doesn't, widen the offsets. This is
  part of exit test 3 (the curve), not a new census.
- Contracts (direction 4) stay optional on top of the pip: a Safe planet
  with a hard contract is a fair way to catch up.

## How it fits today's code

- A planet is already fully set by `(run seed, depth)` through `Run::level`
  and `levelgen::level`. A node is `(depth, k, pip)` with `k` mixed into the seed and `pip` shifting `curve_depth`
  (the rule `Run::level` uses today). The tier comes from a **pool for the
  depth band** in place of `LADDER[depth-1]`, so the eight species can each
  appear in more than one place and the curve stays by depth
  (`curve_depth`).
- `Run` gains `route: Vec<u8>` (the `k` picked at each depth). It goes in
  `save_text`, so saved runs keep their map. An old save without it reads
  as `k = 0` everywhere, which is today's run.
- Contracts extend `campaign::Bonus` (which has `LoseAtMost` today) and
  pay through `Run::found`/`won`, as starfall does.
- Length: `PLANETS` 12 with `curve_depth` spreading the tuned eight-planet
  tables (the dials from #749). At today's 4 to 9 minutes a planet plus
  the shop, that is about 1.5 to 2 hours. `STRETCH` stays 1 until he plays it.
- The map screen is Look and interface's. The pools, contracts and save
  belong to Engine and tools and Direction. Enemy checks the pools keep
  later planets costlier.

## Checks

- `the_length_dials_change_nothing_at_eight_planets` keeps holding with
  `route` all zeros.
- The run bot picks the first node and plays 12. No soft lock, and the
  curve page shows time, soldiers and losses by step.
- A smoke check that every node a map offers can be generated and landed
  on (no empty biome-and-tier pairs).

## Stage two (direction 2)

The star map's edges become a slime network that grows between landings,
along the shortest routes between planets, and feeds the home world. It is
built once the map exists. It gets its own brief from the Lab.
