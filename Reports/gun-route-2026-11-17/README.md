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

Status: **the Flare is built and merged (#412); the Road was built and
dropped; the germ is retuned per planet, twice: the second pass, after a
GPU bug that planted a second germ at the map's middle, adds a frailty
per planet and makes the Sleeper germ-proof asleep and weak after a
Flare.**

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
Sporer (27,472 to 5,482). Both numbers carried the bug below; without
it the same round took 48% of C (13,512).

### The first pass (merged in #412), with a bug in it

Levers tried on C (walkers at 40 s, against 25,772; every one of these
runs carried the bug, so they read high):

- The sick's upkeep (`sick_upkeep`, the toolkit's 60) is a cliff: 20 took
  65%, 12 59%, 10 54%, 9 39%, and 8, 7, 6, 4 or 2 nothing (the sick live
  out their sick seconds and recover).
- Fewer sick seconds (`sick_secs` 3) at 60 still took 69%; at an upkeep of
  10, 3 or 4.5 took nothing.
- A higher catch level: at 10, `sick_catch` x 2 took 49%, x 3 18%, x 4 3%.
- A resistance (`germ_resist`, the catch level times it) at 10: 11 took
  47%, 12.5 40%, 13.5 32.5%, 14 28%.

It set the round's sick pay to 10 times the upkeep, not 60 (`RoundRules`'
kit; the toolkit's own `KitRules` keeps 60), sent `BigParams::germ_resist`
to the GPU (`write_params` sends the catch level times it) and gave shape
C 13.5, the Sporer 1, the Drifter 10, the Swarmer and Grudge 15. Its
re-check (`raw/germ-tiers.txt`) had seven of eight planets in their bands
and the Sleeper at 0%.

### The bug

The designer then asked for the Sleeper to be germ-proof asleep and weak
to a Flare-then-germ combo (06:36Z), with a per-tier sick upkeep (06:41Z,
below). A germ cast deep in the Sleeper's sleeping crust (166, 283),
which cannot catch it, still took 22-24% of the body
(`raw/germ-retune-2.txt`, part 1). The cause: the GPU engine's dish has a
timed seeding (`Settings::seed_at` 4 s, `seed_r` 40) that infects a disc
at the map's middle once contagion is on and the step count passes
`seed_at` x 120. The Larder world turns contagion on only when a germ
lands, and counts 30 ticks a game second, so every game's first germ also
planted a second outbreak at the map's middle, whatever it hit: at once
if it landed after 16 s, at 16 s if before. The CPU
race world seeds nothing, so the twin tests never saw it. Fix: the GPU
world marks that seeding done when a germ turns contagion on
(`GpuWorld::burst_passes`). With it off (`seed_r=0`, part 2), the deep
germ took nothing, C at the first pass's 13.5 lost 20% not 32.5%, and the
untuned germ 48% not 76%. Every germ figure before this fix, the
readers' films among them, read high.

### The second pass, on the fixed build

- **The frailty**, the designer's per-tier sick upkeep (06:41Z: "Go
  ahead ... with a Sleeper factor of about 6, tuned so flare then germ
  lands at 20-40% and germ on a sleeping crust stays near 0"):
  `BigParams::germ_upkeep`, the sick pay `sick_upkeep` times it; the GPU
  sends the product (`write_params`), so the passes are the CPU's. A tier
  key (`germ_upkeep`) on both engines.
