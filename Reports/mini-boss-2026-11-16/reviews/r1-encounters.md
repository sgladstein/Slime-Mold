# Review r1: encounters and pacing

2026-11-16. Lens: encounter and pacing design for action roguelikes and
real-time strategy: what a stronger enemy does to the player's choices
minute to minute (where to stand, what to build, when to retreat, what to
buy), how it paces a planet and a run, and whether a player running several
auto-firing squads can read it. Sections 1 to 6 were written from the brief
alone; the addendum says what I checked afterwards.

## 1. Ideas, before judging

Two shorthands. **Own species**: the enemy's walkers are a separate species
with their own trail field and hue, so they follow each other, not the main
body's veins. **Held bead**: a compact blob of own-species walkers (short
sensor reach, wide sensor angle, heavy deposit, fast trail decay) that moves
as one when a drive field pulls its front harder than its back.

1. **Ram.** A held bead of dense walkers driven by the nearest building's
   scent, keeping heading when blocked, so it runs straight and bites walls
   twice. A bronze bead rolls down a track and grinds through a wall; it
   punishes walls built square across its line and squads in a row.
2. **Siege root.** An own-species cord (long sensor reach, narrow angle, low
   turn) anchored on a crystal, growing one vein toward the nearest pump with
   a dense biting tip fed along the cord. A cut anywhere makes the tip wither.
3. **Spore mortar.** A still dense mound on a crystal whose top walkers turn
   to spores every few seconds, aimed at soldier scent 30 to 40 cells off,
   beyond gun range. Squads that hold still get colonies landing among them.
4. **Bloom.** A mound on a long-held crystal that grows taller and denser on
   a visible clock, then throws every walker it has left out as spores. A
   half-killed bloom bursts small.
5. **Shepherd.** A small bead that writes heavily into the *main* species'
   trail as it follows soldier scent, so ordinary slime falls in behind it.
   A comet on screen; kill the head and the tail spreads out and stalls.
6. **Brood node.** A dense core with a high division rate that sends its new
   walkers out in waves toward soldiers and stores the network's energy. Its
   pulses are real division waves; killing it starves the veins round it.
7. **Carapace.** Dense walkers that read soldier scent but turn across its
   gradient, lining up as a crescent that faces the squads and soaks fire
   while thin slime grows behind. Flanks and explosives beat it.
8. **Leech.** A fast clump driven by pump scent that settles on a pump and
   eats its output. Income falls while a blob on the pump swells; ignored, it
   seeds a colony there.
9. **Murmuration.** Fast single walkers on a short-range trail of their own,
   weighted strongly to each other and weakly to soldier scent, so the cloud
   swirls, darts into range, bites and slips out. Guns waste shots on it.
10. **Turncoat.** A bead that converts ordinary slime it touches to its own
    kind and settings. A colour front crosses the network and the slime
    behind it moves differently.
11. **Rage wave** (not a creature). A contagion that spreads walker to walker
    from a crystal and makes each touched walker surge for a few seconds. A
    band of hue runs along the veins with lunges behind it; a cut stops it.
12. **Tide** (not a creature). A scent field at a pump cluster that every
    walker reads weakly for twenty seconds: the whole body leans and creeps
    one way, then relaxes. Walls and footing matter more than kills.
13. **Grudge.** A bead driven by a mark field that soldiers who hurt it write
    round themselves, so it hunts the last squad that shot it. Baitable.
14. **Sapper.** Dense walkers that read wall cells as food and travel along
    wall edges biting. Gaps open in walls without a fight.
15. **Fire carrier.** Walkers drawn to dry plants whose bites write fire, so
    a flame front is herded toward the base. Clear brush or cut firebreaks.
16. **Splitter.** A bead that, when hit, splits along the alarm gradient into
    two beads that run apart and regroup later. Which half do you chase?
17. **Warden.** A bead tethered by a cord to a crystal it patrols. Cut the
    tether and it panics and charges, then withers: a gamble for the crystal.
