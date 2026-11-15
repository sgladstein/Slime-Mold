# Resume a half-built building with one click (2026-11-15)

Owner, 07:50Z: "If you start building something but have to stop you should
just need to click again to get the squad to resume building".

Page: https://claude.ai/artifact/RtqbiucG1oQVpTm3MgFzZm (`index.html` here).

## What changed

- **`Game::unfinished_at(p)`** (`buildings.rs`): the unfinished building a
  click means, a ghost under it (`ghost_at`) or the pump rig still going up
  on the crystal under it (a rig's crystal is wider than a ghost's
  footprint).
- **`App::resume`** (`input.rs`): sends builders back to it with
  `Order::Build`. No new placement, so nothing is paid again (every price
  leaves the bank at placement since #322). In squad mode the controlled
  squad goes (`Cmd::SquadOrder`, Shift `Cmd::SquadQueue`); in the RTS round
  the selected soldiers (Shift queues), or with none the nearest free one
  (`Cmd::Send`). An alert says "Back to the turret (40% built)".
- **Where it is reached:** a right click on the half-built building (squad
  mode and RTS round; Ctrl + right click on a pump rig's crystal still
  drains it), and the same build card's click on it again (T then a click on
  the turret ghost, P on a crystal whose pump is still a rig: before, that
  said "Can't build there" or "The pump is still being built"). The squad
  mode's help lists the right click.
- No new command id: it is the existing `Order::Build`, so replays carry it.
  Nothing touches healing or the squad-order code the squad thread owns.

## Checks

- `a_click_on_a_half_built_building_sends_the_squad_back_to_it` (window
  test, squad mode): a turret raised to 20%+ and left stops rising; one
  right click on it sets the squad's work to it, every member on
  `Order::Build`, bank and building count unchanged, and it stands; the
  turret card's click on a second half-built turret does the same.
- Full `cargo test --release --locked`: 483 + 39 + 35 passed, 0 failed;
  clippy clean.
- Frame cost: none (one ghost search per click).

## The clip

`resume-build.gif`, Fork, squad mode, zoom 6, every second frame of 0:00-0:14:
a turret placed (bank 2000 -> 1505), raised to 40%, the squad sent away and
the ghost left; one right click on it; the squad walks back and finishes it.
Bank before the click 1522, after the turret stands 1539 (income only,
nothing paid again); one building throughout. Frames from the ignored test
in `raw/film.rs` on llvmpipe.
