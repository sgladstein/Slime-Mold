---
title: Guns, bullets and gun cards
kind: design
status: open
updated: 2026-11-18
sources:
  - Reports/card-text-2026-11-15.md
  - Reports/droplets-2026-11-18/README.md
  - Reports/gun-budget-2026-11-14.md
  - Reports/gun-cut-count-2026-11-17/README.md
  - Reports/gun-element-2026-11-17/README.md
  - Reports/gun-feel-2026-11-15/README.md
  - Reports/gun-look-2026-11-15/README.md
  - Reports/gun-mind-2026-11-16/README.md
  - Reports/gun-multicast-2026-11-16/README.md
  - Reports/gun-order-2026-11-14.md
  - Reports/gun-provoke-still-2026-11-17/README.md
  - Reports/gun-range-2026-11-15.md
  - Reports/gun-readers-2026-11-17/README.md
  - Reports/gun-route-2026-11-17/README.md
  - Reports/gun-seek-2026-11-16/README.md
  - Reports/gun-shapes-2026-11-16/README.md
  - Reports/gun-shove-2026-11-16/README.md
  - Reports/gun-system-review-2026-11-16/README.md
  - Reports/gun-tinder-2026-11-17/README.md
  - Reports/gun-triggers-2026-11-16/README.md
  - Reports/guns-2026-11-14.md
  - Reports/no-recharge-2026-11-14.md
  - Reports/own-goal-2026-11-18/README.md
  - Reports/rolled-bullets-2026-11-14.md
  - Reports/sleep-wears-off-2026-11-18/README.md
  - Reports/spray-share-ab-2026-11-15/README.md
code:
  - Ammo::roll
  - BigDish::mind
  - BigDish::read_strike
  - Burst::light
  - Burst::push
  - BurstRules::mind_steps_for
  - Engine::mind_pass
  - Engine::read_pass
  - Engine::snap
  - Game::catches
  - Game::release
  - Game::steer
  - Gun::roll
  - RoundRules::burst_rules
  - Rounds::burning
  - Rounds::cast_cards_at
  - Rounds::flush
  - Run::range
  - Set::shape
  - Squad::gun
  - World::bursts
  - mind_reads
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

Strong single cards wear off or carry a danger to the squad: see *Wear-off and danger* below.

## Wear-off and danger

**Sleep and the Mind statuses wear off** (2026-11-18, after the owner's
playtest: Mist on his turrets "basically stopped everything"): a walker
sleeps once, no gas or cloud lengthens that sleep, and once awake it shrugs
off sleep for 20 s (`KitRules::gas_immune`); a Mind status cannot be given
again until 20 s after it ends (`RoundRules::mind_rest`). GPU engine only.
On his planet 3 Gas alone had kept the squad at 92-97 health against 53-61
with a plain gun, and the Trunk's hardening was not the cause
([report](../../Reports/sleep-wears-off-2026-11-18/README.md)), which also
lists the Noita-rule audit of the other single cards.

Two strong cards are dangerous to the squad (2026-11-19, on): a soldier in
a sleep cloud thick enough to sleep the slime dozes, neither firing nor
walking until it thins (`Rules::doze`), and a blast takes 40 health off every
soldier in its kill disc (`RoundRules::blast_hurt`). Fire on its own (a
soldier or turret, not an aimed volley) holds a payload that would reach a
soldier and lands as a plain shot (`RoundRules::blast_margin`,
`gas_margin`), and Mist lays no cloud by a soldier, so the danger is your
own aim and drifting clouds; with no hold the greedy squad on planet 3
gassed or blasted itself to nothing
([report](../../Reports/own-goal-2026-11-18/README.md)).

## The cards, family by family

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
slime round it toward your nearest turret within `corral_reach` (40 dish
cells), or with none in reach on along its line, farther with each heave;
`Pull` drags it toward the gun. Heave was two cards, Heave and Corral, until
the lane merged them (2026-11-18; `corral` still loads as `heave`). They
steer a slug's own shove too. The shove is a push (`Burst::push`): every
walker in the ring flies one heading, on both engines. With one turret beside
the squad, one heave raised the turret's kills in 10 s from 139 with plain
bullets to 371, the squad's own falling as with every shove; a second heave
by a turret scatters the slime and feeds it less (101). Pull first dragged
the slime onto the squad and lost all eight in 10 s; since the owner's
19:40Z pick it stops short, the pulled slime landing at least
`RoundRules::pull_short` (6 dish cells) in front of the gun, and with no
room it does nothing. The squad then kept 7 of 8 as with plain bullets;
pulls happen at range (18 in 10 s while a squad marched and shot slime
10-24 cells off), with kills about even
([shove cards](../../Reports/gun-shove-2026-11-16/README.md), *2026-11-18*
for the merged card).

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
hits. Version 3's grid steering (`Game::steer`, `seek_turn`) added nothing
on top and was deleted 11-17 (dead ends). Dropped on the way: catching beside the line at every cell
(it struck the thin fringe, not the body: thick-front kills 471 to 266),
catching only on the last leg (a soldier's bullet flies to full range, so
that is far past its target), turning only a bullet with no walker on its
line, and flying over the slime to a mini-boss or crystal feeders
([seek](../../Reports/gun-seek-2026-11-16/README.md)).