18. **Lure crystal.** Dormant walkers lying dim round a free crystal that
    wake on alarm when a squad drains it. A careful player sees the rind.
19. **Minefield.** Single dormant dense walkers on likely paths that wake as a
    soldier passes and write alarm, calling the body.
20. **Snare web.** The thin mesh lays a trail soldiers read: strong trail
    slows them. Glistening threads; fire or scorch opens a lane.
21. **Antibody.** A bead that forms where scar is deepest, resistant to
    whatever did most of the scarring, drawn in that tool's hue. It pushes
    the player to vary their kit.
22. **Pilgrim.** The break-away as a held bead: dense walkers leave a crystal
    for a far fresh one, eat, grow and set off for the next. Bigger at each
    stop; intercept it or lose crystals.
23. **Husk.** A building overrun for some seconds fills with dense walkers in
    its own footprint, which hold and bite: an enemy strongpoint in your lines.
24. **Courier** (a signal). A bright dense walker running along veins with
    food news; where it arrives the slime surges. Kill it in transit.
25. **Encircler.** A bead that reaches a target and spreads into a ring
    (drawn to the scent, pushed off inside a short radius); the radius
    shrinks and the ring closes round a pump.
26. **Chaff thrower.** A bead that sheds single cheap walkers ahead of itself
    so auto-fire spends itself on the nearest specks.
27. **Vein highway** (not a creature). Walkers on very strong trail step
    twice, so thick cords are fast lanes; cutting cords slows every enemy.
28. **Sleeping giant.** A large dim dormant mass on a rich crystal cluster
    that wakes if soldiers linger. An optional prize or ground to avoid.
29. **Cocoon.** A ring of dormant dense walkers round a core that opens after
    a timer and releases a converted species; cracked early, a smaller one.
30. **Pack.** Three small beads on one shared trail, loosely linked, that
    come at a squad from different sides and bite in turn.
31. **Beacon rider.** A clump that latches on to a squad and lays heavy trail
    where it walks, so the body follows. Stop and clear it, or lead the body.

## 2. How they arise

**Triggers.** Drive formation from crystals, in both directions.

- *Ripening (neglect).* Each crystal the slime holds keeps a ripeness count
  that rises while walkers feed there and many are dense (the crystal cell
  reads its own walkers: local and cheap). Past a threshold each second
  carries a chance of a formation, of a kind that stays and builds (mortar,
  bloom, brood, warden). This is the owner's "stronger by holding crystals"
  made visible and put in the player's hands: taking a crystal is income
  *and* defence.
- *Flight (pressure).* When the player takes a crystal, the dense walkers
  cut off there panic and a share condense into a Pilgrim or Grudge instead
  of streaming off as specks. Without this the rate falls as the player wins
  and the second half of a planet goes quiet.
- *The player's habits.* Deep scar seeds Antibodies; pump clusters draw
  Leeches and Roots; long walls draw Sappers and Rams. The counter appears
  where the player concentrates, which reads as the slime answering them.
- *Planet and chance.* The tier picks the pool; chance picks only the kind
  and the second. A director caps the rest: nothing in the first minutes, a
  minimum gap, one alive at a time on planets 1 to 3 and two after, and no
  hatch within 30 s of a raid unless it is that raid's vanguard.

**Warning.** Zoomed in with several squads to watch, the player will be
looking elsewhere for most formations. Three honest warnings: the gathering
pulls walkers toward the site, so the veins *on screen* lean and thicken
toward it (the slime you can see points at the event); the minimap shows
density, so a hot spot appears; and a sound, which is off the screen. Forming
should take 20 to 40 s: time to react from across the map, short enough to
matter.

**The moment of forming** (the clip). *Gather*: veins flow inward, a
break-away in reverse. *Condense*: walkers turn dense and the lump shrinks
as it goes bronze, since each now counts twice. *Cast*: a conversion front
sweeps out from the centre in the new hue; this is the transformation shot.
*Hatch*: it lifts off and leaves a scorched footprint where trail cannot
hold, so its absence shows too.

