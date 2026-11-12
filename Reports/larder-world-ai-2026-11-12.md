# Larder player AI on the world (2026-11-12)

The small-dish player AI ([larder-ai-2026-11-11.md](larder-ai-2026-11-11.md), PR #65) ported onto the Larder world's game layer (`larder_world::Game`, W2b, PR #81). It was asked for by the owner at 14:07Z on 11-11: "a player AI thread, for automated testing so you can keep developing without me having to playtest everything". Code: `src/mould/larder_world_ai.rs`. The census runs in `examples/larderworld.rs`: any hand name that isn't a scripted hand is an AI style. Raw output is in [larder-world-ai-2026-11-12/](larder-world-ai-2026-11-12/). Pictures, with the counts beside them, are on the [Artifact page](https://claude.ai/artifact/BuPV981PmqpXkDhxTnadzs). Its source is archived in `larder-world-ai-2026-11-12/page.html` (see [pages.md](pages.md)).

## What it is

- **It plays through the player's commands.** It gives only `Cmd`s (order, stance, buy, wall) through `Game::apply`, so every AI round is a replay. Test `an_ai_round_is_a_replay` replays a round to the same world hash.
- **It sees what a player sees** (`View`): its soldiers, the bank, the crystals and their charge, the clock, the walker count, and the mould as walker counts on a 4-dish-cell grid. The grid is a new `World::mould_grid`. `BigDish` bins its occupancy for it; it defaults to None, and the GPU world has none until someone adds an occupancy read-back. Without the grid the brain still plays, but it reads mould only through its soldiers' touch.
- **It commands a `Side`** (a home and its soldiers). A second soldier team would be a second brain (test `a_side_commands_only_its_own`).
- **One brain, five styles on knobs** (`Style`). The world scores a stalemate by the share of contested crystals held at the cap (`Game::share`), so the knobs choose which crystals to farm, which mould-held crystals to take back and when, where to put walls, and when to strike the core. Knobs change from the command line (`claim:claim_at=200+keep=4`).
  - **boom**: farm the 20 nearest crystals the mould is not near, one soldier each, and buy a soldier for each. Like the W2b farm hand, but wider, and every buy keeps 30 s of fire for the whole squad in the bank.
  - **claim**: boom to 28 soldiers. From 10:00, pairs stand on the nearest mould-held crystals while 12 keep farming.
  - **late**: claim, starting 90 s before the cap.
  - **fort**: boom, plus up to 8 walls, each 16 dish cells long, 8 cells out from a farmed crystal, laid across the mould's line of approach when it comes near.
  - **strike**: boom to 30 soldiers. From 6:00, once 20 stand, all but 8 attack the core. Once the core is down they claim.
- Turrets, bait and drops are not ported, because W2b did not port them. The M2 reader (adapt) is gone too: its signal was the mould turning small, and on the race world the mould only grows.

## Built on seeds 1-4

- **Farming wider was the biggest gain.** Boom farming 12 crystals scored a mean of +0.14. Farming 20 scored +0.25. The scripted farm hand (8) scored -0.05 ([dev1](larder-world-ai-2026-11-12/dev1.txt), [dev2](larder-world-ai-2026-11-12/dev2.txt)).
- **A claim from 5:00 cost soldiers.** Pairs on Defend sent into the mould's network were eaten (claim seed 1 ended with 5 soldiers). Fours on Free fire were no better (mean +0.03). A claim from 10:00 scored +0.08, and the final claim, with 12 farmers kept, scored +0.23 ([dev3](larder-world-ai-2026-11-12/dev3.txt)).
- **The first strike sent everyone and lost.** With no farmers left, the bank ran dry mid-strike. Three of four squads ended with one soldier or none, and one was wiped. Keeping 8 farming turned it around: the strike wiped the mould on seeds 1 and 3 (mean +1.03). These were the first mould wipes on the world.
- **Fort's first walls never went up.** It walled only crystals it was still farming, and a crystal leaves the farm list once the mould comes near (`dare`). It now walls the crystals it was clearing at the start of the think, with `wall_alarm` 10.

## Census, held out (seeds 21-36)

Deciding measures, named before the run:
1. Each style beats idle and random on 15 or more of 16 paired seeds, and survives Holm.
2. At least one style beats the scripted farm hand on 13 or more of 16, and survives Holm.
3. The best style per seed is split: no one style is best on 13 or more of 16.
4. The think costs well under a game second's steps.

All 19 comparisons are Holm-corrected together. Scores are `End::score`: a win is in (1, 3], a stalemate at the cap is the share minus 0.5, and a loss is in [-2, -1). The raw output is [census-21-36.txt](larder-world-ai-2026-11-12/census-21-36.txt).

| Hand | Median score | Ends (mould wiped / cap / squad wiped) | Against farm (higher / lower) | Best on |
| --- | --- | --- | --- | --- |
| idle | -0.780 | 0 / 8 / 8 | | |
| random | -1.786 | 0 / 0 / 16 | | |
| farm (scripted) | -0.029 | 0 / 16 / 0 | | |
| wall (scripted) | -0.029 | 0 / 16 / 0 | | |
| boom | +0.175 | 0 / 16 / 0 | 15 / 1, holds | 3 |
| claim | +0.175 | 0 / 16 / 0 | 16 / 0, holds | 6 |
| late | +0.173 | 0 / 16 / 0 | 16 / 0, holds | 4 |
| fort | +0.129 | 0 / 16 / 0 | 16 / 0, holds | 0 |
| strike | +0.035 | 1 / 15 / 0 | 11 / 5, p 0.21 | 3 |

- **Measure 1 passes.** Every style beats idle and random on 16 of 16, and all of them survive Holm.
- **Measure 2 passes.** Boom, claim, late and fort each beat the scripted farm on 15 or 16 of 16 and survive Holm. Strike does not (11 of 16).
- **Measure 3 passes.** The best style per seed is claim on 6, late on 4, boom on 3 and strike on 3. Among the AI styles no paired difference survives a correction: the closest are boom over strike 12-4, claim over strike 12-4 and late over fort 12-4, each p 0.08.
- **Measure 4 passes.** A think takes a median of 180 µs and is given once a game second (15 ticks). The step itself takes about 7 ms a tick on the CPU race world, so the think is about 0.2% of the time. The slowest single think was 3.4 ms. Laying a wall costs most, because `build_wall` runs inside the think's `apply`.
- **Strike is the high-variance plan.** It wiped the mould on seed 23 at 616 s, the only win on these seeds. It took the core and left under 600 walkers on 7 more seeds (7, 17, 208, 344, 371 and 574 walkers at the cap), but those still score as stalemates. On 6 seeds it scored below farm, because the soldiers it lost were not holding crystals at the cap.
- **Late against claim is 7-7.** Grabbing crystals in the last 90 s does as well as holding them from 10:00. The share is counted only at the cap, so a last-minute grab is a valid plan under today's scoring. This is a question about the score rule for the Larder thread, not a bug in the AI.
- **Fort's walls add nothing over boom** (boom over fort 10-5, p 0.30). W2b's one wall on the core-to-larder road added nothing over farming either.

## Fresh seeds 37-52

After the held-out run: idle, farm, boom, claim and strike ([census-37-52.txt](larder-world-ai-2026-11-12/census-37-52.txt)). Boom and claim are the final code. Strike here carries the mop-up below, which was rejected.

- Boom beats farm 15-1 and claim beats farm 16-0, both surviving Holm over 7. Claim beats boom 11-4 (p 0.12).
- Claim wiped the mould on seed 44, the first wipe by a plan that never attacks the core.

## Tried and rejected: a mop-up after the strike

After the core fell (no more births), every soldier not farming walked to the nearest mould on Free fire. On seeds 21-36 the strike with the mop-up scored lower on 6 seeds, higher on 1 and the same on 9 ([mop-21-36.txt](larder-world-ai-2026-11-12/mop-21-36.txt)). It wiped the mould on one seed either way: 36 with the mop-up, 23 without. The hunters left their crystals, and tens to hundreds of walkers lived to the cap anyway. A source comment in `Brain::think` records it. This rejection holds for the current scoring, a share at the cap. A score that counted the mould's size would change the answer.

## How to use it

```
cargo run --release --example larderworld -- census seeds=21-36 hands=idle,random,farm,boom,claim,late,fort,strike
cargo run --release --example larderworld -- census seeds=1-4 hands=farm,claim:claim_at=200+keep=4
cargo run --release --example larderworld -- look=DIR seeds=23 hands=strike every=150
```

## Next

- **The tier counter tests** (asked by the creature-rules thread, 11-12 00:17Z): run each `mould::tiers::Tier` (Sporer, Sleeper, Swarmer) against these styles, idle and random. Add knobs for the counters the styles don't cover yet: bait then strike, a kill zone, and starving the mould.
- **The GPU world needs `mould_grid`** (an occupancy read-back) before the AI sees there.
- **Two AI teams**, once the world has a second side (`apply_as`, a home per team).
- **The scoring questions** go to the Larder thread. The share is counted only at the cap (late ties claim), and a mould broken to a few hundred walkers scores as a stalemate.
