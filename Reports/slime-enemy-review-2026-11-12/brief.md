# Shared brief: the ideal slime enemy, and how to get there

You are one of five reviewers. The owner (Scott) asked for a design document on the
ideal slime enemy for Larder World, an RTS-style game where a squad of soldiers fights
a slime mould that covers the map. Then "a deep review/thought on how we expect to
achieve that", and the owner's lead question for every reviewer:

> "Is the Jones mechanics a good backend for what we want, or will we never achieve it
> with this approach?"

Answer that question from your area's angle first, then your area's own questions.

## The owner's standing rulings (do not argue against these; design within them)

- Fun over fidelity: interesting simulation that plays, not a realistic mould. Biology
  and published models (Jones 2010, Sage Jenson, Tero et al.) are inspiration, not the bar.
  Biological realism should not be ignored either.
- The picture is the data: everything on screen is the simulation. No drawn-only effects,
  stamps, painted wounds, thresholded tube edges or fed/unfed colour lines.
- The mould is integral to play; several strategies that counter each other.
- A creature, not an organic pattern generator: a body with identity that moves, fights, grows.
- Owner playtest 11-12 04:13Z: it did not feel like one cohesive organism; a cut-off part
  should "feel like a cut off hunk and not a bunch of individuals"; shots lacked impact;
  the mould ignored soldiers.
- 11-12 14:36Z: "there are a bunch of individual specks". 16:11Z: "we didn't always see
  this in the standard jones or sage slime mold, right?" 16:20Z: "These are just forming
  single strands instead of a spread out network."
- Spread must take minutes, not seconds (11-11: filling a dish in ~10 s is far too fast).
- Difficulty comes from mould kinds (a tier ladder), not bigger numbers.
- The real game is a native GPU app (wgpu); target GPU is a GTX 1070. The owner chose
  100,000 walkers on 1024 x 640 as the GPU scale goal (11-10).

## The design goals (from the doc "The ideal slime enemy")

Fantasy: one living thing spread across the map. A glowing network: thick trunks between
food it holds, a fine mesh where exploring, a creeping front where hungry. It notices
soldiers, flows toward their scent, recoils from fire, floods back. Cut a trunk and the
far side is visibly on its own: thins, gropes for food or the body, starves or rejoins.
It grows on what it eats, so supply lines matter. Each kind plays differently.

Look: L1 continuous body (no specks); L2 a lasting spread-out network, mesh where
exploring thickening into trunks between food, not collapsing to a few strands; L3 one
organism, cut pieces visibly separate lumps; L4 intent visible (hungry fronts creep and
fan, fed body calm, flee/flood as motion); L5 damage shows as walkers dying (done);
L6 kinds differ in silhouette and motion.

Behaviour: B1 forage (explore with mesh, reinforce food paths into trunks, thin the rest);
B2 one supply (food shared through the body); B3 cut pieces stay one lump, then hunt,
creep back or starve over 10-30 s; B4 reacts to soldiers (comes for them, recoils from
sustained fire, floods back); B5 defends core and best food harder than fringe;
B6 learnable per tier.

Pace/threat: P1 fronts slower than soldiers, minutes to cross a map, trunks fast enough
to refill a gap in ~2 s; P2 grows only by eating, idle player loses; P3 damage counts mould
mass not walker count; P4 tier ladder; P5 outcomes are a spread, plans beat random.

Counterplay: strike, siege, farm, wall, plague, lure; best plan changes by tier.

## What we know today (measured this week; details in Reports/)

- Engine: Jones-style agents ("walkers") on a fine grid (`src/mould/big.rs` BigDish on CPU;
  `src/gpu/engine.wgsl` + `src/gpu/mod.rs` on GPU, twin-tested). Sensor offset 9, sensor
  angle 45 deg, rotate 45 deg (BigParams::today; race world scales offsets by scale 2), step 1, deposit 5, trail keep 0.9
  after a 3x3 mean, one walker a cell (crowd). Plus game layers: growth (walkers have energy,
  pay upkeep, starve, divide into free cells when fed), crystals (food) with a food scent
  field, walls, rock, wet ground, alarm field, tiers (`src/mould/tiers.rs`), squad shots
  that kill walkers, craters that cut trail.
- The race world: 512 x 320 fine cells (scale 2), starts 5,000 walkers at one core, 30
  simulation steps per game second (15 ticks x 2 substeps). The mould settles at 3-5% of
  cells holding a walker.
