# Lab: lap two's seven pairings, planets 9 to 15 (2026-11-24)

The Lab lane's tenth job, on the lab branch `claude/project-thread-2o4cic`,
engine and rule keys only (no new code), not in the game. Brief from
Direction 2026-11-24 06:53Z, for the run-length card (12 or 16 planets):
draft the seven pairings for planets 9 to 15, one tier each with a second
trait that does not fight it (by the keys that fight, in [the pairing report](../lab-pairing-2026-11-24/README.md)),
smoke-test each for about two minutes (does it run, does it do the named
thing, is it not a free win), one line each, one page in all.

Page: "Lap Two Pairings" (source in `page/`, listed in `Reports/pages.md`).
Planet 8 is already the Swarmer that naps (Enemy, 06:53Z).

## How each was tested

Each base tier on its own planet's map (run seed 1, bank 3,000, 2:00,
lavapipe), alone and with the second trait's keys added (`raw/pairs.txt`;
`raw/run2.sh`, `raw/run3.sh`, `raw/m2.py`). Lap two's own sizes and
clocks are not modelled. One seed each: a smoke test, not a balance.

## The seven

| Planet | Pair | What it does | Ran | Named thing, alone against paired | Free win? |
|---|---|---|---|---|---|
| 9 | Drifter + the Sporer's bursts | It comes for your pumps, and struck it scatters spores that start new slime | yes | spores in flight at once, most: 45 against 970 | no: 45 soldiers both, slime 10,712 against 11,816 |
| 10 | Sprawler + the Drifter's pump smell | It smells your soldiers and your pumps from far off, and eats the pumps | yes | `pumps` hand: pumps standing at 2:00, 2 of 4 (153, 106 hp) against 0 of 4; food 1,833 against 454 | no: harder (squad 4 against 3) |
| 11 | Trunk + the Sleeper's naps | It sleeps behind its walls until you shoot, then all of it sets against you | yes | awake 100% always against 13% at landing, 89% by 0:30 after the first shots | no: 30 soldiers both; killed 7,795 against 6,299 |
| 12 | Swarmer + the Grudge's scar | Its surges come back where you hurt it | yes | scar's peak 0 against 4,319; walkers on scar over 100 at 2:00, 0 against 215 | no: 30 soldiers against 28 |
| 13 | Sporer + the Swarmer's charge (alarm pull 3, surge haste, last stand) | Struck it scatters spores, and swarms at the wound | yes | killed 11,748 against 15,635 (more of it comes to the wound); spores most 7,398 against 8,468; the last stand not reached in 2:00 (82,000 walkers left) | no: 34 soldiers against 30 |
| 14 | Sleeper + the Drifter's pump smell | It wakes to the smell of your pumps | yes | a pump at its edge (`pump=312,456`, squad idle far off): awake at 0:10 to 0:20, 12% against 51-56%, asleep again once the pump fell | no: the pump worn down in both |
| 15 | Grudge + the Trunk's setting and walls | Where you hurt it, it sets in amber and comes back to set again | yes | killed 7,123 against 3,865; walkers on scar over 100 at 2:00, 432 against 854 | no: 28 soldiers against 29, half the kills |

Tried and dropped (no visible change in 2:00): the Trunk with the
Sprawler's soldier smell (idle squad, 4 soldiers both, killed 258 against
336), with the Drifter's pump smell (pumps at 2:00 all whole in both), and
with the Grudge's scar (scar peak 91 against 1,914, but 21 walkers on it);
the Swarmer with the soldier smell (idle squad wiped 1:37 against 1:41).
The Trunk sets and its cords are slow, so pulls barely move it.

## What fights, and how each pair avoids it

- **Sleepers don't move** in surges, raids or the last stand: no sleeper is
  paired with the Swarmer's charge (8 is, by Enemy's choice; 11's Trunk has
  no surge trait).
- **One alarm field:** no sleeper with the Sporer's burst (one shot would
  burst the whole woken wave) and no sleeper with the Sprawler's fire. 13
  adds the Swarmer's alarm pull to the Sporer on purpose: it swarms at the
  wound and bursts there.
- **One life-cycle timer:** no Sporer naps.
- **One scar field:** only 12 and 15 use it, the Grudge's own scar.
- **One walk per slime:** no pair takes the Sprawler's walk (`keep`,
  `sensor_offset`); 10 is the Sprawler keeping its own.
- **The same trigger twice:** the Trunk's setting and the Sporer's burst
  both answer a shot; not paired.
- 14 reads the scent the Sleeper wakes on: the pump smell lands in it, so
  a pump near it wakes it. That is the named thing, not a fight.

## Recommendation

Use the seven as lap two's first draft; each runs, does its thing in two
minutes and is no free win on one seed. Enemy sets each pair's numbers by
play on lap two's sizes. The traits that read weakest in a pair: 13's last
stand (not reached in 2:00) and 11's naps (it wakes at the first shot and
stays up).

## The rig for planet 9 (added 11-24, Direction 12:08Z)

Ready in case the run goes to 12 or 16 planets. On the lab branch,
`LAB_LAP2=1` lets `run=SEED:9` (to `:15`) through: planet 9 is levelgen's
own map for depth 9, at planet 8's size (the map sizes stop at 8), with
the Drifter as its tier and every per-planet row (raids, mini-bosses,
surges, origin) planet 8's, since the run's tables clamp at 8. Planets 10
to 15 take the next tiers in the same order; their second trait's keys
come from `raw/pairs.txt`. `raw/rig9.sh OUT SEED [HAND] [pair|base] [keys]`
runs it with the Sporer's bursts added (`pair`) or not (`base`); lap two's
own numbers go on the end as keys (`set=raid_first=...`, `capacity=...`).

