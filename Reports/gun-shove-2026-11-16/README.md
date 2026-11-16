# Shove cards: herd the slime

2026-11-16. The owner, after the step-A films (05:10Z): "The slug looks fun";
05:11Z: "There should be ways to modify the pushback. I could image a gun that
lets you corral and push the slime onto areas. Into a kill zone". Built before
the rest of step B of the [gun-system review](../gun-system-review-2026-11-16/README.md)
(multicast, the read-back batching), after the
[trigger cards](../gun-triggers-2026-11-16/README.md).

Page: [Shove Cards](https://claude.ai/artifact/2xhG8dfegCSryLGDPexpdE) (source in `page/`;
its second version opens with Pull stopping short).

## Pull stops short (the owner's pick, 19:40Z)

Asked whether to keep Pull, change it or drop it (it dragged the slime
onto its own squad: all eight soldiers lost in 10 s), the owner chose "Stop
it short of the squad": the slime lands a few cells out in front, pulled
off cover or crystals into the squad's fire.

- **Built** (`Rounds::shove`): a pull's push flies at most as far as leaves
  its nearest walker `RoundRules::pull_short` (6 dish cells) from the
  shooter: the room is the distance from the hit to the soldier, less the
  push's reach and the 6 cells, and the flight is cut to the whole fine
  steps that cover it (`World::walker_step` times `fling_speed`). With no
  room, the pull does nothing (`Rounds::short`, printed last on the
  headless rounds line as "pulls too near to pull").
- **The old scene** (the Fork, holding at the larder, one turret, from
  29 s): the slime is already at the squad's feet, so most pulls have no
  room (67 of 69 over 10 s). The squad keeps 7 of 8 soldiers, as with plain
  bullets, where the old pull lost all eight.
