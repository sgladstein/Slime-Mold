# Two Fevers, version 3: putting the mould at the centre (design, 2026-11-10)

**Status: design, nothing built.** The owner, 21:52Z, after playing
version 2: "This is better, but still feel overly simple with 1-2 tools that
lead to mostly 1 strategy with little complexity (although, I didn't fully
understand how hush worked, so many that was the issue). Dig deeper. How do
we make this game more complex, more interesting. How do we take advantage of
the interesting features/abilities of the slime mold. The slime mold seems
more like background in this game and not the integral part."

## Why the mould is background today

In versions 1 and 2 the fever rides the host, but nothing the player does
changes what the host does. The host builds the same network whoever owns
it. A fever spreads across agar as much as along tubes, so the network barely
shapes the war. Owning ground earns nothing, so there is no economy to
build. Hush was the only verb that touched the mould, and it was global and
invisible. The skirmish thread's playtest the same hour came to the same
verdict on its own game, from the opposite side: the mould "explores well on
its own", so a free nudge gives no decision.

So the player needs verbs that act **through the mould's own decisions**:
which food it joins, which routes it thickens, which branches it drops.

## What the mould does by itself (the model, neutrally)

- **It joins food into a network.** Walkers pile onto trail and scent, so
  tubes form between food sources and settle into short routes with a few
  loops.
- **Traffic fattens a route and neglect drops it.** A busy tube thickens. A
  branch nobody walks fades in a few seconds.
- **It re-solves when food changes.** A pile that runs dry loses its tubes,
  and a new one gets joined. With regrowing piles the map never stops moving.
- **A route is one thing used three ways.** The same tube is how walkers
  reach food, where they stand, and the only path along which anything they
  carry can travel.
- **It surprises.** The mould picks the route, not the player. Two pieces
  meeting make a bridge where there was no food (the owner's favourite
  moment in the dish).

## The core tension: an efficient network is an exposed network

The mould's best trick is building an efficient network: a few thick trunks
that feed everything. In a contagion war, a trunk is also a highway. If a
fever travels **only through tube**, faster through thick tube, then:

- a **trunk** feeds you best and carries the enemy's fever into your body
  fastest;
- a **web** of thin, looped tubes survives cuts, but a fever arriving from
  two sides at once takes it;
- **cells** joined by single narrow links can be sealed with one cut, but
  they feed slowly and a cut also starves you.

The mould builds these shapes by itself; the player decides which one it
builds by deciding where it wants food. The same trade-off appears in the
Tokyo rail experiment (Tero et al. 2010), where the mould balances cost,
efficiency and fault tolerance. Here the third axis is how exposed a route is
to the fever.

## The redesign

**Ownership becomes a body with a will.** The walkers you own (your colour,
as in version 2) follow **your call**: they smell a field you lay, and the
rest of the host does not. So your part of the shared mould grows toward your
call and builds its own network there. The mould still chooses the route.

**Fever travels by tube.** Each fever's field spreads only through tube,
carried in proportion to trail thickness, and a walker catches it only while
standing on tube. A gap stops it cold. A thick trunk carries it at speed. A
loop brings it from both sides.

**Income is foraging.** Your walkers on a food pile earn spores, shared with
the rival's walkers there by head count (today's `Pile::bite` rule). Piles
run down and regrow, so a good call goes stale and must move.

**A body cut from your food starves.** A piece of your colour with no tube
route to a pile your walkers eat from fades back to neutral over 10-20 s. The
dish already computes this per piece (`Params::pieces`), so cuts bite.

### The verbs

| Verb | Cost | What the mould does with it | The decision |
|---|---|---|---|
| **Call** (place or move one of 2-3 calls) | free, short cooldown | Your walkers smell it; your body grows a route there and drops routes it no longer needs | Where your food is: income now, and the shape of your network |
| **Seed** | 1 spore | The walkers under it catch your fever and become yours once they recover; the fever runs along the tubes it reaches | Where to attack: a rival trunk is a highway into their body |
| **Pinch** | 1 spore | Your walkers in a small disc stop laying trail for 6 s, so the tube there withers and breaks. Only your own body can be pinched | Seal a route the rival's fever is coming down, at the price of your own supply through it |