- Specks: at 3-5% density the game look shows beads/dots along trails. At 15-20% (25,000
  walkers, upkeep 0.0002) the mould is smooth continuous tubes. Drawing the trail alone at
  3-5% still shows beads, so density, not the drawing, is the main cause. A coarser grid
  (scale=1, same walkers) is also smooth but blockier, and its size swings wildly.
- Network: with walkers scattered over the whole map and no food (`start=spread food=0`),
  the engine forms Jones's spread-out mesh at 4 game s, which coarsens into a few thick
  strands by ~30 s (~900 steps). Growing from one core with food scent gives strands
  food-to-food from the start.
- One body: food sharing between touching walkers (`energy_flow` 0.05) + hunger
  (`hunger_scent` 8) gives one connected body (largest part 0.97 vs 0.88) but 2.5-3x the
  size, and breaks the gate for the rush plan. Matching its size by cutting food makes it
  live or die whole (random play starves it 9 of 16). Slow sharing alone looks no different.
- Cuts: the body refills a cut gap within 2 s (walkers ~15 dish cells/s). A held-off piece
  stays one round lump and starves in ~20 s; it does not hunt or creep back. A piece moving
  to food scatters into streams (largest part 0.27-0.58 at 20 s). Tried and failed for that:
  heading alignment, body pull toward the local crowd, more starting density. A "lone step"
  rule (lone walkers slow) holds moving pieces together (0.72 vs 0.46) but slows the
  mould's spread ~25% and the frontier is still specks.
- Damage counts walkers today; an agreed `walker_mass` factor would make it count mass.
- GPU cost: sharing + hunger +0.23 ms per 2-tick frame on the 1070; 20k walkers ~8.5 ms
  p95 frame (measured on a busy PC).

## Where to look (read only what you need)

- `CLAUDE.md` (repo rules), `wiki/index.md`, `wiki/model/*.md` (growth, scale...),
  `wiki/decisions.md`, `wiki/decisions-game.md` (all owner rulings with quotes).
- `Reports/README.md` (one line per report; grep it), `Reports/dead-ends.md` (grep for a
  mechanism before proposing it), `Reports/one-body-2026-11-12.md`,
  `Reports/jenson-looks-2026-11-11.md`, `Reports/mould-shape-2026-11-11.md`,
  `Reports/growth-2026-11-11.md`, tier and toolkit reports.
- Code: `src/mould/big.rs` (BigParams docs explain every rule and why), `src/mould/larder_world.rs`
  (game rules), `src/gpu/engine.wgsl`, `src/world/view.wgsl` (how the slime is drawn).
- Repo: /home/user/Slime-Mold. Do NOT edit any file in the repo, do NOT run cargo build or
  cargo test (another session uses the build directory). You may run the already-built
  binaries for short experiments (each under ~5 minutes of CPU; the machine has 4 cores
  shared by five reviewers, so keep to one run at a time):
  - `target/release/larder-world look=DIR hand=idle at=5,30,60 screen=1280x800 KEY=VALUE...`
    renders the real game look (GPU engine on software Vulkan) to PNGs; keys are
    `gpu::set` names (e.g. walkers=, upkeep=, start=spread, food=0, sensor_angle=, rotate_angle=,
    sensor_offset=, keep=, laid=, energy_flow=, hunger_scent=, scale=). Look at the PNGs.
  - `target/release/examples/cutprobe` (CPU; whole-mould shape and cut probes; see its
    header doc) and `target/release/examples/larderworld` (the census).
  Put any files you make under raw/<your-area>/.
- Literature: you may search the web and paper tools (Jones 2010 "Characteristics of
  pattern formation and evolution in approximations of Physarum transport networks";
  Tero et al. 2010 Science "Rules for biologically inspired adaptive network design";
  Sage Jenson's physarum work; Jones's later shape/amoeboid papers; Physarum
  "shuttle streaming"; reaction-diffusion; others you find). Cite what you open.

## What to return

Lead with the big picture, in this order:
1. Your answer to the owner's question for your area: can the Jones agent backend (as we
   run it) reach these goals, and if not, what backend or hybrid would, with the evidence.
2. Is our current approach right for your goals? What have our tests missed (controls,
   confounds, wrong measures)?
3. What biology and published models suggest (inspiration, not the bar).
4. Ranked directions (3-6), each with: the mechanism, why it should work, the risk, the
   frame-cost guess on a GTX 1070, and the first cheap experiment with its deciding measure.
Then any correctness concerns. Plain English, short sentences, numbers with units. Write
your full report to your scratchpad folder as report.md and return it as your final
message (under ~1,200 words).
