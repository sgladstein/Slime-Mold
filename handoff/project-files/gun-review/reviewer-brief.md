# Brief: weapons for a game against a simulated slime

2026-11-16. Neutral brief for independent design reviewers. It describes the
simulation and the game, not today's weapon cards, on purpose: generate your
own ideas from the model first.

## The game

A top-down real-time game, played zoomed in (about 24 screen pixels a cell).
The player commands **squads of soldiers** RTS-style: move, attack-move,
guard, build. Soldiers **aim and fire on their own** at slime within their
gun's range; the player does not aim individual shots. A run is eight
generated planets, harder each time, with a **shop between planets** that
sells guns, gun cards, squad upgrades, buildings and thrown tools. Income
comes from pumps on energy crystals. The slime wants the crystals too: it
eats them, grows, and attacks soldiers and buildings.

Standing rules from the owner (treat as constraints):

- **Fun over fidelity.** The goal is an interesting simulation turned into a
  game; biology is inspiration only.
- **Everything on screen is the simulation, and looks like it.** No drawn-only
  effects: what a weapon does must be visible because it changed the
  simulation (walkers, fields), not because a sprite was painted on top.
- **An outcome is a spread, not a yes/no**; every tool must deliver
  something you can see.
- **The squad owns its gun.** A gun has rolled stats (range, rate, spread,
  bullets a shot), a **budget** and a number of **slots**. Cards cost budget;
  modifiers stack. Bullets are rolled cards (damage plus speed, range and
  spread multipliers). Guns fire their cards in order, forever, with **no
  recharge and no ammo cost**: rate of fire is the only limit while firing.
- **Luck is the normal distribution itself**: every roll can be a bit
  better or worse, rarely much better; nothing is labelled "lucky".
- **The owner wants complexity**, and wants guns to be one of the main
  draws of the game ("really satisfying"). Inspirations they named: Noita's
  wand spells and the ammo-card guns of Flashes of Chaos (a 2026 top-down
  roguelike shooter), where lucky finds stack into absurd, chaotic builds.

## The simulation (what a weapon can act on)

The slime is not a health bar. It is a **population of walkers** (tens of
thousands, up to ~100k, on a GPU) on a fine grid, after Jeff Jones's
Physarum model:

- Each walker senses a **trail** field ahead of it (three sensors), turns
  toward more trail and more **food scent**, steps, and lays trail. The
  network of veins and blobs you see *is* the walkers and their trail.
- **Food scent** spreads from energy crystals; walkers climb it. Soldiers
  also give off a scent.
- **Growth**: walkers eat at crystals, store energy, divide into new walkers,
  starve without food. Energy is shared along the body, so a part **cut off
  from its food withers**, and a cut-off part **panics**: it leans, then
  charges the nearest soldier or runs for a crystal.
- **Thickening / condensing**: long-fed slime turns dense (fewer, stronger
  walkers); dense lumps can **break away** and run to fresh crystals.
- **Surges**: fed slime near soldiers creeps, then lunges.
- **Alarm** field: some species flee it, some swarm to it, one remembers
  where it was hurt.
- **Ground that holds no trail** for a time (a scorched patch or crater):
  walkers cannot keep their network there, so it acts as a temporary wall.
- A **sleep gas** cloud that drifts, is held by walls and decays, and puts
  walkers to sleep.
- A **contagion** state: sick walkers pay extra upkeep and infect neighbours.
- **Flung walkers**: walkers can be thrown ballistically; they land and live
  on (seeding new growth where they land).
- **Plants** (four kinds) and **fire** that spreads through dry plants by a
  moisture field, burns walkers and soldiers, and is stopped by water, damp
  ground and the slime's own body.
- Terrain: rock, water and rivers, bridges, walls the slime slowly damages.
- Several **species / tiers** differ in how they read these fields (for
  example one ignores alarm, one floods to it, one sleeps until disturbed,
  one spreads by spores).

A shot today is a bullet in flight (it is game state, it can be outgrown by
slime before it lands). When it reaches slime it kills the walkers in a disc
and can write into any of the fields above. Anything that can be expressed as
"at this place, at this moment, change walkers or a field" is cheap to build;
per-bullet behaviour in flight (curving, seeking, bouncing) is game-side code
and also cheap. Hard limits: a GTX 1070 runs the whole simulation, so effects
should be bounded (a few hundred bullets and bursts a frame), and every
effect should run on both the CPU and GPU versions of the world.

## The question

Design the weapon and card system so that **categories feel totally
distinct** (by behaviour and look in the simulation, not by stat size), and
**combinations can compound into absurd, luck-driven chaos** while staying
legible at RTS scale with auto-firing squads.

Work in this order, and write each part before reading on:

1. **Generate first.** List at least 25 distinct weapon effects or card
   ideas, each one line: what it does to the walkers or fields and what the
   player sees. Aim for distinct *verbs* (what changes in the world), not
   bigger numbers. Don't judge yet.
2. **Structure.** Propose how cards are grouped and combined: what kinds of
   card exist, how a gun reads them, how combos arise, and what stops a
   combo from being merely "more damage". Say where luck enters.
3. **Big picture.** What would a typical player's three best builds look
   like on planet 6? Do they differ? What would make them differ by slime
   species or by map? What is missing from this whole framing, and is there
   another direction (not cards in guns at all) worth considering?
4. **Then answer the owner's own questions**, after your own list:
   a. Should a shot be able to carry and release further shots or effects
      (a chain), and how deep?
   b. Seeking or target-finding projectiles: worth it in a game where
      soldiers already aim for you? What would make one interesting here?
   c. Thrown and placed explosives and devices (grenades, mines, charges,
      gas canisters): should they stay a separate tool system from the guns,
      or join the weapon system (for example as a squad's carried slot)?
      Argue both sides, then pick.
5. **Rank** your top 5 ideas and give each a first cheap experiment that
   would show whether it works (what to film, what to count).

Keep it concrete: name the field or walker rule each idea touches. Avoid
anything that only multiplies damage. Do not copy names, art or specific
cards from any existing game; take structures, not content.