**Getting there early.** The outcome is size, not yes or no. Shooting during
gather kills ordinary walkers; below a floor the formation aborts and that
mass is gone, a big bite of the 1% bar and a reward for scouting. During
condense dense walkers take two hits and what survives hatches. During cast,
alarm makes it hatch at once, smaller and with fewer traits. The player
trades time for size.

**Endings.** Killed: its mass leaves the bar and it drops something.
Starved: a travelling one carries stored energy as brightness and burns it
as it moves; kite it from food and it dims and withers, so "fight or run" is
a real call. Merged back: if it gets home it converts back and its energy
feeds a surge there, a raider bringing food home. Settled: a Pilgrim that
reaches a fresh crystal becomes a held crystal that starts ripening.
Matured: the staying kinds escalate if left (the bloom bursts).

## 3. Structure and variety

**Four parts, combined.** A kind is a body, a drive, a weapon and an
optional trait.

- Body: bead, cord, mound, cloud, ring.
- Drive: pump scent, building or wall, soldier scent, a mark on one squad, a
  fresh crystal, home.
- Weapon: bite, spores, convert, contagion, fire, alarm call, laying trail.
- Trait: splits when hit, sheds chaff, starts dormant, resists one tool,
  armoured front, fast on veins.

Ram is bead, building, bite; Pilgrim is bead, fresh crystal; Grudge is bead,
mark, bite; Leech is bead, pump, eat; Mortar is mound, soldier, spores;
Bloom is mound, a clock, spores. One working bead rule unlocks about a dozen
kinds, so build bodies first.

**Tie families to tiers.** Each tier already does one named thing; its
mini-bosses should be that thing concentrated, so the player learns the tier
through its champion. Droplets give beads (Ram, Pilgrim, Splitter); the fast
mesh gives clouds and webs (Murmuration, Snare, Pack); thick cords give Root,
Warden, Vein highway; flood-to-hit gives Tide, Encircler, Rage wave; spores
give Mortar, Bloom, Cocoon; the sleeper gives Lure crystal, Minefield,
Sleeping giant.

**Across the run.** Planet 1: one kind, long warning, no traits. Planets 2
and 3: two kinds from the tier. Planets 4 and 5: one trait, flight Pilgrims
begin. Planets 6 and 7: a guest kind from an earlier tier, two traits,
Antibodies live. Planet 8: the boss planet. The shop names the next planet's
pool, which turns buying into counter-picking: range for mortars, explosives
for dense armour, area for clouds. That forecast is the cheapest way to give
shop choices weight.

**Winning and losing.** A kill takes a visible chunk off the bar. Its stored
energy is left as a short-lived food pocket a squad can drain for shop
money, which gives greed a lever: a ripe bloom killed just before it bursts
pays most. A kill can add a themed card to the next shop, so optional fights
widen the run. Ignoring one costs income (Leech, Root), a new held crystal
(Pilgrim), more mass to cut (Bloom) or a building (Husk): visible, partial,
never an instant loss.

**Readability for auto-firing squads.** One silhouette and motion per body
(a bead rolls, a cord inches, a mound pulses, a cloud flickers, a ring
closes); own hue per family; brightness is stored energy, bronze is armour.
Every attack has a wind-up of a second or more that is a sim state: the
ram's walkers bunch at its back before a run, the mortar's top walkers heap
before a launch. No more than two on screen.

**Targeting decides half of these.** Soldiers choose their own targets. If
they take the nearest walker, a mini-boss inside or behind ordinary slime is
shielded for free and the player has no verb to change it. Give soldiers a
priority rule (elite and dense walkers first, when in range) by default or as
a cheap gun card, or Shepherd, Carapace and Chaff will frustrate.

**Cost.** Each species is a trail field diffused over the grid every step.
Put all mini-bosses in one shared elite species, with kind and settings per
walker. Side effect: two elites that meet share trail and may fuse,
combining traits. Cap it, but film it.

