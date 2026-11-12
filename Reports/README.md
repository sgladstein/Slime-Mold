# Reports

Everything written for the mould game, newest thinking last within each
group. Lines marked **(from Pixel_Physics)** are the index entries as they
stood there on 2026-11-10, when the game moved; the rest were added with the
move.

## Reports and their folders

- [mould-probe-2026-11-07.md](mould-probe-2026-11-07.md) **(from Pixel_Physics)** — **probe built,
  playtest fixes in (2026-11-08), waiting on the owner's play.** A Jones-model
  slime mould herded with a lamp, oats, salt and snips (`--bin mould`). The
  lamp now pushes a tube aside instead of erasing it (join lost while held on
  it: 5 of 5 seeds before, 0 of 5 after); oats come back, food turns up at
  0:30 and 0:55, and the storm is warned 5 s ahead and aimed at the worst
  cut. Over the same seven scripted hands, doing nothing fell from 2 stars to
  0 and a random hand from 6 to 3, while the plan rose 6 to 8 and a fresh
  player's script 6 to 9. **Owner played it (2026-11-08): "the most promise
  so far", but needs instruction and a challenge.** The game now opens on a
  six-step lesson and a briefing card, labels the storm and its scar, and
  no longer flashes grey (about 420 cells a second to 0). Also records why
  the storm is one cut, not three. **Three challenge dishes and a dish
  picker (2026-11-08):** Behind the Wall, Beat the Ruler, Running Dry, each
  meeting its ship bar over 5 seeds (do nothing 0, random at most 1, the
  scripted intended hand 3); strays limited on hand-made dishes, which
  doing nothing had used to join all three in Beat the Ruler.
- [mould-keep-alive-2026-11-10.md](mould-keep-alive-2026-11-10.md) **(from Pixel_Physics)** — **built, waits on the owner's play: an endless
  dish, and water and sleep so the tools work on each other.** Keep It Alive is Running Dry at 22.5 degrees with no bell;
  piles land further each time and the score is how long it lived. The lamp dries the ground and dry mould sleeps through
  a famine; a drop of water wakes it by the next pile. Over 16 seeds a hand using lamp, oats and water lives 2:51-10:42,
  longer than an oat-only hand and random on 15 of 16 each; doing nothing and random get 0 stars on all 16.
- [mould-wild-one-2026-11-10.md](mould-wild-one-2026-11-10.md) **(from Pixel_Physics)** — **built, waits on the owner's play: a wild second
  mould that steals food and fights at the seam.** A whole second dish under the same rules, nobody steering, in violet;
  both moulds eat from one table per pile and kill each other where they touch. In THE WILD ONE, all four tools end
  bigger than it on 16 of 16 seeds (mostly 2 stars) where doing nothing and random get 0 stars on 16 and 14; lamp and salt
  each win alone, the snip does not. Costs about +1.4 ms a frame. **Since review 2: scored on the share of the food
  eaten** (57%, 62%, none left): doing nothing and random 0 stars, a clumsy tool hand 1, an exact one 2-3; late seam
  piles tried and taken out; open next steps listed at the end.
- [mould-game-2026-11-08/](mould-game-2026-11-08/README.md) **(from Pixel_Physics)** — **the thinking around the mould game, kept so it
  is not lost (owner, 2026-11-09):** the five-agent brainstorm after his first play, the overnight build's briefs and
  results (dishes, Wanderer, Organ, the Night look and its blind key), both adversarial reviews, the HTML of every
  picture page shown to him with links to the pages, and the 2026-11-09 depth reports: Jenson, Cherouvim and Jones 2010
  against our model, and how tools, events, ground and the mould's responses could interact for discovery. Proposals,
  not built.
- [mould-real-world-2026-11-09.md](mould-real-world-2026-11-09.md) **(from Pixel_Physics)** — **design exploration, no build: what the mould game
  looks like out of the dish.** The shipped mould dropped unchanged into a side-on slice of the lab box grows one runner
  along the ground (4 of the 5 surface piles beyond home in 10 s, untouched), because the world's dead matter lies on the
  surface; with five piles buried 12+ rows down it branches into roots (9 of 10), where the same ground without buried
  food stays a runner. From above, a forest floor keeps the dish's network but is not the engine. Records an edge-of-window
  trail bug the review caught (`Dish::spread` skips the border). Recommends a side-on "mould under the garden" probe in its
  own binary, top-down as the fallback. **The owner picked the forest floor from above (card, 2026-11-10 16:04Z)**;
  §5 lists what it needs here. Not built.
- [mould-review-2026-11-10.md](mould-review-2026-11-10.md) **(from Pixel_Physics)** — **independent review, no build: how to make the mould game
  fun and good-looking.** Played from written hands on the mould thread's unlanded branch. At 45 degrees the mould obeys
  every oat and then freezes (far pile joined by 15 s with 3 oats; every move the same move); at 22.5 it joins all the food
  untouched and the lamp's effect is lost in its own rewiring. Running Dry over 8 seeds: doing nothing loses at both
  settings, a random hand lives on 1 of 8 at 45 and 6 of 8 at 22.5, the scripted plan 8 of 8 at both. Recommends, ranked
  with pros and cons: a keep-it-alive dish that does not end, player marks that stay (lights set down, salt rubbed out),
  drawing the real walkers and thinner tubes; plus design writing and a table of comparable games. Waits on the owner's pick.
