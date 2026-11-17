# Review: the simulation-use plan, as play (emergent-systems designer)

2026-11-17. Reviewer 1 of 3, on `plan.md` draft 2, against branch
`claude/project-thread-i8ltol` at 85a1bd14. No code changed, nothing run.
Every claim about play is a reading of the code and reports, not a film;
guesses are marked.

## The verdict in short

The direction is right. A slime that changes the ground back is the
missing half of the game's systems. Bulwarks (E2) are the plan's best
idea: they are the enemy progression the owner asked for, and they come
from how the player plays, not from a timer.

Four changes would make it much better:

- **Drive bulwarks from the Grudge's scar, not from the live alarm.** The
  scar already exists (CPU only) and is exactly "where it was shot, and how
  long". The live alarm on planets 1-3 lasts a few fine steps, and the
  flinch moves walkers *off* the cells being shot. As written, E2 may
  never fire on the first three planets.
- **Bulwarks stop bullets, not sight, and stop them by chance.** If they
  block sight (they will, if they are wall cells), the squad just stops
  shooting. The player sees a squad that does nothing. The chip and the
  "kills fall" measure both break.
- **Name the owner rulings the plan pushes against**, and design round
  them: sustained fire must carve a growing hole (11-12 04:39Z); slow
  ground "might be annoying" (11-14 17:29Z); fire must not "just wipe out
  the whole slime" (11-14 17:29Z).
- **Get a playable build to the owner after E2a**, with crust burning
  (half of C2) folded in, before E2b's pathing work.

## 1. Is the approach right?

Yes, for three reasons.

- **It adds a second writer to the world.** Today the player writes fields
  and the slime only reads them (round 1, `README.md`). Immersive sims get
  depth when both sides write the same fields. Crust, bulwarks and
  moisture make the slime a writer of ground, fire fuel and cover.
- **E2 answers the owner's oldest play note.** "I mostly run to the biggest
  crystal, harvest ... until I have 40 soldiers and then I attack"
  (`wiki/world/run.md:169`). A slime that digs in where it is shot from one
  place punishes the parked deathball and rewards flanking, turrets and
  throws. The mould-tiers page already lists this as a candidate: "a mould
  that grows round cover ... so the player has to move rather than sit"
  (`wiki/world/mould-tiers.md:76`).
- **It leaves the run's scripted pressure alone until a field version is
  no worse.** That respects the owner's guard.

What I would do differently, in one line each:

- Treat **E2 as the feature** and E1 as its support. E1 alone gives the
  player nothing to decide. Build only the parts of E1 that E2 and fire
  need.
- **Tie each new rule's strength to depth** (`run::enemy_keys`), and name
  it on the planet card (`does_to_you`). The run lacks enemy progression
  (`wiki/world/run.md:173`). Bulwarks that start weak on planet 1 and grow
  are that progression, and the planet card teaches the rule.
- **Write three target stories first**, and make each a scripted hand and
  a film. For example:
  1. "I parked my squad on the ridge, the slime grew a wall against it, so
     I moved round the side."
  2. "It dug in at its home crystal, so I lobbed fire over the ridge, and
     the starving back half went up."
  3. "I burned its old road and its next surge came the long way round,
     into my turrets."
  If a step cannot produce one of these on film, it is not ready.

## 2. What the plan misses

### 2a. Owner rulings in tension (not cited in the plan)

- **The growing hole.** 11-12 04:39Z: "A group of soldiers shooting the
  slime should start carving out a bigger and bigger hole that doesn't
  refill while being shot at" (`wiki/decisions-game.md:135`, `:139`).
  Bulwarks rise at the rim of exactly that hole and stop it growing. The
  two can live together if the bulwark needs time under fire first, so the
  first 15-20 s still carves. But the owner should see the clash on the
  page and pick, not find it in play.
- **Slow ground.** 11-14 17:29Z, on slow ground: "We can test this, but I
  am guessing it might be annoying" (`wiki/world/plants.md:75`). That is a
  stated uncertainty, so under "defaults on" `crust_wade` should start
  **off**. Also, the crust spreads wherever the slime has lived, so late
  in a long planet most of the map slows soldiers. That is busywork, not a
  decision. Wading in live slime already exists (`Rules::wade`,
  `larder_world/rules.rs:230`). My advice: drop `crust_wade`, or keep it
  only on bulwark-adjacent cells.
