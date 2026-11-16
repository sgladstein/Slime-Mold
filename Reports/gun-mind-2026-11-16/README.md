# Mind bullets: step D of the gun-card plan, Mind first

2026-11-16. Step D of the [gun-system review](../gun-system-review-2026-11-16/README.md)
("bullet families by what they change in the simulation"; the plan's Mind
row: "how walkers sense, for a few seconds"), after
[seek](../gun-seek-2026-11-16/README.md). The plan's test for the step: "Each
status stamped on one network for 10 s, blind. Can the owner name the
status from the clip? Frame cost."

Page: [Mind Bullets](https://claude.ai/artifact/PLHGQc5kKxXi5qwNmqQqyb)
(source in `page/`): the four blind clips with their counts, and the
armory's new tab.

Status: **built, for merge; the blind quiz awaits the owner; the 1070
timing is booked.** Three Mind bullets, Blind, Sour and Spin, in both
engines, exact twins. Lace was dropped (nothing to see), as was Spin's
first form.

## What was built

- **The status** (`BigDish::mind`, the CPU; the GPU's `news` words after
  the gas slots, `mind_at()`): one word a walker, its kind in the top 3
  bits and the fine step it runs out at in the low 29 (`MIND_T`), as the
  gas words hold theirs. A Mind hit stamps every living walker (or dormant
  one) within `mind_r` (8 dish cells) for `mind_secs` (6 s) of fine steps
  (`BigDish::mind`, the GPU's `mindlay` pass with the strike row: centre,
  radius, steps, kind). A child is born with none. Until the first stamp the
  GPU reads no word (`Params` row `mind_on`, written from `Engine::minds`, so
  the flag and the shader's test cannot disagree).
- **Blind:** the three readings times 0 after everything else has made them
  (`mind_reads`; a wall, -1e5 or less, stays a wall), and the wander, floor
  and chance rules off (`!blind`), so the walker takes only the plain rule:
  straight on, turning only off a wall, into water or fire too.
- **Sour:** the three readings times -1: it turns from the trail and food it
  would follow, as an explorer does.
- **Spin:** `spin_curl` radians (35 degrees) added to the heading after the
  turn rule every step: rings that stop advancing.
- Every walker takes them, scared, raiding, surging and lump walkers too:
  the readings change after whatever made them (the slime-enemy thread's
  note, 22:04Z).
- **The cards** (`Card::Blind`, `Sour`, `Spin`; `Shell::Blind` and on): payloads, cost
  2, tier 2, the Mind cards' rose (`card_look`), icons an eye struck
  through, a U-turn, a ring round a dot. A burst carries `Burst::mind`;
  `BurstRules` carries `mind_steps` and `spin_curl` (`RoundRules::burst_rules`
  builds them for all four callers). `RoundRules` keys `mind_r`, `mind_secs`,
  `spin_curl` (degrees), settable with `larder-world set=NAME=V`. The rounds
  count Mind casts in their own slot (`Rounds::cast[4]`).
- **Sold**: the run's shop rolls them as bullets from planet 2
  (`shells_at`); like any payload they take their set's shape (buckshot
  blind is four small blind clouds). The gun range's armory has them on a
  tab of their own, standard only (`range_shelf`, "Bullets for the mind"):
  in every set on the bullets tab they put it on 4 pages at 200% UI on 1,080
  lines (66 rows), standard only on 3 (57); the test
  `the_menus_real_lists_fit_one_screen` failed both. LOADOUTS: `blind`,
  `sour`, `spin`, `multi-blind`.
- **Films:** `boom=` now casts the deck `load=` gave the squad
  (`Rounds::cast_at` uses `casts_of(g, 0)`): before, a loaded film's boom
  cast the `rounds=` round, and the first paired films showed nothing.
  `watch=X,Y,R` prints each second the walkers within R of a point and from
  R to 2R (the look's walker count).

## Tests (both engines exact)

- `big::tests::mind_statuses_blind_sour_and_spin_the_walkers` (CPU): among
  walkers that moved, blind kept its heading 1.00 of the time against 0.50
  plain; spin turned exactly `spin_curl` more than plain on all 89 walkers
  that moved on both dishes; sour's trail ahead of the front sensor a step
  on 0.971 against 2.276 plain; a 5-step status runs out.
- `rounds::tests::a_mind_bullet_casts_its_status_and_kills_nothing`.
- `mouldgpu`: `mind_statuses_match_the_cpu` (each kind stamped 273 walkers,
  0 words off, 0 walkers off; planted bugs caught: a status a step longer
  put 273 words off, a GPU spin with no curl 184 walkers off); a Mind dish
  in `the_world_matches_the_cpu_for_one_tick` (a fifth each blind, sour,
  spinning, run out, none; panic and walls on; 2,786 held, 2,638 born, 0
  words off, 0 walkers off); and a Mind case in
  `the_squad_matches_the_cpu_for_one_frame` (the slime-enemy thread's ask;
  panic on: 2,740 of 4,600 living walkers hold a status after the frame,
  0 words off, 0 walkers off, the squad's outputs equal).

## The blind clips (the deciding test)

`raw/film.sh` and `raw/make_clips.py`, counts in `raw/counts.json`. One
game on the Fork (race world, seed 0, idle hand, plain rounds) the same to
19.9 s; at 19.9 s every squad loads the clip's bullet; at 20 s one round
goes off, free, on the thin vein at (110, 264), with a 10 s status
(`set=mind_secs=10`). Near view at zoom 8, 15 frames a second, 19.9 to
31 s, llvmpipe, with the HUD cropped off (its card strip showed the loaded
bullet's icon, which gave the key away in the first cut).

| Clip | Walkers within 8 cells: at 20 s, mean 21-30 s, lowest, at 31 s | From 8 to 16 cells: at 20 s, mean 21-30 s, at 31 s |
|---|---|---|
| plain | 15, 19.1, 14, 18 | 18, 27.5, 31 |
| blind | 15, 11.9, 8, 13 | 18, 20.6, 37 |
| sour | 15, 11.2, 7, 18 | 18, 14.9, 16 |
| spin | 15, 12.1, 4, 14 | 18, 25.5, 43 |

Each status cuts the walkers on the vein by about 40% while it lasts
(11-12 against 19). Sour also halves the ring round it (14.9 against
27.5); blind lowers it less (20.6) and spin hardly (25.5). The vein is
thin, so these are small counts from one film each. No round killed a
walker.

What the clips show: blind walkers leave the vein in straight lines one by
one and scatter; sour walkers stream off it in chains and the vein frays,
then knits back after the status; spin beads the vein into knots that stop
moving. Blind and sour both send walkers off the vein, so the quiz is
whether they read apart.

**The blind clips' key** (kept off the page): clip 1 plain, clip 2 spin,
clip 3 sour, clip 4 blind (drawn with `random.shuffle`).

## Dropped on the way

- **Spin by turning harder** (the turn angle x 3 for the status): nothing you
  could see in 5 s on the vein; a walker on its trail seldom turns.
- **Lace** (sensing reach x 0.15 for a finer mesh, x 2.5 for thicker cords):
  nothing you could see in 10 s on the vein; the reach matters only where
  the trail forks. Both in `../dead-ends.md`.

## Cost

- Until the first Mind hit, nothing: the GPU's sense pass tests one flag in
  `Params`, and the CPU's Mind list is empty.
- After it, each walker reads one more word a step (the CPU: one branch a
  walker), and each Mind hit is one pass over the walkers (`mindlay`, as the
  gas's stamp). The `news` buffer is one word a walker slot longer (the
  dense words moved up by n). Births clear the child's word.
- **1070 timing: booked** (the PC was offline at 23:30Z), `larder-world
  bench=40` plain on main and the branch, and blind, sour and spin on the
  branch.

## Not settled

- Whether the owner can name the four clips.
- The numbers are first guesses: radius 8, 6 s, 35 degrees a step.
- Next in step D, per the plan: readers and marks, then route, element,
  force; cut last.

## Files

- `raw/film.sh`: one clip; `raw/make_clips.py`: MP4s and counts;
  `raw/counts.json`: the counts.
- `page/index.html` and its clips (`quiz-1.mp4` to `quiz-4.mp4`, key above)
  and `armory-mind.png`: the Artifact page's source.
