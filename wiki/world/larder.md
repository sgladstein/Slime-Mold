---
title: Larder, the mould as the enemy (Larder World)
kind: design
status: open
updated: 2026-11-12T08:03Z
sources: [Reports/gpu-readout-2026-11-12.md, Reports/larder-campaign-2026-11-12.md, Reports/life-gpu-2026-11-11.md, Reports/species-gpu-2026-11-11.md, Reports/larder-world-roadmap-2026-11-11.md, Reports/larder-maps-2026-11-11.md, Reports/mould-shooter-2026-11-11.md, Reports/larder-m1-2026-11-11.md, Reports/larder-m2-2026-11-11.md, Reports/larder-m3-2026-11-11.md, Reports/larder-build-plan-2026-11-11.md, Reports/rts-research-2026-11-10.md, research/sources/creeper-world-2025.md]
code: [Tracker, Game::begin_step, Game::settled_ticks, GpuWorld::frame_start, Engine::record_count, Game::take_share, Rules::home_dry, View::set_alive, BigParams::sand_keep, Dish::fire, Params::pieces, Params::wound, Params::light_weight, Dish::step_cost, Engine::set_squad, squad::frame, larder_world::Game, distance_map, GpuWorld, race_settings]
---

# Larder, the mould as the enemy (Larder World)

## Now

