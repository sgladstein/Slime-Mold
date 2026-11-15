# Menu and HUD fixes from the three-reviewer review (2026-11-15)

The first step of the [menu review](../menu-review-2026-11-15/README.md)'s
order: small fixes, each a bug a reviewer found or a place the screen said
something untrue. It also carries two asks from the whole-game review
(coordinator, 11-15 11:57Z, agreed): Run first on the start menu, and the
walkers now and the goal beside the run's 2% bar. Before/after page:
`page/index.html` (Artifact link in `Reports/pages.md`).

## What changed

| Where | Before | After |
|---|---|---|
| Hover card over the world | a click put away only the card already up; aim sliding onto the home crystal mid-volley raised a new one 0.35 s later; near the right edge or the bar it slid under the cursor | no card while a mouse button is held or WASD steers, and what the aim reaches meanwhile stays shut until the cursor is on something else; it goes to the cursor's other side instead of under it |
| Squad header keys | with the build page open it showed Split Y, Combine J, Guards I, Heal L, but the card takes those letters first (Siren, lures on/off, Outpost, Lure) | a key letter shows only where no command on the card takes it |
| Squad header line | "commanding Squad 1, 4 soldiers", which the YOU tile shows, cut to "commanding..." | gone; with none commanded, "none commanded: all on their own" only where it fits whole |
| Squad tile | an AUTO squad's state read "on its ..." (the note's first word) | its activity: firing, healing, working, moving, holding (`SquadRow::state`) |
| Gun strip | "Lance / A card a shot", "Plain / Plain shots"; the gun only an icon; in a run an H button that only raised a refusal | the gun's name first ("Rifle"), then the loadout or "Plain shots"; no H in a run, and its tooltip says the shop changes the gun |
| Build page in a run | a locked building's price in red, like "can't afford" (Turret 500 red with 500 banked) | a lock and "Locked" in grey (`Command::locked`); red only for want of food |
| Options | "UI and text size" was the 15th of 15 rows; a click anywhere on a row reset it, so one stray click took 200% back to 100%; All to defaults reset the UI size too; "default" printed under every value | UI size first; a click on a row takes the focus (the arrows turn it, Enter turns it on); under a changed value, "back to X" resets it; All to defaults keeps the UI size; nothing printed under a default value |
| Start menu | Sandbox first and lit; its line listed the UI size as a rule ("Free play, with ui 200%") | Run first and lit (Enter starts a run); Sandbox's line lists changed rules by their labels and leaves the UI size out |
| Run goal | "Wipe out the slime: cut it to 2% of its peak (0%)", "(0%)" for the first minute with no reason | "(counting from 1:00; 3,088 walkers now)" before the peak counts, then "(0%: 12,630 walkers now → 252)" |

## Checked

New tests: `the_hover_card_waits_fades_and_keeps_out_of_a_volley` (rest,
show, fade, shut; held button; the key reached mid-volley stays shut),
`the_squad_header_shows_no_key_the_card_takes` (Split and Heal lose their
keys with Y and L on the card, Combine keeps J), `all_to_defaults_keeps_the_ui_size`,
the options test (a row click focuses, "back to" resets and goes), the
keyboard test (Run first), and the shrink goal's line in
`take_stays_empty_for_idle` (before 1:00 and after, with thousands set
off). `turning_the_map_on_the_menu_switches_the_dish_behind` walks to the
map row by its key now: with the UI size first it turned the UI size and
saved it to the settings file. `cargo test --release --locked`: 493 + 35 +
39 pass; clippy clean.

Frames (`larder-world look=DIR screen=1920x1080`, llvmpipe), before = main
with #334 (d5af42ab), after = this change:

- `ui=100% run=1 mode=squads select=1 at=70` and `ui=200%` (the bar and
  the goal in a run; the squad was wiped by 70 s, so no tiles)
- `ui=100% mode=squads select=1 hand=squads rounds=lance at=30` (tiles,
  header and the gun strip with cards)
- `ui=100% run=1 hand=buildq hover=900,500 at=4` (the build page in a run)
- `ui=100% menu=main at=1`, `ui=150% menu=options at=1`

Pictures by `raw/make_pics.py`. Not in a picture: the hover card (it runs
on real time) and the key letters with the build page open in squad mode
(`hand=buildq` does not open it there); the tests cover both.

Frame cost: the model and HUD took 0.07 to 0.25 ms a frame in both builds
(one frame each, on a machine busy with the test suite, so no difference
can be read from them). The change adds a list of the card's key names
and a compare per header button, once a frame.

## Not changed (later steps of the review)

The bottom row still shrinks the gun strip to 0.6 at 200% (step 2, one
allocator for the row); no star conditions on the HUD yet (step 3); the
shop's slots and cards still apart at 150% and up (step 4).
