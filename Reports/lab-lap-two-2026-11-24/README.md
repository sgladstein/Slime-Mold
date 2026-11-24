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
counted, not a free win; checked by eye only. (Round two, below: they are
counted, and the fall is real.)

## Smoke checks, round two (added 11-24, Direction 14:26Z)

Run seed 1, greedy squad of 11 unless named, 2:00, on the GTX 1070
(Engine; `raw/pc-lap2-fix.ps1` and `raw/pc-lap2-fix2.ps1`, outputs beside
them).

**1. What wipes 12 and 15 alone: the strikers.** Each planet was played
as it is and with one thing off: no mini-bosses (`set=boss_first=9999`),
no tier trait, slow surges (`surge=2400`), no strikers (`set=strikers=0`).
Every arm wipes inside a minute (12 at 49 to 55 s, 15 at 38 to 43 s)
except no strikers, which lives to 2:00 with the squad grown to 24 on
both. Lap one's planets 4 and 7 on the same hand never wipe (29 soldiers
at 2:00). Why: a lap-two slime starts bigger than its crystals hold and
sheds about half in its first 30 s (12: 126k to 69k; 15: 234k to 124k),
which puts it under the strikers' trigger (60% of its peak, the peak
taken at the start) within seconds, so they condense on the squad from
the first contact. Lap one's slimes don't start oversized.

| Planet, arm | 1:00 | 1:30 | End |
|---|---|---|---|
| 12, `strikers_below` 0.5 | | | wiped 0:51 |
| 12, size 0.5 | | | wiped 0:50 |
| 12, **`strikers_below` 0.45** | 8 soldiers | 13 soldiers, slime 67k | lives, 425 strikers taken |
| 15, `strikers_below` 0.5 | | | wiped 0:41 |
| 15, size 0.5 | | | wiped 0:58 |
| 15, **`strikers_below` 0.45** | 5 soldiers | 8 soldiers, slime 10k | lives, 50 strikers taken |
| 15 paired, size 0.5 | 10 soldiers, slime 41k | 6 soldiers, slime 37k | lives |

**First number: `set=strikers_below=0.45` on lap two** (the default is
0.6). Both planets then pass the smoke rule; 0.5 does not, nor does a
smaller slime alone. The cleaner fix is for Enemy's: take the strikers'
peak once the slime has settled (say from 30 s) rather than at the start,
which would leave lap one alone and need no lap-two number. On 15 alone
the slime still starves to 10k by 1:30 even so, as it does with strikers
off (14k at 2:00): 15's crystals hold far less than 0.7 of the size rule.

**2. Pump smell seen within a minute: on 14, at 40.** The pump placed at
the Sleeper's edge (645,234), squad idle:

| Planet 14 | Pump | Harvested |
|---|---|---|
| base | still standing at 1:30 | 582 |
| `pump_smell` 20 | worn down by 1:30 | 232 |
| **`pump_smell` 40** | hp 74 of 200 at 0:30, 0 at 0:45, worn down by 1:00 | 232 |

At 40 the sleeping slime gets up and takes the pump inside 45 s, where
without it the pump works on to 1:30. `raw/pairs.txt` now has 14 at 40
with the pump there. **Planet 10 shows nothing either way**: the
Sprawler's threads take a pump inside the threads (275,340) by 1:00 and
one outside them (214,680) by 0:30, with and without pump smell, the
same harvest each time. Pump smell is redundant on the Sprawler; 10
needs a different second trait.

**3. Planet 15's count is honest.** Set walkers stay in the count: the
engine's `count` pass (`engine.wgsl`) counts every slot with energy and a
life state other than dead, and setting is a mark on the cell, not a life
state. The life rows agree (`LAB_DUMP_LIFE`, paired, sec / walker /
spore / dormant / dead): 1 242,393 0 0 0; 30 105,913 399 0 0; 60 16,333 0
0 0; 119 15,944 0 0 0. The slime really starves to 16k, so the goal line's
count needs no change; at 0.7 the paired 15 is close to a free win. At
size 0.5 the paired slime holds about 37 to 41k through 1:30 and the
squad lives, so **0.5 is 15's first size** where it is paired. Not run:
15 alone at size 0.5 with `strikers_below` 0.45 together.

## Planet 10's trait and the strikers' settled peak (added 11-24, Direction 15:39Z)

Greedy squad of 11, bank 3,000, on the GTX 1070 (Engine; 35 games in
`raw/pc-lap2-settle.ps1` and `raw/pc-lap2-settle2.ps1`, outputs beside
them).

