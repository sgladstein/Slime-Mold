# Route: step D of the gun-card plan, the third family; and the germ retune

2026-11-17. Step D of the [gun-system review](../gun-system-review-2026-11-16/README.md)
("bullet families by what they change in the simulation"), after the
[Mind bullets](../gun-mind-2026-11-16/README.md) and the
[readers](../gun-readers-2026-11-17/README.md). The plan's route family:
cards that tell the slime where to go, not what to be. The owner, 03:07Z:
"Continue without my guesses. I am busy", so this step went ahead on the
thread's own picks. The germ's retune is the game designer's call (11-17
05:40Z, below).

Page: [Route Bullets](https://claude.ai/artifact/9Y1un1X9zRn9fwC348qYf9) (source in `page/`): the paired
clips with their counts, the armory's tab and the germ by planet.

Status: **the Flare is built, for merge; the Road was built and dropped;
the germ is retuned per planet.**

## The Flare

- **What it is** (`Card::Flare`, `Shell::Flare`): a payload with no kill
  that lays the alarm a strike lays, `flare_alarm` 100 over `flare_r` 16
  dish cells (times `Wide`), and burns: it lays again every `flare_every`
  0.2 s for `flare_secs` 3 s (`Rounds::burning`, at most 64 at once).
  `Quiet` takes its alarm. Cost 2, sold from planet 2 (`shells_at`), on the
  armory's Special bullets tab; loadouts `flare` and `flare-blast`; the
  rounds line ends with the flares cast (`Rounds::cast[6]`).
- **The slime's tier decides what it means.** Shape C, the planets 1-3
  slime, flinches from alarm (`flinch` 0.9), so a flare pushes it back; the
  Swarmer, Sleeper and Grudge walk to alarm (`alarm_weight` 3), so it draws
  them. Nothing new in the engines: the alarm is the strike's.
- **Look**: a flame on a stick with rays, red; the bullet's light red to
  pink. What you see of its effect is the slime's own move.

## Tests

- `rounds::tests::a_flare_burns_its_alarm_and_kills_nothing`: the card,
  its burst, `Quiet`, the `set=` keys, one lay every 0.2 s for 3 s and out.
- `rounds::tests::a_flare_pushes_a_flinching_mould_and_draws_a_swarmer`
  (CPU, race world): within 12 cells of the flare after 2 s a flinching
  mould went 53 to 11 walkers, one with the Swarmer's keys 53 to 136.

## The paired clips

`raw/film.sh` and `raw/films.sh`, `raw/make_clips.py`, counts in
`raw/counts.json`. The race world (seed 0, idle hand, plain rounds), each
pair the same game but for one free flare cast at 20 s (`boom=`); the near
view at zoom 6, 15 frames a second, 19.9 to 30 s, llvmpipe. Walkers within
12 cells of the flare and from 12 to 24, each second.

| Clip | Within 12: 20, 23, 28, 30 s | From 12 to 24: 20, 23, 30 s | Slime at 30 s |
|---|---|---|---|
| shape C, the Fork lobe | 293, 291, 305, 301 | 755, 767, 750 | 24,773 |
| shape C, flare at 20 s | 293, 253, 186, 200 | 755, 594, 666 | 24,766 |
| Swarmer keys, open ground | 0, 0, 0, 0 | 0, 0, 0 | 23,241 |
| Swarmer keys, flare at 20 s | 0, 121, 440, 439 | 0, 672, 894 | 23,289 |
| Sleeper keys, open ground | 0, 0, 0, 0 | 6, 6, 6 | 22,498 |
| Sleeper keys, flare at 20 s | 0, 34, 383, 412 | 6, 101, 894 | 22,484 |

What the clips show: on C the lobe backs away from the flare and the ring
opens for as long as it burns, then knits back; the Swarmer and the Sleeper
pour onto empty ground toward it and stay. A flare kills nothing, so the
slime's total does not move; it is a route card, a decoy.

## The paired windows in play

`raw/windows.sh`, table by `raw/windows_table.py` in `raw/windows.txt`.
The seek report's scenes (greedy on the Fork at 5 and 11 s, the Fork's
thick front held at 10 and 30 s, planet maps 1-3 with guarding or
squad-mode hands at 10, 25 and 40 s; 15 windows): each plays one game to
T, then every squad loads the variant's deck and the window runs 10 s.
Plain is plain bullets; flare is flare then plain, in turn. All of these
planets are shape C's, which flinches.

