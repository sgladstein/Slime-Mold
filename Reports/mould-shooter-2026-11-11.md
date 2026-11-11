# The mould as the enemy: a shooter brainstorm (2026-11-11)

A design brainstorm, no runs and no code. Every claim about how the game
would play is a hypothesis to test; the facts it leans on are cited.

## The ask

Owner, 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the
enemy. The player is a soldier or unit of soldiers who can shoot the mold to
destroy parts of it. The player needs food to upgrade, build defenses, etc.
and the slime mold wants food to expand/survive, etc,"

So: one map, food piles on it, two hungry sides. One side is a person with
guns. The other is the mould, which has no mind, only its rules.

## The model, described neutrally

What the simulation can do today, in its own terms, with no tools in mind:

- **Walkers** move a step at a time. Each senses the field ahead at three
  points (left, centre, right), turns toward the strongest, steps and lays
  **trail**. Trail spreads and fades each step. Walkers that follow each
  other's trail make **tubes**, and many tubes make a **network** with loops.
  The sensor reach sets the mesh size ([sensors](../wiki/model/sensors.md),
  [scale](../wiki/model/scale.md)).
- **Food piles** give off **scent**, which spreads further than trail.
  Walkers far from the network smell food and head for it.
- **Food is the population.** A walker on food eats; eating pays for new
  walkers (division) and for every walker's upkeep. With `Params::pieces`,
  food travels only along tube, so each connected **piece** lives on the food
  it touches. A piece cut off from food starves, farthest from food first,
  over seconds (`src/mould/mod.rs`, module notes).
- **Ground** can be blocked (walkers cannot step there) or **wounded** for a
  time (`Params::wound`, 4 s by default).
- **Two populations** can share a dish and fight where they meet
  (`Params::fight`, The Wild One, Two Fevers).
- **At scale** (the GPU target, 100,000 walkers on 1024 x 640) the network
  holds about 90-160 loops against today's few dozen
  ([scale](../wiki/model/scale.md)). The big dish has no economy yet.

## Three facts a shooter has to respect

1. **Killing walkers is weak; cutting supply is strong.** The knife (snip)
   kills walkers and clears trail along a stroke and is the weakest tool:
   a cut heals in 4 s and a trunk between two fed piles leaves both sides
   fed ([knife](../wiki/tools/knife.md)). At 100,000 walkers one bullet's
   kills mean nothing. A gun only matters if a shot changes **where food
   flows**.
2. **An empty stretch of tube is a cut, for as long as it stays empty.** The
   old sensor lamp, which only turned walkers away, cut the food join on 5 of
   5 seeds while it was lit, because "an empty stretch is a cut"
   ([dead-ends](dead-ends.md), `Params::light_weight`). So a weapon that
   keeps a line clear (sustained fire) should cut where a single shot heals.
   This is the strongest hint in the record for what shooting should be.
3. **The mould already "wants" food in a way a player can read.** It smells
   piles from afar, sends fronts toward them, thickens the tubes that carry
   the most, and lets the rest die back. The picture is the data
   ([simulation look](../wiki/looks/simulation-look.md)): a thick bright
   tube is a supply line, a pale one is starving.

## Ideas, many before judging

What a shot could do to the model:

- **Clear** trail along its path (a cut while held; heals when you stop).
- **Scorch**: wound the ground for N seconds so nothing crosses (a timed wall).
- **Kill** walkers in a small disc (shrinks a front, little else).
- **Sour**: lay a repellent field walkers turn from (the old sensor lamp,
  aimed); the network bends round where you aim.
- **Spoil** food: destroy part of a pile so neither side gets it (denial).
- **Sever-and-hold**: a stake you fire into a tube that keeps it clear until
  the mould overgrows it.

What food could do for the player (one resource, many uses, after Browder in
[rts-craft](../wiki/rts/rts-craft.md)):

- Currency for upgrades, soldiers, turrets and walls.
- **Ammunition**: every shot costs food, so firing and building trade off.
- **Bait**: drop food to lure a front into a kill zone or away from your base.
- **Score**: food shipped out is the points.
- **Smell**: the food you hoard gives off scent like any pile, so a rich base
  draws the mould. This is the idea the rest of the design hangs on.

What the mould could do to the player:

