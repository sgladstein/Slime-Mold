# Review 3: looks, motion and wildcards

2026-11-16. Independent creative director (art and motion), contrarian
seat. Answers `../brief.md` in its order. Every section was written from the
brief alone; the postscript says what I checked afterwards.

My bias, stated up front: Physarum-style agent models make a few shapes very
well (lacy mesh, fat rivers, spots, rings, rotating spirals, pulsing blobs)
and one badly (a held lump that travels). The cheapest route to enemies that
look cool is to pick shapes the model already makes and give them a job.

## 1. Generate first

Moving bodies:

1. **Fist on a stalk.** A dense droplet at the tip of a cord that stays
   plugged into the body; the cord feeds it and it slams walls and pumps.
   Seen as a bronze knot on a lit rope, feed pulses running down the rope.
2. **Comet.** A droplet whose front reads a target scent and whose back
   reads its own fresh trail, so it migrates head first and its wake is its
   own fading trail. Long straight charges, wide turns. A teardrop.
3. **Pinwheel.** A species with a fixed turn bias, so its walkers circulate
   into a rotating spiral of three to five arms that sweep across soldiers
   and bite. A slowly turning galaxy.
4. **Ring.** Wide sensor angle and a strong weight on own trail make a closed
   circulating loop; it rolls over a squad, then tightens. A streaming hoop.
5. **Lantern.** Each walker carries a phase and nudges it toward a
   short-lived phase field; deposit and step rise with phase, so the blob
   breathes every two seconds and bites at full breath.
6. **School.** The "only step next to others" rule used on purpose: a dozen
   tight droplets sharing one heading field, wheeling like starlings,
   splitting round fire and closing behind it.
7. **Hydra.** Two lineages that avoid each other's trail; hit hard, it
   cleaves along the alarm line into halves with different targets.
8. **Shell.** Dense walkers on the rim, soft core, the rim opening into a
   mouth toward whatever it eats. A bronze hoop with one bright gap.
9. **Mortar.** A dense knot stores energy, squeezes, then throws a fan of
   spores in ballistic arcs at buildings.
10. **Shepherd.** A small bright knot writing a "follow" field that ordinary
    slime weights strongly, herding scraps into a moving flock. Kill it and
    the flock relaxes into mesh.
11. **Herald.** A thin fast runner that reaches a pump and writes a beacon
    the next raid follows. One bright thread racing across open ground.
12. **Dodger.** Reads alarm as repellent close up, attractive far off: it
    ripples away from each shot disc, then collapses onto the shooter.
13. **Borer.** A thin dense line that travels inside a wall like a fuse,
    damaging it cell by cell. Light moving through cracks.
14. **Firebrand.** Ignores scorched ground and writes fire through dry
    plants, burning a lane toward walls. An orange-lit front.
15. **Bridger.** Spends walkers into water as raft cells, a field that makes
    water passable to slime. A pontoon of dead slime.
16. **Twins.** Two bodies on one long vein sharing energy; hurt one and the
    other brightens and comes.
17. **Mother.** A slow long-fed lump that bulges, pinches and buds runners.

Places and weathers:

18. **Bloom.** A crystal held long enough roots the ground round it into
    cells that spawn walkers and beat. A place you breach or burn.
19. **Drain.** A point that pulls every walker in a wide radius, so veins
    across the screen turn like rivers into a plughole; left alone it
    becomes a giant.
20. **Revenant.** Slime that reads the scar field as food, regrowing on the
    oldest scars: pale mesh redrawing your old battle lines.
21. **Minefield.** Dormant walkers under plants or scorch that wake in a
    ring on soldier scent.
22. **Spore storm.** A drifting gas-like front carrying spores on a wind and
    dropping them as it passes. A tinted band crossing the minimap.
23. **Blind fog.** A body exhaling a gas inside which soldiers' auto-aim
    reach drops from 24 cells to about 8.

Rhythms and rule changes:

24. **Heartbeat.** The whole network carries a phase; waves run outward from
    the oldest crystal and walkers lunge one step on the beat. Rings of
    light travelling through the mesh every few seconds.
25. **Choir.** Separate bodies whose phases drift into sync; when they
    agree, all lunge at once.
26. **Inversion.** For a minute the slime climbs soldier scent instead of
    food scent; every vein on screen turns to face your squads.
27. **Migration.** The slime abandons its crystals and rolls up behind
    itself toward a fresh region.
28. **Converter tide.** A species that converts slime to its own settings,
    so fast mesh spreads through thick cords and re-tunes them. A hue front
    that changes texture as it passes.

Parasites in the player's things:

