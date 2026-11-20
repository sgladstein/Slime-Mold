---
title: Squad mode
kind: design
status: open
updated: 2026-11-20
sources:
  - Reports/auto-fire-2026-11-18/README.md
  - Reports/bug-sweep-2026-11-14.md
  - Reports/build-flow-2026-11-14/README.md
  - Reports/command-card-2026-11-15/README.md
  - Reports/free-camera-2026-11-15/README.md
  - Reports/guards-2026-11-15/README.md
  - Reports/map-sharing-2026-11-16/README.md
  - Reports/narrow-squads-2026-11-15/README.md
  - Reports/pathfinding-2026-11-20/README.md
  - Reports/soldier-cap-2026-11-16/README.md
  - Reports/squad-mode-2026-11-14/README.md
  - Reports/squad-movement-2026-11-16/README.md
code:
  - Game::demolish
  - Game::drawn_between
  - Game::fight_turn
  - Game::guard_zone
  - Game::heal
  - Game::rank_squad
  - Game::steer_point
  - MAP_NEAR
  - Rules::funnel
  - Rules::guard
  - Rules::max_squad
  - Rules::squads
  - Rules::steer
  - squad::UNITS_MAX
---

# Squad mode

## Now

You steer one squad of soldiers with WASD; it volleys at the cursor while the
left button is held, and fires twice as often with a 1.4x strike while it is
the one you command. Let go of the button and it fires by its fire mode (Firing on its own, below). The squads you do not command keep their last order and fire on
their own. Squads split, join, link, queue orders and stand guard while
they drain or build; there is no larder. It is `mode=squads` (`Rules::squads`),
the start menu's *Control* option. It is the window's default (2026-11-15, #330; owner 07:57Z); missions and
headless `look=`/`bench` runs stay on the RTS round (`mode=rts` or Control: rts
brings it back), so the campaign keeps the old mode (owner 08:09Z). Detail: [report](../../Reports/squad-mode-2026-11-14/README.md).
Up to 200 soldiers since 2026-11-16 (`Rules::max_squad`; 40 before; owner
05:06Z picked "Flat 200, the realistic max": "I want to test the max"); a
soldier costs the CPU about 0.01 ms a step on planet 1 and 0.03 ms on a 6x
planet, and the GPU next to nothing; on the 1070 at 200 soldiers a step's
frame p95 is 9.1 ms on planet 1 and 14.9 ms on planet 8
([report](../../Reports/soldier-cap-2026-11-16/README.md)); balance at 200
is still to come. A walking map serves any goal within `MAP_NEAR` 12 dish cells that has a straight walk from the map's goal, about a squad of 25's formation: at 4, 200 soldiers in 8 squads on a 6x planet overflowed the map cache and 47% of map lookups went unserved, at 12 19% ([report](../../Reports/map-sharing-2026-11-16/README.md)).

## How it plays

- **Firing on its own** (2026-11-18). Not volleying, the commanded squad
  fires by its fire mode like every squad: *defend*, the default, shoots the
  thickest slime within 7.5 cells of each soldier, so among loose walkers the
  soldiers spread their fire; *free* shoots the thickest in the gun's reach.
  Before, it was held to defend whatever its mode, and defend read the whole
  reach, so a clump 20 cells off hid the walkers on the squad
  (`Game::stance_of`, `Game::sight`;
  [report](../../Reports/auto-fire-2026-11-18/README.md),
  [page](https://claude.ai/artifact/EdTkz7B7MwwHV7FbRtgGE3)). A held volley
  still puts every bullet on the cursor: spreading it over the slime round
  the cursor won at a body's edge but lost among loose walkers, so it was not
  kept ([dead ends](../../Reports/dead-ends.md)).
- **Attack-move stands only for a fight** (owner, 2026-11-19 16:55Z: one
  walker stopped the whole squad). A squad on attack-move stands to fight
  when the walkers in its members' reach reach `Rules::fight_hold` (1) a
  living member, at least `FIGHT_LEAST` (3), or a mini-boss is in reach, and
  stands till under half that (`Game::fight_turn`, `Squad::holding`);
  fewer, and it walks on, shooting as it goes. `set=fight_hold=0` is the old
  way, where any soldier with a walker in reach stood still.
- **Commanding.** The bottom bar's middle panel has a tile per squad (hotkey,
  soldiers, health, state); with more than fit, every tile shrinks to its
  key, count and health rather than some hiding behind "+N" (11-19). Click a
  tile or press its number to take command; ` (Shift + ` back) and Ctrl +
  wheel go round them. **Esc** or *Let go* commands none: every squad shows
  AUTO, WASD pans, and a click on a squad takes it back
  ([free camera](../../Reports/free-camera-2026-11-15/README.md)).
- **Loadouts** (owner, 2026-11-19 19:22Z; `run::Loadout`, `Run::loadouts`).
  A loadout is a gun, or none, plus its cards; each squad (`Squad::loadout`)
  and each turret (`Building::loadout`, `Building::deck`, `Cmd::TurretLoad`)
  fires one, and a change to it changes all of them. A gun or a card is in
  one loadout at a time. Squads land on the first; new turrets fire
  `Run::turret_loadout`'s cards (the turrets' deck). A loadout with no gun
  fits the turrets' rack. On a planet H opens the panel, paused, with
  nothing for sale; changes apply at once, with no reload.
