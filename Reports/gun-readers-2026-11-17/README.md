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

Status: **built and merged (#408); the 1070 timing is booked.** Two
readers, Shatter and Detonate, in both engines, exact twins. **Re-run
2026-11-17 after the GPU germ fix** (`../gun-route-2026-11-17/`): the
germ's clips and paired windows below are the fixed build's; the first
ones carried a second germ the bug planted at the map's middle, which made
the germ look like a plague.

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
| germ | germ at 20 | 0 | see below | | |
| germ then detonate | germ at 20, detonate at 22 | see below | | | |
| detonate alone | detonate at 22 | 0 | 293, 291, 289, 301 | 755, 767, 773, 750 | 24,773 |

What the clips show: the gassed lobe goes grey and still; the shatter
takes it all at once, leaving a round hole whose rim of fresh sleepers (the
cloud still lingers) holds it open to the end. Alone, a shatter changes
nothing.

**The germ clips, re-filmed on the fixed build** (2026-11-17, main after
#416: shape C's resistance 10, the rounds' sick upkeep 10), 19.9 to 40 s,
because the germ starves its sick 12 to 18 s after the cast, after a 10 s
clip ends:

| Clip | Casts | Kills | Within 12: 20, 24, 30, 34, 36, 40 s | From 12 to 24: same | Sick within 24: 22, 24, 30 s | Slime at 30 s, 40 s |
|---|---|---|---|---|---|---|
| none (to 40 s) | none | 0 | 293, 289, 301, 284, 290, 286 | 755, 773, 750, 760, 752, 750 | none | 24,773, 25,772 |
| germ | germ at 20 | 0 | 293, 289, 301, 156, 30, 0 | 755, 773, 750, 387, 244, 62 | 780, 1,067, 0 | 24,772, 17,628 |
| germ then detonate | germ at 20, detonate at 22 | 780 | 293, 23, 68, 42, 0, 0 | 755, 321, 535, 474, 238, 0 | 780, 79, 526 | 23,996, 19,590 |

The germ turns the lobe red and the red runs on through the body; nothing
dies for 12 s, then the lobe thins to a net and is gone by 38 s, and the
body is 31.6% smaller at 40 s. A detonate 2 s in pops the 780 sick within
24 cells at once and opens a hole, but the wave goes on from its rim; by
40 s the body is 19,590 against the germ alone's 17,628. **An early
detonate cuts the germ short**: it kills its carriers before they pass
it on. One clip, seed 0.

The first germ clips (with the bug) had the whole body eaten in 10 s,
24,766 to 7,279, and a detonate popping 3,918; neither was the germ.

## The paired windows in play

`raw/windows.sh`, table by `raw/windows_table.py` in `raw/windows.txt`;
the germ pair re-run on the fixed build in `raw/windows-germ.txt`.
The seek report's 15 windows (14 in the first count, a miscount) (greedy on the Fork at 5 and 11 s, the Fork's
thick front held at 10 and 30 s, and five planet maps with guarding or
squad-mode hands at 10, 25 and 40 s): each plays one game to T, then every
squad loads the variant's deck and the window runs 10 s. In each pair the
setter's cadence is the same; only the second card differs, a plain bullet
or the reader. Built on main after #407 (crust), llvmpipe.

| Pair | Windows where the reader was cast | Kills in the window (all 14) | Paired: more kills | Slime at the windows' ends | Paired: less slime |
|---|---|---|---|---|---|
| gas+plain / gas+shatter | 4 of 14 (31 casts) | 1,098 / 2,135 | 4 of 4, 0 fewer (p = 0.125) | 378,775 / 377,807 | 3, 0 more (p = 0.25) |
| germ+plain / germ+detonate, fixed build | 15 of 15 (186 casts) | 1,076 / 1,689 | 12, 2 fewer (p = 0.013) | 378,559 / 378,248 | 13, 1 more (p = 0.002) |
| (with the bug, withdrawn) | 15 of 15 (161 casts) | 911 / 3,255 | 13, 1 fewer (p = 0.002) | 238,643 / 238,335 | 8, 6 more (p = 0.79) |

- **Shatter pays when it is cast, and it seldom is in 10 s.** Squads that
  guard fire few rounds, so in 10 of 14 windows the gun never reached its
  second card. Where it did, the window's kills rose every time: the held
  thick front at 30 s 858 to 1,746 (slime at the end 24,507 to 23,672), at
  10 s 106 to 193; greedy at 11 s 24 to 47. On greedy at 5 s its one cast
  read nothing: the squad's own shots had woken the gassed slime.
- **On the fixed build Detonate leaves less slime at the window's end in
  13 of 15 windows (1 more; p = 0.002) and kills more in 12 (2 fewer;
  p = 0.013)**; one window (p2s1 at 10 s) tied on both. In a 10 s window the germ's own starving has
  not landed yet (it comes 12 to 18 s after a cast), so the detonate's
  kills are most of what the germ shows there: germ+plain left 378,559
  walkers over the 15 windows, gas+plain 378,775. The 40 s clip above says
  the other half: over 20 s an early detonate leaves more slime than the
  germ alone.
- **With the bug** the windows read the planted germ: germ+plain left
  7,700-18,500 walkers in 8 of the 11 planet windows, Detonate more kills
  but no less slime. Withdrawn.

Four tests (two pairs, kills and slime), so p below 0.0125 survives a
Bonferroni correction: Detonate's less slime does (p = 0.002); its kills
(p = 0.013) just miss.

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
- **The germ was named a dominance check** (slime-enemy thread's review,
  11-17; it was the bug, below, and is retuned per planet in the route
  report). One germ round took the Fork's one dense body
  from 24,766 to 7,279 in 10 s: contagion working as built, but too strong
  for one round sold from planet 4. The levers are the rounds' own germ
  rules (`Rounds::kit`, so the toolkit's germ dish is untouched), in order:
  fewer sick seconds past immunity (`sick_secs`, 6, against `immune_secs`,
  40), a smaller `sick_upkeep` (60, what starves the sick) and a higher
  `sick_catch` (0.2). Named here, not retuned in the readers' PR; it should
  not stay the run's default without that retune.
  **Correction (2026-11-17, the route report):** the GPU world's first germ
  also infected a 40-cell disc at the map's middle once its tick count
  passed the dish's timed seeding (16 s of game time), a second germ the
  CPU never had. Every germ clip and window here from 16 s on carried it,
  so the plague was partly that: without it one round on the Fork lobe
  took 48% of shape C in 20 s, not 76%. Fixed in `GpuWorld`'s burst
  ([route](../gun-route-2026-11-17/README.md), *The germ*). The germ
  clips and windows above were re-run on the fixed build the same day,
  on the game designer's ask (08:05Z), so no page shows the planted
  spread.
- When to detonate: an early detonate (2 s after the germ) cuts the wave
  short; the game designer's call whether that is the card's lesson
  (detonate late, when the wave has spread) or a tuning note.
- The numbers are first guesses: reach 12 and 24, cost 2.
- Next in step D, per the plan: route, element, force; cut last.

## Files

- `raw/film.sh`: one clip; `raw/make_clips.py`: MP4s and counts;
  `raw/counts.json`: the counts.
- `raw/windows.sh`, `raw/windows_table.py`, `raw/windows.txt`: the paired
  windows; `raw/windows-germ.txt`: the germ pair on the fixed build
  (`windows_table.py OUT germ-plain,germ-detonate`).
- `page/index.html`, its clips and `armory-special.png`: the Artifact
  page's source.