- **Fire wipe-outs.** Same message: "You don't want to make it too easy to
  just wipe out the whole slime with fire." Part C adds four fire buffs at
  once: starving slime burns (C1), crust is fuel (C2), fire herds (C4) and
  gas flashes (C5). Together they make a loop. Fire herds the slime off
  its crystal, the slime starves, starving slime burns, and the burn
  leaves barren char it won't cross. That is a likely dominant strategy
  (a guess, but every link is in the plan). Each C rule needs a fire cost
  that the player feels. Fire already hurts soldiers (`Rules::fire_hurt`,
  `plants.md:41`). Crust fuel carries fire along the slime's roads, which
  may run back to the squad. Say this, and film it.

### 2b. The Grudge's scar is the field E2 needs

- `BigParams::scar_keep`, `scar_gain` and `scar_weight` (`src/mould/big.rs:1192-1212`).
  Each fine step, scar = scar x keep + gain x alarm. It does not spread and
  lasts about 40 game seconds at keep 0.9999. It is CPU only, and the
  Grudge is out of `LADDER` until the GPU takes it (`simulation-to-play.md`
  finding 12).
- The plan's E2 reads "the alarm is at least `bulwark_alarm`" on a
  once-a-second tick. On planets 1-3 the alarm is the flinch's, at keep 0.9:
  "a shot's alarm halves in a few fine steps" (`big.rs:1181-1188`). A
  once-a-second sample of that field is mostly zero. The scar integrates
  it, so "under fire for a while" is one read.
- Porting the scar to the GPU for E2 also unlocks the Grudge for the run
  ladder. Two features for one field.

### 2c. The flinch works against the bulwark

- The flinch reads the alarm at -1 (`FLINCH_WEIGHT`, `big.rs:1425-1436`),
  so walkers back off the cells being shot. E2 needs at least
  `bulwark_occ` walkers standing on a cell under alarm. On planets 1-3 those
  two rules fight. The likely result (a guess) is a ridge a few cells back
  from the shot face, or nothing.
- On the Swarmer (alarm read toward), walkers pile onto the wound, so the
  bulwark forms fast and thick. That is a big gap between tiers. It is a
  spread, which is good, but the plan should film one planet of each kind.

### 2d. Do bulwarks block sight? The plan doesn't say, and it decides the feel