Smoke test, run seed 1, `hand=greedy`, 1:00 (one seed): it runs; the
slime starts at 333,000 to 348,000 walkers. The Drifter alone: 5 of 11
soldiers left, 13,958 killed. With the bursts: 10 left, 6,808 killed,
the slime 273,808 against 240,556 (likely because a struck
walker bursts into spores that start new slime; not checked, and one
seed). Lap two's sizes are Direction's to set; planet 9 at planet
8's size is a big slime for a first lap-two planet.

## Lap two's first sizes and rigs (added 11-24, Direction 13:09Z)

**The rig.** `raw/rig.sh OUT PLANET SEED [HAND] [pair|base] [keys]` plays
any of planets 9 to 15 on the lab branch (`LAB_LAP2=1`, code `9115723b`):
levelgen's own map for the depth at planet 8's size, the ladder's tier
again in order (9 Drifter, 10 Sprawler, 11 Trunk, 12 Swarmer, 13 Sporer,
14 Sleeper, 15 Grudge), the second trait's keys from `raw/pairs.txt`, and
every per-planet row (raids, mini-bosses, surges, origin) planet 8's. The
slime's size is the game's own rule (a fifth more a planet, times the
area, the seed's roll and the tier's factor) times `LAB_LAP2_SIZE`
(code `bd836d45`).

**Planet 9's size: 0.7 of the rule.** At 1.0 planet 9 starts at twice
planet 8 (330,000 walkers against 173,000 on seed 1), because the rule's
depth factor runs on planet 8's area. The run bot (`hand=bot`, bank
3,000) on run seeds 1 to 3, planet 8 against planet 9 at three sizes,
on the GTX 1070 (Engine; `raw/pc-size9.ps1`, `raw/pc-size9-output.txt`):

| Seed | Planet 8 | 9 at 0.55 | 9 at 0.65 | 9 at 0.8 |
|---|---|---|---|---|
| 1 | 173k, wiped 7:02 | 187k, wiped 5:33 | 217k, wiped 6:03 | 278k, wiped 5:33 |
| 2 | 169k, wiped 5:51 | 195k, wiped 3:51 | 231k, wiped 7:21 | 281k, wiped 4:28 |
| 3 | 208k, wiped 2:54 | 172k, **won 4:29** | 197k, wiped 3:46 | 238k, wiped 2:01 |

(start walkers, and how the game ended.) The bot loses planet 8 itself
on all three, so the measure is how long the squad lasts: planet 8 5:16
on average, planet 9 at 0.65 5:43, at 0.8 4:01. 0.65 costs about what
planet 8 does and 0.8 clearly more, so **0.7** is the first number: a bit
more than planet 8, not a jump. Caveats: three seeds; `hand=bot` is a
harsh yardstick (the league's bot, with the shop, won planet 8 on 6 of 6
that day); Engine found a run-bot pick bug that day (it went for the
biggest crystal under the whole body) that may touch these games.

**The same rule for 10 to 15, a first guess with no census:** 0.7 on
every lap-two planet, so each is a fifth of planet 1's more than the one
before, as on lap one. Smoke tests at 0.7, run seed 1, 1:00, alone and
paired (pumps hand on 10 and 14, else greedy; GTX 1070,
`raw/pc-lap2-smoke.ps1`, `raw/pc-lap2-smoke-output.txt`):

| Planet | Starts (alone) | Ran | 1:00, alone against paired | Flag |
|---|---|---|---|---|
| 9 Drifter + spores | 226k | yes | 5 against 11 soldiers, slime 170k against 205k | |
| 10 Sprawler + pump smell | 63k | yes | the same (4 soldiers both) | pair not seen |
| 11 Trunk + naps | 141k | yes | 11 against 5 soldiers, slime 68k against 205k | |
| 12 Swarmer + scar | 126k | yes | wiped 0:52 against 0:48 | ends under 1:00 |
| 13 Sporer + charge | 279k | yes | 13 against 11 soldiers, 10,577 against 12,076 killed | |
| 14 Sleeper + pump smell | 284k | yes | identical (no shot fired) | pair not seen |
| 15 Grudge + setting | 234k | yes | wiped 0:39 against 9 alive, slime 14k | ends under 1:00 alone |

All seven run. Flags for Enemy before building: 12 (both arms) and 15
alone wipe the greedy squad of 11 in under a minute at planet 8's rows;
greedy walks into the body, so a planet that ends that fast for it may
still be fair for a player's army, but it breaks the smoke rule. 10 and
14 showed nothing in 1:00: the pumps never stood at the slime's edge (14
fired no shot), so it is untested here, not failed (the planet 2 and 6
smoke tests above did show them). On 15 the mould count falls to 14k by
1:00 with the Trunk's setting, likely because set walkers are not
counted, not a free win; checked by eye only.

## Raw

`raw/pairs.txt` (planet, base, hand, keys), `raw/run2.sh` (the fourteen),
`raw/run3.sh` (the retests), `raw/m2.py` (the counts from logs and dumps),
`raw/nap.txt` (the Sleeper's keys), `raw/rig9.sh` (planet 9's rig), `raw/rig.sh` (any of 9 to 15), `raw/pc-size9.ps1` and `raw/pc-lap2-smoke.ps1` (the PC batches) with their outputs.