- [mould-review-2-2026-11-10.md](mould-review-2-2026-11-10.md) — **independent review, no build: Keep It Alive, water and sleep, and The Wild
  One work as built but are not yet games a person can get better at.** About 270 scripted runs with the hands made
  clumsier and slower on purpose; every figure in both dishes' reports reproduces. Keep It Alive: water is the mechanism
  and the hand is not flattered by precision (the clumsiest hand beats the oat-saver 8-0), but lives fall in two humps
  (17 of 32 dead by 5:05, 14 past 7:20) and the same seed flips between them, so the 5:30 star was earned once in 32.
  The Wild One: the first star is the opening salt wall (letting go at 30 s keeps it on 8 of 8), the second and third need
  aim within about 2 cells, the snip delivers nothing, and the bell rings while both moulds are crashing (the player's
  loses 28-54% in the last ten seconds). No bug found in the two-mould step. Recommends scoring the share of food eaten.
- [mould-keep-alive-luck-2026-11-10.md](mould-keep-alive-luck-2026-11-10.md) — **measurement, no build: what puts a Keep It
  Alive game in the short or the long hump.** 48 traced games (16 seeds x three water hands, `PILETRACE=1`): the first pile
  that lands on mostly-asleep mould decides it. A sleeper within 45 cells of it: long 13, short 2; none: long 7, short 26
  (Fisher p 0.00003, exploratory, cut named after looking). Luck because the landing shows 5 s ahead, after the mould is put
  to sleep. Proposes a 20 s preview with a hand that uses it, measured on fresh seeds.
- [mould-review-thread-handoff-2026-11-10.md](mould-review-thread-handoff-2026-11-10.md) — **hand-off note, nothing built: what
  the review thread knew when the work moved to a new account.** The one open owner decision (what to fix first after
  review 2), the next steps in order, the owner's words in that thread with their times, the looks-pass review in words,
  how to re-run review 2's probe on this repo (two baselines reproduce on `e242d2d`), what nobody has checked, and three
  mistakes the review made.
- [mould-looks-2026-11-10.md](mould-looks-2026-11-10.md) **(from Pixel_Physics)** — **built, drawing only: the mould game's looks pass.** A speck
  per real walker, one colour ramp for the food the tube holds (rescaled: the old reading was full on over half the tube),
  smoothing of the rim, tube insides, salt walls and cuts, a fading stain where mould has been, the lamp drawn as the light
  the simulation shines, and narrow tool tiles so the bar stops overlapping. All on by default, each behind a key (F3-F6).
  Same dish drawn both ways on the page; costs about 1.5-2.5 ms of drawing a frame at scale 4.
- [rts-design-2026-11-10.md](rts-design-2026-11-10.md) — **design review, no build: how the simulation could become a
  real-time strategy game.** About half an RTS already runs in The Wild One (shared piles as contested resources, tubes
  as supply lines, trail repel as borders, fights that favour a fed front); proposes Bloomfront, an indirect-control
  RTS where every tool is paid from the food store, with wells, announced blooms, spores and species as `Params`
  presets. Measured here: two moulds step at about 1.1 ms a step. Recommends a cheap Stage 0 (a scripted hand steers
  the wild mould on a mirrored dish) gated on equal hands splitting 45-55% and a sharp hand beating a clumsy one.
  **Brainstorm added the same day** ([`rts-design-2026-11-10/brainstorm/`](rts-design-2026-11-10/brainstorm/README.md)):
  six agents' notes (Creeper World and Particle Fleet, RTS craft, 20 simulation games, cards, and two clean-slate
  agents that set the old tools aside), a verification pass, and a synthesis. New directions: a contagion war over a
  neutral mould, timed food waves, road tension, pattern counters. Correction: the food store sits at a median of 0,
  so spending needs its own currency. **Red team the same day** ([`rts-design-2026-11-10/red-team/`](rts-design-2026-11-10/red-team/README.md),
  harness, raw output and stills kept): road tension fails (raising the pull shrank the network to a third in 5 s on
  4 of 4 seeds; lowering it starved the mould); a squeeze from home moved food share by 0.000 on 3 of 3 seeds; the
  pattern counters are a straight ladder in 164 games at today's density; flow on two moulds costs 2.6x the step
  time. Food share mostly counts heads on piles. Strongest left: Two Fevers. First test: a paired-clone verb census.
  **Field retest and verb census the same day** ([`rts-design-2026-11-10/field/`](rts-design-2026-11-10/field/README.md),
  after the owner said 7 piles limit the options): on 23 smaller mirrored piles (about 6,600 walkers, Jones's
  density) the counters form a cycle on one layout (hunter > maze 32/32, maze > cord 30/32, cord > hunter 25/32, all
  surviving Holm) but not on a second (cord v maze 21/32, p 0.11); Taut's pull wins share on 8 of 8 seeds and 16 of 16
  in the census (median +0.139), so it is a lever there but, as one dial, always best at full; with no hands the side
  ahead at 60 s keeps the lead in about 80% of 700 games. Verb census, 16 seeds a dish: on the field a salt line
  across the rival's road moves share 0.02 on 14 of 16 and beats random 14 of 16; on The Wild One only the lamp
  moves it (15 of 16 up, 11 by 0.02); placed food loses on both. 700 of the games ran on four runner threads.
  **Layout sweep and comeback test, later the same day** (in `field/`, with `sweep/` and `comeback/`): the counter
  cycle holds on 0 of 8 new layouts (1 of 10 in all; cord usually simply wins), 768 games on runner threads; the side
  behind at 60 s turns 6 of 12 lost games with one salt wall across the leader's road and 9 of 12 with one every 15 s
  (both beat a random wall 16/16), and 12 of 15 and 13 of 15 on a symmetric Wild One; the lamp and the cut never do.
