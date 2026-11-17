# Readers: step D of the gun-card plan, the second family

2026-11-17. Step D of the [gun-system review](../gun-system-review-2026-11-16/README.md)
("bullet families by what they change in the simulation"), after the
[Mind bullets](../gun-mind-2026-11-16/README.md). The plan's readers: "the
payoff bullets: quiet setup, loud payoff. Shatter kills sleeping slime
connected to the hit; Detonate pops marked walkers wherever they have flowed
... A reader fires as a weak plain shot when its state isn't there, and uses
the state up when it is." The owner, 03:07Z: "Continue without my guesses. I
am busy", so this step went ahead without the Mind quiz's answer.

Page: [Reader Bullets](https://claude.ai/artifact/UR2xBjHKmd7mVHdookdxsQ) (source in
`page/`): the paired clips with their counts, the paired windows in play,
and the armory's tab.

Status: **built, for merge; the 1070 timing is booked.** Two readers,
Shatter and Detonate, in both engines, exact twins.

## What was built

- **The read** (`BigDish::read_strike`, the CPU; the GPU's `readkill` pass,
  `Engine::read_pass`): every living ordinary or dormant walker within the
  reach of the hit, in the state the reader reads, takes a blow as a
  strike's (a lump's armoured member is hurt and not counted, a condensed
  one breaks and is counted) and, if it dies, the trail on its cell is
  cleared. Its kills count on the strike's counter, so the game counts them
  as a kill's, not fire's. With none of the state in reach it kills
  nothing; the bullet's own hit still kills where it lands, as every
  bullet's does, which is the plan's "weak plain shot".