- Sight is "every cell on the line is agar" (`src/mould/squad.rs:811-818`;
  GPU `sees`, `src/gpu/engine.wgsl:2676-2688`). A unit aims only at slime it
  can see (#198, `ROADMAP.md` L-F).
- If a bulwark blocks sight, soldiers behind it see nothing and fire
  nothing. Then:
  - "Shots start hitting ridges" never happens. The squad goes quiet.
  - "Rifles still chip them" fails, because no rifle aims at a bulwark.
    The anti-soft-lock guarantee rests on chip.
  - The deciding measure, "kills fall", passes for the wrong reason.
- Recommend: **bulwarks stop bullets, not sight.** The player sees the
  slime behind the ridge, watches shots splash on it, and reads "move".
  That is the visible fight the "tool you can see" rule asks for.
- A bulwark also has to be agar to walkers (the slime walks through) and
  not agar to bullets and soldiers. So it needs its own bit, checked in
  `sees`, the shot trace and `standable`. The ground word's bits 16-23 are
  free (`engine.wgsl:558-600`). Say which functions change.

### 2e. The fade rule contradicts "what the player sees"

- Rule: hard "falls back to crust over `bulwark_fade` seconds **with no
  walker on it**".
- Seen: "Stop shooting and the ridges sag back to glaze."
- The slime walks through its own bulwarks and lives there. With walkers
  on it, the rule never fades a ridge. Bulwarks would build up over a
  planet and never sag. That is the turtle the plan's own Risk names.
- Fix: fade when the scar (or alarm) is low, whatever stands on it.

### 2f. Soft-locks: nest doors and planet-1 tools

- Every nest has a front door 7-9 dish cells wide (`DOOR_MAX` 10) and a
  breach 28-35 wide (`wiki/world/planets.md:46`). A slime under fire at its
  nest door can wall a 7-9 cell door in one ridge. With E2b that seals it
  for soldiers. The level generator's check is static, so it cannot catch
  a wall that grows in play.
- On planet 1, the player has rifles and pumps only. Walls, turrets,
  blasts, mines, cord and gas are shop unlocks (`src/mould/run.rs:393-402`).
  So "turrets and throws are the answer" does not hold on planet 1, and
  the only answer is rifle chip, which 2d may break.
- Ask for one number before E2b: rifle chip per second against bulwark
  rise per second at a 7-cell door, with a four-soldier squad. If rise
  beats chip, the door is a lock. Also start bulwarks at 0 on planets 1-2
  (by depth), so the first planets teach without trapping.

### 2g. Fast crust can let walkers hop walls

- The walls report: a walker's move checks only its landing cell, and "its
  longest step (2 fine cells in a surge or panic lunge, at scale 2) is
  shorter than the thinnest player wall (4 fine cells)"
  (`Reports/walls-2026-11-16/README.md:15-20`).
- Step factors multiply: soil, panic haste, lump haste
  (`engine.wgsl:975`, `1003-1008`). `crust_fast` 1 would make a 2-cell lunge
  4 cells, which can land past a 2-cell wall. And crust is thickest along
  walls, where the slime presses and bites.
- Fix: cap the step at under the thinnest wall, or let crust speed only
  the plain step. Add a twin test for it.

### 2h. B1 provoke is erased the next step

- `nhear` does `atomicMax(&news[k], cell news)` for every ordinary walker
  every step (`engine.wgsl:1381-1393`, the max at 1389). In a connected, fed body the
  cell's news is fresh. A walker stamped "heard nothing" re-hears at once.
  As I read it, B1's "one store a walker" does nothing in a connected body.
- And while the stamp lasts, `life` withers it by chance
  (`engine.wgsl:1212`), so provoke also kills. That makes it a damage tool,
  not just a timing tool.
- Fix: provoke is a timed **deaf** status (a Mind-style bit that skips
  `nhear`), for lag plus lean. That is B3's hush on walkers instead of
  cells, so B1 and B3 share code.

### 2i. B2 calm can delete the slime's main threat

- Surges are the run's main pressure (`wiki/world/run.md:92`). A device that
  stops every surge, on a cooldown shorter than the surge's wait (10-80 s),
  removes the threat with no answer. That is a dominant pick (a guess
  until the cooldown is set).
- Give calm a cost the player sees. For example: calmed walkers count as
  fed, so they stop withering and a cut-off part heals; or calm only slows
  the charge and spreads it out (the plan's own "slows and spreads" look).

### 2j. Interactions the plan doesn't list (the real payoff)

These are where moments come from. Each is a guess at play; each is cheap
to film once the pieces exist.

- **Building dries the slime (E2 x C1).** `bulwark_cost` spends energy. With
  C1, low energy means low moisture, and low moisture burns. So "make it
  dig in, then burn it" comes out of the model with no new rule. Make this
  the headline combo of Part C.
- **Turrets over walls (E2 x the 11-16 ruling).** Already in the plan, and
  right. But turrets are strong already: a turret ring "holds against the
  default slime with no wall lost in 3 minutes" (`walls` README, lines 156-158).
  Bulwarks make them relatively stronger. Watch for turret creep (build a
  turret line, wait). Measure it.
- **The origin boss.** The owner wants an origin at each planet's start
  crystal that must be killed (`decisions-game.md:71`). That is the most
  crusted ground on the map. With bulwarks, the origin fight becomes a
  siege with a lair. Design it as a feature, not a side effect.
- **Mini-bosses.** A wanderer roots 15 s on a crystal: it lays crust, and
  if shot it walls in. A leech rings a working pump: turret fire on it
  grows a bulwark round the pump, which shields the leech from rifles. A
  sapper chews player walls while its own side hardens. All good stories.
  Check that a bulwark never forms on a soldier's own cell (wading
  soldiers stand in slime, under fire, on crust).
- **Gun statuses.** Shove and Heave push walkers off a ridge, so it stops
  growing (needs occupancy). Blind walkers walk straight out of their
  ridge. Sour flips trail-following, so a slime may flee its own memory
  roads. Sleep gas holds walkers still on a ridge, so it holds; then C5's
  gas flash. These make the existing cards matter more.
- **Smother and S1.** If surges climb soldier smell, the existing `smother`
  card (disrupt scent) becomes the dodge. The plan says "a soldier who
  masks their smell"; name the card.
- **Mines on memory roads.** If the slime comes back along its old roads,
  the player can read the glaze and mine them. That is the decision E1
  creates. Put it in the film.
- **Rubble and moisture.** Rubble is wet agar (`walls` README, line 25).
  With C1, broken walls become firebreaks. Fine, but note it.

### 2k. Controls and confounds in the proposed checks

- **E1's blasted road.** The blast's crater holds 20 s
  (`wiki/world/guns.md`, blast), and the plan wipes crust under craters.
  So at the gap the crater decides the refill and crust has nothing to
  act on. Measure return along the road's surviving stretches, or kill
  with a crater-free burst.
- **E1 changes the body, not only the roads.** `crust_mem` raises trail
  keep on all crust, and the living body sits on full crust. Lower decay
  changes the Jones pattern everywhere (thicker, blurrier). Apply it only
  where no walker stands, or make crust a sensed field (like the scar's
  `scar_weight` term, `big.rs:3321-3324`) instead of a change to keep.
  The sensed version also gives the nomad sign flip for free.
- **E2 needs a bulwark-off arm.** Standing against moving, both with
  bulwarks on, confounds geometry, range, wading and the flinch. Run the
  2 x 2: stand or move, bulwark on or off. Log shots fired as well as
  shots stopped, so a sight block can't pass as cover.
- **Four paired seeds cannot pass a sign test.** The smallest two-sided p
  with 4 pairs is 0.125. Call these looks, or use 6 or more (6 of 6 gives
  p 0.031).
- **C1.** A cut-off part withers anyway. Print deaths by cause (shot, fire,
  wither, starve), which finding 17 of the 11-15 review says is missing.
  Otherwise fire kills and wither kills mix.
- **Part S.** "The owner can't tell them apart" is a weak bar for effort
  spent. Ask for "prefers, or can't tell and the counts are no lower".

## 3. Making each step better, cheaper or more visible

**E1 crust.**
- Cut to: the field, the glaze, and crust as fire fuel. Leave memory roads
  as a sensed weight (2k), off until filmed both ways. Drop `crust_wade`.
- Pack crust and hard as two 16-bit halves of one `news` word, not two
  blocks. Plants' blocks exist only with plants on (`gpu/mod.rs:3256-3292`),
  so crust needs its own block either way; on a 6x map one block is
  about 16 MB.
- Make `crust_fast` small (1.2x at most) and capped (2g). A body that is
  faster everywhere it lives surges and lunges faster at every depth. That
  is a balance shift the plan doesn't mention.

**E2 bulwarks.**
- Read the scar (2b). Fade on a low scar (2e). Stop bullets, not sight
  (2d).
- Make stopping a chance: a bullet crossing a cell stops with chance equal
  to its hard. That is a spread, not a yes or no, and the ridge's darkness
  *is* the stop chance. Keep a threshold only for soldier walking (E2b).
- Require time under fire before rising, so the owner's growing hole
  survives the first 15-20 s (2a).
- Scale rise by depth. Zero on planets 1-2.
- Show the owner a playable build after E2a. E2b touches the levelgen
  thread's walking maps and steer, and is the riskiest code in the plan.
  The owner ruled "both", so build it, but after a playtest of E2a and
  behind a key.

**C1 and C2.**
- Move "crust burns" and "char is barren" into the E1/E2a build. Fire is
  the counter to crust that planet 1 can reach soonest (blasts and scorch
  rounds light plants), and it makes the first playable build have
  counterplay.
- Moisture is already a per-cell block (block 7, `gpu/mod.rs:3286`). C1
  needs walkers to write it; WGSL has no float atomics, so do it as a cell
  pass that reads the per-cell energy share, not per walker.

**B1 and B2.** Fix 2h and 2i before filming. Both are small once B1 is a
deaf status.

**C4 fire lays alarm.** The flee dead ends (`Reports/dead-ends.md:51`,
`:54`) say more than "a census gate failed": "any rule that keeps the mould
off soldiers makes their fire a free win", and the scar version "herds it
off every crystal". Fire that herds is that rule with a bigger brush. Film
whether fire can herd the slime off its home crystal; if it can, cap the
push near the slime's own crystals (the dead end's own re-test condition).

