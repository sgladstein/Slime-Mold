# Two Fevers, the playable demo (2026-11-10)

The owner, 20:33Z: "lets focus on actually building playable demos". This
report covers the first playable build of Two Fevers, the contagion war the
RTS brainstorm and its red team ranked strongest
([brainstorm](../rts-design-2026-11-10/brainstorm/README.md),
[red team §3](../rts-design-2026-11-10/red-team/README.md)). It covers two
versions, the census of each, and the owner's verdict on the first.

**Play it:** https://claude.ai/artifact/UMzPJmevoYJYmx8KRqG37a (a copy is in
[`page/`](page/)). **Code:** `src/mould/fever.rs` (the model and the
scripted hands), `examples/fevers.rs` (look, front and census),
`fevers-web/` (the browser build).

**In short.** Version 1 scored each fever on the walkers carrying it. It
plays as rings that sweep the host and leave it as it was. The owner,
21:11Z: "Not sure I follow what the strategy or challenge is in this game. I
click and it send a wave through the network, a few seconds later the
immunity moves through the network and eventually everything ends back at
the initial state." The census agrees: a planner beats random clicks on 16
of 16 seeds, but only by 0.536 of the summed score. Version 2 (the owner's
pick, "Both together", 21:15Z) makes ground persist. A walker keeps the colour of
the last fever it caught, ground just taken is held for 15 s, and a Hush
switch lets a fever cut the tubes under it. It is a tug of war now. The hold is
what makes aim matter: without it, random clicks beat the planner on 14 of 16
seeds. But the planner's margin over random is still 0.537, and Hush used the plain way
moves nothing measurable. Human play is the next test.

## The model

A host of 5,000 Jones walkers (sensor angle 22.5, rotate 45, reach 9, one
walker a cell) on The Wild One's ellipse, fed by the field retest's 23
mirrored piles ([field](../rts-design-2026-11-10/field/README.md),
`field_piles`). The piles are eaten down and grow back, so the tubes keep
moving. The population is fixed: nobody owns the host, nothing dies, nothing
is born. It is its own small host (`Fevers`) rather than a `Dish`.
`Dish`'s store, births and `swap_remove` deaths would scramble per-walker
fever state, and none of its tools, clock or storm are wanted.

Each fever has its own field, laid by its carriers, spread 3x3 and kept 0.80
a step. A clear walker catches a fever with chance `catch * (f - floor)` a
step (0.03, 0.3). A carrier recovers after 4 s and is then immune to that
fever for 40 s. In version 2 it is also immune to the other fever for 15 s
(`Rules::hold_seconds`), and it keeps the colour of the fever it caught
(`Rules::owned`). A seed (one spore; one every 10 s, three at most) gives
the fever to every clear, non-immune walker within 5 cells. A seed that
would catch nobody costs nothing. Hush (`Fevers::hush`) makes the fever's
carriers lay no trail; it can switch every 3 s.

**Everything on screen is state.** Tubes are the trail. A carrier is its
fever's colour. Owned ground is deep where it is held and pale where the
rival can take it. Each fever's field is a tint. A pile is as green as it is full.

**Cost.** 0.49-0.64 ms a step on one core of this container, so about 1.1
ms a frame at the dish's 2 steps a frame. Plan v random, seed 1, 3:15 in all:
0.637 ms a step in version 2 with stills written, 0.489 in version 1 over 30 s
with none. In the browser (V8, `node`), one 60 Hz tick with two steps, the
planner and the paint costs 2.5 ms.
Two cuts got it there, both recorded in the source: the spread without
per-neighbour branches (0.48 to 0.10 ms a step for four fields) and one
`sin_cos` a walker (move 0.40 to 0.30 ms).

### Tuning the front (look first)

`fevers front` seeds once near the centre after the warm-up and prints, each
second, the carriers and their distance from the seed (p90 / farthest).

| catch, sick | Front, p90 cells at 1 / 5 / 9 s | Most carriers at once | Seeds |
|---|---|---|---|
| 0.05, 8 s | 15 / 56 / 80 (about 9 cells/s) | 4,153 of 5,000 at 11 s | 2 |
| 0.05, 4 s | 15 / 56 / 85 | 2,452 at 9 s | 2 |
| 0.03, 4 s (shipped) | 10-13 / 46-47 / 75-76 (about 7 cells/s) | 1,983 at 11 s | 2 |
| 0.02, 5 s | 10-11 / 40 / 65-68 | 2,192 at 13 s | 2 |

The red team asked for 10-40 cells a second. At about 7, a front crosses the
dish in about 15 s and reads as a travelling ring (stills checked by eye).
At 8 s of sickness the ring was the whole dish.

### Spores and immunity (version 1, before the census)

Plan v random, 8 seeds, both colours, 2:00 matches. In every arm plan won
16 of 16 games. The table gives plan's part of the summed score, and the
games in which the side ahead at 1:30 still led at the bell.

| Spore every, immunity | Plan's part (mean) | Leader at 1:30 kept it |
|---|---|---|
| 5 s, 20 s | 0.515 | 31 of 32 (16 seeds) |
| 10 s, 20 s | 0.533 | 15 of 16 |
| 15 s, 20 s | 0.563 | 16 of 16 |
| 15 s, 40 s | 0.555 | 8 of 16 |
| 10 s, 40 s (shipped) | 0.558 | 7 of 16 |

Fewer spores make each seed count for more. Longer immunity lets a lead
turn. 10 s and 40 s were kept.

