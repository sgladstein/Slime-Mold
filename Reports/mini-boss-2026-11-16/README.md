# Mini-boss slimes, round 1 (2026-11-16)

The owner, 2026-11-16 03:41Z: "explore adding mini-boss slimes. There should
be random chances (when slime get concentrated) that mini-boss appear. It can
be similar to the condensation event (maybe even replace it). The slime forms
a stronger, more aggressive, sub-unit that leaves to seek out the player.
This is just my initial thought, expand on it. There should be variety in
them in behavior, attack, look/visual. We could also explore full bosses, but
I less convinced that is a good idea, but willing to consider. Think outside
the box."

This folder is round 1 only: research and options, with questions to the
owner before anything is built or written up (the owner's 11-16 02:37Z rule
for research threads). The report proper comes after their answers.

Page: https://claude.ai/artifact/4hyVm1cPgWBYssTEnitdBV (source [`page/`](page/index.html)).

## What is here

- [`brief.md`](brief.md): the neutral brief every designer read. It describes the model
  and the game, puts the owner's first thought after the designer's own list,
  and asks for ideas before judging.
- [`notes/games.md`](notes/games.md): outside research on elites, mini-bosses and bosses in
  other games (spawn directors and decks, cocoons and forming windows,
  affix systems, rewards, swarm and fluid enemies, merge and split bosses,
  when full bosses hurt a roguelike), 82 cited sources, and 20 structures
  worth borrowing.
- [`reviews/r1-encounters.md`](reviews/r1-encounters.md) (encounter and pacing designer),
  [`reviews/r2-simulation.md`](reviews/r2-simulation.md) (agent-based simulation designer, with a
  feasibility check against `BigParams`), [`reviews/r3-looks-and-wildcards.md`](reviews/r3-looks-and-wildcards.md)
  (art and motion, and the contrarian). Each wrote every section from the
  brief alone before reading anything in the repository.
- [`page/today-raid.mp4`](page/today-raid.mp4): run 2, planet 2, an idle squad of four at zoom 6,
  1:10 to 1:25 at 2x: a raid reaches the squad as a stream of single specks
  (22,700 walkers, 9,292 condensed or thick at 1:20). Frames from
  `larder-world run=2:2 mode=squads hand=idle look=DIR centre=372,236,6
  at=70,70.25,..,85 screen=960x600` on llvmpipe at main 62b56f4.

## Where the three designers agree

- A held body first: a sub-population with its own trail and the
  touch-step rule for it alone, pushed front to back by avoiding the ground
  it just covered. The two cohesion rules in `dead-ends.md` (`tether`,
  `blocked_keep` with `blocked_deposit`) failed as rules for the whole slime;
  for a separate body they give the shapes wanted. `tether`, `lone_step`,
  `blocked_keep` and the grazing wake (`graze_weight`) exist; making them
  per species is the new piece (r2, *Feasibility check*).
- Keep condensing as the material and first stage; replace the break-away's
  speck stream with a held body.
- A chance on a visible cause (crowding, long feeding, a cut-off part, the
  player's buildings nearby), capped per planet, with a 20-40 s forming the
  player can shoot to make it smaller.
- Kinds from parts (body, errand, the tier's temper), not a bestiary.
- Squads that shoot the nearest slime shield a mini-boss for free: squads
  should prefer it in range.
- A boss only as the slime itself: a last stand where the remnant gathers
  into one body, and the core world as a heart with organs fed by cords.

## Questions to the owner (posted 2026-11-16)

1. Which kinds to prototype first (recommended: Hunter, Fist on a stalk,
   Mortar).
2. Condensation: build on it (recommended) or replace it.
3. Should some mini-bosses stay put so the player has to go to them
   (recommended: a mix).
4. Pure chance, or a chance on a visible cause (recommended: the cause).
5. Boss: last stand, boss planet, both or neither (recommended: last stand
   as an option, filmed).
6. Squads shoot a mini-boss first when in range, by default (recommended).
