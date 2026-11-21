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

## The army-scaled strikers (PR #566, 19:00Z)

The slime thread's next defaults: W, with the share of strikers scaled by
soldiers alive (none at 8 or fewer, `strikers_few`; full from 24,
`strikers_full`), plus its mass fix. Scored in the cloud session on
lavapipe (not GitHub's; one game a map and arm, so read it beside the run
above, not paired with it), #566 merged into this branch locally; 27
games, `strikers-566-rows.jsonl`. Arms: the new default (D), W unscaled
(`set=strikers_few=0 set=strikers_full=1`), a softer charge
(`set=strikers_haste=2`).

| Map | D: late bites, result | W unscaled | Haste 2 |
| --- | --- | --- | --- |
| 1 (army 80) | 37, won 7:01 with 78 | 35, won 7:27 | 26, won 7:01 |
| 2 | 33, lost 8:36 (army peaked at 30) | 25, lost 5:02 | 9, won 6:07 (peak 22) |
| 5 (army 80) | 28, won 3:32 | 25, won 3:32 | 9, won 3:30 |
| 7 (small squad) | 2, won 4:23 | 4, lost 1:14 | 2, won 4:23 |
| 8 (small squad) | 1, won 4:58 | 5, lost 2:04 | 1, won 4:58 |

Maps 3, 4, 6 and 1795038793 are the same loss in every arm. Against D:
W unscaled up 2, down 3, tied 4 (p 1.0); haste 2 up 0, down 3, tied 6
(p 0.25). Wins: D 4 of 9, W unscaled 2, haste 2 5.

So the scaling does what it was for: small squads win again (maps 7 and
8), and the big army is still bitten, 28 to 37 soldiers late on maps 1
and 5, which the bot rebuys and still wins. Its cost is map 2, where the
strikers hold the army near 24 to 30 and finally break it; haste 2 keeps
that win and bites less. The pick is the slime thread's.

## The new strikers on planets 1 and 3 (21:10Z)

Main at 554d0d5 (#566's strikers, the lunge, Starfall), GitHub lavapipe,
run 44809389422 (`jobs-strikers-566-p13.txt`,
`strikers-566-p13-rows.jsonl`); 36 games. Arms: #566's default against
the first numbers (`set=strikers_below=0.4 set=strikers_band=12
set=strikers_share=0.5 set=strikers_haste=1.5 set=strikers_few=0
set=strikers_full=1`).

**Planet 1**: the same game in both arms on all nine maps. The strikers
never start: the slime is never cut under 60% of its peak. The plan (the
planets 2 and 3 deck to watch) loses planet 1 on every map with 4 to 18
soldiers, which is the plan's planet, not the strikers'.

**Planet 3** (the Trunk): fire still kills most soldiers (the bot's own
Needle Blast). Late bites against the first numbers: up 4, down 3, tied 2
(p 1.0). Wins: 8 of 9 with the new numbers, 9 of 9 with the first. The
ninth, map 5, did not end: from 10:00 to the 12:00 cap the bot had all 80
soldiers and the slime sat at 6,500 to 8,700 walkers (15 to 20% of its
peak), regrowing as fast as it was cut; the first numbers broke it at
6:30. Before that the strikers held the army at 29 to 48 for five
minutes. The new numbers made the game longer on 6 of 9 maps (median
5:11 against 4:42) and cost fewer soldiers on 6 of 9.

## The last-stand heart (PR #580, 21:15Z)

The slime thread's option D (key `stand`): under 20% of its peak, against
24 soldiers or more, every free walker gathers into one armoured heart on
its heart crystal that lunges every 20 s. Scored in the cloud session on
lavapipe, #580 merged locally onto main at 554d0d5 (so with #566's
strikers, the lunge and Starfall); one game a map and arm; 27 games,
`jobs-heart-580.txt`, `heart-580-rows.jsonl`. Arms: the default (D), the
heart off (`set=stand=0`), armour 2 (`set=stand_armour=2`).

| Map | D: late bites, result | Heart off | Armour 2 |
| --- | --- | --- | --- |
| 1 (army 80) | 28, won 4:37 | 28, won 4:52 | 28, won 4:46 |
| 5 (army 80) | 46, won 3:58 | 26, won 3:42 | 1 (33 in all), won 4:03 |
| 6 (army 58) | 50, won 8:57 with 7 left | 35, won 9:31 with 7 | 23, lost 11:15 |
| 2 (army 19) | 7, not over at 12:00 | 19, lost 5:41 | 18, lost 5:03 |
| 7 (army 43) | 0, won 4:54 | 0, won 5:03 | 0, won 6:29 |
| 8 (army 53) | 1, won 6:25 | 1, won 5:57 | 1, won 6:25 |

Maps 3, 4 and 1795038793 are the same loss in every arm. The heart
against off: late bites up on 2 maps, down on 1, tied on 6 (p 1.0); wins
5 and 5 (armour 2: 4). It adds bites where the army is big when the
slime is nearly gone (maps 5 and 6: 20 and 15 more), and on map 1, where
the strikers already took 28 at 4:30, it adds none. Map 2's army never
reached 24, so its difference is the game going another way, not the
heart.

