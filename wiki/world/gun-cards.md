---
title: Gun cards by planet
kind: design
status: open
updated: 2026-11-17
sources:
  - Reports/gun-mind-2026-11-16/README.md
  - Reports/gun-readers-2026-11-17/README.md
  - Reports/gun-route-2026-11-17/README.md
  - Reports/gun-seek-2026-11-16/README.md
  - Reports/gun-shove-2026-11-16/README.md
  - Reports/mould-tiers-2026-11-11.md
  - Reports/rounds-2026-11-12.md
  - Reports/squad-toolkit-round2-2026-11-12.md
code:
  - Card::tier
  - power_from
  - shells_at
---

# Gun cards by planet

A sub-page of [Guns, bullets and gun cards](guns.md), asked for by the game
designer (11-17 03:53Z, via the guns look thread) as a design step, not a
census: each card, the run planet it is the best pick against, and what it
does that you can see. A card with no planet is a merge or cut candidate; a
planet with no card is a gap.

## Now

Only two cards have a planet settled by a census: the blast against the
Drifter and the germ against the Sporer, each 30 of 32 paired seeds after a
Holm correction ([toolkit round 2](../../Reports/squad-toolkit-round2-2026-11-12.md),
*Confirmed at 32 seeds*); the germ was retuned per planet since (11-17,
seed 0), still strongest on the Sporer and Drifter. Five more rest on a
narrower measure (Corral, Pull, Seek, Detonate, Flare); the rest of the
table is this thread's judgement of each card against
[the tier's counter](mould-tiers.md). The gaps are the Grudge and the
Trunk; Pull and Thrift answer no planet.

## Cards

The planets are `run::LADDER`: 1 Drifter, 2 Sprawler, 3 Trunk, 4 Swarmer, 5
Sporer, 6 Sleeper, 7 Grudge. A card is sold from the planet its tier opens
(`Card::tier`, `power_from`); payloads come as rolled bullets
(`shells_at`). **M** marks a measured row (its report in Sources), **J**
this thread's judgement.

| Card | Sold from | Best against | | Why | What you see |
|---|---|---|---|---|---|
| Plain | 1 | Drifter | M | any squad beats it | a small bite at the hit |
| Blast | 1 | Drifter; Trunk, Sleeper | M | the loud plan leads on these three, held on the Drifter | a crater; the slime parts round it |
| Rate | 1 | Drifter | J | more of the bites that already win | faster fire |
| Sour | 2 | Drifter | J | it follows food; sour turns it away | the vein frays into fog, then knits back |
| Scatter | 2 | Sprawler | J | small hits over a thin mesh | three small hits in a ring |
| Split | 2 | Sprawler | J | pieces over a mesh | the bullet breaks where it lands |
| Scorch | 2 | Sprawler | J | a line it turns from, as it turns from walls | a burnt line the slime avoids |
| Seek | 2 | Sprawler | M | near misses on thin slime become hits | near misses strike the slime beside them |
| Blind | 2 | Sprawler | J | a fast mesh walks straight into walls, water, fire | walkers leave the vein in straight lines |
| Pierce | 1 | Trunk | J | runs on along a long cord | the bullet carries on past its first hit |
| Trigger, Expire | 2 | Trunk | J | carry a payload past the cords to a body | a second payload goes off past the first |
| Smother | 2 | Trunk; Sporer | J | both are starved; it wipes the way to food | the slime loses its line to a crystal |
| Gas | 1 | Swarmer | J | the charge sleeps before it bites | a grey, still patch |
| Mist | 2 | Swarmer | J | a sleep line across the charge | a thin grey line along the flight |
| Thick | 2 | Swarmer | J | waits for the crowd a swarm brings | the hit waits until slime crowds it |
| Spin | 2 | Swarmer | J | rings stop a charge | the vein beads into knots |
| Heave | 2 | Swarmer | J | pushes the swarm back | the slime shoved as a block |
| Corral | 2 | Swarmer | M | herded into turrets it dies faster (the swarm's counter is a kill zone) | the slime shoved toward a turret |
| Shatter | 2 | Swarmer, after gas | J | the gassed charge dies at once | the grey patch is gone and the hole stays |
| Bomb | 4 | Swarmer | J | throws a rushing swarm back off the squad | the slime flung out in a ring |
| Germ | 4 | strong vs Sporer and Drifter, weak vs Swarmer and Grudge; useless on a sleeping Sleeper, after a Flare strong | M | each planet's slime resists it (`germ_resist`) or is frail to it (`germ_upkeep`); a sleeping crust neither catches it nor starves | red runs along the veins, then the body thins to a net |
| Detonate | 4 | Sporer, after the germ | M | less slime at the window's end in 13 of 15 paired windows; cast early it cuts the germ's wave short (one clip) | every sick walker in reach dies at once |
| Bait | 1 | Sporer | J | draws it off its crystals without a hit (its counter: starve it, don't hit it) | a scent line the slime follows |
| Flare | 2 | Swarmer, Sleeper | M | a decoy: they walk to its alarm (clips); it wakes the Sleeper for a germ; shape C backs off from it and in play loses kills | an alarm haze; the slime comes to it or backs away |
| Quiet | 1 | Sleeper; Swarmer | J | no alarm to wake the crust or draw the swarm; keeps gas asleep for Shatter | nothing: the slime does not stir |
| Delay | 1 | Grudge | J | goes off when the slime comes back to where it was shot | the hit lodges, then goes off |
| Range | 1 | Grudge | J | fire from new spots, out of a scar's reach | longer shots |
| Wide | 1 | any | | grows any payload | the payload's disc grows |
| Multicast | 4 | any | | two payloads in one shot | two payloads go off together |
| Thrift | 2 | none | | a budget tweak | nothing |
| Pull | 2 | none | M | stops short of the squad, but adds no kills where squads fight | the slime dragged to just short of the squad |

**Gaps.** The Sleeper ignores every round
([rounds](../../Reports/rounds-2026-11-12.md)); its measured counter is a
decoy, a harmless shock at the far edge before the strike
([tiers](../../Reports/mould-tiers-2026-11-11.md)), which the Flare now is
(it draws a woken crust onto empty ground; no window on that planet yet). Shatter reads gas sleep only,
not the Sleeper's own dormancy; reading it would make one card answer a
whole planet, a call for the designer or the owner. The Grudge is beaten by
moving, not by a card. The Trunk is beaten by starving it, which waits on
the cut family.

**Merge or cut candidates.** Pull and Thrift (no planet); Heave and Corral
(one shove, two cards); Trigger and Expire; Gas and Mist. Wide and Multicast
are general on purpose. Blind and Sour may not read apart (the Mind quiz is
with the owner).

## Open

- A J row becomes M when a run pits the card against its planet, paired
  windows as for seek and the readers.
- The germ, retuned per planet on the game designer's call (11-17 05:40Z
  and 06:41Z): the round's sick pay a sixth of the toolkit's upkeep, each
  planet's slime has a resistance and a frailty, seed 0 only
  ([route](../../Reports/gun-route-2026-11-17/README.md)). The first pass
  was measured with a GPU-world bug (the first germ also infected the
  map's middle) and redone; the Drifter's 43.5% the designer counts in band
  (08:05Z). Whether an early Detonate cutting the germ short is the card's
  lesson or a tuning note is the designer's call
  ([readers](../../Reports/gun-readers-2026-11-17/README.md)).
- The Road (bullets lay trail) was dropped: the squad's own shots stopped
  on it ([dead ends](../../Reports/dead-ends.md)).
- Whether the run shop weights offers toward the next planet's answers is
  the [run](run.md) thread's call.
- The guns look thread offered a "what you see" pass.

## Sources

- [toolkit round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Confirmed at 32 seeds*: Blast, Plain, Germ.
- [mould tiers](../../Reports/mould-tiers-2026-11-11.md): each tier's counter.
- [rounds](../../Reports/rounds-2026-11-12.md): the Sleeper ignores every round.
- [shove cards](../../Reports/gun-shove-2026-11-16/README.md): Corral, Pull. [seek](../../Reports/gun-seek-2026-11-16/README.md): Seek. [readers](../../Reports/gun-readers-2026-11-17/README.md): Detonate, Shatter. [Mind bullets](../../Reports/gun-mind-2026-11-16/README.md): Blind, Sour, Spin. [route](../../Reports/gun-route-2026-11-17/README.md): Flare, the germ by planet.
