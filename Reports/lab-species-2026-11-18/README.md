# Lab: a second slime species that fights the first

2026-11-18. The Lab lane's next brief (`handoff/lab.md`, *Later briefs*): "a
second slime species that fights the first". Direction (11-18 03:49Z):
"describe it from the model first (what fields and walkers two species
would share or fight over) before picking a look". The model comes first,
then what was built, the counts and the recommendation. Page: "Two Slimes"
(`page/`, listed in `Reports/pages.md`).

## The model, first

What the GPU engine already keeps for more than one species
(`engine.wgsl`, the outside rules and the species block, 2026-11-11):

- **A walker's species** is the low byte of its `spec` word. Children take
  their parent's. Up to four species (`nsp`).
- **Each species lays its own trail** (`sp_in`, one field a species) and
  the sum is the plain trail. A walker reads its own species' trail plus
  `avoid` times everyone else's (`seen`): positive draws it to the others,
  negative turns it away. One `avoid` serves every species, so the reading
  is symmetric; the CPU has a per-species version (`species_avoid_of`).
- **Each species can have its own settings** (`sets`): sensor angle and
  reach, turn, step, deposit, keep, crowd limit, birth cap. So two species
  can differ in shape (a droplet and a network) and speed.
- **Conversion** (`convert`): a walker joins the species whose trail at its
  cell outweighs its own by a factor. It is the only fight the engine has.

What the two species would share, because the engine keeps one of each:

- **Space.** One walker a cell (the crowd rule) holds for everyone, so two
  bodies that meet push against each other and the border is physical.
- **Food.** Crystal charge, scent and the far smell are one field each.
  Whoever stands on a crystal eats it, and births cost food, so two species
  compete for crystals by holding them.
- **The alarm.** Shots and deaths lay one alarm field, which both read.
- **The ground and the units.** Walls, rock and water bound both; raids and
  panics aim at the squad for both.

What they could fight over, and how:

1. **Ground, by conversion** (exists). Measured on 11-11 and 11-12: on
   symmetric dishes the border stalls at an even split, because sorted
   bodies hold their colour; where one side had more trail it took
   everything (`Reports/creatures-2026-11-11.md`,
   `Reports/player-mould-2026-11-12.md`, dead ends on `species_boost`,
   `species_share`, raw-trail conversion). A stat that converts wins
   everything or nothing.
2. **Crystals, by starving the other** (needs nothing new). Two colonies
   on different crystals spread until they meet, and each grows only where
   it holds charge. The fight is territory and it is slow.
3. **Each other, by eating** (new). One species kills the other where its
   walkers outnumber the other's at contact and gains the food. With a
   hunter drawn to the prey's trail and prey turning from the hunter's,
   the engine has what it needs for chases, a moving front and boom and
   bust in numbers, all from walkers and trails.

And where the player comes in: the war between two slimes is a third party
the player can use. The lure (now going into the game) can lead one body
into the other, guns can tip a fight already under way, and a hunter that
clears the prey may then turn on the squad.

The prototype picks 3, built on the GPU engine only, with 2 as its
control (the same map with conversion and no eating).

## What was built

Lab branch `claude/project-thread-4tc75h` only, GPU only, two code commits
that never merge (`849670d`, `a96e78c`). The second species is the green
mould of the garden (`own=1`), started on run planet 1 against the orange
enemy, idle hand, run seed 1.

- **One-way conversion** (`cycle=1 next=1 prev=-1`, existing keys): green
  takes orange walkers; nothing takes green. The green is drawn to orange
  trail and the orange turns from the green's.
- **An attacker's edge** (`LAB_CONVERT`): an orange walker turns green
  where green trail is at least that share of its own. The garden's own
  setting (1.2) overwrote the `convert=` key, so the first tries at an edge
  measured nothing.
- **A hunter that starves** (`LAB_HUNTER`): green takes no crystal share
  and lives only on the energy of the walkers it converts. That is the
  "eating": prey's food becomes the hunter's, by the engine's own energy.

Run: `LAB_CONVERT=0.8 LAB_HUNTER=1 larder-world run=1:1 mode=squads own=1
hand=idle cycle=1 next=1 prev=-1 at=...` (`raw/run.sh`, `raw/film.sh`).

## Results (planet 1, one seed a scene, green / orange walkers)

| Arm | 60 s | 120 s | 240 s | end |
|---|---|---|---|---|
| Control: conversion at 1.2, both eat crystals | 13,397 / 12,538 | 14,201 / 10,891 | 14,161 / 12,078 | 300 s: 13,914 / 12,325 |
| Hunter edge 1.0, eats crystals | 13,850 / 11,980 | 14,925 / 10,292 | 13,444 / 9,762 | |
| Hunter edge 0.8, eats crystals | 13,850 / 11,987 | 15,401 / 9,539 | 13,641 / 8,709 | |
| Hunter edge 0.6, eats crystals | 13,790 / 12,037 | | | orange gone at 113 s |
| Starving hunter, edge 1.0 | 13,679 / 11,932 | 998 / 9,588 | 2,179 / 9,768 | 420 s: 4,260 / 10,345 |
| Starving hunter, edge 0.9 | 13,674 / 11,937 | 265 / 10,387 | 457 / 11,489 | 420 s: 1,588 / 13,168 |
| Starving hunter, edge 0.8 | 13,651 / 11,957 | 1,556 / 9,016 | 2,779 / 9,087 | orange gone at 299 s |
| Starving hunter, edge 0.7 | 13,647 / 11,964 | 2,395 / 8,148 | | orange gone at 149 s |
| Starving hunter, edge 0.6 | 14,157 / 11,450 | | | orange gone at 100 s |