- **The Sleeper**: its woken walkers are full, so at the round's upkeep a
  wave that caught the germ lived out its sick seconds (06:36Z check,
  `raw/flare-germ-1.txt`, on the bug build: the combo 0.1%). Frailty 6
  (the sick pay 60, the toolkit's) with resistance 1: the combo took
  28.2% at 46 s (15,445 against the flare alone's 21,526); 25% at
  resistance 2, 20% at 4, 14% at 6, 11% at 10. On the final build
  (`raw/flare-germ.txt`) a germ 4 s after the flare took 31.8%, 6 s
  28.2% and 8 s 24.0%; the same germ with no flare 0%. A germ on its sleeping
  crust, deep or on the Fork lobe, takes nothing at any of them
  (21,705 against 21,705 at 40 s).
- **The Sporer**: resistance 1 and frailty 1 took 33%; frailty 2 39%, 6
  47%, 12 53%. It takes 6.
- **The Drifter**: resistance 1 took 37%; frailty 6 or 12 43.5%, at
  resistance 0.25 to 1 alike. Its droplet bodies are all one germ
  reaches, so it sits just under its 45% floor; it takes 1 and 6.
- **Shape C**: resistance 1 36%, 3 35%, 6 34%, 8 33%, 10 31.6%, 13.5 20%.
  It takes 10; the Sprawler and Trunk keep it.
- **The Swarmer and Grudge** stay at 15 (12% each; the Swarmer at 6 38%).

**The re-check** (`raw/germ-tiers.sh` on the final build, defaults only,
`tier=` for each planet; `raw/germ-tiers-2.txt`). Walkers at 40 s:

| Planet's slime | Resistance | Frailty | No germ | One germ | Down | Target |
|---|---|---|---|---|---|---|
| Drifter (planet 1) | 1 | 6 | 24,440 | 13,806 | 43.5% | 45-55% |
| Sporer (5) | 1 | 6 | 27,472 | 14,656 | 46.7% | 45-55% |
| Shape C, no tier | 10 | 1 | 25,772 | 17,628 | 31.6% | 25-35% |
| Sprawler (2) | 10 | 1 | 25,773 | 18,953 | 26.5% | 25-35% |
| Trunk (3) | 10 | 1 | 25,052 | 17,137 | 31.6% | 25-35% |
| Swarmer (4) | 15 | 1 | 23,491 | 20,603 | 12.3% | 10-20% |
| Grudge (7) | 15 | 1 | 23,789 | 20,932 | 12.0% | 10-20% |
| Sleeper (6), asleep | 1 | 6 | 21,705 | 21,705 | 0.0% | near 0 |
| Sleeper, Flare at 20 s then germ at 26 s (at 46 s) | 1 | 6 | 21,526 (flare alone) | 15,445 | 28.2% | 20-40% |

Six of the other seven slimes are in their bands, and the Sleeper is as
the designer asked:
the germ does nothing to it asleep and takes 28% after a Flare. The
Drifter is 1.5 points under its floor and no germ setting moves it
further; a germ the Drifter fears more would need a rule of its own, not
a number. Against the first pass: C 32.5% to 31.6%, Sporer 49.0% to
46.7%, Drifter 52.4% to 43.5% (the first pass's figures carried the
planted middle), Swarmer 12.8% to 12.3%, Grudge 15.7% to 12.0%.

**The toolkit's thrown germ** (the command card's G) keeps its upkeep of
60, times the planet's frailty, and meets its resistance (`raw/germ-tool.txt`):
on C it took 38.6% (15,811 at 40 s; the untuned germ with no bug
47.6%, 13,512); on the frail Sporer, whose sick pay 360, 54.0% (12,627). A round deep
in the Sleeper's sleeping crust takes nothing (21,704). The world holds
one contagion setting, set by the first germ cast in a game
(`squad::burst` and the GPU world's), so whichever comes first, round or
tool, sets the upkeep for the rest of it.

## Cost

- The Flare: nothing a step. Each burning flare queues one alarm-only
  burst every 0.2 s into the frame's batch (`World::bursts`, one read-back
  for every burst in the frame), at most 64 at once.
- The germ resistance and frailty: none; one multiply each when the
  parameters are written. The seeding fix: none (a flag set once).

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
  `each_tier_on_the_gpu_race_world_is_the_cpus`), the resistance and the
  frailty among them.
- `gpu::larder_world::a_frail_mould_dies_of_the_germ_on_both_engines`: a
  kit germ at the race world's home (seed 5, 2,000 walkers) at the rounds'
  upkeep of 10, frailty 1 and 6: 666 and 479 living after 20 s on both
  engines, to the walker. A planted bug, the GPU sending the upkeep alone,
  left the frail GPU mould at 666 and failed it.
- `gpu::larder_world::a_germ_infects_only_where_it_lands`: a kit germ cast
  at 15 s into the race world's empty corner (scale 2, 25,000 walkers),
  with the dish's timed seeding set over the whole map: 0 sick on both
  engines 2 s later. The old code planted back made 26,443 sick on the
  GPU and failed it. (Cast at 5 s it passed even with the bug: the world
  counts 30 ticks a game second against the 120 fine steps `seed_at` is
  written in, so the seeding struck from 16 s.)

## Not settled

- No window pits the Flare against the Swarmer or Sleeper planets; the
  card table's row rests on the clips and the tiers' counter.
- On C a flare near the squad can draw a `panic` charge (the hold at 30 s).
- The germ's numbers are seed 0 and one spot, as the designer asked; the
  balance pass may move them. The Drifter sits at 43.5%, under its 45-55%.
- The readers' germ films (the Fork 24,766 to 7,279 in 10 s) carried the
  bug and are not re-run; their report has a correction note.
- Next in step D, per the plan: element, force; cut last.

## Files

- `raw/film.sh`, `raw/films.sh`: the clips; `raw/make_clips.py`: MP4s and
  counts; `raw/counts.json`: the counts.
- `raw/windows.sh`, `raw/windows_table.py`, `raw/windows.txt`: the paired
  windows.
- `raw/germ.sh`, `raw/germ.txt`: the germ before; `raw/germ-tiers.sh`,
  `raw/germ-tiers.txt`: the first pass's re-check (with the bug),
  `raw/germ-tiers-2.txt`: the second's.
- `raw/germ-retune-2.txt`: the bug's runs and the second pass's tuning.
- `raw/flare-germ.sh`: the Sleeper's combo; `raw/flare-germ-1.txt` on the
  first pass's build, `raw/flare-germ.txt` on the final one.
- `raw/germ-tool.txt`: the toolkit's thrown germ on the final build.
- `page/index.html`, its clips and `armory-special.png`: the Artifact
  page's source.
