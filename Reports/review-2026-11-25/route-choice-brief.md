# Build brief draft: a star map with forks and contracts

Step C of the owner's order (11-25 21:35Z: "i really like the idea of c and
I think there are a lot of different directions that we could take it").
The five directions are on the [route-choice page](route-choice.html)
(https://claude.ai/artifact/85hVGbUEAL2pBYA4xovKts). This brief covers the
recommended mix, directions 1 and 4, in case he picks it. It is a draft until
he does.

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

## How it fits today's code

- A planet is already fully set by `(run seed, depth)` through `Run::level`
  and `levelgen::level`. A node is `(depth, k)` with `k` mixed into the seed
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
