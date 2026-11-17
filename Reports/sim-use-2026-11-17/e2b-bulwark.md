# E2b: set slime blocks soldiers (2026-11-17)

The simulation-use plan's E2b ([plan.md](plan.md), draft 3, "E2. Setting",
Blocking): the owner ruled at 01:13Z that set slime blocks "Both", shots
and soldiers. E2a ([report](e2a-setting.md)) gave the shots' half (set
walkers soak blows); this is the soldiers' half. Page:
https://claude.ai/artifact/AKuE9RVmPtcNqsK1nW93Dx (source `page-e2b/`).

## What was built

- **The count.** `BigDish::set_cells`: the set walkers a fine cell, counted
  at the end of every tick while setting runs (`count_set`, after the
  plants' deaths). On the GPU two passes, `setclear` and `setcount`, after
  the plants' passes of a game tick's last step (`Engine::set_passes`), into
  a block after the set words in `news`; `Engine::set_cells_of` reads it.
  The look will read the same block (next).
- **The bulwark.** `Game::bulwark`, a fine-cell mask beside the doors' mask
  (`larder_world/bulwark.rs`). A cell is in it when its 3 x 3 holds
  `Rules::set_block` set walkers or more, and stays in it until the count
  falls under half of that, so the edge doesn't flicker. A fine cell holds
  at most one walker, so a lone set walker is never a wall. Refreshed once a
  game second (`World::set_cells`; the GPU world reads the block back then).
- **Feet and maps.** `Game::ground` (open or a door, and not bulwark) reads
  it at once: the walk, the steer and the squad's anchor share that check,
  so a soldier won't head into or stand in a bulwark cell, and one caught
  inside as it forms walks out the old way (`walk`'s "shut"; the
  squad-movement thread confirmed it is the one hook its steer needs). The
  walking maps (`standable_cells`) read it when they are built, and are made
  stale for it at most every 5 s (`BULWARK_STALE`) and only when 16 cells or
  more changed since (`BULWARK_MOVED`). The levelgen thread (11-17): every
  map is a whole-world Dijkstra that takes 6-7 steps at 6x, so staling them
  every second would keep them stale in every fight and halve how fast
  orders answer; at most every 5 s on a real change was its stated limit.
- **Point check alone, tried first and dropped.** With the maps blind to set
  slime, a soldier ordered past a band of it walked its map into the band
  and stood there (the technical lead's review: the stuck-squad bug of
  #252). The test below fails that way (planted: the soldier never got
  past).
- `set=set_block=N` turns it on (off at 0, the default). It needs setting
  (`setting=1`).

## Tests

| Test | Result |
|---|---|
| `setting_matches_the_cpu` (llvmpipe), now also the set count a cell | 0 of the dish's cells off; planted GPU that never sets: 946 cells off (caught) |
| `set_slime_is_no_ground_to_soldiers_and_holds_to_half` (CPU world, `set_block` 5) | nine set walkers in a 3 x 3: not ground, not standable, ground two cells past; the walking maps' cells see it; three hold it, two let it go, four don't raise it |
| `a_soldier_ordered_past_set_slime_goes_round_it` (the technical lead's ask: a 3 x 21-cell band across a soldier's straight 24-cell way) | round it in 3.1 s, never standing in it; planted, maps blind to it: never got past in 30 s (caught) |

## Films (planet 3, llvmpipe, `gun=rifle crust=1 crustlook=1 setting=1`)

One look each, `set=set_block=5` against 0. Raw lines in `e2b-raw/`.

**A squad that rushes in** (`hand=push`: eight rifles attack from 3 s,
walking into the body; the owner's ruling is that thick slime is very
deadly):

| Seed | Block | Wiped at | Killed | Bulwark cells at 20 s |
|---|---|---|---|---|
| 1 | off | 19 s | 806 | - |
| 1 | 5, feet only (dropped) | 24 s | 1,172 | 2,496 |
| 1 | 5, feet and maps | 28 s | 1,365 | 2,510 |
| 4 | off | 30 s | 670 | - |
| 4 | 5, feet only (dropped) | 37 s | 1,205 | 1,676 |
| 4 | 5, feet and maps | 39 s | 776 | 1,676 |

With the bulwark the rushing squad is held at the set rim and shoots from
there instead of walking into the thick body, so it lasts 8-9 s longer. On
seed 1 that rim is the way in and it killed 69% more. On seed 4 the maps
took it round the set rim, 66 cells south, into unset body elsewhere: it
lasted as long and killed about as many as without (776 against 670).
So, as built, blocking soldiers holds a reckless push out of the deadly
body (shelter for the player) or sends it round. Whether the shelter is
wanted is a playtest question; the plan's intent was ground denied (a nest
door, a crystal ring), which the next films should show.

**Block or trap** (the game designer's review of #413, 11-17: blocking
shelters a reckless rush, against the owner's ruling that walking into
thick slime is very deadly; set slime should hold soldiers, not stop them).
`set=set_hold=F` (over 0): bulwark cells stay ground and in the walking
maps, and a soldier in one walks at F of its speed (`Game::wade`) while the
slime keeps biting. The same rush at `set_hold=0.05`:

| Seed | Set slime | Wiped at | Killed |
|---|---|---|---|
| 1 | off | 19 s | 806 |
| 1 | blocks | 28 s | 1,365 |
| 1 | traps | 20 s | 840 |
| 4 | off | 30 s | 670 |
| 4 | blocks | 39 s | 776 |
| 4 | traps | 30 s | 750 |

Trapping keeps a rush as deadly as without setting (seed 1 at 20 s: one
soldier left in it, against five held in a line at the rim when blocked).
The owner picks (card in the thread); both stay behind their keys until
then. Test: `set_slime_that_holds_slows_a_soldier_in_it` (still ground,
the maps unchanged, a twentieth of the speed).

**A squad that stands** (`hand=stand`, four rifles): the bulwark formed
(209 cells on seed 1 at 45 s, 927-1,080 on seed 4) but a squad that doesn't
walk never meets it. Seed 1 played exactly as without; seed 4 nearly so
(killed by 75 s 1,145 against 1,079; the squad's few steps at 45-75 s met
it). Feet only and feet with maps played the same here.

## Frame cost

Off (the default, `set_block` 0 or setting off): nothing; the count passes
run only while setting runs. With setting on: two passes a game tick (a
cell clear and one load a walker), on the CPU one pass over the walkers a
tick. With `set_block` on: the walking maps made stale at most every 5 s, and once a game second a blocking read-back of a
word a fine cell (655 KB on planet 3) and a 3 x 3 sum on the CPU. Not timed
on the 1070 yet; if the read-back costs, the GPU can pack the mask to a bit
a cell (20 KB) before it is read.

## Left

- **The look**: set walkers drawn by their own state from the same count,
  a material settled by blind A/B (the plan: translucent amber against
  raised and glossy; not dark ridges, not bronze). Next.
- **The nest-door check**: before setting goes in the run, no nest door
  (7-9 dish cells on planets 1-2) may be sealable by planet 1's rifles. Set
  slime isn't on by default anywhere, so it waits for that.
- **Map-wait measure** (both threads asked): `move_tests.rs` with
  `MOVE=planet8` and `soldiercap depth=8 n=200 fight=1`, block on and off.
  Those harnesses run without setting, so the bulwark is empty and nothing
  changes by construction; the measure needs setting on in them, and waits
  for setting going into the run.
