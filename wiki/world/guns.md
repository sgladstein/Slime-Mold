---
title: Guns, bullets and gun cards
kind: design
status: open
updated: 2026-11-16
sources:
  - Reports/card-text-2026-11-15.md
  - Reports/gun-budget-2026-11-14.md
  - Reports/gun-feel-2026-11-15/README.md
  - Reports/gun-look-2026-11-15/README.md
  - Reports/gun-multicast-2026-11-16/README.md
  - Reports/gun-order-2026-11-14.md
  - Reports/gun-range-2026-11-15.md
  - Reports/gun-seek-2026-11-16/README.md
  - Reports/gun-shapes-2026-11-16/README.md
  - Reports/gun-shove-2026-11-16/README.md
  - Reports/gun-system-review-2026-11-16/README.md
  - Reports/gun-triggers-2026-11-16/README.md
  - Reports/guns-2026-11-14.md
  - Reports/no-recharge-2026-11-14.md
  - Reports/rolled-bullets-2026-11-14.md
  - Reports/spray-share-ab-2026-11-15/README.md
code:
  - Ammo::roll
  - Burst::push
  - Engine::snap
  - Game::catches
  - Game::release
  - Game::steer
  - Gun::roll
  - Rounds::flush
  - Run::range
  - Set::shape
  - Squad::gun
  - World::bursts
  - muzzle_flash
  - shells_at
  - spray_one
  - squad::trace
---

# Guns, bullets and gun cards

## Now

A squad owns its gun (`Squad::gun`): numbers for range, rate of fire, spread,
bullets a shot, a card budget and slots, rolled around one of four kinds
(rifle, carbine, long gun, scattergun; `Gun::roll`). A squad with no gun fires
the standard rifle, so the campaign plays as before. The gun fires its cards
left to right with no recharge: every shot carries the next card, and a
payload (blast, bomb, gas, germ) is a bullet. Bullets are rolled cards
(`Ammo::roll`): damage plus speed, range and spread multipliers (0.25x to 4x),
priced by how good the roll is; modifier cards stack (range, rate, thrift,
smother). Cards cost, the gun's budget caps the total, and a split squad's both
halves keep the gun and cards. Card text is one short sentence with its key
number. Deeper planets roll bigger budgets and better stats; the run's shop
sells guns and cards, and the gun range (`Run::range`, start menu) is a free
armory to try any of them against slime ([the run](run.md)). The H loadouts
are test presets, not the game's way to arm. Nothing here is balanced.

**Each bullet set has a shape** (2026-11-16, step A of the gun-system review;
`Set::shape`): a slug's hit shoves the slime around it away from the gun (a
fling with no kill), a needle flies on through two more slime hits, buckshot
fires four pellets in a fan at half the radius; standard is as it was. A
payload takes its set's shape (a needle blast blasts three times in a line, a
buckshot gas is four small clouds). **Blast and bomb are apart**: the blast
kills a 6-cell disc and its crater holds 20 s; the bomb kills 2.5 cells, throws
the slime out to 10 cells away from the gun and holds no ground, in ice blue.
The germ bullet is sold from planet 4 (`shells_at`). Films, counts and why
([bullet shapes](../../Reports/gun-shapes-2026-11-16/README.md)); the owner
named all four sets from unlabelled clips (11-16 05:09Z).

**Trigger cards** (2026-11-16, step B): `Trigger` makes a bullet carry the
cast after it and let it out where it hits slime; `Expire` where it stops
(after its last hit, at the end of its range, or bounced back off a wall). The
carried cast leaves the gun's own turn, so every shot fires both, and is let
out as its own bullets (`Game::release`): a carried buckshot is four pellets.
The carrier's hit clears a hole first, so the carried bullet goes off at the
next slime on its line. A carried cast may carry one more. A carrier draws a
bead of its cargo's colour; the card strip lights both as the next shot. A
bomb carrying a gas burst 13.6 times a second against 7.5 for the two in turn;
a buckshot trigger with a blast is strong
([trigger cards](../../Reports/gun-triggers-2026-11-16/README.md)).

