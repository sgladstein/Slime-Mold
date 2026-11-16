# Seeking bullets: step C of the gun-card plan, first try

2026-11-16. Step C of the [gun-system review](../gun-system-review-2026-11-16/README.md)
("seekers that follow the simulation"; the owner's 02:34Z ask for homing),
after the [trigger cards](../gun-triggers-2026-11-16/README.md) and
[multicast](../gun-multicast-2026-11-16/README.md).

Page: [Seeking Bullets](https://claude.ai/artifact/JxxzYTtiug3oNrhfwX3b7J) (source in `page/`).

Status: built on the branch, **not merged**; the owner is asked how homing
should work (the page's last section). A prototype of the recommended answer
(seekers fly over the slime to a mark) is on the branch too.

## What was built

- **`Card::Seek`** (kind "Always", cost 2, tier 2): every bullet of the
  squad, once it has flown `Rules::seek_after` (3 dish cells; a split piece
  or a let-out cast at once), turns each frame by at most `seek_turn`
  (0.45 rad) a card (`Game::steer`, before the frame's legs). Its mark: a
  live mini-boss ahead in reach and sight first; else the cheapest square
  of the mould grid (`World::mould_grid` at a dish cell, the GPU's from the
  frame's readout) within `seek_r` (12) and `seek_cone` (1 rad) holding
  `seek_min` (2) walkers, in sight, more than `seek_gap` (2) dish cells from
  last frame's hits and from earlier seekers' picks this frame (the plan's
  "shrapnel seek": each pellet or piece its own patch). No engine change:
  bullets are game state and each frame's leg is straight.
- Counts: `Game::seek_turns`, `Game::hits`, `Game::hit_cells` (the dish
  cells hit, once a frame); the headless rounds line prints them.
- Presets `seek`, `seek-buck`, `seek-split`; `set=seek_*` tunes the rules.
- Test: `larder_world::tests::a_seeker_turns_toward_slime_no_other_bullet_took`.

## Films

`hand=hold@150,265 bank=8000`, the Fork, squad mode, llvmpipe, near view
`centre=176,266,12`, 960 x 600, 30 frames a second (`raw/films.sh`,
`raw/make_clips.py`, `raw/counts.json`). One game to 29 s with plain
bullets, then each loadout; the mini-boss clips add `boss=29,185,250`.

| 3 s, 29-32 s | Bullets | Hits in cells | Seeker turns | Killed |
|---|---|---|---|---|
| buckshot | 84 | 81 in 58 | | 58 |
| seek + buckshot | 84 | 84 in 57 | 62 | 63 |
| split + blast | 21 | 49 in 49 | | 1,497 |
| seek + split + blast | 21 | 50 in 50 | 134 | 1,366 |
| mini-boss, plain | 21 | 21 in 21 | | 68 |
| mini-boss, seek + plain | 21 | 20 in 20 | 35 | 71 |

| 10 s, 29-39 s | Without seek | With seek |
|---|---|---|
| plain: killed | 472 | 434 |
| buckshot: killed | 228 | 214 |
| split + blast: killed | 1,744 | 1,756 |
| mini-boss, plain: its members left of 62 | 25 | 40 |
| mini-boss, buckshot: its members left of 62 | 60 | 62 |

These tables ran before the fly-over prototype; to reproduce them now, add
`set=seek_over=0`. One game each: a few percent is noise. Nothing says seek helps; the
mini-boss rows lean the other way.

## Why it changes little

- A shot stops at the first slime on its line (owner 11-11). The front is
  solid, so a bent bullet still hits the slime in front, a cell or two
  from where it would have.
- Soldiers already aim at the densest slime in sight and their hits were
  already spread (the plain game to 39 s: 215 hits in 214 cells), so the
  gap rule has no crowding to undo.
- The plan predicted it for plain homing ("lands where the bullet was
  going anyway"); the gap and the mini-boss mark did not change that.
- Aim cards meet the same wall: squads on their own hold the defend stance
  and only shoot slime near them, so an aim card would change only which
  nearby slime they pick; it matters for squads on attack orders.

## Prototype: seekers fly over the slime to a mark

To try the recommended answer (on this branch, `Rules::seek_over`, on): a
seeker past `seek_after` with a mark ahead within what it has left to fly
and in sight (a live mini-boss's centre, else the densest square on a
crystal holding `seek_fed` 20 walkers) is not traced (`Bullet::over`;
`Flight::legs` keeps every leg, the world gets the others) until this
frame's leg reaches the mark, so it strikes nothing on the way and lands
there. No engine change.

| Mini-boss, 10 s, 29-39 s | Members left of 62 | Killed | Soldiers left of 7 |
|---|---|---|---|
| plain | 25 | 259 | 6 |
| seek + plain, flying over | 30 | 508 | 7 |
| buckshot | 60 | 136 | 6 |
| seek + buckshot, flying over | 45 | 78 | 5 |

3 s clips from 34 s (the hunt's start): buckshot 91 hits in 75 cells, the
mini-boss 62 to 61, 7 soldiers; seek + buckshot flying over 82 hits in 40
cells, 62 to 52, 6 soldiers. The pellets reach the mini-boss instead of
the slime in front; the slime between is left alone. With plain bullets
the two came out about even on the mini-boss. In the hold scene with no
mini-boss nothing changed (434 and 214 killed, as without it): crystal
feeders seldom became marks, as a squad on its own fires only at slime
near it.

Rejected: flying over from the muzzle. That squad lost 4 of 7 soldiers in
8 s (from 29 s) against none, because it stopped shooting the slime on
itself; the first leg is now traced.

## Asked of the owner

How homing should work: seekers that fly over slime to their mark (a
mini-boss, the slime feeding on a crystal, a vein), bending the
first-slime rule for that card only (recommended: the only version that
reaches what a plain shot cannot); keep seek small as built; or drop
homing and go on to step D.

## Files

- `page/index.html` and its clips: the Artifact page's source.
- `raw/films.sh`: every clip and 10 s run; `raw/film.sh`: one clip's frames.
- `raw/make_clips.py`: frames to H.264 MP4 and the counts as JSON.
- `raw/counts.json`: the counts above.