What it looks like (clips `page/base.mp4` and `page/hunt.mp4`, 300 game seconds at
1 frame a second, played at 20 fps; posters at 4:30):

- **Control.** The two bodies meet and hold a border; green gains about
  2,000 walkers by 2:00 and gives some back. The stalled even split of
  11-11 and 11-12, again.
- **Starving hunter at 0.8.** Green leaves its crystal hungry, most of it
  starves between 1:20 and 1:40 (13,930 to 881), and the survivors reach the
  orange body's north edge. From there a green front moves south through
  the orange body for two minutes, slow while the orange still holds its
  crystal and fast once the crystal falls (2,766 green at 4:00, 12,837 at
  5:00). Green walkers also gather round the squad, because the garden's
  mould tends the soldiers.
- **The edge is the dial.** 0.6 to 0.8: the hunter eats the whole slime in
  1:40 to 5:00, and the planet ends (it is "wipe out the slime"). 0.9: the
  hunter barely survives and grows slowly. 1.0: it lives beside the orange
  and keeps growing (4,260 at 7:00). No arm showed a second boom: once the
  prey is gone the hunter has nothing left, and where the prey survives the
  hunter has not yet crashed again.

## Part two: leading the slime into the hunter (11-18)

Direction, 08:49Z: "yes to the one more step, leading the slime into the
hunter with the Lure ... A day at most." The lab's lure field (the
branch's, every walker reads it, at the game's pull of 10, `far_smell=-10`)
laid in the hunter's ground north-west of the orange body from 2:00 to
7:00: `LAB_MARKS=lure:120:45:110:300` (world units), hunter edge 1.0, run
seed 1, idle hand. Clips `page/lure-none.mp4` and `page/lure-lure.mp4`
(2:00 to 7:00 at one frame a second, played at 20 fps).

| Arm (green / orange) | 3:00 | 5:00 | 7:00 | Both at 7:00 |
|---|---|---|---|---|
| Edge 1.0, no lure | 1,243 / 9,438 | 3,133 / 9,998 | 4,260 / 10,345 | 14,605 |
| Edge 1.0, lure at 45,110 | 2,244 / 8,887 | 6,714 / 7,395 | 8,247 / 6,651 | 14,898 |
| Edge 1.0, lure at 60,95 | 2,366 / 8,721 | 6,698 / 7,419 | 8,568 / 6,519 | 15,087 |
| Edge 0.9, no lure | 173 / 10,525 | 774 / 12,301 | 1,588 / 13,168 | 14,756 |
| Edge 0.9, lure at 45,110 | 147 / 10,923 | 1,397 / 12,934 | 4,002 / 11,989 | 15,991 |

- **The lure feeds the hunter.** It draws orange walkers out of their body
  into the green, which converts them; the front moves south about twice
  as fast, and by 7:00 the orange is 36% smaller (6,651 against 10,345).
  Where the lure sits within the hunter's ground matters little.
- **The slime in all is no smaller.** Green and orange together are within
  2% either way at edge 1.0: the lure turns orange slime into green slime.
  It pays the player only if the hunter is worth more to them than what it
  ate (it takes no crystal food, so a crystal it holds is free).
- **Run seed 2: the hunter starved to nothing by 2:00**, with and without
  the lure (green 0 at 2:00, orange 13,293 and 13,409 at 7:00), because it
  starts far from prey. The small-seed start comes before anything else.
- A purple patch shows on the lure clip at 7:00 where the front reaches
  the eaten crystal; not looked into.

One seed, so a direction, not a proof.

## Recommendation

**Keep the starving hunter as the second species; it is the first fight
between two slimes that does not stall.** Conversion alone stalls (11-11,
11-12, the control here). A hunter that lives only on what it converts
moves a front through the other body, and one number (the edge) sets
whether it wins in two minutes, in five, or lives beside it.

Not for the game yet. Before a card:

1. **Start the hunter small.** Here it starts as half the walkers on a
   crystal it cannot eat and nine tenths of it starves in 20 s. A hunter
   that spawns as a small seed near the prey would show the chase without
   the crash.
2. **Its own colour and side.** Here it is the player's garden mould, so
   it tends the squad. In the game it would be a second enemy (a strain of
   the slime), counted in the planet's slime, and the planet should not
   end because one slime ate the other.
3. **The player's angle** (tried in part two): the lure leads the orange
   into the hunter and doubles its speed, but the slime in all is no
   smaller. For it to pay, the hunter should be worth something to the
   player: a strain that dies off once its prey is gone, or one that the
   turrets render for more food.

Dead end recorded: an edge of 0.6 to 0.8 wins everything, as a stat that
converts did on 11-11 and 11-12; the shape of the fight is set by the edge,
not by the bodies.
