# Raids march as one body; specks are tethered (2026-11-18)

The owner's play notes, 2026-11-18 15:56Z to 15:58Z: "Often, especially
during raids the slime feels more like a bunch of individual walkers
instead of a single slime unit"; "This should be explore more generally,
then just during raids but that is when the issue is biggest"; "Seeing the
individual walkers doesn't bother me when I am attacking a big slime blob
and the walkers surge towards me, but when they all leave the blob and most
of what I see is individual walkers that is problematic." Direction
narrowed it (16:24Z, 16:53Z): keep the slime together when it moves away
from its body (a raid, a breakaway, a walk to a new crystal), leave the
surge at the squad as it is; the Lab's tether only for roaming and
breakaway walkers, since any tether of 2 or more kept the Lab's raid at
home ([one body, lab](../lab-body-2026-11-18/README.md)); the march is
this lane's.

Page: https://claude.ai/artifact/KATAdrfNoVexvNjYsAX4My (source `page/`; open `index.html`).
Films: `film.sh`; clips: `make_clips.py`.

## What changed

- **The march** (`RaidRules::march`, on; `raids.rs`): a raid takes a free
  lump (`big::Lump`, shared with the mini-bosses, with none of their look
  or strength) for `march_size` 1.5 times its drawn walkers, the ones
  nearest its target, and steers it down the raid's walking distance round
  walls (`down_way`: a point a body's length and ten dish cells ahead;
  formed from the walkers nearest along that way, not as the crow flies) at
  `march_pull` 100, `march_leash` 1.3, `march_haste` 2. By the target, or
  where a soldier is within gun range and the panic margin of its edge, it
  lets go and pours on as a raid of share 1 of every walker there (the
  surge he does not mind). One that is not ten dish cells nearer after
  `march_stall` 15 s, or still walking after 90 s, lets go and streams to
  its target with the raid's own reach and share, so a raid always lands;
  under 30% of its members (shot down) it only lets go. With no lump free,
  or for the last stand, a raid streams as before. A mini-boss never
  forms on a march (`form_lump` takes other lumps' members).
- **The tether** (`Settings::tether`, `run::TETHER` 2 on every run planet;
  engine.wgsl's `tethered`, GPU only): a scared walker on its own way out
  (a speck, a cut-off walker making for food), not raiding, not charging a
  unit and not a lump's member, steps only into a cell whose 8 neighbours,
  its own aside, hold 2 walkers; else it is blocked there as at a wall.
- `larder-world look=` prints `march t ...` each second (members, centre,
  spread: the members' mean distance from the centre over a packed disc's,
  1 packed) and `loose:` each frame: walkers in dish cells holding one or
  two, and the Lab's count, walkers with at most one other in their 8
  neighbouring fine cells (the deciding measure here, named before the
  films).

## The films

Planet 1 of run seed 1, the pumps hand, the first raid forced at 0:20
(`set=raid_first=20 set=raid_first_sd=0 set=raid_early=0`) so the films
stay short; a frame every half second to 0:56, one film each, software
GPU. The raid goes for the far pump (374, 25); the squad stands by the
near pump (290, 50). The march takes 1,789 walkers (the stream sent 60%
of the 1,866 within its reach). "Loose" is the Lab's count over the whole
map, so it also holds the slime by the crystal near the squad surging at
the soldiers from 0:26 in both films.

| Time | Stream: loose | squad health | far pump | near pump | killed | March: loose | squad health | far pump | near pump | killed |
|---|---|---|---|---|---|---|---|---|---|---|
| 0:20 | 214 | 100 | 200 | 20 (building) | 0 | 214 | 100 | 200 | 20 | 0 |
| 0:24 | 666 | 100 | 200 | 200 | 0 | 404 | 100 | 200 | 200 | 0 |
| 0:28 | 788 | 100 | 199 | 190 | 50 | 561 | 100 | 200 | 194 | 70 |
| 0:32 | 748 | 98 | 92 | 170 | 189 | 568 | 100 | 200 | 182 | 227 |
| 0:36 | 533 | 97 | 0 | 152 | 350 | 462 | 100 | 200 | 166 | 410 |
| 0:40 | 568 | 95 | 0 | 132 | 546 | 531 | 99 | 200 | 148 | 623 |
| 0:44 | 512 | 76 | 0 | 98 | 1,035 | 560 | 99 | 200 | 136 | 790 |
| 0:48 | 460 | 68 | 0 | 85 | 1,263 | 693 | 98 | 200 | 120 | 876 |
| 0:52 | 493 | 65 | 0 | 70 | 1,442 | 578 | 93 | 200 | 31 | 1,346 |
| 0:56 | 520 | 59 | 0 | 58 | 1,636 | 609 | 85 | 200 | 0 | 1,865 |

From 0:22 to 0:38 the loose count averages 485 a frame with the march
against 635 with the stream (peaks 570 and 801). The march forms inside
the ring of rock round the home crystal, packs (spread 1.5 to 1.9), sinks
a little into the body for 5 s, leaves the ring at about 0:33 and walks
out past the crystal by the squad as one long body, meets the soldiers at
0:45 and pours onto them as a broad front; the loose count then rises to
about 600 to 700, the surge. In the stream the walkers cross the map one
by one to the far pump, which goes by 0:34. The march took the near pump
by 0:54, and the squad stood at 85 at 0:56 against 59.

## Picking the march

Each one film, same game, the raid forced at 0:20; size 1 unless named.
These ran before the march started from the walkers nearest along its way
and before the stall rule, so the landed row differs a little from the
film above (there: 1,789 members, 111 cells to go at 0:39).

| Setting | Sets off (its way 5 cells shorter) | Spread by the squad | Note |
|---|---|---|---|
| leash 1.3, pull 30 | 0:26 | 2.3 (83 cells to go) | the first build |
| leash 1.15, pull 40 | 0:30 | 1.9 | |
| leash 1.0, pull 30 or 40 | 0:35 to 0:37 | 1.45 to 1.5 | rounder, but waits 9 to 11 s at the edge |
| leash 1.0, pull 20 | 0:36 | 1.5 | |
| size 1.5 or 2, pull 30 | never | 0.65 | formed inside the ring and sank into the body (its way 122 cells at the start, 156 a minute later) |
| size 1.5, pull 60 | 0:39 | 1.4 to 1.9 | 99 cells to go at 0:45 |
| size 1.5, pull 100 (landed) | 0:33 | 1.65 to 1.9 | 95 cells to go at 0:39 |

Raid size, to 1:08 to 1:20 (Direction: a raid should still hit as hard):

| | Squad health at 1:08 | Pumps lost by 1:08 | Killed by 1:08 |
|---|---|---|---|
| Stream | 53 | the far one at 0:34; the near one at 36 | 2,001 |
| March, size 1, pull 30 | 75 | the near one at 0:50 | 2,243 |
| March, size 1, pull 100 | 83 | the near one at 0:50 | 1,710 |
| March, size 1.5, pull 100 | 93 (80 at 1:20) | the near one at 0:50, the third (building) by 1:08 | 2,947 |
| March, size 2, pull 100 | 80 (68 at 1:20) | the near one by 1:02, the third by 1:14 | 2,638 |

The march hits pumps as hard as the stream and the squad less, since the
squad shoots one body coming at it; 1.5 is the pick, with Direction to
retune.

## The tether

Before the raid (0:02 to 0:20, a frame every quarter second, same game
but the tether), walkers with at most one other round them:

| Tether | 0:04 | 0:08 | 0:12 | 0:16 | 0:20 | Mean | Peak |
|---|---|---|---|---|---|---|---|
| off (`tether=0`) | 309 | 430 | 445 | 348 | 377 | 381 | 450 |
| 2 (landed) | 307 | 286 | 313 | 226 | 214 | 284 | 319 |

The specks leave the home ring for the crystal north of it as a column
either way; with the tether fewer single walkers spray off its sides. The
earlier pick, with the greedy hand (frames at 0:20 to 0:40): mean 257
with no tether, 166 at 2, 179 at 3, so 2 (Direction asked for 2 or
tighter; the Lab found tighter adds nothing). With the tether off the
march formed deeper inside the ring and stood (its way 120 cells at 0:21,
131 at 0:37), which is what `march_stall` is for: it now streams after
15 s.

## Frame cost

- The tether: 8 more cell reads for a walker it applies to, only when it
  steps into a new cell; nothing for the rest.
- The march: CPU work once a game second per march (the lump sums the
  mini-bosses already read back, and `down_way`, at most about 30 blocks),
  and the mould grid once when it forms, as the raid's reach already reads.
- Not timed: the software GPU is not for timing. Engine and tools times it
  on the GTX 1070 in the milestone's batch.

## Not checked

Other planets and seeds; a raid that is not forced; several marches at
once; how often a march stalls and streams; the GTX 1070's time. One film
each. A march formed inside the home ring first sinks a few cells into
the body before it finds the gap (its way 122 cells at the start, 143 at
0:24, 125 at 0:33).