- [rts-skirmish-2026-11-10.md](rts-skirmish-2026-11-10.md) — **the field skirmish, a playable RTS demo, and what wins
  it** (`src/mould/skirmish.rs`, `web/`, harness `duel` mode; folder `rts-skirmish-2026-11-10/`). v1 (habits, salt,
  lamp, a rival that counters your habit) is playable in the browser; its rival beat an idle hand 16/16 and lost 14 of
  16 to a hand that counters at once; the owner played it and found no strategy (21:14Z). v2, the budget skirmish: a tithe of each
  bite into a bank, spent on beacons (`Dish::beacons`), spores (`Dish::plant`) and habits, with late food. Nine
  scripted plans, 576 games on four runner threads: soft (beacons then spores on the rival's thinnest pile) beat all 8
  others, 127 of 128 games; no cycle among 36 pairings; 300 games ended in a wipe-out. A guard, a spore reach and
  sleeping spores did not stop it. Round 3 (a 5 s rally, a rising spore price, spores asleep abroad, plans trade, keep
  and ward; 720 games): soft still lost no pairing, 124 of 144 games. The left mould's edge (14 of 16 mirror games) was
  the piles sitting half a cell off a mirror, fixed. Rounds 4-6 (changes to the model, 3,136 games paired against the
  control): a holder's bonus made the snowball worse, supply lines did nothing, size upkeep eased it most but let doing
  nothing tie the best plans, and desperate fights (starving walkers fight harder) eased it with the tools intact and
  are now the page's default; soft and keep stay on top under every rule. Half the food is gone by 1:00, so even with
  no snowball the side ahead then would win about 70%. Rounds 7 and 7b (the economy sketch, 2,240 games after two bug
  fixes): food after 1:00 (a 4:00 match, regrowing piles) goes to the side ahead at 1:00, which still wins 80-85%;
  regrowth cuts wipe-outs (132 against 217 of 448); a held-ground score changes nothing. A strike counted down 5 s in
  view, with a Fortify that kills attackers in its ring, dethrones soft (brace 15/16, keep 14/16), but brace (strike,
  and fortify what is struck) then wins every pairing. v2 is playable in the browser against any plan and opens on
  Strike and Fortify (pages *Mould Budget Skirmish* and *Budget Skirmish Strategies*).

- [jenson-physarum-2026-11-10.md](jenson-physarum-2026-11-10.md) — **research, no build: why Sage Jenson's physarum looks
  so much more alive than ours, what contagion is, and a survey of other slime mould simulations.** Same Jones rules as
  ours, but 5-10 million GPU particles with no one-per-cell limit and the trail drawn at full resolution, against our
  400-2,200 walkers on 256 x 160. Jenson never published the code; Bleuje's port of *36 Points* is the closest. Contagion
  is a state particles pass on through an "infected trail", the simulated pulse wanted since 11-09, not built. Surveys MCPM
  and Polyphorm, Tero 2010, Lague, Ghassaei and Webb's list; five experiments, cheapest first. Live doc linked inside;
  the owner's PDF of Jenson's post (text and all five pictures) is in [jenson-physarum-2026-11-10/](jenson-physarum-2026-11-10/).

- [gpu-plan-2026-11-10.md](gpu-plan-2026-11-10.md) — **plan, no build: the mould at scale, GPU included.** Six gated
  phases: CPU foundations (dish hash, a "GPU semantics" mode and a 4 Hz, two-frames-late census tested paired on every
  dish, integer-or-float and scaling-rule choice, new rules all off by default); 1024 x 640 with about 100k walkers
  offline with the economy on, gated on paired seeds, plus a GPU timing test on the owner's machine; a GPU sandbox on
  pixels' wgpu device; game rules on the GPU with the census on the CPU from a fine readback, labels sent back; choose,
  port and retune the dishes; new play from scale. Revised after an independent review
  ([gpu-plan-2026-11-10/review.md](gpu-plan-2026-11-10/review.md)): the census feeds the economy every step, a 4x finer
  grid slows the mould and shortens its smell like the sensor-reach dead end, and cloud sessions have no GPU, so an
  all-integer simulation is recommended. Single-core step cost: 1024 x 640 with 100k walkers 11 ms, with 1M 97 ms;
  2048 x 1280 dropped (111 ms). Research status, risks, and seven owner decisions. *Phase 1 progress*: the step's
  time by pass (`Dish::step_cost`, printed by `mouldprobe`): diffusion is 62% of the simulation (0.69 ms a frame), moving
  1,800 walkers 24%, the census 4.5%. Then three GPU arms, 16 paired seeds on every scored dish
  ([census/](gpu-plan-2026-11-10/census/)): a census 4 a second and two frames late is safe (no change survives
  Bonferroni); fixed-point trail and scent change play, because floats keep a smell too faint to draw over half the dish
  and walkers past it fly straight (random play reaches the food behind the wall on 5 seeds of 16), so the fields stay
  `f32`; moving all walkers at once cuts seam fights and costs The Wild One's warden its share on 15 of 16 seeds
  (p 0.0005). The owner (20:55Z) ruled the current dishes unlikely to be the true games, so later gates use engine
  measures on a neutral dish. Then the scaling rule and phase 2's big dish on the CPU ([scale/](gpu-plan-2026-11-10/scale/),
  engine-only `mould::big`, 16 paired seeds): every number in today's units gives no network (today's net comes from its
  coarse grain); four fine steps a tick costs 3.7x and is worse on every measure (rejected); the plain 4x port plus food
  smelled at today's reach is chosen (64.5 loops against today's 3, strays 2.2%, 11 ms a tick on one core), though a
  packed home blob stays a bag (3 piles at 20 s against 4). 100,000 walkers make 164 loops at 10 s.
  The owner picked 100,000 walkers (22:32Z). The GPU timing test (`examples/gputime.rs`, four wgpu compute passes a
  tick) is built; on a software GPU its dish matches the CPU's (165 loops against 163 at 100k, 10 s) and it waits for
  the owner's GTX 1070. *Since 11-11:* run there, 1.37 ms for a frame of two ticks at 100k (the gate's timing part;
  raw recovered by the review into `gpu-plan-2026-11-10/raw/`); the independent review's corrections are dated in
  the plan (one seed, no raw for phase 1's step table).

- [jenson-compare-2026-11-10.md](jenson-compare-2026-11-10.md) — **research with small tests, no game change: why
  Jenson's clips look different from our mould.** Owner asked 22:48Z. Jenson's GIFs against the big dish (no food), and
  the step as Bleuje, Lague, PolyPhy, gpu-io and Barradeau build it (all drop the one-a-cell limit). With our numbers
  the limit off folds walkers into ropes (loops at 10 s 151.5 to 0, 8 of 8 seeds); sensor 45 / turn 22.5 / reach 18
  gives polygons; the game's brightness curve makes tubes look about three times wider than a soft curve does; respawn 1% a
  step (new `BigParams::respawn`, off by default) held a fine mesh (loops at 10 s 43.5 to 196, 8 of 8) and gave
  Jenson's haze, for 4% more step time. Not reproduced: feathery fronts, trunks, contagion.
  Page, pictures, run.sh, analyze.py and raw output in [`jenson-compare-2026-11-10/`](jenson-compare-2026-11-10/).
- [world-look-2026-11-11.md](world-look-2026-11-11.md) — **built: the Larder world drawn
  from the simulation**: mould height, gloss and colour from its thickness, every
  walker a speck, shots that cut trail and leave a fading burn (`Engine::set_hits`),
  walls by health, crystals as six-faced cones by charge (`world::look` ground
  field), soldiers as the SVG sprite the owner picked in a blind A/B, shots that
  stop at the first slime (`first_hit`; at the visible 26/255, not 60, 98 of 120 shots crossed slime
  before, 0 after), hexagonal crystals. Bare ground as soil, rock and wet
  ground shown. View 0.8-1.2 ms on the 1070 at 1080p (1.6-1.8 before the prep pass), against 0.3 ms without.
- [world-camera-2026-11-11.md](world-camera-2026-11-11.md) — **built: the camera and UI
  for a world bigger than the screen** (`slime_mould::world`): pan, zoom at the
  cursor, minimap, GPU marks and Larder's controls as `UiCommand`s; the GPU
  engine moved into the library (`slime_mould::gpu`). In the sandbox (W, Q, M).
  Today's dish through the camera is pixel-identical; 0.3-0.4 ms a frame on
  the 1070 at 1920 x 1080.
- [world-ui-2026-11-11.md](world-ui-2026-11-11.md) — **built: the game UI over the
  Larder world** (`world::{canvas, ui, icons, demo}`): a GPU canvas pass (glass
  panels, Inter text, line icons) and a rules-free HUD (`Model` in, `HudEvent`s
  out): top bar, alerts, groups, selection with health, a command card with
  costs and cooldowns, placement ghosts, enemy hover cards, menus. 0.09-0.10 ms
  GPU in play on the 1070 (0.27 ms with the help page); a 480-pixel ring mark
  had cost the view 0.7 ms, now lines (view 1.04 -> 0.295 ms). An independent review
  found 9 input bugs and an overflow that dropped text; all fixed, 12 tests.
  Page and pictures in [`world-ui-2026-11-11/`](world-ui-2026-11-11/).
- [gpu-sandbox-2026-11-11.md](gpu-sandbox-2026-11-11.md) — **built: GPU plan phase 3's sandbox,
  `--bin mouldgpu`.** The big dish on the GPU, drawn live, with every option
  the Jenson comparison tested as a control and its dishes on keys 1-8 (the
  owner's ask, 2026-11-11 00:06Z). On a software GPU its dishes match the
  CPU's at 10 s, seed 1 (100k one a cell: 148 loops against 151; 1M with
  respawn: 200 against 200), and the same seed gives the same frame twice.
  On the GTX 1070, 100k walkers take 1.3 ms of GPU a frame with the drawing.
  The twin test (one GPU tick against one CPU tick) runs in CI on software
  Vulkan. *Independent review (11-11):* fed scent and jumps after the step
  were missing on the GPU (key 2 strays 3.6% against the CPU's 0.8-1.8%;
  key 5 width outside all 8 CPU seeds); fixed, and on 8-16 seeds on both
  GPUs no measure differs after Bonferroni over 20. The window's slow
  frames were the GPU's clocks stepping down at light load (797 MHz, memory
  810 MHz); the Jenson settings cost twice ours in the sense pass. Seed
  tables, per-pass costs and clock logs in
  [`gpu-sandbox-2026-11-11/review/`](gpu-sandbox-2026-11-11/review/).
  *Looks as controls (11-11):* the looks report's rules (disc and haze,
  trail-set settings, deposit cap, floor, wander, contagion) on two new
  panel pages and seven presets (keys 9, 0, Shift+1-5); a new twin test
  matches them to the bit on six dishes (planted bugs: 7165 and 3046
  walkers off). Off, they cost 0.01 ms a frame on the GTX 1070; contagion
  0.3 ms at 100k; the 1M presets 3.9-9.8 ms. Videos, raw output and the
  1070's runs in [`gpu-sandbox-2026-11-11/looks/`](gpu-sandbox-2026-11-11/looks/).
  (Corrected 11-11: to the bit on four of the six; within 1.1e-7 with the
  cap.) *Outside rules as controls (11-11):* the outside rules and
  creatures reports' rules (species with bands, cyclic species,
  conversion and species 0's boost; explorers; blocked lay and keep; the
  chance turn, size spread, area sensors; grazing) on a fourth panel page,
  with seven presets (Shift+6-0, Ctrl+1-2). A third twin test matches every
  walker, age and species on fifteen dishes, fields within 2.4e-8; all 16
  planted bugs failed it. Seed 1 on the software GPU looks as the CPU's
  reports do (droplets 182 bodies against 179-193; grazing droplets drift
  7.62 cells in 2 s against 6.5-7.6). Compiled in but off they cost 18-24%
  on llvmpipe, so they are compiled out when off (then within noise). The
  1070's timing is pending. Videos and raw output in
  [`gpu-sandbox-2026-11-11/outside/`](gpu-sandbox-2026-11-11/outside/).