- **At range** (the same game from 12 s, while the squad walks to its hold
  and shoots slime 10-24 cells off): 18 pulls in 10 s, 37 too near; the
  pulled slime lands in the open in front of the squad. Kills about even
  with plain (282 against 286, the turret's included); all 8 soldiers.

| Pull, `raw/pull-films.sh` | Shoves | Too near | Thrown | Killed (turret's) | Soldiers | Health |
|---|---|---|---|---|---|---|
| Old scene, 29-32 s, plain | | | 0 | 199 (78) | 7 of 8 | 76 |
| Old scene, 29-32 s, pull + heave + plain | 1 | 19 | 369 | 247 (75) | 7 of 8 | 76 |
| Old scene, 29-39 s, plain | | | 0 | 606 (139) | 7 of 8 | 72 |
| Old scene, 29-39 s, pull + heave + plain | 2 | 67 | 717 | 559 (122) | 7 of 8 | 73 |
| The old pull, 29-39 s (below) | | | 11,186 | 572 (172) | 0 of 8 | |
| On the march, 17-20 s, plain | | | 0 | 101 (59) | 8 | 99 |
| On the march, 17-20 s, pull + heave + plain | 12 | 7 | 108 | 72 (47) | 8 | 99 |
| On the march, 12-22 s, plain | | | 0 | 286 (166) | 8 | 97 |
| On the march, 12-22 s, pull + heave + plain | 18 | 37 | 209 | 282 (163) | 8 | 98 |

The game's kills here (the plain rows) are lower than the tables below,
which ran on the build of 11-16 morning; compare rows within one table.
Clips: `page/close-*.mp4`, `page/march-*.mp4` (`raw/pull-counts.json`).
Pull is now safe and visible, and it adds no kills in these scenes: the
squads on their own fire at slime near them, which has no room to pull.

## What changed

- **A push** (`squad::Burst::push`, a heading): every walker in the burst's
  ring flies that one heading for the whole flight, where a fling throws each
  out from the centre and the nearest furthest. Both engines: `BigDish::fling`
  on the CPU, the WGSL `fling` pass on the GPU (the strike row's `kcr` carries
  the heading and `kir` marks a push), so no new pass and no new read-back.
- **Three cards** (`Kind::Modifier`, tier 2), each on the squad's bullets:
  - `Heave` (cost 2): the hit shoves the slime round it on along the bullet's
    line; each heave multiplies the reach and the flight by
    `RoundRules::heave` (1.5).
  - `Corral` (cost 2): the shove heads for the nearest standing turret within
    `RoundRules::corral_reach` (40 dish cells) of the hit; none in reach,
    straight on.
  - `Pull` (cost 1): the shove heads for the soldier who fired.
  A standard bullet with a card shoves every walker within 1 +
  `RoundRules::shove` (2) strike radii of the hit (1 + 2 x 1.5^n with n
  heaves), for `shove_secs` (0.25 s, x 1.5^n). A slug keeps its own shove (step A)
  and the cards steer it: `corral+slug` herds a slug's whole throw.
- **The gun range** shelves them on a tab of their own, "Cards that shove":
  on the next bullet's tab they made it two pages at 200% on 1080 lines (the
  window's menu test).
- **Counts:** `Rounds::pushes`; `Game::turret_killed` (walkers the turrets'
  shots killed); the headless rounds line prints both.
- **Film key** `turret=X,Y,T`: a building standing from game second T, so a
  film's game is the same as without it up to T.
- **Test loadouts** (H): `heave` (heave+heave+plain), `corral`
  (corral+heave+plain), `pull` (pull+heave+plain), `corral-slug`.

## Tests

- `rounds::tests::shove_cards_push_the_slime_where_they_say` (seed 2): with
  a turret beside the squad, at least 80% of the walkers each card throws
  move the way it says (on along the line, toward the turret, toward the
  gun), at least 70% of the free ones within 0.5 rad of that heading; two
  heaves throw farther than one.
- `mouldgpu` twin `an_explosion_flings_the_same_walkers_as_the_cpu`, new
  dish "a push, a wall across it": the GPU's push flies the same walkers as
  the CPU's, every timer the whole flight and every heading the push's
  (lavapipe, exact).
- The window's card test: card faces under 60 characters.

## Films

`hand=hold@150,265 bank=8000`, the Fork, squad mode, llvmpipe, near view
`centre=170,258,10`, 960 x 600, 30 frames a second (`raw/film.sh`,
`raw/make_clips.py`). Every clip is `rounds=plain load=29,NAME
turret=180,240,29`: one game to 29 s, then each loadout with one turret
standing up and to the right of the squad, off its line of fire. Counts from
29 s (`raw/counts.json`); the game's kills include the turret's.

| 3 s, 29-32 s | Shoves (pushes) | Thrown | Killed | By the turret | Soldiers at 32 s | Health |
|---|---|---|---|---|---|---|
| plain | 0 | 0 | 658 | 157 | 7 of 8 | 73 |
| slug | slug's own | 2,324 | 293 | 184 | 8 | 81 |
| heave + heave + plain | 21 | 1,886 | 91 | 26 | 8 | 83 |
| corral + heave + plain | 21 | 1,848 | 314 | 241 | 8 | 78 |
| pull + heave + plain | 24 | 3,517 | 247 | 79 | 7 of 8 | 66 |
| corral + slug | 22 | 2,508 | 410 | 287 | 8 | 80 |

| 10 s, 29-39 s | Thrown | Killed | By the turret | Soldiers at 39 s | Health |
|---|---|---|---|---|---|
| plain | 0 | 1,510 | 422 | 6 of 8 | 76 |
| heave + heave + plain | 9,612 | 283 | 78 | 8 | 83 |
| corral + heave + plain | 5,088 | 563 | 434 | 8 | 77 |
| pull + heave + plain | 11,186 | 572 | 172 | 0 of 8 | |
| corral + slug | 5,896 | 680 | 482 | 8 | 78 |

**A kill box** (three turrets, at 172,240, 180,240 and 188,242, from 29 s):

| Kill box, 10 s | Thrown | Killed | By the turrets | Soldiers at 39 s | Health |
|---|---|---|---|---|---|
| plain | 0 | 1,696 | 617 | 7 of 8 | 71 |
| corral + heave + plain | 4,914 | 1,384 | 1,234 | 8 | 76 |

Its 3 s clips: plain killed 722, the turrets 268, a soldier lost (7 of 8,
health 77); corral threw 2,254 in 22 shoves and killed 510, the turrets 440,
all 8 standing (health 78). In the corral clip a stream of thrown slime flies
up into the three turrets and heaps at their feet.

## What it shows

- **Corral herds the slime into the turret's fire.** In 3 s the one turret
  kills 241 against 157 with plain bullets and 26 when heave throws the same
  slime the other way. Over 10 s one turret saturates (434 against 422): it
  fires on its own clock and already had slime in range. With three turrets
  the corral doubles what they kill (1,234 against 617) and keeps every
  soldier, where plain bullets lose one.
- **Heave buys room, not kills.** The squad's own kills fall (91 against 658
  in 3 s), since the slime is thrown out of reach, and no soldier falls in
  10 s where plain loses two.
- **Pull is a trap as built.** It drags the slime onto the squad: one soldier
  down in 3 s, all eight in 10 s. It would need to stop short of the gun (the
  flight is one length for the whole push; the engine knows the step length,
  the rounds do not yet) or it goes; the owner's call. (Since changed: it
  stops short, above.)

## Not settled

- **Corral finds turrets only.** A marked spot, a wall's face, or any
  building is the next choice; turrets came first because they are the kill
  zone the owner named.
- **The squad's own kills fall with every shove card.** Shoves trade kills
  for where the slime goes. Balance is for the balance pass (build first,
  balance later); no census run.
- **Frame cost.** A push is the fling burst a slug's shove already was: no
  new GPU pass, one read-back a shove (the plan's batching covers it), and on
  the CPU one heading set a walker in the ring. Not timed on the GTX 1070.

## Files

- `page/index.html` and its clips: the Artifact page's source.
- `raw/film.sh`: one clip's frames (`START`, `CENTRE`, `TURRET` from the
  environment).
- `raw/make_clips.py`: frames to H.264 MP4 and the counts as JSON.
- `raw/counts.json`: the counts above.
- `raw/pull-films.sh`, `raw/pull-counts.json`: Pull stopping short.
