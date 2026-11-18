---
title: The roguelike run
kind: design
status: open
updated: 2026-11-18
sources:
  - Reports/breakaway-run-2026-11-15/README.md
  - Reports/enemy-switches-2026-11-17/README.md
  - Reports/flinch-2026-11-15/README.md
  - Reports/gun-order-2026-11-14.md
  - Reports/gun-range-2026-11-15.md
  - Reports/lab-steering-2026-11-17/README.md
  - Reports/lure-field-2026-11-18/README.md
  - Reports/menu-fixes-2026-11-15/README.md
  - Reports/mini-boss-2026-11-16/README.md
  - Reports/origin-design-2026-11-18/README.md
  - Reports/planet-things-2026-11-18/README.md
  - Reports/planet2-sprawler-2026-11-18/README.md
  - Reports/pump-economy-2026-11-15/README.md
  - Reports/roguelike-run-2026-11-14/README.md
  - Reports/run-progression-2026-11-15/README.md
  - Reports/run-progression-2026-11-15/does-to-you.md
  - Reports/shop-b-2026-11-15/README.md
  - Reports/sim-use-2026-11-17/c4b-fire-shy-sprawler.md
  - Reports/sim-use-2026-11-17/s1-unit-paths.md
  - Reports/sim-use-2026-11-17/s2-far-smell.md
  - Reports/sim-use-2026-11-17/step0-scar-on-gpu.md
  - Reports/specks-2026-11-16/README.md
  - Reports/surge-2026-11-15/README.md
  - Reports/trunk-fort-2026-11-18/README.md
  - Reports/whole-game-review-2026-11-15/README.md
code:
  - BossRules::for_depth
  - Game::boss_aim
  - LADDER
  - LURE_PULL
  - LureMark
  - PLANETS
  - Playlog
  - RANGE_DEPTH
  - REF_OPEN
  - Run::buy_into
  - Run::carried
  - Run::gun
  - Run::level
  - Run::put_in
  - Run::range
  - Run::rules
  - Run::shop
  - STUCK_WARN
  - WIN_SHARE
  - ammo_price
  - area_of
  - bring
  - bring_shell
  - clock
  - does_to_you
  - enemy_keys
  - fort_mask
  - gun_price
  - head_start
  - range_args
  - reward
  - round_walk
  - run_args
  - run_hidden
  - run_shape
  - run_walkers
  - shells_at
  - shop_of
  - shrink_goal
  - size_roll
  - surge_food
  - surge_secs
  - tier_size
  - time_fort
  - time_lure_paths
  - two_stars
---

# The roguelike run

## Now