**Mind bullets** (2026-11-16, step D, the first bullet family): Blind,
Sour and Spin kill nothing; every walker within `mind_r` (8 dish cells) of
the hit takes a status for `mind_secs` (6 s), one word a walker on both
engines (`BigDish::mind`, `Engine::mind_pass`). A blind walker's readings
are 0 and it takes only the plain rule, so it walks straight on and turns
only off a wall; a sour one's are flipped (`mind_reads`), so it turns from
trail and food; a spinning one curls `spin_curl` (35 degrees) a step after
its turn. Scared, raiding and lump walkers take them too. They cost 2, are
sold from planet 2, take their set's shape, and have their own tab in the
armory. On a thin vein, 10 s each, every status cut the walkers within 8
cells from 19 to 11-12 while it lasted, and sour halved the ring round it
too. The blind quiz on their four clips was dropped 11-17. Spin as a harder turn
and Lace (sensing reach x 0.15 or 2.5) showed nothing and were dropped
([Mind bullets](../../Reports/gun-mind-2026-11-16/README.md)).

**Provoke and Still** (2026-11-17, the Mind family's last two, on): the
simulation-use thread's signal statuses as cards, for the devices' 12 s and
10 s (`BurstRules::mind_steps_for`), cost 2, sold as bullets from planet 2.
Provoked slime hears no food news, so after about 5 s it believes it is cut
off: it goes grey, lets go of what it holds and charges. Still slime neither
charges nor eats. On planet 1 Provoke pulled the slime's film off a crystal
onto a squad's guns (kills 387 against 319 in 16 s), and Still took the bite
out of a raid (8 of 8 soldiers at health 87 against 7 at 71, kills 320
against 585). In six paired windows Still left the squad better off in 4 and
more slime alive in 6 of 6; Provoke changed little where the slime already
charged
([Provoke and Still](../../Reports/gun-provoke-still-2026-11-17/README.md)).

**Readers** (2026-11-17, step D's second family): Shatter and Detonate
kill only the walkers in the state another card put there, within their
reach, on both engines (`BigDish::read_strike`, `Engine::read_pass`):
Shatter the walkers asleep from gas (`read_r`, 12 dish cells, a gas hit's
own), Detonate the sick ones (`detonate_r`, 24, for the sickness that has
spread from the germ's hit; the germ is the family's mark). Each kill
clears the trail on its cell; with no state there it kills nothing beyond
the bullet's own hit. They cost 2; Shatter is sold from planet 2,
Detonate with the germ from planet 4; the armory's Mind tab became
"Special bullets" for both families. On the Fork's thick lobe a gas then a
shatter 3 s later took all 1,062 sleepers and left a hole that stayed open;
a shatter alone took none. On the fixed build (re-filmed 11-17) a germ
kills nothing for 12 s, then the lobe thins to a net (the body 17,628 at
40 s against 25,772); a detonate 2 s after it popped 780 at once but cut
the wave short (19,590). In paired 10 s windows germ+detonate left less
slime in 13 of 15 (p = 0.002). Fire it late, once the wave has spread
(the game designer, 11-17; its words end "let it spread first"): the red
wave on screen follows the sick count, r = 0.91, 1-2 s behind at its end
([readers](../../Reports/gun-readers-2026-11-17/README.md)).

**Route** (2026-11-17, step D's third family): the Flare kills nothing and
burns an alarm (100 over 16 dish cells, again every 0.2 s for 3 s); the
slime's own rule decides the rest. Shape C (planets 1-3) flinches, so it
backs off (within 12 cells 293 to 186); the Swarmer and the Sleeper walk to
alarm and pour onto empty ground toward it (0 to 439 and 412). On shape C in
play it costs kills (fewer in 13 of 15 paired windows): it is the Swarmer's
and Sleeper's decoy. Sold from planet 2 on the Special bullets tab. The Road
(bullets lay trail) stopped the squad's own shots and was dropped. With it
the germ was retuned per planet (Open)
([route](../../Reports/gun-route-2026-11-17/README.md)).

**Element** (2026-11-17, step D's fourth family): the Torch kills nothing
itself and lights every plant within 6 dish cells; the fire kills thin
slime standing in it or caught by its spread, and spares a body damp with
trail. On planets 1-3 in play it killed more than plain bullets in 9 of 9
paired windows (p = 0.004) and left less slime in 8 of 9; against the
blast (cost 3) 7 of 9, not significant. It depends on the ground: strong on
the meadow (350 kills against plain's 230 over six windows), little on
the barrens and the fen. Sold from planet 2 on the
Special bullets tab. Heat in the sensors (`fire_weight`, the designer's
herding idea) was a flag, off: in its one A/B film the slime ringed the
fire instead of crossing it, but more of it burned
([element](../../Reports/gun-element-2026-11-17/README.md)); it was
deleted 11-17 (below). **Tinder**,
the second card (cost 2, sold from planet 2, a trail): every bullet lays
a strip of dry brush along its flight, never within 6 dish cells of a
soldier, that withers in under a minute; it kills nothing, and a fire
runs along it. With the torch it killed more than the torch alone in 9 of
9 paired windows (p = 0.004), most where plants are thin (barrens 159
against 123), and lost no soldier; laid up to the muzzle, every fire ran
back to the squad (23 of 80 soldiers lost). It adds nothing to a scorch
trail. The designer's crossing test (a lit tinder line across a stream of
walkers going to food, one seed, 16-45 s): 214 crossed with no line, 10
with the line, 57 with `fire_weight` -5 and 31 with the simulation-use
thread's `fire_alarm` 50; with either the slime waited at the edge and
crossed the char when the fire died, and did not flow round the line.
The alarm held harder and adds no sensor read, so `fire_weight` was
deleted and `fire_alarm` is the one fire-shyness switch, off
([tinder](../../Reports/gun-tinder-2026-11-17/README.md)).

## The look

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
([gun feel](../../Reports/gun-feel-2026-11-15/README.md)). Since 11-18 the
rim is gone: each droplet is a bead in the slime's gold lit by its key
light, with a small shadow
([droplets](../../Reports/droplets-2026-11-18/README.md)). Camera shake
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
  trigger and expire cards, the owner's shove cards (heave, pull),
  multicast and the GPU read-back batching are built. Step C: seek is
  homing in the shot trace (hits up from 80% to 89% on thin slime, kills
  +3%); aim cards wait (squads on their own hold the defend stance, so an
  aim card changes only which nearby slime they pick). Step D's first
  family, Mind (blind, sour, spin, then provoke and still), is built (its
  blind quiz was dropped 11-17: play shows whether they read); the readers (shatter, detonate), the route's Flare and the
  element's Torch and Tinder are built; force counts as built (the shove
  cards; the game designer, 11-17 12:23Z: no new card). The cut count
  (11-17) found no Cut card worth building: the largest part one thin cut
  could leave without food is under 2% of the slime in most counts, the
  Swarmer's first minute the one exception, and a wall or a squad in a gap
  already makes that cut ([cut count](../../Reports/gun-cut-count-2026-11-17/README.md));
  thinnest-vein aim waits on the same shapes. Next, step E (one device
  slot a squad). Douse (water) is held: the view's wet ground is
  refreshed only with walls and crystals.
- Readers: multicast gas and shatter in one shot is a kill as wide as the
  gas for 8 of a gun's 10 (a blast's is 6 cells for 3); the squad's own
  gunfire wakes gassed slime before a shatter reads it.
- The germ, retuned per planet on the game designer's call (11-17): the
  round's sick pay a sixth of the toolkit's upkeep, and each planet's slime
  has a resistance (`germ_resist`) and a frailty (`germ_upkeep`): strong
  on the Sporer and Drifter, weak on the Swarmer and Grudge, useless on a
  sleeping Sleeper and strong on one a Flare woke. The GPU world's first
  germ had also infected the map's middle (fixed 11-17; a twin with the
  first germ in a planet's corner now fails on the old code,
  [element](../../Reports/gun-element-2026-11-17/README.md)), so the earlier
  "plague" figures were inflated: one round took 48% of shape C, not 76%
  ([route](../../Reports/gun-route-2026-11-17/README.md), seed 0 only).
- **Which card answers which planet**: [gun cards by planet](gun-cards.md)
  (the game designer's note, 11-17). The gaps are the Sleeper, the Grudge
  and the Trunk; Pull and Thrift answer no planet.
- Pull stops short but adds no kills where squads on their own fight (the
  slime they shoot is already near). A heave finds turrets only, not a
  marked spot or other buildings.
- Whether per-shot mana is layered on if fights lack tension (owner 11-14).
- Balance of budgets, prices and rolls waits for the owner's playtest.

## Sources

- [guns](../../Reports/guns-2026-11-14.md), [rolled bullets](../../Reports/rolled-bullets-2026-11-14.md), [no recharge](../../Reports/no-recharge-2026-11-14.md), [gun budget](../../Reports/gun-budget-2026-11-14.md), [gun order](../../Reports/gun-order-2026-11-14.md), [card text](../../Reports/card-text-2026-11-15.md), [gun range](../../Reports/gun-range-2026-11-15.md), [gun look](../../Reports/gun-look-2026-11-15/README.md), [gun-system review](../../Reports/gun-system-review-2026-11-16/README.md), [bullet shapes](../../Reports/gun-shapes-2026-11-16/README.md), [trigger cards](../../Reports/gun-triggers-2026-11-16/README.md), [shove cards](../../Reports/gun-shove-2026-11-16/README.md), [multicast](../../Reports/gun-multicast-2026-11-16/README.md), [seek](../../Reports/gun-seek-2026-11-16/README.md), [Mind bullets](../../Reports/gun-mind-2026-11-16/README.md), [readers](../../Reports/gun-readers-2026-11-17/README.md), [route](../../Reports/gun-route-2026-11-17/README.md), [element](../../Reports/gun-element-2026-11-17/README.md), [tinder](../../Reports/gun-tinder-2026-11-17/README.md), [cut count](../../Reports/gun-cut-count-2026-11-17/README.md).
