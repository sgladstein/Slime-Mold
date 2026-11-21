# Enemy lane

State first, kept current after every merged batch; the brief the lane was
given on 2026-11-17 is below it, unchanged.

## Mini-boss review (2026-11-20, the thread "Analyse and improve the minibosses")

Not this lane's thread, but the lane's file (`bosses.rs`), so recorded
here. The owner (11-20 05:41Z): "analyze the minibosses, including looking
at my playtests and improve them". His 13 logged rounds had no boss lines,
so the review ran the strategy bot on his own maps (run seeds 1795038793,
1795106014, 1795119328, planets 2-4) to 300 s, per-kind table in the
[report](../Reports/miniboss-review-2026-11-20/README.md). Before, on 17
bosses: 2 reached the squad (both hunters), no leech or sapper got to its
errand, 3 were swallowed by a raid's march, 11 of 20 (to 420 s) stood 10 s
or more pressed on rock. Built, on by default, no engine change:

- A march grows its lump (`World::grow_lump`) instead of forming it, so it
  never takes a mini-boss's members (`start_march`, `raids.rs`).
- Bosses walk round rock: `Game::boss_steer` goes straight while the line
  is clear, else builds a `Way` off-frame (`Way::start_units` to every
  soldier for a hunter, rebuilt each second; `Way::start` to the goal
  otherwise) on the slime's ground with deep water closed, and aims at the
  farthest point down the way it can see (string-pulled). A body stuck 6 s
  with no soldier near regathers on its leading part, holding no more
  walkers than its own area (`reach_holding`), `Bosses::regrouped`;
  on a straight walk too (`Game::boss_stall`): a tail wedged in packed
  slime held a planet 3 leech 54 s on open ground with a clear line.
- They form nearer the player: a crowded knot weighs 1/(1+(d/`near`)^2),
  `BossRules::near` 40 dish cells, and none forms inside a soldier's guns.
- A leech's or sapper's walk runs on its own clock (`BossRules::trip`
  90 s), `life` counts only the time after.