| Pair | Kills in the windows | Paired: fewer kills | Paired: more slime | Soldiers left |
|---|---|---|---|---|
| plain / flare | 1,095 / 1,165 | 13 of 15, 1 more (p = 0.002) | 12, 2 less (p = 0.013) | 121 / 119 |
| plain / road | 1,095 / 579 | 12, 3 more (p = 0.035) | 12, 3 less (p = 0.035) | 121 / 121 |

- **On flinching planets a flare costs kills**: it pushes the slime out of
  the squad's reach, so the plain bullets after it find less (fewer kills in
  13 of 15, the one result here that survives a Bonferroni correction over
  four comparisons, p < 0.0125). The one window it won, the held thick front
  at 30 s (912 against 471), it drew the slime's charge onto the squad and
  left 5 soldiers against 7: on C a flare near the squad is an alarm the
  slime answers by `panic`.
- So the Flare is the Swarmer's and the Sleeper's card (draw them where the
  squad wants them, as the tiers' measured counter, a decoy, asks:
  [tiers](../mould-tiers-2026-11-11.md)), and on C a way to open a ring
  that costs fire. Its row in the [card table](../../wiki/world/gun-cards.md)
  says so; no window here pits it against those planets.

## The Road, dropped

Every bullet laid the slime's own trail along its flight (`trail` along
the shot trace, both engines, twin exact), a road for the slime to follow.
It undid the squad: the trail is what the shots and the look call slime
(`squad::slime_trail`, 0.65), so the next bullets stopped on the squad's
own road and it drew as slime. On the held front at 10 s: 115 soldier hits
and 9 out of range against plain's 74 and 34, and 2 walkers killed in the
window against 86; over the 15 windows 579 against 1,095. Its code is
reverted; `../dead-ends.md` has the condition to try it again.

## The germ retune

The [readers](../gun-readers-2026-11-17/README.md) named the germ a
dominance check. The game designer (05:02Z) asked how far one germ round
spreads through plain C and the Sporer, then (05:40Z): "retune it now.
It's numbers, so it doesn't wait for the balance pass", with targets for
one cast, 20 s after: Sporer and Drifter 45-55% down, plain C, Sprawler
and Trunk 25-35%, Sleeper, Swarmer and Grudge 10-20%; "a lower base
potency plus per-tier germ_resist, using the trait the engine already has
... A re-check of 3 tiers on seed 0 is enough, with no census."

**Setup** (`raw/germ.sh`; the tuning runs' script is the same with keys):
the race world, seed 0, idle hand, plain rounds; one germ cast at 20 s on
the Fork lobe (363, 244; `boom=20,363,244,germ`); walkers at 40 s against
the same game with no cast.

**Before**: one round took 76% of C (25,772 to 6,204) and 80% of the
Sporer (27,472 to 5,482).

**Levers tried on C** (walkers at 40 s, against 25,772):

- The sick's upkeep (`sick_upkeep`, the toolkit's 60) is a cliff: 20 took
  65%, 12 59%, 10 54%, 9 39%, and 8, 7, 6, 4 or 2 nothing (the sick live
  out their sick seconds and recover).
- Fewer sick seconds (`sick_secs` 3) at 60 still took 69%; at an upkeep of
  10, 3 or 4.5 took nothing.
- A higher catch level: at 10, `sick_catch` x 2 took 49%, x 3 18%, x 4 3%.
- A resistance (`germ_resist`, the catch level times it) at 10: 11 took
  47%, 12.5 40%, 13.5 32.5%, 14 28%.

**What changed**:

- The germ round's sick pay 10 times the upkeep, not 60 (`RoundRules`'
  kit; the toolkit's own `KitRules` keeps 60).
