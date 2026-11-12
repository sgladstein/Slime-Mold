# Soldier controls for the Larder world, part 1 (2026-11-12)

The owner asked (2026-11-12 04:22Z) for better soldier controls and
commands in `larder-world`: "How I order them to harvest. How I order them
to build. Etc". The scheme was proposed in the thread at 04:30Z; the owner
picked "soldiers build over time" on the card at 04:41Z (not built yet: part
2, after the buildings thread's shared `Building` type). Then (04:42Z): "I
want to be able to zoom in more too. That was limited in my play test".

## What part 1 adds

Rules (`mould::larder_world`):
- **An order queue.** `Cmd::Queue(k, order)` puts an order after soldier
  `k`'s current one (`Soldier::queue`, up to `QUEUE` 8); a holding soldier
  starts it at once; a plain `Cmd::Order` drops the queue. A move or an
  attack-move that has arrived starts the next; a harvest order goes on
  once its crystal runs dry and the load is banked. With nothing queued a
  move keeps its order as before (the player AI reads it).
- **Attack-move**, `Order::Fight(x, y)` (in the shared `larder::Order`;
  M3 plays it as a move): walk there on free fire, standing to fight while
  mould it can hit is in sight (`Game::engaged`).
- **Moving on from a dry crystal**, `Rules::seek`: after banking (or at
  once, empty-handed) the soldier takes the nearest crystal with a frame's
  harvest left (`Game::seek`), one the mould has not eaten from first, never
  the mould's home.
- **The larder's rally point**, `Cmd::Rally(order)` / `Game::rally`: a
  bought soldier starts with it.
- Replays: `queue k ...`, `rally ...` and `order k fight x y` lines; the
  round trip is tested.

Window (`larder-world`) and controls (`world::controls`):
- Shift + any order (right click, G, A, F) queues it; the selection's queue
  is drawn as a chain of lines, red for fighting.
- A is now a targeted command: click a point to attack-move, click the core
  to attack it as before.
- Click the larder to select it: its card shows the bank and where new
  soldiers go; right click (or R, a new command) sets the rally point, on a
  crystal they start harvesting it.
- Double click a soldier: every local soldier on screen.
- `MAX_ZOOM` 8 → 24 pixels a cell (steps 12, 16, 24 added).

Cost: nothing new runs on the GPU; the rules add a queue check and, on a
dry crystal, one pass over the crystals. Not timed.

## The census: moving on, on and off

The Larder thread asked that `Rules::seek` either be a player's choice or
pass the gate (`Rules::home_dry` was measured under score v2). The deciding
measure, named before the run: the gate (farm and wall beat idle and random,
sign test, Holm), then paired farm and wall scores, seek on against off.

`larderworld census seeds=5-20 cap=900 hands=idle,random,farm,rush,wall`
with `seek=1` and `seek=0` (raw: `soldier-controls-2026-11-12/`):

| hand | mean score, on | mean score, off | on vs off, paired |
| --- | --- | --- | --- |
| farm | +0.141 | +0.048 | 8 higher, 7 lower, p 1.0 |
| wall | +0.120 | +0.152 | 2 higher, 14 lower, p 0.004 |

The gate holds both ways (farm and wall beat idle 14-15/16 and random 16/16,
all hold under Holm). Wall's loss survives a Bonferroni correction over the
two comparisons; the mould ends bigger on most of its seeds with seek on
(inferred: its farmers walk on past its wall instead of holding behind it).
That first version was not kept. Two reviews (the game designer and the
technical lead, 05:25Z) objected to the split it led to (on in the window,
off for the hands): the census would no longer measure the game played.

### The safe version, one default

`Game::seek` now picks only a safe crystal: a frame's harvest left, not the
mould's home, none the mould has eaten from, none nearer the core than the
larder; of those the nearest the larder that no other soldier works, else
the nearest the larder; none safe sends the soldier home to hold. Same
census with it on (`census-seek-safe.txt`), against off:

| hand | mean score, safe on | mean score, off | on vs off, paired |
| --- | --- | --- | --- |
| farm | +0.217 | +0.048 | 10 higher, 5 lower, p 0.30 |
| wall | +0.052 | +0.152 | 4 higher, 11 lower, p 0.12 |

The gate holds (farm 16/16 over idle and random, wall 15/16, all hold
under Holm). Neither difference is significant, so under "defaults on" it
is **on** for every hand and the window alike. Wall still leans lower (its
mean drops 0.10; it lost the squad on 2 seeds, against 1): re-test it when
soldiers raise the wall (part 2) or the hands plan with moving on.

## Next

Part 2: soldiers raise buildings over time (`Order::Build`, the buildings
thread's `Building`/`Game::raise`), walls included; harvesters carry to the
nearest depot once outposts exist. Not built from the proposal: right click
on mould as attack-move (needs a mould read under the cursor on the GPU
world) and a line formation for group moves.