29. **Siphon.** A thin cord plugs into a pump and its income runs up the
    cord into the slime. Packets of light travelling away from your pump.
30. **Hitchhikers.** Spores ride soldiers dormant and hatch when the squad
    stands near a crystal or in the base.
31. **Tenant.** Dormant walkers settle in a pump or turret footprint; its
    scent flips sign and it becomes food to the slime.

The end of a planet:

32. **Last Stand.** Below a small share of peak, the remnant condenses into
    one held body that fights: every leftover scrap rerouting to one point.

## 2. How they arise

**What makes one appear.** One visible cause per kind, and chance as a
hazard rate on that cause, not a lottery. Crowding and long feeding grow
organs (Fist, Lantern, Mother, Shell): the chance per second rises with
dense mass times minutes fed, so the player can see it getting likely.
Starvation makes runners: the Comet is what a Fist becomes when its stalk is
cut, and the existing panic rule supplies the charge. Alarm that builds
without killing makes Dodgers and Hydras; old scars make Revenants; a
crystal held past a limit makes a Bloom.

The player's play picks the errand: whatever is nearest the forming site
(walls make a Borer, a pump a Siphon, turrets a Mortar, open squads a
Comet). What hurt the slime recently picks its temper (born in burned ground,
a Firebrand; in gas, a fog-breather), and the planet's tier adds one
modifier. That is how kinds "grow from what the player does" without a
counter-AI: the cause is on the ground where the player can see it.

**Warning,** from the data, over 20 to 40 seconds. On screen, the veins near
the site turn toward it, their flow reverses and the crystal goes bronze. On
the minimap, draw brightness from stored energy rather than mass, so a
gathering is the brightest spot on the map before it moves. In sound, a low
thrum that rises with the knot's energy.

**Forming, as 2-6 second clips,** each in the animator's order of
anticipation, action, settle:

- *Gathering and launch (Fist).* Veins across the frame turn and run into
  one knot; it swells, goes bronze, its core brightens; a stalk shoots
  toward the nearest building and the knot rides out on it. About 5 s.
- *Spin-up (Pinwheel).* A crowded blob's walkers align, the turn bias
  switches on, the edge frays into arms and the arms turn. 3-4 s.
- *Sync (Heartbeat).* Random flicker clumps into a ring and the first wave
  crosses the frame. 3 s, and it reads on the minimap.
- *Hatch (Tenant).* The building's footprint shimmers with waking specks,
  then veins pour from its intake. 2-3 s.
- *Wash (Converter).* A hue front sweeps the body; behind it, cords turn to
  lace. 4-6 s.

**Early arrival is a spread, not a cancel.** If the knot holds less than its
target mass when shot, it aborts into an ordinary panic surge and its dense
walkers scatter as flung specks. Above that, it launches early: smaller,
dimmer, fewer arms. Getting there early changes size and look.

**How one ends.** *Killed:* below a cohesion mass its own-trail field stops
reinforcing itself, the core goes dark and the shape slumps into ordinary
mesh; it dies into the slime, not out of it. *Starved:* a cut-off organ dims
over 20-40 s and panics once. *Merged back:* it returns to a crystal and
dissolves into it, leaving the crystal denser. *Settled:* it reaches a
fresh crystal and becomes a Bloom. *Promoted:* it completes its errand (a
pump tapped, a wall bored) and the planet's slime keeps the gain.

## 3. Structure and variety

**Three axes, not a bestiary.** A kind is a *body* (how it holds and
moves), an *errand* (what it goes for) and a *temper* (the tier modifier).
Bodies: Fist, Comet, Pinwheel, Lantern, School, Shell. Errands: hunt
soldiers, siege a building, siphon a pump, claim a crystal, bore a wall.
Tempers: the tier list. Over a hundred combinations from about fifteen
rules, but the local state picks the body, the base picks the errand and
the planet picks the temper, so each meets the player for a reason. Ship at
most four bodies in a run: bodies are what the eye has to learn.

The second axis is a **planet condition**, one wildcard per planet from
planet 3 (Heartbeat, Spore storm, Revenant, Tenant). They change how every
enemy on that planet behaves, and should carry most of the variety between
planets.

**Telling them apart: shape and tempo, not hue.** Against a dark ground the
screen already carries the slime's hue, dense bronze, the player's colours,
crystal cyan and a hazard colour. Elites should add luminance and motion,
not new hues: stored energy as a brighter, warmer core; a flow sheen from
each cell's summed walker heading, so streams look like they move even in a
still frame; a short bright rim where shots land, from the alarm field. The
exception is the Converter, where moving hue is the verb.

Each body gets one tempo and the ordinary slime none, so any rhythm on
screen means "elite":

