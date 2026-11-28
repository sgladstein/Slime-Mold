# The squad's planned path (2026-11-28)

The owner asked (07:09Z): "Could we improve the visual of the squads planned
path? What are our options?"

**Before:** `order_chain` (`src/bin/larder-world/draw.rs`) drew a one-pixel
line, dashed every 2 px, straight from the squad's middle to each order,
with a ring at each goal (red for fighting). It crossed water and rock the
squad walked round, lost itself on bright slime, and was thinner still at
4K and 250%.

**Options** (prototyped in the game, planet 4 of run seed 1, a squad of 9
with two queued orders, zoom 3 and 6; `index.html`, published at
https://claude.ai/artifact/Q7A9QPpLpQNBTSpyCFVMrX):

- A, the real route as a solid cream line on a dark pen (recommended: quiet
  when many squads are linked).
- B, chevrons drifting along the real route toward the goal.
- C, dots one walking second apart, so their count is the trip's time.

**Picked:** B (owner's card, 07:29Z). Built as `order_path` in `draw.rs` over
`Game::route` (`paths.rs`), which reads down the walking map the squad
already walks by; a queued leg uses the map built for its goal if one is
cached, else the straight line. The chevrons are wide `Line` marks
(`Marks::stroke`, `extra` = pen width in `view.wgsl`), 4 marks each, at most
60 a leg. The RTS mode's per-soldier line is unchanged.

**Prototype gotcha:** drawing the line as pixel discs ran past `MAX_MARKS`
(4,096) on a long path and the soldiers, drawn later, were dropped.

Film: `larder-world look=DIR run=1:4 mode=squads hand=movetour,10,queue
centre=squad,6 at=6,6.2,6.4` (`queue` makes the tour Shift-queue a move and
an attack-move after each move).