## The closing measure on the striker rate cap (PR #598, 11-21 00:30Z)

The whole late package (strikers, the lunge, the heart) against all three
off, the same 36 jobs as the closing measure (`jobs-closing.txt`), rerun on
main with #598 (strikers form at a capped rate: about 90 a second on
planet 2, 150 from planet 3); GitHub run 44840663837,
`closing-598-rows.jsonl`.

| | Late bites, maps up / down / tied | Sign test p | Wins, package on / off |
| --- | --- | --- | --- |
| Planet 2, before #598 | 5 / 0 / 4 | 0.062 | |
| Planet 2, with #598 | 5 / 1 / 3 | 0.22 | 4 / 6 |
| Planet 3, with #598 | 4 / 1 / 4 | 0.38 | 9 / 9 |

The rate cap softened the bite a little. On planet 2 the package still
costs mid-size armies, not small squads: maps 2 and 6 run to the 12:00
cap unfinished with it (27 at peak to 8 alive; 56 to 31), and the
package-off arm wins them at 11:01 and 9:46; maps 3, 4 and 1795038793 lose
the same way in both arms with 4 to 7 soldiers. Since #606 planet 2 keeps
only the strikers (at 0.6), so those two maps are the ones to recheck.

## The heart's bout cap (11-21, Direction's ask)

The deep bug check found the last-stand heart's first lunge killing 68
soldiers in under 8 s on planet 2 (a wipe; the whole-run bot lost the
planet). Direction asked for it toned down, the method ours, with the
measure named first: soldiers lost in the heart's first 10 s on planet 2's
big-army maps, paired by map against main; at most about 20, or a quarter
of the squad, while it still bites big armies late; squads of 8 or fewer
lose no more planets.

The change (`StandRules::cap`, `cap_most`, `sated`; `stand.rs`): a bout
(the heart's forming, its first 10 s, or a lunge) stops once the slime's
touch has taken a quarter of the squad's health (`Game::bitten_hp`;
never more than 20 soldiers' worth). A lunge then draws back at its pace,
and the whole slime bites no soldier for 5 s. What did not hold, in
order: a cap on deaths (soldiers under a heart are all near death when
the first dies: capped at 1 of 4 it killed 3, the GPU test); a health
cap alone (the body still lies on the rest as it goes: 3 of 4); a pause
only within the heart's reach (a gathering heart's members are spread
over the map and its strikers are not in it: a forming bout capped at 20
took 24 of 80, a lunge capped at 9 took 17 of 35; rows
`heart-cap-reach-rows.jsonl`, run 44849983687).

Measured on planet 2 with the heart forced on (`set=stand=1`; since #606
it first plays on planet 4, and planet 2 has the only big-army maps the
bot reaches reliably), the strat bot's big-army plan, strikers on, and
three whole runs of the bug check's plan; capped (the default) against
main's heart (`stand_cap=1 stand_cap_most=1000 stand_sated=0`); GitHub run
44857683499, `jobs-heart-cap.txt`, `heart-cap-rows.jsonl`. The league line
has `stand=BORN:LUNGES:FIRST:WORST:KILLED` (second formed, lunges,
soldiers bitten to death in its first 10 s and in its worst bout's 10 s,
all it bit).

| Game | Main's heart: worst 10 s, all it bit, result | Capped |
| --- | --- | --- |
| Planet 2 map 1 (army 80) | 65 in one lunge, 65, won with 43 | 13, 13, won with 80 |
| Planet 2 map 5 (army 80) | 10, 10, won | 10, 10, won |
| Run 1 planet 1 (51) | 9, 12, won | 3, 6, won |
| Run 3 planet 1 (43) | 13, 13, won | 13, 13, won (same game: the cap was not reached, the squad was already hurt) |
| Run 3 planet 2 (37 to 48) | 19, 27, **lost** with all 37 dead | 0, 0, won with 48 |
| Run 1 planet 2 (50 to 57) | 24, 24, won | 7, 7, won |
| Run 1 planet 3 (75 to 80) | 11, 12, won | 3, 3, won |
| Run 3 planets 3, 4 (80) | (run over) | 4 and 16, 14 and 37, won both |

Runs 1 and 3 diverge from their first heart on, so only the first planet
with a heart in each run is a clean pair. Clean pairs (the two planet 2
maps, each run's first heart planet): the worst bout fell on 3 and tied
on 2 (sign test p 0.25); in every game it is lower or equal capped. The
whole runs reached planets 4, 1 and 5 capped against 4, 1 and 2. On the
cloud session (lavapipe, strikers off, the bug check's own trace): run 2
lost planet 2 to main's heart, all 44 bitten in its one lunge, and won it
capped with a worst lunge of 4 (the heart still bit 28); run 1's planet 1
heart took 22 of 51 forming, and 5 capped.

The bar holds: no capped bout took more than 16 (20% of an army of 80)
or more than a quarter of its squad, except run 3's planet 1 heart, which
took 13 of 43 without reaching its cap in either arm. It still bites: 13,
10, 37 and 14 over a heart's life on the big armies. Squads of 8 or
fewer never meet it (it gathers only against 24 or more), and maps 3, 4,
6, 7, 8 and 1795038793 play identically in both arms.

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