- **Engulf**: where walker density on a soldier or structure passes a line,
  it takes damage (the mould eats it); dense fronts are dangerous, thin
  searchers are not.
- **Share the pile**: a harvester on a pile the mould also covers gets only
  its share (piles already split food among their eaters, `eaters` and
  `foe_eaters` in `src/mould/mod.rs`). The mould steals by sitting on food.
- **Eat your stores**: mould that reaches the base eats from the larder, which
  feeds it, which grows it.
- **Grow round** walls and turrets; loops route round any single block.

Shapes the game could take:

- **Hold the larder** (base defence, the lead below).
- **Forage run** (extraction): a squad enters a mould-covered dish, grabs
  food, gets out; every pile you take wakes the network toward your smell.
- **Supply war**: the player lays pipes from piles to base, mirroring the
  mould's tubes; each side cuts the other's lines.
- **Versus**: two players and one mould; you steer the mould onto your rival
  by dropping food near them (the mould is a weapon both sides aim).
- **Purge**: clear a dish by starving the mould to zero, scored by time and
  food spent.

## The lead concept: Larder (working title)

A top-down shooter on one dish. You start with a small squad, a base with a
little food, and a few harvesters. The mould starts in a corner with a store.
Food piles are spread across the map. A round lasts a fixed time (say 8
minutes); your score is the food you shipped out, against the food the mould
ate. That is a spread, not a win or loss (CLAUDE.md), and a points ending,
which the RTS research recommends over a wipe-out.

### The loop

1. Harvesters on piles pull food into your larder, sharing it with any mould
   on the same pile.
2. You spend food on shots, soldiers, turrets, walls and bait, or ship it for
   points.
3. **Food in the larder smells.** The more you hold, the further it carries,
   so hoarding brings the mould to you. This is a cost that grows with
   success, the RTS research's first fix for a snowball, here coming
   straight from the model's scent field rather than from a rule.
4. The mould grows where it is fed and starves where it is cut. It has no
   plan; its fronts go where the scent is strongest.

### The squad's kit

Few pieces, each mapped to one model lever:

| Piece | What it does to the model | Costs | Good against |
| --- | --- | --- | --- |
| Rifle | clears trail on a thin line, kills walkers on it; a cut while you hold fire | a little food a second | a thin trunk, a lone searcher |
| Burner | clears trail and wounds ground in a short cone | more food | a dense front at close range |
| Stake | holds a tube clear at one point until overgrown | food, placed | a trunk you cannot stay beside |
| Turret | holds fire on the densest trail in its range | food to build, food per shot | fronts arriving at a known gap |
| Wall | blocked ground | food | a straight-running front; loses to loops |
| Bait | a small food pile you drop | the food in it | pulling a front off your base or into a turret |
| Ship | removes food from the larder for points, and its smell with it | the food | lowering your scent before a wave |

Bullets are drawn as what they do: the cleared trail line *is* the tracer,
so nothing on screen is painted on (owner, 2026-11-09 and 2026-11-10).
Soldiers and structures are player pieces, drawn as pieces.

### The mould's "units", all emergent

The mould has no unit list, but players will learn its shapes:

- **Searchers**: thin, scattered walkers following scent. Harmless alone,
  they lay the first trail a front will follow.
- **Fronts**: a dense band moving toward a pile or your larder. These engulf.
- **Trunks**: thick tubes feeding a front from a pile behind it. The target.
- **Loops**: spare tubes that carry food round a cut. The armour.
- **Die-back**: a cut-off piece paling and thinning over seconds. The reward,
  and the sign your cut worked.

### Strategies, and what in the model counters each

The demo bar is several strategies that counter each other. The mould is not
a player, so here the counters come from the state each strategy pushes the
mould into, and from the map. These are hypotheses for the census.

| Strategy | Play | Works when | Fails when, and why |
| --- | --- | --- | --- |
| **Fortress** | hoard, wall in, turrets at the gaps | the mould is lean, so its fronts are thin and come one at a time | the mould is fat: the hoard's smell draws every front at once and loops route round each wall |
| **Hunter** | a mobile squad cuts trunks and holds them | the network is a tree (few loops), so one held cut starves a big piece | the network is loopy (fed mould makes loops), and soldiers far from base get engulfed |
| **Farmer** | many harvesters, ship early, keep the larder low | piles are many and spread, so you out-eat the mould and starve it lean | harvesters are spread thin; a front sitting on a pile takes its share and kills the harvester |
| **Baiter** | spend food on bait to steer fronts into kill zones | turrets are in place and you can afford to throw food away | you are poor; bait feeds the mould if it reaches it |

