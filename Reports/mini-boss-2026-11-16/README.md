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

## First experiment: a held body (CPU)

`examples/bodyprobe.rs`: a blank map, 600 walkers in a disc and one crystal
70 dish cells east, no growth, the race world's scale 2 and 15 ticks a game
second. Measures named first: the largest part's share, `compact` (rms
distance from the centre over a packed disc's) and `aspect` (long over
short axis), mid-way at 5 s and at 10 and 20 s, and the time to arrive.

- The body rules alone (`tether` 2-5, `lone_step`, `blocked_keep` with
  `blocked_deposit`, grazing) moved nothing: the crystal's scent does not
  reach 70 cells (went 0-6 cells in 20 s, 4 seeds;
  [`raw/body-r1-rules-alone.txt`](raw/body-r1-rules-alone.txt)).
- New `BigParams::aim_weight` / `aim`: each sensor also reads the weight
  times the cosine toward the aim point, added to trail and scent (panic,
  breakaway and raids replace the reading with the target). It gets there,
  as a stream: with `tether` 3-5, `lone_step` 3-6 or short sensors the line
  stayed (aspect 8-17 at 20 s; [`raw/body-r2-aim.txt`](raw/body-r2-aim.txt)
  and sweeps not kept). Why `tether` fails here: a walker touching few
  others may step to a cell touching as few, so a line can always advance.
- New `BigParams::leash`: a walker farther than `leash` x the body's radius
  (sqrt(walkers / pi)) from the body's centre, as the fine step began, runs
  back toward it. **Aim 30 + leash 1.3 (+ `blocked_retry` 4) against aim 30
  alone, seeds 1-6, at 5 s: compact 1.23-1.27 against 3.78-3.98, aspect
  1.9-2.0 against 13.1-14.3, 6 of 6 seeds, sign test p 0.031**; it covers 70
  cells in 22-23 s (about 3.3 cells a second, against 10-11 s as a line)
  ([`raw/body-r6-held-vs-stream.txt`](raw/body-r6-held-vs-stream.txt)).
  300 walkers: compact 1.06 at 20 s; 1,500: 1.21, slower (47 cells in 20 s).
  Chasing a target that circles 35 cells out at 5.5 cells a second, it turns
  after it and stays one body (clip `page/held-chase.mp4`).
- Cost: the leash's centre is one sum over the body's walkers a fine step;
  the probe's tick was 2.3-3.0 ms either way on one CPU thread.

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
