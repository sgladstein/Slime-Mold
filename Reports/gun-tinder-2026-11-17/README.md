# Tinder: the element family's second card, and the crossing test for heat in the sensors

2026-11-17. Step D of the [gun-system review](../gun-system-review-2026-11-16/README.md)
after the [Torch](../gun-element-2026-11-17/README.md). The owner, 03:07Z:
"Continue without my guesses. I am busy"; the game designer's order
(08:43Z): "Torch, then Tinder, with Douse held", and after the Torch
(10:14Z): "Keep fire_weight off for now ... Build Tinder next, then run
that test on one seed: food beyond a lit Tinder line, counting walkers
that cross in 30 s, weight 0 against -5. If the weight cuts crossings
clearly and the body flows around the line, turn it on, because that's
the fire fence. If it only holds the slime in the flames, leave it off and
note it. Leave the Torch's cost alone until the balance pass, as you did."

Page: [Tinder Bullets](https://claude.ai/artifact/DMHMAsve7qyjLV8JXsK54D) (source in
`page/`): Tinder in play on the barrens against the torch alone, the
windows, the crossing clips with their counts, and the armory.

Status: **Tinder is built; heat in the sensors stays off**, by the
designer's rule: the weight did not cut crossings (57 against 10 from 16
to 45 s) and the slime did not flow round the line; it waited at the
fire's edge and crossed the char when the fire died. The simulation-use
thread's fire alarm, filmed as a fourth arm, did the same (31).

## Tinder

- **What it is** (`Card::Tinder`, a flight card, a trail like Scorch,
  Bait, Mist and Smother; `trails()` now has five counts): every bullet's
  leg lays dry brush in discs of `trail_r` 2 every 2 dish cells
  (`squad::Burst::fuel`, `RoundRules::trail_fuel` 1, a full jungle's, at
  which fire burns at full heat), up to the hit, but none within
  `trail_fuel_gap` 6 dish cells of a soldier (below). Copies widen the discs as for the other trails.
  It kills nothing. A later fire runs along the line: a torch on the same
  gun lights it from the hit, a scorch trail lights what it lays, a blast
  lights where it lands.
- **Laid fuel** (`BigDish::lay_fuel`; on the GPU the IGNITE pass in a lay
  mode, word 49's byte 1, so no new pass or binding): a disc raises every
  open, dry cell off a pile to the fuel level if it is lower, even ground
  where nothing grows; a bare cell turns scrub. Rock, water, piles and
  crystals are left alone. Laid before the burst's light, so a scorch
  trail's own light finds it.
- **It withers**: the plant rule now lets plants over their ground's
  capacity rot back to it by `BigParams::plant_rot` (0.0002 a tick, x 8 a
  plant step; GPU plant header word 62): on barren ground a full line is
  gone in about 40 s, and too thin to carry a killing fire after about 25.
  With nothing laid the rule is exactly the old one (every earlier twin
  and census number reproduces: the element report's p1g2 plain and torch
  windows matched to the walker, `raw/windows.txt` "check" lines).
- **The gap**: the first windows laid the brush from the muzzle, and every
  fire ran back along it to the squad: torch+tinder left as few as 2 of 8
  soldiers after a 10 s window (`raw/windows-nogap.txt`, 23 of 80 lost
  over ten windows), against 8 of 8 with the torch alone. The second try
  laid brush only from 6 dish cells along each leg, and laid nothing in
  play: a bullet flies over several frames, each frame's leg starts where
  the last ended, and a frame's leg is shorter than 6 (torch+tinder matched
  the torch to the walker in all ten windows, 0 trail bursts;
  `raw/windows-alonggap.txt`). Now no disc lays brush within
  `trail_fuel_gap` 6 dish cells of a living soldier (`rounds::keep_clear`),
  whichever leg it is on: every soldier lived through every window again
  (8 of 8 in all ten), and the kills rose (below). The other trails
  start at the muzzle as before ([dead ends](../dead-ends.md)).
