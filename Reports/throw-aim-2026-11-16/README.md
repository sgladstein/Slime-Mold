# Aiming a throw: thrower, reach, arc and area (2026-11-16)

Owner, 2026-11-16 05:04Z: "when throwing an explosive, we should improve the
visual for aiming and range."

## Before

The Tools page (blast, det cord, mine, germ, sleep gas) drew only a green
disc of the tool's radius at the cursor. It showed no thrower, no reach and
no throw line, though a tool goes off only within `KitRules::reach` (24
dish cells) of a soldier holding one. A click out of reach was taken, and
then an alert said "Too far".

## What it draws now

Aiming any of the five tools (`Model::throw`, a `Throw`; drawn by
`throw_aim` in `src/world/ui.rs`, filled by `App::throw_ghost` in
`src/bin/larder-world/draw.rs`):

- **The thrower**: a ring round the soldier that would throw. This is
  `Kit::thrower`, the nearest soldier holding a tool, the same one whose tool
  `use_tool` spends.
- **Its reach**: a dashed cream ring of `KitRules::reach` round the thrower.
- **The arc**: a dashed cream arc from the thrower to the landing point,
  bowed up the screen by `ARC_RISE` (0.28 of its length) over a faint dashed
  ground line, brightening toward the landing, with a small ring where it
  lands. A det cord lands at its middle, where `use_tool` measures reach.
- **The area** in the tool's colour: fire orange for blast and cord, amber
  for a mine, green for the germ, lavender for sleep gas. It has a dark edge
  so it holds on the gold slime. Dashed rings show what reaches past the
  area: a blast's crater (7 cells) and its alarm (12 cells), and the blast a
  mine sets off.
- **Out of reach**: everything turns red, the arc stops at the reach's edge
  with a cross where the throw would fall, and dots run on to the cursor. The
  note by the cursor says how many cells out it is. The click is refused and
  targeting stays, as for a building's red ghost. Food short and closed
  ground are refused the same way, each with its own note.

The reach ring and the arc are cream, not the tool's colour. In the first
films the orange arc was lost on the gold slime.

All lines are 1-pixel marks drawn side by side (`stroke`), 2 or 3 pixels
wide with a dark edge, so they stay crisp at the owner's zoom-in. The arc is
drawn before the rings: marks past `MAX_MARKS` (2,048) are dropped, and in
the first build a busy frame lost the arc that way.

The note chip by the cursor now moves to the cursor's left where it would run
off the screen's right edge. An out-of-reach note was cut off there.

## Cost

Aiming adds at most about 260 line and ring marks (about 70 for the arc, 120
for the reach ring's 60 dashes, 64 for the two dashed rings) and only while a
tool is aimed. Model and HUD on the cloud CPU: 0.130 ms a frame aiming against
0.091 ms with nothing aimed. That is one sample each, taken while tests ran on
the same machine. The GTX 1070 has not been timed.

## Films

`film.sh` makes the frames: the Fork, seed 1, camera held at zoom 12 on the
squad, cursor swept over 31 frames between 6 and 9 s. A new headless key,
`hover=X,Y,X2,Y2`, sweeps the cursor, and `hand=throw` (or `throwgas`) aims
the blast (or the gas). Page: `page/index.html`, listed in `Reports/pages.md`.

## For the device slot

The gun-card plan's step E moves explosives into a per-squad device slot on a
cooldown. Any point or line command can carry a `Throw` on `Model::throw`, so
a device aims with this ghost by filling the struct. It is not tied to the
Tools page.

## Not done

- No flight: a tool still goes off the moment it is clicked. Whether a
  grenade should fly along the arc is a gameplay question for step E.
- The reach is drawn round the nearest holder only, not as the union of every
  holder's reach. In a tight squad the two are nearly the same.
