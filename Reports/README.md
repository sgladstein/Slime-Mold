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
  1,800 walkers 24%, the census 4.5%.

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
- [pages.md](pages.md) — every Artifact page made for the game, and where its source is kept.
- [dead-ends.md](dead-ends.md) — the mould's tried-and-reverted mechanisms.
