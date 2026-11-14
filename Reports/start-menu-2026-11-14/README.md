# The Larder World start menu (2026-11-14)

Cloud plan item C2 (`Reports/cloud-plan-2026-11-14.md`): the native
window opens on a start menu, with sandbox, the campaign's missions, an
options page for today's switches with their defaults, and a key card on
F1 or ?. Built while the owner could not playtest and the 1070 was busy,
so it was checked on the cloud's software Vulkan (llvmpipe).

Page: https://claude.ai/artifact/C6wF9cqX9EVFPQLnUAyVhS (source `page/`).

## What it does

- `larder-world` with **no arguments**, or with `menu=1`, opens on the menu.
  A live sandbox round plays behind it: the round Sandbox would start
  (shape C on Fork today), with the camera on the whole map and no HUD. If
  the round ends under the menu, a fresh one starts. Any other command line
  starts its round at once, as before. `menu=0` never shows the menu.
- **Sandbox** plays the options as set. **Campaign** lists every file in
  `Reports/larder-campaign-2026-11-12/missions/` by its number, with the
  mission's name, goal, map and mould, and starts it as `mission=STEM`
  does. **Edit maps** starts `larder-edit` (#209) beside the binary, or
  says how to build it. **Keys** is the key card.
- **Options**: sandbox map, own mould, rounds, bullet speed, panic, wither,
  shared food (`energy_flow`). Each row shows its default; a value off the
  default is amber. Panic, wither and shared food default to shape C's
  values (`shape_keys("c")`), bullet speed to `Rules::default()`, rounds to
  plain. Values given on the command line with `menu=1` are the presets.
  An option at its default adds nothing to the round, so a mission keeps
  its own mould unless one is moved. Rounds lists only loadouts the GPU
  world can cast (no germ yet, as H skips them; gas since #211).
- **In play**: F1 or ? opens the HUD's controls page and pauses; again, or
  Esc, goes back to play. The Esc menu and the end screen have **Main
  menu** (`MenuItem::MainMenu`), which saves the replay and returns to the
  menu, on the options as they were left, over a fresh dish.

## The dish follows the map (11-14, after the owner's playtest)

The owner, 05:04Z: "when you switch the map in the menu it should switch in
the background". Turning the sandbox map, or your own mould, on the options
page now starts the dish behind the menu again on that world
(`DISH_OPTS` in `menu.rs`; a game-mode option goes in the same list). Other
options, and moving between rows, leave the dish playing. Test:
`turning_the_map_on_the_menu_switches_the_dish_behind` (llvmpipe).

## Checks

- `world::start` tests: keyboard walk, options turn and reset, F1/? from
  every page, clicks on every page at 640x400, 1280x800, 1920x1080 and
  3840x2160, Edit maps only when offered. HUD tests: F1/? from play and
  from the paused menu, Main menu only when the game has one.
- `the_menu_plays_the_command_lines_rounds`: the menu's untouched Sandbox
  gives the bare command line's round (same map, walkers, keys, loadout,
  bullet); a mission gets no shape C keys; moved options land as the
  command line's keys would; a command-line value is the preset and a
  reset puts C's own value back.
- Full `cargo test --release --locked` with llvmpipe present: 358 passed,
  0 failed. Clippy clean.
- Pictures: `larder-world look=DIR menu=main|campaign|options|keys
  hand=idle at=20 screen=1600x900` (options with `panic=0 rounds=split`).
  At 20 s, mould 24,555 walkers, 4 soldiers, 681 killed by 75 shots.

## Not checked

- Clicking through the live window (no screen in the cloud).
- The 1070's frame cost. The menu draws on the HUD's canvas instead of
  the HUD, only while it is up.
