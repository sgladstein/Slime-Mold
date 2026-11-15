# Letting go of the squad: no squad controlled, WASD pans (2026-11-15)

Owner, 02:58Z: "There should be a way to deselect so that you're not actually
controlling any specific squad in there and then you can move around the map
using wasd and it's easier to confirm that the squad is now running on its
own automated protocol".

Page: https://claude.ai/artifact/W49VVdkXFWNjMpuCv6f6w8 (`index.html` here,
with `free-camera.mp4` and two stills).

## What changed

- **Esc lets go** in squad mode (`App::let_go`, `SquadInput::free`), and so
  does a new **Let go** button on the squad panel's header
  (`HudEvent::SquadFree`). Esc keeps its old order of duties: a building
  being placed is cancelled first, then Esc lets go, then the next Esc opens
  the menu. Picked over a click on empty ground because a left click is the
  squad's volley, and over a new letter so no card key moves. No command id
  was needed: the button is the panel's, as Split and Combine are.
- **The sim**: `Cmd::Active(u8::MAX)` (already parsed and logged, so it
  replays) now stops the squad let go, and its link, steering and volleying,
  as a switch to another squad does; its sent order and Shift queue stay.
  A soldier bought with none controlled joins the lowest live squad
  (before it would have started at the larder, which squad mode has none of).
- **While none is controlled** nothing is selected, the camera stops
  following, and WASD pans it unpaused too (`Controls::wasd_pan`). The frame
  no longer takes a squad back on its own; only when the controlled squad
  dies does it pick the next one, as before.
- **Taking command again**: a click on a squad (nearest soldier within 3
  cells or 14 px), its tile, its number, Shift + box, or Space (the last
  squad controlled). A right click with none controlled gives no order and
  says how to pick one.
- **Showing it**: each squad tile now says AUTO (green) unless it is the one
  you command (YOU) or linked with it; tiles are 86 px wide (were 72) to fit
  it. Over each squad on the map the number now reads "N auto" for those,
  and the label sits above the squad's topmost soldier (the longer label was
  hidden under the sprites at its old height). The panel's header reads
  "none commanded: all on their own, WASD pans", and an alert says how to
  take command again.

## Checks

- `letting_go_stops_the_steer_keeps_orders_and_a_bought_soldier_joins_a_squad`
  (sim) and `esc_lets_go_of_the_squad_and_wasd_pans_until_a_squad_is_taken_again`
  (window, llvmpipe): Esc lets go, the frame keeps none, WASD pans 30 frames
  right with no squad steering, a second Esc passes to the HUD, Space, a
  tile and a click each take command.
- The panel UI test now presses Let go and checks it greys out with none
  commanded.
- Full `cargo test --release --locked`: 540 passed, 0 failed; clippy clean.
- Frame cost: one more label string per squad and a scan of its members for
  the top; not measurable.

The clip comes from the ignored test in `raw/film.rs` (append it to
`src/bin/larder-world/tests.rs`, run with `FILM=DIR cargo test --release
--bin larder-world -- --ignored film_free_camera`), rendered on llvmpipe.
