# Review 2: weapons as verbs on the simulation

2026-11-16. Independent review from the neutral brief only. Lens: what a
projectile can do to a walker-and-field model that it could never do to an
enemy with a health bar.

## The big picture, first

A health bar has one verb: subtract. This slime has at least six things a
shot can change: **where walkers die** (topology), **what walkers sense**
(the trail, scent and alarm fields they climb), **how walkers sense** (their
per-walker turning rule), **their energy** (which flows along the body),
**how they move** (impulse and drift), and **what the ground reacts to**
(gas, fire, moisture). A good weapon system gives each of those its own card
family, so each family looks different on screen because it breaks a
different part of the model.

What I think is most likely missing:

1. **Choosing what the squad aims at.** Soldiers aim for themselves, so
   their target policy is the strongest lever the player has, and it is
   probably still "nearest slime". A **Doctrine** card per gun (aim at the
   thinnest vein, the growth tip, the feeding blob, marked walkers, gas
   clouds) turns auto-fire into a decision. It is also where seeking belongs.
2. **The network's shape as the target.** The best shots here make the slime
   hurt itself. Cut a vein and the part beyond withers and panics. Lay false
   scent and it spends energy reaching for nothing. Damage should be the
   setup, and the slime's own rules should deliver the outcome.
3. **A combo layer that lives in the world, not in the gun.** If combos are
   field interactions on the ground, two squads combo with each other, a
   thrown canister combos with a gun, and terrain and species join in. That
   kind of combo is native to an RTS and impossible in a single-hero shooter.
4. **The slime pushing back.** Nothing stops one verb from dominating.
   Real *Physarum* habituates to a repeated bitter stimulus (Boisseau, Vogel
   and Dussutour, 2016). Per-species **habituation** to each stimulus field
   gives a counter you can see: after a while the slime stops flinching at
   the alarm you keep firing, and switching verbs brings the flinch back.
5. **A reason not to kill.** If walkers killed is the real score, every build
   collapses into a kill hose. The stakes are crystals, soldiers and
   buildings, and verbs that move, split or starve the slime have to win
   those stakes visibly.

Other directions worth considering, not all guns:

- **An allied mould.** Seed and feed your own species, which avoids the
  enemy's trail and lays its own. Two coloured networks fighting for one
  dish (a standard multi-species extension of Jones's model) is the most
  "simulation" weapon there is.
- **A commander's hand.** The player paints fields (scent, alarm,
  repellent) on a cooldown; the squads trigger what was painted.
- **Missions about shape**: keep it off this line, cut it into three, starve
  the north lobe. These give the non-lethal verbs a purpose.
- **The slime's own deck.** Between planets it gains mutations that answer
  your verbs (alarm-deaf, re-knits faster), shown in the shop.

## 1. Generate (one line each, unjudged)

Format: *name*: what it changes in the model; what you see.

**Cut (topology)**

1. *Hairline*: piercing 1-cell kill line that severs veins; a dark hairline, then the cut-off side fades and panics.
2. *Conductor*: a charge field floods along trail cells and kills a budget of walkers on the veins (further on wet slime); a vein darkens outward like a lit fuse.
3. *Solvent*: erases trail in a disc, kills nothing; walkers lose their vein, drift as fog, then re-knit.

**Route (writes the fields walkers climb)**

4. *Tracer road*: lays trail along the whole flight path; the slime grows along your lines of fire.
5. *False feast*: writes food scent at impact; a finger of slime reaches an empty spot and withers there.
6. *Scent mask*: zeroes food scent around a crystal; the feeding fan goes slack and wanders.
7. *Decoy musk*: writes soldier scent; surges and panicked fragments charge empty ground (or a mine).
8. *Alarm flare*: writes alarm; a ring pulls back, or floods in, or is remembered, by species.
9. *Repellent halo*: a sensed repulsion gradient; veins bend smoothly around it at a distance.
10. *Scorch streak*: ground that holds no trail along the flight path; a temporary wall drawn by gunfire.
11. *Fuel line*: plants dry grass along the flight path; a fuse waiting for fire.
12. *Wet / Dry rounds*: write moisture; fire stops at a line, or plants turn tinder.

**Mind (the per-walker turning rule, for a while)**

