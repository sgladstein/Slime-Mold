# The origin, again: a design for Direction (2026-11-22)

The Enemy lane's next job in `NOW.md`: the origin, Milestone 2's way a
planet is won, as a design for Direction before any code.

**Built 11-22** on a branch that lands once the owner calls Milestone 1
done (his card, "Build now, land after M1"). Direction settled the three
questions below as recommended. What was built and measured, and where it
differs from this page (no new engine key for the forming; the die-back's
key is `no_eat`), is in `wiki/world/run.md`, *The origin*. It replaces the [first design of 11-18](../origin-design-2026-11-18/README.md),
because much of what that one proposed has been built since, in other
forms, and two of the owner's rulings since change what the origin should
be.

## What the owner has said

- **The idea** (11-16 04:58Z): "every planet there is a more boss like
  entity at the slime start crystal that you need to kill to win. The
  origin of the slime of the planet."
- **No king** (11-12 20:21Z): "As long as the slime has food/crystal
  energy it can make walkers. Headless." So the origin is something you
  must reach, not something the slime needs to live.
- **A base** (11-18 03:11Z): he pictured the slime's walls "protecting the
  core of the slime or the slime crystals giving them a base of sorts".
- **Crystals** (11-21 06:30Z, "this 100%"): crystals never run out from
  the slime's eating; only the squad empties one by draining it. 06:34Z:
  the slime's size follows the crystal it holds.
- **Tough slime must attack** (11-20 14:16Z): hardened slime "doesn't
  attack you. It makes it easier."
- **Balance** (11-18): no single thing is decisive alone; a strong thing
  must also be dangerous.

## What is built since 11-18 that the origin can stand on

