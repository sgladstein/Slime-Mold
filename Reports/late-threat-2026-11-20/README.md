# Keeping a big squad under threat: the measure, and the late lunge (2026-11-20)

The owner, 11-20 14:10Z: "once your squad is big enough/they do enough
damage the slime cannot touch them, so it doesn't matter how much slime
their is". The brief from the model, with eight options:
https://claude.ai/artifact/YD9LSGAkAorDNKg5WA2zZH (a Claude Doc). His pick
on a card in the balance thread, 15:10Z: "Condensed strikers + late lunge"
(A and B). The slime thread built A (`strikers`, #563); this thread built
B (`late_at`) and the measure, and ran both.

## What his logs say (19 planets, 11-18 and 11-19, the `play-logs` branch)

The slime kills about 1 to 1.6 soldiers a minute at every army size. With
the two heavy-loss planets left out, a squad of 1 to 4 loses 14.7% of
itself a minute, an army of 41 or more 2.4%, while recruiting 16.9 a
minute against 1.6 deaths. Two planets hold 204 of the 302 soldiers lost:
11-19 planet 4 (the Swarmer's lunge, 32 in 30 s, then its last stand) and
11-19 planet 3 (57 in 90 s at the end, cause not in the log). Script:
`losses.py`.

## The measure

`larder-world league=...` now prints, besides `marks`:

- `fell=T:FALLEN/BITTEN/BURNED,...` every 30 s: soldiers fallen so far,
  those the slime's touch killed (`Rules::drain`) and those fire killed
  (`Rules::fire_hurt`); the rest died to blasts and casts.
  `Game::fell_bitten`, `Game::fell_burned`.
- `fell_end`, `bitten_end`, `burned_end`, and `late` (the slime was
  cornered, `Game::late`).

`league.py` jobs may carry extra game arguments after SECS (the arm).
`measure.py` pairs arms by map. **The deciding measure, named before the
runs: soldiers the slime bites to death in a game's last quarter.** Not
all fallen: on planet 3 fire, not the slime, kills the bot's army.

Plan: `econ,go=4,aim=6,cards=0,want=wide/rate/blast.needle` (the league's
deck to watch, peak 80 soldiers). Maps: seeds 1-8 and 1795038793 (the
owner's run seed). 720 game seconds at most. Games on GitHub's software
Vulkan (strategy-league workflow, dispatched on this branch).

## Results

**Planet 3** (Trunk, main at c61bd26 plus `late_at`; 27 games, run
44762522498, `planet3-rows.jsonl`): 2,906 soldiers fell, 34 (1.2%) to the
slime's touch, 2,683 (92%) to fire, the rest to blasts. Seeds 1 and 5 lose
the opening four to fire before 1:30. Late bites, median 0 in every arm.

| Arm against base | Late bites up | down | tied | sign p |
| --- | --- | --- | --- | --- |
| `set=late_at=0.4` | 1 | 2 | 6 | 1.0 |
| `set=wave=0` | 0 | 1 | 8 | 1.0 |

**Planet 2** (Sprawler, main at dc30d20 with strikers and the heartbeat;
36 games, run 44773519346, `planet2-rows.jsonl`): every death is a bite (no
fire). Where the bot wins with its 73 to 80 soldiers (maps 1, 2, 5), it
loses 1 to 9 soldiers in the whole game in every arm. Where it never
builds an army (maps 3, 4, 6, 1795038793) it loses with 4 or 5, the same
in every arm.

| Arm (base: strikers and waves on) | Late bites up | down | tied | sign p |
| --- | --- | --- | --- | --- |
| `set=strikers=0` | 1 | 3 | 5 | 0.63 |
| `set=late_at=0.4` (strikers and lunge) | 1 | 1 | 7 | 1.0 |
| `set=strikers=0 set=late_at=0.4` (lunge alone) | 1 | 2 | 6 | 1.0 |

One map turned on the strikers: map 7, where the bot cut the slime with
four soldiers, won at 4:21 without strikers and lost at 3:05 with them. A
small squad, not a big army.

Nothing survives a correction; nothing comes near p 0.05. **Neither the
strikers nor the lunge, at their first numbers, make the slime bite an
80-soldier army.** It fits the brief's model: the army kills about 220
walkers a second at range, so a charge four times as tough, or three times
as fast, still dies before it touches.

## The striker retune (the slime thread's arms, 18:00Z)

Planet 2, main at dc30d20 plus this branch (strikers, waves, the lunge
on); 27 games, run 44787524336 (`jobs-strikers-retune.txt`,
`strikers-retune-rows.jsonl`). H: `set=strikers_haste=3`. W:
`set=strikers_haste=3 set=strikers_share=1 set=strikers_band=30
set=strikers_below=0.6`: everyone in a wider band condenses, from 60% of
peak instead of 40%.

| Arm against the run as it is | Late bites up | down | tied | sign p |
| --- | --- | --- | --- | --- |
| H | 2 | 0 | 7 | 0.50 |
| W | 5 | 0 | 4 | 0.062 |

Neither survives the correction for two arms (0.025), but W is the first
setting that bites the big army, on all three maps where the bot builds
one:

| Map | Base: late bites, soldiers left | W: late bites, soldiers left |
| --- | --- | --- |
| 1 | 4, 80 of 80 (won 7:09) | 80, 5 of 80 (won 7:13; 79 died between 6:30 and 7:00 while the slime fell from 14,038 to 17) |
| 2 | 0, 74 (won 8:43) | 18, 42 of a peak 45 (won 8:24) |
| 5 | 0, 80 (won 4:24) | 17, 80 after rebuying (won 3:44) |

The cost falls on small squads: on map 8 the bot's four soldiers won at
4:38 in the base and H, and with W lost at 2:05. Map 7, the four-soldier
game, now loses in every arm (base 1:47, H 1:40, W 1:26); before the
lunge and strikers it won at 4:21. The lone losses on maps 3, 4, 6 and
1795038793 are the same game in every arm.

## What landed

`late_at` 0.4 on every run planet (`run::ENEMY_RULES`), with
`late_margin` 25 and `late_haste` 3: the owner picked it, it is visible,
and the league found no harm. Its "what is new" line says the bot games
found no gain, for him to judge in play. Not a balance call.

## Next

- The slime thread retunes the strikers first (its plan if A did not hold
  up), and this measure scores it: the same 36 planet 2 jobs
  (`jobs-planet2.txt`).
- The brief's levers that change the model's sum rather than its terms:
  contact with no crossing (buried knots, F; the Mortar's throw) and the
  army walking into dense slime (the heart, D).
- The bot's fire deaths on planet 3 went to the strategy-bot thread.
