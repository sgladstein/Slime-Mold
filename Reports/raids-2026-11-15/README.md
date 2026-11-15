# Raids, the lure kill box and the last stand (2026-11-15)

The owner (11-15 20:56Z): "I could build a huge defense, but that just ends
with the mold clustering around its own crystals and not doing anything.
even if i build lures, only a thin trickle moves into my kill box not
enough to even counteract the mold growing in size." Offered three fixes
(raids, the kill box pays, hold ground); picked 21:15Z: "Raids is the
number 1 option. Also consider that a slime that is losing might rush you
will everything it has. Not sure if this is a good idea or not. 2) the kill
box pays but only with an upgrade".

Page: https://claude.ai/artifact/8qKTwT5RmZbKeUQF4GZYG6 (source in `page/`,
open `index.html`).

## Why the turtle stalled (read from the code)

- The surge and the panic charge reach only units within `panic_r`, the
  gun's range + 5: a defense out of that reach was never attacked.
- A lure smells 8 times a soldier over only 6 cells (`BuildRules::lure_r`)
  and is eaten fast (150 health; in the film below it was worn down by
  0:45, before any raid).
- `pump_smell` is 0, so the economy draws nothing.
- Surplus food becomes thickening at home.

Outside games make static defense pay by making the enemy come to it:
Factorio's pollution reaches the nests and sets off attacks, They Are
Billions' noise draws the horde, tower defense's enemy must pass the
towers; Creeper World's turrets win ground (the third option, not built).

## What was built

- **Raids** (`raids.rs`, `Rules::raid`, on): a meter of the food the slime
  eats from its crystals (the surge meter's count). At 400 food, no sooner
  than 1:00, at least 40 s apart and at most 150 s apart, the game sends
  15% of the slime's mass at the pump drawing most (else the standing
  building with most health, else the thickest knot of soldiers). The
  reach is the smallest disc round the target whose mould-grid squares
  hold that mass / 0.6; 6 in 10 of those walkers go. Engine:
  `BigParams::raid_at, raid_to, raid_r, raid_share` (set by the game,
  `World::set_raid`) and the keys `raid_lean` 60, `raid_len` 450,
  `raid_haste` 1. A raider creeps at `panic_creep` through the lean, then
  runs for the target by its sensors' direction alone, as a panicking
  walker runs; it neither panics nor withers during the raid; afterwards
  it is an ordinary walker (bites, surges, panics if cut off). The draw is
  keyed on the raid's first step (`DRAW_RAID` 406), so the same walkers
  raid throughout. GPU: a new Params row (976-1007, `RAID_AT`), `raid_to`
  in engine.wgsl ahead of panic. HUD alert names the target.
- **Lures steer raids**: a lure that is on within `RaidRules::lure_r` (40
  cells) of the raid's target takes the raid.
- **Last stand** (`RaidRules::last_stand`, off, the owner unsure): once,
  when the slime's walkers fall under `last_stand_at` (0.25) of its peak,
  every walker rushes the thickest knot of soldiers. `set=last_stand=1`.
- **The kill box pays** (`BuildRules::render_cost` 600, `render_food`
  0.25): a turret's one upgrade (the turret card on a standing turret)
  makes it render; every walker any shot kills within its range pays 0.25
  food (`Game::render_pay`, `Game::rendered`, counted in `income`).

Keys: `set=raid_food=0 set=raid_every=0` (off), `raid_size`, `raid_share`,
`raid_lure_r`, `raid_first`, `raid_min`, `last_stand`, `last_stand_at`;
`turret2=X,Y` in the look mode places a rendering turret.

## Looks (the game window on llvmpipe; Fork, seed 1, squad mode, idle squad)

Turtle: pumps on the three landing crystals, five turrets, a lure at
256,175 (`raw/turtle-raids-on.txt`, `raw/turtle-raids-off.txt`):

| at 4:00 | raids off | raids on |
|---|---|---|
| raids | 0 | 4 (1:15, 1:57, 2:39, 3:21) |
| slime killed | 7,850 | 10,462 |
| pumps standing | 3 | 1 |
| soldiers | 4 | 2 |
| harvested | 5,834 | 3,524 |
| slime walkers | 37,872 | 38,014 |

The raids reach the defense and it kills a third more, but the slime still
grows as before: the turtle is now attacked, not yet winning. That is the
balance pass's question (raid size, render pay), not this build's.

Films (`page/*.gif`, frames a game second apart, `raw/*.txt` their counts):
`raid.gif` (first raid 1:16 on the top pump, reach 80 cells holding 8,084
walkers), `lure-killbox.gif` (lure at 256,62 with rendering turrets: the
raid turns for it at 1:17; 608 food rendered by 1:40 against 2,155 pumped),
`last-stand.gif` (`set=last_stand=1 set=last_stand_at=0.8` so it fires at
1:27 in a short film).

**Seen and not fixed:** raiders steer straight at the target, so the home
body piles up against the Fork's walls in the last stand rather than going
round. Steering raids round walls (a path or flow field) is the next step;
pathing (`paths.rs`) belongs to the levelgen thread.

## Tests

- `big::a_raid_runs_for_its_target_from_beyond_panic_reach`: off and zero
  reach change nothing; about half the walkers raid and close on a target
  far beyond `panic_r`; a smaller reach takes fewer; raiders don't panic
  while the rest of a stale body does; phases run once.
- `the_slime_raids_the_richest_pump_and_a_lure_takes_it`,
  `a_rendering_turret_pays_for_kills_in_its_range`.
- Squad twin (`mouldgpu`): new case "a raid on a crystal while a cut-off
  part charges" (143 raiders, 519 chargers): exact; a planted GPU bug
  (raid ignored) fails it.

## Commands

```
larder-world look=DIR map=fork mode=squads hand=idle seed=1 screen=960x600 at=45,75,90,105,120,150,180,240 pump=256,85 pump=235,125 pump=280,120 turret=240,148 turret=256,152 turret=272,148 turret=222,105 turret=292,100 lure=256,175 [set=raid_food=0 set=raid_every=0]
larder-world look=DIR map=fork mode=squads hand=idle seed=1 screen=960x600 at=70,..,102 pump=... turret2=240,62 turret2=272,62 turret2=256,45 turret2=222,105 turret2=292,100 lure=256,62
larder-world look=DIR ... (turtle) set=last_stand=1 set=last_stand_at=0.8
```
