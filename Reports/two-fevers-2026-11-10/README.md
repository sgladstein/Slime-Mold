# Two Fevers, the playable demo (2026-11-10)

The owner, 20:33Z: "lets focus on actually building playable demos". This
report covers the first playable build of Two Fevers, the contagion war the
RTS brainstorm and its red team ranked strongest
([brainstorm](../rts-design-2026-11-10/brainstorm/README.md),
[red team §3](../rts-design-2026-11-10/red-team/README.md)). It covers two
versions, the census of each, and the owner's verdict on the first.
Version 3 (calls, fever by tube, pinch, foraging) is at the end:
[*Version 3*](#version-3-the-mould-at-the-centre).

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

## Version 3: the mould at the centre

The owner, 21:52Z, after version 2: "still feel overly simple with 1-2 tools
that lead to mostly 1 strategy ... The slime mold seems more like background
in this game and not the integral part". The design is in
[`v3-design.md`](v3-design.md); the owner picked "Build it after the checks"
(22:06Z). `Rules::v3()` holds every setting below; the page and the census
use it.

**What changed.** Each side starts owning the walkers within 12 cells of its
end pile (`home_r`). The walkers you own smell your **calls** (two at most,
free): a linear pull out to 60 cells with a flat core of 15, weight 10, felt
by nobody else. A fever's field spreads only through tube, in proportion to
trail up to 6 (`tube_fever`). Owned walkers on a pile earn spores (0.005 of a
spore a step each, at most 12 per pile) on top of one every 20 s (at most 4).
A walker you own that has stood off every tube route to food for 12 s goes
back to nobody's (`starve_seconds`; the route is a breadth-first search from
the piles through trail of 2 or more, every 6 steps). A **pinch** stops your
own walkers in a 5-cell disc laying trail for 6 s. And a walker you own that
is fed catches the rival's fever ten times less often (`resist` 0.1).

### The three checks (before the build)

Raw: [`raw/v3-checks/`](raw/v3-checks/), `examples/feverprobe.rs`, 16 seeds.

| Check | Bar | Result |
|---|---|---|
| Fever by tube stops at a 3-cell cut | 16/16 | 0 carriers past the cut on 13/16, at most 7 on any; ungated 600-936 on every seed. Passed in substance |
| A trunk carries the fever twice as fast | 2x | **Failed:** thick tube 5.3 cells/s, thin 5.1. What tube does is carry *more*: 10,230 catches on thick tube, 668 on thin (ungated 17,567 and 16,574) |
| A call reshapes the body | 13/16 | 16/16: 96-165 owned walkers on the called pile at +20 s, 0-1 without |
| A pinch cuts | 16/16 under tube strength within 5 s, for 6 s | 14/16: trail about 10 falls under 1.5 within 1 s |

### Tuning (seeds 1-2 or 1-6, before the census)

| Setting | What happened |
|---|---|
| Call weight 30 | 70% of the host heaped on the call by 2:00; no roads. Weight 10 and a flat core of 15 keep roads |
| Uncapped foraging (0.008-0.02) | The raider's seeds bought ground, ground bought spores: 44 seeds to 7 |
| Pinch at 1 spore | The Web plan spent every spore on pinches once the rival's fever was near, seeded 1-2 times a game, kept 1-2% of the host |
| No resistance | Whoever seeded the neutral mould first took it all: raider 0.96-0.99 of every game |
| `resist` 0.1, free pinch every 5 s, forage 0.005 (shipped) | Contested games: web v raider 0.44-0.63, trunk v raider 0.39-0.86 |

### The census

16 seeds, both colours, 3:00, `Rules::v3()`. Hands (`Hand::Trunk`, `Raider`,
`Web`): **Trunk** calls the best pile near its body (food over distance) and
moves when it runs low, and seeds as Plan with every spore. **Raider** keeps
its call at home and seeds at once on the rival's walker on the thickest
trail. **Web** calls two good piles 50 or more cells apart, pinches its own
walker nearest a rival carrier (at most every 15 s) and seeds as Plan while
it keeps one spore spare. Raw: [`raw/v3/`](raw/v3/).

