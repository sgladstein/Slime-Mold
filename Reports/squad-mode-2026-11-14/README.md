# Larder World squad mode (2026-11-14)

The owner, 04:08Z: "revamp potentially the whole feel of the game. the unit
control should be squad based from the ground up. All soliders in a squad
stick together at least as well as they can. We need a really good and
intuitive system to split and combine squads and to switch control between
them and when you are controlling a specific squad you mouse is the aim for
the guns and they alll fire in unison at the mouse. maybe whoever you are
actively controlling is stronger (and that should be visible)." 04:49Z:
"This should be built as a seperate mode for right now. Not erase the other
game."

Page (films with counts): https://claude.ai/artifact/JVSTNATVeyXGivTGoT9HU7,
source `page/`.

## The owner's answers (04:18-04:28Z)

1. Steering: both, WASD steers and right click sends ("c").
2. Hold the left button, and all the soldiers fire at the cursor on one beat ("a").
3. The squads you aren't controlling keep their last order and fire on their own ("a").
4. Splitting and merging, as proposed ("Yes this sound perfect"): 1-9 or the wheel switch, X splits, a box pulls soldiers out, G merges, and there's no size cap.
5. The controlled squad fires faster and hits harder, and you can see it in the bullets and the holes ("a").
6. The default is a tight blob facing the cursor, and other formations can be picked.
7. A bought soldier joins the controlled squad.
8. Get rid of the larder. Each crystal is drained or guarded (the card, 04:27Z), and "guarding has to be more valuable in the long run" (04:28Z). The owner wants more complexity than a race to grab crystals.
9. The slime's goal is to hold as many crystals as it can, so it grows. You lose when every soldier is dead.
10-12. A bought soldier appears beside the squad. WASD is freed. The camera follows the controlled squad, you can pan away freely, and Space brings it back.

## What was built

Everything is behind `Rules::squads` and `Rules::larder` (off and on by
default), so the RTS round, the AI, the census and the missions play as
before. All 51 `larder_world` tests pass, the 9 old ones unchanged.

- **Squads as game state** (`src/mould/larder_world/squads.rs`):
  `Soldier::squad`, `Game::squads` (16 slots), `Game::active`. Every action
  is a `Cmd` with a replay line: `Steer`, `SquadOrder`, `Fire`, `Face`,
  `Split`, `Join`, `Merge`, `Active`, `Formation`.
- **Sticking together.** A steered or sent squad has an anchor, and each
  member walks to its place in the formation round it. The anchor moves at
  0.75 of a soldier's speed, times the wade of its slowest member, and it
  waits once it is more than 4 cells ahead of the squad's middle. A member
  more than 14 cells from the middle stops counting. Formations: blob, line,
  wedge and ring. Tests: a steered blob of eight stays under 7 cells wide; a
  squad with one member held to a fifth of its speed keeps it within 12
  cells of the middle.
- **The volley.** A squad with a fire point fires every member at it on the
  same tick and fires at nothing else. The controlled squad fires 2x as
  often with a 1.4x strike (`LeadRules`). Test: 80 shots against 48 over
  6 s; the first wait comes on the old beat. When it isn't volleying, the
  controlled squad only fires in self-defence.
- **Drain or guard** (`Rules::larder` off). There is no larder unit, so
  nothing smells it or steals from it. A harvest banks the charge as it is
  taken. A crystal with a soldier by it (not draining it) and at most
  `Rules::guard_walkers` (8) walkers on it pays its charge left /
  `Rules::charge` / `Rules::guard` (120 s) each game second. Test: guarding
  pays to within 1% of that rate and nothing is carried.
- **The window** (`src/bin/larder-world/squadmode.rs`): the controls in the
  module comment and on the key card, `mode=squads`, and "Control: squads"
  on the start menu's options page (missions always play the RTS round).
  Each squad's number floats over its middle, and guarded crystals get a
  green ring. In this mode the screen edge doesn't pan, because the cursor
  is the aim.
- **The film hand** `hand=squads` (look mode): buy eight, walk to a slimed
  crystal, volley, split to guard, then go back to the fighting half.

## Rejected on the way

- **The anchor always 4 cells ahead of the middle.** Every member ran at
  full speed, so the ones that started behind never caught up. An 8-soldier
  blob moved 10-12 cells wide instead of under 7.
