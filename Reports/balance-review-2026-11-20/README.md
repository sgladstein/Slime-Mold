# Balance review (2026-11-20)

Where the game's balance stands and what needs fixing, from the data we
already had. The owner asked at 11-20 05:49Z: "Start a thread to explore
where the game balance and what aspects need fixing. Extra effort." A review,
not a retune: no tuning number was changed, and nothing new was played on his
PC. Fixes are proposals to the lane that owns each one.

Doc for the owner: [Game balance review: where it stands and what to fix](https://claude.ai/artifact/EbhGU5rMAt7XC8P5zocjYc)
(a Claude Doc with one chart; the chart's source is `chart.jsx`, its rows
`chart-rows.json`).

## The short answer

Most of what feels off comes from one mechanism: every planet is a long
stand-off that ends in a one-to-two-minute collapse once the army is big
enough, and nothing pushes back against a big army. The rush, the short
planets and cards mattering less than numbers all follow from it.

| # | Finding | Whose call | Recommended fix |
|---|---|---|---|
| 1 | The army snowball decides every planet, rush or not | The owner, with D7 (via Direction) | Put to him as part of D7: the rush and the snowball are one lever. Nothing built unasked |
| 2 | A time-out with half the slime cut counts as a won planet | Direction asks the owner; Engine and tools for the bot's count | Count it apart from wins now; ask whether a time-out should carry on at base pay with an honest screen |
| 3 | Planets 1 and 3 end well short of 8 to 12 minutes for everyone | Direction | No retune for this alone; finding 1 is the cause |
| 4 | About half the shop's cards do nothing, or hurt, on their own (exit test 5) | Squad and arsenal | Don't offer cards with nothing to act on; say so on the card; decide Wide and friends on plain bullets |
| 5 | Loadouts can't be filled: an owned card is never offered again | Squad and arsenal | Let owned cards back into the draw |
| 6 | The Needle Blast deck is a rare find in a real run (1 run in 213 by planet 3) | Direction | No further change to Needle Blast now |
| 7 | Planet 2 is now the hardest of the three | Direction | Measure it on today's game before touching the Sprawler's size |
| 8 | Food is lost at the win and points are flat | The shop-money thread | Already being worked ([its doc](https://claude.ai/artifact/XHLmAP2RFnSAHba6orJLjW)) |
| 9 | Past the milestone, planet 5 ended a run in 1:51 | Milestone 2 | Note only |

One clear bug sits inside finding 4: the run bot bought Tinder without the
Torch in every planet 1 shop, so its gun laid brush that never burned. Fixed
in the same pull request as this report (`runbot.rs`, Engine and tools' file).

## Sources and method

Every number below comes from one of these; each script prints the numbers it
gave, saved beside it.

- **The owner's play logs** (`play-logs` branch, at 9eeae2b): five runs from
  11-18 and 11-19, 14 won planets and 2 losses, one line a second.
  `play_logs.py` → `play-logs-output.txt` (and the chart's rows). A planet
  is won when its slime ends under 1% of its peak; "above half the peak" is
  the share of the planet's seconds with the slime at half its peak or more;
  "half to 5%" runs from the first second under half after the peak to the
  first under 5%.
- **The strategy bot league** (PR #529, rows on the strategy thread's branch
  `claude/project-thread-8grg6d` at acce17b, 11-20 05:46Z): 3,302 games.
  `league_times.py` → `league-times-output.txt`, split by the code stamp on
  each row: "first" is the first league, which played planets 1, 2 and 3
  alike; "old" is every game before shots stopped at set slime; "set" after
  it (11-20); "now" after one burst per bullet (#532).
- **The gun lab** ([gun builds](../gun-builds-2026-11-19/README.md), PR
  #527): its single-card table `raw/singles-pc.txt` and pairs and triples
  tables. These ran before one burst per bullet, which changed only Needle
  bullets.
- **The run bot on main**: seven Run bot checks on 11-20, run seeds 1 to 3,
  planets 1 to 3 (`runbot-ci-2026-11-20.txt`, from the job logs).
- **A shop simulation**: `shopsim.rs` (copy into `examples/` to run) drives
  the real `Run::restock` over 4,000 run seeds → `shopsim-output.txt`.
- **The code**: `run.rs` (rewards, prices, offers), `campaign.rs` (verdicts
  and stars), `rounds.rs` (how cards become shots).

## Findings

### 1. The army snowball decides every planet, rush or not

In the owner's 14 won planets the slime stays at half its peak or more for a
median 75% of the planet, then falls from half to 5% in a median 1.37
minutes. A planet is a stand-off while the army builds, then a collapse.

- Armies at the win: 15 to 86 soldiers on planet 1, 42 to 104 on planet 2,
  38 to 85 on planet 3, 39 to 103 on planet 4. The bots' winning armies peak
  at a median 35 to 80 (league, by planet and code).
- Food left at the win: 694 to 12,628, over 3,500 on 8 of 14 planets.
- Why, in the code: a soldier is a flat 400 food with no upkeep (owner,
  2026-11-14) and a cap of 200; pumps never run dry; the slime's size is set
  at landing, and its surges and raids don't grow with the army it faces.
- The rush is the same thing, sooner: his 11-18 rush had 40 soldiers at 1:02
  and won planet 1 in 3:14 with 86. The bots learned the same rush on
  planets 1 and 2 (the strategy report).

Recommendation: the core of D7, the owner's call; nothing built unasked.
Options that respect his no-maintenance ruling: surges and raids that grow
with the army facing the slime (this review's pick, because it plays as
something the slime does to you, exit test 4); a soldier price that climbs
with the soldiers alive; the origin as the win (Milestone 2).

### 2. A time-out with half the slime cut counts as a won planet

At the clock, `Cause::Cap` grades a planet by its best cut: half the peak
or more is 1 star (`campaign.rs`), any star calls `Run::won`, and the end
screen reads "Mission won" though the planet card says "kill every last
walker".

- The run bot's only planet 3 win on main: run seed 1, "won, 1 star at 18:00
  (Time: held on, 1 star (best 70% of the goal))" with 22,090 slime left, on
  all four checks since #531.
- The owner's 11-19 evening planet 3 ended this way at 17:59 with about 40
  walkers he could not kill (the set-slime bug, since fixed).
- The league stops games at 12:00, so its wins are real wipes.

Recommendation: Engine and tools has the run bot's summary count "held on"
apart from wins, so exit test 1 can't be passed by the clock; Direction asks
the owner once what a time-out should be. Pick: the run carries on, pays the
base 150 only, and the screen says the slime survived.

### 3. Planets 1 and 3 end well short of 8 to 12 minutes

- The owner: planet 1 in 3:14 to 7:26 (median 4:05 over 6), planet 2 in 7:31,
  9:05 and 13:37, planet 3 in 5:03, 7:23 and the 17:59 time-out, planet 4 in
  9:11 and 9:51.
- Bots: median win 4:05 on planet 1 and 6:26 on planet 2 (first league), 4:50
  on planet 3 after the set-slime fix and 5:27 after one burst per bullet;
  79 to 91% of each planet's wins under 8 minutes.
- His two clean runs spent 18:27 and 21:18 on planets 1 to 3, against the
  milestone's 30 minutes.

Recommendation: don't retune slime sizes for this; a bigger slime lengthens
the stand-off in finding 1, not the fight. If the band stays, consider exit
test 1 as about 30 minutes for planets 1 to 3 with planet 1 the short opener.

### 4. About half the shop's cards do nothing, or hurt, on their own

- Trigger, Expire and Multicast with one bullet: each with Blast alone
  matched Blast alone exactly in all 18 arena games (gun lab pairs table).
  `rounds::casts` drops a carrier with no bullet card after it. The owner's
  11-19 evening planet 2 gun was Trigger plus one Torch.
- Wide, Scatter, Quiet, Delay and Thick on a plain bullet: identical to the
  plain needle in 18 of 18 games each (gun lab singles). With no bullet card
  at all, as a run starts, they are dropped. Wide's text: "Its hit is wider."
- Slug and buckshot bullets: 9 of the 10 measured remove less slime than no
  card on at least 2 of 3 planets (gun lab singles), and the two sets are
  half of the bullets a shop rolls (`shopsim-output.txt`). The slug's push
  still guards a crystal, which the arena doesn't count.
- Tinder is in 100% of planet 2 shops (`bring`) and kills nothing without
  fire. The run bot bought it without the Torch on every check.
- 64.3% of planet 2 shops offer at least one of those eight cards.

Recommendation (Squad and arsenal): the shop skips a modifier or carrier
while the run owns no bullet card it could act on, as `Run::restock` already
does for Shatter and Detonate; such a card says "needs a bullet after it";
decide whether Wide and friends act on plain bullets (the code calls Plain
a payload; this review's pick is yes). This bears on exit test 5: one shop
visit should change how the gun plays.

### 5. Loadouts can't be filled: an owned card is never offered again

`Run::restock` draws only cards the run doesn't own until it owns all 19 on
sale; since loadouts (11-19) each card sits in one loadout, so a second
squad's loadout can never get its own Seek, Rate or Wide. Only rolled bullets
repeat. Recommendation (Squad and arsenal): let owned cards back into the
draw, perhaps at half weight.

### 6. The Needle Blast deck is a rare find in a real run

Over 4,000 simulated runs buying the Torch, the shops before planets 2 and 3
offered a Needle Blast bullet in 7.8% of runs (price 90 to 300, median 160),
Needle Blast with Wide, Rate or Seek in 4.5%, and Wide, Rate and Needle Blast
together in 0.47% (1 in 213). The league puts wanted cards on the shelf, so
it measures a deck's strength, not how often a run gets it. Recommendation
(Direction): no further change to Needle Blast now; when the question is
what a run gets, the league buys only what the shop offers.

### 7. Planet 2 is now the hardest of the three

- First league: 64% of 334 planet 1 games won, 24% of 320 planet 2 games,
  2% of 320 planet 3 games.
- Planet 3 since shots stop at set slime: 89% of 992 games won.
  No planet 2 batch has run since.
- The run bot: seed 2 loses planet 2 at the 16:30 clock on all seven checks
  of 11-20 with 34 to 43% of the goal, killing 41,428 to 43,793 walkers while
  12,014 to 12,557 remain. Seed 1 lost it the same way (28% of the goal) on
  the three checks before the loadout fixes (#531). Seed 3 loses planet 1
  (behind rock: a basin with one gap, Engine and tools' handoff).
- The owner: planet 2 is his longest planet and his only loss in planets 1
  to 3 (11-18, before its slime was cut to 0.6).

Recommendation (Direction): before touching the Sprawler's size, one planet 2
job on today's game, maps 1 to 8 plus his three planet 2s (run seeds
1795038793, 1795106014, 1795119328), in the strategy thread's queue.

### 8. Food is lost at the win, and points are flat

The shop-money thread has this. From his logs: food left at the win
(finding 1), and in his two runs with shop records he bought no squad
upgrade (Turrets, Pump power, Mortars and cards instead).

### 9. Past the milestone: planet 5 ended a run in 1:51

On 11-19 his run landed on planet 5 into 80,544 walkers (after its 25 s head
start) with 8 soldiers, down to 1 by 1:34; planet 4 had peaked at 57,053 and
63,355. For the curve over all eight planets in Milestone 2.

## Corrections to what we thought

| We said | What the data says now |
|---|---|
| Bots win planet 3 only rarely without good decks | True of the first league (2%). Since shots stop at set slime the planet 3 search won 89%, and plans with no deck (Tinder only) won up to 13 of 17 maps |
| Needle Blast decks lead | In the league. A real run is offered Needle Blast by planet 3 in 7.8% of runs, the whole best deck in 0.47% |
| Bomb and Buckshot decks do worse than no cards | Wider: 9 of 10 slug and buckshot bullets cost kills on most planets |
| The run bot is stuck on planet 2 | By seed: seed 1 clears planets 1 to 3 (planet 3 only by the clock), seed 2 loses planet 2, seed 3 loses planet 1 |
| Rushing clears planets 1 and 2 faster | Yes, and the owner's own non-rush wins end the same way, later |

## What this can't tell yet

- Planet 2 on today's game (finding 7).
- The owner's push-and-guard plan: the bots' split test used Heave, not slug
  bullets, and was worse on 7 of 9 maps; a split job should try slugs too.
- Kills are not the whole value of a card: the arena counts slime removed;
  guarding, herding and sleep show only in full games.
- Five runs is a small record, and his 11-19 evening run had the crystal
  regrow experiment on, which may have lengthened its 13:37 planet 2.

## Files

- `play_logs.py`, `play-logs-output.txt`, `chart-rows.json`, `chart.jsx`: the
  owner's logs and the doc's chart.
- `league_times.py`, `league-times-output.txt`: league games by planet and code.
- `shopsim.rs`, `shopsim-output.txt`: the shop simulation.
- `runbot-ci-2026-11-20.txt`: the run bot's results on main, 11-20.