- [jenson-looks-2026-11-11.md](jenson-looks-2026-11-11.md) — **research with small tests, no game change: Jenson's
  three missing looks on the big dish.** Owner asked 00:06Z. New `BigParams` rules, all off by default: a centred
  starting disc and haze, 36 Points' trail-dependent settings, Bleuje's square-root deposit, a sensing floor, wander, and
  contagion. A dense disc keeps a thick ring at 20 s (8 of 8 seeds, p 0.008), as long as respawn is slow. Contagion runs
  along the tubes, coarsens the mesh while it passes (8 of 8 at 9 s) but leaves no mark by 20 s, and a knife cut does
  not stop it (4 of 8). Feathery fronts: seven mechanisms failed; any haze organises on its own. Page, pictures,
  videos, scripts and raw output in [`jenson-looks-2026-11-11/`](jenson-looks-2026-11-11/).
- [outside-rules-2026-11-11.md](outside-rules-2026-11-11.md) — **research with small tests, no game change: outside
  rules on the big dish, judged as creatures.** Owner asked 03:00Z (test Lague, species, MCPM, search wider) and 04:11Z
  (a creature for play, not a pattern generator). New `BigParams` options, all off: area sensors, species with own
  trails, zones, chance-weighted turn, cyclic species, explorers with a spread stage, size spread, blocked walkers that
  keep heading, retry or lay trail. Rival species hold territories with a clean border (avoidance cleans it 8 of 8; own
  trail alone keeps them home, 5 of 8). Explorer spread grows feathery radial veins from a disc. Packed walkers that lay
  trail where they stand make about 180 still droplets. Chance turn more loops 8 of 8, area sensors fewer 8 of 8 at
  +50% cost. Jones and Adamatzky's crawling blob failed. Survey of 14 more open-source simulations, page, videos and
  raw output in [`outside-rules-2026-11-11/`](outside-rules-2026-11-11/).