A separate mode beside the campaign (Run on the start menu, or
`run=SEED`): eight planets on the way to the slime's home world, each one
map with one slime to cut to `shrink_goal` of its peak (`WIN_SHARE`, 1% on
every planet since 11-15 18:02Z; 2% from 03:02Z, and before that 25% on
planet 1 down to 10%; each time the owner kept winning with slime left; under 3% the slime is broken and its
stragglers off food starve, `Rules::broken`), the tier climbing `LADDER` and walkers a fifth more each planet. The goal on the HUD counts it out: "counting from 1:00" and the walkers now before the peak starts counting, then the slime now against the count to reach, both as mass (a thickened walker twice, as the bar counts it; "12,630 walkers now → 126" at 1%), [report](../../Reports/menu-fixes-2026-11-15/README.md)). A
won planet pays points for a shop between planets that upgrades the squad,
unlocks and upgrades buildings and tools, and sells guns and gun cards
that get better deeper; a lost planet ends the run. The shop page is one
screen at every UI size (shop B, the owner's pick of 2026-11-15 19:37Z):
the planet's line and the points along the top, the carried gun's slots
and the bag pinned under it, a row of tabs (Cards, Guns, Squad, Buildings,
Tools) over a counter of half-size tiles that pages rather than scrolls,
and a pane at the right that tells the picked tile, else the planet's
briefing, with its Buy button; a click picks, Enter or the button buys
([report](../../Reports/shop-b-2026-11-15/README.md)). Until then it had
upgrades as a grid of command-card tiles with Buildings, Tools and Dev as
folders, and an armory of gun cards and bullet cards below, which scrolled
at large UI sizes (cards in the HUD's icons, in the carried gun's slots,
the bag, for sale). The
run starts with the average rifle and carries one gun (`Run::carried`,
the gun thread's `guns::Gun`); two guns rolled for the coming planet are on
sale each shop, priced by `gun_price`, and the carried gun's budget and
slots hold the cards. The gun fires its slots left to right; the player
drags a card onto a slot to move it, buys a card for sale straight into a
slot, drags one onto the bag to take it out, or presses Q and E; a card
put in without a slot lands before the first bullet (`Run::put_in`)
([report](../../Reports/gun-order-2026-11-14.md)). Bullets are rolled (`Ammo::roll`, two a shop, priced by `ammo_price`); the fixed payload cards are no longer sold. Walkers scale with depth and the planet's area, then its open cells at level start (`open_scale`). The clock grows with the planet's area (`clock`: 15:00 on planet 1 to 55:30 on the core world; two stars before four fifths of it, `two_stars`), and on planets bigger than today's map the slime grows alone first (`head_start`: 11 to 87 s on planets 4-8), the clock starting on landing. The slime is shape C under the tier's keys (`run_shape`), from C's 25,000 walkers times the level's factor (`run_walkers`) and a per-planet roll of 0.8 to 1.25 (`size_roll`; owner 11-15 08:34Z: "It should vary and be more later on") and the tier's (`tier_size`: the Sprawler 0.6 since 11-18), C's slots grown to match, and then the planet's own engine keys (`enemy_keys`, 11-15, the slime-enemy thread): the slime lunges 1 s to 1.8 s (`surge_dash`), so deeper slime presses harder, and on every planet it sends specks off to fresh crystals, most often early (`roam` 300, a chance every 10 s, taken at `roam_share` 0.05 on planets 1-2 down to 0.02 on the core world; the owner, 11-16 04:58Z: "Remove the bronze, keep the specks ... Specks more often early game"): ordinary walkers crowded or feeding at a crystal break away and land there, about 1,000 on their way at once on planet 2, and the slime is 7-10% heavier at 5:00 than with none (planets 1 and 2, one seed each); the GTX 1070's step costs 0.05-0.07 ms more on planets 1-2 and 0.30-0.33 ms on the core world ([specks](../../Reports/specks-2026-11-16/README.md)). Until 11-16 it also thickened from 2 down to 1.3 births' energy (`thicken`) and from planet 2 condensed (`condense` 300) and broke away bronze pieces (the whole-game review's D2, owner 11-15 21:41Z; [breakaway](../../Reports/breakaway-run-2026-11-15/README.md)); both are out of the run, shape C and the start menu. On planets 1-3, whose tiers read no alarm of their own, C's flinch is on (`BigParams::flinch` 0.9): the slime backs off ground a volley keeps hitting and closes in when it stops, at no frame cost on the GTX 1070; planets 4-8 keep their tier's alarm ([flinch](../../Reports/flinch-2026-11-15/README.md)). Its surges come when it has eaten enough from its crystals (`Rules::surge_food`, set from `surge_food`: a third more than the sandbox's 150 food on planets 1-2, as much on 3-4, down to half on the core world, times the level's walkers, since a bigger slime eats more), at least 10 s apart and at most 80 s on planets 1-2, 60 s on 3-4, 50 s on 5, 40 s on 6-7 and 30 s on the core world (`surge_every`, the longest wait); the shop's clock line names the longest wait (`surge_secs`). Before food-driven surges they came on a fixed beat of half those waits; neither costs a frame on the GTX 1070 ([surge](../../Reports/surge-2026-11-15/README.md)); until 11-15 a run played the bare race slime by mistake, which shed nine tenths of itself in a minute ([report](../../Reports/roguelike-run-2026-11-14/README.md), *The run's slime is shape C*). Locked buildings and tools show
locked on the command card, except pumps, the main income since 2026-11-15: they are never locked, never drain their crystal (owner, 2026-11-15 17:14Z), and the shop sells Pump power (`pump+`, a quarter more draw a level) in place of their unlock ([report](../../Reports/pump-economy-2026-11-15/README.md)). Dev mode adds rows to jump planets, add points
and redraw cards. Each planet is the level generator's `levelgen::level(seed, depth)` for the run's seed ([planets](planets.md)). Not yet played by the
owner; nothing is balanced ([report](../../Reports/roguelike-run-2026-11-14/README.md)).