- `BigParams::germ_resist` reaches the GPU (`Settings::germ_resist`, the
  `germ_resist` key; `write_params` sends the catch level times it, so the
  passes are the CPU's) and the tiers' keys.
- Shape C resists 13.5; the Sporer 1; the Drifter 10; the Swarmer and
  the Grudge 15. The Sprawler, Trunk and Sleeper keep C's (a tier's keys go
  on after the shape's).

**The re-check** (`raw/germ-tiers.sh`, defaults only, `tier=` for each
planet; `raw/germ-tiers.txt`). Walkers at 40 s:

| Planet's slime | Resistance | No germ | One germ | Down | Target |
|---|---|---|---|---|---|
| Drifter (planet 1) | 10 | 24,440 | 11,632 | 52.4% | 45-55% |
| Sporer (5) | 1 | 27,472 | 14,008 | 49.0% | 45-55% |
| Shape C, no tier | 13.5 | 25,772 | 17,403 | 32.5% | 25-35% |
| Sprawler (2) | 13.5 | 25,773 | 18,680 | 27.5% | 25-35% |
| Trunk (3) | 13.5 | 25,052 | 17,013 | 32.1% | 25-35% |
| Grudge (7) | 15 | 23,789 | 20,059 | 15.7% | 10-20% |
| Swarmer (4) | 15 | 23,491 | 20,494 | 12.8% | 10-20% |
| Sleeper (6) | 13.5 | 21,705 | 21,705 | 0.0% | 10-20% |

The Drifter first ran at 1 (56.6%, just over its band); resistance barely
moves it (56.2% at 3, 55.4% at 6), so it took 10 (52.4%, run with
`germ_resist=10` after its tier keys, the same settings as the default
since). Seven of eight are in their bands.

**The Sleeper** is under its band at any resistance: 0.1, 0.3, 1, 8, 12
and 15 all took under 0.1% at the new upkeep, against 26% at the toolkit's
60. Its sleeping crust pays no upkeep (`tiers.rs`: paying it while asleep
lost the bait counter), so the germ's starving reaches only the few awake.
Reaching 10-20% would take a new rule, such as a tier's own sick upkeep
(the rounds set the world's upkeep when they cast, so it would be the
round's times the tier's), not a number; left for the designer.

**The toolkit's thrown germ** (the command card's G) keeps its upkeep of
60 but meets C's resistance: a cast at 60 on C took 43.7% (14,504 at 40 s)
against 76.0% before. The world holds one contagion setting, set by the
first germ cast in a game (`squad::burst` and the GPU world's), so
whichever comes first, round or tool, sets the upkeep for the rest of it.

## Cost

- The Flare: nothing a step. Each burning flare queues one alarm-only
  burst every 0.2 s into the frame's batch (`World::bursts`, one read-back
  for every burst in the frame), at most 64 at once.
- The germ resistance: none; one multiply when the parameters are written.

## Tests (the germ)

- `mouldgpu`: `the_looks_match_the_cpu_for_one_tick` gains a resisted
  contagion dish (`germ_resist` 50): one GPU tick from the warmed CPU dish
  against one CPU tick, 0 of 250,000 walkers and counters off, the germ
  field exact (llvmpipe), and fewer walkers sick after the warm-up than on
  the same dish without it (191,823 against 194,182). A planted bug, the
  GPU sending the catch level alone, put 983 walkers and counters off and
  failed it. A resistance of 3, tried first, could not tell: 380 fewer
  sick over the 40-tick warm-up, about 10 a tick, under the test's bound
  of 250 walkers off.
- `tiers`: every tier's keys reach both engines alike
  (`the_gpu_takes_every_tier_as_the_cpu_does`,
  `each_tier_on_the_gpu_race_world_is_the_cpus`), the resistance among them.

## Not settled

- No window pits the Flare against the Swarmer or Sleeper planets; the
  card table's row rests on the clips and the tiers' counter.
- On C a flare near the squad can draw a `panic` charge (the hold at 30 s).
- The germ's numbers are seed 0 and one spot, as the designer asked; the
  balance pass may move them.
- Next in step D, per the plan: element, force; cut last.

## Files

- `raw/film.sh`, `raw/films.sh`: the clips; `raw/make_clips.py`: MP4s and
  counts; `raw/counts.json`: the counts.
- `raw/windows.sh`, `raw/windows_table.py`, `raw/windows.txt`: the paired
  windows.
- `raw/germ.sh`, `raw/germ.txt`: the germ before; `raw/germ-tiers.sh`,
  `raw/germ-tiers.txt`: the re-check.
- `page/index.html`, its clips and `armory-special.png`: the Artifact
  page's source.
