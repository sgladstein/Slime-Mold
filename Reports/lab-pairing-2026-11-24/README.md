# Lab: a lap-two pairing, the Swarmer that naps (2026-11-24)

The Lab lane's ninth prototype, on the lab branch `claude/project-thread-2o4cic`
(engine keys only, no new code; the dump of walkers awake and asleep is
`LAB_DUMP_LIFE`, code `3508d9d4`), not in the game. Brief from Direction
2026-11-24 06:31Z: ask Enemy which of planets 4, 5 and 8 reads weakest and
prototype one change for it; if all read well, take the lap-two pairing
(planet 12 as a Swarmer that also naps like the Sleeper), show one pairing
working in the lab and list which trait keys fight each other. Enemy
(06:32Z): planet 8 reads weakest, since it has no thing of its own and
rolls one of planets 5 to 7, so make planet 8 the pairing: one tier
carrying a second tier's trait.

Page: "Napping Swarmer" (source in `page/`, listed in `Reports/pages.md`).

## The pairing

The Swarmer (planet 4; card "Its surges charge your squad from far off, and
fast. Cut down to a quarter, all of it rushes you") with the Sleeper's
life cycle added as engine keys (`raw/nap.txt`): `sleep_rate=0.01
sleep_deposit=0.3 wake_scent=1 wake_alarm=0.3 alarm_relay=20
sleep_rest=600 sleep_death=0.0001`, the Sleeper's tier values with the
run's wake at an alarm of 0.3. The Swarmer's own keys stay (`surge_haste`
3, `alarm_weight` 3, `alarm_keep` 0.98, the last stand). Planet 4's map
stands in for planet 8's: planet 8 runs about 11 s wall a game second on
lavapipe, planet 4 about 3.

A card for it: **"It sleeps off every charge. Each shot wakes it, and
awake it charges."**

## Results

Planet 4, bank 3,000, lavapipe, `LAB_DUMP_LIFE` for walkers awake and
asleep a second (`raw/m4.py`).

**It wakes, charges, and sleeps again.** The run bot (`hand=greedy`, which
drains the biggest crystal and buys soldiers): of the paired slime's
walkers, 13% awake at landing (6,946 of 52,228), over half from 0:18 when
the squad's first shots land (85% at 0:21), 13% at 0:41 while the squad still fights at its
crystal, then waves again at 0:50 and 1:56. Seed 1: three waves in 3:00,
119 of 178 seconds with under 30% awake. Seed 2: two waves (0:21, 1:42),
37 of 180 seconds under 30% awake (that squad fights the body's middle
more). Off, every walker is awake all the time.

| Seed | Arm | Soldiers at 3:00 | Shots | Killed | Slime at 3:00 | Crystals left |
|---|---|---|---|---|---|---|
| 1 | Swarmer | 38 | 4,164 | 32,174 | 30,929 | 0.509 |
| 1 | naps | 43 | 2,569 | 25,912 | 25,296 | 0.460 |
| 2 | Swarmer | 37 | 3,017 | 42,453 | 60,208 | 0.727 |
| 2 | naps | 37 | 2,372 | 43,004 | 43,002 | 0.720 |

**A farm.** Seed 1, `hand=pumps` (4 soldiers pumping crystals, firing at
what comes): off, the surges found the farm and wiped the squad at 1:43
(10,254 food pumped); paired, the slime slept through the farm to 2:00
(13,494 pumped, 6 shots fired), woke at 2:04 after the squad's first shots at
its edge, and wiped it at 2:42.

So both traits read: the sleeping crust is pale and still, a shot wakes a
wave of orange that charges, and the wave falls back asleep in about 20
seconds once the alarm fades. As tuned here the pair is easier than the
Swarmer alone for the bot (5 more soldiers on seed 1, level on seed 2) and
the slime ends smaller (likely because a sleeper dies slowly and does
not breed): planet 8
would want the Swarmer's numbers raised, Enemy's call by play.

**Frame cost:** no new code; the pair adds the life cycle's passes, which
the tiers already pay (at most 0.74 ms a frame at 100,000 walkers on the
GTX 1070, [the tiers](../../wiki/world/mould-tiers.md)).

## Trait keys that fight each other

From the code (engine.wgsl `sense` and `fate`, `raids.rs`), with Enemy's
list of 06:32Z:

1. **Asleep walkers do not move.** A dormant walker skips `sense`, so a
   surge, a raid and the last stand move only the awake ones. The last
   stand counts every living walker (`raids.rs`, the mould against its
   peak), sleepers included, and fires once: with half the body asleep, half
   of it rushes. A sleeper also pays no upkeep, does not eat, does not
   breed, and dies at `sleep_death`.
2. **One alarm field.** `wake_alarm` (Sleeper), `spore_alarm` (Sporer),
   `alarm_weight` (Swarmer and Grudge steering) and `fire_alarm` (Sprawler,
   fire lays alarm) all read or write the same alarm. `alarm_relay` (the
   Sleeper's 20) has every waking walker lay alarm, so one shot spreads.
   Paired with the Sporer, one shot would wake a wave that is itself over
   `spore_alarm` and bursts into spores; with the Sprawler's `fire_alarm`, a
   fire would wake a sleeper.
3. **One life-cycle timer.** Fruiting is checked before dozing on the same
   timer (`sense`), so a Sporer that naps fruits first, and a dormant walker
   never fruits.
4. **One scar field.** The Grudge's `scar_weight` and scar tissue
   (`LAB_TOUGH`) both read the scar.
5. **One trail per slime.** `keep` and `sensor_offset` are one slime's walk
   (the Sprawler's threads set them), so a pairing has to pick one walk.
6. **Lab traits on the same row.** `LAB_DREAM` (the Sleeper grows asleep)
   and `LAB_TOUGH` share the lab's one key word, so the lab branch can show
   only one at a time; in the game each would be its own setting.

## Recommendation

**Make planet 8 a pairing, and this one works as a first:** the Swarmer that
naps. It reads in motion and in the counts, and it is just keys, so a
paired tier is `Tier::settings` of both plus both run rows. Before it
lands:

- Enemy: the pair's numbers by play (as here it is easier than the Swarmer
  alone on seed 1), on planet 8's own map; two seeds here.
- Direction: which pairs make lap two. The list above says which pairs
  multiply (the Sleeper's relay with the Sporer's burst) and which cancel
  (two walks).
- The Sleeper's growth asleep (`LAB_DREAM`) would carry over to this pair
  and make its naps cost the player too (untested together).

## Not tried

Planet 8's own map; a Sporer that naps (the burst chain in item 2); the
Grudge with the Sleeper (would the scar wake it?).

## Raw

`raw/run4.sh` (the run: env, hand, frames; engine keys after the name and
env), `raw/nap.txt` (the Sleeper's keys added), `raw/m4.py` (waves from the
dump's `life.txt`).