| Body | Silhouette | Tempo |
|---|---|---|
| Fist | knot on a rope | feed pulses ~1 Hz, slam every 4-6 s |
| Comet | teardrop, long wake | steady stream, no pulse |
| Pinwheel | spiral, 3-5 arms | one turn every ~8 s |
| Lantern | round blob | breath every ~2 s |
| School | cloud of beads | wheels, splits, rejoins |

Two tests for any body. Turned grey and shrunk to about 4 px a cell, it
still reads as itself beside the others. And every attack has a wind-up the
rules produce (the Fist crowds to its back before a slam, the Mortar
squeezes): anticipation has to be in the simulation, not the renderer.

The minimap cannot honestly show kind; at its scale an elite is a few
pixels. It can show that one exists and where it is going: brightness from
energy plus the elite's own recent trail as a streak. Places read there as
growing patches, weathers as bands, the Heartbeat as rings, which favours
wildcards that are big.

**Winning and losing.** A killed elite falls as a puddle of stored energy
that pumps can tap for a minute, so killing it is shop money; the first kill
of each body in a run could unlock a matching gun card. Ignored, it does its
errand for good: a Siphon keeps the pump's income, a Fist that reaches a
crystal settles as a Bloom, a Borer leaves a hole later raids use. The slime
gets stronger by holding crystals in a way the player watched happen.

## 4. Full bosses

**For.** Eight planets want a summit. The owner wants transformations that
look cool, and a boss is the longest transformation clip the game can
justify. A final fight gives the shop something to build toward.

**Against.** Each planet's slime already is a boss: one organism the size of
the map, beaten by cutting it down. A separate boss unit with phases and a
bar is a health multiplier by another name, and a scripted unit fights the
spread rule. Planet 8 is already the costliest frame (12-14 ms). And a boss
is the cohesion problem at its largest. There is also something a boss can
fix: "cut it to 1% of peak" tends to end in a mop-up, hunting the last
scraps, the dullest minutes of a planet.

**My pick: yes to a boss, no to a boss creature. The boss is the remnant**
(idea 32). At about 15% of peak mass the slime picks its largest surviving
dense knot once (one global choice, not a per-step cost) and every remaining
piece climbs a gather field toward it, the Gathering clip at map scale.
Pieces the player has cut off wither on the way, so the boss's size is a
spread set by the whole planet's play. On planet 8 it changes body at mass
thresholds, using the bodies the player met this run (Fist, then Pinwheel,
then School as it breaks up), each change a transformation clip. It ends
when the planet ends, below 1%. A short version can close every planet from
3 on, with planet 4 as the dress rehearsal. Risk to test: players stopping
at 16% and starving it, so the trigger may need a clock as well as a mass.

## 5. The owner's first thought

**What it gets right.** The cause is concentration, which is in the
simulation and visible. Replacing the condensation event is right: one
forming event done well beats two half-done. A sub-unit that comes for the
player is pressure, and the wider wish (press the player, attack what they
build, get stronger by holding crystals) is the right target.

**What I would change.**

- *Random to readable:* a hazard rate on a visible cause with a 20-40 s
  warning, so a spawn is something the player saw coming.
- *Leaves to stays attached:* at first an organ on a stalk, fed by the body.
  Leaving is what happens when the player cuts the stalk, so "cut off
  withers, no news panics" becomes its second phase for free and cutting is
  the counterplay. A stalk is a vein, the thing this model draws best.
- *Seeks the player to seeks what it wants nearest:* soldiers, pumps, walls
  or a fresh crystal. One that claims crystals is "stronger by holding
  crystals" given a body.
- *Variety from history, not a roll,* as in section 3.

**Beside what exists.** Four systems already say "slime comes at you"; a
fifth only helps if each owns a reach and a tempo. As a ladder: dense
walkers are the *material*; a surge is a *reflex* near soldiers; a raid is
an *errand of the many* on a clock; a mini-boss is an *errand of one* with a
body and a cause; the Last Stand is the *ending*. The break-away is the same
event as a launch and should become it; if cohesion fails it degrades into
what it is today, a stream of specks. Fix cohesion once and both benefit.

**The premise, challenged.** The owner's real problem is "more interesting
enemy progression", and spawning stronger units is the most conventional
answer. Three others deserve an equal hearing. *The organism progresses in
public:* each crystal held gives the whole planet's slime one trait (longer
sensors, longer step, spores on hit), shown as the whole body re-texturing;
it costs a settings change. *The planet is the antagonist:* weathers and
rhythms change the rules for everyone and fill the map, where a zoomed-in
game is weakest at showing scale. *Your own things are the battleground:*
parasites make the base the frontline without another unit walking at you.