## 4. Full bosses

**For.** A run of eight needs a peak, and a known final test turns six shop
visits into a plan. The owner wants transformations that look cool; the
largest belongs at the end. A boss also says the slime has intent.

**Against.** The slime already is each planet's boss: the 1% condition makes
every planet a fight against one body, and a separate monster competes with
that. A big held body is the hardest cohesion case, and a boss that streams
apart reads as a bug. Squads that aim themselves cannot dodge patterns, so
phases collapse into positioning and damage checks, which drifts toward a
health bar. The last planet is already the costliest frame. And a run game
punishes spikes: one fight that ends a well-built run feels unfair.

**My pick: yes, as a boss planet, not a boss creature.** Planet 8's slime is
one organised body. A *heart* sits on the largest crystal cluster, still and
mostly dense, holding a big share of the mass; its raids are real
heartbeats, division waves sent out along cords. Its *organs* are kinds from
earlier planets (a Root, a Mortar, a Brood) fed through cords from the
heart, so cutting a cord makes that organ wither over twenty seconds or so.
The fight is which lines to cut and in what order, which squads can do. At
about half mass the heart hatches a piece of itself and relocates to another
cluster: the big transformation clip, and a chase. Planet 7 previews it with
one organ.

**The last stand, from planet 4.** When the slime falls near its last few
percent, the dense walkers still in reach gather into one held bead and run
for the farthest crystal. It replaces the mop-up (hunting specks across the
map, the dullest stretch of most real-time strategy) with a short chase that
has a climax. On planet 8 the last stand is the heart.

## 5. The owner's first thought

**What it gets right.** It comes out of the simulation, not a spawn table,
and condensation already warns in bronze. "Leaves to seek out the player" is
right for a deeper reason than aggression: once separate from the body, a
mini-boss is the nearest target, so auto-fire finds it and the player sees
it. And variety in behaviour, attack and look is what section 3 organises.

**What I would change.**

- *Rate over dice.* Keep chance for the kind and the second; set the rate
  from what the player can see and change (crystals held, crystals just
  lost). Pure chance in a run game gives blowouts and droughts and nothing to
  plan around.
- *Not only the player.* Seekers of pumps, walls, turrets, crystals and one
  marked squad, per the owner's "attack what they build".
- *Not all should leave.* Mortars and blooms make the player go to them. If
  every threat walks to you, the player never chooses to push, and pushing
  is half of real-time strategy.
- *A staged, interruptible formation* whose outcome is size.
- *Endings besides death:* a fuel gauge to starve it, a way home to merge.

**Condensation.** I take it to be the turning dense of crowded or long-fed
slime. Keep it as the material (armour, double bite) and the first stage of
formation; don't replace it. What I would replace is the break-away's stream
of specks: the break-away is the first mini-boss (the Pilgrim), and fixing
its cohesion is the first experiment.

**Beside what exists.** Give each a time scale. Surges are seconds and local.
Raids are the minute-long rhythm. Mini-bosses are punctuation, each doing one
named thing. The boss planet is the climax. Early in the run keep a
mini-boss off a raid; from planet 5 let raids carry one as their vanguard.
The Shepherd can start surges; the Courier makes a surge's trigger visible.

## 6. Top five, with a first cheap experiment each

Measures are counted in the probe, not in the game step; each quotes its
frame cost against the same dish without it.

**1. Pilgrim (and the held bead under it).** Most kinds need a body that
travels as one, and the break-away already has the destination and the
reason. *Build:* convert a disc of dense walkers at a crystal into the elite
species with tight settings, plus a contact rule: a walker applies its drive
(fresh-crystal scent) only while its own-trail reading is above a threshold,
and below it turns to its strongest own trail. *Film* a 100-cell trip at 24
pixels a cell. *Count* each second the share within 8 cells of the median,
the share that arrives, and speed against a soldier's walk. *Works:* 80% or
more held all the way, half to full soldier speed, reads as one bead.
*Fails:* under half held, or it never arrives; then add a push from behind
(scorch or alarm at its back) first.

