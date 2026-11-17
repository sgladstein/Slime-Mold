---
title: The squad toolkit
kind: tool
status: open
updated: 2026-11-17
sources:
  - Reports/burning-trails-2026-11-14.md
  - Reports/card-text-2026-11-15.md
  - Reports/explosions-2026-11-14/README.md
  - Reports/flashes-of-chaos-guns-2026-11-12.md
  - Reports/gas-cloud-2026-11-14.md
  - Reports/gun-budget-2026-11-14.md
  - Reports/gun-route-2026-11-17/README.md
  - Reports/gun-shapes-2026-11-16/README.md
  - Reports/guns-2026-11-14.md
  - Reports/larder-world-roadmap-2026-11-11.md
  - Reports/no-recharge-2026-11-14.md
  - Reports/noita-spell-look-2026-11-14.md
  - Reports/rolled-bullets-2026-11-14.md
  - Reports/rounds-2026-11-12.md
  - Reports/rounds-look-2026-11-14/index.html
  - Reports/shot-lights-2026-11-14.md
  - Reports/sleep-gas-gpu-2026-11-14.md
  - Reports/squad-toolkit-2026-11-12.md
  - Reports/squad-toolkit-round2-2026-11-12.md
  - Reports/throw-aim-2026-11-16/README.md
  - Reports/trail-cards-2026-11-14.md
code:
  - Ammo
  - Ammo::roll
  - BigDish::fling
  - BigDish::gas
  - BigDish::gassed_near
  - BigDish::lay_gas
  - BigDish::scorch_disc
  - BigParams::fling_deposit
  - BigParams::fling_speed
  - BigParams::gas_death
  - BigParams::gas_keep
  - BigParams::germ_resist
  - BigParams::germ_upkeep
  - BigParams::scorch_full
  - BigParams::scorch_keep
  - BigParams::sick_upkeep
  - Bullet::cast
  - Bullet::speed
  - Burst
  - Engine::burst
  - Engine::calm
  - Engine::fling
  - Engine::gas
  - Engine::lay_gas
  - Game::arm
  - Game::gun
  - Gun
  - Gun::roll
  - GunKind
  - Kit
  - Kit::restock
  - Kit::thrower
  - KitRules
  - KitRules::carry_max
  - KitRules::gas_cloud
  - Soldier::load
  - Soldier::smell
  - Throw
  - Tool
  - World::burst
  - World::calm
  - squad::Burst
---

# The squad toolkit

## Now