- [creatures-2026-11-11.md](creatures-2026-11-11.md) — **research with small tests, no game change: creatures that
  move and fight.** Owner, overnight: "Keep exploring". New `BigParams` options, off by default: grazing (a regrowing
  grass field walkers eat and lean toward) and species conversion. Grazing droplets crawl 6.5-7.6 cells per 2 s
  against 0.7 (8 of 8, p 0.008) and stay bodies; two species roam among each other as one-colour cells. Conversion
  alone moved no border on even dishes (seed 1); a 1.5x louder trail wins early and loses walkers by 16 s (6 of 8,
  p 0.29). Page, scripts and raw output in
  [`creatures-2026-11-11/`](creatures-2026-11-11/).
- [spread-mix-2026-11-11.md](spread-mix-2026-11-11.md) — **looks, 3 seeds, no paired test: spread from one spot,
  and mixed walkers in one mould.** Owner, 13:58Z: how far do the mould types spread from one spot, and what if
  some walkers differ? Without food only explorers cross the dish (13.8-14.0k tube cells at 10 s); the network
  curls into one ring (1.8-1.9k cells at 20 s); droplets hold. New `BigParams` mix (a share of walkers with their
  own reach, angles, turn, step, deposit, one shared trail): the kinds sort themselves into parts, such as
  outriders riding the wall, a splitter's core apart from its ring (3 of 3), a crawling knot, and grazing scouts
  that haze over the dish (r90 299-303 against 192-199). Page, script and raw output in
  [`spread-mix-2026-11-11/`](spread-mix-2026-11-11/).
- [research-review-2026-11-11.md](research-review-2026-11-11.md) — **an independent review of the creature
  research** (jenson-looks, creatures, spread-mix), asked for by the owner at 15:08Z with a big-picture brief. It
  found the work tuned to the wrong operating point: no food, a fixed walker count, the dish wall as a hidden rail,
  and no player verb. Species can't have their own settings, which blocks the ladder. The knife test cut before the
  front arrived (void); the boost story contradicts the model; spread-mix's tube share was a unit slip (6%, not
  0.4%). It gives six ranked directions with first experiments, and a list of biology that was ignored. The prompt,
  the review verbatim, and the thread's response; scripts in
  [`research-review-2026-11-11/`](research-review-2026-11-11/).
- [mould-tiers-2026-11-11.md](mould-tiers-2026-11-11.md) — **research with paired tests, no game change: new
  mould tiers and how each answers attack.** Owner, 15:07Z: "consider how the mold reacts to the player, how it
  reacts to being attacked". New `BigParams` life cycle, off by default (spores, dormancy, an alarm field the
  player's strike lays). Sporer holds 6-7 of 7 piles against 3-5; struck, it scatters (7 of 7); a baited Sleeper
  leaves 1858-2216 walkers at the strike against 2360-2971; a Swarmer loses 1144-4158 to fire against 614-3591:
  each 8 of 8, p 0.0078, survives Bonferroni. Fleeing (Skittish) fails, p 0.375. Alarm +31% a step on one core.
  Page, scripts and raw output in [`mould-tiers-2026-11-11/`](mould-tiers-2026-11-11/).
