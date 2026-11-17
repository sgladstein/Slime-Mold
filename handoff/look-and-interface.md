# Lane brief: Look and interface

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
