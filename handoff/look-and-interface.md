# Look and interface: state and brief

## State (2026-11-20)

**Slime viewer (branch `claude/project-thread-517zll`, the thread "Build a
slime viewer to tune looks"; owner 11-20 23:17Z):** `viewer=PLANET`, the
main menu's "Slime viewer" row and `viewer.cmd` open a run planet on the
gun range's clock with a panel at the right (`src/bin/larder-world/viewer.rs`;
F1 hides it). Four tabs: Slime (the planet's slime, its body look, and its
on-off rules), Spawn (every mini-boss kind, fruit, wave, the last-stand
heart, strikers, heartbeat, ore, core, raid at the click, by
`larder_world/force.rs`, which calls play's own starts), Look (every plain
number or colour `const` at the top of `view.wgsl`, grouped by its comment,
changed live by text-substituting the shader and rebuilding its pipelines,
`View::reshade`), Rules (the slime's rule keys, `Rules::get`/`set`, and 20
engine keys). Save writes `slime-tweaks.txt` at the repo root
(`world::tweaks`, gitignored; `LARDER_TWEAKS` overrides), which every game
then plays, with "your slime tweaks (N)" in the corner; a copy goes to
`play-logs/`. A new look `const` in `view.wgsl` shows up in the Look tab by
itself if it is a plain number or `vecN<f32>` at the top level; the lump
colours were lifted into `LUMP_SKIN_*`/`LUMP_CORE_*` for it. Headless
checks: `LARDER_VIEWER_TAB=look|spawn|rules|slime`,
`LARDER_VIEWER_SPAWN="T:NAME[@X,Y];.."`.

**On main:** the flinch's look (#454; `Reports/flinch-look-2026-11-17/`,
page https://claude.ai/artifact/XqBjSyq4eqiTdzdgE6Wqxg). An alarm shows as
a warm flush on the slime (`flush_at` in `view.wgsl`), bare ground shows
none, and a burning Flare is an ember and a red light (`draw.rs`).

**Also on main:** the Lure's rings (#458; `Reports/lure-rings-2026-11-18/`,
page https://claude.ai/artifact/4QyfFzQNXQuAPfvz83WqTe).
`View::set_steer(at)` draws the engine's steering field as pale cream
hairlines of equal walking distance (`steer_px` and the block in
`shaded`), and while they show the Lure's green scent light is unlit.
The window calls it each frame since Enemy's #457 (`prepare_now` in
`main.rs`: the field's word while a lure stands, else 0).

**Also on main:** the set slime's look settled as amber (#460; the glaze,
the unmarked side, the row `set_look` and the key `setlook=` deleted;
`Reports/sim-use-2026-11-17/e2b-look.md`, *Settled*). The switch list is
two rows (`feel`, `crystal_regrow`).

**Also on main:** the build pages (#471; `Reports/build-pages-2026-11-18/`,
page https://claude.ai/artifact/2SwV4MTJMh7MFLQAyuqHPn). A "What is new"
row on the main menu with a count; `Page::News`, named for the build, with
two tabs (what is new with NEW marks and a pane; things to try with ticks,
`StartEvent::Tried`); the build and any experiment on in the main page's
corner; `ui::Model::trying`, a yellow mark under the clock in play. The
window fills them in `menu.rs` (`build_page`) and `draw.rs`.

**Also on main:** raised walls and the Lure's rim (#477;
`Reports/raised-walls-2026-11-18/`, page
https://claude.ai/artifact/EBAKa6mpRV9M31H5CoDiF1): set slime stands
`SET_RISE` 4 above the sheet (`set_rise` in `prep`); no ring within half a
ring of the Lure's reach (`View::set_steer_reach`, one call in `main.rs`'s
`prepare_now`). Flung droplets in the slime's material (#482, Look A3;
`Reports/droplets-2026-11-18/`, page
https://claude.ai/artifact/9vsJuSofq14P5jShjiWyqe): gold beads lit by the
body's key light, each with a shadow (`draw_specks`, `DROP_*`);
`DROP_GLOW` is the number if hits read too quiet at zoom 6.

