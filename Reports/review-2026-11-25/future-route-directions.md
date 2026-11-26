# Route choice: directions kept for later

The owner's words (11-25 23:45Z): "Keep 2 and 5 as potential future
directions (recorded in the repo)." He picked directions 1 and 4 (the star
map with contracts, [brief](route-choice-brief.md)). These two are not
built. Each one needs the star map first. The page with all five is
[route-choice.html](route-choice.html).

## Direction 2: the slime spreads between planets

The star map runs its own slime simulation at the scale of the run. Between
landings the mould grows tubes along the shortest routes between planets,
the network-finding real Physarum is known for. Planets on a thick tube start
bigger and better fed. Freeing a planet cuts its tubes, so planets the
player skips keep growing. The home world is as strong as the network still
feeding it, so the route is a cut through a living network: the mould is
the game at the run's scale too.

- Needs: the star map (direction 1), a small network simulation over its
  nodes and edges between planets, a map screen that draws the tubes, and
  planet sizes read from the network.
- Estimate: 3 to 4 thread-days on top of the map. The Lab prototypes it first.

## Direction 5: planets remember you

A planet left after a goal short of a full clear (a rescue, a drained heart
crystal) stays on the map with its slime regrowing from what was left. The
player can go back for a bigger reward, or the planet sends raids toward the
home world. Save and resume carries the planet's state forward.

- Needs: the star map, a saved per-planet state (save and resume's format,
  #757), a return path on the map, and a rule for regrowth while away.
- Estimate: 2 to 3 thread-days on top of the map.
