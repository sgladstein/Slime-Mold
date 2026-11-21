---
title: Squad mode
kind: design
status: open
updated: 2026-11-21
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
  - Formation::Column
  - Game::demolish
  - Game::drawn_between
  - Game::fight_turn
  - Game::guard_zone
  - Game::heal
  - Game::mark_hot
  - Game::rank_squad
  - Game::rim_spots
  - Game::steer_point
  - MAP_NEAR
  - Rules::fireway
  - Rules::funnel
  - Rules::guard
  - Rules::max_squad
  - Rules::squads
  - Rules::steer
  - SPREAD
  - squad::UNITS_MAX
---

# Squad mode

## Now

You steer one squad of soldiers with WASD; it volleys at the cursor while the
left button is held, and fires twice as often with a 1.4x strike while it is
the one you command. Let go of the button and it fires by its fire mode (Firing on its own, below). The squads you do not command keep their last order and fire on
their own. Squads split, join, link, queue orders, take a formation and
drain or build with every member on the work; there is no larder. It is `mode=squads` (`Rules::squads`),
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
  AUTO, WASD pans, and a click on a squad or its number takes it back
  ([free camera](../../Reports/free-camera-2026-11-15/README.md); the number
  did nothing here until 2026-11-21). The cursor is the crosshair while a
  squad is commanded and the pick arrow while Ctrl is held, since a Ctrl
  click or drag then picks squads instead of firing (`cursor.rs`). On a card
  page (Build, Tools, Formation) Esc first goes back to the main card
  (2026-11-21). **Space** brings the camera back to the squad; while it
  already follows, Space looks at the newest alert that has a place (a raid,
  landing ore) and Space again comes back (2026-11-21; Space is the alerts'
  key outside the squad mode). Only a Shift + drag splits soldiers off: a
  Shift + click no longer pulls the one soldier under it into a squad.
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
- **Picking squads with a drag** (owner 2026-11-21 06:53Z: "add click and
  drag to select multiple squads"). With no squad commanded a plain left drag,
  with one Ctrl + drag, boxes squads: every squad in the box is commanded as
  one, as a link is (moves, queues, volleys, facing, heal, formation, fire
  mode, Hold fire), led by the one you commanded if it was in the box
  (`Cmd::Band`, `Squad::band`). Sent together, squads picked far apart close
  up round the click, at most `BAND_SPREAD` (6) cells a squad from the middle.
  It is a selection, not a link: a click on one squad (Ctrl + click with one
  commanded), its number, its tile or Esc ends it. A plain drag with a squad
  commanded still fires, and Shift + drag still splits. The fire button and
  Hold fire now reach linked squads too (they changed only yours before).
- **Orders.** Right click sends, Ctrl + right click drains or guards a crystal,
  Shift queues up to six. A tap of WASD is the stop (the card has no Stop)
  and drops the queue (2026-11-21; before, the squad walked off to its next
  queued order once let go). A bought soldier and a squad combined into
  yours take your squad's fire mode.
- **Formations** (the card's Formation button, T, opens its page; owner
  2026-11-21 05:04Z): blob (default), spread (the blob at 1.9 times its room,
  `SPREAD`), line, wedge, column (two abreast, facing where it walks; moving, it
  always walks as the funnel's column two abreast, so it files through doors
  and round rock) and
  ring (`Formation`, `Game::rank_squad`).
- **No lookouts** (owner 2026-11-21 05:04Z: "remove the feature that some
  soldiers guard while harvesting"). From 2026-11-14 to 11-21 a squad
  draining or building with slime near posted a share of itself as lookouts
  at the work's edge, and its workers fired at half rate; now every member
  works and fires at full rate (`cover.rs`; the old measure is in the
  [report](../../Reports/guards-2026-11-15/README.md)).
- **Guard** (owner 2026-11-15 19:51Z, in Burn a line's place, which is gone):
  F, then click a spot, a building or a crystal. The squad walks there as one
  and shoots slime in the zone on top of its own defence: `Rules::guard_r` (8
  cells) round a spot, `GUARD_RIM` (3) past a crystal's rim or a building's
  footprint (`Game::guard_zone`, `Squad::guard`, `Order::Guard`); a ring shows
  the zone. Steering or another order ends it. Defend stays the default fire
  mode ([report](../../Reports/command-card-2026-11-15/README.md)).
- **The command card** is Card 2 (owner 19:38Z) as changed 2026-11-21, 6 x 2
  with the gun strip narrower (06:21Z): Attack-move, Guard, Drain, the fire button (X: defend or free fire), Hold
  fire (Z, a toggle; a held soldier fires the button's mode once let go,
  `App::unheld` in the window), Heal; Buy, Pump, Wall, Build, Tools, Formation. (The RTS
  round keeps one fire button that cycles hold, defend and free.) The Build page (option 1, 19:53Z):
  turret, pump, wall, lure, siren, then the lures' switch and Back. With no
  squad commanded a click on a building selects it: its health, level and
  state, and its card (call off, demolish for half its price back, upgrade a
  pump, switch a lure).
- **The device slot** (owner 2026-11-21 06:55Z: "One device slot per squad,
  on a cooldown", his 11-16 gun-system ruling, step E's first part). The
  Tools page (E) throws from the squad you command: a soldier of it within
  `reach` (24 cells) sets the tool off, free, with nothing held, and the
  squad's one slot then waits that device's own cooldown before any device
  goes again (`Cmd::Device`, `Game::device`, `KitRules::cooldown`: blast
  20 s, mine 25, det cord 30, sleep gas 35, germ 40, provoke and still 30;
  first guesses; the shop's cheaper-tools upgrade shortens them). Squads
  commanded together each have their own slot, the commanded one first.
  The wait shows on every tool and on the Tools button; with no tool owned
  yet (a run's planet 1) the Tools button is gone. Before, the squad mode
  kept the RTS round's carry rule: a soldier took a tool only within 6 cells
  of the undrawn landing spot, so each threw one a planet and a bought one
  none. Devices are commands, so they replay. Still to come from step E:
  bodies holding gun cards, rolled stats, autocast.
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
  ([report](../../Reports/pathfinding-2026-11-20/README.md)). Routing round
  slime was tried and not kept: the slime comes to the squad anyway.
  **Round fire on a move** (2026-11-21, `Rules::fireway`, on; Direction's
  call): a squad sent with a plain move walks round ground marked burning
  (a burn or light burst, or a soldier that felt heat, for 15 s) where
  there is a way round; a squad standing to fight in fire stays, the price
  of a fire deck. Health lost to fire on the way 413 -> 274 (24 of 24 maps
  lower), trips no slower on the probe's maps.
  **Own spots round a crystal** (2026-11-21, `Game::rim_spots`; Look's exit
  test 2 pass): soldiers draining, guarding or standing at a crystal each
  take their own spot round its rim, `SPOT` 3.5 dish cells apart (a drawn
  figure's width), the ring filled all the way round before a second ring
  starts. Before, each soldier stood at the rim on its own side, so the
  squad pressed into one arc. Planet 1, seed 1, greedy hand: nearest
  neighbour 2.1 -> 3.5 cells median, under 2.4 cells 92% -> 0%, harvest
  at 60 s 4,081 -> 4,199. A first try that put a second ring out before the
  first was full halved the harvest (the outer ring is out of reach).
  Since the follow-up (same day) only move, guard and drain orders take
  spots: on attack-move or attack at a crystal (the run bot's rush at the
  slime's home crystal) the squad spread round the whole rim, half of it
  through the slime, and CI's run bot took 16% longer over seeds 1 to 3
  (6 of 9 planets longer; losses 59 -> 31, harvest no worse). Every ring
  inside harvest reach (the shrinking disc to the whole radius) fills
  before any past it.
- **Narrow gaps.** Squads funnel into columns through bridges and doors
  narrower than their formation (`Rules::funnel`, on); with the new walk a
  column ends once its tail is near the anchor; since 2026-11-21 no
  column place lies past a wall from its trail (60 soldiers leaving a
  walled base by a 4-cell door stood inside for good; now out in 26 s)
  ([report](../../Reports/narrow-squads-2026-11-15/README.md),
  [rework](../../Reports/squad-movement-2026-11-16/README.md)).
- **Building** is shared with the RTS round: ghosts soldiers raise from reach 6,
  Shift queues, an unfinished one can be called off for the whole price
  ([report](../../Reports/build-flow-2026-11-14/README.md)). Since
  2026-11-20 Shift + a build, harvest, guard or attack-move goes on the
  squad's own queue (`Cmd::SquadQueue`), so move then Shift + build raises
  it once the squad gets there; before, Shift skipped the squad and the
  build was never raised (owner's report, 2026-11-20).

Income and crystals: [economy](economy.md). Guns: [guns](guns.md).

## Owner's word

- 2026-11-14 04:08Z: "the unit control should be squad based from the ground up. All soliders in a squad stick together at least as well as they can."
- 2026-11-14 04:49Z: "This should be built as a seperate mode for right now. Not erase the other game."
- 2026-11-15 07:57Z: "squad mode is default mode".
- 2026-11-15 08:09Z: "No campaign can stay the old mode".
- 2026-11-15 07:41Z: "Healing should be a specific action".
- 2026-11-15 19:38Z: "Card 2"; 19:51Z: "Remove burn line. Use options c for guard/defend. (b) Click a building to select it when no squad is selected."; 19:53Z: "Build option 1"; 21:37Z: "Demolishing a building giving half back is good. Canceling a partial build should give all of it back".
- 2026-11-15 18:00Z, on guards while working: "It doesn't seem to work well. Either improve it or remove it." (Improved: posts at the work's edge.)
- 2026-11-21 05:04Z: "remove the feature that some solider guard while harvesting. add a formations button to the hud with good formation options. hold fire should be a seperate button from defend vs free fire." (Built: lookouts removed, the Formation page, Hold fire its own toggle.)
- 2026-11-16 05:17Z, of squad movement and pathfinding: "It is still clunky overall" (start over rather than patch).
- 2026-11-18 17:01Z: "When I am surrounded by a bunch of individual walkers ... it isn't ideal to have all the soliders shooting at the same point anymore. So I typically stop manually shooting and let them shoot automatically, but the soliders seem to shoot much slower or not all of them are shooting" (fixed the same day: the commanded squad keeps its fire mode and defend reads what is near).
- 2026-11-15 02:58Z: a way to deselect, move with WASD and see each squad run on its own (built as Esc and AUTO).

## Open

- A call-in delay for bought soldiers.

## Sources

- [squad mode](../../Reports/squad-mode-2026-11-14/README.md), [auto-fire](../../Reports/auto-fire-2026-11-18/README.md), [squad movement rework](../../Reports/squad-movement-2026-11-16/README.md), [bug sweep](../../Reports/bug-sweep-2026-11-14.md), [free camera](../../Reports/free-camera-2026-11-15/README.md), [narrow squads](../../Reports/narrow-squads-2026-11-15/README.md), [build flow](../../Reports/build-flow-2026-11-14/README.md).
