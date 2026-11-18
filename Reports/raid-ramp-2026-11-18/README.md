# Raids rare early, often later; no mini-bosses on planet 1 (2026-11-18)

The owner's play notes, 2026-11-18 15:56Z: "The raids come too early and
too often (or at least too often in the early game). Occasionally a raid
can happen early in a session but not every time. and they can happen
often later on in a session, but not right at the begining. It is hard to
build pumps, walls, turrets because you get raided so quick. Mini-bosses
also shouldn't show up in world one." Direction's targets: the first raid
usually 3:00 to 5:00, about one game in five before 2:00; the gap from
about 150 s falling to about 45 s by 10:00.

Page: https://claude.ai/artifact/ArxsZH7SzCuFPgkEyeRZAc (source `page/`; open `index.html`).
Films: `film.sh`; clips: `make_clips.py`.

## What changed

- `RaidRules` (raids.rs): the first raid 240 +- 40 s after the landing, or
  with chance `early` 0.2 at 80 +- 20 s; the gap 150 +- 45 s at the landing
  falling in a straight line to `late` 0.3 of that (45 +- 13.5 s) by `ramp`
  600 s, then flat (`RaidRules::gap_at`). Before: the first at 75 +- 15 s
  from the world's start, then 60 +- 20 s all game.
- `BossRules::for_depth` (bosses.rs): none on planet 1; the old curve moved
  up a planet (planet 2 has planet 1's old chances, planet 8 the same
  every 45 s and 10%).

## Planet 1, run seed 1, pumps hand, to 12:00

The deciding measures, named before the films: when the raids come, and
how long the near pump (by a free crystal) stands.

| | Before | After |
|---|---|---|
| Raids | 0:59 (near pump), 1:35, 3:00, 4:15, 5:32, 6:14, 7:13, 8:21, 8:48, 9:26, 9:49, 10:55 | 3:15 (near pump), 4:32, 6:12, 7:47, 8:57, 9:32, 9:58, 10:56, 11:52 |
| Mini-bosses | 1:41, 4:09, 6:18, 8:36, 11:21 | none |
| Near pump | gone 1:12 | 185 at 2:00, 151 at 3:00, gone 3:24 |
| Food pumped | 1,607 | 4,271 |
| Soldiers | 4 to 1 by 2:12 (one held to 9:00) | 4 till 3:24, 0 by 3:48 |

The after film's gaps: 77, 100, 95, 70, 35, 26, 58, 56 s.

## The owner's run seed (his play log's), run bot

Raid times come off the raids' own dice, so his game met the old ones.

| Planet | Before (what he met) | After |
|---|---|---|
| 1 | raids 1:08, 1:47, 3:07; mini-boss 1:32 | raids 4:43, 6:48, 8:40; the bot wins 8:56 |
| 2 | raids 0:54, 1:14, 2:06, 2:35; mini-boss 1:26 | raid 4:45; mini-bosses 1:33, 4:06 |

## The Drifter's pump smell

Direction asked whether the smell (`pump_smell` 20 over `lure_r` 15,
`run::does_to_you`) is part of "raided about a minute in". The pumps
hand's second pump stands by a crystal the slime holds; with the new
raids, to 3:00 (no raid by then):

| pump_smell | That pump at 1:00 | Near pump at 3:00 | Food pumped | Killed | Slime |
|---|---|---|---|---|---|
| 20 (the Drifter's) | 75, gone 1:12 | 151 | 3,741 | 7,994 | 15,820 |
| 8 | 97, gone by 2:00 | 0 | 3,944 | 3,776 | 18,646 |
| 0 | 167, gone by 2:00 | 0 | 6,303 | 1,064 | 19,056 |

The minute-in loss was the first raid (0:59 here, 1:08 on his seed). A
pump by the slime's crystal goes within two minutes whatever the smell,
the slime growing over it; the smell takes it sooner and draws the slime
onto the squad's guns (eight times the kills). It stays as the Drifter's
trait; one seed, one film each.

## Not checked

- Other seeds; planets past 2 in a run as played; the bot's later
  planets. One film each on the software GPU (not timed: nothing here
  runs on the frame).