- Cost 2, sold from planet 2 (`Card::tier` 2), on the armory's "Trails and
  shoves" tab (the shove cards and the trails share a tab: a seventh tab
  cut the tab row off at UI 200%, and the flight tab with Tinder had ten
  rows; `page/armory-trails.png`). Words: "Bullets lay a line of dry brush
  that fire runs along."; its icon stalks along a line with a bullet dot.
  Loadouts `tinder-torch` and `tinder-scorch`.
- **Frame cost**: none while unused (the plant rule's extra compare is a
  cell's p against its cap, already read). A cast adds one IGNITE dispatch
  over the dish per disc laid, up to three discs a dispatch, as the scorch
  trail's lights already do; not timed on the 1070.

## The crossing test (the designer's, one seed)

`raw/film.sh`: planet 1:3, idle hand, plain rounds. At 16 s a stream of
walkers runs from the slime's arm by the big crystal (world 205,216) to the
crystals across the meadow to its north-west. The line runs 40
world units across the stream, from 162.6,190.8 to 191.4,163.2, laid whole
at 16 s by the headless key `trail=16,LINE,tinder,lit` (a tinder line in
the card's discs, with no gap, every disc lit at heat 1). `cross=LINE,30`
counts the walkers beyond the line (on the food's side, within 30) each
second and the walkers that crossed it; the table counts crossings from 16
to 45 s (the last count line of a film that ends at 46 s). A fourth arm,
the simulation-use thread's ask (11:33Z): its `fire_alarm` (C4, off: each
burning cell adds `fire_alarm` x its fire to the alarm field shape C's
flinch reads) at 50, filmed with a build of that branch with Tinder
applied. Its weight-0 run matched this build's to the count every second,
so the arms compare.

| Clip | Crossed 16-45 s | Of them while the fire burned (to 40 s) | Killed by fire | Cells burning on the line at 17, 30, 40, 45 s |
|---|---|---|---|---|
| no line | 214 | | 0 | 0 |
| lit line, weight 0 | 10 | 10 | 119 | 372, 524, 255, 87 |
| lit line, weight -5 | 57 | 4 | 44 | 367, 527, 253, 84 |
| lit line, fire alarm 50 | 31 | 0 | 28 | 372, 527, 248, 85 |

- **Weight 0**: the stream walks into the flames and burns (119); 10
  crossed between 22 and 30 s, then none: the walkers that reached the
  line died there, and nothing came after the fire died at 40-45 s.
- **Weight -5**: the stream stops at the fire's edge. Only 4 crossed while
  it burned, fewer burned (44), and the walkers banked up behind the line;
  as the fire died (41-45 s) 53 crossed the char in 5 s. The frames do not
  show the body flowing round the line's ends.
- **Fire alarm 50**: the same, held harder: none crossed while it burned
  and 28 burned, then 31 crossed the char in the last 4 s.
- **The call, by the designer's rule**: the weight did not cut crossings
  (57 against 10 from 16 to 45 s), and it did not turn the body round the
  line. It held the slime at the edge while the fire lasted, which is not
  the fence the designer named, though it is not "held in the flames"
  either: fewer burned. `fire_weight` stays off (0). A note for later: with
  the weight a fire is a wall that lasts as long as it burns and kills
  little; without it a fire kills what walks in. The fire alarm behaves the
  same way. A fence would need a fire that lasts, or a line that stays
  scorched.

## Tests

- `rounds::tests::tinder_lays_a_line_that_a_fire_runs_along` (CPU, the
  flats map planted only in its top band): the card, its words, cost and
  tier; `trails` counts it; a 20-dish-cell leg lays a disc every 2 (10),
  and `keep_clear` round a soldier at the muzzle leaves 8, from 6 out; a
  scorched leg keeps its 10 discs with brush only past the gap; a later
  leg (10-15) beside a soldier lays none, and one (10-20) with the squad
  back at the muzzle lays all 5; the `set=` keys. A
  tinder line laid across bare soil (59 cells laid) and a torch at its end:
  21 cells lit, the fire ran to within 0.4 dish cells of the line's far
  edge and killed 44 walkers in 10 s. Without the tinder the same torch
  lit and burned nothing.
- `plants::tests::laid_fuel_withers_back_to_the_ground`: a cell laid over
  its capacity rots to it (0.99 after a step, 0.5 after 200), a laid cell
  on cap 0 to nothing; rock and crystal cells stay 0.
- `gpu::larder_world::tests::tinder_lays_and_burns_on_the_gpu_as_on_the_cpu`:
  flats, the top 40 rows planted and the bottom 30 barren soil; three
  tinder lines laid with no gap (48 discs, 223 cells laid) and a torch;
  plants and kinds equal after the lay, the fire and plants equal (max
  difference 0) each second for 10 s with fire on, 43 killed by fire on
  both (llvmpipe).

## The paired windows in play

`raw/windows.sh`, table by `raw/windows_table.py` in `raw/windows.txt`:
the element report's scenes (planets 1-3, guarding and squad-mode hands),
each played to T (25 and 40 s), then every squad loads the variant's deck
and fights 10 s. Plain, torch and blast are the element report's windows
(p1g2's plain and torch re-run here match them to the walker); scorch is
plain bullets with the scorch trail; tinder-scorch and tinder-torch add
Tinder to it and to torch bullets.

| Planet (biome, windows) | torch | tinder-torch | scorch | tinder-scorch | plain |
|---|---|---|---|---|---|
| 1, meadow (6) | 350 | 391 | 381 | 369 | 230 |
| 2, barrens (2) | 123 | 159 | 162 | 157 | 117 |
| 3, fen (2; one with no kills) | 18 | 19 | 20 | 16 | 11 |
| all (10) | 491 (233 by fire) | 569 (382 by fire) | 563 (114) | 542 (192) | 358 |

- **Tinder with the torch kills more than the torch alone** in 9 of 9
  windows with kills and leaves less slime at the end in 9 of 9 (p = 0.004
  each; it survives a Bonferroni correction over this report's six
  pairings, p < 0.0083). The gain is the fire's: 382 of its 569 kills,
  against the torch's 233 of 491. **It matters most where plants are
  thin**: on the barrens 159 against 123 (the torch there is a plain
  bullet with a little fire); on the meadow 391 against 350; on the fen
  19 against 18.
- **Tinder adds nothing to the scorch trail** (more kills in 3 of 9, fewer
  in 6, p = 0.51): the scorch already burns and lights its own line, and
  what the brush adds to the fire it takes from the scorch's burn.
- **What it looks like** (the page's play clips, `raw/play.sh`: the
  p2s1-25 window filmed, the same counts): with the torch alone the
  barrens' scrub hardly burns (49 killed, 3 by fire); with Tinder the
  strips of brush from the squad's flights catch from the torch hits and
  the slime by them burns in big flame patches (78 killed, 38 by fire, 98
  brush discs laid), none of it reaching the squad.
- **No soldier was lost** in any window with the gap. Without it (the first
  run, `raw/windows-nogap.txt`), torch+tinder lost 23 of 80 soldiers and
  scorch+tinder 4.
- A note for the balance pass, not a gate: the two cards cost 4 of the
  budget, against the blast's 3, and kill more than the blast in 9 of 9;
  the scorch trail alone (3) kills nearly as much (563). Nothing is
  retuned here.

## Raw

- `raw/windows.sh`, `raw/windows_table.py`, `raw/windows.txt` (with the
  gap round the soldiers), `raw/windows-nogap.txt` (the first run, brush
  from the muzzle), `raw/windows-alonggap.txt` (the gap along the leg):
  the windows. Plain, torch and blast are the element report's windows
  (`../gun-element-2026-11-17/raw/windows.txt`), checked here on p1g2.
- `raw/film.sh`, `raw/play.sh`, `raw/make_clips.py`, `raw/counts.json`:
  the crossing clips (frames from `larder-world look=DIR map=planet:1:3
  hand=idle rounds=plain cross=LINE,30 centre=177,177,4` on llvmpipe) and
  the play clips (`map=planet:2:1 hand=squadwork load=25,DECK
  centre=70,80,5`: windows.sh's p2s1-25 window, the same counts).
- `page/index.html`, its clips and `armory-trails.png` (`larder-world
  ui=200% "menu=shop@Trails and shoves@Standard tinder"`): the Artifact
  page's source.