- **Shatter** (`READ_SLEEP`) reads sleep from gas: dormant with gas left
  (`gassed`, the GPU's gas words). Its reach is `read_r`, 12 dish cells, a
  gas hit's own (`KitRules::gas_r`).
- **Detonate** (`READ_SICK`) reads the germ's sickness: sick past the
  immune count. The germ is this family's mark, so no mark card was built:
  it already spreads along the veins from its hit and shows. Its reach is
  `detonate_r`, 24 dish cells, for the sickness that has spread.
- **The cards** (`Card::Shatter`, `Detonate`; `Shell::Shatter` and on):
  payloads, cost 2; Shatter tier 2, sold from planet 2 (gas is sold from the
  start), Detonate tier 3, sold with the germ from planet 4 (`shells_at`).
  Each takes its set's shape and `Wide` like any payload. The burst carries
  `Burst::read`, after the strike and crater in both engines. `set=read_r=`
  and `set=detonate_r=` tune them. The rounds count reader casts in their own
  slot (`Rounds::cast[5]`, printed last on the rounds line).
- **Look**: icons a sleeper's crescent moon split by a crack, and a blast
  with the germ's two spots; the bullet's light a paler flash of what each
  reads (the gas's violet and the germ's green toward white). What you see
  of the kill is the simulation's: the sleeping slime is gone.
- **The armory**: the readers share the Mind bullets' tab, renamed "Special
  bullets" (`Shell::special`; standard set only), five rows on one page.
  LOADOUTS: `shatter`, `detonate`, `gas-shatter`, `germ-detonate`.
- **Films**: `boom=T,X,Y,CARDS` casts those cards at T
  (`Rounds::cast_cards_at`), so one film sets a state with one card and
  reads it with another seconds later; the boom line counts gassed walkers
  too. With a germ cast, `watch=` counts the sick walkers within 2R.

## Tests (both engines exact)

- `big::tests::readers_kill_the_state_they_read_and_nothing_else` (CPU):
  from one warmed dish, a disc of 10 gassed: Shatter with a reach of 20
  takes all 321 sleepers and no one else and clears their cells' trail;
  Detonate there takes none. A disc infected, a quarter of it set immune:
  Detonate takes the 152 sick and spares the immune; Shatter takes none.
- `rounds::tests::a_reader_casts_its_read_and_kills_only_that_state`: the
  burst, its reach with `Wide`, the slot; a shatter alone on the race world
  kills nothing, gas then shatter at one point kills all 1,461 gassed.
- `mouldgpu`: `readers_match_the_cpu` (a contagion dish with sick and
  immune walkers, a disc gassed: Shatter 265 killed on both, Detonate 1,449
  on both, 0 states, 0 cell counts off, the trail equal, a tick on 0
  walkers off; planted bugs caught: a GPU reach x 0.6 killed 207 of 265 and
  6 of 1,449, a GPU Detonate that read sleep killed 0 of 1,449), and
  `readers_count_their_kills_in_the_games_bursts_on_the_engine` (the race
  world, `World::burst` and one `World::bursts` batch: gas 1,056, Shatter
  1,056, a second Shatter 0, Detonate with no sickness 0, equal on both,
  none counted as fire's).

## The paired clips

`raw/film.sh` and `raw/make_clips.py`, counts in `raw/counts.json`. One
game on the Fork (race world, seed 0, idle hand, plain rounds), the same in
every clip up to the first cast; free casts (`boom=`) on the thick lobe at
(363, 244); the near view at zoom 8, 15 frames a second, 19.9 to 30 s,
llvmpipe, the HUD cropped off. Walkers within 12 cells and from 12 to 24,
each second.

| Clip | Casts | Kills | Within 12: 20, 23, 24, 30 s | From 12 to 24: 20, 23, 24, 30 s | Slime at 30 s |
|---|---|---|---|---|---|
| gas | gas at 20 s (1,055 asleep) | 0 | 293, 293, 293, 293 | 755, 759, 759, 762 | 24,766 |
| gas then shatter | gas at 20, shatter at 23 | 1,062 | 293, 293, 0, 0 | 755, 759, 3, 3 | 23,710 |
| shatter alone | shatter at 23 | 0 | 293, 291, 289, 301 | 755, 767, 773, 750 | 24,773 |
| germ | germ at 20 | 0 | 293, 159, 99, 0 | 755, 527, 399, 16 | 7,279 |
| germ then detonate | germ at 20, detonate at 22 (849 sick within 24) | 3,918 | 293, 0, 12, 2 | 755, 7, 9, 15 | 6,910 |
| detonate alone | detonate at 22 | 0 | 293, 291, 289, 301 | 755, 767, 773, 750 | 24,773 |

What the clips show: the gassed lobe goes grey and still; the shatter
takes it all at once, leaving a round hole whose rim of fresh sleepers (the
cloud still lingers) holds it open to the end. Alone, a shatter changes
nothing. The germ turns the lobe red and the plague eats the whole body
(24,766 to 7,279 in 10 s); a detonate two seconds in pops every sick walker
within 24 cells at once, so the lobe is gone at 23 s instead of 26, and the
body ends a little lower (6,910). Alone it changes nothing.

**The germ alone is already a plague.** On the Fork's one dense body a
single germ took 70% of the slime in 10 s; detonate made that sudden more
than larger. This is the germ as the toolkit built it (its wave crosses a
body in a few seconds: `../dead-ends.md`, germ amputation) and as step A
put it in the run shop from planet 4; the windows below are where Detonate
has to earn its place.

## The paired windows in play

`raw/windows.sh`, table by `raw/windows_table.py` in `raw/windows.txt`.
The seek report's 14 scenes (greedy on the Fork at 5 and 11 s, the Fork's
thick front held at 10 and 30 s, and five planet maps with guarding or
squad-mode hands at 10, 25 and 40 s): each plays one game to T, then every
squad loads the variant's deck and the window runs 10 s. In each pair the
setter's cadence is the same; only the second card differs, a plain bullet
or the reader. Built on main after #407 (crust), llvmpipe.

| Pair | Windows where the reader was cast | Kills in the window (all 14) | Paired: more kills | Slime at the windows' ends | Paired: less slime |
|---|---|---|---|---|---|
| gas+plain / gas+shatter | 4 of 14 (31 casts) | 1,098 / 2,135 | 4 of 4, 0 fewer (p = 0.125) | 378,775 / 377,807 | 3, 0 more (p = 0.25) |
| germ+plain / germ+detonate | 14 of 14 (161 casts) | 911 / 3,255 | 13, 1 fewer (p = 0.002) | 238,643 / 238,335 | 8, 6 more (p = 0.79) |

- **Shatter pays when it is cast, and it seldom is in 10 s.** Squads that
  guard fire few rounds, so in 10 of 14 windows the gun never reached its
  second card. Where it did, the window's kills rose every time: the held
  thick front at 30 s 858 to 1,746 (slime at the end 24,507 to 23,672), at
  10 s 106 to 193; greedy at 11 s 24 to 47. On greedy at 5 s its one cast
  read nothing: the squad's own shots had woken the gassed slime.
- **Detonate kills more in 13 of 14 windows (p = 0.002, the one decided
  result here), but the slime at the end is no lower** (8 of 14, p = 0.79):
  the plague kills the sick anyway, and Detonate takes them sooner. The
  germ is the strong card: in 8 of the 11 planet windows the germ+plain
  gun left 7,700-18,500 walkers at the end, where gas+plain on the same
  scenes left 24,800-26,700.

Two comparisons, so p below 0.025 survives a Bonferroni correction: only
Detonate's kills do.

## Cost

- Nothing a step: no reader state lives on the walkers; Shatter and
  Detonate read the gas and germ words the slime already holds.
- Each reader hit is one pass over the walkers (`readkill`, as a strike's
  `kill`) and a kill count read back in the frame's batch of bursts
  (`World::bursts`, one read-back for every burst in the frame).
- **1070 timing: booked**, with the held Mind and seek runs: `larder-world
  bench=40` with `rounds=gas-shatter` against `rounds=gas`.

## Not settled

- **Two in one shot.** Multicast with gas and shatter (`multicast+gas+
  shatter`, 8 of a gun's 10) gasses and shatters in the same shot: a kill
  as wide as the gas for 8 points, against a blast's (6 cells) for 3. Built
  as it falls out; a balance note, not a gate. A shatter that reads only
  sleep older than a second would close it.
- **Gunfire wakes the gassed** (alarm ends the gas), so the squad's own
  plain shots between a gas and a shatter can wake what the shatter would
  read; `Quiet` keeps them asleep.
- **Shatter reads gas sleep only**, not the Sleeper's own dormancy (planet
  6 sleeps on its own and wakes to alarm). Reading every dormant walker
  would make Shatter that planet's answer with no setter; kept to gas so a
  reader always pays off another card.
- **Shatter reads a disc, not "connected to the hit"** as the plan put it:
  a flood along the veins on the GPU is its own pass. The disc is the gas's.
- The numbers are first guesses: reach 12 and 24, cost 2.
- Next in step D, per the plan: route, element, force; cut last.

## Files

- `raw/film.sh`: one clip; `raw/make_clips.py`: MP4s and counts;
  `raw/counts.json`: the counts.
- `raw/windows.sh`, `raw/windows_table.py`, `raw/windows.txt`: the paired
  windows.
- `page/index.html`, its clips and `armory-special.png`: the Artifact
  page's source.