- **Formation places on a solid crystal's middle.** The members stood round
  its rim on every side, 10.8 cells wide. Anchors and destinations now move
  past the near rim.
- **Formation places inside rock or past a wall.** On the Fork a squad
  steered into the V wall stood still: its anchor waited for members whose
  places they could not stand on. A blocked place now falls back to the
  anchor itself.
- **Guarding paid on the starting charge.** A crystal drained dry keeps
  crumbs, so it went on paying the full rate forever. The rate now follows
  the charge left.
- **Guarding only with no walker on the crystal.** On shape C a thin film
  covers nearly everything, so no crystal ever paid. The threshold is now
  8 walkers.

## Films (Fork, seed 1, llvmpipe)

`larder-world mode=squads look=DIR hand=squads at=1,1.5,...,65.5` and
`at=14,14.067,...,21.93`. The GIFs were made with ffmpeg from the near frames.

| t | shots | killed | bank | mould |
|---|---|---|---|---|
| 1 s | 0 | 0 | 430 | 25,000 |
| 16 s (volley) | 104 | 1,441 | 445 | 23,689 |
| 25 s (split) | 240 | 2,477 | 445 | 22,961 |
| 37 s (control back) | 316 | 2,778 | 460 | 23,626 |
| 66 s | 584 | 4,680 | 482 | 27,200 |

## The squad panel (owner 08:22Z)

"The squad menu needs to be totally fixed. It needs to list each squad with
the number of soliders and its hotkey, you can click to take command. There
should also be buttons, to combine, to split."

Then (08:48Z): "This is supposed to replace the bottom middle menu each
squad should be a tile how each solider is now". The first cut, a panel at
the top right, was replaced before it merged. In squad mode the bottom bar's
middle panel (laid out by #250) shows the squads (`ui::SquadPanel`, drawn by
`Hud::squad_tiles`, built by `App::squad_panel` in
`src/bin/larder-world/squadmode.rs`):

- A tile a live squad, in id order; its key is its id + 1 and stays while
  the squad lives (the digit keys and the numbers over the squads agree; they
  first followed the live squads' order, so a squad's key changed when a
  lower one died, merged or split: the bug sweep's review). The size of a soldier's card but wider: hotkey, soldiers,
  their mean health. The squad you command is outlined. Hovering says its
  state (formation if not blob; firing, moving or holding).
- A click on a tile takes command of it. The + on another tile brings that
  squad into yours (`Cmd::Merge`).
- The header line: how many squads, which you command, and Split and
  Combine at its right end. Split: the front half, the way the squad faces,
  becomes a new squad and you command it. Combine: the nearest other squad
  joins yours, however far (the key keeps its 24-cell reach). A button that
  can't act (a squad of one, 16 squads, no other squad) is greyed with the
  reason as its tooltip.
- No Outpost on the build page without the larder: with nothing carried
  home it only smelt of the bank and let the mould eat from it.
- Cost: model and HUD 0.062 ms a frame on llvmpipe (Fork, 1600x900, two
  squads).

Stills `page/squad-tiles-24s.jpg` and `page/squad-tiles-crop.png`:
`larder-world mode=squads look=DIR hand=squads at=24 screen=1600x900`. On
main 68870d0 the film's squad stuck at the inside of the Fork's V wall from
14 s (the bug sweep's stuck anchor, #252), so the hand now splits by 20 s
whatever held it.

## Open

- A call-in delay for bought soldiers (answer 10).
- Keys, after the owner's playtest (08:01Z; [bug sweep](../bug-sweep-2026-11-14.md)):
  the card's letters came second to the squad's, so G merged instead of
  harvesting and X split instead of defending, and the wheel switched
  squads instead of zooming. Now the card's keys come first, split is
  **Y**, join is **J**, the wheel zooms (about the squad while the camera
  follows it) and **Ctrl + wheel** switches; the card's WASD letters moved
  to R (attack-move, mine), F (gas) and U (sow). Answer 4's X and G are a
  two-line change back (`squadmode::SPLIT`, `MERGE`) if the owner prefers
  them, with the card's Defend and Drain moved instead.
- Retune the numbers after a playtest: the rate and strike boost, the guard
  rate, the pace, the merge reach.
- The player AI doesn't play this mode yet.