- [first-reach-2026-11-11.md](first-reach-2026-11-11.md) — **a paired test, no game change: which droplet moulds
  reach food first.** The crystal-mix test redesigned after the review (its start disc covered the crystals).
  30,000 droplet walkers from a centred disc, 12 piles, 16 seeds; far piles reached by 30 s, Holm over 5. Grazers
  49 of 77 against 9 for still droplets (more on 15 seeds, fewer on 0, Holm 0.0003); scouts on grazers 62 against
  49 (10 and 0, Holm 0.0078); fast droplets 19 (7 and 0, Holm 0.047); scouts alone and scouts on droplets no gain.
  The measure was named after the flawed run; fresh seeds 9-16 agree. Grazers' body edge moves about 10 dish
  cells a minute. Page, scripts and raw output in [`first-reach-2026-11-11/`](first-reach-2026-11-11/).
- [species-sets-2026-11-11.md](species-sets-2026-11-11.md) — **a paired test of per-species settings
  (`BigParams::species_sets`, PR #61): a droplet species and a network species on one dish.** The review's
  direction 2; the ladder needs species with their own rules. 29,870 grazing droplets (left) beside 1M network
  walkers (right), avoidance 1, no food, 8 seeds, 60 s. Each species' own area together over alone is within
  0.5-2 on 8 of 8 (Drifter 0.99-1.02, Sprawler 0.65-1.90; on 3 seeds the network's tubes cross into the droplets'
  half). Secondary: fewer droplet bodies beside the network at 60 s, 8 of 8. No fight without a contact rule. The
  second species adds about 10% a tick. Raw output and scripts in
  [`species-sets-2026-11-11/`](species-sets-2026-11-11/); page in `first-reach-2026-11-11/`.

## Added with the move (2026-11-10)

- [new-game-brainstorm-2026-11-07/](new-game-brainstorm-2026-11-07/) — **where the game came from.** The flows,
  gas and growth brainstorm that produced all eight prototypes: the brief, five explorers, three reviews, the
  prototype briefs (`prototypes/2-mould.md` and `common.md` are the mould's), and the 2026-11-08 playtest
  (`playtest-2026-11-08/mould-review.md`). Other games' material is kept with it because the reviews compare them.
- [mould-game-2026-11-08/](mould-game-2026-11-08/README.md) now also carries the pictures that were only in the
  project folder: the toy-feel sheets (`brainstorm/toy-feel/`), the second review's stills
  (`reviews/review-2-pictures/`), and every prototype round's GIFs, PNGs, WAVs and video (`prototypes/*/`).
  [`depth-2026-11-09/PAPERS.md`](mould-game-2026-11-08/depth-2026-11-09/PAPERS.md) cites the two Jones papers, and `depth-2026-11-09/papers/` holds them (PDF and text).
- [mould-review-2026-11-10/](mould-review-2026-11-10/) now also carries `hand-fixed.txt`, the from-the-simulation
  look mock (`mouldraw.rs.txt`), the review hands patch for `mouldprobe` (`mouldrev.patch`: aim error, slow
  reactions, one-move openings, a two-mould invariant check), and the page sources and images of the Looks Review
  and Sim-True Look pages (`pages/`). `mouldhand` itself is now `examples/mouldhand.rs`.
- [mould-scratch-2026-11-10/](mould-scratch-2026-11-10/README.md) **(from Pixel_Physics)** — **raw census behind Keep It Alive, Water and
  Sleep, The Wild One and Finer Tubes**: the four pages' HTML and images, the census runners, `w5.py` (shares and
  paired sign tests), and every raw probe output of the wild-mould tuning arms and the sensor-reach sweep.
- [llm-wiki-plan-2026-11-10.md](llm-wiki-plan-2026-11-10.md) — **built in part: an LLM-maintained wiki
  (`wiki/`), twelve pages so far.** Karpathy's LLM-wiki pattern judged a fit at small size (10 core reports, filed
  by date, so the truth about one subject is spread over 9-54 files); design, rules and workflows; a sub-agent
  review of later reports on the pattern (one measured: long context files cost over 20% more for little gain)
  and of the plan, which cut `log.md`, kept open decisions only in `HANDOFF.md` and put the lint in CI. Success
  check (25 questions, blind-scored, files in `llm-wiki-plan-2026-11-10/check/`): wiki and reports both 0 wrong
  numbers, 22 vs 25 fully correct (0-3, p 0.25), 9% fewer tokens; summaries alone 9 not found. At ten reports the
  wiki ties reading them; re-run when it doubles.
- [pixel-physics-review-2026-11-10.md](pixel-physics-review-2026-11-10.md) — **what to take from Pixel_Physics,
  and the dish hash built.** The frame is 4.93 ms, sim 1.10 + draw 3.83, so the multi-core renderer is the engine
  piece worth taking; sound, keyed random draws and empty-tile skipping next. Tools: determinism test, profiler,
  licence gates, `crispfont.py`. Misfits: capitals-only font, missing OFL notice, unmeasured release profile.
  `Dish::state_hash` and `every_dish_replays_bit_for_bit` built: all seven dishes replay bit for bit, and a
  clock-seeded draw turns the test red.
- [mould-sound-2026-11-10.md](mould-sound-2026-11-10.md) — sound for the game (owner: "yes to sound", 19:22Z). Pixel_Physics's
  listener / voice bank / device split with the mould's own sounds, every one a count off the dish; S toggles.
  Same game bit for bit with sound on; listener about 1 us a frame. Seven recordings on the Mould Sound page.