| Built | What it does today | What the origin takes from it |
|---|---|---|
| Mini-bosses (`bosses.rs`, `big::Lump`) | a body of the slime's own armoured walkers held by a leash; squads shoot them first; a broken one drops a core | the body |
| The last-stand heart (`stand.rs`, planet 4 on) | under 20% of its peak against 24+ soldiers, every walker left gathers on the heart crystal and lunges every 20 s | the end of the fight |
| The heart crystal (`Game::heart_site`) | the crystal the slime holds most; the heartbeat pulses from it | where the origin sits |
| The Trunk's fort (#473, planet 3) | an amber ring with four gates round the home crystal and each held crystal | the keep |
| Crystals never run out (`crystal_keep`) and the ceiling (`ceiling.rs`) | the slime eats without emptying a crystal; its size follows the crystals it holds | how you starve it |
| A hit bites into a mini-boss (#676) | hurt members flinch out of a shot's hole | the hurt the player sees |
| The win | the last walker dead (`WIN_SHARE` 0) | what the origin replaces |

## The proposal

**The origin is a standing heart: a body of the slime's walkers on its
home crystal from the landing to the end, which you win the planet by
killing.**

1. **Where.** It forms at the landing on the slime's start crystal, from
   the walkers there, and stays on that crystal (it does not wander with
   `heart_site`). On the Trunk's planet it sits inside the fort's ring.
2. **What it is.** A lump (one of `big::LUMPS`' four slots) of armoured
   members, as a mini-boss, held tight round the crystal by its leash. Its
   life is its members; the HUD shows them against the most it held. Its
   look is the heart's (`LumpLook::Heart`, deep gold), so the player knows
   the slime's heart from the first minute.
3. **It is fed by its crystal.** It takes in walkers that reach its
   crystal, up to a cap set by the planet, but only while the crystal has
   charge. Since only the squad drains a crystal, **draining the home
   crystal is how you starve it**: a pump on its crystal, or soldiers
   standing on it, stop its regrowth. That gives the drain and pump
   players a way in besides the strike, and makes the crystal ruling the
   heart of the planet's end.
4. **It fights back.** It is the last-stand heart from the start, smaller:
   when soldiers come within its reach it lunges (the heart's bout, with
   its cap on the health one bout may take), and when it loses members
   the slime's surge goes for the soldiers near it, so the body comes home
   to defend it.
5. **Its death wins the planet, and the planet shows it.** Under a floor
   (a tenth of its most, or 50 members) it breaks, and the rest of the
   slime dies back: it stops eating, so it starves where it stands over
   about ten seconds. Today a broken slime (`Rules::broken`) only starves
   off food, and with `crystal_keep` a walker on a crystal never starves,
   so the die-back needs its own rule (one engine key: no eating). The
   planet is won when the die-back ends, or at the last walker, whichever
   comes first.
6. **The last-stand heart becomes the origin's last stand.** Late in a
   planet, the remnant gathering (`stand.rs`) gathers into the origin
   instead of forming a second heart, so the end of every planet is one
   fight at one place.

## What it costs

- **GPU engine only** (the owner's card, 11-17), as every new slime rule.
- **One lump slot** of the four (`big::LUMPS`) for the whole planet. The
  slots are shared today by mini-bosses (one at once on planets 2-3, two
  from 4), fruit, raids' marches and the Mortar's shells, so the first
  build counts how often a lump is refused for want of a slot, and if it
  is often, raising `LUMPS` is a GPU-only change (the CPU engine is
  frozen).
- **Files:** a new `larder_world/origin.rs` (forming, regrowth from its
  crystal, its floor and the die-back), `stand.rs` (gather into the
  origin), `run.rs` (the win), and one engine key for the die-back's no
  eating. Other lanes: the HUD's goal line and the origin's look (Look and
  interface), the run bot learning to strike or starve it (Engine and
  tools).
- **Frame cost:** one more lump in passes that already run, and a
  once-a-second count of its crystal's walkers.

## How it would be judged (first build)

- The run bot wins planets 1 to 3 by killing the origin, each in 8 to 12
  minutes (exit test 1's band), and the run in about the same time as now.
- Idle does not win; nothing soft-locks (every home crystal reachable on
  foot; the fort keeps gates open).
- The strike is a fight: walkers that come home in the 30 s after the first
  hit on the origin, against 30 s with no hit.
- Starving works: with a pump on its crystal the origin is smaller at
  its first strike than without.
- The end is shorter: time from the last big fight to the win, before and
  after.

## The risk: the rush

A big squad that walks to the home crystal early may kill the origin in
seconds, which makes the rush stronger than today, where the planet ends
only at the last walker. Its armour, its lunge, the call home and the fort
push against that, but the brake is D7, which the owner holds. The first
build measures the rush (time from landing to win for a squad that goes
straight for it) before any number is tuned, and reports it to Direction.

**Measured (11-22, run seed 1, lavapipe, headless `hand=origin`: every
soldier the bank buys on attack-move to the core; 44, 16 and 9 soldiers
from `set=start_bank=` 16000, 4800 and 2000), at the sizes Engine's
siege tuned (700, 500, 1,000 on planets 1-3; that siege wins seeds 1-3 on
each in under 10 min):**

| Planet (origin) | 44 soldiers | 16 soldiers | 9 soldiers |
|---|---|---|---|
| 1 (700) | won 1:34, none lost | wiped 4:43 | not run at 700 (at 1,200: wiped 4:35) |
| 2 (500) | won 1:47, 15 lost | wiped 1:32 | wiped 1:05 |

On planet 2 the 44 cut it from 686 (a take overshoots its 500 when its
crystal is packed) to 134 by 1:30 and it broke at about 1:37. The decision
on the rush, D7, is the owner's.

## Questions for the owner (Direction to ask, at most once, with these recommendations)

1. **Killing the origin: does the planet end at once, or does the rest of
   the slime die back first?** Recommended: a die-back of about ten
   seconds, so the win is something the simulation shows.
2. **Is draining its crystal a way to beat it?** Recommended: yes, it
   stops the origin's regrowth; it follows from his crystal ruling and
   gives a second way in.
3. **Does killing every walker still win too?** Recommended: yes, as a
   fallback, so no planet can be left unwinnable; with the origin that
   rarely comes first.

Everything else (sizes, armour, reach, the floor) is the Enemy lane's to
set and retune in play.

## Order of building, once Direction says go

1. The origin as a sitting lump on the start crystal from the landing,
   with its regrowth from the crystal, its floor and the win (a GPU test:
   formed at the landing, regrows on a fed crystal and not a drained one,
   the planet is won when it breaks).
2. Its lunge and the call home.
3. The die-back.
4. The last stand gathering into it.
5. Films of the run bot on planets 1 to 3 to the win, and the rush, on a
   page with the counts.