The hoped-for shape: **Farmer starves the mould lean, a lean mould is a
tree, and a tree is what Hunter needs; a fat mould beats Hunter and Fortress
but is the slowest to reach you, which is Baiter's window.** If the census
finds one strategy wins on every layout, that is the result to fix before
adding a weapon (the field skirmish's soft won 127/128,
[skirmish](../wiki/rts/skirmish.md)).

### Feedback, managed on purpose

- **Your success smells.** Larder scent grows with the larder.
- **The mould's size costs it.** Every walker pays upkeep, and food reaches a
  big network only along long tubes, so the far edge of a big mould is the
  easiest to starve.
- **Attacks are telegraphed.** A front is visible and travels at walking
  speed, and the scent field shows where it is going.
- **A points ending.** No one is wiped out; the round ends and the shipped
  food is counted.

## Other shapes worth keeping

- **Larder Versus** (two squads, one mould). The best answer to "strategies
  that counter each other", because the counterplay is then between people:
  bait near the rival, cut the trunk that feeds the front about to hit you so
  it turns to them, hoard to look strong and draw it. Needs a second player
  or a scripted rival; the census can run it with scripted hands.
- **Forage run.** Shorter, tenser rounds; good if Larder's base-building
  turns out to be one strategy.
- **Supply war.** The player builds pipes the mould can cut, so both sides
  read the same picture. Bigger build; keep for later.

## What must be true, and the first tests

Before building more than one weapon, three cheap runs on today's dish
(CPU, `Params::pieces` on), each with idle and random hands as the controls
and 16 paired seeds, as the census does ([census](../wiki/method/census.md)):

1. **Does held fire matter?** A scripted gunner holds a cleared line across
   the trunk that feeds the most food, for 10 s, against the same line at a
   random place and against idle. Deciding measure: food the mould eats in the
   30 s after. Pass: aimed beats random on at least 13 of 16 (sign test p
   0.02). If it fails, shooting is the knife again, and the shot has to
   scorch, not just clear.
2. **Does the larder's smell draw the mould?** A base pile whose scent scales
   with stored food, at three store levels. Deciding measure: time to first
   contact. Pass: the largest store is reached first on at least 13 of 16.
3. **Lean is a tree, fat is a mesh?** Run the dish at low and high food and
   count loops; repeat test 1 at each. Deciding measure: the aimed cut's
   effect at low food minus at high food, paired by seed. This is what the
   strategy cycle rests on.

Then a plans census (Fortress, Hunter, Farmer, Baiter, random, idle) on three
layouts, read for a cycle or a ladder, with a multiple-comparison correction.

**Frame cost.** A held line clears about length x width cells a step (a few
hundred writes), and a turret scans its range for the densest trail
(range squared cells, every few steps). Both should be well under 0.1 ms a
frame on today's dish; to be measured with `Dish::step_cost` when built.

## How it would be built

- **v0 in the browser, on today's dish.** The `web/` crate already runs the
  mould in a page. One soldier (keys to move, mouse to aim), the rifle,
  harvesters, a wall, bait, ship, and larder smell. About 2,000 walkers on
  256 x 160. Fast to share, and enough to run tests 1-3 and play.
- **v1 at scale, on the GPU.** The big dish has the network a shooter wants
  (more loops, so cutting the right trunk is a real choice) but no economy;
  it waits on GPU phase 4 (game rules on the GPU, [ROADMAP](../ROADMAP.md)
  S4). The open cost is the piece-and-feed search across 1024 x 640.
- **A squad** (several soldiers given orders) can follow v0 if one soldier
  plays well; it turns the game toward an RTS.

## Questions for the owner

- One soldier you steer, or a squad you command?
- Solo against the mould, or Versus (two squads, one mould)?
- Run tests 1-3 first, or build v0 and test inside it?

## Sources

