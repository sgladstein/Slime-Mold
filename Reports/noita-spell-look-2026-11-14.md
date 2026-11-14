# How Noita's spells look, and what that means for our rounds (2026-11-14)

The owner's playtest (2026-11-14 01:49Z) said the Flashes of Chaos bullets
look "really bad". Part of what makes Flashes of Chaos and Noita wands cool
is how good the effects look. The interface also has no way to show the
gun cards. A minute later (01:50Z) the owner added: "The bullets shouldn't
just be flashy effects. Research how noita does their wand animations."
This report covers that research and the design it leads to. Nothing here
is built yet.

## What Noita does

**The world is the effect.** Every pixel is a material in a falling-sand
cellular automaton ("Every pixel in the game follows rather simple rules",
Petri Purho, [80.lv](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/)).
A spell's look is mostly the material it moves, makes or destroys:

- **Trails leave real material.** *Burning Trail* gives a projectile "a
  tail of fire" that can "light burnable materials on fire as well as light
  up dark areas" ([wiki](https://noita.wiki.gg/wiki/Burning_Trail)).
  *Gunpowder Trail* drops gunpowder "in piles to the ground below"
  ([wiki](https://noita.wiki.gg/wiki/Gunpowder_Trail)). There are acid, oil,
  water, poison and rainbow trails too. The trail is the mechanic, and you
  see it because it is there.
- **Mists are real clouds.** A mist is a stationary cloud that stains the
  place and the creatures in it with its material
  ([wiki](https://noita.wiki.gg/wiki/Mist)).
- **Explosions spend energy on the cells.** `ConfigExplosion` casts rays.
  Each ray has `ray_energy` (default 20000), which it spends against each
  cell's hp. Only cells up to `max_durability_to_destroy` break. The
  explosion leaves `create_cell_material` behind (fire by default) and sets
  pixels that are "good candidates" collapsing as sand
  ([ConfigExplosion](https://noita.wiki.gg/wiki/Documentation:_ConfigExplosion),
  [80.lv](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/)).
  A projectile digs only up to its durability. Spark Bolt digs to 8
  ([Spark Bolt](https://noita.wiki.gg/wiki/Spark_Bolt),
  [Materials](https://noita.wiki.gg/wiki/Materials)).
- **Splashes are the world's own pixels in flight.** "When the player jumps
  in the blood, it takes the surrounding pixels and puts them in a particle
  simulation with velocity and gravity". They land back as cells (GDC 2019
  talk, as summarised in
  [macuyiko](https://blog.macuyiko.com/post/2020/an-exploration-of-cellular-automata-and-graph-based-game-systems-part-4.html)).
  `ParticleEmitterComponent` can create its particles as real grid cells
  (`create_real_particles`). Otherwise it makes *cosmetic* particles,
  which "have collisions, but no other physical interactions with the
  world" ([ParticleEmitterComponent](https://noita.wiki.gg/wiki/Documentation:_ParticleEmitterComponent)).
  Even the cosmetic ones hit the world and do not pass through it.
- **The projectile is an entity drawn from its motion.**
  `velocity_sets_rotation` and `velocity_sets_scale` stretch the sprite
  along its flight, and `draw_as_long` draws a particle along its movement
  vector ([ProjectileComponent](https://noita.wiki.gg/wiki/Documentation:_ProjectileComponent)).
  It can dig (`ground_penetration_coeff`, which also depends on mass),
  bounce (`bounces_left`, `bounce_energy`) and explode on death
  (`on_death_explode`).
- **Light is how it reads.** Materials can glow (`gfx_glow`,
  [custom material](https://noita.wiki.gg/wiki/Modding:_Making_a_custom_material)).
  A shot flashes light (`shoot_light_flash_radius`), and an explosion lights
  its surroundings for `light_fade_time` 0.08 s at `light_radius_coeff` 8
  times its radius, warm white (255, 217, 180). It also shakes the camera
  (`camera_shake` 7.5). The burst of light is short. What lasts is the hole,
  the fire and the debris.

So the look is the simulation. The few painted parts (the explosion
sprite, cosmetic sparks, the light flash) are short, and they sit on top
of a world change you can see afterwards.

## What our rounds do now

- A bullet is a game-side point flying at 45 cells a second
  (`Game::bullets`). It is drawn as a flat 3-pixel disc with a straight
  line behind it, in the same colour for every card.
- Where it lands, its cards cast toolkit bursts (`World::burst`). Blast
  kills walkers in a disc and leaves a crater, gas puts walkers to sleep,
  and germ infects them. Pierce and split change the flight.
- The results are in the simulation: the hole in the body, the crater,
  dormant walkers and sick colour. But all of it happens in one frame, it
  is small at the window's zoom, and the bullet itself shows nothing of
  its cards.
- The marks layer has flat colours, and each pixel shows only one mark
  (`world/view.wgsl`, `claim`). Nothing is blended and nothing gives off
  light.

## The design: the same rule as Noita, in our model

Our world is not falling sand. It is walkers, the trail they lay, scent,
alarm, food, walls and craters. The Noita lesson carries over directly: a
card's look should be the field it changes, made legible with light and
motion. In build order:

1. **Bullets are lights, and they light the real slime.** The view already
   shades the world from the mould's own height and gloss (`shaded`). Each
   bullet becomes a point light in that shading, coloured by its payload:
   blast warm white, gas pale violet, germ acid green, a bare shot dim
   yellow. A shot then shows the mould's real relief as it flies over it,
   glinting off thick tubes and dark over bare agar. The bullet itself is
   drawn stretched along its velocity (Noita's `draw_as_long`), as long as
   its last frame's travel, so its speed shows. Wide makes the light and
   the body bigger. Quiet dims it, since quiet is the card that lays less
   alarm. Pierce draws a long thin lance. Split shows the pieces it
   becomes.
2. **The impact lights the world for a moment, then leaves only the real
   change.** A blast lights its own radius times about 3 for about 0.1 s
   (Noita's 0.08 s), then fades. What stays is the crater and the hole in
   the body, which the simulation already keeps. This is a light on real
   relief, not a stamp painted on the slime (the owner's 11-12 04:38Z
   objection to the ember and char looks).
3. **Blasts throw the mould (a simulation change).** This is Noita's
   splash: the world's own matter in flight. A blast kills the walkers at
   its centre and flings the ones at its rim outward. They are real walkers
   moving fast for a few steps, and the view already draws each walker as
   a speck, so the spray is the mould itself. Where they land they keep
   living. So a blast cuts the body but scatters live mould, which is a
   real trade-off between blast and germ, and against the Swarmer tier. It
   goes on both engines, with a twin test.
4. **Gas is a cloud that drifts (a simulation change).** It becomes a real
   field, laid where the shot lands, that diffuses, is blocked by walls,
   and decays. A walker sleeps while it stands in enough gas. You see the
   cloud because it is a field that the view shades, a faint violet haze
   over the ground, and the sleepers are the walkers inside it. It is like
   Noita's mists, which stain what they cover. The squad can walk through
   it (gas spares the squad, as the twin test shows). The field lives in
   binding 48 after the gas words, because there is no binding left.
5. **Trail cards lay real fields along the flight** (new cards, after the
   above). Noita's trails map onto fields we already have. *Scorch trail*
   lays crater along the path, which the mould shuns, so it works as a
   line of fire. *Bait trail* lays food scent, so the mould follows it.
   *Alarm trail* lays alarm, so the mould flees it. Each card is visible
   because it is the field.
6. **A loadout panel in the HUD.** It shows each soldier's round: its cards
   in order, with a small icon and colour for each, the payload first, and
   the cost and recharge. Clicking a card names what it does. H still cycles
   loadouts. The panel is the same for the selected soldier and the squad.

Steps 1, 2 and 6 change the view and the HUD only. Steps 3 and 4 change the
simulation on both engines. Step 5 adds new cards.

## Smoke checks and costs to quote

- The frame cost of the lights in `base` on llvmpipe, and on the 1070 when
  it is free (C6 batch). Lights are capped at 64 a frame. The plan is to
  sum them per pixel, and to bin them into tiles if that costs more than
  0.3 ms.
- Step 3: idle doesn't win, a blast round doesn't end the game in under a
  minute, and walker counts stay exact between the engines.
- Step 4: the gas twin stays exact. The cloud stays inside walls.
- GIFs for each card on a page, with each card's counts (walkers killed,
  flung and gassed) beside its film.

## Sources

- [80.lv: Noita, a game based on falling sand simulation](https://80.lv/articles/noita-a-game-based-on-falling-sand-simulation/)
- [GDC 2019: Exploring the Tech and Design of Noita](https://gdcvault.com/play/1025695/Exploring-the-Tech-and-Design)
  and the [macuyiko write-up](https://blog.macuyiko.com/post/2020/an-exploration-of-cellular-automata-and-graph-based-game-systems-part-4.html)
- Noita wiki: [Spark Bolt](https://noita.wiki.gg/wiki/Spark_Bolt),
  [Burning Trail](https://noita.wiki.gg/wiki/Burning_Trail),
  [Gunpowder Trail](https://noita.wiki.gg/wiki/Gunpowder_Trail),
  [Mist](https://noita.wiki.gg/wiki/Mist),
  [Materials](https://noita.wiki.gg/wiki/Materials),
  [ParticleEmitterComponent](https://noita.wiki.gg/wiki/Documentation:_ParticleEmitterComponent),
  [ConfigExplosion](https://noita.wiki.gg/wiki/Documentation:_ConfigExplosion),
  [ProjectileComponent](https://noita.wiki.gg/wiki/Documentation:_ProjectileComponent),
  [Making a custom material](https://noita.wiki.gg/wiki/Modding:_Making_a_custom_material)
- [dev.to: the falling-everything engine](https://dev.to/wlz/why-noita-bet-on-a-custom-falling-everything-engine-the-pixel-simulation-tech-behind-roguelike-1n22)
  (secondary)

Not found in public sources: how Noita's renderer does bloom, and the
exact moment a flying pixel goes back into the grid. Those are not needed
for the design above.