My honest view: free-roaming mini-bosses risk turning the one enemy no other
game has, an organism whose anatomy is a map, into an ordinary RTS enemy
with blobs for sprites. Build mini-bosses as organs, tethered and fed, as
the moments where the slime's public progression concentrates, and give
half the progression budget to the slime changing as a whole and to planet
conditions.

## 6. Ranking and first experiments

**1. Fist on a stalk, a Comet when cut.** The owner's idea made
model-native: concentration makes it, it attacks what you build, it hangs
on a vein and cutting is the counterplay. *Build:* on a knot that passes the
hazard roll, weight its walkers strongly on own trail and building scent and
leave the stalk ordinary, the brief's "front and back pulled differently"
condition for a droplet to migrate. *Film:* 6 s at zooms 6 and 24 of the
gathering, the stalk reaching a wall, a slam, then the cut and the charge.
*Count:* the knot's radius of gyration and walker count over 60 s, distance
travelled, wither time after a cut, frame cost. *Works if* the radius stays
within about 30% for a minute while it moves a cell a second or more;
*fails if* it doubles, which means it is streaming into specks.

**2. Siphon.** The clearest parasite and the plainest "attacks what you
build". *Build:* a pump touched by slime for a few seconds is tapped; its
income goes into the touching walkers' energy, which travels the body as
today, drawn as light. *Film:* 4 s of light packets leaving a pump.
*Count:* income lost a minute, and in a blind clip shown to the owner and
two others, which way they say the energy flows. *Works if* most say "away
from the pump" unprompted; *fails if* it reads as a plain vein.

**3. Pinwheel.** The strongest silhouette per line of code: one turn-bias
setting. *Build:* the bias on a sub-population seeded from a crowded blob.
*Film:* the spin-up and 10 s of arms crossing a squad. *Count:* arm count
and period over 2 minutes, bites per sweep, frame cost (about zero). *Works
if* the arms hold for a minute and pass the grey test; *fails if* it decays
into a ring or noise (then try the Ring).

**4. Heartbeat, as a planet condition.** The best wildcard: it changes how
the whole planet plays (move between beats) and reads at full scale on the
minimap. *Build:* a phase byte per walker, a short-lived phase field, a
nudge toward it, a longer step on the beat. *Film:* 6 s of waves crossing
the dish, then the minimap. *Count:* the share of walkers inside a phase
window, whether waves survive a cut in the network, frame cost. *Works if*
coherent waves form within a minute on early-planet settings and show at
both zooms; *fails if* phases never settle or the lunge just makes the slime
faster.

**5. Last Stand.** The boss answer and the cure for the mop-up. *Build:* the
mass trigger, the one-off choice of knot, a gather field. *Film:* the
map-scale gathering on the minimap and at the knot. *Count:* minutes from
15% to the win, before and after, on paired seeds; the share of the remnant
that arrives; whether the planet can still be lost then. *Works if* planet
endings get shorter and the fight is visible; *fails if* the knot never
holds (the same cohesion problem as idea 1) or players wait at 16%.

**The forming moment I would film first** is the Gathering and launch:
veins across the frame turn and flow into one knot, it swells and goes
bronze, its core brightens, a stalk shoots toward the nearest wall and the
knot rides out on it. Five seconds. It is idea 1's birth, the replacement
for condensation and, scaled up, the boss: if it looks cool, three of the
top five are on their way.

## Postscript: checked afterwards

After writing every section, I skimmed `wiki/looks/art-direction.md`
against the look ideas. I then cut the review for length, and made four
small changes the guide prompted: a "white-hot" energy core became a
brighter, warmer one; crystal cyan joined the hue list in section 3; I
dropped a bullet asking for relief lighting of the slime, which the guide
already has; and the first film now names zooms 6 and 24, since looks are
judged at several zooms. Agreements: its "slime looks alive" rule (glossy
relief, one key light) is already the lighting I would ask for; its rule that a withering piece loses light and a panic lean shows a
brighter front matches my Starved and Comet looks; its grey-frame check is
my silhouette test. Two tensions to flag. First, its value order puts light
above your units and thick slime below them, so an elite's energy core must
stay inside the thick-slime band and only flare to light at a real change
(the launch, a slam). Second, it gives each enemy species its own hue; if
Pinwheels or Rings are built as separate species, either they take a hue
(which costs my "shape and tempo, not hue" rule) or they are built as a
sub-population of the planet's slime that keeps its hue, which I prefer.
Danger red is for warnings only, so no elite or its minimap streak should
use it.