- [knife](../wiki/tools/knife.md), [dead-ends](dead-ends.md) (`Params::light_weight`, `STORM_CUTS`)
- [scale](../wiki/model/scale.md), [sensors](../wiki/model/sensors.md)
- [rts-craft](../wiki/rts/rts-craft.md) and [rts-research-2026-11-10.md](rts-research-2026-11-10.md) (size cost, points ending, telegraphed attacks; Creeper World as the nearest published game with a simulated enemy)
- [skirmish](../wiki/rts/skirmish.md), [two-fevers](../wiki/rts/two-fevers.md)

## Results of the three tests (2026-11-11, after the owner's choice)

Owner, 2026-11-11 00:30Z: "Squad, start just you against the mold, tests
first". Harness: `examples/larderprobe.rs`. Raw output:
[`fire-snip.txt`](mould-shooter-2026-11-11/fire-snip.txt),
[`fire-clear.txt`](mould-shooter-2026-11-11/fire-clear.txt),
[`smell.txt`](mould-shooter-2026-11-11/smell.txt),
[`lean.txt`](mould-shooter-2026-11-11/lean.txt). Page with pictures:
https://claude.ai/artifact/GnXVrCVuddegthHYoxFixA (source in
[`page/`](mould-shooter-2026-11-11/page/index.html)).

**Setup.** The field skirmish's 23-pile field (`skirmish::FIELD`) with one
mould, started on the left home; the right home pile (216, 80; 30,000 food)
is the larder. 90 s a run, 16 paired seeds, a shot every quarter second
while any mould is within 50 cells of the larder. Deciding measure: food
stolen from the larder.

**Look first.** The mould covers the field and first bites the larder at a
median 4.2 s: at 120 steps a second it crosses the dish in about 4 s, so
no front travels long enough to be seen, shot or steered. A held ring of
cuts 14 cells round the larder closes, and a self-fed island of mould
inside it keeps eating. So a fifth hand, firing through the larder itself,
was added before the census.

