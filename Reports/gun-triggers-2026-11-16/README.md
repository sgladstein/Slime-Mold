# Trigger cards: step B of the gun-card plan, part 1

2026-11-16. The first rule cards of the [gun-system review](../gun-system-review-2026-11-16/README.md)'s
step B. The trigger was in the owner's 11-14 15:06Z notes ("trigger next card
on hit") and never built; the owner, 11-16 02:34Z: "I thought we were planning
on building a trigger card, but I don't think we did."

Page: [Trigger Cards](https://claude.ai/artifact/YHdvSjEmcftsLUiatQamMB)
(source in `page/`).

## What changed

- **`Card::Trigger` and `Card::Expire`** (kind "Next bullet", cost 2, tier 2):
  the cast after the card's own is carried (`Cast::child`, linked in
  `rounds::casts`) and leaves the gun's own turn (`rounds::cycle`,
  `BulletMods::cycle`), so every shot fires both. A carried cast may carry one
  more (`CARRY_DEPTH` 2; with six slots a third carrier has nothing left to
  carry). A trigger with no cast after it carries nothing.
- **The release** (`Game::release`, from `Game::landed`): the carried cast is
  fired as that cast's own bullets would be from the gun (its set's pellets
  and fan, its strike, range, speed, the squad's flight cards, no aim spread),
  from the release point along the carrier's heading. A trigger lets it out at
  the bullet's first slime hit (a piercing carrier flies on empty); an expire
  where the bullet stops: spent on slime (a needle's third hit), at the end of
  its range (it flies on from there), or at a wall (it heads back out off the
  wall's face, from one cell back). Released bullets count in
  `Game::releases`, join after split pieces and are dropped past
  `squad::SHOTS_MAX` like any bullet.
- **The carrier's own hit clears a hole first**, so the carried bullet flies
  through it and goes off at the next slime on the line: in the test, 45 of 45
  blasts within 3 dish cells of a carrier's hit or straight on along its line.
- **What you see:** a carrier draws a bead in the carried cast's colour at its
  head; the gun-card strip lights the carrier and what it carries together as
  the next shot.
- **Counts:** `Rounds::gassed` (walkers put to sleep by rounds); the headless
  rounds line prints for squads with their own cards too, with the carried
  casts let out.
- **Test loadouts** (H): `plain-blast`, `trigger-blast`, `buck-trigger`,
  `needle-deep`, `relay`, `chain`, `bomb-gas`, `trigger-bomb-gas`.

## Tests

- `rounds::tests::a_carrier_takes_the_next_cast_out_of_the_turn`: links,
  chains, the turn, a carrier with nothing after it.
- `larder_world::tests::a_carrier_lets_its_cast_out_where_it_hits_or_stops`:
  a trigger lets a buckshot out as four fanned pellets at the hit with its own
  range and strike while the carrier pierces on empty; an expire keeps its
  cast through a pierced hit, flies it on from the end of its range, and
  throws it back off a wall.
- `rounds::tests::a_trigger_blasts_where_its_bullet_hits` (seed 2, 15 s):
  `trigger+plain+blast` fires only plain bullets; 60 let out, 45 blasts, every
  blast by a plain hit or on along its line; more blasts than `plain+blast`.
- The window's card test: carried casts light with their carrier; card text
  under 60 characters.

## Films

`hand=hold@150,265 bank=8000`, the Fork, squad mode, llvmpipe, near view
`centre=176,266,12`, 960 x 600, 30 frames a second (`raw/film.sh`,
`raw/make_clips.py`). Every clip is `rounds=plain load=29,NAME`: one game to
29 s, then each loadout. Counts from the load (`raw/counts.json`).

| Loadout, 29-32 s | Shots (bullets) | Let out | Payload casts | Killed | Thrown | Slept | Health at 32 s |
|---|---|---|---|---|---|---|---|
| plain + blast | 24 | 0 | 8 blasts | 1,093 | | | 74 |
| trigger + plain + blast | 24 | 21 | 17 blasts | 1,845 | | | 78 |
| bomb + gas | 24 | 0 | 13 bombs, 8 gas | 176 | 2,859 | 2,443 | 84 |
| trigger + bomb + gas | 24 | 19 | 19 bombs, 19 gas | 123 | 3,374 | 4,258 | 84 |
| trigger + buckshot + blast | 24 (96) | 56 | 42 blasts | 1,943 | | | 81 |
| expire + needle + blast | 24 | 18 | 9 blasts | 1,841 | | | 68 |
| expire + plain + expire + plain + gas | 24 | 42 | 20 gas | 1,238 | | 8,215 | 83 |
| trigger + buckshot + trigger + plain + blast | 24 (96) | 96 | 35 blasts | 2,287 | | | 80 |

The plan's deciding measure for step B (bursts a second, slept and flung), 10 s
from the same start (29-39 s, 80 shots):

| Loadout, 29-39 s | Bursts a second | Thrown | Slept | Killed | Health at 39 s |
|---|---|---|---|---|---|
| bomb + gas | 7.5 | 5,764 | 7,267 | 548 | 84 |
| trigger + bomb + gas | 13.6 | 9,099 | 9,635 | 399 | 84 |
| plain + blast | 3.1 | | | 1,941 | 70 |
| trigger + plain + blast | 2.4 | | | 2,037 | 76 |

## Not settled

- **Buckshot with a trigger is strong:** 42 blasts in 3 s against 8 for plain
  and blast in turn (the chain 35). That is the plan's lucky combo; a note for
  the balance pass (build first, balance later), no census run.
- **A trigger alone adds little over 10 s** (2,037 killed against 1,941, one
  seed): its early burst cleared the front and fewer plain bullets found slime
  after. What it changes is where the payload goes off and what it pairs with.
- **Frame cost.** A carried bullet is an ordinary bullet in flight under
  `SHOTS_MAX`. `Game::mods` now builds the round's casts (`rounds::flight`,
  a few casts) per call, on each shot fired and each release: a few
  microseconds a frame at most. Not timed on the GTX 1070. The GPU read-back
  per bomb throw or slug shove is unchanged; the plan's batching covers it.
- Not built from the plan's step B yet: split pieces that carry casts,
  multicast, and the read-back batching. The owner's shove cards come first
  (11-16 05:11Z: "There should be ways to modify the pushback. I could image a
  gun that lets you corral and push the slime onto areas. Into a kill zone").

## Files

- `page/index.html` and its clips: the Artifact page's source.
- `raw/film.sh`: one clip's frames (`START`, `CENTRE` from the environment).
- `raw/make_clips.py`: frames to H.264 MP4 and the counts as JSON.
- `raw/counts.json`: the counts above.
