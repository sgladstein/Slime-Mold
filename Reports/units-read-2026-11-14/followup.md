# Units read, follow-up: group rings and ghost signs (2026-11-14)

The art director's two follow-ups to [#227](README.md), page section
*Follow-up* on https://claude.ai/artifact/FQ1y7vhrqpgGZGn9LFw64F.

- **Group rings** (`controls::ring_groups`): selected units whose rings
  overlap share one ring round the whole group (union-find over pairs,
  centre the group's box middle, radius the farthest member plus one
  ring). In the volley clip at zoom 6, 0:24, nine rings in two knots
  became two rings (3 and 6 soldiers). A lone unit keeps its own ring.
- **Ghost signs** (`Marks::icon`, `WALKING`, `STANDING`, `SLASH`): the
  seconds left stay as text; "on the way" is a striding figure and "no
  builder" a standing figure struck through in the warning red, 5x7 at
  1.5 pixels a dot. On the build clip at 0:18-0:21 three ghosts sit side
  by side and their signs no longer run into each other.

Films: `larder-world map=fork look=DIR hand=volley rounds=shrapnel
centre=268,215,6 at=21..24` and `hand=build centre=224,120,6 at=18..21`,
llvmpipe, every other frame of 15 a second, in `page/followup/`.
