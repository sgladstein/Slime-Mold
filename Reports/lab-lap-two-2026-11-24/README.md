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

## Raw

`raw/pairs.txt` (planet, base, hand, keys), `raw/run2.sh` (the fourteen),
`raw/run3.sh` (the retests), `raw/m2.py` (the counts from logs and dumps),
`raw/nap.txt` (the Sleeper's keys), `raw/rig9.sh` (planet 9's rig).
