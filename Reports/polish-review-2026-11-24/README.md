# Polish review, 2026-11-24

The owner, 2026-11-24 03:00Z: "We need agent to review the game. What
aspects still look unpolished, unprofessional, etc". Review only: each
finding names a fix and the lane that owns it; nothing here is built,
except one key-name bug (below).

**How it was looked at.** Build `406c2c9` (main, 11-23) on llvmpipe, at
1536 x 864 with the UI at 100%, which lays out as his 3840 x 2160 screen at
250% does. The run bot (`hand=strat:bot`) played planets 1, 2 and 3 of run
seed 1 to 4:00, planets 5 and 7 to 1:30, with the near view on the squad at
zoom 6 (he plays at zoom 3 to 8 in 90% of the seconds of his 11-23 play
logs, planets 5 and 6), plus planet 2 at zoom 12 for a second in half
seconds; every menu page (`menu=`), the command card's Build, Tools and
Formation pages (`card=`) and a tooltip (`hover=`). End cards from
`Reports/pr-stills/end-cards-2026-11-22.png`. Stills only, so motion is
not judged here, except the zoom 12 strip.

**Not covered:** sound (no ears in a cloud session; every sound is
synthesized, and the owner called it "really bad" on 11-20 before two sound
passes), the pause menu (no headless key opens it), a full game's end card,
the map editor, the slime viewer, planets 4, 6 and 8.

Page: https://claude.ai/artifact/7STCLzcjcjSz95TkKJYLfu (copy: `index.html`, `img/`).

## What already looks finished

Soldiers at zoom 12 (skins, heads, guns, the flinch); the ground's
materials (the barrens' hardpan plates, the fen's moss, rock with a lit
rim); the slime seen from far out (glossy ridged body, rim highlights);
shot tracers as comets; the top bar; tooltips; the cursors; the end cards'
layout.

## Findings, worst first

Lanes: Look = Look and interface, Squad = Squad and arsenal, Engine =
Engine and tools.

1. **The slime is soft and smeared at the zoom he plays at** (Look; data
   from Enemy if needed). From far out it is glossy and ridged; at zoom 6
   planet 1's Drifter is a flat orange wash with a wide glowing edge and
   perfectly round blurred grey discs in it, and planet 3's Trunk trail is
   a flat translucent yellow sheet. It is the star of the game and the
   least crisp thing on screen, against art direction rules 4 (edge soft
   over at most 2 px at zoom 6) and 5 (raised, glossy, alive). Fix: carry
   the thick body's lighting (slope from walker density, one key light,
   specular) into the thin film from zoom 4 up; cap the edge's soft width in
   screen pixels; break the round discs into the walkers' own sparse grain
   or irregular holes; give the Trunk's trail tube a lit edge and a darker
   core. Before and after clip at zoom 6 and 12 on a page (a big look).
   Stills: `slime-drifter.jpg`, `slime-trunk.jpg`, `slime-far.jpg`.
2. **Cell stairs where slime meets water** (Look). Planet 1, zoom 6: where
   the Drifter fills a river bank its edge is a staircase of cells, against
   rule 4 (no cell stair at any zoom). Likely the slime's mask stops at the
   water cell by cell (`deep_wall`) while the shore is drawn from the soil
   spline. Fix: clip the slime's shading by the same spline the shore uses.
   Still: `slime-drifter.jpg`.
3. **Crystals look like clip-art cubes** (Look). Three flat-shaded faces, an
   isometric cube icon, the same on every planet, up to 250 px across at
   zoom 6, often the largest and brightest shapes on screen, with nothing
   where they meet the ground. Fix, keeping the hexagon (owner rule 8): a
   bevelled rim, a darker inner table, a few inner fracture lines, a cyan
   light pooled on the ground by its charge (light that shows the
   simulation), an occasional glint; consider a smaller drawn size or a
   cluster of shards on the same footprint. Still: `crystal-live.jpg`.
4. **A spent crystal reads as a grey umbrella** (Look). Twenty-four flat
   grey trapezoids in a spoked hexagon, as large and as high as a live one.
   Fix: lower and darker, a few broken shards and dust at ground level,
   thinner spokes. Still: `crystal-spent.jpg`.
5. **Squad marks clutter the fight** (Look). A squad's health bars show at
   95% health, so a 45-soldier squad draining a crystal wears 45 bars most
   of the time; a spread squad draws two or more overlapping rings
   (`ring_groups`); the "1" label and the formation mark sit on the ring's
   line. Fix: bars under 70% or for 2 s after a bite, the squad's health on
   its tile; overlapping ring groups merged into one outline; the label on
   a small dark plate inside the ring. Stills: `rings-bars.jpg`,
   `rings-overlap.jpg`.
