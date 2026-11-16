# Brief: stronger enemies that form out of a simulated slime

2026-11-16. Neutral brief for independent designers. It describes the
simulation and the game, not any one answer, on purpose: generate your own
ideas from the model first. The owner's own first thought comes at the end
of the question, after your list.

## The game

A top-down real-time game, played **zoomed in** (about 24 screen pixels a
grid cell; everything must read crisply at that zoom). The player commands
**squads of soldiers** RTS-style: move, attack-move, guard, drain a crystal,
build. Soldiers **aim and fire on their own** at slime within their gun's
range (about 24 cells); the player does not aim shots. Soldiers do not need
feeding. Buildings: pumps on energy crystals (the main income), turrets,
walls. Thrown tools exist (explosives, gas and so on).

A **run** is eight generated planets, each a map with one slime, harder each
planet, with a **shop between planets** (guns, gun cards, upgrades,
buildings, tools). A planet is won by cutting the slime to 1% of its peak
mass; lost when the squads die. Each planet's slime is a different **tier**
(a set of rule settings: drifting droplets, a fast thin mesh, thick cords,
one that floods to where it is hit, one that bursts into spores when struck,
one that sleeps until shot), and each does one named thing to the player.

Standing rules from the owner (treat as constraints):

- **Fun over fidelity.** The goal is an interesting simulation turned into a
  game; biology is inspiration, not the bar.
- **Everything on screen is the simulation, and looks like it.** No
  drawn-only effects, no sprite pasted on top: an enemy must *be* walkers
  and fields, and its look must come from what they are doing. (Lighting,
  colour ramps and shading of real data are fine.)
- **An outcome is a spread, not a yes/no**; everything must deliver
  something the player can see.
- **The owner wants complexity and variety**, and has said the enemy needs
  "more interesting progression": it should press the player, attack what
  they build, and keep getting stronger by holding crystals.
- **A visual must be shown as a moving clip**, and must "look cool",
  especially transformations.
- Costs: one GTX 1070 runs the whole simulation and the view (today about
  5-6 ms a frame on early planets, 12-14 ms on the last); every rule should
  run on both the CPU and GPU versions. Per-walker rules and local field
  writes are cheap; anything global a step is not.

## The simulation (what an enemy can be made of)

The slime is not a health bar. It is a **population of walkers** (tens of
thousands, up to a few hundred thousand on the GPU) on a fine grid, after
Jeff Jones's Physarum model:

- Each walker senses a **trail** field ahead of it with three sensors, turns
  toward more trail and more **food scent**, steps, and lays trail. A cell
  holds a limited number of walkers. The network of veins and blobs on
  screen *is* the walkers and their trail.
- **Food scent** spreads from energy crystals; walkers climb it. Soldiers
  give off a weaker scent; pumps can too.
- **Growth**: walkers eat at crystals, store energy, divide into free cells,
  starve without food. Energy is shared along the body, so a part **cut off
  from food withers**, and news of food travels through the body: a part
  with no news **panics** (leans, then charges a nearby soldier or runs for
  a crystal).
- **Dense walkers**: a walker can be a dense kind that counts twice, takes
  two hits and bites twice. Slime that is crowded or long fed on a crystal
  turns dense (drawn bronze), and a dense piece can **break away** and run
  to a far fresh crystal (today it streams there as specks, not as a held
  body).
- **Surges** (fed slime near soldiers creeps, then lunges) and **raids**
  (every minute or so a share of the slime walks round walls to the
  player's pumps and buildings) already exist.
- **Species**: several walker populations can share a dish, each with its
  own trail field and its own settings (sensor reach and angle, turn, step,
  crowd limit, deposit), drawn in its own hue. One species can **convert**
  another's walkers on contact.
- **Life states**: a walker can be a **spore** (flies ballistically, lands,
  lives on), **dormant** (asleep, wakes on scent or alarm), or dead.
  Walkers can also be **flung** by explosions and land alive.
- Fields: an **alarm** field laid where walkers are hurt (some tiers flood
  to it, some flee it); a **scar** field that remembers where it was hurt; a
  **contagion** state that spreads between neighbours; a drifting **gas**
  cloud; **scorched** ground that holds no trail for a while; **plants** and
  **fire** that spreads through dry plants; terrain (rock, water, walls the
  slime slowly damages).
- Cohesion is the known hard part: walkers that leave the body tend to
  stream out as a line of specks rather than move as one held lump. Tried
  so far: a rule that a walker may only step to a cell touching others
  (splits the slime into separate tight crawling lumps); walkers that keep
  their heading when blocked and lay trail where they stand (coheres into
  many still droplets, which in the published model migrate only when
  something pulls their front and back differently).

A soldier's shot kills the walkers in a small disc and can write into the
fields. Walkers that touch a soldier or building bite it. Anything of the
form "at this place, change walkers or a field", or "walkers of this kind
read these fields with these weights", is cheap to build.

## The question

The owner wants **stronger enemies that arise out of the slime** during a
planet: rarer, more dangerous, more varied in behaviour, attack and look
than the slime around them. Possibly also a **full boss**, which the owner
is less sure of but willing to consider.

Work in this order, and write each part before reading on:

1. **Generate first.** List at least 25 distinct ideas, one or two lines
   each: what the enemy is in walker and field terms, what it does to the
   player, what the player sees. Aim for distinct *verbs and shapes*, not
   bigger numbers. Include some that are not a "creature" at all. Don't
   judge yet.
2. **How they arise.** What makes one appear (a rule of the simulation, the
   player's play, the planet, chance), how the player is warned, what the
   moment of forming looks like, and what happens if the player gets there
   early. How does a stronger enemy end: killed, starved, merged back,
   settled somewhere?
3. **Structure and variety.** How should many kinds be organised so the run
   has variety without becoming a list of one-offs: families, modifiers that
   combine, kinds tied to a tier or a planet, kinds that grow from what the
   player does? What does the player win by beating one (or lose by
   ignoring one)?
4. **Full bosses.** Argue for and against a full boss in this game, then
   pick. If for: what is it in simulation terms, and where in the run does
   it sit?
5. **Then respond to the owner's first thought**, after your own list:
   "random chances (when slime gets concentrated) that a mini-boss appears.
   It can be similar to the condensation event (maybe even replace it). The
   slime forms a stronger, more aggressive sub-unit that leaves to seek out
   the player. There should be variety in behaviour, attack, look." What
   does this get right, what would you change, and how does it sit beside
   the dense walkers, break-aways, surges and raids that already exist?
6. **Rank** your top 5 and give each a first cheap experiment: what to
   build first, what to film, what to count, and what would tell you it
   works or doesn't.

Keep it concrete: name the walker rule or field each idea uses, and say
what makes it hold together as one thing on screen. Avoid anything that only
multiplies health or damage. Do not copy names, art or specific monsters
from any existing game; take structures, not content.