**Also on main:** the fallen soldier (#491; `Reports/fallen-soldier-2026-11-18/`,
page https://claude.ai/artifact/A6fJKZbrQDx9PCwuHWFBTj): the `dead` layer
of `sprites/soldier.svg` is a soldier on its back in dull olive (atlas tile
3; `render.js` reproduces the other tiles byte for byte), over
`blood_pool` in `view.wgsl`, which spreads over `POOL_GROW` 2.5 s
(`Look::fallen` in `main.rs`; `Marks::fallen`). `BLOOD`, `BLOOD_RIM` and
`POOL_R` are the numbers if the pools read too loud.

**Also on main:** heads on top (#494, Look A3;
`Reports/squad-heads-2026-11-18/`, page
https://claude.ai/artifact/F58mCnsvBbT2CMUk2Rhz31): each living soldier's
helmet drawn again above every body (`Marks::soldier_head`, `HEAD_ONLY` in
`view.wgsl`, the loop after the figures in `draw.rs`), with a ring of its
own figure 1.5 px wide; a lone soldier is unchanged. Squad and arsenal
(11-18) keep `steer.rs` as it is: a squad at rest holds elbow room (5.13
fine cells against 4.8); only a squad draining a crystal heaps, because
the crystal's rim has no room for 35 at 4.8, so the helmets carry the
count there.

**Also on main:** the biting slime (#502; the owner's 11-18 18:04Z and
18:08Z notes: a bite "just looks like more of the same slime";
`Reports/biting-slime-2026-11-18/`, page
https://claude.ai/artifact/Wk5DevY7VGNnCAN21hAPcb): while a soldier is
bitten (`touching` above 0) `draw.rs` sends its touch disc as a bite zone
(`Light::bite`, lift `BITE_LIFT`, lighting nothing); `view.wgsl`'s
`bite_share` turns the slime in it crimson (`BITE_SLIME`), `slime_on`
draws it over the figure (the walkers that bite are all under it), and a
flinch flushes the figure red (`BITE_HOT`) with its helmet white. The
orange bite light is gone. Every other light of the fight was already its
damage's size (the report's table), so Look A4 is done with it.

**Also on main:** a Mind status on the slime (#503, Look A5;
`Reports/mind-status-2026-11-18/`, page
https://claude.ai/artifact/2or1C2HtnaRqNMX7pbznFz): each walker under one
lays a soft rose disc a cell across (`draw_mind` in `draw_specks`, a value
under 256 in the specks' third screen), and `base` tints the slime there
toward `MIND_ROSE`, fading over the last 1.5 s. `View::set_minds(news, at,
fade)` takes the engine's Mind words (`gas_at` / 4 + `GAS_SLOTS` +
walkers), called once in `main.rs`. `MIND_TINT` and `MIND_HALO` are the
numbers if it reads too loud or too faint.

**Look A6 checked (11-18), nothing to fix:** fire at zoom 12 shows no cell
squares. Planet 3's amber walls lit by two Torch shots (run seed 1,
`hand=hold@344,169`, `boom=76,374,116,torch boom=76,317,121,torch`) and
the Fork's meadow under Torch rounds, at 30 frames a second: the mean
step between neighbouring pixels is flat across the 12 pixel phases of a
cell (4.6 to 5.5) in a frame and between frames (0.60 to 0.65). The
squares noted on 11-18 were in Enemy's #451 clip, scaled to 0.75 and
squeezed to 8 frames a second; fire has been drawn per pixel since 11-14.

**Also on main:** the Sprawler's dashes (#510, below) and the goal pill
kept off the top bar's numbers at every UI size (#512). From #504, the end screens (the brief's job 4;
`Reports/end-screens-2026-11-18/`, page
https://claude.ai/artifact/YSjVHU9Es2ztXjeE52VLt1): `ui::Ending`
(`Model::end`), filled in `draw.rs`, drawn and shrunk to fit by
`Hud::end_screen`; `film_ends` (ignored, `film.rs`) makes the pictures.
The Experiments tab as cards and an Experiments page on the Esc menu were
built by a helper thread under this lane (#509, reviewed and queued 11-19).

**Look B, a look a slime** (the owner, 11-19 00:08Z: "multiple looks could
eventually apply to different slimes"; 00:13Z: iterate, they need not all
be right at once). `view::SlimeLook`, by the tier (`SlimeLook::of`, one
call in `main.rs`'s `start`), sent to the view with a walker's pace
(`set_slime_look`; the `V` struct's last row, two words spare). Planned:
the Drifter a soft drifting sheet, the Sprawler fast thin threads, the
Trunk thick veined cords with its amber walls.
The Sprawler's threads lie in a faint film of their own trail (11-20,
`THREAD_VEIL` 0.3 in `view.wgsl`, under the Drifter's
`SHEET_VEIL` 0.4), so its loose front reads as one body.

**The soldiers' Armour looks** (owner, 11-19 14:41Z: "our base soldier
should be olive that upgraded to slate that upgrades to heavy"; 15:11Z:
"Replace heavy with dark camo"): three designs from the owner's image-model
sheet (`sprites/soldiers-2026-11-19.jpg`, cells 1, 2 and 6) are atlas tiles 17-22, body then helmet, cut by
`sprites/cut.py` (run it after `render.js`). The run's Armour level
(`run::ITEMS` "hp") picks one for every soldier (`armour_skin` in
`draw.rs`: 0 olive, 1-2 slate, 3-4 dark camo; `Marks::dress_since`,
`extra` bits 27-28; `skin_body` in `view.wgsl`). `MUZZLE` holds each
look's barrel mouth (cut.py prints them). The fallen wear the look too:
tile 3 (sprite A's fallen olive) and its slate and dark camo recolours,
tiles 23 and 24 (`fallen` in cut.py, `skin_dead`). Not done: a design for
each gun if the owner wants that. Generating designs:
small edits of a sheet he likes, one change a request (a long rule list
made the model draw the guns side-on). Test page:
https://claude.ai/artifact/Q4hcuw3SVcPJVNbVG4s5tD
(`Reports/soldier-skins-2026-11-19/`).

**A soldier asleep in its own Gas** (Squad's #513 puts soldiers to sleep
in their own cloud): its body takes the gas's lavender by the cloud's
level at its feet and dims while it is out, its helmet white
(`Marks::doze_since`, `doze_tint` in `draw.rs`, the `DOZE_*` constants in
`soldier_at`; doze is `extra` bit 26, beside the Armour look's bits
27-28). Filmed on planet 1 (Gas cast on the squad at 40 s): lavender at
42 s, olive and firing at 48 s.

**Look C started (11-20):** rock as stone (#534: mottling, grit from
zoom 5-10, cracks in patches; `ROCK_MOTTLE`, `ROCK_CRACK*` in
`terrain_top`) and planet 1's meadow in patches by the water (#535:
`flora_form` flora 0 meadow, clump 0.6, wet 0.4, spring-green tips).
A spent crystal is a cracked husk of frosted glass (11-20, #550): rays and chords from its middle, each shard tilted
and toned, the cracks coming in with `1 - charge`; the old terraces
(`riser`) are gone. `GEM_HUSK`, `GEM_SHARD`, `GEM_RAYS`, `GEM_SPENT`.
Crystals cast shadows (#553, 11-20): the hexagon swept away
from the light in `shaded` (`GEM_CAST`, `GEM_SHADOW`; applied under the
thin film's floor, never on the crystal's own faces); rock's cast
shadow `TER_CAST` 1.6, `TER_SHADE` 0.55 (`terrain_over`).
Planets 2 and 3's traits (11-20): the slime's trail on bare land
(`trail_lin`, bilinear `look` byte 0) as pale dust tracks on the barrens
(`DUST_TRAIL`, `DUST`, `DUST_LIFT`) and a wet slick in the fen
(`SLICK_DARK`, `SLICK_SHEEN`, `SLICK_GLINT`). On planet 3 the Trunk's
packed body leaves little bare trail, so the slick shows mostly where
the squad shot the slime back and as a halo round the body.

**Look D started (11-20):** the shop's counter as shelf rows
(`shelf`, `shelf_row`, `SHELF_H`, `SHELF_W` in `start/shop.rs`): every
offer a row with its icon, name, the first sentence of its `about`, price
and pips, in columns of at least 420 units, while all fit on one page; else
the old tiles that page. Bends shop B's half-size tiles (owner 11-15).
The landing (11-20): each soldier drops in over its shadow in the
first second (`landing`, `LAND_SECS`, `LAND_STAGGER`, `LAND_LIFT` in
`draw.rs`); only the drawing. A dust ring was tried and dropped.
The end (11-20): the world plays on for `END_BEAT` (2.4 s) before the
card, the clock held and the camera drawing back 18% (`ending.rs`, one
call in `main.rs`'s `step`, Engine told); in the window only, so headless
runs, tests and the run bot end at once. Its frames carry no soldiers, so
a charge on the squad breaks up as it would once they are gone. A win
that shows the slime dying back would be a slime rule (Enemy's).
The shop, 11-20: an empty bag is one strip with its use in a line, and
the pane's idle page opens on a "THE PLANET AHEAD" label, not the top
bar's heading again (`start/shop.rs`).
The main menu, 11-20: its dish covers the window (`menu_camera` in
`menu.rs`; the headless menu pictures frame it the same way), with a
shade under the title and the foot hint (`start.rs`). Options looked
sound as it is (filmed 11-20).
The menu's dish is a run's planet 1 to 4 on the clock's seed while the
sandbox map is its default (`open_menu`); a picked map still previews
there. It starts again at its mission's verdict too (`main.rs`'s frame).
The play-zoom pass (coordinator, 11-20), planets 1 to 3 at zoom 6 and
12 with the greedy hand: fixed the Sprawler's loose dashes (above) and a
squad's number floating between two groups (`LABEL_LINK` in `draw.rs`,
over the soldier nearest its middle, clear of its group's soldiers in that column). Seen and
left: the controlled squad's selection rings (one per straggler; thin, and
they say who is in the squad); planet 4's squad fell in 18 s from a bare
start, which no run reaches (Direction, 11-20: film late planets with a
run's upgrades, the run bot or a dev-mode shop spend). On a funded run (`hand=strat:econ,
go=4,aim=6 bank=20000`) the strikers read as gold beads in the Sprawler's
film (planet 2, 64-70 s); no mortar formed on run 1's planet 4 by 120 s.

The shop, later 11-20: dimmed to 215 behind it (`start.rs`), and shelf
rows grow to `SHELF_TALL` (112) with the counter's room, the icon, name
and price up to a quarter larger and the whole `about` text.
Planets 2 and 3's ground (#574, Reports/planet-ground-2026-11-20/): mud
plates, salt pan, scrub with shadows; fen reeds and moss; Form::fall plant
shadows (a helper thread under this lane; page
https://claude.ai/artifact/Vk2jnBLdLoVFrCGmYHEAZu).
#581, round 2 of Reports/planet-ground-2026-11-20/: fen bog water
(`wet_rgb`/`deep_rgb`), fen reeds spaced for zoom 6, a per-planet
`rock_tint` in `ter_cell`. Its next ideas need new fields: duckweed on
fen water, wind-blown dust on the barrens.

The HUD, 11-20 (planet 2 at play zoom): a squad tile grows to 168, not
136, so one squad's says "working" whole beside YOU, and a long state
("working, 3 on guard") falls back to its first word where it does not
fit (`ui.rs`). The top bar, goal pill and command grid read sound.
The end card's stars are star icons, 26 units, the middle one raised
(`Icon::Star`, `StarOpen`; three 11-unit discs read as loading dots).
The Trunk's cords (11-20, `Reports/trunk-cords-2026-11-20/`, page
https://claude.ai/artifact/Xi8hjRyFbnbnWgsxsx9f7c): `SlimeLook::Cords` for
planet 3, its trail off the body a translucent raised tube (`CORD_VEIL`,
`CORD_RISE`). The Swarmer's swell (11-20, `SlimeLook::Swell`, planet 4):
where its alarm flushes it, its height grows (`SWELL_GAIN`, `SWELL_RISE`),
so alarmed walkers are raised beads, not a flat glow. Sporer, Sleeper and
Grudge are still Plain: the next slime looks.
Selection rings join within `RING_LINK` (3.5 rings centre to centre,
`controls.rs`): a 51-soldier squad on planet 3 wore 51 rings at zoom 6.
The play alerts (top left) hide while the end card is up: the card says
the same, and a stale "the mould is at the larder" read as still in play.
The Campaign and Keys pages were filmed and left; What is new cuts long
rows to one line with the whole in its pane, as designed.

**Open branch:** `claude/project-thread-wypzro`, reused per PR.

**The Sprawler's look (#510):**

- The Sprawler's dashes (`SlimeLook::Threads`): each walker's speck is a
  dash along the way it came over `STREAK_SECS` (0.1 s, 2.25 cells at its
  22.5 cells a second; `speck_line`), and its film is laid along the same
  dash holding one walker in all (`lay_streak`). A first cut weighed every
  cell of the dash as a whole walker: the loose walkers turned into fat
  slugs and planet 2's hub filled in as a thick body. Stills only, by the
  owner's 11-19 ruling (pages for big looks only).
- Baseline stills and clips of planets 1 to 3 at 60 s, zoom 6 and 12,
  from main at 74aeb46, are on the film helper's branch
  `claude/project-thread-5qskxs` (`Reports/look-b-baseline-2026-11-19/`;
  not merged, for this lane's checking).

**The Lure** is in the game from planet 1 (Squad's #461 and #464; Enemy's
#457 calls `set_steer` each frame), its pull round on open ground since
Enemy's #478. Filmed in play on planets 1 to 3 at zoom 6 and 12
(`Reports/lure-in-play-2026-11-18/`, page
https://claude.ai/artifact/QHdZD8LkMJ9E5ZaTDv3Aw2): round rings that bend
round rock, the slime streaming in, nothing to fix. The green scent light
is never lit (the field is drawn whenever a Lure stands); its code stays
while Squad asked to keep it.

**Next, in order.**

1. Look B: the Drifter's is done (#526). The Trunk's look stays its amber
   walls and packed body until a slime rule gives it flow; then the thick body's veins
   and flow; iterate the Sprawler's dashes on what the owner says in play.
   Tried and not kept (`Reports/dead-ends.md`, 11-19): the Trunk's film
   along each walker's way, and a smooth droplet for the Drifter (blurry at
   zoom 6). The Trunk's body has no flow or tubes to draw (headings every
   which way, the trail flat even at `knee=60`, the shared food a smooth
   pool round each crystal): Direction parked its veins for Milestone 2
   (11-19).
2. The rest of Look A: film the squad's spacing when Squad lands it; a
   hit's lasting hole would be a slime rule (Enemy's) if the owner wants
   one.
3. Look C, rest: shadows for buildings if they float; traits for
   planets 4 to 8 after Milestone 1 (parked: more planet polish).
4. A landing, a win and a loss that happen in the picture: done 11-20
   (the landing, and the end's beat).
5. Picks done 11-19: the fen moat deleted; the hunter stays violet
   (filmed on planet 8, bright and glossy over its dark purple plants,
   framed by the gold it forms in); the fort walls' tops smooth
   (`SET_SOFT`, #517); soldiers dressed by Armour (above).
6. Soldier looks: a design per gun if the owner asks.

**Helpers** (11-19, the owner offered them): a film runner thread (builds
and films on its own machine; send it exact commands, it pushes the output
to its branch and sends back the log lines asked for) and the Experiments
builder (#509). To save tokens (owner, 11-19 04:17Z): look at crops at
about a third of the size, one 2 x 2 sheet for motion, single log lines.

**Reviews sent** (11-18, through the coordinator): Squad and arsenal's
Provoke and Still (#448) and Enemy's amber set slime (#451), both fine to
land. Owed from them to the look list: a stilled or provoked body has no
look of its own (A5, done), and burning slabs draw as cell-sized squares
at zoom 12 (A6, checked: not so now). Enemy's Trunk fort (#473): fine to land; its flat walls led to
the raised walls above.

**Worth knowing.**

- `LARDER_VIEW_WGSL=path` swaps the view shader at run time, in the game
  and in the tests, so a look can be tried without a rebuild, two films
  of one build are the same game, and a test can be run against a shader
  planted wrong.
- Films on llvmpipe: 1280 x 800 at 30 frames a game second, 180 frames,
  is about 3 minutes with four films side by side on four cores
  (`raw/film.sh` in either report). For "how much of the picture a sign
  changes", film the same game with the sign compiled out and compare
  frame for frame (`raw/make_page.py` in the lure report).
- The bench for frame cost takes 60 to 90 s a run, and two runs of one
  build differ by 10 to 20% in every pass: take three runs a side, in turn.
- A Mind status is read for the look from the engine's `news` (Enemy,
  11-18): one word a walker slot at `mind_at() + k`, kind in the top three
  bits, the step it ends in the low 29 (`View::set_minds`). A per-cell
  count (the pattern is `setcount`) would need a pass from Enemy.
- The view's `V` struct has two spare words, in its last row (since
  11-19; `mind_at` and `mind_fade` took the old haze strengths' two).
- The owner asked (11-18 03:05Z) whether any green should show in the
  flinch look, since he saw none, and said "Overall it is better". None
  is drawn; Direction recorded it as a question, not a ruling.

## The brief (the plan of 2026-11-17)

Model and effort: Fable, extra high. Read `00-all-lanes.md` in this folder
first.

## Your job

You are the art director Scott asked for on 11-14 ("I want another thread
to review all graphics in general and mange the current graphics work and
start new work where it sees issues. We need to polish this game up"), and
you own what the player sees that is not a rule: the look shader
`src/world/view.wgsl`, `marks.rs`, `sprites/`, `ui.rs`, `start.rs`,
`start/`, the window's `menu.rs` and `draw.rs`, and the style guide in
`wiki/looks/`. On 11-17 he called the visuals "an important factor". For
Milestone 1 your target is exit test 2: the fight looks good in motion at
zoom 6 to 12 on his PC.

The style guide `wiki/looks/art-direction.md` is good and stays: living
light on dark ground, the picture is the data, one hue a role, sharp at
every zoom. Every pixel still comes from a field or a game state.

## The order of look work (Scott, 11-17 17:39Z)

"we should explore all your suggestions, priority in the order that you
listed A->B->C->D". A is Milestone 1's look test. B starts once A passes
his eye, whichever milestone that falls in, and C and D follow the same
way.

- **A. The fight round the squad.** What the planning thread saw in
  frames at zoom 6 and 14 (software renderer, stills only, so check in
  motion): the flinch field's haze draws as a lime wash in cell-sized
  blobs over ground and slime alike, and is the largest thing on screen
  where the player aims. With `flinch=0` the same frame is clean orange
  on dark. Tracers are thin dotted lines. Flung droplets are outlined
  shapes in a different style from the slime. At 42 soldiers the squad is
  one overlapping pile.
  Do: draw the flinch as a change in the slime's own colour, only where
  there is slime (the haze is in `view.wgsl`, `shaded`; history in
  `Reports/alarm-look-2026-11-15/`). Make a shot a short bright streak the
  eye can follow, and a hit the slime's own reaction: the hole it opens
  and the walkers it flings. Draw droplets in the slime's material. Space
  the squad so 40 soldiers can be counted; sprite A stays (Scott's pick).
  Every effect no bigger than its damage (his 11-14 ruling).
- **B. The slime itself.** Veins and flow inside the thick body, loose
  specks that read as parts of one body and not as glowing pebbles, and a
  look for the origin when Enemy builds it.
- **C. Planets and terrain.** Stronger grounding (shadows exist but are
  faint at play zoom), materials for rock and spent crystals, which are
  flat grey today, and one visual trait per planet that only it has.
- **D. Menus, shop and screens.** A landing, a win and a loss that happen
  in the picture, then the shop and menus. Judge the shop closely first;
  the planning thread did not.

## First jobs, in order

1. **The pages that tell Scott what a build holds**, with Engine and
   tools, who supply the data: an Experiments tab on the Options page
   drawn from the switch list, a "what is new since you last played" page
   and a "things to try" list on the start menu, the build number on the
   menu, and a small HUD mark when any switch is off its default.
2. **Look A**, starting with the flinch haze.
3. **Keep one ranked list of look jobs** in `wiki/looks/art-direction.md`
   and review every other lane's visible change against the guide.
4. **Win and lose screens** that show the planet's numbers.
5. **Pending picks that are now yours to decide**: the set slime's look
   (glaze or amber), the haze strength, the mini-boss colours. Delete the
   fen moat A/B's unused side: Scott could not see the difference (done
   11-19).

## How a look is judged

In motion, at zoom 6 and 12, never from stills alone. Each look change
carries a before and after clip of 5 to 10 seconds on an Artifact page
and one line in the game's "what is new" list. Small choices are yours and
Scott vetoes in play. Keep a blind A/B for a choice that sets the whole
art direction. Quote the frame cost on the software GPU as before and
after; GTX 1070 timings go in one batch before the milestone check.