**Planet 10's second trait: the Drifter's spore bursts** (the same keys as
planet 9's pair, `raw/pairs.txt`). The Sprawler's threads already reach
the squad and the pumps from far off, so pump smell added nothing. Bursts
add something threads don't do: a thread the squad shoots scatters spores
that start new slime away from the body. Life rows (`LAB_DUMP_LIFE`,
spores in flight), 2:00 at 0.7 of the size rule:

| Run seed | Spores in flight, 0:15 / 0:30 / 0:45 | 1:00 | End |
|---|---|---|---|
| 1, alone | 31 / 17 / 71 | 14 soldiers, slime 74k | wiped 1:29 (a crowd hunter) |
| 1, **paired** | 381 / 731 / 1,687 | 14 soldiers, slime 85k | wiped 1:17 |
| 2, alone | 31 / 35 / 40 | 15 soldiers, slime 100k | 35 soldiers at 2:00 |
| 2, **paired** | 708 / 1,619 / 1,020 | 15 soldiers, slime 108k | 35 soldiers at 2:00 |

Seen within a minute: 10 to 40 times the spores in flight, and the slime
about 10% bigger by 1:00. The squad lasts past 1:00 on both seeds; seed 1
dies 12 s sooner paired, so the trait costs something and is no free win.
**First number: planet 9's spore keys as they are.**

**The strikers' settled peak** (rule key `set=strikers_settle=S`, code
`8e94466e` and `e7b72d88`; 0 is main's rule). The strikers count their
own peak only once the slime has settled: from the first second its mass
is 1% over its lowest so far, or at `S` game seconds at the latest, with
no strikers before. What was killed by then counts back in, up to the
game's peak, so only what the slime shed by itself is left out. The first
version left the kills out, and planet 4 then lost its strikers. Planet 4
falls under the guns from 53,505 to 35,410 walkers by 1:00, with 19,623
killed, and a peak taken at 35,410 kept the strikers off for 3:00 (main:
3,334 taken). The second version keeps that peak at the game's.

With `S` 60 and no `strikers_below` patch, lap two at 0.7, 2:00:

| Planet, run seed | As it is | Settled peak (settled at, its peak) |
|---|---|---|
| 12, 1 | wiped 0:49 | **lives**: 24 soldiers at 2:00, no strikers formed (0:30, 69k) |
| 15, 1 | wiped 0:39 | **lives**: 24 soldiers at 2:00, 2 walkers taken (1:00, 62k) |
| 12, 2 | wiped 0:42 | **lives**: 3 soldiers at 1:00, 6 at 2:00 (0:26, 81k) |
| 15, 2 | wiped 1:52 by two mortars, no strikers | the same (0:40) |

Lap one, run seed 1, 3:00, main against the settled peak:

| Planet | Settled at | Against main |
|---|---|---|
| 2, 5, 6, 7, 8 | 0:04 to 0:40 (6 wiped at 0:43 by a hunter in both arms) | the same game, number for number |
| 3 | 1:00 (peak 43,361, main's) | strikers from 1:19 in both, 236 taken; at 3:00 1,135 against 1,137 taken, 34 soldiers both |
| 4 | 1:00 (peak 53,513, main's) | strikers from 1:16 in both; at 3:00 3,392 against 3,334 taken, 28 against 31 soldiers |

On 3 and 4 the peak the strikers read is main's own, so the gap after
the first strikers is run-to-run noise in their condensing; every other
planet plays the same game as on main. **The settled peak replaces round
two's `strikers_below` 0.45**, and lap one keeps its strikers.

**For Enemy**: `raw/strikers-settle.patch` applies to main as of 11-24
16:50Z (`git apply`). It changes `strikers.rs` (the rule, the settled
peak, with the why in comments), `rules.rs` (the `strikers_settle` key)
and `tests.rs` (`the_strikers_peak_waits_for_the_slime_to_settle`, which
passes). The default stays 0. Lap two would turn it on at 60, or every
planet could, since lap one plays the same with it. The lab branch's
headless strikers line also prints the settle second and peak (Engine's
`headless.rs`, not in the patch).

## Raw

`raw/pairs.txt` (planet, base, hand, keys), `raw/run2.sh` (the fourteen),
`raw/run3.sh` (the retests), `raw/m2.py` (the counts from logs and dumps),
`raw/nap.txt` (the Sleeper's keys), `raw/rig9.sh` (planet 9's rig), `raw/rig.sh` (any of 9 to 15), `raw/pc-size9.ps1`, `raw/pc-lap2-smoke.ps1`, `raw/pc-lap2-fix.ps1`, `raw/pc-lap2-fix2.ps1`, `raw/pc-lap2-settle.ps1` and `raw/pc-lap2-settle2.ps1` (the PC batches) with their outputs (the fix batch's is Engine's summary; its logs stay on Scott's PC), and `raw/strikers-settle.patch` (Enemy's patch).
