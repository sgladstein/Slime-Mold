# Lab: a second slime species that fights the first

2026-11-18. The Lab lane's next brief (`handoff/lab.md`, *Later briefs*): "a
second slime species that fights the first". Direction (11-18 03:49Z):
"describe it from the model first (what fields and walkers two species
would share or fight over) before picking a look". Work in progress; this
section is that description.

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