## Deciding measures, named before the runs

16 seeds. Each hand plays once as Ember and once as Tide on each seed. The
score is each fever's share of the host (carrying in v1, owning in v2),
averaged over 3:00 of play after a 15 s warm-up.

- **Skill:** plan beats random on summed score, on at least 13 of 16 seeds
  (both colours averaged), one-sided sign test. The same against idle (one
  seed at the start).
- **Comeback:** in plan v plan, the side ahead at 1:30 is behind at the bell
  in at least a quarter of games, and more often than with the comeback rule
  off. For v1 that is immunity to both strains (`cross=1`); for v2 it is no hold (`hold:0`).
- **Hush:** a planner that hushes beats the same planner without it on at
  least 13 of 16.

Hands (`fever::Hand`): **Idle** seeds once on a random walker. **Random**
spends each spore at a random moment within 10 s, on a random walker that
would catch. **Plan**, every quarter second with a spore in hand, seeds the
8x8-cell patch with the most walkers open to it within two patches, minus
three for each of its own carriers there. It counts rival-owned walkers
double and its own half in v2, and walkers immune to the rival 1.5 in v1.
**PlanHush** is Plan that hushes whenever the rival has more carriers.

## Results

Raw output: [`raw/`](raw/). Summaries: `python3 summarise.py raw/*.txt`.

| Version, matchup | a's part, median (range) | a won, seeds | p (one-sided) | Leader at 1:30 kept it |
|---|---|---|---|---|
| v1 plan v random | 0.536 (0.511-0.569) | 16/16 | 1.5e-5 | 13/32 |
| v1 plan v idle | 0.801 (0.797-0.900) | 16/16 | 1.5e-5 | 32/32 |
| v1 random v idle | 0.795 (0.766-0.901) | 16/16 | 1.5e-5 | 32/32 |
| v1 plan v plan | 0.500 (mirror) | | | 18/32 |
| v1 plan v plan, immune to both | 0.500 (mirror) | | | 24/32 |
| v2 plan v random | 0.537 (0.514-0.576) | 16/16 | 1.5e-5 | 31/32 |
| v2 plan v idle | 0.934 (0.899-0.966) | 16/16 | 1.5e-5 | 32/32 |
| v2 random v idle | 0.887 (0.843-0.963) | 16/16 | 1.5e-5 | 32/32 |
| v2 plan v plan | 0.500 (mirror) | | | 18/32 |
| v2 plan v plan, no hold | 0.500 (mirror) | | | 30/32 |
| v2 plan v random, no hold | 0.454 (0.315-0.525) | 2/16 | 1.0 (random wins 14/16, p 0.002) | 30/32 |
| v2 planhush v plan | 0.508 (0.486-0.526) | 10/16 | 0.23 | 22/32 |

Holm over the seven named sign tests (plan over random, plan over idle and
random over idle in each version, and Hush): the six at p 1.5e-5 pass (the
strictest bar is 0.05/7 = 0.0071); Hush fails (p 0.23). The no-hold runs
are the comeback control, and their reversal is a finding.

- **Skill passes the bar in both versions but is thin.** Plan wins every
  seed, by about 0.54 of the summed score. In v2 the planner owns 0.51 of the
  host on average against random's 0.44 (v1: carries 0.088 against 0.075). Idle loses badly. So seeding matters, and aim
  matters a little.
- **The hold carries the skill in v2.** With no hold, ground just taken is
  open to the rival at once. Random clicks then beat the planner on 14 of 16
  seeds, and the leader at 1:30 keeps the lead in 30 of 32 planner mirrors.
  The planner seeds where it can take most, which is the rival's freshest
  ground, and loses it straight back. With the hold, the mirror leader keeps
  the lead in only 18 of 32, so a lead can turn (bar: a quarter turn; 14 of
  32 do).
- **Per-strain immunity does turn leads in v1.** In the planner mirror the
  side ahead at 1:30 lost at the bell in 14 of 32 games, against 8 of 32
  when recovery made walkers immune to both fevers.
- **The rubber band.** In v1 the planner's lead at 1:30 lasted in only 13 of
  32 games against random, though it won all 32 at the bell. The score
  swings with each ring, and the comeback rule pulls it back toward even.
  This is the red team's warning. In v2 a planner's lead over random lasts
  (31 of 32), and the margin is no larger.
- **Hush, used the plain way, is noise.** 10 of 16. Whether a firebreak can
  be aimed (hush, then seed across a road) is untested; the plain rule does
  not show it.

## What it is not yet

- The scripted planner is crude: one look a quarter second, no timing beyond
  spending at once. A human reading the holds may do much better or no
  better. No human has played it yet, apart from the owner's first look at v1.
- A front reaches most of the dish, so one seed can swing 30% of the host.
  Smaller fronts (lower `catch`) would make the map more local. Untested.
- The traits other than Hush (Swarm, Roam, Linger, Beckon, Bloom) are not
  built.
- The opponent is a script on the same screen. Two people can share a mouse
  (right-click or shift-click seeds Tide; J hushes Tide).
- A native window build does not exist; the demo is the browser page.

## Files

- [`summarise.py`](summarise.py): per-seed summary and sign test of a census file.
- [`raw/`](raw/): every census run named above. v1 runs were made with the v1
  code (`owned` did not exist; the cross run later with `set=owned:0`).
- [`page/`](page/): the published page and its `fevers.wasm`
  (`fevers-web/build.sh`).
