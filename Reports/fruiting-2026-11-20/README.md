# Fruiting: a beaten slime races to seed the planet (2026-11-20)

The owner, 11-20 05:46Z, starting the thread "Make the slime a more
interesting enemy": "It is a bit uniform of an enemy sometimes. It can be
interesting really early in a planet when I am poor and weak but once I get
strong enough there is just lots of shooting through masses of uniform
orange. Think outside the box".

Page: https://claude.ai/artifact/XysfKgvTCqNv4xruoPrqDe (source in `page/`, open `index.html`).

## The ideas, from the model

Five, each built from what the slime already simulates (walkers, trail,
food, lumps, spores, the alarm); the thread's first message to the owner:

1. **Fruiting** (built): hurt, the slime streams into a knot that ripens and
   bursts into spores over the planet. Real slime mould fruits when it
   starves.
2. **It reads its losses**: a slime losing walkers faster than it grows
   pulls back out of range, masses, and comes in waves from two or three
   sides at once. Next.
3. **A heartbeat**: a contraction wave from the home crystal every few
   seconds, so the edge surges and ebbs.
4. **Organs**: a rind on the side being shot, fed by a soft core.
5. **Civil war**: once its crystals run dry it splits into two strains that
   eat each other (the Lab's hunter).

Fruiting first because it starts exactly when the player is winning, and it
puts a target with a clock inside the mass.

## What was built

`src/mould/larder_world/fruit.rs` (`FruitRules`, `Game::time_fruit`), on on
every run planet (`run::ENEMY_RULES`), GPU only (`World::can_sow`):

- **When**: from 90 s after the landing, once the squad's kills over the last
  30 s reach a fifth of the slime's mass (`hurt` 0.2), and while the slime is
  over 8% of its peak; one at a time, 40 s from one's end to the next.
- **Where**: the thickest 9 x 9 square of slime (the mould grid) at least
  the guns' reach plus 15 cells from every soldier.
- **The knot**: a lump (`big::Lump`, as the mini-bosses and marches) of 8%
  of the slime (200 to 2,500 walkers), pull 40, leash 1, each member two
  blows. Every 3 s it takes in the slime 2 cells round its rim, up to twice
  what it first held and 2,500.
- **Ripe** after 30 s it bursts: engine.wgsl's `fling` pass with `kir` 2
  (`Engine::sow_pass`) throws every walker within 1.5 knot radii straight
  out as a spore (`SOWN`) for 200 steps; past half its flight (the dish's
  `spore_steps` / 2) it homes on food 36 fine cells out and lands over a
  scent of 1 (`spore_reach`, `spore_land`, now in `run::enemy_keys` on every
  planet; the Sporer's own numbers). It lands with no rest, as living slime.
- **Broken** when shot under 35% of its most members: it is slime again.
- **The look**: `LumpLook::Fruit`, a dark umber skin over its members,
  rust darkening to umber as it ripens (its strength is its ripeness), a
  low light, its members' own cream points, mottled by how tightly they
  pack, rising less than a mini-boss. Spores in the air are drawn as the
  blast's thrown droplets.

Tried and changed in the first films:

- **The slime under 60% of its peak** as a second trigger: on planet 3, run
  seed 1, the strategy bot's squad of four had killed 3,556 when the Trunk
  fell from 43,000 to 26,000 by 1:50 and fruited, the squad nowhere near.
  Deleted (`Reports/dead-ends.md`); the hurt test covers the end of a planet, where a fifth
  of a small slime is a few kills.
- **A tenth of the slime, at most 3,000, growing 3 cells a step**: the
  knot reached 4,945 of planet 1's 11,000 (one grow overshoots its cap) and
  4,327 on planet 3. Now 8%, at most 2,500, 2 cells.
- **A cream core light** washed the packed knot pale tan, the bone
  wanderer's read: the light is a fifth of a mini-boss's now and the cream
  is in its members' points.
- **A stair round the rim at zoom 12**: a stray member's share of the
  walkers is whole however thin, and stopped dead where the body
  Gaussian's nine cells ran out, so the skin ended in squares of that
  window. The fruit's share now fades by its members' cover.

## Measured

Software GPU (llvmpipe), run seed 1, one film each, so first numbers.

- **Forced, planet 1, greedy hand** (`fruit_first=30`, `fruit_hurt=0`): the
  knot formed at 0:31 with 1,113 walkers, held 2,004 at 0:44 and 2,271 from
  0:54, and burst at 1:00 with all 2,271; its place was empty by 1:04, the
  slime 11,894 at 0:58 and 11,805 at 1:04.
- **In play, planet 1, strategy bot** (`strat:econ,go=4,aim=6`, `bank=20000`,
  80 soldiers by 1:30), one game on and one off:

  | | Fruiting on | Fruiting off |
  |---|---|---|
  | First knot | 1:39 to 2:07, 1,836, burst with 1,693 | none |
  | Second knot | 2:47 to 3:17, 1,245, burst with 1,238 | none |
  | Slime at 3:00 | 5,639 | 2,484 |
  | Slime at 3:40 | 1,440 | 31 |
  | Planet cleared | not by 4:00 (562 left) | 3:53 |
  | Walkers killed | 27,517 by 4:00 | 23,999 |
  | Squad health at the end | 0.72 | 0.84 |

  The second knot burst with the squad 23 cells away; in the next ten
  seconds its health fell from 0.85 to 0.73 and four of its five pumps went
  down. The bot does not aim at the knot, so it never breaks one. Each knot
  grew once (856 to 1,836, 528 to 1,245) and then stopped at twice its first
  look.
- **Earlier tuning** (a tenth, at most 3,000, the under-60% trigger on): planet 1 knots
  of 2,341 and 3,599, the slime 10,034 before the first burst and 12,861
  twenty seconds after; planet 3 a knot of 4,327.

## Commands

```
# The fruit forced early, to film it (the hurt test off, from 0:30):
larder-world look=DIR run=1:1 mode=squads hand=greedy set=fruit_first=30 set=fruit_hurt=0 centre=40,115,6 at=28,30,..,90
# As it comes in play: a strong strategy bot, on and off:
larder-world look=DIR run=1:1 mode=squads hand=strat:econ,go=4,aim=6 bank=20000 at=240 [set=fruit=0]
```

The log prints `fruit t ...` each second (members, centre, ripeness, the
nearest soldier) and `fruit ended ...`. Clips: `make_clips.py`.

Not checked: the GTX 1070's frame cost (the burst is one `fling` pass the
second it bursts; the rest is CPU work once a game second, a summed table
over the mould grid), planets 4 to 8, a fruit inside the Trunk's fort walls,
and how often a person breaks one.