**C5 gas ignites.** Cheap and good. It turns sleep gas plus any fire into a
combo with no card rule, which is what the gun plan wants.

**Order I'd use** (same E, then C, then B as the owner set):

| # | Step | The owner sees |
|---|---|---|
| 1 | Scar on the GPU; crust field and glaze; crust burns, char barren | a page: crust over 5 min; a road burned |
| 2 | E2a: bulwarks as cover (scar-driven, chance stop, sight passes) | a **playable build** and the 2 x 2 films |
| 3 | E2b: bulwarks block soldiers, behind a key | a door test at a nest; a playable build |
| 4 | C1 wet and dry; then C5 gas ignites | "make it dig in, then burn it" |
| 5 | B1 as a deaf status, B2 with a cost | provoked charge into a mine; calmed surge |
| 6 | C4, filmed against the herding dead end | herd or no herd |
| 7 | S1, S2 | blind side by side |

## 4. My own ranked directions, each with an afternoon experiment

1. **Scar-driven cover (E2a's core).** CPU big dish, shape C on planet 3,
   the Grudge's `scar_keep` 0.9999 and a small gain, a four-soldier squad
   standing 90 s at a crystal edge. Dump the scar field at 30, 60 and 90 s
   and look: is it a ridge at the face or a blob? Then a CPU-only hack in
   the shot trace: a cell with scar x walkers over a level stops a bullet
   with that chance. Count shots fired, shots stopped and kills a 10 s
   window, on and off, 6 paired seeds. If the scar is a blob, try the
   termite join term before anything else.
2. **Crust as a sensed field, both signs.** CPU: crust = slow average of
   occupancy; sensors read `crust_weight` x crust beside the trail, like
   the scar. Planet 3, idle, 5 minutes, weight +0.3 and -0.3 against 0.
   Film it. Count crystals touched and the body's centre drift. One run
   answers memory roads and the nomad together.
3. **Build dries, dry burns.** After 1, give scar cells an energy cost and
   let fire's damp test read local energy instead of trail (a stand-in for
   C1). Light a scorch line on a dug-in face at 60 s. Count fire kills on
   the face, with and without the cost. This tests the headline combo
   before C1's moisture work.
