# Review: the simulation-use plan, through an artificial-life lens

2026-11-17. Reviewer: the artificial-life and agent-simulation lens (the
brief in [prompts.md](prompts.md), *Lens 2*). Plan: [plan.md](../plan.md),
draft 2, read on branch `claude/project-thread-i8ltol` at 85a1bd14. I changed
no code. Line numbers below are on that commit.

**What I ran.** None of it is a game capture of a built feature.

1. **Crust probe (the real CPU engine, a modelled rule).** Shape C (the
   shapes page's keys, `larder_world/shapes.rs:99`) on the CPU race world
   (`race_params`, scale 2, 512 x 320 fine cells, 25,000 walkers), idle
   squad, seeds 1-4. Each game second I applied the plan's crust rule, as
   written, to the engine's real walker counts and trail (arm A). Arm B is a
   graded rule I propose below. The idle squad was overrun and the game
   ended between 2:30 and 3:30 on every seed, so the numbers stop there.
   The engine is real; the crust is my reading of the plan's text.
2. **Bulwark toy (numpy, not the engine).** A body face under a cone of
   rifle fire, with the engine's trail and alarm kernels, 2 s craters, and
   the plan's bulwark rule in four variants. Strike radius (3 fine cells),
   shot rate (8 a second) and refill speed are my guesses.
3. **Memory-road toy (numpy).** A tube of walkers, the engine's 3 x 3 trail
   kernel, keep raised on crust as E1 says; then the walkers leave.

Scripts were in my scratch space and are not kept; the rules are stated
where the numbers are quoted, so each can be rebuilt in a few lines.

## Short answer

The direction is right. The slime writing the world back is the strongest
loop the game does not use yet, and stigmergy (structure left in the
environment that steers later behaviour) is the right family. But at the
game's rates several rules will not do what the plan says:

- **The walkers never read the new fields.** Crust acts on trail keep and
  step; that gives about 3 s of memory, not minutes.
- **Crust becomes a footprint, not a road map.** C's home is a packed sheet;
  crust saturates over it and the roads are a faint smear.
- **E2 wipes its own precondition.** Every rifle hit lays a 2 s crater and
  the plan wipes crust under craters, so the firing face never has crust.
- **A bulwark cannot be a WALL cell** if the slime walks through it, and its
  fade rule never fires while the body stands on it.
- **B1 is undone in one step.** The body's news overwrites a stamp at once.
- **S1 cannot steer by smell.** Soldier smell is in the food scent, which
  fades by e every ~3 fine cells.

Each has a cheap fix, mostly by reusing machinery the engine has: the
Grudge's scar (a slow per-cell integrator read by the sensors), the raid's
distance field, the Mind statuses, the news age, and the heritable walker
kind.

## 1. Is the approach right for the goal?

**What is right.**

- Slow per-cell fields written by the body are the right tool. They are GPU
  friendly, they are data you can draw (the owner's "the picture is the
  data"), and they are how real collective builders work: termite mounds
  (Grassé's stigmergy; Bonabeau et al. 1998), ant trails with a long-lived
  pheromone beside a short one (Jeanson et al. 2003; Dussutour et al.
  2009), Physarum's own extracellular slime (Reid et al. 2012).
- Each rule has its own key and its own film. Good, because several of
  these rules fight each other (below).
- The order E, C, B, S matches the owner's ask.

**What is wrong with the approach as written.**

1. **The loop is not closed through the walkers.** In every published
   stigmergy model, agents *sense* what was built, and that is where the
   behaviour comes from. In the plan, crust changes how fast trail fades
   and how fast walkers step; `hard` changes what a shot does. No walker
   turns toward or away from either. The engine already has the working
   pattern: the Grudge's scar, a leaky per-cell sum of the alarm
   (`big.rs:1192-1212`, updated at `big.rs:4934-4940`) read by the sensors
   beside the trail with a weight (`big.rs:3321-3325`). That is chemotaxis
   with memory. The plan does not mention it.
2. **Slow rules sample fast fields once a second.** Shape C's alarm keep is
   0.9 a fine step (the flinch, `shapes.rs:99`, folded by
   `big.rs:1433-1438`): half-life 6.6 steps, 0.22 s at 30 fine steps a game
   second. A once-a-second test of "alarm at least X" sees a coin toss.
   Integrate every step instead. That is one multiply-add a cell in the
   `trail` pass, which already computes the alarm (`engine.wgsl:1852`), so
   it needs no new dispatch.
3. **"Walkers on the cell" is 0 or 1.** Shape C runs one walker a cell
   (`crowd` 1, `big.rs:1464`; the shapes page films "held one walker a
   cell"). So `crust_occ` and `bulwark_occ` above 1 are meaningless. Use
   the trail (already a ~10-step average of occupancy) or the integrator
   itself.
4. **Four new slow fields where one primitive would do.** Crust, `hard`,
   dynamic moisture and the far smell are each "a value a cell that rises
   with input X and fades". Build that once.

**What I would do instead.** Port the scar to the GPU first (the 11-15
review's finding 12 asks for it anyway) as a general **memory field**:
every fine step, in the `trail` pass, `M = M x keep + gain x input`, with
an optional sensor weight. Give it two instances:

- **Crust** = memory of occupancy (input: trail over the slime threshold,
  or walkers). Read by the sensors at a small weight, both signs filmed.
- **Threat** = memory of alarm (exactly the Grudge's scar). Bulwarks grow
  where crust and threat are both high.

Then the effects in E1 and E2 (memory, fast ground, fuel, cover) read these
two fields. Wet and dry (C1) need no field at all: use the news age (C1
below).

## 2. What the plan misses, step by step

### E1. Crust

**It will be a footprint, not a road map.** Crust probe, arm A (the plan's
rule: +1/60 a second where a walker stands on trail >= 1, x 0.994 a second
where none stands), 4 seeds:

| Game time | Cells with crust > 0.05 | Of those, at >= 0.95 |
|---|---|---|
| 1:00 | 35,654-38,859 | 12-22% |
| 1:30 | 41,058-45,379 | 53-60% |
| 2:00 | 48,964-59,454 | 52-57% |
| 2:30 | 54,087-66,032 | 54-60% |
| 3:30 (seed 3 only) | 60,971 | 71% |

The distribution is two-humped: at 2:00 on seed 1, 29,479 cells sit in the
top tenth and under 1,900 in each tenth from 0.4 to 0.9. In the pictures
(seeds 1 and 3) the home area is one flat, saturated plate. The network on
the far side shows as faint, wide fans, because Jones-style tubes drift and
coarsen, so each cell is held only briefly. Why: of the cells holding a
walker at some time in a second, the median was held on 87-100% of that
second's ticks (10th percentile 7-33%). C's home is a packed sheet. By the
plan's rate any cell held more than about a quarter of the time saturates
(equilibrium: held share / (1 - share) x 2.8 >= 1).

So "brightest on its oldest roads" will read as "a bright home plate". That
can still be a good look (a territory). If roads are wanted, the input must
be flow or age, not occupancy. Arm B (rise x (1 - crust) x share of ticks
held, fade every second wherever) is graded and does not saturate (it caps
near 0.74 under a full sheet), but under a sheet it is still a plate.

**Memory roads give seconds, not minutes.** Memory-road toy: a tube of
walkers, then they leave.

| `crust_mem` | Steady trail | Trail above 3 (fire_damp) for | Above 0.646 (shot and look threshold) for |
|---|---|---|---|
| 0 | 36 | 0.7 s | 1.1 s |
| 0.3 | 49 | 1.0 s | 1.6 s |
| 0.6 | 74 | 1.8 s | 2.8 s |
| 0.9 | 152 | 4.7 s | 6.6 s |

At the plan's 0.6 a road is "remembered" for under 3 s, and only by walkers
within one sensor reach (9 fine cells). The play promise ("it comes back
along the same roads") needs the walkers to read crust directly.

**It changes everything that thresholds the trail.** With `crust_mem` 0.6
the trail over crust doubles (36 to 74 in the toy). Shots stop at trail >=
0.646 (`engine.wgsl:2338-2340`, `squad.rs:79-81`), so they stop on ghost
slime 2.5 times as long after walkers leave. Fire treats trail >= 3 as wet
(`engine.wgsl:3815`). Jones's network shape depends on decay, so the body's
pattern changes too.

**Clay already measured this, and it traps the mould.** Clay ground raises
trail keep by x 1.05 (0.9 to 0.945). On 16 seeds it held walkers on it and
the mould ate less on 15 of 16 (`Reports/larder-maps-2026-11-11.md`,
lines 97-104). `crust_mem` 0.6 at full crust is keep 0.96, stronger than
clay, laid by the mould under its whole home. Expect the "freeze" the plan
lists as a risk to be the default outcome, not a risk.

**Fast ground pulls the other way.** Wet ground (step x 0.5) crowded walkers
onto it and halved the mould on 16 of 16 seeds (same report). Fast ground is
the reverse: walkers leave crust sooner. So memory roads (stay) and fast
ground (leave) partly cancel. Inside a packed sheet a longer step does
little: the landing cell is usually full. Also, `walk` tests only the
landing cell (`engine.wgsl:1093-1100`), so a step well over 1 can hop a thin
barrier; keep it at or under the Sprawler's 1.5, which already plays.

**The nomad option is a known dead end shape.** `crust_mem` below 0 (old
ground repels) is close to `Tier::Wary` (the scar read away) and the flee
from gunfire: in both, random fire herded the mould off its crystals
(`Reports/dead-ends.md` lines 51 and 54). Under build-first that does not
block a film, but say it on the page.

**The deciding measure cannot see the effect.** A blast lays a crater, and
E1 wipes crust under a crater. So the blasted gap has no crust in either
arm. And a gap already refills in about 0.5-2 s (`rules.rs:371-375`;
`Reports/dead-ends.md` line 62), so "seconds until walkers cross it again"
is at its floor. Measure at minute scale instead (section 4, direction 1).

### E2. Bulwarks

**The crater wipes the crust where the shots land.** Every rifle hit lays a
2 s crater (`Rules::crater` 2.0, `rules.rs:767`; `turn.rs:250`). The hit is
sunk into the first slime on the line (`engine.wgsl:2396-2452`). E1 says a
crater wipes crust. So "crust >= 0.6" is false at exactly the cells under
fire. Bulwark toy, 90 s of fire, 2 seeds:

| Variant | Finished bulwark cells at 30 s / 90 s | Kills a 10 s, last 60 s | Shots stopped by bulwarks |
|---|---|---|---|
| Plan as written (wipe, 1 Hz, walkable) | 68-86 / 222-246 | 1,545-2,041 (no fall) | about 0 |
| No wipe by rifle craters | 358-487 / 422-549 | 245-370 (from ~1,800) | 60-70% |
| No wipe, alarm integrated every step | 794-864 / 804-846 | 239-397 | 60-70% |
| As above, but a finished cell is a WALL | 65-91, flickering | 335-477 | 25-40% |

As written, bulwarks grow only on the flanks of the fire cone, where hits
are rare, and kills do not fall. Without the wipe, a slab forms within
about 20 s and kills fall about five-fold. That is the turtle the plan's
risk names, arriving in 20 s (in a toy whose shot rate and radius are
guesses). Rifle craters must not wipe crust; let only blasts and fire do it.

**A bulwark cannot be a WALL cell.** A wall cell is off the agar
(`engine.wgsl:2226-2232`). Walkers cannot step onto off-agar cells
(`engine.wgsl:1096`) and are evicted when one appears (`evict`, 2255).
Trail, scent and news do not pass through. Three consequences:

- "The slime walks through its own" is impossible on a WALL cell.
- The fade rule ("falls back with no walker on it") fires the moment a cell
  finishes, so it flickers (the toy's fourth row: 60-90 finished cells, never
  settling).
- News spreads only through cells holding walkers (`nturn`'s `v > 0`,
  `engine.wgsl:1346-1360`), so a ridge would cut the news. The thin front
  beyond it would wither and panic, and charge the squad.

Make it a ground bit (the plan's own 16-23 range) on agar: walkers pass at a
slow step, shots and soldiers test the bit.

**The fade never fires while the body stands on it.** If walkers can stand
on a bulwark, "no walker on it" is rare. So "stop shooting and the ridges
sag back to glaze" will not happen. Fade on a low threat memory instead.

**The join term makes a slab, not ridges.** Local self-reinforcement with no
longer-range inhibitor fills whatever region meets the condition. In a toy
band where every cell meets it, the join filled 720 of 720 cells. The
ridge's shape comes from where the alarm is (the band at the face), not
from the feedback. Termite pillars need a diffusing cement pheromone and
something that runs out (Deneubourg 1977; Bonabeau et al. 1998), that is,
short-range activation with longer-range inhibition (Gierer-Meinhardt). My
crude inhibitor try gave growth only at the band's edges, not spaced
pillars, so getting loopholes is real tuning work. Keep the join only to
speed coalescence; do not promise ridges from it.

**The energy cost does not bind for most of a planet.** In the probe the
slime reached its 50,000 slots by 2:30-3:30. At the cap shape C eats about
three times its upkeep (`shapes.rs`, the `thicken` note), and nothing spends
the surplus since thickening went off. So `bulwark_cost` costs nothing once
the slime is full. A cost in bodies would bind (below).

**An alternative worth one film: a living wall.** Army ants build bridges
out of their own bodies and leave when the need passes (Reid et al. 2015).
Here: a walker on crust under sustained threat "sets": it stops moving and
takes several hits before it dies, reusing the lump's armour count
(`armoured`, `engine.wgsl:426-434`) and the life cycle's dormant state; it
wakes when the threat memory fades. The wall is walkers (the picture is the
data), the cost is real at the cap (set walkers neither move nor breed),
and it needs no new per-cell state. Blocking soldiers would still need a
cell test. This is a guess, not measured.

**The measure needs a bulwark-off arm.** Kills by a parked squad fall anyway
as the slime near it is shot away; moving 40 cells also changes range,
sight lines and slime density. Run 2 x 2 (bulwark on/off x stand/move); the
deciding number is the interaction, on paired seeds.

### C1. Wet slime, dry slime

- **Fire needs fuel.** It only catches and burns where a cell holds plants
  (`engine.wgsl:3812-3853`, fuel `pf(7)`). On bare ground a dry, starving
  body still does not burn. C1 does nothing without C2's crust fuel or
  plants under the slime. Keep them in one step, as the order table does.
- **Energy is a poor dryness signal.** `energy_flow` pools energy through
  the body, so a cut part holds its store (before withering, a cut part
  "stayed whole for 75 s or more", `shapes.rs`). Upkeep is 0.0002 a step,
  about 0.36 a minute. With moisture relaxing over a minute, a part cut 20 s
  ago has dried by about a quarter. Withering has removed about a quarter by
  then and 59% by 45 s. So "wait for it to dull" lands as it dies anyway.
- **Use the news age instead.** The engine already knows which cells have
  heard of food lately (the spread blocks in `news`, `engine.wgsl:1346-1394`).
  Wet = trail over `fire_damp` and fresh news. That is no new state, it
  turns dry within the wither lag (5 s) of a cut, and drawn as the sheen it
  shows the player *which parts are cut off*, which nothing on screen shows
  today. Check the pass order so the fire pass sees this step's spread
  (a guess that it can).
- **Measure:** normalise fire kills by walkers present (the cut part is
  withering), with the same fuel under both parts.

### C2. Burnt ground

- **Barren char will not stop crossings.** Barren is upkeep x 3 and no
  births (`big.rs:1262-1274`, `barren_upkeep` 3 at 1627). At C's upkeep that
  is 0.0006 a step, and no sensor reads it. On the soil test "walkers do not
  clearly avoid it; they die there" (larder-maps, line 104). The real effect
  is *no births on char*: the slime crosses but cannot fill. Measure walkers
  on the strip at +60 s, not crossings.
- **Crust as fuel makes an excitable medium.** Fuel (crust) recovers only
  under living slime; char is refractory; the fed body is wet. Fire then runs
  along abandoned roads and into dry parts and stops at the fed body. That
  is the forest-fire model's front (Drossel and Schwabl 1992), a good
  emergent use. It is one-shot unless the slime returns.

### C4. The slime flees fire

At keep 0.9 the alarm reaches about 3 fine cells. The flee from gunfire
pushed only walkers next to the wound (`Reports/dead-ends.md` line 40). So
fire will not herd; it will make walkers step back from the front, which
means fewer fire kills. Measure both: kills by fire and where the body's
centre is 30 s later. For real herding, heat needs a longer-reach channel
(a smoke field) than the alarm.

### C5. Gas ignites

Sound. A diffused cloud is an even fuel bed, so a front crosses it as a
visible flash. Check the cloud's values against `gas_burn` on a real cloud
first; the cloud fades as `gas_keep`.

### B1. Provoke

**A stamp is overwritten on the next step.** Feeding walkers stamp "now"
(`ngather`, `engine.wgsl:1335-1342`); cells take the max of their
neighbours 2 cells a step (`wither_reach` 2, `big.rs:1541`); each walker
takes its cell's max (`nhear`, 1381-1394). A provoked walker joined to fed
neighbours hears fresh news on the next step, so it panics for one step.
It needs a status that blocks hearing for the lag plus the lean (a new Mind
kind, `mind_of`), not one store. Note also that provoked walkers can wither
(`engine.wgsl:1212`), a side effect to film.

It also adds a written target: panic runs at the nearest unit
(`panic_to`, 743). If S1's field lands first, provoked walkers can descend
that field instead.

### B2. Calm

`surge_to` (`engine.wgsl:835-866`) never reads the news, so "stamped fresh"
does nothing to a surge. Make Calm a Mind status that `panic_to` and
`surge_to` check. Rename it: `calm` is already a pass (gas counting,
`engine.wgsl:3453`).

### S1. Surges by smell

Soldier smell is added to the food scent (`smell`, `engine.wgsl:2647-2668`).
That field falls by e about every 3 fine cells (11-15 review, finding 4);
the surge reaches the gun's range plus 5. At that range the smell is about
1e-4 of its source, under the crystals' scent in the same field. A surging
walker reading it goes to food. Use a distance field instead: the raid's
Dial search (`raid_field_for`, `big.rs:5492`) with every unit as a source,
on 2-cell blocks, rebuilt once a game second. Walls bend it; a masked
soldier is left out of the sources. It is a field, not a picked target.

### Part S's keep rule

"Kept if the counts are no lower" on 4 paired seeds has no power to show
"no lower". Name a margin ("first contact no more than 5 s later on at least
3 of 4 seeds") before the run.

### Outside the run

- **A. An evolving slime.** `size_spread` is keyed on the walker's slot
  (`engine.wgsl:965-968`), so it is not inherited. But the walker's *kind*
  is: births copy it on both engines (`engine.wgsl:1469`, `big.rs:4465`),
  with `mix_step`, `mix_reach` and the rest as its trait. The first
  experiment needs no code. Selection will be weak: energy is shared and
  births go where there is room, so only death selects. Drift and "gene
  surfing" at an expanding edge make sectors on their own (Hallatschek et
  al. 2007), so a neutral control (both kinds the same) is required.
- **F. Rival slimes.** `species_cycle` already gives three-way conversion;
  rock-paper-scissors with movement makes spiral waves (Reichenbach,
  Mobilia and Frey 2007). It would be a fine terrarium piece.
- **E3. Tunnels.** As written, walkers press on rock wherever the body
  touches it, because scent does not pass rock (`open`, 613-616). Erosion
  then dissolves the whole contact edge. Let scent seep through soft rock
  (`wall_scent` exists for walls) and erosion follows the gradient. That is
  Laplacian growth, which makes branching fingers toward food for free.

## 3. Making each step better, cheaper or more visible

- **One memory primitive, updated every step inside `trail`.** No 1 Hz pass,
  no aliasing, no new dispatch.
- **E1:** input from the trail, a graded rule (rise x (1 - crust), fade
  always), read by sensors at a small weight, both signs. Wiped by blasts
  and fire, not rifle craters. Draw it as a territory; consider drawing
  time-since-held rather than amount held, which shows roads better.
- **E2:** a ground bit on agar, not a WALL. Grow from crust x threat memory.
  Fade on low threat. Add hysteresis for soldiers' walking maps (block at
  1, unblock under 0.5) so a chipped ridge does not churn `paths.rs` (a
  wall going up already caused churn once, `Reports/dead-ends.md` line 81).
  Film a living-wall arm beside it.
- **C1:** news age, not dynamic moisture. Drawn as sheen it is the plan's
  best readability win.
- **C2:** measure fill, not crossings.
- **B1:** a deaf status; **B2:** a Mind status. Both are one more Mind kind.
- **S1:** the units' distance field. It also gives B1 and panic a field.
- **Free visible data:** the energy flow between cells (`egain`) is already
  computed; drawn, it shows arteries, and a bulwark's cost would show as
  flow toward the face.

## 4. My own ranked directions, each with an afternoon experiment

1. **Crust that the walkers read (territory and return).** Repoint the CPU
   scar's input from alarm to occupancy (a few lines in `BigDish`, CPU
   only), weight 0, +w and -w. Shape C, race world, seeds 1-4. At 2:00 push
   or blast half the home away. Deciding measure, named first: overlap
   (Jaccard) of cells re-held at +60 s with the old crust footprint, paired
   against weight 0. Smoke: crystals held at 3:00.
2. **Threat memory and bulwarks (E2a on the CPU).** The Grudge's scar as the
   threat input, rifle craters not wiping crust, a ground bit not a wall.
   2 x 2 (bulwark on/off x squad stands/moves), seeds 1-4, 90 s. Deciding:
   the interaction in kills a minute. Film where the ridges form.
3. **News-age sheen and news-age fire damp.** Draw the news age on the GPU
   (no new state), cut a ring at 2:00, film 30 s. Then the damp test, with
   the cut part on grass, lit at +10 s. Deciding: fire kills per walker
   present, cut against joined.
4. **A units distance field for surges.** Generalise `raid_field_for` to
   many sources; planet 3, a squad behind a wall, seeds 1-4. Deciding:
   walkers reaching the squad per surge and seconds to first contact,
   against today's straight line.
5. **Heritable kinds, with a neutral control (no code).** `mix_share` 0.5
   with `mix_step` 1.3 against all-1 kinds; a shooting hand for 10 minutes;
   kind share every 30 s and a picture of where each kind is. Deciding:
   share change in the trait arm beyond the spread of the neutral arm.
6. **Tunnels by seeping scent.** Soft rock with `wall_scent` above 0 between
   the home and a crystal, CPU, 5 minutes. Does erosion make fingers toward
   the crystal, or wear the whole edge?
7. **Spirals for the terrarium.** Three cycling species on an open map with
   no squad; film 2 minutes. Look only.

## 5. Checks of numbers, files and costs

- **Rates.** Half-life of `crust_keep` 0.994 a second: 115 s, as stated.
  `crust_rise` 1/60 fills a held cell in a minute, as stated; the probe
  had 12-22% of crust cells full at 1:00 and 53-60% at 1:30.
- **Clock.** 15 game ticks a second (`rules.rs:709`) x 2 fine steps a tick
  at scale 2 (`world.rs:432`) = 30 fine steps a game second. Every decay
  above uses that.
- **Map size.** The CPU race world is 512 x 320 fine cells. The brief's
  1024 x 640 and the 6x planets scale costs by area.
- **Line references in the plan.** Mostly right. `life` barren line is
  `engine.wgsl:1245`, not 1243. `build_wall` (2203) is the comment above the
  `build` pass (2209). The "16-23 kept for mould-made ground" reservation is
  in `Reports/larder-maps-2026-11-11.md` row 8, not in `engine.wgsl`
  558-600. `Burst::alarm` is a field at `squad.rs:223`, not 200.
- **Costs.** The plan's guesses (0.02-0.03 ms) are plausible. My proposal
  (an integrator updated every step in `trail`) costs one read-modify-write
  a cell a fine step: about 5 MB a step at 1024 x 640, a guess of
  0.02-0.04 ms a frame on the 1070, times the area on bigger planets. That
  is more than a 1 Hz pass but avoids aliasing; a 1 Hz pass fed by the trail
  is a fair cheaper option for crust only. Values in the `news` buffer are
  read through `atomicLoad`, which may cost a little more than plain loads
  on the card (a guess; measure it).
- **The plan says nothing is built and every number is a first guess.**
  That holds; nothing above contradicts its scope.

## References (inspiration, not a bar)

- Bonabeau, Theraulaz, Deneubourg et al. 1998, *A model for the emergence of
  pillars, walls and royal chambers in termite nests*.
- Deneubourg 1977, *Application de l'ordre par fluctuations à la
  description de certaines étapes de la construction du nid chez les
  termites*.
- Drossel and Schwabl 1992, *Self-organized critical forest-fire model*.
- Dussutour, Nicolis, Shephard, Beekman and Sumpter 2009, *The role of
  multiple pheromones in food recruitment by ants*.
- Hallatschek, Hersen, Ramanathan and Nelson 2007, *Genetic drift at
  expanding frontiers promotes gene segregation*.
- Jeanson, Ratnieks and Deneubourg 2003, *Pheromone trail decay rates on
  different substrates in the Pharaoh's ant*.
- Jones 2010, *Characteristics of pattern formation and evolution in
  approximations of Physarum transport networks*.
- Reichenbach, Mobilia and Frey 2007, *Mobility promotes and jeopardizes
  biodiversity in rock-paper-scissors games*.
- Reid, Latty, Dussutour and Beekman 2012, *Slime mold uses an externalized
  spatial "memory" to navigate in complex environments*.
- Reid, Lutz, Powell, Kao, Couzin and Garnier 2015, *Army ants dynamically
  adjust living bridges in response to a cost-benefit trade-off*.
- Gierer and Meinhardt 1972, *A theory of biological pattern formation*.