**Mini-bosses** (2026-11-16; the owner's answers 04:58Z and 05:23Z in [decisions](../decisions-game.md)): a piece of the slime clenches into one body and hunts the nearest soldier (`larder_world/bosses.rs`, the engine's `big::Lump`). It forms only where the slime crowds or where a big part is cut off from food, more often and stronger deeper (`BossRules::for_depth`: none on planet 1 since 11-18, the owner's play notes; every 150 s on planet 2 to 45 s on planet 8, one at once on 2-3). Each draws a grade that sets its size, the hits each member takes (4 to 15) and its touch (2 to 5 walkers), and squads shoot it first in reach (`Game::boss_aim`, by the next bullet's reach; a soldier with slime on it shoots that unless the boss is close). A cut-off part waits out the landing and never re-forms where one just ended; an armour blow counts no kill. On planet 3 one held 418 of its 426 walkers across the map; the weakest broke in 4 s, the strongest killed all eight soldiers ([report](../../Reports/mini-boss-2026-11-16/README.md), round 2 and its review fixes). The break-away's specks are separate. Each draws a **kind** as it forms (round 3, `BossKind`, weights 1 each among the kinds with a target; a cut-off part draws only the runners): a **hunter** lunges every 5 s (planet 3: 16.5 world units a second lunging, 9.3 between); a **wanderer** roots 15 s on each far fresh crystal and takes in the slime there (116 members to 233 to 415 over two crystals); a **sapper** presses into the nearest wall until it is holed across, goes through, then hunts; a **leech** rings a working pump just outside its rig (`BossRules::ring`, the engine's `Lump::ring`) and takes its pay (282.7 food in 55 s with the bank still), giving it all back if broken, and the rig under it is not worn. Members don't starve, step three walkers past the slime's crowd limit so a mini-boss shoulders through its own thick body (`LUMP_CROWD`), and a broken or spent one fades 1.5 s before its members go back to the slime.

Ending early, the goal's mass, the play log and the planets' pressure: below.
The gun range plays with its shop as a free armory (below).

## Ending early, the play log, pressure by depth

Since 11-15 (the whole-game review's W17, W30 and W13;
[report](../../Reports/run-progression-2026-11-15/README.md)), a second
counts as stuck only when, besides no shot, no food taken and the bank under
a soldier's price, no soldier walked, no pump stands, nothing is being built
and shots cost food or every crystal is dry (in squad mode shots are free,
so a planet with charge left never ends stuck); from `STUCK_WARN` (10 s) the goal line warns
that the planet ends at `STUCK_END` (30 s); before, a run ended at 1:37 with
four healthy soldiers and both pumps standing. The shrink goal counts the
slime's mass (`World::mass`, the enemy's living walkers; until 11-17 a
thickened walker counted twice, so a body that thickened did not read as
shrinking; thickening was deleted). The window writes a play log
(`Playlog`), a file a round in `play-logs/` beside the game (since 11-17;
before, `larder-world-log-MAP-SEED.txt`, which the next round wrote over),
headed by the build, the switches on and the planet, a line a
game second (walkers, mass, soldiers, squads, bank, income, shots, kills,
buildings, frame p50/p95) with each command; `cargo run --release --example
playlog -- FILE` prints time to 10-40 soldiers, splits, pumps, kills a
minute against the slime's mass, and frame times. F9 in play drops a
numbered note (the second and where the camera looked).

