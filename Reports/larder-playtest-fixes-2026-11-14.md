# Larder World: harvest refusals, stuck builders and upkeep (2026-11-14)

The owner's playtest of the 11-14 pack (branch `claude/project-thread-pu1x88`)
found three things, 04:44-04:53Z:

1. "They are not harvesting a crystal when I tell them too ... These four
   immediately go back to the larder" (four soldiers, while the rest obeyed).
2. "soilder get stuck on the wall they build".
3. "There should be no maintenance costs in this game".

## 1. Soldiers holding tools refused harvest orders

Cause: the window's kit (`kit_between`, `src/bin/larder-world/main.rs`)
gives every *selected* soldier standing at the larder `KitRules::carry_max`
5 tools, and each weighed `KitRules::tool_weight` 50 of the soldier's
`Rules::carry` 300 (`Soldier::load`). A soldier selected at the larder was
full after 50 food. Sent to a far crystal with 50 or more food already on
it, it turned for the larder at once (`turn.rs`: home when
`carry >= carry - load`), banked, walked out, took 50 and came back: from
the window, a soldier refusing the order. Nothing on screen showed the
load. The four were the ones that had been selected at the larder.

Fix: `tool_weight` 0 by default (the kit's own note from round 2 already
said "weight 100 or 50 made no difference (the cost is the trips home, not
the carry)"; `tool_weight=50` in `examples/kitplans.rs` plays the old rule).
A soldier never heads home with nothing to bank (`carry > 0`), so a load
heavier than the carry can't make it shuttle. A new order also clears
`Soldier::dry`, which a soldier kept from a crystal that had run dry and
which made its next trip home leave the new crystal as if it were dry.
Test: `a_soldier_holding_tools_goes_to_the_crystal_it_is_sent_to`.

## 2. Builders were shut in their own wall, or stuck at its end

Two causes, both in `paths.rs`:

- A wall's builders walked to the nearest point *on* its line (plus the
  squad's slot). When the wall rose, its cells closed under them, and the
  walking map from inside a wall reaches nothing, so they stood in it for
  good. Repro (race world, scale 1, the four starting soldiers ordered to a
  20-cell wall 12 cells west of the larder, seeds 1-5): a builder was inside
  the risen wall on seeds 2 and 3.
- `downhill` stepped diagonally across a wall's corner, which the distance
  map never does and `walk` refuses, so a soldier at a wall's end stood
  still. Same repro, seed 1: soldier 2 froze 17 cells short of a move order
  round the wall for the whole 20 s.

Fixes: builders stand beside the line on their own side, half the wall's
width and a cell clear, spread along it by their slots; a soldier inside
rock or a wall walks straight out to the nearest standable cell; a goal in
rock or a wall becomes the nearest standable cell within 8 dish cells
(`Game::nearest_standable`, `GOAL_SEARCH`), where before the soldier froze;
`downhill` takes a diagonal only past two open side cells. After: on seeds
1-5 every builder stands on open ground when the wall rises and every
soldier is within 12 cells of the far-side target after 20 s. Tests:
`builders_stand_beside_their_wall_and_walk_on_once_it_stands`,
`a_soldier_shut_in_a_wall_walks_out_and_a_move_into_one_stops_beside_it`.

Frame cost: the ring search runs only for a soldier whose goal is in rock or
a wall, or who stands in one (none in normal play); the corner check is two
reads per diagonal. Not timed.

## 3. Upkeep

Every bank drain in `mould::larder_world` was checked. The one running
cost was the lure: `BuildRules::lure_burn` 2 food a second while on. It is 0
now, so a lure costs its price once (the build card says "Free to keep on").
Rations (`Rules::ration`) were already 0 since 11-14 01:49Z. What is left
is paid per use, not per second: soldiers, walls and buildings when placed,
a toolkit tool or a gun card's cast when used. The mould's steal at the
larder and outposts is the enemy's doing, and the own-mould gardeners'
`feed` turns food into the player's mould when told to. Test:
`standing_buildings_cost_nothing_to_keep`.

No census: these are bug fixes (CLAUDE.md, build first). The lure and kit
defaults move census numbers that use lures or tools.
