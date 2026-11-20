# The soldiers' Armour looks: olive, slate, dark camo (2026-11-19)

Look A (`wiki/looks/art-direction.md`, rule 8). The owner made soldier
designs with an image model (2026-11-19 14:24Z: a 3 x 2 sheet on magenta,
`src/world/sprites/soldiers-2026-11-19.jpg`), and after the test below
ruled (14:41Z): "I think our base soldier should be olive that upgraded to
slate that upgrades to heavy", then (15:11Z) "Replace heavy with dark
camo". Page: `page/index.html`, published at
https://claude.ai/artifact/Q4hcuw3SVcPJVNbVG4s5tD (listed in
`Reports/pages.md`).

## What landed

Every living soldier wears the look of the run's Armour level (the shop's
"hp" item, 0 to 4): olive at 0, slate at 1 and 2, dark camo at 3 and 4
(`armour_skin` in the window's `draw.rs`). The fallen keep sprite A's
dead tile. The designs are atlas tiles 17-22, body then helmet, cut by
`src/world/sprites/cut.py`: the magenta keyed out, the helmet found as the
largest grey-white blob, each figure scaled so its gun's tip lands at x
1.45 radii with the helmet's centre at (0.04, 0).

| Look | Sheet cell | Helmet radius | Muzzle across |
|---|---|---|---|
| olive | 1 | 0.28 | +0.11 |
| slate | 2 | 0.27 | +0.12 |
| dark camo | 6 | 0.29 | +0.23 |

Sprite A's helmet is 0.36 radii and its muzzle 0.305 across; `MUZZLE`
now holds each look's own point. Heavy (cell 5: 0.27, +0.17) was the
first pick at Armour 3 and 4, for 30 minutes.

Filmed on planet 1, run seed 1, greedy hand, at 40.0 to 42.9 s (15 to 16
soldiers draining the first crystal), the look forced by a local-only
override at Armour 0, 2 and 4: the three runs are the same frame for frame
(shots 449, kills 370 at 43 s), so only the look differs. At zoom 6 the
helmets stay the same white dome; dark camo is the darkest of the three
and reads clearly against the bare ground and the crystal there.

## The test that chose them

All six designs on one squad (each soldier design *its number mod 7*,
with sprite A as 0), on planet 1 at 45 s and planet 3 at 60 s, zoom 6 and
12, on a test build only. At zoom 12 the new designs carry more character
than sprite A (packs, pauldrons, the radio's aerial). At zoom 6 they come
down to colour and outline: olive (1), radio (4) and heavy (5) read best,
dark camo (6) sinks into dark ground, and the cloak (3) reads as a bush.
Their thick outline does not double with the game's.

## Making designs with an image model

The first prompt worked: a top-down view of one soldier facing right, a
big round white helmet in the middle, a flat #FF00FF background, a 3 x 2
sheet. A second prompt with a long list of rules (a design for each of
four guns, size and centring rules, thinner outlines) came out worse:
guns drawn side-on, figures bigger. Ask for small edits of a sheet that
works, one change a request; size and centring do not matter, since
`cut.py` rescales each figure.

## Open

- A fallen version of each look.
- A design for each gun (rifle, carbine, long gun, scattergun), if the
  owner wants to see what a soldier carries.