Run seed 1, planets 1-6 ([report](../../Reports/run-progression-2026-11-15/README.md), *Measured*): idle is clearly losing by 5:00 on planets 1-3, and a four-soldier squad with the depth's average rifle kills 7-40 walkers a second while the slime's own swing is ten times that, so its kills don't show in the mass at 2:00; mass at landing grows 11x from planet 1 to 6 and the kill rate 2.5x. What rises by planet: see below.

## What each planet does to you

By minute 2 on run seed 1 (11-18, one film each,
[report](../../Reports/planet-things-2026-11-18/README.md)): planet 1's slime
eats two pumps by 1:15 but leaves a squad that builds none alone (the run
bot: 50 soldiers at health 93 at 2:00); planet 2's wears the bot's squad
down (health 35 at 1:00, 6 of 11 soldiers at 2:00); planet 3's walls its
crystals and sets where shot (12,140 walkers set by 2:00).

Planet 2 is where the run bot loses (11-18, run seeds 1 to 4, landing
straight on planet 2 with 9 soldiers,
[report](../../Reports/planet2-sprawler-2026-11-18/README.md)): mostly the
planet. On seeds 1, 3 and 4 the squad lands by crystals of 20,000 to
120,000 charge while every crystal over 130,000 sits in a slime of 23,000
to 25,000 walkers (planet 1's: 13,000), and the squad is down to 3 to 9
soldiers at 2:30. A bot that backs off farther, or the Sprawler's smell
turned down to the plain slime's, gains a little on 2 of 3 seeds; the
slime at 0.6 of its size builds 18 to 24 soldiers on all three, at 0.8
only seed 1 holds. Seed 2 lands by a free crystal of 365,000 and builds 30
soldiers at today's size. The size is Direction's to set.

Since 11-15 (the owner's D3, 21:41Z: "Agreed. We can test it out";
[report](../../Reports/run-progression-2026-11-15/does-to-you.md)), each
rung of `LADDER` does one thing to you (`does_to_you`), named on the planet
card and in the landing alert: the Drifter smells your pumps
(`pump_smell` 20 over `lure_r` 15 cells) and comes to eat their crystals; the
Trunk walls the crystals it holds in amber and sets where you shoot it
(below; until 11-17 it had the Drifter's pump smell, so planets 1 and 3 did
the same thing); the
Sprawler smells your soldiers from far off (`smell` 4 over 4 cells) but
shies from fire (`fire_alarm` 50, since 11-17: the designer's fire-shy
tier, as a switch for every slime it fenced every fire;
[report](../../Reports/sim-use-2026-11-17/c4b-fire-shy-sprawler.md); on
run seed 1's planet 2 a plain light found nothing to burn); the
Swarmer's surges reach 25 cells past your range (`panic_margin`) and lunge
3x (`surge_haste`), and since 11-17 it makes a last stand (`last_stand`: cut
under a quarter of its peak, all of it rushes the thickest knot of soldiers,
once; the owner's 11-15 21:15Z idea, "a slime that is losing might rush you
will everything it has", given to the one tier whose card is already the
charge); the Sporer bursts into spores at an alarm of 0.3; the
Sleeper wakes at an alarm of 0.3; the Grudge (planet 7 since 11-17, when
its scar reached the GPU; [report](../../Reports/sim-use-2026-11-17/step0-scar-on-gpu.md))
reads the scar where it was hit at twice the campaign tier's weight
(`scar_weight` 1, a first number; one film, seed 1 with four soldiers standing: gone at 0:40 against 0:48 with it off, a small effect). Run seed 1 at 2:00, each ends or wears the
four-soldier test squad (or the pumps) sooner than before: three planets that
squad survived now end it. A test: the owner's play sets the size. The run
shop no longer sells the outpost or siren (D10; `run_hidden` hides
them on the command card; the lure was out with them until 11-18), and the 40-soldier rush is left as it is (D7). A
lab prototype met D10's condition for the lure: a lure
that every walker leans toward (a walking-distance field, so walls bend it)
drew planet 1's home colony into three turrets, 4,158 walkers killed by
1:30 against 1,623 without it (run seed 1, one seed, software GPU;
[report](../../Reports/lab-steering-2026-11-17/README.md)). The owner's card,
11-18 03:03Z: "Lure now, in the three planets". The Lure comes back to the
run shop for planets 1 to 3 with that rule at a third of the lab's pull
(first numbers: pull 10, shop unlock 150, the building 300 food and 150
health, mortal); Enemy builds the field with its own GPU slot and weight,
Squad and arsenal the building and the shop item, Look and interface the
rings' colour. Both are built: the field (below), and since 11-18 the
Lure and the lures' switch are back on a run's build card, free from the
first planet with nothing to unlock (`run_lock`; Direction, 08:38Z, after
an hour as a 150 unlock that left planet 1 without it). The siren and the
outpost stay out.
The lab's later parts (same report): marks laid by every shot do not herd,
so there is no bullet steering card; a dart that sets a lure where the squad
aims takes a crystal off a colony in 30 to 45 s at pull 10, for fewer kills
(about 3,500 against 8,612 by 2:00, planet 1, run seed 1). The dart is not
placed: it waits until the owner has played the Lure building.
Part four, to 5:00: a colony pulled off its crystal mostly dies in the
turrets, and the crystal is resettled within one to two minutes once the
darts stop; with no turrets a raid on the squad overrides the lure, and the
port keeps that (below).

**A lure pulls** (every planet, 11-18). The lab's field is a slime rule of
the GPU engine (`lure_pull`, `run::LURE_PULL` 10 on every planet, no CPU
twin): calm slime within `Rules::lure_reach` (60 cells of walking) of a
Lure that is on, or of a lure mark (`LureMark`, for a thrown or shot lure),
leans down the walking distance to it, which is round on open ground
(`round_walk`, since 11-18: the raid's search drew octagons); the game
rebuilds the field once a game second (`time_lure_paths`). A panicking, surging, raiding or breakaway
walker and a mini-boss's member ignore it, so a lure draws the body at rest
and is no shield from a charge; a blind walker does not see it and a soured
one runs from it. In the lab's scene on the game's build, 4,461 walkers
killed by 1:30 with a Lure building (which the first raid wore down at
0:59), 5,978 with a mark that cannot be eaten, 1,813 with no lure (run seed
1, one film each, software GPU;
[report](../../Reports/lure-field-2026-11-18/README.md)).

**The Trunk's walls** (planet 3, 11-17; `TRUNK_KEYS` and `TRUNK_RULES` in
`run.rs`; [report](../../Reports/enemy-switches-2026-11-17/README.md)).
Where the Trunk is shot it sets: the hurt walkers stop where they stand,
take three blows instead of one (`setting`, `set_armour` 3) and are drawn as
amber resin. Five of them round a cell make it no ground to soldiers
(`set_block` 5), and the walking maps route round it. Its fed body does not
burn (`fire_fed`); its set walls and its starved parts do, with the crust
under them as fuel (`crust_fuel` 3). It keeps to ground it has held
(`crust_weight` +8, `crust_fast` 1). These were the simulation-use plan's
four open picks; the plan the owner approved on 11-17 had the Enemy lane
pick one configuration for one tier, and planet 3 was chosen because it is
inside Milestone 1 and its old card repeated planet 1's. One film each, run
seed 1: eight rifles holding a front had set 1,143 walkers into 1,130 wall
cells by 1:15, and two Torch shots on that wall cut it to 679 cells in two
seconds while it grew to 1,284 with no fire; the scripted rush killed 3,205
by 1:00 against 7,992 before and still fell to the first raid.

**The Trunk's fort** (planet 3, 11-18; `fort` 3 in `TRUNK_RULES`;
[report](../../Reports/trunk-fort-2026-11-18/README.md)). The owner had
pictured the walls guarding the slime's core or crystals (Owner's word), so
the Trunk walls each crystal it holds: a quarter covered by walkers holds
it, under a twentieth or dry loses it (`time_fort`), and each held crystal
gets an amber ring with four gates (`fort_mask`; the home ring's first gate
faces the map's middle, the others face home). Slime on a ring sets with no
scar once the ground has crusted, and the bulwark makes it wall; the
slime's own ways go through the gates. It still sets where it is shot. GPU
engine only. One film each, run seed 1, the squad standing still: the home
ring stands from 0:15 (1,422 walkers set), and by 3:00 3,916 walkers are
set into 3,681 wall cells against 1,321 and 1,339 with no fort, with the
slime 9% smaller. A fire on the wall burns the stretch between two gates
(the fed slime in a gate does not burn) and runs on over the crust outside.

**Every run planet's slime** (11-17; `enemy_keys` and `ENEMY_RULES`) smells
crystals it has not reached from across the map (`far_smell`;
[report](../../Reports/sim-use-2026-11-17/s2-far-smell.md)) and its charges
go round walls instead of piling against them (`unit_paths`;
[report](../../Reports/sim-use-2026-11-17/s1-unit-paths.md)). Both were
switches until then. With both on, the scripted rush on run seed 1 still had
42 soldiers at 2:00 on planet 1 and had 7 on planet 2 against 18, one film
each ([report](../../Reports/enemy-switches-2026-11-17/README.md), *Films*).

A planet whose tier asks for a card (`bring`; the Sprawler, which shies
from fire, asks for Tinder) gets that card in one of its shop's card slots
when you don't own it, on every reroll (the game designer, 11-17 15:26Z);
drawn by chance it was in 23 of 200 planet-2 shops. A tier may ask for a
bullet too (`bring_shell`, 2026-11-18): the Sprawler and the Trunk ask for a
Torch, which is the shop's first bullet while you own none, because
Tinder's brush kills nothing until a fire runs along it and the Trunk's
amber walls burn. Since the same day a shop's two bullets are never one
shell, never the plain standard bullet the run starts with and never a
reader with nothing to read (a Shatter with no gas bullet owned, a Detonate
with no germ), and its two guns are two kinds, neither the kind carried
(`Run::restock`). Tinder costs 100 and a Torch at most 100 (`TINDER_PRICE`,
`TORCH_MOST`; Direction, 11-18), so the pair fits what the least win of
planet 1 pays.

## The gun range

Gun range on the start menu, or `range=SEED`, uses a run
as a free armory (`Run::range`): every gun kind owned at its average with a
rolled one of each on the rack to swap in, a bullet of every payload and set
(the standard set exact) and every other card, never sold out, and tiles to
roll them again, deeper or shallower (from `RANGE_DEPTH`), and to empty the
gun or the bag. The armory is the shop's frame with the range's tabs: Guns
as a table whose stats are green or red against the carried gun, Bullets as
a grid of shape by payload, and the modifiers' two shelves; the roll and
empty tools sit on the bottom bar ([report](../../Reports/shop-b-2026-11-15/README.md)). Back to the range plays the sandbox's round armed from it
(`range_args`), with no time limit and a 100,000 bank; H there opens the
armory over the paused round and Back re-arms every squad
([report](../../Reports/gun-range-2026-11-15.md)).

## Owner's word

- 2026-11-14 15:21Z: "For now separate, but it might become the main game. I don't want to erase the campaign yet"; one level one map with a shop between: "Yes. Unless you have other suggestions. The maps should get bigger too as we progress."
- 2026-11-14 15:25Z: "the slime is taking over planets and each level is a new planet as we move towards the core slime world. This is just head cannon right now".
- 2026-11-14 15:26Z: level generation is its own thread; this one reviews it "but you don't do level generation".
- 2026-11-14 16:55Z: "Guns and bullets should have icons. This whole menu should not just be uniform buttons. There should be one area for unlocking/upgrading that should match the player menu from the screen (buildings and tools should be separate menus that you open up). The second area of the screen is for Guns/bullet cards."
- 2026-11-14 20:32Z: "How do I actually build and adjust my guns. When I buy a card it goes into the gun but I don't see how to move cards around".
- 2026-11-15 02:58Z: "everything shifts in the shop. Once you add a bag that shouldn't happen" (fixed: the page reserves two card groups; in shop B the gun and bag are pinned and hold still, a test checks it).
- 2026-11-15 17:31Z: "Having to scroll is horrible. That's not professional." 17:41Z: "The bullet tiles could easily be half the size without even losing any info." 19:37Z: "Shop B, menu A," (the tabbed counter with a pane).
- 2026-11-15 08:34Z (to the slime thread, on a planet's slime size): "It should vary and be more later on."
- 2026-11-15 21:41Z (the review's decisions): "1. Agreed. We can test it out. 2. No. 3. Let me see the video. 4. Play first. No changes yet. 5-7) agreed" (D3 does-to-you as a test, D5 no payload card, D7 no rush brake yet, D10 no outpost, lure or siren in the run shop).
- 2026-11-15 18:02Z: "We are still winning the game with too many slime left. Drop it to 1% of peak" (`WIN_SHARE` 1%).
- 2026-11-15 03:02Z: "I know I originally said getting the last few slime stragglers is annoying but right now how it's set up I feel like I keep winning before I've actually killed everything. So maybe we need a different threshold or something" (`WIN_SHARE` 2%).
- 2026-11-14 15:33Z: "There should be a dev mode to control what level planet i go to"; rare planets with base remains or soldiers are "more of a rescue the human planet".
- 2026-11-15 02:55Z: "I need a test environment built where I can play with the guns and cards and bullets."
- 2026-11-15 08:30Z (play notes for the whole-game review): "I mostly run to the biggest crystal, harvest and create new soliders until I have 40 soliders and then I attack the slime. That is not what I want to play though"; "Some enemy slimes just hang at their home crystal ... They don't attach my pumping crystals so no need to defend"; "There needs to be more interesting progression in the enemy"; the real game is squad mode in the run; "The gun graphics need work. They should really be one of the draws of the game."
- 2026-11-18 03:11Z (in the Enemy lane's thread, on the walls the Trunk sets where it is shot): "This is interesting and I want to play test it, but when it was original suggested, I thought the walls would form protecting the core of the slime or the slime crystals giving them a base of sorts" (built as the Trunk's fort).

## Open

- The origin as the win rule (Milestone 2; the owner, 11-16 04:58Z): designed 11-18 as a sitting lump of the slime's walkers on its home crystal whose death wins the planet, with three questions for the owner; not built ([design](../../Reports/origin-design-2026-11-18/README.md)).
- The whole-game review (11-15, [report](../../Reports/whole-game-review-2026-11-15/README.md)): the slime sitting at home was the bare race slime, fixed by shape C (#336); re-filmed on main, planet 3's slime wears two of four pumps to 0 by 3:00. Still open: the drain-and-buy rush reaches 40 soldiers by 2:00 on planet 1 against the live slime; what the enemy does to you planet by planet is now a test (above), awaiting play.
- Mini-boss slimes: hunter, wanderer, sapper and leech are built and on in the run (Now); Mortar and Pinwheel, each kind's look (its own thread) and the origin at each planet's start crystal (owner 04:58Z) are to come ([report](../../Reports/mini-boss-2026-11-16/README.md), round 3).
- A mortar or artillery building; rescue planets; saving a run.
- Prices, rewards and the goal by depth wait for the owner's playtest.
- A breather planet around planet 6 (the planet review, 11-15): the owner judges the pace in play first (02:17Z, via the levelgen thread).
- The core world with C: 364,924 walkers, frame p95 12-14 ms on the 1070 (planet 1 4.6 ms) and a 12 s head-start pause before landing; kept until play says otherwise.

## Sources

- [report](../../Reports/roguelike-run-2026-11-14/README.md), *What was built*.
- [arranging the gun](../../Reports/gun-order-2026-11-14.md), *What was built*.
- [the gun range](../../Reports/gun-range-2026-11-15.md), *What was built*.
- [the whole-game review](../../Reports/whole-game-review-2026-11-15/README.md), *Re-check on main after the review*.