Six tools for the Larder world's squad, each built on one term of the
mould's model (`mould::kit`): blast, mine, det cord, germ, sleep gas, and
mask or reek on a soldier's smell. Every tool comes down to bursts
(`squad::Burst`) that a world applies (`World::burst`): blast, cord and mine
run on the GPU world too (the engine's strike lays the crater), and since
11-14 the sleep gas ([report](../../Reports/sleep-gas-gpu-2026-11-14.md))
and the germ (the strike's `kill` pass infects; the `life` pass charges the
sick `sick_upkeep`: on race seed 5, 77 made sick and 449 of 2,123 left after
20 s on both engines). A mine's trigger scales with `Game::density`. In
the window (`larder-world`) E opens the tools page: blast, det cord and
mine, germ and sleep gas work; every soldier at the larder takes one tool, a selected one up
to five. Since 2026-11-16 aiming a tool shows who throws (`Kit::thrower`),
its reach ring, a lobbed arc to the landing point and the area in the tool's
colour, all red with the click refused out of reach (`Model::throw`, `Throw`;
[report](../../Reports/throw-aim-2026-11-16/README.md)). The crater a blast, cord
or shot leaves (`BigDish::scorch_disc`; GPU binding 47 since #113, shared
`scorch_keep` and `scorch_full`: 0 and 150 in the race world since PR 3b's
cone, were 0.1 and 30, which earlier `kitplans` tables used) is the one
scorch in the game. Each tool
shows in the mould; mines are weak and the det cord starves nothing until
food flows ([report](../../Reports/squad-toolkit-2026-11-12.md)). Tools take
carry space and come from the larder (`KitRules`: a runner holds 5 at 50
carry each). At 32 seeds on main, Holm over the table, the best plan changes
with the tier: loud (blasts carried to the body) over plague (the germ) on
the Drifter (30/2), plague over loud on the Sporer (30/2); by the mean
plague also leads on none, Swarmer, Sprawler and Grudge, loud on Sleeper
and Trunk. Random play beats or ties every plan on the Drifter and the Trunk
(gate 1 fails there, before the core pull of #126; re-run on main pending). Gas and the det cord are never best
([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Confirmed at 32
seeds*; [page](https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC)).

Built on the CPU (2026-11-12, owner: "Build it"): **rounds**, after the ammo-card guns of
Flashes of Chaos and Noita's wands. A soldier's or turret's shot becomes up to
four cards in order (payloads such as crater, bait, alarm, germ, sleep, scar;
modifiers such as pierce, scatter, quiet; triggers that read the mould, such as
"when thick" or carried along the tubes), so the toolkit's tools ride on shots
and the best loadout can vary by tier
([research](../../Reports/flashes-of-chaos-guns-2026-11-12.md)); reviewers: compile rounds onto
this toolkit's burst list, not a new system. First census
([rounds](../../Reports/rounds-2026-11-12.md), `src/mould/rounds.rs`): the
germ, on its own or as plague, beats no round 8/0 on plain, Swarmer and Sporer
and the Sleeper ignores every round, so the best loadout does not yet vary by
tier. Germ resistance 10 on the Swarmer and Sporer cancels it (4 seeds), 3
halves it: a per-mould dial. Rounds now cast through `World::burst`, so they play in
the `larder-world` window (`rounds=`, H; gas and germ play there since 11-14). On Flats, blast and
trap keep a soldier alive to 32 s where plain shots lose the squad by 24 s.
Since 11-13 shots fly as bullets (`Rules::bullet_speed` 45 dish cells a
second; 0 gives back the ray), and the flight cards pierce and split ride on
every bullet; they are game state, so both engines play them alike (an
exact bullet twin in `mouldgpu` checks the engine's trace of each leg).
After the owner's 11-14 playtest ("the bullets are really bad"; "shouldn't just be
flashy effects") the rounds' look follows Noita: what a card changes in the world
is its look ([research and plan](../../Reports/noita-spell-look-2026-11-14.md)):
bullets as lights on the real slime, blasts that fling live mould, gas as a
drifting field, trail cards that lay real fields, and a loadout panel. Steps 1-2 are
built ([films](../../Reports/rounds-look-2026-11-14/index.html)): each bullet is a streak
in its payload's colour and a light on the real slime (`View::set_lights`), and a burst
lights its area for 0.12 s; the simulation is unchanged. Step 4 is built on both engines
([report](../../Reports/gas-cloud-2026-11-14.md)): a gas burst also lays a cloud
(`BigDish::lay_gas`) that spreads and halves every `KitRules::gas_half` 4 s, is held by
walls, and puts to sleep any walker on a cell over 0.2; the window shows it as a violet haze.
Step 5 is built ([report](../../Reports/trail-cards-2026-11-14.md)): the flight cards scorch,
alarm and mist lay crater, alarm or a thin gas cloud along every soldier bullet's flight,
through `World::burst` on both engines.
Since 11-14 08:41Z (owner) scorch also burns the walkers on its line (`Burst::burn`,
`World::burned`), and the engine's draw shows crater as a fading ember and alarm as a
green haze straight from the fields ([report](../../Reports/burning-trails-2026-11-14.md)).
The default mould ignores alarm (`alarm_weight` 0), so on 09:02Z (owner's card) the alarm trail
became `bait`: food scent along the flight (`Burst::bait`, GPU pass `baitlay` 60) that every mould follows.
Cards cost budget (`Card::cost`, gun budget 10, capacity at loadout, owner 15:34Z); `rate`, `thrift` and
`smother` (wipes food scent) added and `range` made 2x a card ([report](../../Reports/gun-budget-2026-11-14.md)). Each card says one short sentence with its key number (`draw::card_face`), its kind as Bullet, Next bullet or Always (`CardKind::label`, 11-15; [report](../../Reports/card-text-2026-11-15.md)).
Since 16:46Z (owner) **the squad owns the gun** (`mould::guns`, [report](../../Reports/guns-2026-11-14.md)): a
`Gun` is range, rate, spread, bullets a shot, budget and slots, rolled (`Gun::roll(seed, depth)`) round one of four
kinds (rifle, the old gun; carbine; long gun; scattergun, 2-6 bullets a shot), each stat its own normal draw
(18:07Z; no gun is labelled lucky, 18:18Z), better and with bigger budgets on deeper planets; `Squad::gun`
(`Game::gun`, `Cmd::Arm`, `gun=` and Shift+H in the window), none being the standard rifle of the game's `Rules`;
each squad's own cards (`Squad::deck`, `Cmd::Load`, H in the squad mode), none being the army's round; both halves
of a split carry a copy of the gun and cards (18:05Z); `rounds::fit_gun` and `guns::Loadout` (a squad's gun and
cards between planets). Since 18:05Z there is **no recharge** ([report](../../Reports/no-recharge-2026-11-14.md)):
every shot carries the next cast in turn (`Soldier::next`, `Bullet::cast`) at the gun's rate of fire and casts it
where it hits slime; payloads are bullets, free to fire (18:38Z: "The bank is just to build the gun. How many and
which cards"; at the toolkit's prices eight soldiers on gas and blast spent a 2,000 bank in about 5 s).
**Rolled bullets** (owner's 16:46Z spec, [report](../../Reports/rolled-bullets-2026-11-14.md)): a bullet card
(`Card::Ammo`, `mould::bullets`) is a shell (plain or a payload), a set (standard, slug, needle, buckshot, each
leaning the rolls) and damage plus 0.25-4x speed, range and spread multipliers, each its own normal draw; damage
scales the strike's and the burst's radius as its square root (`Ammo::reach`); its cost (shell base × grade^1.5 ×
luck) comes out of the gun's budget. `Ammo::roll(seed, depth)` is the shop's roll; the old payload cards and
`Card::Plain` are the standard bullets. A shot's lights reach only as far as its damage (owner 20:22Z): a bullet
and its hit their strike (`Shot::r`), a burst its radius, a muzzle 1 cell (`MUZZLE_LIGHT`); a bullet's lit 7 cells
until the owner's 20:09Z playtest ("each bullet that hit has a huge wave that goes out. it is too much";
[report](../../Reports/shot-lights-2026-11-14.md)).
The `range` flight card (11-14 08:39Z, owner's ask) stretches every soldier's sight and
reach by `Rules::range_card` (1 of `Rules::range` a copy since the 15:06Z notes' "2-4x
distance", was 0.5), through `Game::gun_range`: 48 cells with one, 72 with two; building
reach and turrets keep their own ranges.
**Bomb rounds** (11-14, the owner's "Do we have explosions yet"): the
`bomb` payload kills a small core (4 dish cells) and throws the live mould
out to 2.5 times that straight outward, as real walkers in flight
(`Burst::fling`, `BigDish::fling`, `Engine::fling`) that land and live on,
drawn as hot specks with a streak of trail. Loadouts bomb, cluster and
blowout. Exact engine twin on llvmpipe; one shot into C kills 171 and
throws 678 against a blast's 334 killed. Every round, blast too, ends the
rush, idle and farm smoke hands sooner than plain shots on one seed; with
bombs the rush is wiped at 21 s, a tuning note
([report](../../Reports/explosions-2026-11-14/README.md), films page linked there).
Since 2026-11-16 the bomb throws and the blast holds: the bomb kills 2.5 cells,
throws the ring out to 10 cells for 0.7 s, away from the gun, and keeps its
crater 1 s; at C's middle it kills 57 and throws 959 where a blast kills 360
([bullet shapes](../../Reports/gun-shapes-2026-11-16/README.md)); the owner
named all four sets in the blind film quiz (11-16 05:09Z).

## Owner's word

- 2026-11-12 04:25Z: "yes and maybe explosions, mines, etc.  do we need scortch when we have walls?"
- 2026-11-12 04:31Z (card): "All six together".
- 2026-11-12 04:39Z: "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at."
- 2026-11-12 04:46Z (card): "Drop it, scorch is the crater".

## Tried and rejected

- Steering-only contagion as the germ: no effect at 30 s (2 higher, 6 lower against none), as in [`dead-ends.md`](../../Reports/dead-ends.md), contagion.
- A blast price of 120 and a reach of 15 dish cells to slow the loud plan: it still wiped the mould on 4 of 4 seeds ([report](../../Reports/squad-toolkit-2026-11-12.md), *The four plans*).

- Carry 2 tools at weight 100 or 50: loud fell to the farm hand's level and plague led every tier; weight made no difference, the trips home are the cost ([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Counter 1*).
- Amputation by steering (healthy walkers steer from germ, sick ones lay no trail): no effect on the germ ([round 2](../../Reports/squad-toolkit-round2-2026-11-12.md), *Answers to the germ*).

## Open

- Random play wipes the Drifter and the Trunk at 945e080 (gate 1 fails): re-run on main with the core pull (#126) and the new Drifter (#133).
- Round 2 on a second map, and as `larderworld census` hands.
- A tier where gas or the det cord is the best plan.
- `germ_resist` and `germ_upkeep` (a frailty: the sick pay times it) are tier keys since 11-17 (shape C resists 10; the Sporer, Drifter and Sleeper are frail 6): the toolkit's thrown germ, at its own upkeep of 60, took 38.6% of C in 20 s and 54% of the Sporer. The 76% of C quoted before came with a GPU bug that planted a second germ at the map's middle (fixed 11-17); the untuned germ without it took 48% ([route](../../Reports/gun-route-2026-11-17/README.md), seed 0).
- The germ's and the gas's 1070 twins are pending.
- The tools in the game's replay (the kit sits outside `Game`).
- The det cord once food flows (the one-body thread's `energy_flow`).

## Sources

- [squad-toolkit-2026-11-12.md](../../Reports/squad-toolkit-2026-11-12.md), all sections.
- [squad-toolkit-round2-2026-11-12.md](../../Reports/squad-toolkit-round2-2026-11-12.md), all sections.