**Larder World is the line all Larder work now goes to** (owner, 2026-11-11
14:06Z: "Put everything into the world version"): a squad in a persistent
world on the GPU big dish, energy crystals that run out, several mould
species. Six gated phases, W0-W5
([roadmap](../../Reports/larder-world-roadmap-2026-11-11.md); live doc
https://claude.ai/artifact/1N2NHKefAzKxUMZcr3ktjJ), replacing M4-M6 below.
**Done:** W1a, the world's fields on the CPU big dish (PR #64); W1b, the
same fields on the GPU engine; W2a, the squad's hooks (PR #74); W2b, M3's
rules on the world, on the CPU and the engine; WM1, designed
[maps](maps.md) loaded on both.
**Playable:** `cargo run --release --bin larder-world -- map=fork` (W2c's first cut: the real rules on the GPU world, on a map). `mouldgpu`'s Q squad is the UI's stand-in, with no rules. **Smoke tests (CI, 2026-11-12):** `the_window_plays_every_mission` and `the_window_takes_every_command` play every mission file and `hand=every` (each command on the card pressed each game second) for 60 game seconds through `larder-world -- bench`, at scale 1 with the sound on tape, on CI's software Vulkan; a new mission file or card command is covered without a new test. Keep them green.
**Next:** the frame time on the 1070 (`larder-world -- bench=SECS` measures it without a window; a game step now waits once, on one packed readout: [report](../../Reports/gpu-readout-2026-11-12.md); the window draws while the GPU runs a step). The census on both engines (`larderworld census gpu=both`, `campaign gpu=both`) finds no bias over 32 seeds and five hands; the Rim mission's leak on the GPU is open (3 of 4 seeds hold, against 4 of 4 on the CPU). The maps' soil plays on the GPU since 2026-11-12. A stalemate now scores the charge taken and the mould's size (score v2).
**Campaign (W5), designed and approved 2026-11-12 04:44Z, not built:** eight missions on the v2 maps, each one verb against one tier, shown to the player as a goal sentence and bar with one to three stars, with a no-soft-lock rule and a dead-time measure; only unlocked verbs carry between missions ([design](../../Reports/larder-campaign-2026-11-12.md)). The mission layer (`mould::campaign`) and missions 1-3 are built: `larder-world -- mission=1-landfall` plays one, and `examples/campaign.rs` is its census ([report, *The mission layer, built*](../../Reports/larder-campaign-2026-11-12.md)). Tuned on the plain mould (four seeds): the AI meets missions 1-3 in 2:31-2:48, 1:42-5:31 and 10:00-11:41, idle and random never; mission 3 does not yet need walls, and missions 2-3 are scored on both engines since the GPU world reads its mould grid (#101) ([report, *Missions 1-3 tuned*](../../Reports/larder-campaign-2026-11-12.md)). In the window the Rim mould trails out the north-east gap by 5:00 on seed 1 ([pictures](https://claude.ai/artifact/Pu2j5ComXhUPTAMv4MyMRJ)), yet the GPU census holds that seed (peak leak 0.050, inside the 5% allowance): the trail is 2-5% of the walkers spread thin (at most 3 in a 4 x 4 bin), so the goal line now says the share outside against the 5% allowed ([report, *What the player sees outside the Rim*](../../Reports/larder-campaign-2026-11-12.md)).

## Larder World so far

**W1a** (PR #64): crystals that run dry or refill (`crystal_charge`,
`crystal_eat`, `crystal_regrow`), rock and wet terrain, a rectangular world,
and player walls (`build_wall`) that mould pressing on wears down to slow
rubble (owner, 15:24Z: walls must not seal a crystal for good). All on the
big dish, CPU, off by default. Wall health defaults to 400: a whole caged mould breaks out
in 20-30 s, while a passing tube never breaks a wall. Crystal charge flows
inside a crystal (`crystal_flow` 0.01), so the mould eats whole crystals.
With growth (scratch, no player), charge sets how long the mould lasts, not
how big it gets: charge 512 gives an idle mould about 10-12 minutes of
food on the 512 × 320 world
([report](../../Reports/larder-world-roadmap-2026-11-11.md), *Crystal
sizing with growth*).
Walls leak 0.9 of their scent, so a walled crystal still calls and a growing
mould gets through a ring in about 90 s; sealed, it walked past. Owner,
16:48Z: crystals are hexagons, and gunfire hits the first mould in its path.

**W1b**: the crystals, terrain and walls run on the GPU engine
(`Engine::build_wall`), and one GPU tick matches one CPU tick on nine world dishes, with all 17
planted bugs caught ([report, *W1b*](../../Reports/larder-world-roadmap-2026-11-11.md)).
(Fixed 11-11: a wall built on the GPU with growth on also moved the empty
slots under it, putting cell counts off and closing cells; the wall twin
test now has a growth dish, [species on the GPU, *A wall
fix*](../../Reports/species-gpu-2026-11-11.md).) On the GTX 1070 the world adds 0.25 ms to the frame at 1024 × 640 (1.73 to
2.11 ms). **W2**, Larder in the world, is planned in three steps: engine
hooks, M3's rules as a game layer, then the window ([report, *W2
plan*](../../Reports/larder-world-roadmap-2026-11-11.md)). **W2a** is
built (`mould::squad`, `Engine::set_squad`): shots stop at the first
cell that looks like slime (trail for 26/255) or at a wall, and strike a
disc; soldiers lay smell; each unit reads its touch and its densest block
in sight, and harvests crystal charge. One GPU frame equals the CPU's
exactly. On the 1070, 8 to 40 units add 1.0-1.5 ms a frame, mostly the
units pass running one thread a unit; W2c cuts it. A hit lays a tier's
alarm on the GPU too ([life on the GPU](../../Reports/life-gpu-2026-11-11.md)).

**W2b** (`mould::larder_world`, `gpu::larder_world`): M3's squad on the
world. Soldiers harvest crystal charge and carry it home, buy, fire, build
walls, and lose food at the larder to the mould. The game runs on the CPU
race world and on the engine, which takes `scale` engine ticks a game tick
to match the CPU's fine steps; a GPU test plays both for 20 s and they stay
within 8%. The first census found soldiers stuck behind rock firing into it
(13 kills from 80 shots), so soldiers now walk on a distance map and fire
only along a clear line (1,282 kills from 200 shots on the same seed).
On held-out seeds 5-20 idle and random lose on 16 of 16 and farm and wall
beat both 16-0; no hand wipes the mould, a farm holds 27-64% of the
contested crystals at the cap, one wall adds nothing over farming, and a
rush kills about 900 walkers and then dies, within 36 s on 10 of 16 seeds.
**Score v2** (2026-11-12, [report, *Score v2 and the dry crystal*](../../Reports/larder-world-roadmap-2026-11-11.md)):
a stalemate scores the squad's share of the charge taken over the game
(`Game::take_share`) and the mould's size against its start. The gate holds
16-0 under it, a carrier leaves a dry crystal and holds at the larder
(`Rules::home_dry`), and the farm hand, which had stood on emptied
crystals, now harvests twice the food and kills five times the walkers.
**Playtest fixes** (2026-11-12, [report, *Playtest fixes*](../../Reports/larder-world-roadmap-2026-11-11.md)): after
the owner's first playtest, shots are free (`Rules::price` 0, the fire
rate the only limit), and soldiers smell like prey (`Rules::smell` 0.5 over
`smell_r` 2 dish cells). A mould that turns from fresh gunfire (the alarm
the shots lay read at -1, `larder_world::FLEE_WEIGHT`, an opt-in) carves
the hole the owner asked for, 102 dish cells after 20 s of four soldiers'
fire against 29 ([page](https://claude.ai/artifact/5wQ3xSk8pBN6uig1rxKd4R)),
but as the default it failed the gate: random held the core, which the
mould no longer defended, and wiped the mould on 12 of 16 seeds. Without
it the gate holds on the race world (32 seeds), the Fork and the Rim, but
strike beats every plan (farm 28-4) with food left unspent. Next, the hole
from the toolkit's crater, then running costs. Off by default for other threads:
`Rules::overrun`, `Rules::reserve`, `Game::stuck`, `Game::density`.

## Small-dish milestones (M1-M3)

The evidence the world builds on: what the squad's tools do and why the
plans formed a ladder.

Brainstorm: [`mould-shooter-2026-11-11.md`](../../Reports/mould-shooter-2026-11-11.md).
A squad shoots the mould; both want the same food; the player's larder is a
pile the mould steals from. The owner chose a squad, solo, tests first, and
ruled that a held ring of fire should stop all mould: after a snip fix it
does (0 stolen, 16/16). **Priced** (every shot costs food, the larder runs
dry, fronts slowed to arrive at about 32 s), ringing forever is never the
best plan: once the larder is dry the ring guards nothing. But one plan,
ring the larder while it has food then mow the eaters off the other piles,
tops all 16 seeds at shot prices 10 and 40, and at 120 no shooting pays. It
is a ladder with a timing choice, not counter-strategies, because the mould
does not adapt to the squad. Three independent reviews are in the report's
folder.

**Build, M2** ([report](../../Reports/larder-m2-2026-11-11.md)): with
spending (soldiers, bait, turrets) and three mould answers (hoard smell,
hunger, drops), the gate fails on held-out seeds: no plan crosses over
between a fed and a lean field, and one plan (bait the mould away, farm,
expand once it is small) is best on 12-13 of 16. Hoard smell makes saving
cost and drops keep a lean mould alive; hunger barely acts. Next, per the
plan: one more mould answer that makes waiting lose against a lean mould.

**Build, M3** ([report](../../Reports/larder-m3-2026-11-11.md)): two such
answers (spores, a dormant crust) failed and were cut. What loosened the
ladder was the soldiers: firing from range instead of walking onto an eaten
pile, spreading shots, and a loose formation. On held-out seeds no plan is
best on 13+ of 16 any more. The window has groups, a scent view and replays;
its frame on Scott-Desktop is 7.2 ms mean with 40 soldiers. Next: the
owner's playtest.


## Owner's word

- 2026-11-12 04:39Z: "A group of soldiers shooting the slime should start carving out a bigger and bigger hole that doesn't refill while being shot at" (04:38Z, on two drawn hits: "Neither. Both of those look like stamps. Placed on the slime that fade. Maybe part of the issue is the slime/walkers move to fast.").
- 2026-11-12 04:13Z (first playtest): the slime did not feel like one organism; shooting killed many walkers but "visually it didn't impact much"; "the slime mostly ignored my soldiers unless they walked into the slime"; "energy required for shooting isn't fun".
- 2026-11-11 14:54Z (14:55: "just my initial thought", for playtests to settle): test crystals that run out against ones that don't. Run-out crystals go with the race (an idle player should be overrun), refilling ones probably with the landing. Walls first, the Sporer and Sleeper tiers and a player-owned species are agreed. A CPU world for small tests, and the 1070 for GPU work.
- 2026-11-11 14:25Z: the mould tier ladder is agreed, and it grows as research finds new behaviours. Walls or weapons can come first.
- 2026-11-11 14:14Z: the early leader's win rate is a per-mould difficulty, not a gate. There should be mould tiers from easy to hard, and player walls, upgrades and weapons, so strategy depends on the mix.
- 2026-11-11 14:09Z: also explore a race start, "both the player and the mold starting in one small area and then we're both trying to expand at the same time".
- 2026-11-11 14:07Z: a player AI thread for automated testing; later "two soldier teams against multiple molds so in the future the AI could also play soldiers".
- 2026-11-11 14:06Z: "A longer persistent world. Maybe a campaign. Crystal should run out which forces you to expand. Several species sounds interesting. Put everything into the world version."
- 2026-11-11 14:01Z: "You should also be moving your game into the big dish ... should become more of a world, and the food should probably become more something like energy crystals".
- 2026-11-11 04:04Z (card): "Both": food stops arriving late in the round, and a 15-game-minute cap ends any stalemate on share of the map held.
- 2026-11-11 03:38Z: "Start with 4 and can grow to 40. They should definitely die and be attacked by the slime. The round ends when the soldiers or slime are all dead?"
- 2026-11-11 03:14Z: "Rts-style. When you finish your current task, let's develop a full detailed plan of how to build the demo".
- 2026-11-11 03:03Z: "Yes." (to the priced-ring test).
- 2026-11-11 01:50Z: "Shooting should be able to directly cut through a branch after a free seconds and holding a ring of fire should stop all mold from getting to the food".
- 2026-11-11 00:30Z: "Squad, start just you against the mold, tests first" (and asked why a browser build, since the browser cannot run the GPU-scale game).
- 2026-11-11 00:18Z: "brainstorm a demo where the slime mold is the enemy. The player is a soldier or unit of soldiers who can shoot the mold to destroy parts of it. The player needs food to upgrade, build defenses, etc. and the slime mold wants food to expand/survive, etc,"

## Tried and rejected

- A ring of cuts raised round a pile once the mould is near (a ring held from the start works): [`dead-ends.md`](../../Reports/dead-ends.md), Larder entry.
- Spores and a dormant crust as the mould's fourth answer ([`dead-ends.md`](../../Reports/dead-ends.md), two Larder entries).
- The reader buying soldiers toward an army once the mould is small: the extra soldiers feed a starving mould ([`dead-ends.md`](../../Reports/dead-ends.md), larderm2 entry).

## Open

- **Maps** (owner, 19:29Z: "We will want maps though. That should be part of the plan"). Height is parked after the owner's 20:06Z challenge; ground is tested before it goes on a map. On 16 paired seeds every soil kind changes the mould: slow (wet) and barren ground halve it, sand makes it spread further, clay traps it (`BigParams::sand_keep`, `clay_keep`, `barren_upkeep`, off unless a map sets `BigDish::soil`). Five flat maps (Rim, Fork, Flats, Nest, Passes) wait on the map loader and the W2b harness ([report, *Which ground the mould feels*](../../Reports/larder-maps-2026-11-11.md), [page](https://claude.ai/artifact/XPwkBkTAzsWq3Wg37pff7K)).
- Larder World, phase by phase, per the roadmap; the open questions there (world size, whether two soldier teams are allies or rivals, fog of war) are the owner's; how a campaign mission ends and what carries are decided (11-12 04:44Z).
- The M3 window stays playable (`cargo run --release --bin larder -- 17`). Hunger barely acts on the small dish and needs a lower bar or should go.

- Make the mould answer the squad so plans can counter each other: smell growing with the bank, starving mould fighting harder at a ring, drops that pull fronts away, layouts where the best piles differ.
- The build plan, reviewed by three independent agents and revised ([larder-build-plan-2026-11-11.md](../../Reports/larder-build-plan-2026-11-11.md); Claude Doc https://claude.ai/artifact/5ricncRubPz7WTBGJZFnHs): M1-M3 built, M4-M6 replaced by the world phases (11-11 14:06Z). M1 ([report](../../Reports/larder-m1-2026-11-11.md)): fortress and farmer beat idle and random 16/16 held-out; four soldiers attacking die in about 45 s.
- Re-run smell on a dish the mould takes 30 s or more to cross.
- Versus (two squads, one mould) is set aside for now (owner, 00:30Z: solo first).

## Sources

- [larder-campaign-2026-11-12.md](../../Reports/larder-campaign-2026-11-12.md): the campaign's design (W5): missions, goal bar and stars, pacing, no soft lock, dead time.
- [larder-world-roadmap-2026-11-11.md](../../Reports/larder-world-roadmap-2026-11-11.md): the world plan, phases W0-W5 and their gates.
- [larder-maps-2026-11-11.md](../../Reports/larder-maps-2026-11-11.md): maps and ground kinds (WM2, WM3), a plan.
- [life-gpu-2026-11-11.md](../../Reports/life-gpu-2026-11-11.md): the tiers' life cycle and the shots' alarm on the GPU.
- [species-gpu-2026-11-11.md](../../Reports/species-gpu-2026-11-11.md): the species rules on the GPU, and a fix to walls built with growth on.
- [mould-shooter-2026-11-11.md](../../Reports/mould-shooter-2026-11-11.md): the whole brainstorm.
- [rts-research-2026-11-10.md](../../Reports/rts-research-2026-11-10.md) and the [Creeper World card](../../research/sources/creeper-world-2025.md): the nearest published game with a simulated enemy.