6. **The bottom HUD reads as a developer's panel** (Look; Squad for the
   names). Four empty unlabelled boxes beside "Rifle" and an unexplained H
   button; a lowercase status line ("blob, tight · fire: defend · no orders
   queued · Loadout 1 · 13 tools"); labels broken over two lines with a
   hyphen ("Attack-/move", "Fire:/defend"); key letters about 8 px in the
   corners; a 120 px minimap jammed in the corner; a lone "11" pill
   floating over the squad tile. Fix: one HUD pass: empty gun slots
   collapse to one line saying where cards come from, H labelled Loadout,
   the status line as icon chips, one-word command names, key letters in a
   keycap, a framed minimap at least half again as big. Still:
   `hud-bar.jpg`.
7. **The command card's pages and icons** (Look for icons; Squad for page
   layout and keys). Holes in the grid; Back in column 5 on two pages and 6
   on the third; one letter meaning different things by page (B: Buy
   soldier, Mortar, Blast, Open; G: Drain crystal, Door, Germ); on planet 1
   the Tools page is five locks. Icons: Sleep gas is a pause sign; Blob and
   Tight use a dot cluster that is the crystal and currency mark (and the
   "1 ✱" beside the squad); Turret power and Cheaper tools share a chevron;
   the Lure is a water drop; the Mine is the same target ring as Fire:
   defend. Fix: a fixed grid with Back always bottom right; locked tools
   hidden behind one "more tools in the shop" tile; one small icon set drawn
   for this game at one stroke weight. Stills: `card-pages.jpg`,
   `shop-tools.jpg`.
8. **The shop's counter and cards** (Look; Engine for the Dev tab;
   Direction for words). Five cards fill a band across the middle; about
   45% of the counter above and below is blank, and the planet under the
   shop shows through (a crystal's hexagon behind the bag row). Each card
   shows different rows: payload cards show multipliers (Speed x1.8), gun
   modifier cards show the whole gun's five stats, identical on every one
   (Damage 72, Fire rate 0.8/s, Range 20, Speed 36, Aim 7.0), and "no room"
   once the gun is full; building and tool cards are one line on a tall
   card. A "Dev mode" tab is in every run's shop. The planet-ahead map is a
   blocky 64-cell mosaic. Fix: cards larger or centred with the next
   planet's story under them; an opaque counter; a card shows only what it
   changes, as before and after ("Damage 10 → 16"), the gun's own stats once
   in the loadout strip; Dev mode behind an Options switch; the map
   smoothed. Stills: `shop.jpg`, `shop-tools.jpg`.
9. **No title or identity** (Look; Direction for the name). The main menu's
   title is "Larder World" in the UI's own font at 26 px with no logo, over
   a live map, with a "218 new" badge; the subtitle says "mould"; the larder
   it names is gone from the squad game. Fix: a title drawn from the
   simulation (for example the name as a mask the slime fills in the
   menu's dish) and a question to the owner on the name, through
   Direction. Still: `main-menu.jpg`.
10. **Words** (Direction; Look applies them). Both "mould" and "slime" for the
    enemy (the title's subtitle, Options' "The mould" tab and the
    Attack-move tooltip's "any mould in reach" against "slime" everywhere
    else); the simulation's own word on screen
    ("kill every last walker", "13,119 walkers now", "0.5 times the first
    planet's walkers"); larder-era lines ("The slime took the larder", "on
    the core, attack it", "R: the larder's rally point"); a double colon
    ("Planet 1 of 8, Dreskor: Wipe out the slime: kill every last
    walker"); the objective three times at landing (the top pill and two
    toasts); "Crystal 2 has run dry". Fix: a short word list (slime,
    crystal, squad, slime count), one objective line, toasts only for
    news. Still: `toasts.jpg`.
11. **Options and Keys are the developer's console** (Look for pages, Squad
    for the key rows). The Options tab "The mould" is raw numbers (Panic 2,
    Wither 0.01, Shared food 0.05, Surge 1800, Flinch 0.9); Keys is about
    60 rows mixing squad mode with the old RTS round and gone features
    (siren, outpost, larder, scent view, next map, test guns). Fix: tuning
    tabs move to the Slime viewer or a Dev page; Keys shows squad mode
    only, grouped (move, fight, build, camera). Stills: `options-mould.jpg`,
    `keys.jpg`.
12. **The camera shows past the map's edge** (Engine for the camera; Look
    for the edge). At zoom 6 on planet 1 the top 70 px of the screen is the
    off-map band with a hard black line, and the right edge too, from the
    landing on. Fix: keep the near view inside the map, or fade the edge
    into dark rock. Still: `map-edge.jpg`.
13. **The Starfall pickup's bloom** (Squad). A white flash about 120 px
    across with pink sparks round "40", the brightest thing on screen,
    against the owner's 11-14 rule (an effect is never bigger than its
    damage), here under the HUD's edge. Fix: half the size, gold rather
    than white; no fall where the HUD covers. Still: `starfall-bloom.jpg`.
14. **Sprawler threads drawn over crystals** (Look). The planet 2 slime's
    dashes cross the crystals' faces like scratches. Fix: draw the specks
    under the gem layer, unless the slime covers it (the film over a
    covered crystal already shows that). Still: `sprawler-over-gem.jpg`.
15. **The end card's graph** (Look). A row of identical orange bars with a
    dotted line; the lost-run card still says "The slime took the larder".
    Fix: the slime as an area and the soldiers as a line over it. Judged
    from 40-second films only. Still: `end-cards.jpg`.

## Fixed here

The key card's row for the spread keys read ", " because `ui::key_name`
had no name for `[` and `]` (added). The HUD and pages that print the
same keys get them too.
