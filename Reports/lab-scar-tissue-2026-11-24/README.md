# Lab: scar tissue, a nameable thing for the Grudge (2026-11-24)

The Lab lane's seventh prototype, on the lab branch `claude/project-thread-2o4cic`
only (code `4c76a347`, on main of 11-24 after the origin), not in the game.
Brief from Direction 2026-11-24 04:37Z: ask Enemy which of planets 4 to 8 has
the weakest "one nameable thing", prototype a candidate from the model, about
two hours. Enemy (04:38Z): planet 7, the Grudge. Its card says "It remembers
where you hurt it, and comes back there"; that runs on the scar
(`scar_weight` 1), a steering bias that reads as "the slime comes at me",
which every slime does; nobody has measured it in play. Runner-up: planet 6,
the Sleeper, which is free to farm before it wakes.

Page: "Grudge Scar Tissue" (source in `page/`, listed in `Reports/pages.md`).

## From the model

The Grudge already keeps a memory of harm: the scar, a field a fine cell
that every alarm adds to (`scar_gain` 0.01 a step x the alarm) and that
fades very slowly (`scar_keep` 0.9999 a step). Its walkers read it in their sensors (`scar_weight`), and the view
draws slime on it as pale scar tissue and bare scarred ground as a bruise
(`SlimeLook::Scarred`). Nothing else reads it: a walker on deep scar dies to
a hit like any other.

Measured on planet 7, run seed 1 (squad holding at the edge of the colony at
709,386): the scar under the slime's walkers near the squad has a median of
12 at 0:20, 85 at 0:30, 513 at 1:00 and 3,500 at 1:30, and it is highest
exactly where the squad's shots land.

## The candidate: scar tissue

`LAB_TOUGH=P:S0`: a hit on a walker standing on scar s shrugs off with
chance P x min(s / S0, 1), in `armoured()` beside the set walker's and the
striker's armour. So where you hurt it, it grows back tough, and its old
pull (`scar_weight`) brings it back to that same ground. The card would
read: **"Where you hurt it, it grows back tough. Don't fight on old
ground."** The picture is already the data: the view's pale scar tissue is
the tough slime (Look would set `SCAR_FULL` to S0 so the paleness is the
toughness, not any scar at all).

Picked: S0 2,000, so the toughness builds over about a minute of fire on
one place; P 0.5 as a first number (0.75 measured too). Costs one scar read
and one hash a hit, nothing a step.

## Results

Planet 7, run seed 1, bank 3,000, lavapipe, one seed a scene.

**A squad holding one place** (8 soldiers at 770,400, firing freely into
the colony; P 0.75, S0 2,000; frames every 3 s, zoom 6):

| Time | Killed, off | Killed, scar tissue | Walkers within 60 cells, off | on |
|---|---|---|---|---|
| 0:16 | 1,380 | 434 | | |
| 0:20 | | | 949 | 2,135 |
| 0:34 | 5,538 | 1,463 | | |
| 0:40 | | | 368 | 1,924 |
| 0:49 | 8,230 | 2,248 | | |
| 0:57-0:59 | 8,837 (wiped 0:59) | 2,600 (wiped 0:57) | 1,165 | 3,301 |

Off, the squad cuts the colony's near side back to a bare gap by 0:40; with
scar tissue the body stays pressed against the squad, pale where it is
scarred. Both squads were overrun at about 0:58 (an earlier run of
the off arm, without the frames, lasted to 1:41 and the on arm to 1:19).

**The run bot** (`hand=greedy`, which moves, buys and pumps), at 2:00:

| Arm | Soldiers | Killed | Shots |
|---|---|---|---|
| off | 30 | 11,749 | 1,230 |
| P 0.5 | 21 | 8,795 | 1,966 |
| P 0.75 | 4 | 5,360 | 3,218 |

At 1:00 the three are level (21 soldiers each, 732 to 881 killed): the
toughness only bites once the bot has fought in one area for a while.
Moving does not wholly escape it, since the bot's fights at 1:00 to 2:00
are round the same crystals and the scar is wide (7,000 cells over 50 by
0:30).

## Recommendation

**Build it, changed, as the Grudge's thing** in place of the plain pull:
scar tissue at P 0.5 and S0 2,000, with the pull kept. It is a rule the
player can name and play against (fight on fresh ground, drain the crystal
and leave, burn or wait out the scar), and the view already draws it. Before
it lands:

- Enemy: P and S0 by play; one seed here. 0.75 took the bot from 30 soldiers
  to 4 by 2:00, so it can easily be too strong.
- Look: `SCAR_FULL` (1.0 today against scar in the hundreds and thousands,
  so every scar reads full) set to S0, so pale tissue means tough.
- A counter on the card (Direction): what ends a scar? Fire on scarred
  ground, or the scar fading faster where no slime stands, are untested.

## Not tried

The scar fading faster under bare ground; a hit on scar tissue that lays
less alarm (so it does not deepen its own scar); the Sleeper's runner-up
idea (it wakes when you pump, not only when you shoot).

## Raw

`raw/run7.sh` (the run: env, hand, frames, camera), `raw/m7.py` (walkers on
the scar and near the squad from the dump).