13. *Blindfold*: sensors off; the patch bursts into straight rays that walk into walls, water or fire.
14. *Sour*: trail-following inverted; the vein blows apart into a cloud, then condenses back.
15. *Revulsion*: food scent inverted; a feeding blob lets go of its crystal.
16. *Spin*: rotation bias plus short sensor offset; walkers lay rings that hold them, whirlpools that stop advancing.
17. *Lace / Rope*: retunes sensor angle against rotation angle; the network turns to fine weak mesh, or a few thick cords.
18. *Fever*: faster steps, higher upkeep; frantic streaming, then fading.
19. *Schism*: hit walkers take a new colony id, ignore the main trail and stop sharing energy; a chunk peels away.

**Body (energy and state)**

20. *Drain*: removes stored energy without killing; the patch pales and a dimming spreads back along the vein as the body refills it.
21. *Overfeed*: adds energy and forces early condensing; lumps form, break away and run (to a crystal, or your false feast).
22. *Mark*: a heritable walker tint that later cards act on; a tinted patch that spreads as it divides.
23. *Contagion* (existing): sick walkers pay upkeep and infect neighbours; a slow paling front.

**Force (impulse and drift)**

24. *Fling cone*: throws walkers ballistically; arcs land behind, perhaps on scorch, in fire, across water.
25. *Well*: a drift field toward a point; spiral funnels into a dense knot.
26. *Shockwave*: radial push; a hole with a piled-up dense rim.
27. *Current*: drift along the flight path that pushes walkers, gas and fire; a streamline smear.
28. *Wormhole pair*: two impacts link two discs; the network continues through nothing.

**Spark (readers: act on what is already there)**

29. *Igniter*: lights fire; a flash front through any gas.
30. *Detonator*: each marked walker pops (small fling or kill); tinted patches erupt at once.
31. *Steam*: fire on water makes drifting fog that blinds walkers it touches.
32. *Buried fuse*: sits under the slime until trail density above it passes a threshold, then releases; the slime grows over it, then pops.

**Flight (game-side bullet behaviour)**

33. *Vein rider*: the bullet steers by the slime's own three-sensor rule; it snakes along veins to a trunk or crystal, then releases.
34. *Lob*: arcs over the front and lands behind it, where a cut orphans the most.

## 2. Structure

**Six families, one verb each.** Every payload card belongs to exactly one
family, and each family has its own look:

| Family | Writes | Signature look |
|---|---|---|
| Cut | walker death in thin shapes | dark hairlines, withering downstream |
| Route | trail, scent, alarm, repellent, ground | the slime reroutes |
| Mind | per-walker turning rule (status id + timer) | the network's texture changes: fog, rays, rings, rope |
| Body | energy, mark, sickness | brightness and density change, lumps |
| Force | drift field, ballistic state | motion: arcs, funnels, rims |
| Spark | reads a field, then writes | a front that travels through what was set up |

Plain disc-kill is not a family: it is cheap filler that only removes
walkers. Only the Cut shapes make killing compound.

**How a gun reads its tape.** A gun loops its slots as *phrases*:
`[carrier] [shape] verb`. Shape cards (disc, streak along flight, ring,
cone, conducted) and carrier cards (split, lob, buried fuse, vein rider,
on-hit release) attach to the next verb. Rate of fire spends one phrase per
shot, and bullets-a-shot copies the phrase. On top of the tape sits one
**Doctrine** card, which fills no slot but does cost budget. It sets what the
squad aims at, by reading the simulation: nearest, growth tip, thinnest vein
(a bridge in a coarse trail graph), densest lump, feeding blob, marked
walkers, fleeing fragments, gas cloud, or another squad's field.

**Three layers of combo.**

1. *In one phrase*: shape times verb. Streak plus Scorch is a wall, streak
   plus trail is a road, ring plus alarm is a corral, conducted plus Drain
   is a dimming that runs the length of a vein.
2. *Along the tape*: setup then trigger in one gun (Mark then Detonator,
   Well then disc-kill, Fuel line then Igniter). Fire rate against field
   decay decides whether they meet. A slow gun's gas has already drifted,
   which is a natural source of spread.
3. *In the world*: any field written by anyone is read by any reader. Squad
   A lays gas and squad B, with Doctrine "aim at gas", lights it. A player's
   thrown canister, a dry meadow or an alarm-flooding species are all part
   of the same chemistry. A short table of field-on-field reactions, about
   eight rules, gives most of the chaos: gas plus fire makes a flash, fire
   plus water makes blinding steam, conduct is stronger on wet trail, dry
   ground plus fire races, contagion spreads mark, sleep holds drift-pulled
   walkers in a knot.