- A wanderer hops toward the player (the nearest fresh crystal nearer the
  player's things than it is).
- The play log writes `raid` and `boss` lines (formed, every 5 s, ended).

After, merged with main, same nine games: 7 of 18 reached the squad,
39 s stood still against 258, none swallowed; no errand reached in 5:00
(pumps 150-250 units off; the trip or a nearer pick is tuning, unreviewed).

Then (#548, Direction's call): the kind is drawn first and a crowd's leech
or sapper forms at the crowded knot nearest its pump or wall; one sapper
reached its wall, 2 of 3 leeches met the squad guarding the pumps.

Then the Mortar (the owner's pick, 11-20 13:21Z, "Mortar first", over the
Pinwheel): `BossKind::Mortar` (5), drawn from planet 4 on
(`BossRules::for_depth`, `kinds[4]`). It forms at the crowded knot nearest
a player thing it can see along a clear line, never walks, and every
`lob_every` (5 s) throws the ring of its own body's edge at the nearest
player thing just past the guns (`lob` 30 dish cells past the longest
gun's reach and the panic margin) as one `squad::Burst` with `push`
(`Game::mortar`); thrown members are let go and the rest formed again on
its spot; with no shot along a clear line it walks toward the nearest
player thing on its trip's clock (on the owner's three planet 4 maps, all
bosses mortars: 7 mortars, 48 throws by 5:00; what they cost the player
is not measured yet). Moss skin, acid core (`LumpLook::Mortar`, Look's
`view.wgsl` case 6). Also: a fruit grows its lump instead of forming it,
so it no longer swallows a mini-boss beside it (3 of 14 had been). Set
keys `boss_mortar`, `boss_lob`, `boss_lob_every`, `boss_shell`.

Then (11-20 evening, the coordinator's ask from the balance brief): a
throw lands as a shell (`Bosses::shells`, a lump with the mortar's bite,
the armour of half its grade and its look, no leash, for `shell_life` 8 s), cut from
the body's outer part where it holds 60% (`SHELL_CORE`), and only
while another lump stays free after it (4 lumps in all); and late in a
planet (slime under `late` 0.6 of its peak, `Game::boss_late`) a new boss
is drawn from `late_grade` 0.8 up. Numbers in the report's last section.
Then the Pinwheel (the owner's second pick): `BossKind::Pinwheel` (6),
planet 3 on (`kinds[5]`), a hunter whose lump's arms turn within
`spin_near` (45 dish cells, 90 world units) of its prey: `Lump::spin` (the 32-byte row's old pad),
engine.wgsl's spokes after the lump pull (`PIN_ARMS` 3, `PIN_LEAN` 3,
`PIN_TWIST` 0.08); the turn must stay constant (phase is spin x S.t).
Look's `LumpLook::Pinwheel` (7, cobalt). Set keys `boss_pinwheel`,
`boss_spin`, `boss_spin_haste`, `boss_spin_near`. Then the big-army census: no grade or size outlasts 50 guns (a member
takes at most 15 blows); options A to E in the report, sent to
Direction. Direction picked A and B: `late_mortar` (Mortar weight x army
scale when late, from planet 2) and `Game::round_errand` (from the
full army scale, a late hunter or pinwheel goes to an out-of-reach
building or the squad's far side; `round_far` 160). Play log
`bossfell` lines. Next: what the Pinwheel's arms cost the squad; the
off-screen tell from the owner's doc.

Follow-up (Direction's call): the kind is drawn before the knot, and a
crowd's leech or sapper forms at the crowded knot nearest its errand
(`crowded_knot(Some(&boss))`). Drawing them only with a target within
100 or 180 units drew none at all on these maps (dropped).

Known limit: a lump has one aim, so on a rock bar a third of a body stays
pressed on the near side (unit test `a_mini_boss_walks_round_rock`).
Proposals in the report and the owner's doc: the Mortar first from planet
4 after Milestone 1, the Pinwheel second; a bounty; an off-screen tell.

## Condensed strikers (2026-11-20, the thread "Make the slime a more interesting enemy")

The owner's pick on the balance thread's card (11-20 15:10Z, "Condensed
strikers + late lunge"; the brief is the options doc linked under
Heartbeat). A is this lane's: `larder_world/strikers.rs`
(`StrikerRules`, `Game::time_strikers`), on in every run
(`ENEMY_RULES`' `strikers`), GPU only (`World::set_strikers`,
`World::condense`, `gpu::Engine::strikers` and `Engine::condense`).
Under 40% of its peak, once a game second, `condense` (a thread a slot)
lets each ordinary walker within the guns' reach + 12 dish cells of a
soldier (by the units' walking distance, bit 31 of `cond_r2`; as the crow
flies without one) try with chance 0.5; tries count on a word a
`RAID_BLOCK` block in the raid region's third quarter (count << 24 | the
first's slot); the next three die, taken in. `condlead` (a thread a
block) marks the first condensed (`DENSE_BIT`, bit 16 of its dense word)
holding them in `HURT_BITS`. Params' lump row: `lump_on` is also 1 with
strikers (bite cells counted), then `cond` (haste), `cond_r2`,
`cond_share`. `armoured` spends one held a blow, `dcount` adds the held
to its cell's bite, `panic_to` sends it at the nearest soldier,
`lumpform` clears the bit on joining, `ripen` never sends one off as a
speck. Counts in `wstat[8..10]`. The look: view.wgsl's `draw_striker`, a
lit bead in the droplets' layer, radius by what it holds, a 48-step pulse;
it counts as its walkers in the body, no bronze. Test:
`condensed_strikers_take_in_their_neighbours_charge_and_take_more_blows`.
The balance thread (cmsg_01LaKUNJU8dhcZFTMf8VboUgN41f84yygVjCkvqcyxGYdX)
measures it against `set=strikers=0` with its late lunge (B). Its first
score (planet 2, 36 games) found the first numbers did not bite an
80-soldier army, so A was retuned that evening: `below` 0.6, `band` 30,
`share` 1, `haste` 3, and the share scaled by the squad from `few` 8 to
`full` 24 soldiers; `most` (blows a striker takes, 1-15, packed in the
condense row's reach word) is a rule now, default still 3. The probes and
why are in `StrikerRules::default`'s comment. The balance thread scores
the retune: small squads win again, 80-soldier armies lose 28 to 37 late
and still win; map 2's mid army now loses (haste 2 is the fallback).
Merged in #566, with a fix: a lump forming over strikers no longer
deletes what they hold (lumpform skips DENSE_BIT).

**Capped, 11-20 night (#598):** the full share condensed 4,218 walkers into
1,607 strikers in one second on planet 1 (run seed 1, 43 soldiers dead in
8 s). Now `condense` counts the band's walkers (`wstat[11]`) and the game
cuts the next pass's share to about `rate` strikers a second (150, or
`rate_share` 0.02 of the slime); run.rs `STRIKER_SHARE` scales share and
rate by planet (0.25, 0.6, 1). Real yield is about a quarter of the cap
(sparse tries rarely pair in a block). The cap also ended the regrowth
the bug hunt saw (mass +1,588 the second after an uncapped formation);
keeping the taken-in slots empty did not (`Reports/dead-ends.md`).
Scott's ruling the same night: planet 1 gets no strikers; the pacing
thread gates `ENEMY_RULES` by planet (strikers from 2, waves and lunge
from 3, heart from 4). The heart's strength (68 soldiers in 8 s on planet
2) is the balance thread's to tune.

## Last-stand heart (2026-11-20 evening, the same thread)

D of the brief: `larder_world/stand.rs` (`StandRules`, `Stand`,
`Heart`), on in every run (`ENEMY_RULES`' `stand`), GPU only. Under 20%
of peak against 24+ soldiers, every free walker grows into one lump on
`Game::heart_site` (armour 4, bite 1, regrown every 3 s), lunging at the
nearest soldier every 20 s (haste 3, 10 s); it breaks under 15% of its
most or at 150 s (keeping it to the end stalled planets). Look
`LumpLook::Heart` (view.wgsl case 7). Report and clips:
`Reports/stand-2026-11-20/`. Next: the balance thread can score it on its
planet 2 jobs (bites in the last quarter); then F (buried knots) if
Scott wants more.

**11-21 (#608):** a fruit's grow is held to `fruit_reach(centre, cap)`
(one took 975 to 2,812 past a cap of 1,950), and sown spores never home
(the scent holds the soldiers' smell: a burst landed 1,455 walkers on a
squad and bit 21 of 30 in 5 s); they land where the throw ends. Every
flying spore writes its flight word, so the Sporer's draw as tailed beads.
New slime events also need a `force_*` in `larder_world/force.rs` and a
Spawn entry in `viewer.rs` (the slime viewer, #607).

## Crystals never run out; the slime's size follows them (2026-11-21, the same thread)

Scott 06:30Z: "crystals never run out from eating or pumping, and only
your squad can empty one by draining it ... this 100%"; 06:34Z he picked
option 1, size follows food. Built together, GPU only:
`crystal_keep` (`run::enemy_keys`; `eat` takes no charge, `pilesum` sums
the eating into pile word 7 for the surge meter, `World::slime_ate`) and
the ceiling (`Rules::ceiling`, `run::CEILING` 12 in `ENEMY_RULES`,
`larder_world/ceiling.rs`): covered crystal cells (pile word 8; `Pile`
is now 12 words, `gpu::PILE_WORDS`) times the landing density, roof 3x
landing or the slots; over it species 0's birth cap stops births
(`WORLD_CEIL`, `capcount`) and upkeep rises to x3 at 20% over. Numbers
and rejected variants: `wiki/model/growth.md`, *The run's ceiling*.
Film tools: headless `LARDER_DRAIN=T`, `film_ends`' `FILM_DRAIN`,
`FILM_KEYS`. Next if Scott wants a sharper shrink: `CEILING_OVER` 0.1 or
`ceiling_upkeep` 4 (measure the starts again).

## A hit's hole and a mini-boss's flinch (2026-11-21, the same thread)

Both GPU only, in the `deep_wall` Params word (bit 0 deep wall, bit 1
solid walls, bits 3-15 the flinch's steps, bits 16-31 the shut steps).
`crater_shut` 0.5 (#670, the Look lane's A2): a crater cell is closed to
walkers for the first half of its countdown. `boss_flinch` 0.5 (Direction's
job from the Look lane's exit test 2): a lump's member on a fresh crater
runs down its slope (`lump_flinch`), so a hit bites a hole into a
mini-boss. The wanderer's flat look is the view's (a fringe and a looser
rooted hold changed nothing; `Reports/boss-flinch-2026-11-21/`). The view
side of both is the Look lane's.

## Heartbeat (2026-11-20, the same thread)

The third rule, on by default on every run planet (`ENEMY_RULES`' `heart`;
`larder_world/heart.rs`). Every 6 s a beat runs out from the home crystal
(else the crystal held most) through the body; calm walkers lean out
(25) and quicken (up to 60%, `HEART_HASTE`) as it passes. Params: the
heart's place in the ground row's last two words, its lean in the lone
step row's second, its clock (period steps | wavelength << 16) in the
fling row's second (`Engine::heart`). The balance thread's options brief
(late-planet threat) is at https://claude.ai/artifact/YD9LSGAkAorDNKg5WA2zZH:
its A (condensed strikers) and D (last-stand heart) are this lane's to
build once the owner picks.

## Waves (2026-11-20, the thread "Make the slime a more interesting enemy")

Its second rule after fruiting, on by default on every run planet
(`run::ENEMY_RULES`' `wave`; `larder_world/waves.rs`). Losing fast (the
squad's kills over 20 s at least an eighth of the slime's mass, the mass
down over them), from 1:30: a field in the raid region after the raid's
blocks (`Engine::write_wave_field`, engine.wgsl `wave_dist`, the pull in
Params word 880, the lone step row's first) leans calm walkers to two or
three points 20 dish cells past the guns round the thickest knot of
soldiers for 12 s, then to the soldiers for 8 s with a surge whose reach
is stretched to the masses (`keep_panic_reach`). `write_raid_field` now
writes only its own blocks so it never clears the wave's. The owner
dropped the rind and the civil war (11-20 14:16Z: hardened slime does not
attack, so it makes the game easier); the late-planet problem (a big
squad the slime cannot touch) is shared with the balance thread
"Keep big squads under threat late in a planet".

## Fruiting (2026-11-20, the thread "Make the slime a more interesting enemy")

Not this lane's thread, but the slime's rules, so recorded here. The owner
(11-20 05:46Z): the slime is interesting early "but once I get strong
enough there is just lots of shooting through masses of uniform orange.
Think outside the box". Built on by default on every run planet
(`run::ENEMY_RULES`' `fruit`; `larder_world/fruit.rs`,
[report](../Reports/fruiting-2026-11-20/README.md)): from 1:30 after the
landing, once the squad has killed a fifth of the slime's mass in 30 s
(a fall-under-peak test was tried and deleted: it fired on planet 3's Trunk
with the squad nowhere near), the slime takes a lump (`big::LUMPS`, shared:
`Game::lump_taken` now covers bosses, marches and fruits) for 8% of
itself at its thickest square out of the guns' reach plus 15 cells, holds
it with pull 40 and leash 1 (armour 2), swells it to twice its first size
(at most 2,500) and after 30 s bursts it: engine.wgsl's `fling` with `kir`
2 (`Engine::sow_pass`, `World::sow`) throws every walker in the knot out
as a homing spore (`SOWN`, bit 15 of `spec`) that lands on food
(`spore_reach` 36 and `spore_land` 1 are now in `run::enemy_keys` for every
planet). Under 35% of its most members it breaks. One at a time, 40 s from
one's end to the next. The view draws it as `LumpLook::Fruit` (umber,
darkening as it ripens), from the window's `main.rs`. GPU only
(`World::can_sow`). Next ideas from the same thread, not built: a slime
that reads its losses and comes in waves from several sides; a heartbeat;
a rind that grows on the side being shot.

## State (2026-11-18, the march)

**Open branch:** none (`claude/project-thread-s5k9cw` restarts from main
for the next change). Merged on 11-18: #451 (the switch settlement, job
1), #457 (the lure's field), #473 (the Trunk's fort), #478 (the lure's
field round on open ground), #480 (job 2's page and job 3's design), #486
(the planet 2 look), #490 (raids rare early, no mini-bosses on planet 1),
#496 (the march and the tether; it changed Engine and tools'
`gpu/mod.rs`, `headless.rs` and `rules.rs`, and they were told).

**The owner's play notes of 11-18** (15:56Z to 15:58Z, relayed by
Direction; in `wiki/decisions-game.md`) gave this lane three jobs, in
order: raids rare early and more often later, no mini-bosses on planet 1
(both #490, [raid ramp](../Reports/raid-ramp-2026-11-18/README.md)), and
the slime holding together as one body, worst in raids (the march and the
tether, below). Direction also asked whether planet 3 has an unfinishable
tail now that a planet is won at its last walker (#487): no; answered.

**Raids march as one body; specks are tethered**
([report](../Reports/raid-march-2026-11-18/README.md)). Direction's narrowing
(11-18 16:24Z and 16:53Z, after the owner liked the Lab's tighter-tether
page): keep the slime together when it moves away from its body (a raid,
a breakaway, a walk to a new crystal), leave the surge at the squad as it
is, and the march is this lane's (the Lab builds no muster).

- The march (`RaidRules::march`, `Raids::start_march`, `Game::time_march`
  in `raids.rs`): a raid takes a free lump (`big::LUMPS`, shared with the
  mini-bosses) for 1.5 times its walkers nearest the target along its way,
  steers it down the raid's walking distance (`down_way`, a point a body's
  length and ten dish cells ahead) at pull 100, leash 1.3, haste 2, and
  pours it on as a raid of share 1 by the target or at a soldier it meets
  (gun range plus the panic margin past its edge). Not ten dish cells
  nearer after `march_stall` 15 s, or still walking at 90 s, it streams
  with the raid's own reach and share; under a 30% quorum it only lets
  go. No lump free, or the last stand: it streams as before.
- `form_lump` takes other lumps' members, so `form_boss` refuses near a
  march and never takes a marching lump's slot.
- A bigger march formed inside planet 1's rock ring and, at the
  mini-boss's pull of 30, sank into the body; pull 100 gets it out in about
  12 s. Not checked: other planets, a march that meets a wall it cannot
  find a way round, several marches at once.
- The tether (`Settings::tether`, engine.wgsl's `tethered` in `sense`;
  `run::TETHER` 2 on every run planet): scared walkers that are not
  raiding, not charging a unit and not a lump's members step only where
  their next cell touches 2 others. Raiders are left out (the Lab: a
  tether of 2 or more keeps a raid home). Params word 892.
- Frame cost: the tether reads 8 cells for such a walker stepping into a
  new cell; the march is CPU work once a game second (lump sums, as for a
  mini-boss, and a walk of at most about 30 blocks). Not timed: Engine and
  tools booked both in Milestone 1's GTX 1070 batch (11-18 17:54Z) and
  will ask this lane for the Remote Control thread when it runs it.
- Next, if Direction or the owner's play asks: how often a march stalls
  and streams on other planets and seeds; a march formed inside the home
  ring first sinks a few cells into the body before it finds the gap.
- `headless.rs` prints `march t ...` each second and a `loose:` line (the
  Lab's measure: walkers with at most one other round them).

**The order of work now** (11-18): the origin is designed and waits on
Direction ([design](../Reports/origin-design-2026-11-18/README.md): a
sitting lump on the home crystal, regrowing from its charge, calling the
body home when struck, its death the win; three questions for the owner,
which Direction holds until after his next play). It is Milestone 2's, so
it is not built until Direction says go. Direction counts this lane's
Milestone 1 jobs done but planet 1's minute-2 gap for a rusher (D7, the
owner's call).

**Planet 2, the bot or the planet: mostly the planet** (Direction's
question, 11-18; [report](../Reports/planet2-sprawler-2026-11-18/README.md)).
On run seeds 1, 3 and 4 the squad lands by crystals of 20,000 to 120,000
charge while every crystal over 130,000 sits in a slime of 23,000 to
25,000 walkers; the bot has 3 to 9 soldiers at 2:30. A bot that backs off
farther, or the Sprawler's smell off, gains a little on 2 of 3 seeds; the
slime at 0.6 of its size gives 18 to 24 soldiers on all three. Seed 2
lands by a free 365,000 crystal and builds 30. Direction set the
Sprawler's slime to 0.6 of its size (#485, 11-18): `run::tier_size`, a
factor on `Level::walkers` per tier (other tiers 1), kept apart from the
depth rule in the density test. Direction retunes it from the run bot's
daily run on seeds 1 to 3 and the owner's play. The two local patches the
films used are in the report.

**Job 2, what planets 1 to 3 do by minute 2: page delivered**
([report](../Reports/planet-things-2026-11-18/README.md), one clip a
planet). Planet 2 (hunts the squad) and planet 3 (walls its crystals,
hardens where shot) do their thing to the run bot by 2:00. Planet 1 eats
pumps by 1:15 but leaves a squad that builds none alone (the bot: 50
soldiers at health 93 at 2:00). That gap is D7's; Direction holds the
candidate (below).

**The lure's field is round** (#478; Look and interface's note, 11-18):
the lure has its own walking search, `raids::round_walk` (chamfer 5, 7,
11 with the knight's moves, within 3% of the line, stopping at the
reach). The charges, far smell and raids keep `big::units_field_for`'s
octagon (`big.rs` is frozen). Look's `set_steer_reach` reads the same
units.

**The Trunk's fort: built** (the owner in this lane's thread, 11-18 03:11Z,
on the walls that form where the Trunk is shot: "This is interesting and I
want to play test it, but when it was original suggested, I thought the
walls would form protecting the core of the slime or the slime crystals
giving them a base of sorts";
[report](../Reports/trunk-fort-2026-11-18/README.md)). On planet 3 the slime
walls each crystal it holds: an amber ring with four gates, round its home
crystal by 0:30 and round each crystal it takes after. Hardening where it
is shot stays on, so he can play both.

- The rule is `Rules::fort` (the ring's thickness in dish cells; 0 off, 3 in
  `run::TRUNK_RULES`). `Game::time_fort` (`bulwark.rs`) looks every 2 game
  seconds at the walkers on each crystal: held at a quarter covered, lost
  under a twentieth or dry. On a change it writes the rings
  (`bulwark::fort_mask`) through `World::set_fort` into the fourth quarter
  of the cells' region in `news` (`fort_at` in engine.wgsl), and Params
  word 888 is the flag that a mask is written (`Engine::forted`). A walker
  on a masked block with crust under it sets with no scar and stays set;
  the bulwark rule makes it a wall. GPU only.
- All four quarters of the cells' region are now taken (charge paths, far
  smell, lure, fort). Params words 880, 884 and 892 are free.
- The slime's own ways (charges, far smell, lure, raids) go through the
  gates: `Game::slime_ground` in `raids.rs` is the raid ground less the
  wall cells.
- The first gate of the home ring faces the middle of the map and every
  other ring's faces home; `open_gate` turns a ring so that at least two
  gates open on ground.
- A fire burns the stretch of wall between two gates and stops at them (the
  fed slime in a gate does not burn). The burnt stretch had not grown back
  70 s later; whether a wall chewed by shots refills was not checked.
- Not checked: a squad fighting through a ring, planets other than 3 (the
  rule is the Trunk's alone), the GTX 1070's time. Look and interface
  reviews the clips: the set slime's flat slab is now a standing feature.
- It belongs in the origin's design page (job 3): a walled home crystal is
  the base to break into.

**The lure's field: merged** (#457; the owner's card, 11-18 03:03Z: "Lure
now, in the three planets";
[report](../Reports/lure-field-2026-11-18/README.md)). Calm slime within
`Rules::lure_reach` (60 cells of walking) of a Lure that is on, or of a
`LureMark` in `Game::lure_marks`, leans down the walking distance to it.
The weight is the engine key `lure_pull`, 0 by default and
`run::LURE_PULL` 10 on every planet (Direction's to tune). GPU only.

- The field is the third quarter of the cells' region in `news`
  (`lure_dist` in engine.wgsl). The pull is Params word 844, sent only
  while a field is written (`Engine::lured`).
- Charging, surging, raiding and breakaway walkers and mini-boss members
  ignore it (this lane's call); Blind and Sour apply to it.
- Squad and arsenal brought the Lure back to the run's build card (300
  food, 150 health; free from planet 1 with no unlock since #464) and wrote
  its news line. Look and interface filmed it pulling on planets 2 and 3
  (412 and 503 walkers within 30 cells at 0:40, 11-18). Films use
  `lure=X,Y[,T]` (the building); `lure_mark=T,X,Y,SECS` lays a mark.
- The rings on the ground are Look and interface's (`View::set_steer`,
  #458). This lane's call is in the window's `prepare_now` (`main.rs`):
  each frame, the field's word (`dense_at / 4 + DENSE_HEAD + n + 2 * nb`)
  while `Engine::lured`, else 0.
- Not checked: a lure behind a wall with a gap, the GTX 1070's time.

**Job 1, the switches: done.** What became of each of the fourteen is in
`Reports/enemy-switches-2026-11-17/README.md` (the roadmap that held the
table is history since 11-18). In short:

- On on every run planet: the far smell (`far_smell`, in `run::enemy_keys`)
  and charges round walls (`unit_paths`, in `run::ENEMY_RULES`).
  `play-new-slime.cmd` is deleted.
- The Trunk's trait (planet 3; `TRUNK_KEYS`, `TRUNK_RULES` in `run.rs`):
  crust, its pull at +8 with fast ground, setting, the wall (`set_block` 5)
  and `fire_fed`, and since 11-18 the fort. Its card: "It walls the
  crystals it holds in amber, and sets where you shoot it".
  Planet 3 was picked over a deeper tier because it is inside Milestone 1
  and its old card was the Drifter's.
- The Swarmer's trait (planet 4; `SWARMER_RULES`): the last stand.
- Deleted, with dead-end lines: the trap (`set_hold`), and the GPU game's
  half of lossy flow and the lone step (which freed Params words 844, 880,
  884 and 888; the lure took 844). `src/mould/big.rs` still has `flow_loss` and `lone_step`: it is
  frozen until Milestone 2 (the owner's 19:21Z answer).
- Not deleted, against the brief: the reserve recruit. The run never turns
  `Rules::reserve` on; it is the frozen campaign's mission rule (a free
  soldier 20 s after the last one dies, the owner's 11-12 no-soft-lock
  pick), so deleting it would change the campaign. The switch list gave it
  the row "Free recruits", which is Squad and arsenal's. Direction was told.
- Still off: crystals that refill (`crystal_regrow`), which is D7's. It is
  the only off switch in this lane's files.
- The switch list in code (`src/build/switches.rs`, Engine and tools') lost
  the ten rows this settled and has four left: `crystal_regrow` (this
  lane's), `set_look` (Look and interface), `feel` and `reserve` (Squad and
  arsenal). That is one over the plan's three, so its `SETTLING` is still
  true; the lane that settles the next row sets it false.
- Handed over: `setlook` and the haze to Look and interface (amber is what
  the game draws). The flinch stays at 0.9 (`shape_keys`), and the 11-15
  blind page for it is closed: it is this lane's number, changed if the
  owner says the slime backs off too much or too little.
- The crust "name the ground" quiz is dropped (Direction's sort, 11-17).

**Numbers, all first numbers from one film each on the software GPU**
(run seed 1, `mode=squads hand=greedy`, at 2:00; before, after):
planet 1, 42 soldiers and 42 (health 97, 90); planet 2, 18 and 7; planet 3,
1 and 0, with 9,831 of the Trunk's 18,984 walkers set and the squad's kills
by 1:00 down from 7,992 to 3,205. Nothing here was timed on the GTX 1070;
the frame costs quoted in the pull request are the simulation-use
reports' llvmpipe ones, and the 1070 run goes in Engine and tools'
Milestone 1 batch.

**Not done, and known:**

- The last stand has no film on planet 4 (a scripted hand does not cut a
  Swarmer to a quarter in a short film). It is covered by
  `all_off_switches_on_together_play_on_both_engines` and the raid tests.
- The bulwark's map wait and the nest-door check from
  `Reports/sim-use-2026-11-17/e2b-bulwark.md` were never measured.
- `tiers::set_key` (the CPU setter) does not know the Trunk's composite
  keys (`crust`, `setting`, `fire_fed`); the run sets them on the GPU
  engine only, and `every_ladder_tier_does_something_to_you` lets the CPU
  refuse them for the Trunk alone.

**Job 2's gap**: planet 1 does nothing to a squad that drains by hand and
builds no pump (the Drifter only smells pumps). No rush brake (D7):
Direction (11-17 22:28Z) holds "the Drifter also smells crystals drained
by hand" as the first candidate for when the owner rules on D7, and it is
not to be built before, not even behind a switch.

**Planet 1's two hands, for the run bot's paired seeds** (Direction's ask;
the gap between them is the fact D7 needs). Run seed 1 on the software
GPU, this branch, one film each:

- The builder is raided:
  `larder-world look=DIR run=1:1 mode=squads hand=pumps at=30,60,90,120,180`.
  The pump at (290,50) is at 187 of 200 hp at 0:30 and 73 at 1:00; raid 1
  goes at 0:59 for the pump at (374,25); both pumps are worn down by 1:30.
  The squad of 4 is at health 67 at 1:30 and 12 at 2:00 (3 soldiers), and 1
  soldier is left at 3:00.
- The rush is not touched:
  `larder-world look=DIR run=1:1 mode=squads hand=greedy at=20,60,120`.
  42 soldiers at health 90 at 2:00, 3,283 killed, 38% of the crystals
  left; two raids of about 1,670 walkers went at the squad (0:59, 1:35)
  and the slime was down to 9,603 walkers in its bowl.

The same builder hand on the other two planets (`run=1:2`, `run=1:3` with
`crustlook=1`, `at=20,40,60,90,120`): on planet 2 the squad of 4 is dead
by 1:30 (raid 1 at 0:59, pumps at 183 and 140 hp at 1:00 and down by
1:30); on planet 3 raid 1 goes at 1:12, the pumps are down by 2:00, 578
walkers are set at 1:30 and 3,019 (2,932 wall cells) at 2:00, and the
squad of 4 is at health 40.

**Filming notes** beyond `Reports/whole-game-review-2026-11-15/kit.md`:
`light=T,X,Y,R` takes dish cells, which are half the world units the log's
`squad at` prints on a run planet (scale 2); `centre=`, `hold@` and `boom=`
take world units. `crustlook=1` adds the set-walker and wall-cell counts to
the log.

---

# Lane brief: Enemy

Model and effort: Fable, extra high. Read `00-all-lanes.md` in this folder
first.

## Your job

The slime is the game's enemy and its best idea. You own its rules
(`src/mould/big.rs`, `src/gpu/engine.wgsl`), `src/mould/tiers.rs`,
`larder_world/{bosses,raids,shapes,bulwark}.rs`, and the ladder in
`src/mould/run.rs` (`LADDER`, `does_to_you`, `enemy_keys`). For Milestone
1 your target is exit test 4: after playing planets 1 to 3, Scott can say
in a sentence what each slime did to him.

Start from `handoff/slime-enemy.md`, `handoff/simulation-use.md`,
`handoff/mini-boss.md` and `handoff/run-progression.md` as history.

## First jobs, in order

1. **Settle the 14 built-but-off switches** (ROADMAP, "Switches that are
   off") in your first two days. Scott cannot test what he cannot see.
   - Far smell (`far_smell`) and charges round walls (`unit_paths`): on by
     default on the run's planets. Delete `play-new-slime.cmd`.
   - Setting, block or trap (`set_block`, `set_hold`), crust and its pull,
     fed slime fireproof (`fire_fed`): pick one configuration yourself and
     give it to one deeper tier as that tier's trait, as fire shyness
     became the Sprawler's. The 11-17 films exist if you need them.
   - Reserve recruit, lossy flow, lone step: delete, each with its
     `Reports/dead-ends.md` line.
   - Last stand: one deep tier's trait, or delete.
   - Crystals that refill: stays off; it belongs to Scott's D7 answer.
   - The set slime's look (`setlook`) and the haze strength go to Look and
     interface. You keep the flinch strength.
   After this at most three switches are off in the whole game, each
   declared in the switch list Engine and tools is building.
2. **Planets 1 to 3 each do one nameable thing to the player by minute
   2.** Today's film of run seed 1 (one seed, scripted hand): on planet 1
   a drain-and-buy squad reaches 42 soldiers at 2:00 with nothing touching
   it and the slime stays in its bowl; on planet 3 the same hand is down
   to 1 soldier at 2:00. Make the three steps of that curve deliberate.
   Deliver one still or clip per planet showing its thing. Do not add a
   rush brake (prices, soldier caps): that is D7 and Scott is holding it.
3. **Then the origin** (Scott's 11-16 idea, Milestone 2's win rule): the
   body at the slime's start crystal as the thing you kill to win a
   planet. Write the design in a page for Direction before building.

## Rules particular to this lane

- New slime rules are written on the GPU engine and tested on the
  software GPU. Write no new CPU twin until Scott answers the 19:10Z card.
  Prefer levers that write fields both engines already have (scent,
  alarm, `squad::Burst`).
- Quote the frame cost of anything that runs every step. The GTX 1070
  runs go in one batch before the milestone check; ask Engine and tools.
- New mini-boss kinds: Scott picked the Mortar first (11-20 13:21Z);
  the Pinwheel (his doc's second proposal) is next.
