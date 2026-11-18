# The slime's off switches, settled (2026-11-17)

The Enemy lane's first job under the plan the owner approved on 11-17
([plan](../continuation-plan-2026-11-17.md), [brief](../../handoff/enemy.md)):
fourteen slime features were built and off; each is now on, one tier's
trait, or deleted. What became of each is in
[ROADMAP](../../ROADMAP.md), *Switches that are off*. This folder holds the
films behind the choices and the page made for the owner.

Page: https://claude.ai/artifact/1qg9m9ktNWHCCLL5Li74JQ (source `page/`; open `index.html`). Films: `film.sh`;
clips from their frames: `make_clips.py`.

## What was chosen, and why

- **The Trunk (planet 3) gets the setting configuration**: crust, its pull
  at +8 with fast ground, setting, the wall (`set_block` 5) and `fire_fed`.
  The brief says "one deeper tier". Planet 3 was picked over a tier past the
  milestone because Milestone 1 is planets 1 to 3 and its exit test is that
  the owner can say what each of the three did to him; the Trunk's card was
  the Drifter's (both smelt your pumps), so planets 1 and 3 did the same
  thing; and every film of setting was made on planet 3.
- **Wall, not trap.** The owner's ruling (11-17 01:13Z) is that bulwarks
  block "Both" shots and soldiers. The walking maps route round a wall and
  walked straight through a trap. `set_hold` is deleted
  ([dead end](../dead-ends.md)).
- **The pull stays at +8.** The Trunk idle to 3:00 with the pull and without
  (`crust_weight=0 crust_fast=0`): 37,758 walkers against 33,784, 983 food
  eaten against 988, the same shape on the map. No harm seen, and keeping to
  held ground is the Trunk's character, so it stays; -8 was the Sprawler's
  shape ([E1b](../sim-use-2026-11-17/e1b-c2-crust-effects.md)).
- **`fire_fed` on for the Trunk**: the owner agreed (11-17 01:13Z) to the
  plan's rule that fed slime is fireproof and starving slime burns. Fire is
  then the answer to the walls only, which the fire film below checks.
- **The last stand is the Swarmer's** (planet 4): it is the charging slime's
  way to lose. Not filmed; see below.
- **The reserve recruit was not deleted**, against the brief. The run never
  turns `Rules::reserve` on: it is the frozen campaign's mission rule
  (`campaign::RESERVE`, the owner's 11-12 04:44Z no-soft-lock pick), so
  deleting it would change the campaign. The switch list
  (`src/build/switches.rs`) lists it as "Free recruits", Squad and
  arsenal's row.
- **Lossy flow and the lone step went from the GPU game only.** The owner's
  19:21Z answer leaves `src/mould/big.rs` exactly as it is until Milestone 2.

## Films (run seed 1, llvmpipe, one seed each)

The scripted rush (`mode=squads hand=greedy`), before (main `d08583d`) and
after, at 2:00 unless said:

| Planet | Soldiers | Health | Killed | Slime |
|---|---|---|---|---|
| 1 before | 42 | 97 | 1,318 | 10,523 |
| 1 after | 42 | 90 | 3,283 | 9,603 |
| 2 before | 18 | 63 | 10,717 | 11,903 |
| 2 after | 7 | 47 | 16,157 | 13,034 |
| 3 before, 1:00 | 15 | 43 | 7,992 | 36,084 |
| 3 after, 1:00 | 11 | 88 | 3,205 | 40,749 (5,327 set, 4,800 wall cells) |
| 3 before | 1 | 97 | 12,461 | 16,866 |
| 3 after | 0 | | 4,652 | 18,984 (9,831 set) |

On planet 3 the first raid (1:12; 8,736 walkers before, 10,356 after) ends
the squad both times. Planet 1 still does nothing to the rush, which is the
lane's next job.

Eight rifles holding a front on planet 3 (`hand=hold@344,169 gun=rifle`):

| Time | Shots | Killed | Walkers set | Wall cells |
|---|---|---|---|---|
| 0:20 | 41 | 130 | 0 | 0 |
| 0:30 | 80 | 212 | 24 | 20 |
| 0:45 | 140 | 367 | 210 | 206 |
| 1:00 | 233 | 973 | 460 | 453 |
| 1:15 | 332 | 2,058 | 1,143 | 1,130 |

**Fire on the walls.** The same game, two Torch shots at 1:16, one on each
set patch (`boom=76,374,116,torch boom=76,317,121,torch`), against no fire:

| Time | Cells burning | Wall cells, lit | Wall cells, no fire | Walkers set, lit |
|---|---|---|---|---|
| 1:16 | 0 | 1,212 | 1,212 | 1,225 |
| 1:18 | 740 | 679 | 1,284 | 1,332 |
| 1:20 | 815 | 692 | 1,350 | 1,432 |
| 1:24 | 1,008 | 743 | 1,485 | 1,631 |
| 1:30 | 1,239 | 812 | 1,563 | 1,871 |
| 1:40 | 1,313 | 884 | | 2,169 |

A plain light on the same two patches (`light=76,187,58,4
light=76,158.5,60.5,4`) burnt 178 cells at once and 1,165 at 1:40, and the
wall stood at 906 cells then. Planet 3 is the fen (moisture 0.5), so the
crust under the slime is the fuel. The fed body beside the patches did not
catch in either film.

`light=` takes dish cells, half the world units the log's `squad at` prints
on a run planet. Three films lit at world units first, missed the map and
burnt nothing; they are the "no fire" column.

## Not checked

- No GTX 1070 timing. The per-step costs are the simulation-use reports'
  ([S1](../sim-use-2026-11-17/s1-unit-paths.md),
  [S2](../sim-use-2026-11-17/s2-far-smell.md),
  [E2b](../sim-use-2026-11-17/e2b-bulwark.md)); the 1070 run goes in the
  Milestone 1 batch.
- The last stand has no film: no scripted hand cuts a Swarmer to a quarter
  of its peak in a short film.
- One seed everywhere. Planet 2's 18 soldiers to 7 is not a finding.
- The shop before planet 3 does not offer a Torch (`run::bring` names a card
  for the Sprawler only). That is Squad and arsenal's to decide.