- [two-fevers-2026-11-10/README.md](two-fevers-2026-11-10/README.md) — **Two Fevers, a playable browser demo**
  (`src/mould/fever.rs`, `fevers-web/`). v1 scored carriers: rings swept the host and left it as before; owner
  found no strategy. v2 (owner's card): walkers keep the last fever's colour, a 15 s hold, Hush. 16 seeds, both
  colours: plan beats random 16/16 in both versions but by 0.536 / 0.537; without the hold random wins 14/16;
  Hush when behind 10/16 (noise). 0.5-0.64 ms a step. v3 (owner's card, 22:06): calls move your own walkers,
  fever by tube only, foraging income, starvation, free pinch, fed bodies resist. Census: trunk > web > raider,
  a ladder not a cycle (only trunk > raider survives Holm); trunk and web beat random 15/16, raider 7/16;
  mirror 1:30 leader kept the lead 32/32 (snowball). 0.70-0.84 ms a step.
- [rts-research-2026-11-10.md](rts-research-2026-11-10.md) — **how RTS games work and are made, from 22 outside
  sources** (Sirlin, Johnson, Browder, Shelley, Meier, Company of Heroes, Tooth and Tail, Creeper World, lockstep
  engines, AlphaStar, the spinning top), lessons for the mould, and feedback on both demos: soft and Trunk are
  dominant strategies and the snowball is unmanaged positive feedback; fix with a size cost, a pull-back rule, a
  points ending and telegraphed attacks before adding verbs. Reading only, no runs. Cards in `research/sources/`.
- [growth-2026-11-11.md](growth-2026-11-11.md) — **Growth and starving on the big dish (Larder World W0)**: walkers eat, pay upkeep, divide and starve, in fixed slots paired by one keyed mask a step; on the GPU it matches the CPU exactly. Fed 96,358 against starved 428 alive at 30 s (8 of 8, p 0.0078); size nearly linear in food (1.9x a doubling); a walker lives about 3 s; CPU cost within noise. Open: tubes between point crystals may starve.
- [species-gpu-2026-11-11.md](species-gpu-2026-11-11.md) — **The species rules on the GPU (Larder World W3a)**: species 0's share, conversion over a square, a second kind of walker in one mould and each species' own settings on `gpu::Engine`, off by default. One GPU tick matches one CPU tick on 12 dishes and a growth dish (every kind exact), every row sum of the square matches the CPU's, and all 28 planted bugs are caught (27 of 28 before the row check). Off they cost nothing measurable on the software GPU; the square of radius 8 costs about a trail pass a tick there (rows 24-25 ms against trail 18). A W1b fix: a wall built with growth on moved the empty slots under it (195 under a test wall, 349 counts off; now 0). The 1070's timing is to come. Scripts and raw output in [`species-gpu-2026-11-11/`](species-gpu-2026-11-11/).
- [mould-shape-2026-11-11.md](mould-shape-2026-11-11.md) — **What shapes the mould (owner's questions, 16:01-16:05Z)**: food sets the size, what walkers follow sets the shape. Grass and crystals in one world (seed 1): crystals alone give roads, grass that never regrows a boom and bust (dead by 40 s), regrowing grass a steady fine net, piles that only smell a coarse one; full grass swamps the crystals. Walker count matters only with the crowd limit: off, 32k-1M all fold into strands (0-1 loops at 30 s); on, 32k/100k/200k give 43-54/57-70/120-144 loops (8 of 8 each, p 0.0078, Holm). On crystals size follows intake (crystal area x bite), not charge; scratch `energy_flow` turns the hoard into 15-24% more walkers.
- [larder-ai-2026-11-11.md](larder-ai-2026-11-11.md) — **Larder player AI**: plays through `Cmd`s only (every round a replay), reads a `View` of what a player sees, commands a `Side` (ready for a second soldier team); five styles on knobs plus idle and random. Held out (17-32), boom, lure and adapt beat idle and random 16/16 in both fields; turtle and rush lose to idle 16/16. Fixed: buying unarmed soldiers, bait while hunting. A harness to port to the world, not a balance result.
- [larder-m3-2026-11-11.md](larder-m3-2026-11-11.md) — **Larder M3, full controls and soldiers that fight from range**: groups, scent view, replays to the same hash, a headless frame shot and bench. Firing from range, spread shots and a loose formation loosened the ladder: on held-out seeds no plan is best on 13+/16 (reader 8 fed, 10 lean; was 13 and 12), two pairs flip between fields but neither survives Holm. Desktop frame 7.2 ms mean, 9.6 p95 at 40 soldiers. Spores and a dormant crust were tried as mould answers and rejected.
- [larder-maps-2026-11-11.md](larder-maps-2026-11-11.md) — **Larder World maps (WM2, WM3)**: a first plan led by height (five paper maps, ground kinds ranked), then, after the owner's challenge (20:06Z), height parked and the ground tested: `examples/worldground.rs`, 16 paired seeds, a 30% patch of each kind against plain. Every kind changes the mould: wet (step x 0.5) halves it (7,165 to 4,044 alive at 60 s, 16/16), barren (upkeep x 3, no births) halves it (15/16), sand (trail x 0.8) makes it bigger (11,462, 14/16) though walkers leave sand, clay (trail x 1.05) traps it (eats less 15/16). New `BigParams::sand_keep`, `clay_keep`, `barren_upkeep` on `BigDish::soil` (off while empty). Maps rebuilt flat (Rim, Fork, Flats, Nest, Passes) with loader files in `maps/`; page `page.html`.
- [larder-world-roadmap-2026-11-11.md](larder-world-roadmap-2026-11-11.md) — **Larder World roadmap**, its three independent reviews (skeptic, expander, feasibility) and W1a (crystals that run dry, rock and wet terrain, a rectangular world on the big dish, CPU; cost within noise): the owner moved Larder into a persistent GPU world with energy crystals that run out and several mould species. Six gated phases (W0 growth on the GPU first), the thread split (GPU, creature, Camera and UI, Player AI), landing vs race starts, open questions. Walls the mould wears down (default health 400: a caged mould breaks out by 20-30 s; a passing tube never breaks one) and crystal charge that flows inside a crystal (default 0.01: share eaten 0.476 → 0.829, 12/12 seeds, p 0.0005); page `page-walls.html`. With growth (scratch): charge sets how long the mould lasts, not how big it gets (512 gives an idle mould 10-12 minutes on 512 × 320); any wall costs it about a tenth of its size at 60 s and a wall on its road is no better than a random one; walls now leak 0.9 of their scent (a sealed ring was ignored; leaky, it was bitten on 7 of 8 seeds). Crystals are hexagons (owner, 16:48Z). **W1b**: the crystals, terrain and walls on the GPU engine. One GPU tick matches one CPU tick on nine world dishes, a GPU-built wall matches the CPU's, all 17 planted bugs are caught, and on the GTX 1070 the world adds 0.25 ms to the frame at 1024 × 640 (1.73 to 2.11 ms); raw in `w1b/`. Then the **W2 plan**: engine hooks, M3's rules as a game layer, the window. **W2a**: the squad's hooks (shots that stop at the first slime and strike, smell, touch, aim, harvest) on the CPU and GPU, one frame equal exactly, all 16 planted bugs caught; 8 to 40 units add 1.0-1.5 ms on the 1070, most of it one thread a unit in the units pass; raw in `w2a/`. **Maps** (owner, 19:29Z): ROADMAP WM1-WM5. **W2b**: M3's squad rules on the world (`mould::larder_world`, `gpu::larder_world`): soldiers walk distance maps round rock and fire only along a clear line (the first census had a rush put 196 of 200 shots into rock); on held-out seeds 5-20 idle and random lose 16/16 and farm and wall beat both 16-0, no hand wipes the mould, one wall adds nothing over farming, a rush dies; the GPU world runs `scale` engine ticks a game tick and stays within 8% of the CPU's mould over 20 s; page `page-w2b.html`, raw in `w2b/`. The ladder table gains Sporer, Sleeper and Swarmer. **WM1**: the map loader (`mould::map`, `BigDish::with_map`, `GpuWorld::with_map`): each crystal its own size and charge, the walkers shared among the map's starts, the larder on its landing, walls at their own health; the v2 Fork and Nest play within 1% on the CPU and GPU, and a saved noise world loads back hash-identical. The maps found two game bugs, fixed: a soldier waited forever at a dry crystal, and a blob by the rim placed 3,752 of 5,000; raw in `wm1/`. **W2c** (first cut): the game in a window, `cargo run --release --bin larder-world -- map=fork`, W2b's rules on the GPU world on a v2 map with the camera, look and UI; the owner's `mouldgpu` playtest had found the UI's rule-less stand-in squad. Two fixes rode along: no specks for dead walker slots, and the game's shots leave the look's wound. Pictures in `w2c/`; the frame time on the 1070 is not measured yet.
- [larder-m2-2026-11-11.md](larder-m2-2026-11-11.md) — **Larder M2, spending and the mould's answers** (buy soldiers, bait, turrets; hoard smell, hunger, drops): the gate fails on held-out seeds 17-32. No plan crosses over between a fed and a lean field; the reader beats every plan but the farmer in a fed field (13/16) and the baiter in a lean one (12/16). Ablations: hoard smell is what makes saving cost (off, fed reader -0.30 to 0.17), drops are what keep a lean mould alive (off, it starves 16/16), hunger barely acts. Game layer 0.053 ms a step at 40 soldiers.
- [larder-m1-2026-11-11.md](larder-m1-2026-11-11.md) — **Larder M1, soldiers and rules in the harness**: fortress and farmer beat idle and spend-matched random 16/16 on held-out seeds; hunter (4 soldiers attacking) loses 16/16; no mould wipe in 80 runs; squad layer 0.098 ms a step at 40 soldiers. Code `src/mould/larder.rs`, `examples/larderm1.rs`, window `src/bin/larder.rs`.
- [larder-build-plan-2026-11-11.md](larder-build-plan-2026-11-11.md) — **Larder build plan for the RTS demo** (Claude Doc copy), three independent reviews and their prompts (`larder-build-plan-2026-11-11/reviews/`), and the plan revised after them: crossover gate in place of an impossible counter cycle, bait and turret, killable mould core, one ordered score.
- [mould-shooter-2026-11-11.md](mould-shooter-2026-11-11.md) — **the mould as the enemy, a shooter brainstorm**
  (owner, 11-11 00:18Z). Lead shape Larder: a squad and the mould want the same food; stored food smells, so a
  hoard draws the mould; shots cut supply by holding a tube clear, not by kills (the knife is weak); four
  strategies meant to counter through the mould's lean or fat state. **Tests run** (`examples/larderprobe.rs`, 16
  seeds): a held ring of fire round the larder saves 0.5% (16/16), firing on the mould eating it saves 90% (16/16):
  a fed pile feeds its own eaters, so cuts only hurt mould with no food; a trail-only gun does nothing; smell and
  lean-vs-fat show 16/16 but tiny effects, because the mould crosses today's dish in about 4 s. **Correction:** a re-fired
  snip did not refresh its wound; fixed, a ring held from the start stops all theft (16/16); three reviews in `reviews/`.
  **Priced ring** (`econ`): ring-forever never best; ring-then-mow best on 16/16 at prices 10 and 40; a ladder.
- [pages.md](pages.md) — every Artifact page made for the game, and where its source is kept.
- [dead-ends.md](dead-ends.md) — the mould's tried-and-reverted mechanisms.
