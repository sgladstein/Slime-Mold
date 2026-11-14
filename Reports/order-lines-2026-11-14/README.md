# Squad order lines and WASD panning while paused (2026-11-14)

Owner's playtest, 20:12Z: "when clicking a move action, there should be a
line showing where the selected squad is moving"; "when paused wasd should
let me move around the map, similar to the arrow keys". 20:15Z: "same with
attack move and other commands. the action queue should also be visible. if
I set up three moves, I should see the full path".

Page: https://claude.ai/artifact/4bbZ1NXACyVKnbXfxgkuXR (`index.html` here).

## What changed

- **Order lines** (`order_chain` in `src/bin/larder-world/draw.rs`). In
  squad mode the controlled squad and the squads linked to it each draw one
  chain: from the squad's middle to its current order (`Squad::dest`, else
  `Squad::work`, else a soldier's attack or burn), then through each
  Shift-queued order (`Squad::queue`, up to 6), each leg ending in a ring.
  Red for attack-move and the core, white else. Before, a squad sent
  somewhere drew nothing (its soldiers hold no order while the squad moves)
  and a drain drew one line per soldier. The classic mode keeps its
  per-soldier lines through the same function. A first leg under one dish
  cell (a squad settling where it was let go) is not drawn.
- **WASD pans while paused** (`Controls::wasd_pan`, set by the window from
  `App::paused`). Unpaused nothing changes: WASD steers the squad, and in
  the classic mode A, S, W, D stay the card's attack-move, stop, sow and
  mine. A key let go while paused still lets go of a squad steer held from
  before the pause; a key held through unpausing stops panning.

## Checks

- `a_squads_orders_and_queue_draw_as_one_path`: a move and two queued
  orders give 3 legs, the attack-move leg red, and 1 line to the first point
  from a 4-soldier squad.
- `wasd_pans_only_while_set_and_lets_go_when_cleared` (controls unit test).
- Full `cargo test --release --locked`: 511 passed, 0 failed; clippy clean.
- Frame cost: at most 7 line marks a frame for the chain; not measurable.

The clip comes from the ignored test in `raw/film.rs` (append it to
`src/bin/larder-world/tests.rs`, run with `FILM=DIR cargo test --release
--bin larder-world -- --ignored film_order_lines`), rendered on llvmpipe.
