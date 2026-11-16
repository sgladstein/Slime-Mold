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

## Round 2 (2026-11-16): the hunter, built

The owner's answers (04:58Z, quoted in [decisions-game](../../wiki/decisions-game.md)):
every kind on the page; not bronze (the break-away's specks stay, separate,
more often early); a mix of kinds that hunt and kinds that stay put; a
chance on a visible cause, rising through the run, forming only "where
there is crowding or a big part gets cut off"; squads shoot mini-bosses
first. Then (05:23Z): "They should be significantly stronger than normal
slime too and vary quite a bit in strength from boss to boss". The look is
its own thread (05:21Z, "Make mini-boss slimes look cool").

**What was built** (branch `claude/project-thread-g8thj2`):

- The engine's held body, `big::Lump` (four at once, both engines): any
  walker can be a member (its lump in dense word bits 25-27). Each sensor
  adds a pull toward the lump's aim, a leash holds members within 1.3 x the
  radius of a packed disc of them, and members are spared panic, raids,
  surges, withering and the break-away's pick. Strength is the lump's too:
  a blow that would kill a member (a shot, a blast, fire) only hurts it
  until it has taken `armour` (its hits in bits 28-31), and each member
  touches a soldier as `1 + bite` walkers. The GPU twin matches the CPU
  exactly (`lump`, `lump plain`, and two squad-frame cases where blows hurt
  some members and kill others; 0 words off); planted bugs (pull's sign,
  haste dropped, GPU shots ignoring armour) all fail it.
- The game (`larder_world/bosses.rs`): a crowded knot (a square of reach 6
  round it at least 80% full, a walker a fine cell full) gets a chance at
  drawn times; a part of 400+ walkers cut off from every crystal with food
  gets a chance each second. Each mini-boss draws a grade as it forms that
  sets its size (reach 6 x 0.7 to 1.3), the blows each member takes (4 to
  15), its touch (2 to 5 walkers) and its pace (x1 to x1.4). It clenches in
  place 6 s, then hunts the nearest soldier; it breaks up under 30% of its
  members or after 90 s of hunting. `BossRules::for_depth` puts it in the
  run (table on the page): a crowding chance every 150 s on planet 1 down
  to 45 s on planet 8, a cut-off part's 2% to 10% a second, one at once on
  planets 1-3 and two from 4, grades 0-0.6 on planet 1 and 0.4-1 on
  planet 8. Off in `Rules::default()`.
- Squads shoot a mini-boss in reach and in sight first (`Game::boss_aim`,
  ahead of the engine's aim in `Game::target`), one function the gun
  thread's aim cards can override.

**One hunt** (run 1, planet 3, eight soldiers holding at the larder on free
fire, `hand=hold`, GPU on llvmpipe; `raw/round2-*.txt`): a crowd mini-boss
formed by itself at 91 s from 426 walkers inside the body by a crystal at
world 307,65, clenched to 0.95 of a packed disc's spread, left at 97 s and
crossed about 190 world units to the squad in 43 s, holding 418 of 426 and
a spread of 0.95-1.36 the whole way. Before the squad aimed at mini-bosses
first and before armour, it reached the squad at 140 s and broke at 146 s.

**Weak and strong, the same seed** (`set=boss_grade_lo/hi=0` and `=1`):
the grade 0 one formed from 219 walkers (4 hits each, touching as 2), came
under fire at 2:12 and broke at 2:15 (Broken, 60 left); no soldier died. The
grade 1 one formed from 718 (15 hits each, touching as 5), came under fire
at 2:12 and killed all eight soldiers by 2:22, losing 23 members; it wrapped
round the squad in a crescent while the guns carved its middle
([`raw/round2-grade0.txt`](raw/round2-grade0.txt), [`raw/round2-grade1.txt`](raw/round2-grade1.txt)). One shot
that landed on the packed body hurt about 45 members (223 kills from 5
shots at 2:12 on grade 0), so an armour of 5 a member (the first try)
lasted 5 s under the squad's focus, about as long as no armour had before
the squad aimed at mini-bosses first; the range was raised to 4-15. The
pace bonus (x1.4 at grade 1) made the crossing only about 8% faster: a
packed body moves as fast as it has room to.

Clips (`round2/`, page https://claude.ai/artifact/AEQ89ydrEj4qQva7CehQZg):
`hunt-wide.mp4` (1:26-2:20 at 5x, `centre=380,160,3.5`, before armour and
boss-first aim), `gather-close.mp4` (1:28-1:44 at 3x, `centre=312,66,10`:
inside the body the knot cannot be told from the slime round it, the look
thread's first job), `grade0.mp4` and `grade1.mp4` (2:06-2:25/2:30 at 4x,
`centre=425,245,8`); every clip `larder-world run=1:3 mode=squads hand=hold
look=DIR screen=960x600` on llvmpipe, made with
[`raw/round2-clip.py`](raw/round2-clip.py). The hunt's lines:
[`raw/round2-hunt.txt`](raw/round2-hunt.txt).

**Review fixes** (an independent review of the branch, 2026-11-16; it found
no CPU/GPU parity bug in the dense word, params or passes):
- An armour blow no longer counts as a kill (it takes no mass). Counted, a
  grade-1 boss's 15 blows a member paid a render turret 15 kills' food
  each; a condensed walker's break still counts (a walker's mass).
- A cut-off part waits out the landing's `first` seconds (the opening's
  blob is often cut off) and never re-forms within `rest` (30 s) where one
  ended: its own remnant, an unfed part of 400+, re-formed the next second
  with its hits healed.
- `Game::boss_aim` takes the next bullet's reach (`Game::shot_reach`:
  buckshot's 0.6 fell short of a boss its gun's reach took in, so every
  shot hit empty ground), and a soldier with slime on it shoots that
  unless the boss is within its defend reach.
- Members no longer ripen: packed past the crowd limit, they condensed and
  half were absorbed, so a boss lost about half its walkers with no shot
  fired and broke at its quorum.
- A release clears the lump and its hits on every slot, a member in flight
  from a blast too; before, it landed still a member and joined whatever
  boss next held that lump. The GPU's lump readout reads none once no lump
  is on (its sums stopped, the last ones stayed), a negative leash is none
  on both engines, and the checked GPU world and the tests' delayed world
  pass the lump calls on.
- New tests: how a boss ends (fizzled, broken, spent: no slot keeps its
  bits), a cut-off part through the game's second (grace, then at once,
  then the rest), the same seed gives the same bosses, planet 8's first
  minute (none before `min`, the game doesn't end), and a GPU blast twin on
  a mini-boss (25 members killed on both, 112 hurt, 125 flung, 0 words off;
  let go mid-flight, no slot keeps the lump; the old GPU release left 150
  slots marked and fails it).

**Cost**: the lump passes run only with a lump on (`lumpclear`, one thread;
`lumpsum`, a thread a walker; `dcount` with a lump on); the CPU side's
crowd and part searches run once a game second over the mould grid. Not
yet timed on the GTX 1070.

**Next**: the other kinds on the same body (Wanderer, Leech, Sapper,
Mortar, Pinwheel), the stay-put ones from crowding on a crystal and the
runners from cut-off parts; the origin at each planet's start crystal is
the run-progression thread's question.
