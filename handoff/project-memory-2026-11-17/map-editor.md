---
name: map-editor
description: Larder World map editor (WM5) merged 2026-11-14 as PR #209: larder-edit binary, mould::map_edit; open follow-ups
metadata:
  type: project
  modified: 2026-11-14T03:43:50.355Z
---

Built 2026-11-14 from plan item C10 (Reports/cloud-plan-2026-11-14.md), merged PR #209 (fb3de33).

- `cargo run --release --bin larder-edit -- map=fork name=NAME`: its own window, not a mode of larder-world, so it never shares an entry point with the start menu (menu.rs, C2). Saves the WM1 files to `maps/NAME` by default (designed maps in Reports/larder-maps-2026-11-11/maps are only read). Enter saves and spawns `larder-world map=DIR/NAME` beside it.
- Editing in `src/mould/map_edit.rs` (Editor, Brush, Tool, Mirror, draw), 9 tests; headless `look=FILE.png` and `demo=DIR`.
- Cloud window check: Xvfb + xdotool + llvmpipe works (`apt install mesa-vulkan-drivers libxkbcommon-x11-0 xdotool`); script Reports/map-editor-2026-11-14/raw/window-run.sh.
- Report Reports/map-editor-2026-11-14/README.md, page https://claude.ai/artifact/NAJ89BJNhtkszVmFBWHutK.

**Open:** an "Edit maps" item in larder-world's start menu that runs larder-edit; keys for crystal charge and wall health; the owner has not tried it yet. Related: [[larder-maps]], [[larder-world]].