**What stops "more damage".**

- Field values saturate (cap 1.0 per cell), so stacking one writer past the
  cap does nothing. More effect needs more *area or space*, not more
  numbers.
- Readers scale with what is on the ground, not with card counts.
- Disc-kill on a fed network is regrown in seconds. Kill only compounds
  through topology (cuts, orphans).
- Habituation: each species carries a tolerance scalar per stimulus that
  rises with exposure and decays over time. Repetition fades and variety
  refreshes. The cost is a handful of floats.
- Optional, and worth testing: dead walkers leave food scent, so a kill
  zone becomes a lure. That makes over-killing feed the slime, and makes
  "kill then lure into a fire" a loop.

**Where luck enters.** Card rolls are normal (potency, radius, duration,
decay), and so are the shop's offers. The tape's timing against field decay
adds more. The simulation's own spread adds the rest: where flung walkers
land, which way a schism chunk runs, how far conduct floods through a random
topology. The important move is to **put rolled parameters near the model's
phase boundaries**. Jones's parameter sweeps show sensor angle, rotation
angle and sensor offset select qualitatively different patterns (fine mesh,
coarse cords, spots, contraction), with sharp boundaries. A Lace roll that
is a bit better makes a slightly finer mesh, and a rare tail roll crosses
into spots and the network falls apart. A Spin roll past its threshold makes
rings that sustain themselves. That is how a normal distribution produces
"absurd" outcomes without any label saying "lucky".

**Legibility.** At most eight phrases a tape, shown as an icon strip in
the UI. Because each family has its own look, a player reads which squad is
which from the slime's texture near it.

## 3. Big picture: planet 6

**Under the current framing** (disc kill plus field writes, and auto-aim
that I assume picks the nearest walker), I expect the three best builds to be: a kill hose (rate
times bullets times radius); fire (it spreads by itself and kills, so it is
the strongest field); and sleep then kill. They look different but are
judged on one axis, walkers removed near my squad per second, so they
converge, and the strongest one wins everywhere.

**Under the proposed framing**, the three best builds differ in kind:

- *Severer*: Hairline plus Lob, with Doctrine "thinnest vein", and a guard
  squad holding Fling or Decoy musk. Cuts orphan whole lobes; the orphans
  panic and charge, and the guard squad turns the charge into a delivery
  into fire or water. Few kills, large mass lost to starvation.
- *Shepherd*: Tracer road, False feast and Revulsion, with Doctrine
  "feeding blob". It peels the slime off crystals and leads it along
  streaks to a burn meadow or a scorched dead end. It wins crystals rather
  than kills.
- *Alchemist*: a gas canister from the tool belt, plus Mark, Conductor and
  Igniter, with Doctrine "gas cloud" or "marked". Big set-piece fronts,
  timing-dependent, the chaotic build.

**Species and maps should pick the build.** An alarm-deaf species breaks
the Shepherd's flares, and an alarm-flooding one turns them into magnets. A
sleeper species makes sleep useless but makes Mark-and-detonate easy on a
still target. A spore species punishes Fling and cuts (spores re-seed) and
rewards Route. A dense, condensing species resists Hairline (few thin
veins) and is weak to Well plus Igniter. On maps, rivers favour Fling and
Conductor, meadows favour Fuel line and Igniter, and open rock favours
Scorch walls.

## 4. The owner's questions

