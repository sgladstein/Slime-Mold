# Multicast and batched bursts: step B of the gun-card plan, part 2

2026-11-16. The rest of step B of the [gun-system review](../gun-system-review-2026-11-16/README.md):
"a multicast fires the next two or three groups as one volley", after the
read-back batching the plan put first ("`GpuWorld::burst` waits on a
read-back for every killing burst ... so chains multiply stalls; batch the
read-backs into the frame"). Follows the [trigger cards](../gun-triggers-2026-11-16/README.md)
and the [shove cards](../gun-shove-2026-11-16/README.md).

Page: [Multicast](https://claude.ai/artifact/RbbC4a8fjoKunEW4cRmuu1) (source in `page/`).

## What changed

- **Batched bursts** (`World::bursts`): a step's payload and shove bursts
  are queued (`Rounds::flush`) and run as one batch at the step's end. On
  the GPU world each burst's passes are followed by a copy of the engine's
  three counters (walkers struck, gassed, flung) into a small buffer
  (`Engine::snap`, no wait), and the batch reads that buffer back once
  (`Engine::read_snaps`). Each burst's counts are the differences between
  its snaps, so they are exactly what it did alone. Before, each killing
  burst read the kill counter back, each fling and gas its count too: a bomb
  was two stalls and a gas one. The CPU world runs the same bursts in the
  same order. Bursts in a step never read each other's counts, so batching
  moves no walker.
- **`Card::Multicast`** (kind "Next bullet", cost 3, tier 3): the shot that
  fires the cast it is in fires the next cast not yet taken beside it
  (`Cast::with`, `rounds::volley`), each its own bullets, fanned
  `rounds::VOLLEY_FAN` (0.08 rad) apart. A second multicast in the cast adds
  a third; links count toward `CARRY_DEPTH` (2) with the trigger's. The
  casts beside it leave the gun's own turn. A trigger in the same cast takes
  the cast after it first, so `multicast+trigger+plain+blast+gas` fires a
  plain bullet carrying a blast with a gas beside it. A carried cast that is
  let out fires its own volley too (`Game::release`).
- **What you see:** two bullets (three) leave the barrel in one shot; the
  gun-card strip lights every cast the next shot fires.
- **Counts:** `Game::multicast` (casts fired beside another),
  `Rounds::batches` and `Rounds::batched`; the headless rounds line prints
  them.
- **Test loadouts** (H): `multi-blast`, `multi-bomb-gas`, `triple`
  (multicast+multicast+plain+blast+gas), `multi-trigger`.

## Tests

- `gpu::larder_world::tests::a_batch_of_bursts_counts_each_as_alone_with_one_read_back`:
  a blast, a bomb's throw, a push, a gas, a burn and a second blast on the
  mould's home, as one batch on one GPU world and one at a time on its twin:
  the same counts burst by burst, the same fire's kills; 1 read-back for the
  batch against 5 one at a time; within 1% of the CPU (lavapipe).
- `rounds::tests::a_multicast_fires_the_next_cast_beside_its_own`: links,
  depth, a trigger and a multicast in one cast, the turn.
- `rounds::tests::a_multicast_shot_fires_its_casts_side_by_side` (seed 2,
  15 s): `multicast+plain+blast` fires 85 plain bullets and 85 blasts, each
  pair in the same step, against 43 and 42 in turn for `plain+blast`.

## Films

`hand=hold@150,265 bank=8000`, the Fork, squad mode, llvmpipe, near view
`centre=176,266,12`, 960 x 600, 30 frames a second (`raw/film.sh`,
`raw/make_clips.py`). Every clip is `rounds=plain load=29,NAME`: one game to
29 s, then each loadout; the same start as the trigger cards' clips. Counts
from 29 s (`raw/counts.json`); 24 soldier shots in each 3 s clip.

| 3 s, 29-32 s | Bullets | Payloads cast | Killed | Thrown | Slept | Health at 32 s |
|---|---|---|---|---|---|---|
| plain + blast | 24 | 8 blasts | 1,093 | | | 74 |
| multicast + plain + blast | 48 | 20 blasts | 1,720 | | | 81 |
| bomb + gas | 24 | 13 bombs, 8 gas | 176 | 2,859 | 2,443 | 84 |
| multicast + bomb + gas | 48 | 21 bombs, 21 gas | 100 | 3,338 | 2,183 | 84 |
| triple: 2 multicasts + plain + blast + gas | 72 | 13 blasts, 14 gas | 1,644 | | 3,514 | 84 |
| multicast + trigger + plain + blast + gas | 48 | 13 let out; 12 blasts, 11 gas | 1,755 | | 3,006 | 84 |

The plan's deciding measure for step B, 10 s from the same start:

| 10 s, 29-39 s | Bullets | Payloads cast | Bursts a second | Thrown | Slept | Killed | Health at 39 s |
|---|---|---|---|---|---|---|---|
| bomb + gas | 80 | 39 bombs, 36 gas | 7.5 | 5,764 | 7,267 | 548 | 84 |
| trigger + bomb + gas | 79 | 70 bombs, 66 gas | 13.6 | 9,099 | 9,637 | 399 | 84 |
| multicast + bomb + gas | 160 | 67 bombs, 73 gas | 14.0 | 8,337 | 5,086 | 475 | 84 |
| plain + blast | 80 | 31 blasts | 3.1 | | | 1,941 | 70 |
| multicast + plain + blast | 154 | 38 blasts | 3.8 | | | 2,018 | 80 |

## What it shows

- **Multicast doubles what a shot carries.** Over 3 s it cast 20 blasts
  against 8 and killed 1,720 against 1,093, and the squad's health held
  (81 against 74).
- **Multicast and trigger read differently.** Both fire about twice the
  payloads of cards in turn (14.0 and 13.6 bursts a second against 7.5). The
  trigger puts the gas into the slime the bomb just threw (9,637 slept); the
  multicast's gas goes off beside the bomb, which throws the slime out of
  it, so fewer slept than in turn (5,086 against 7,267). Breadth against
  depth: that is the combo choice the plan was after.
- **Over 10 s the front thins.** Multicast's blasts that found slime fell to
  38 of 77 fired, as the trigger's did: the extra payload clears the near
  slime and the rest fly on.

## Frame cost

- **Read-backs:** in the 10 s multicast + bomb + gas run the step's 207
  bursts went in 69 batches, so 69 read-backs; one at a time they were 207
  (a bomb's kill and its throw, a gas). Bomb + gas in turn: 114 bursts in 53
  batches. The batch test: 6 bursts, 1 read-back against 5.
- **On the GPU** each burst is the passes it was, plus one 12-byte copy. Not
  yet timed on the GTX 1070.
- **Multicast** adds bullets, under the same `squad::SHOTS_MAX` cap, and
  each finds its own slime: no new pass.

## Not settled

- **Balance:** cost 3, tier 3, as the plan's rule cards; nothing measured
  for price (build first, balance later).
- **Copy and always**, the plan's last rule cards, are not built.

## Files

- `page/index.html` and its clips: the Artifact page's source.
- `raw/film.sh`: one clip's frames (`START`, `CENTRE` from the environment).
- `raw/make_clips.py`: frames to H.264 MP4 and the counts as JSON.
- `raw/counts.json`: the counts above.