- **Split, join, link.** Y splits the front half off; J or the tile's + joins
  only a squad that touches yours. Linked squads (Ctrl + number, only while
  touching) are commanded as one and keep their own guns and settings.
- **Orders.** Right click sends, Ctrl + right click drains or guards a crystal,
  Shift queues up to six. Formations: blob (default), line, wedge, ring.
- **Lookouts** (called Guards until 2026-11-15 19:51Z, so the word stays the
  Guard order's). A squad draining or building with slime near posts a share of
  itself as lookouts (the card's Lookouts, key I: none, some, more) at the work's edge on the slime's
  side; 3 of 8 cost about a fifth of the work (`cover.rs`). Posted out in the
  slime's film (#276) they died for nothing; at the edge they killed a third
  more slime than none and the workers were bitten less, 4 of 4 seeds
  ([report](../../Reports/guards-2026-11-15/README.md)).
- **Guard** (owner 2026-11-15 19:51Z, in Burn a line's place, which is gone):
  F, then click a spot, a building or a crystal. The squad walks there as one
  and shoots slime in the zone on top of its own defence: `Rules::guard_r` (8
  cells) round a spot, `GUARD_RIM` (3) past a crystal's rim or a building's
  footprint (`Game::guard_zone`, `Squad::guard`, `Order::Guard`); a ring shows
  the zone. Steering or another order ends it. Defend stays the default fire
  mode ([report](../../Reports/command-card-2026-11-15/README.md)).
- **The command card** is Card 2 (owner 19:38Z): 5 x 2, Attack-move, Guard,
  Drain, one fire button that cycles hold, defend and free (Z, X, C pick one),
  Heal; Buy, Pump, Build, Tools, Lookouts. The Build page (option 1, 19:53Z):
  turret, pump, wall, lure, siren, then the lures' switch and Back. With no
  squad commanded a click on a building selects it: its health, level and
  state, and its card (call off, demolish for half its price back, upgrade a
  pump, switch a lure).
- **Healing is an order** (#330): tell a squad to heal (Heal on the card, key L; the
  whole link). It stops, each member heals after 1 s still, paying food from the
  bank, and stops when all are whole or on any other order
  (`Game::heal`, `Cmd::Heal`). Nothing heals on its own.
- **How soldiers walk** (rework 2026-11-16, `Rules::steer`, on; owner
  05:17Z: "still clunky overall"). Each soldier has a velocity: in step
  with its squad's anchor, closing on its place, slowing into it; elbow
  room and queuing instead of shoving; places in rock kept as a distance
  from the anchor; one walking map per squad; any angle down a map; the
  window draws soldiers between game steps and turns figures smoothly.
  The anchor eases into its goal and waits softly. Numbers, films and what
  is still open (big squads in caves; places fitted to the ground):
  [report](../../Reports/squad-movement-2026-11-16/README.md),
  [page](https://claude.ai/artifact/MWtDtnMANViS1NytZLwaNB). Jerk lower on
  23 of 24 planet runs, turning on 24 of 24; 200 sent on open ground
  4.9 -> 0.9 ms a step.
  Pathfinding review (2026-11-20): a squad steered past a rock's tip or
  along a slanting wall now walks round it (it stood for good: the point
  40 cells ahead fell in rock, `Game::steer_point`), an anchor beside a
  crystal takes its map, and members behind a crystal walk to the anchor.
  Steered squads reached 4.8 crystals a map against 1.9 (19 of 24 maps
  more, none fewer), back-and-forth walking 9.7% -> 4.9%
  ([report](../../Reports/pathfinding-2026-11-20/README.md)). Open: walking
  maps don't steer round slime or fire.
- **Narrow gaps.** Squads funnel into columns through bridges and doors
  narrower than their formation (`Rules::funnel`, on); with the new walk a
  column ends once its tail is near the anchor
  ([report](../../Reports/narrow-squads-2026-11-15/README.md),
  [rework](../../Reports/squad-movement-2026-11-16/README.md)).
- **Building** is shared with the RTS round: ghosts soldiers raise from reach 6,
  Shift queues, an unfinished one can be called off for the whole price
  ([report](../../Reports/build-flow-2026-11-14/README.md)).

Income and crystals: [economy](economy.md). Guns: [guns](guns.md).

## Owner's word

- 2026-11-14 04:08Z: "the unit control should be squad based from the ground up. All soliders in a squad stick together at least as well as they can."
- 2026-11-14 04:49Z: "This should be built as a seperate mode for right now. Not erase the other game."
- 2026-11-15 07:57Z: "squad mode is default mode".
- 2026-11-15 08:09Z: "No campaign can stay the old mode".
- 2026-11-15 07:41Z: "Healing should be a specific action".
- 2026-11-15 19:38Z: "Card 2"; 19:51Z: "Remove burn line. Use options c for guard/defend. (b) Click a building to select it when no squad is selected."; 19:53Z: "Build option 1"; 21:37Z: "Demolishing a building giving half back is good. Canceling a partial build should give all of it back".
- 2026-11-15 18:00Z, on guards while working: "It doesn't seem to work well. Either improve it or remove it." (Improved: posts at the work's edge.)
- 2026-11-16 05:17Z, of squad movement and pathfinding: "It is still clunky overall" (start over rather than patch).
- 2026-11-18 17:01Z: "When I am surrounded by a bunch of individual walkers ... it isn't ideal to have all the soliders shooting at the same point anymore. So I typically stop manually shooting and let them shoot automatically, but the soliders seem to shoot much slower or not all of them are shooting" (fixed the same day: the commanded squad keeps its fire mode and defend reads what is near).
- 2026-11-15 02:58Z: a way to deselect, move with WASD and see each squad run on its own (built as Esc and AUTO).

## Open

- A call-in delay for bought soldiers.

## Sources

- [squad mode](../../Reports/squad-mode-2026-11-14/README.md), [auto-fire](../../Reports/auto-fire-2026-11-18/README.md), [squad movement rework](../../Reports/squad-movement-2026-11-16/README.md), [bug sweep](../../Reports/bug-sweep-2026-11-14.md), [free camera](../../Reports/free-camera-2026-11-15/README.md), [narrow squads](../../Reports/narrow-squads-2026-11-15/README.md), [build flow](../../Reports/build-flow-2026-11-14/README.md).