**2. Siege root.** It attacks what the player builds and makes "cut or
shoot" a decision, using rules that exist (energy along the body, cut-off
withering). *Build:* elite walkers with long reach, narrow angle, low turn,
pump-scent drive and a dense tip; a dish with a crystal, a pump 60 cells off
and a wall between. *Film* the cord growing and a squad cutting it midway.
*Count* time to the pump, pump damage, seconds from cut to dead tip.
*Works:* the tip dies within about 15 s of a cut, the gap shows, and on
paired seeds cutting saves more pump health than shooting the tip. *Fails:*
the tip carries enough energy to finish the pump anyway; then cut its store,
not its bite.

**3. Spore mortar.** The only kind that changes where to stand: it outranges
the guns, so holding loses and pushing wins. *Build:* a dormant dense mound
on a crystal; every few seconds its top walkers become spores aimed, with
scatter, at the strongest soldier scent within 40 cells. *Film* a squad on
guard at 30 cells, then attack-moving in. *Count* colonies founded a minute
while it holds, walkers alive 30 s after each landing, seconds from
attack-move to the mortar's death, soldiers lost. *Works:* on paired seeds
holding is clearly worse than advancing, and the arcs read at zoom. *Fails:*
spores die on landing, or the mortar kills the squad before it closes
(lower the rate, not the range).

**4. Shepherd.** It makes ordinary slime dangerous without new slime, it is a
priority target, and the comet reads at a glance. *Build:* a small elite
bead on soldier-scent drive writing strongly into the main species' trail.
*Film* the column forming and the head being shot. *Count* main walkers
within 10 cells of the head over time, the share still advancing 10 s after
it dies, and time to kill the head with nearest-target against priority aim.
*Works:* the column breaks within about 5 s, and the aim comparison says
whether priority aim must be default. *Fails:* the main slime ignores the
trail because its other drives dominate.

**5. Bloom.** A clock that says "drop everything and push", the best spike in
planet pacing, with the spread built in and a burst that films well; still,
so cohesion is free. *Build:* ripeness on a held crystal raises a mound's
division and dense share; at full, every walker in it becomes a spore thrown
outward. *Film* 30 s of ripening and the burst. *Count* mound walkers over
time, burst size, colonies alive after 60 s, the rise in peak mass, on
paired seeds with a squad arriving at a quarter, half and three-quarters of
the clock. *Works:* burst size falls smoothly with arrival time, and an
untouched bloom adds a noticeable but not run-ending share of mass (near 10
to 20%). *Fails:* all or nothing, or too small to notice.

Smoke checks for all five: an idle player is not killed in under a minute by
a mini-boss alone; none heals faster than two squads hurt it; nothing flees
forever (fuel runs out). Those are the soft-lock guards.

## Addendum: what I checked afterwards

After writing every section above, I skimmed `wiki/world/run.md`, and
nothing else, to check two claims (what condensation is, what the shop
shows); the same passage corrected two more.

- *Condensation* is what I assumed: from planet 2 crowded slime turns bronze
  and twice as tough and pieces break away for far crystals. But break-away
  "moved nothing" because thick walkers sit boxed in the body, so the
  Pilgrim's gather must pull dense walkers to the body's edge first, or form
  only on edge crystals. That belongs in experiment 1.
- *The shop* already shows a planet briefing, and each tier's "does to you"
  is named on the planet card, so naming the mini-boss pool extends what
  exists. Surges already fire when the slime has eaten enough, so ripening
  can share that food count.
- *Planets are long*: 15:00 on planet 1 to 55:30 on the core world. A gap
  rule alone would flood late planets; the director should set a count per
  planet (three to five early, eight to twelve late) spread over the clock.
- *The mop-up tail* is partly handled: under 3% the slime is "broken" and
  stragglers off food starve. The last stand should fire at that break,
  gathering the dense remnant instead of letting it starve quietly.