**Test 1, held fire (snip gun).** Median stolen: idle 12,273; ring aimed at
the thickest tube in 12,213 (lower on 16/16, p 0.00003, but by 0.5%); ring
blind 12,175 (13/16, p 0.021); random anywhere 12,264 (9/16, p 0.80);
**through the larder 1,220** (lower than idle 16/16 and than the aimed ring
16/16, p 0.00003). With a gun that only zeroes trail (no kills, no wound),
every hand stole 12,269-12,275 (7-9 of 16 against idle, p >= 0.80): walkers
lay the trail again at once. **Fails as stated: cutting a supply line does
not protect a pile, because the mould on a pile is fed by that pile.** Only
mould with no food of its own (a front in transit, a tree's branch) starves
when cut. Killing the eaters works, tenfold.

**Test 2, larder smell (idle).** Smell 3 against 0.3 times a full pile's:
first bite sooner on 16/16 (p 0.00003) and theft higher on 16/16, but the
first bite moves from 4.6 s to 4.1 s and theft by under 1%. **Passes the
sign test, fails in size:** on this dish everything is reached in seconds.

**Test 3, lean against fat.** Piles at a quarter of their food: loops at
45 s 1 against 10 (fat more on 16/16, p 0.00003); walkers at the end 341
against 6,900. Starved mould is the tree the design hoped for, with grey
unfed branches. Aimed ring fire saved a median 1% of the larder either way
(lean more on 8 of 12, p 0.39): the piece on the larder lives on the larder.

**Corrections.** 26 comparisons; under Holm every 16/16 result, aim against
random (15/16) and the smell results at 15/16 and 12/12 survive; the blind
ring (p 0.021) does not.

**Cost.** One run of 90 s takes about 11 s on one core, idle or firing; the
shots were not timed apart from the step.

### What it changes

1. **The shot is a kill on mould that is eating**, not a cut. The core verb
   becomes clearing mould off a pile and keeping it off. Cuts keep a place
   in the design only against mould with no food: fronts and branches.
2. **Fronts must travel for the rest of the design to exist** (smell,
   telegraphed attacks, cutting fronts, bait). That needs a map the mould
   takes tens of seconds to cross: the GPU scale (4x the cells), or a
   slower mould on today's dish as a cheap stand-in.
3. **Lean against fat is real** (16/16), so Farmer-then-Hunter has a
   footing once fronts travel.

Next, if the owner agrees: re-run tests 1 and 2 on a dish where the mould
needs 30 s or more to reach the larder (slowed walkers on today's CPU dish
first, since the GPU dish has no economy yet).

## Correction: the snip did not hold, and a ring held from the start stops all theft (2026-11-11, 01:50-02:10Z)

Owner, 01:50Z: "Sound like your shooting is too weak. Shooting should be able
to directly cut through a branch after a free seconds and holding a ring of
fire should stop all mold from getting to the food." Then (01:51Z) "Have 1-3
independent agents review your work" and (01:55Z) a game design review.

**The bug.** A snip wounded only agar, so a snip fired again over an open
wound did nothing, despite `heal_wounds` expecting "a later snip keeps it
open longer". A ring held by repeated snips healed all at once 4 s after its
first shots, and the mould poured in then (seed 1: 600 wound cells to 0.5 s
... 4.0 s, 149 at 5.0 s, walkers inside from 5.0 s). Fixed in `stroke_dab`:
a snip over a wound now keeps it open from that step. `cargo test` passes
(101 tests). The fix touches every dish with snips, but only where two snips
overlap within 4 s.

**The field moved.** Between the first census and this one, the field's
rival home moved half a cell to (215.5, 80) (another thread, addebdb). The
merged harness looked for (216, 80) and panicked on main; the first census
ran on the tree before the move. `LARDER` now reads `skirmish::HOMES[1]`.
Numbers below are on the moved field with the fix.

**Fortress (new hand `fort`): the whole ring of 14 held from 0 s**, five
24-cell lines in turn, four shots a second, whether or not mould is near.
16 seeds: **stolen 0 on every seed** (no bite in 90 s) against idle 12,301
(16/16, p 0.00003); mould ate the rest of the field as before (274,660
against 274,717). Raw: [`fort.txt`](mould-shooter-2026-11-11/fort.txt).

**Test 1 rerun with the fix** ([`fire-snip-fixed.txt`](mould-shooter-2026-11-11/fire-snip-fixed.txt)),
median stolen: idle 12,301; aimed ring raised when mould is near 12,225
(15/16 lower, p 0.0005, 0.6%); blind ring 12,192 (13/16, p 0.021); random
12,299 (9/16); **through the larder 154** (16/16, 98.7% saved). Aim against
blind ring 9/16 (p 0.80): aiming adds nothing on the ring. On mould eaten
over the whole dish, random fire beat aim 16/16 (272,275 against 274,713):
shots anywhere cut the mould's whole economy.

**What the reviews changed** (full text in [`reviews/`](mould-shooter-2026-11-11/reviews/)):

- *The measure is capped* (write-up review): theft is about 12,200-12,300 in
  every condition with any eaters, even the lean dish's few hundred walkers,
  because a pile feeds at most `Params::table` walkers a step. So theft moves
  only when the eaters go to zero. The earlier claim "the mould on a pile is
  fed by that pile" is the picture (a self-fed island inside the late ring),
  but the number is the table cap; both say the same for play: a late ring
  does nothing, a ring before contact or fire on the eaters does everything.
- *The ring was raised late*, not tested as a defence. The fort hand is that
  control: **a ring up before the mould arrives stops all theft.**
- 24 comparisons were printed in the first census, not 26; the Holm
  conclusions stand. The smell test's fair reference is gain 0 (4.2 s), not
  gain 0.3 (4.6 s): against no smell, smell 3 moves the first bite 4.2 s ->
  4.1 s. The lean dish ate all its food in every run (86,250), so its end
  walker count measures starvation; its loops at 45 s stand.
- *The pile hand only culls*: a snip never wounds a pile's own cells, so on
  the larder it kills walkers and clears trail but does not block.
- *Design* (game design review): the ring is now the dominant play ("ring
  the larder, sit"), Hunter and Baiter are empty while fronts do not travel,
  and the economy has a dead state with no food to shoot. It proposes a ring
  priced per cell per second, a larder that runs dry, slowed fronts, and the
  mould given teeth from the model (engulfed soldiers become food, starving
  mould fights harder, drops smelled before they land). Its test: scripted
  plans (ring forever, ring then raid, raid only, farmer, random, idle) on a
  priced ring with slowed walkers; pass if ring forever is not top.

**Cost.** A 90 s run takes about 9-11 s on one core whatever the hand
(287.6 s for 80 runs on four cores); shots were not timed apart from the
step.
