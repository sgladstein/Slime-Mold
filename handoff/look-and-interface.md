# Look and interface: state and brief

## State (2026-11-18)

**Open branch:** `claude/project-thread-8bv885`, one pull request: the
flinch's look (Look A, first piece). Report
`Reports/flinch-look-2026-11-17/`, page
https://claude.ai/artifact/XqBjSyq4eqiTdzdgE6Wqxg.

**What that pull request does.** The lime wash round a firefight was the
engine's own paint of the alarm (`engine.wgsl`, `draw`), not the view's
haze; both are gone, with `ALARM_LOOK` and the key `alarm_look=`. Slime
under alarm flushes warm orange in `prep` (`flush_at`, `flush_mat` in
`view.wgsl`); bare ground shows no alarm. A burning Flare, whose only sign
was the haze, is an ember and a red light (`draw.rs`, `FLARE_LIGHT`). The
haze strength pick is closed by it.

**Next, in order.**

1. Job 1, the pages that say what a build holds. Engine and tools' data is
   on main since #449 (`slime_mould::build`: `build()`, `switches`,
   `news`, `Memory`), with a first Experiments tab wired in `menu.rs`
   (`experiments::options` and `experiments::apply`: keep those two
   calls). Still to build here: the build number in the menu's corner, the
   "what is new" page with the new lines marked (`memory.is_new`), the
   "things to try" list with ticks (`memory.set_tried`), the Experiments
   tab's look, and a small HUD mark when `switches::state` is not empty.
2. The rest of Look A, from the ranked list in
   `wiki/looks/art-direction.md`: shots as bright streaks, droplets in the
   slime's material, a squad of 40 you can count, no effect bigger than
   its damage.
3. Reviews owed: Squad and arsenal's Provoke and Still (#448, page
   https://claude.ai/artifact/KeLdsKF61XaDDdESBLEdGF) and Enemy's amber
   set slime (#451, page https://claude.ai/artifact/1qg9m9ktNWHCCLL5Li74JQ).
   Both were filmed with the old green wash round the hits.
4. Win and lose screens with the planet's numbers.
5. Picks still open: the mini-boss colours; delete the fen moat A/B's
   unused side. (The set slime's look went to amber in Enemy's #451.)

**Worth knowing.**

- `LARDER_VIEW_WGSL=path` swaps the view shader at run time, in the game
  and in the tests, so a look can be tried without a rebuild.
- Films on llvmpipe: 1280 x 800 at 30 frames a game second is about 5 s a
  frame with four films side by side on four cores (`raw/film.sh` in the
  report). The bench for frame cost takes 90 s a run and two runs of one
  build differ by up to 10%.
- Where a Mind status can be read for a look (Enemy, 11-18): not in the
  look word (full) and not in `spec`. It is one word a walker slot in the
  engine's `news` buffer at `mind_at() + k` (kind in the top three bits,
  the step it ends in the low 29; valid only with `P.mind_on`), which the
  view already binds, so `draw_specks` can tint a walker with one new
  offset word passed as `set_crust` passes `setc_at`. A per-cell count
  (the pattern is `setcount`) needs a pass from Enemy: ask, naming the
  statuses.
- An owner's row to mind: `wiki/decisions-game.md`, 11-14 08:39, says the
  alarm is drawn on bare ground from its field. The plan of 11-17 put the
  flinch on the slime alone; the tools that lay an alarm (Flare, siren)
  now show by their own light. Direction agreed (11-18) and adds the
  dated note once the pull request is on main.

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