4. **The death spill (outside-the-run D, which I'd bring in).** It makes
   every kill visible and gives turrets and bulwarks a reason: the slime
   comes back to eat its dead in the kill zone, and farming versus
   wiping is a real decision. First: a spill of 0.5 x energy into the
   grass field (finding 10's own probe), a squad holding a crystal edge
   20 s, walkers within 10 cells. Grep `dead-ends.md` for spill first
   (I found no entry).
5. **Provoke as deaf.** CPU: a status that skips the news hearing for lag
   plus lean, stamped in a disc. Planet 2, a disc on a part 15 cells from a
   squad, 4 seeds. Does it charge in 0.5-1.5 s? How many wither on the way?

## 5. Checks of numbers, files and costs

- Citations checked on this branch: `walls` 2092, `fire` 3812, `firekill`
  3864, `ignite` 3888, `panic_to` 743, `surge_to` 835, `raid_to` 869,
  `mindlay` 3914, `ngather` 1335, the barren upkeep 1243-1245 in
  `engine.wgsl`; `build_wall_hp` 2595, `bite_walls` 2698, `mind` 5248 in
  `big.rs`; `Rules::smell` `rules.rs:233`. All match. `build_wall` at 2203
  is a comment block, not a `fn` (the pass has another name). `Burst::alarm`
  is the field at `squad.rs:222`, not 200 (200 is the struct's doc). The
  plan says main d024480d; I read 85a1bd14.
- Crust's rise: `crust_rise` 1/60 at full occupancy fills in 60 s, and
  `bulwark_crust` 0.6 needs 36 s. So bulwarks can only form where the slime
  has lived over half a minute: home, and held crystals. That is fine, but
  it means the parked deathball at a fresh front never meets one. Say so.
- Crust's fall: 0.994 a second gives a half-life of ln 0.5 / ln 0.994,
  about 115 s. The plan's "about two minutes" is right.
- Cost guesses (0.02 ms a pass, 0.02 ms of reads, 0.03 ms for E2) look
  plausible for one cell pass a second and one more word read per walker
  step. Not measured. The 6x map's buffers are already 235 MB
  (`planets.md:46`); two more blocks add about 31 MB. Packing both values
  into one word halves it.
- E2's "share of shots stopped" needs a counter in the shot trace on both
  engines. Not in the plan's code list.

## Top points (summary)

1. Right direction; bulwarks (E2) are the plan's best idea and the run's missing enemy progression. Tie them to depth and the planet card.
2. Drive bulwarks from the Grudge's scar (`big.rs:1192-1212`), ported to the GPU. The live alarm on planets 1-3 fades in a few steps.
3. The flinch moves walkers off shot cells, and E2 needs walkers on them. Film planets 1-3 before trusting E2 there.
4. Bulwarks should stop bullets, not sight, and stop them by chance. A sight block makes the squad go quiet and breaks the chip and the measure.
5. Fix the fade rule: "no walker on it" never fades a ridge the slime lives in. Fade on a low scar.
6. Soft-lock risk at nest doors (7-9 cells) with E2b, and planet 1 has only rifles. Measure chip against rise first; bulwarks start at 0 on planets 1-2.
7. Cite and design round three owner rulings: the growing hole under fire, slow ground "annoying" (drop `crust_wade`), fire must not wipe the slime.
8. B1 provoke is undone the next step by `nhear`'s `atomicMax` (`engine.wgsl:1389`); make it a deaf status. B2 calm needs a cost, or it deletes surges.
9. The headline combo is "building dries the slime, dry slime burns" (E2 x C1); also crust fast can let lunges hop 2-cell walls (cap the step).
10. Order: scar plus crust plus crust-burns, then E2a as a playable build, then E2b behind a key. Fix the E1 and E2 checks: crater confound, 2 x 2 with bulwark off, 6+ seeds.