**4a. Chains.** Yes, but think of them as events, not recursion. A carrier
spawns children on *hit*, *land*, *timer* or *field threshold*, and children
inherit a share of the parent's budget. Allow depth 2 normally and depth 3
with one rare card. The value of a chain here is not more bullets; it is
**the second effect landing on ground the first one changed** (Well, then
release kill; Solvent, then Tracer road redraws the slime's route). Bound it
with a global per-frame burst queue (for example 256 bursts). Overflow
defers to later frames and shows as a stagger, which reads fine.

**4b. Seeking.** Homing on the nearest walker is worthless when soldiers
already aim. Seeking is worth it when it **seeks a feature of the
simulation**: a bridge vein, a growth tip, the trunk feeding a crystal, a
marked or sick patch, gas. The best version is the *Vein rider*, a bullet
that steers by the slime's own three-sensor rule and rides the network.
You see it snake along veins to where the body is thickest, driven by the
field the slime lives on. Doctrine
and seekers share one targeting service: a coarse trail graph (for example
128x128 cells) rebuilt about twice a second on the CPU. Finding its bridges
is linear time, well under a millisecond.

**4c. Thrown and placed devices.**

- *For joining*: one budget gives real trade-offs and one shop pool, and a
  squad could drop a carried device on a condition (slime within N).
- *For separation*: guns are automatic, and the throw is the only thing the
  player aims: the player's hand, where intent and setup live. Folding it
  into auto-fire removes the one aimed act, and lets the tape spam setup.
- **Pick: separate systems, one chemistry.** Devices stay player-aimed with
  their own inventory, and they write exactly the same fields that gun
  cards read. Gun cards can therefore trigger devices (Doctrine "aim at
  gas", Igniter, Detonator), so the player sets up and the squads pull the
  trigger. Add one bridge squad upgrade, a *satchel slot* that auto-drops a
  device on a stated condition, priced high, for players who want it.

## Cost and risk notes

Estimates for 100k walkers on a GTX 1070; each needs measuring.

- **Mind status**: 1 byte status id plus 1 byte timer per walker and one
  lookup in a constant table (sensor angle, rotation, offset, step, trail
  and scent sign): likely under 0.1 ms. Risk: Sour and Lace spread the slime
  wider, so it may find new crystals.
- **Drift field** (Well, Shockwave, Current, gas push): one two-channel
  decaying grid field, one extra sample per walker: about 0.1-0.3 ms. Risk:
  Well concentrates mass and may trigger condensing.
- **Charge field** (Conductor): masked diffusion along trail cells, at most
  about 4 iterations a frame and a kill budget per shot. Risk: wiping a
  whole network, which the budget bounds.
- **Route writes**: stamps into existing fields, nearly free. Risk: a Tracer
  road sustains itself after you want it gone (also the point).
- **Fling** reuses the ballistic state. Risk: seeding colonies behind you,
  a deliberate double edge.
- **Schism** needs a colony id and energy sharing that respects it: the most
  invasive change here. **Wormhole** breaks locality; keep it last.
- **Doctrine and seeking**: a 128x128 trail readback (64 KB), pipelined a
  frame late; a synchronous readback would stall.
- **Readability**: Mark tints walkers by state (data, not paint); the
  charge field renders as itself, so the fuse is the field.

## 5. Top 5, each with a first cheap experiment

1. **Doctrine plus Hairline (shoot the topology).** Experiment: a seeded
   dish with three crystals and two identical squads with the same disc-kill
   gun. One uses Doctrine "nearest" and the other "thinnest vein". Film a
   120-second side by side. Count walkers alive at 60 and 120 seconds, the
   number of disconnected components, deaths by starvation against deaths by
   shot, and crystals held. Use 20 paired seeds with a sign test. It works
   if "thinnest vein" wins on crystals held with fewer shot kills.
2. **Mind cards that retune the turning rule** (Blindfold, Sour, Spin,
   Lace/Rope). Experiment: on a steady network, stamp each status in a
   20-cell disc for 10 seconds. Film each at play zoom in a blind lineup and
   ask whether a viewer can name the status from the picture alone. Count
   time to re-knit and the mass displaced. Quote the frame cost of the
   status lookup.
3. **Writers and readers in a shared chemistry, across squads.**
   Experiment: the player throws one gas canister, squad A fires Mark, and
   squad B fires Igniter and Detonator on Doctrine "gas cloud", against the
   same budget spent on disc-kill. Film the flash front. Count crystals held, the mass lost, and
   the worst frame time at maximum gas.
4. **Vein rider (seeking by the slime's own rule).** Experiment: give
   rider bullets a release payload, either Hairline or Drain, against
   straight bullets with the same payload. Film one bullet's path over a
   network. Count the share of releases that land on a bridge vein or trunk,
   and the starvation that follows.
5. **Route verbs: Tracer road, False feast, Revulsion.** Experiment: a squad
   fires tracer streaks from a crystal toward a burn meadow, with no kill
   cards. Count the fraction of slime mass that enters the meadow within 60
   seconds against an idle control, and whether the slime leaves the
   crystal. Film it. It works if the slime visibly follows the lines.

Habituation is a rule, not a card, so it is not ranked; it is the first
thing to add once two verbs work. Test: alarm every 2 s for 60 s at a
fleeing species, plot flee distance per hit, then switch to repellent and see
whether the response returns.