**Shove cards** (2026-11-16, the owner's 05:11Z ask: "corral and push the
slime onto areas. Into a kill zone"): `Heave` makes a bullet's hit shove the
slime round it on along its line, farther with each heave; `Corral` sends the
shove to the nearest turret in reach; `Pull` drags it toward the gun. They
steer a slug's own shove too. The shove is a push (`Burst::push`): every
walker in the ring flies one heading, on both engines. With one turret beside
the squad, corral's turret killed 241 in 3 s against 157 with plain bullets;
with three turrets over 10 s, 1,234 against 617, and no soldier lost. Heave
keeps the slime off the squad but kills little. Pull first dragged the slime
onto the squad and lost all eight in 10 s; since the owner's 19:40Z pick it
stops short, the pulled slime landing at least `RoundRules::pull_short` (6
dish cells) in front of the gun, and with no room it does nothing. The squad
then kept 7 of 8 as with plain bullets; pulls happen at range (18 in 10 s
while a squad marched and shot slime 10-24 cells off), with kills about even
([shove cards](../../Reports/gun-shove-2026-11-16/README.md)).

**Multicast** (2026-11-16, step B): the shot that fires its cast also fires
the next cast beside it, each its own bullets, a little apart; a second
multicast adds a third (`rounds::volley`). The casts beside it leave the
gun's turn, as a trigger's do. With a bomb and a gas it cast 14.0 bursts a
second against 7.5 in turn, like the trigger's 13.6, but the gas goes off
beside the bomb, which throws the slime out of the cloud: 5,086 slept
against the trigger's 9,637. Breadth against depth. With it a step's
bursts run as one batch and the GPU's counts come back in one read
(`World::bursts`, `Rounds::flush`, `Engine::snap`): 207 bursts in 69 reads
over 10 s, each burst counted exactly as alone. On the 1070 the
reads were not stalls: frame p95 held at 5.4 ms, multicast +0.1 ms
([multicast](../../Reports/gun-multicast-2026-11-16/README.md)).

**Seek** (2026-11-16, step C; homing in the shot trace, the owner's 19:38Z
pick): squads on their own strike nothing with about one shot in ten on a
thick front and one in five against thin slime (the owner's 14:24Z
question). A seeking bullet that reaches the point its soldier aimed at
with nothing on its line strikes the slime or walker beside it there, up to
`seek_catch` (2 dish cells) a card either side, in both engines from the
live slime (`squad::trace`, `Game::catches`); it also stops at live walkers
on its line. In paired 10 s windows on thin slime it hit 89.1% of shots
against 80.3% (13 of 13 windows) and killed 554 against 538; the thick
front was unchanged. The hit is the thin edge, so kills rise far less than
hits. Version 3's grid steering (`Game::steer`, `seek_turn`) is off: on top
it added nothing. Dropped on the way: catching beside the line at every cell
(it struck the thin fringe, not the body: thick-front kills 471 to 266),
catching only on the last leg (a soldier's bullet flies to full range, so
that is far past its target), turning only a bullet with no walker on its
line, and flying over the slime to a mini-boss or crystal feeders
([seek](../../Reports/gun-seek-2026-11-16/README.md)).

**The look** (owner 11-15 08:30Z: the guns "should really be one of the
draws of the game"): a muzzle flash at the barrel's mouth whose jets are
the gun kind's (rifle three prongs, carbine short and wide, long gun a
lance, scattergun a fan), a recoil kick, tracers that glide between steps
from the barrel, a hit light when the bullet gets there, and the spray:
every walker a shot kills flies out of the hit back toward the shooter as
a droplet and dies where it lands, so kills are as without it
(`Rules::spray_secs`, `spray=0` turns it off;
[gun look](../../Reports/gun-look-2026-11-15/README.md)). How much of the
kill flies (`Rules::spray_share`, 1 all) went to the owner as a blind
A/B ([spray A/B](../../Reports/spray-share-ab-2026-11-15/README.md)); they
saw no difference, and the droplets were made bolder: their own layer in
the view, a dark rim, white-hot cooling to orange, 0.45 s and 7 cells
([gun feel](../../Reports/gun-feel-2026-11-15/README.md)). Camera shake
and hit-stop on blasts are built and off (`shake=K`, `hitstop=K`; the
review's D6, waiting on the owner).

## Owner's word

The quoted rulings, with times, are in [game decisions](../decisions-game.md):
2026-11-14 16:46-16:47 (the squad owns the gun, bullets are rolled), 18:05-18:38
(no recharge), 15:06-15:35 (costs and a budget) and 2026-11-15 02:57 (short
card text) and 08:30 (the guns a draw of the game).

## Open

- **The gun-system review** (owner 11-16 02:34Z: "five different categories
  of bullets and two of them are just two different sized explosions"): the
  run's bullets hold three real effects and the four sets only lean numbers;
  the trigger card was planned 11-14 and never built. The plan
  ([report](../../Reports/gun-system-review-2026-11-16/README.md)): bullet
  families by what they change in the simulation (cut, route, mind, body,
  force, element) plus readers; rule cards, trigger first; seekers that follow
  the simulation; aim cards for auto squads if they help; one device slot per
  squad on a cooldown; luck that changes rules. Steps A-F, each filmed; cut
  last (the owner, 03:44Z: slime sits on crystals all over, "this may not be
  useful"). Step A (set shapes, blast and bomb apart, germ sold), the
  trigger and expire cards, the owner's shove cards (heave, corral, pull),
  multicast and the GPU read-back batching are built. Step C: seek is
  homing in the shot trace (hits up from 80% to 89% on thin slime, kills
  +3%); aim cards wait (squads on their own hold the defend stance, so an
  aim card changes only which nearby slime they pick). Then step D: bullet
  families, mind first.
- Pull stops short but adds no kills where squads on their own fight (the
  slime they shoot is already near). Corral finds turrets only, not a
  marked spot or other buildings.
- Whether per-shot mana is layered on if fights lack tension (owner 11-14).
- Balance of budgets, prices and rolls waits for the owner's playtest.

## Sources

- [guns](../../Reports/guns-2026-11-14.md), [rolled bullets](../../Reports/rolled-bullets-2026-11-14.md), [no recharge](../../Reports/no-recharge-2026-11-14.md), [gun budget](../../Reports/gun-budget-2026-11-14.md), [gun order](../../Reports/gun-order-2026-11-14.md), [card text](../../Reports/card-text-2026-11-15.md), [gun range](../../Reports/gun-range-2026-11-15.md), [gun look](../../Reports/gun-look-2026-11-15/README.md), [gun-system review](../../Reports/gun-system-review-2026-11-16/README.md), [bullet shapes](../../Reports/gun-shapes-2026-11-16/README.md), [trigger cards](../../Reports/gun-triggers-2026-11-16/README.md), [shove cards](../../Reports/gun-shove-2026-11-16/README.md), [multicast](../../Reports/gun-multicast-2026-11-16/README.md), [seek](../../Reports/gun-seek-2026-11-16/README.md).