Hush becomes Pinch: local, aimed, paid for, and visible as a tube thinning
under your click. The hold (15 s after taking ground) stays.

**What is on screen** is all state. Your colour is ownership, a bright
walker is a carrier, each call is its own scent field drawn as a faint tint,
a pinch is a tube going thin, and a starving piece pales as it reverts.
Spores and both players' calls are visible, so the rival can be read: a
trunk forming toward a pile is their income route and your target.

### How a match runs (3-5 min, two players, mirrored field)

1. Each side owns a home blob on a neutral host, with a call on home.
2. **Expansion:** move your calls to piles. Your body reaches out, and the
   routes it builds decide your exposure.
3. **Contact:** the two bodies' routes meet, and bridges form. Each bridge is
   a road for both fevers.
4. **Raids and seals:** seed down the rival's trunk, and pinch your own when
   their fever comes.
5. **Rewiring:** piles run dry, calls move, and old trunks are abandoned
   while new ones form. The map changes because of what both players asked
   for.

Score: share of the host owned, averaged over the match, as in version 2.

## Strategies it should support

| Plan | Does | Should beat | Should lose to |
|---|---|---|---|
| **Trunk** | One rich call, thick routes, max income | Web (more spores, more seeds) | Raider (one seed rides the trunk home) |
| **Raider** | Spends on seeds early, aims at the rival's thickest tubes | Trunk | Web (no highway, and pinches seal it) |
| **Web** | Calls spread out, many thin looped routes, pinches when attacked | Raider | Trunk (out-earned) |

If this cycle holds, the opening does not decide the match: a player who
reads the rival's shape picks the counter, and the rival can reshape by
moving calls. That gives two kinds of skill, reading and timing, on top of
aim.

## Risks, and the cheap tests that come first

1. **Does a fever by tube behave?** Seed one end of a trunk and of a thin
   tube on the same dish. Pass: the trunk front is at least twice as fast,
   and the fever stops at a 3-cell gap on 16 of 16 seeds. Looked at first.
2. **Does a call reshape the body visibly?** Give one third of the host a
   call on a far pile. Pass: a route to it within 10 s, seen, and that side's
   eaters on the pile rise on 13 of 16. The risk is the field retest's Taut
   result: a strong pull piles walkers onto the food instead of building a
   road (on 7 piles it collapsed; on the 23-pile field it held).
3. **Does a pinch cut?** Pass: the pinched tube is below tube strength
   within 5 s and stays cut for the 6 s, on 16 of 16.
4. **The cycle.** Scripted Trunk, Raider and Web hands, plus random and idle,
   play every matchup on 16 seeds with sides swapped. The deciding measure is
   a cycle whose three links each survive Holm, and each plan beating random
   on 13 of 16.
5. **Snowball.** Income from ownership feeds more seeds. The guard is the
   one from version 2: in mirror games the 1:30 leader still loses in at
   least a quarter.
6. **Cost.** Two call fields and tube-weighted spreading add about two
   spreads (about 0.05 ms a step at today's cost), and piece starvation adds
   a census every few steps (the dish's takes a fraction of a millisecond).

Tests 1-3 take an hour and need no new hands. If any fails, the design
changes before the hands are written.

## Directions considered and ranked lower

- **Each player is their own mould** (the skirmish thread's game). It is the
  same lesson, and the call-pinch-foraging economy would fit there too. The
  two demos could converge once one of them shows a cycle.
- **Traits bought with spores** (Swarm, Roam, Linger, Beckon, Bloom from the
  brainstorm). A trait is a change to how your body senses, so it is a
  second layer on the call. It is worth adding once the base cycle works,
  not before.
- **Persuading a creature nobody owns** (the brainstorm's Custody). It was
  rejected because the mould joins both players' calls and sits still. Here
  each call moves only that player's walkers, which removes that failure.
- **Timed sap waves** (Squeeze). Food flow costs 2.6x the step time, and a
  squeeze from home moved share by 0.000 in the red team's probe.

## Next step

Run tests 1-3 on the current host, look at them, then build Call, Seed by
tube, Pinch, foraging income and starvation as version 3 at the same link,
followed by the three-hand cycle census.
