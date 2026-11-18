# Look and interface: state and brief

## State (2026-11-18)

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
own figure 1.5 px wide; a lone soldier is unchanged. How close soldiers
stand (`steer_all`'s soft push: 0.69 of elbow room draining a crystal) is
Squad and arsenal's; they will hold elbow room when a squad stands and
tell this lane when to film it.

**Open branch:** `claude/project-thread-8bv885`, one pull request at a
time.

- The bite light at a bite's reach (Look A4;
  `Reports/bite-light-2026-11-18/`, page
  https://claude.ai/artifact/Wk5DevY7VGNnCAN21hAPcb): `draw.rs`'s bite
  light reaches `Rules::touch` where it reached 3.5 dish cells. Every
  other light of the fight was already its damage's size (the report's
  table), so Look A4 is done with it.

**The Lure** is in the game from planet 1 (Squad's #461 and #464; Enemy's
#457 calls `set_steer` each frame), its pull round on open ground since
Enemy's #478. Filmed in play on planets 1 to 3 at zoom 6 and 12
(`Reports/lure-in-play-2026-11-18/`, page
https://claude.ai/artifact/QHdZD8LkMJ9E5ZaTDv3Aw2): round rings that bend
round rock, the slime streaming in, nothing to fix. The green scent light
is never lit (the field is drawn whenever a Lure stands); its code stays
while Squad asked to keep it.

**Next, in order.**

1. The rest of Look A, from the ranked list in
   `wiki/looks/art-direction.md`: a Mind status on the slime, fire at
   zoom 12; film the squad's spacing when Squad lands it; a hit's
   lasting hole would be a slime rule (Enemy's) if the owner wants one.
2. The Experiments tab's own look (still Engine's first tab, one row a
   switch), and the experiments in the pause menu.
3. Win and lose screens with the planet's numbers.
4. Picks still open: the mini-boss colours; delete the fen moat A/B's
   unused side.
5. Seen and left: at 200% on 1080 lines the goal's pill covers the top
   bar's fourth number.
6. The fort walls' tops still mottle at zoom 12 (pale flecks that are not
   their height), and their arcs show cell steps.

**Reviews sent** (11-18, through the coordinator): Squad and arsenal's
Provoke and Still (#448) and Enemy's amber set slime (#451), both fine to
land. Owed from them to the look list: a stilled or provoked body has no
look of its own (A5), and burning slabs draw as cell-sized squares at zoom
12 (A6). Enemy's Trunk fort (#473): fine to land; its flat walls led to
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
- Where a Mind status can be read for a look (Enemy, 11-18): not in the
  look word (full) and not in `spec`. It is one word a walker slot in the
  engine's `news` buffer at `mind_at() + k` (kind in the top three bits,
  the step it ends in the low 29; valid only with `P.mind_on`), which the
  view already binds, so `draw_specks` can tint a walker with one new
  offset word passed as `set_crust` passes `setc_at`. A per-cell count
  (the pattern is `setcount`) needs a pass from Enemy: ask, naming the
  statuses.
- The view's `V` struct has two spare words (`spare_a`, `spare_b`, the
  old haze strengths).
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
   fen moat A/B's unused side: Scott could not see the difference.

## How a look is judged

In motion, at zoom 6 and 12, never from stills alone. Each look change
carries a before and after clip of 5 to 10 seconds on an Artifact page
and one line in the game's "what is new" list. Small choices are yours and
Scott vetoes in play. Keep a blind A/B for a choice that sets the whole
art direction. Quote the frame cost on the software GPU as before and
after; GTX 1070 timings go in one batch before the milestone check.