Deciding measures, named before the run: a cycle whose three links each
survive Holm; each plan beating random on 13 of 16; and the snowball guard,
the mirror's 1:30 leader losing at least a quarter of games.

| Matchup (a v b) | a's part, median (range) | a won, seeds | p (one-sided, for the winner) | Leader at 1:30 kept it |
|---|---|---|---|---|
| trunk v raider | 0.651 (0.489-0.863) | 15/16 | 0.00026 | 32/32 |
| raider v web | 0.417 (0.173-0.593) | 5/16 | web 0.105 | 30/32 |
| web v trunk | 0.443 (0.219-0.598) | 4/16 | trunk 0.038 | 30/32 |
| trunk v random | 0.659 (0.496-0.919) | 15/16 | 0.00026 | 29/32 |
| web v random | 0.670 (0.465-0.909) | 15/16 | 0.00026 | 29/32 |
| raider v random | 0.446 (0.189-0.846) | 7/16 | 0.77 | 31/32 |
| trunk v idle | 0.767 (0.569-0.969) | 16/16 | 1.5e-5 | 27/32 |
| web v idle | 0.768 (0.441-0.924) | 13/16 | 0.011 | 28/32 |
| raider v idle | 0.786 (0.222-0.953) | 15/16 | 0.00026 | 26/32 |
| raider v raider | 0.500 (mirror) | | | 32/32 |

Holm over the nine named tests (three links, three plans over random, three
over idle): trunk over raider, trunk and web over random, and all three over
idle pass. Trunk over web (p 0.038) and web over raider (p 0.105) do not,
and raider over random fails outright.

- **No cycle.** The order is a ladder, trunk over web over raider, and only
  its top link is firm. The design hoped the raider beats the trunk; with a
  fed body resisting, seeding into the rival's thickest tube is seeding into
  its best-fed walkers, so the raid is the worst use of a spore.
- **Income is the game.** Both plans that move their calls to food beat
  random 15/16; the one that stays home does not. The mould's shape matters
  through where it eats.
- **The snowball guard fails.** The raider mirror's 1:30 leader kept the lead
  in 32 of 32 (version 2: 18 of 32), and every matchup is 26-32 of 32. The
  hold no longer turns leads: ground pays spores, so the leader also earns more.
- **Spreads are wide.** a's part runs 0.17-0.97 across seeds and colours; the
  same pair on the same seed can swing by 0.5 with sides swapped (web v
  trunk, seed 1: trunk's part 0.71 as Tide, 0.18 as Ember).

**Cost.** 0.70-0.84 ms a step on one core of this container with stills
written and a census on the other cores (version 2: 0.49-0.64), so about
1.5 ms a frame at 2 steps. The rest is the starvation search every 6 steps,
the call pull and the per-pile forage count.

**What it is not yet.** The scripted plans are crude, and a ladder among
three scripts is not a ladder among people; the owner's hand is the next
test. The levers the census points at: something that feeds the trailer (a
cap on spores from ground, or income that falls as a body grows), and a raid
that pays (seeding a starving or cut-off part is the raid the rules reward
now, and no script does it).

## Files

- [`v3-design.md`](v3-design.md): the owner's verdict on version 2 and the design for version 3 (calls, fever by tube, pinch, foraging income).
- [`summarise.py`](summarise.py): per-seed summary and sign test of a census file.
- [`raw/`](raw/): every census run named above. v1 runs were made with the v1
  code (`owned` did not exist; the cross run later with `set=owned:0`).
- [`page/`](page/): the published page (version 3 now) and its `fevers.wasm`
  (`fevers-web/build.sh`).
- [`raw/v3-checks/`](raw/v3-checks/) and [`raw/v3/`](raw/v3/): version 3's
  checks (`examples/feverprobe.rs`) and census.
