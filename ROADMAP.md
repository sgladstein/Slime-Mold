# Roadmap: every active plan in one order

Written 2026-11-10 18:15Z against `main` at `865c802`. It puts the four
plans that are running at once into one sequence, shows what each step
waits on, and names which project thread owns it. It adds no new plan: the
detail, numbers and gates stay in each plan's own source, linked below.
[HANDOFF.md](HANDOFF.md) is still the front page, and the owner's open
decisions are listed only there (wiki rule 5); this page points to them by
number.

**Refreshed 2026-11-12 ~04:50Z** by the game integration thread: the paused
G, E and R bands (owner, 11-10 20:55Z: today's dishes are unlikely to be the
real games; the direction is RTS-type games, now the Larder World) moved to
*Archive* at the end, and the Larder World game threads got rows in *Now*.

The goal behind every line: interesting simulations that can be turned into
a game, not a realistic mould (owner, 2026-11-10 17:01Z). Fun over fidelity,
everything on screen is the simulation, and an outcome is a spread
([`wiki/decisions.md`](wiki/decisions.md)).

## The plans

| Track | Plan | Owning thread | Source |
| --- | --- | --- | --- |
| **G** Game fixes *(paused, archived)* | Keep It Alive's luck, The Wild One's middle, the snip, star bars, the forest floor | *Triage the previous session's hand-off* | Hand-off triage doc (https://claude.ai/artifact/XWm6u7y1UR3cfp2XxkhC7w); [HANDOFF.md, *Next steps*](HANDOFF.md) |
| **E** Engine checks and ports from Pixel_Physics *(done or paused, archived)* | Dish hash, profiler, parallel draw, licence gates, keyed random draws, sound | *Review Pixel_Physics for reusable code* | Pixel_Physics review doc (https://claude.ai/artifact/Pe83NVdRMXhPvxeYoKASRu) |
| **S** Scale (GPU) | Six gated phases, from CPU foundations to new play at scale | *Research Sage Jenson's physarum work* | [`Reports/gpu-plan-2026-11-10.md`](Reports/gpu-plan-2026-11-10.md), [the mould at scale](wiki/model/scale.md) |
| **R** RTS (Bloomfront) *(paused, archived)* | Five gated stages, from two keepers on today's dish to online play | *Explore turning the simulation into an RTS* | [`Reports/rts-design-2026-11-10.md`](Reports/rts-design-2026-11-10.md) |
| **L** Larder World | Six gated phases (W0-W5): growth on the GPU, a world with crystals, Larder in the world, several species, persistence, campaign; designed maps (WM1-WM5, owner 11-11 19:29Z) run beside W2 | *Larder* thread, with the GPU, creature, *Camera and UI* and *Player AI* threads | [`Reports/larder-world-roadmap-2026-11-11.md`](Reports/larder-world-roadmap-2026-11-11.md) (doc https://claude.ai/artifact/1N2NHKefAzKxUMZcr3ktjJ); owner ruling 11-11 14:06Z |

Thread names are the project's thread titles on claude.ai. When a thread is
closed, its track is unowned until a new thread takes it; say so in its row.

## The order

Work in a band can run in parallel; a step starts when what it depends on
is done.

### Now: the Larder World game (11-12)

While the owner can't playtest and the 1070 is busy (11-14), the work to run in the cloud, what waits, and the picks the reviewers settle are in [`Reports/cloud-plan-2026-11-14.md`](Reports/cloud-plan-2026-11-14.md).

Every row edits or reads the same game; who owns which shared piece, and
the merge order, are in *Larder World: shared rules and merge order*, below.

| # | Step | Owner | Waits on | State |
| --- | --- | --- | --- | --- |
| L-I | The game's shared rules: one owner and one code path per mechanic, the merge order, the GPU `Params` queue | *Integrate the Larder game's shared rules* thread | nothing | **agreed 11-12** with every thread below, after the technical lead and game designer reviews found scorch designed in four places and lures, alarms, walls and depots in two or three each. `Engine` Params layout asserts are real in release builds since 11-12 |
| L-F | The owner's playtest fixes (11-12 04:13Z): free shots, soldier scent as a rule; shots that leave a lasting hole | *Make the Larder World fight feel alive* (Larder) thread | nothing | PR #106: free shots and scent (0.5 over 2 dish cells); the gate holds on the race world (32 seeds), the Fork and the Rim, but strike beats every plan (28-4 over farm) and food does not bind. The mould turning from gunfire made the hole (102 dish cells against 29) but failed the gate (random wiped the mould on 12 of 16 by holding the core), so it is an opt-in; the slower clock was dropped too (both dead ends). #106 merged 11-12 07:37Z. PR 2 is #113: each hit lays the toolkit's crater on CPU and GPU (on by default, 10 s), and a walker standing in a crater stops a shot (without that the rush squad was wiped on 22 of 32): the hole after 20 s of fire is 60 dish cells against 29 (8 of 8); the gate holds on race 5-36, the Fork and the Rim (every plan beats idle 32-0 and random 24-8 or better, strike against farm 20-12), with random wiping the mould on 7 of 32 race seeds (2 without, p 0.06). A ragged crater edge was tried and dropped (dead end); two look switches (scattered strike, cone crater) wait on the owner's blind pick. **Watch item**: on race 201-232 (raise 0) every plan beats idle 32-0 but random only 20-12 to 23-9, none under Holm, and random wiped the mould on 11 of 32; the no-crater control has farm over random 25-7 (holds) and 5 wipes, so the craters cause about 7 of the 11 (p 0.07). Re-run at #122's build time (99c5e29, raise 8): farm, wall and fort beat idle 32-0 and random 25-7 (holds) on race 5-36, and 16-0 on the Fork and the Rim. Ruled (integration and both reviewers, 11:16-11:22Z): merge with craters on; next, PR 3 is core defence, ahead of the economy (a deliberate core-attack hand beats random, random's core wipes about 1 in 10, a failed attack costs the squad, tier-dependent; every plan over random after Holm on both blocks); then PR 4, the economy (running costs). PR 3 built (11-12 13:21Z): soldiers near the core smell like a lure (`Rules::core_pull` 8) and the `Core` type tiers set (`Tier::core`); over both blocks random wipes the mould on 5 of 64 (18 without) and every plan beats random after Holm on both; strike still beats farm about 2 to 1 (food doesn't bind). PR #126; the twins' GPU gap is chaos (one walker more moves the CPU as far, 8 seeds); on the Fork and the Rim random never wipes the mould on or off, and the rush loses to random on the Fork either way (the map). The crater looks as clips for the owner's pick (11-12 14:40Z, [page](https://claude.ai/artifact/HzgDT3XPLX59Ba621tVH2H)). PR 3b (11-12 16:45Z): look 3, the cone, is the default and soldiers wade in thick slime (`Rules::wade` 6, the owner's 14:41-14:43Z rulings); over both blocks random wipes the mould on 4 of 64 and every plan, the rush included, beats random after Holm. The crater's time: the owner asked for shorter (15:18Z: "3 fills in too slow") and picked 4.5 s from the volley clips (17:29Z), now `Rules::crater`'s default; the 6 and 3 s gates were stopped under the owner's 17:26Z testing rule (a census is not a gate for a look). The Trunk's core was built (11-12 17:40Z, #163): `CoreKind::Regrow { starve_secs }` falls and grows back unless the mould eats nothing in the window; PR 4 takes it out after the owner's no-king ruling (20:21Z, every mould headless; dead-ends.md). The cone's crater times are filmed with the squad firing into the thick ring (`volley` hand; the owner's 16:55Z ask). PR 4, the economy (11-12 20:50Z; `Reports/larder-economy-2026-11-12.md`): rations (`Rules::ration` 1 food a second a soldier; hunger costs hit points only, so no spiral) and the overrun on (20 s); on race 5-20 idle starves in 16 of 16, fort's unspent harvest 80% to 43%, strike's 64% to 17%, strike over farm 12-4 (not under Holm). The core rule is not why plans stalemate (`core=headless`: farm, wall and fort cap 16 of 16 either way; the player wins 2 of 112 without it, 6 with). Shots sink in (11-12 23:40Z, the owner's 23:14Z "Yes"; `Reports/larder-shot-sink-2026-11-12.md`): a hit's blast sinks one strike radius into the slime (`Rules::sink` 1, both engines); on shape C standing fire kills 20.3 a shot (12.6 before), walking in 47.9 (42.0). Wading counts every walker (11-13, the owner's 15:55Z "why are they not way slowing down"; `Reports/larder-wading-2026-11-13.md`): `Rules::wade` no longer scales by `Rules::mass`, so on shape C 12 walkers hold a soldier to 20% of its speed (86% before); walking into the body was already deadly on main since #174's bite (eight soldiers dead 5-7 s after contact). A rule keeping a soldier's blast off its own touch circle was built and dropped (the owner, 16:15Z: a soldier rushed while standing still must be able to shoot what reaches it; dead-ends.md). A unit now aims only at slime it can see (`squad::sees`, both engines; #198): a squad by a wall fired 47 shots in 26 s, now 198. Open: the owner's crater-length pick (`crater=2` or `scorch_keep=0.5`). Next: buildings' running costs, the player AI feeding its army (its thread) |
| L-O | One body: the mould holds together, and a cut-off piece starves | *Make the mould one cohesive body* thread | nothing | **CPU rules merged 11-12 (#103)**, all off ([report](Reports/one-body-2026-11-12.md), [page](https://claude.ai/artifact/E9MSpcgBj7F8jZX6i6LA5q)): `energy_flow` 0.05 + `hunger_scent` 8 is one body on more seeds and 2.5x the size; cut gaps refill in 2 s unless held. GPU port of those two (binding 46, `Params` row 848) twin-tested, PR open. Larder census with them on (11-12 10:15Z): gate holds for farm and wall, rush stops winning, every hand scores lower; off by default pending the owner's A/B. Hunks on maps eat in 2 s but scatter; align rejected. Matched size (13:00Z): no level of the two rules gives one body at today's difficulty (hunger sets the size, fast sharing makes the mould starve whole); options for the owner: a Colony tier, a new rule shape, or looks only. Owner 14:36Z: both look like individual specks; game-look films on the page. Lone step (a walker with few neighbours slows; CPU and GPU, `LONE_AT` 880, off) thickens tubes and holds a moving hunk together (0.72 vs 0.46 on Fork) but the frontier is still specks and the mould spreads slower. Owner 16:11Z asked why Jones's and Jenson's moulds don't speck: density. At 15-20% of cells (25k walkers, upkeep 0.0002) Fork draws as smooth tubes; trail alone at today's 3-5% still beads. The dense mould killed the idle squad in 78 s, so damage must count mould mass, not walkers (one factor, agreed through L-I). **Deep review 11-12 17:00Z** ([report](Reports/slime-enemy-review-2026-11-12.md), [doc](https://claude.ai/artifact/6Jb8uKgtnHrgP7vy7cqKyZ)): five reviewers, all keep Jones and add layers. The 25k films ran with one-walker-a-cell off (GPU drops it when slots pass half the agar; the GPU thread is fixing the check); with it on, 25k walkers are a lasting mesh or one continuous sheet, no specks. Next, in order: the one-walker rule at density (8 seeds), sensor 22.5° + offset 5, the drawing (9x9 body window), a packed slow front with density-gated births, a piece layer with creep, a coarse supply grid; the mass factor with the Larder thread. Game-rule findings (king rule, idle survives, food unbound) sent to the integration thread. **Owner 20:21Z: no king rule**, every mould headless (`Core::of`; built 11-12, this thread); the owner playtests the shapes (sheet, mesh, trunks) before one is picked. **C picked (22:19Z); 11-13 C shares its food by default** (`energy_flow` 0.05, the window's shape C only; the owner could not see it grow from its crystals): 17.1% of the screen at 5:00 against 7.0% ([report](Reports/fed-mould-2026-11-13/README.md), blind A/B page); **the owner picked it in the A/B (11-13 02:39Z, "Definitely film 1")**. Cut off from every crystal, a part of it stays whole 75 s+ then dies at once; `flow_loss` 0.05 (now on the GPU too) makes that 35-50 s and C 30% smaller. **Withering** (`wither`, both engines, twin-tested; #195): news of food travels through the body and a part with none for 5 s thins steadily (uncut C unchanged on 4 of 4 seeds). **Owner 11-13 15:51-15:53Z picked it at half the speed: on in shape C at 0.01** (74% of a cut piece at 20 s, 15% at 90 s), for the owner's playtest. **A cut-off part panics** (owner 15:57Z, "act more like a scared animal", no stronger bite; `panic` 2, both engines, twins exact, on in shape C): after 5 s with no news it leans 1.5 s, then charges a unit within 20 cells at twice its pace or runs for a crystal with food ([films](https://claude.ai/artifact/SRxAj8nri2iGAsvE4DFGbP)); smoke unchanged; awaits the owner's playtest |
| L-C | Soldier controls: order queue, attack-move, moving on from a dry crystal, the larder's rally point; then building over time and the larder as a depot | *Improve soldier controls and commands* thread | part 2 waits on buildings batch 1 (soldiers build over time: owner's card, 11-12) | part 1 built (branch `claude/project-thread-oldrb7`), census gate with moving-on on and off running |
| L-B | Buildings: turret, lure, pump, wall and outpost as one `Building` type; then siren | *Add turrets, walls and other buildings* thread | no Scorcher (owner's card, 11-12 04:46Z) | **built 11-12** ([report](Reports/larder-buildings-2026-11-12.md)): turret, lure, pump, outpost and wall on the CPU and the engine, twin-tested; `UNITS_MAX` 64 to 96; build page Q. Census: guns at the crystals (turtle, guard) win the race world on every tier and an army hand ties them, so food not binding is the cause (economy retune: Larder thread). Siren built 11-12 on the toolkit's burst (a Swarmer floods to it; the plain mould ignores it). After 8 s builds, craters and core defence: soldiers best on the race world, turrets on the nest |
| L-K | Squad toolkit from the model: blast and crater, mine, det cord, germ, sleep gas, mask and reek | *Design squad tools from the mould's model* thread | nothing (owner picked "all six", 11-12 04:31Z) | **CPU prototype built and measured 11-12** ([report](Reports/squad-toolkit-2026-11-12.md), [page](https://claude.ai/artifact/Vr6ysJedMVtEa6XXEA1pTC), [wiki](wiki/world/squad-toolkit.md)), off by default: the crater keeps a blast's hole open (8/8), the germ kills once sickness costs energy, gas stops eating and noise wakes it, mines weak, the det cord waits on food flow. Four plans beat idle and random on every tier. **Round 2 (11-12, [report](Reports/squad-toolkit-round2-2026-11-12.md))**: tools take carry space and come from the larder (a runner holds 5); the best plan now changes by tier (loud on Sleeper, Trunk, Drifter, Wary; plague on Swarmer, Sporer, Sprawler; 8 seeds, by the mean). **32 seeds after #113**: the split holds after Holm on the Drifter (loud) and Sporer (plague); random beats or ties every plan on the Drifter and Trunk. **Step 2 started (11-12)**: tools are bursts (`squad::Burst`, `World::burst`); blast, cord and mine run on the GPU world (the strike lays the crater; twin on software Vulkan). in the window E opens the tools page. Next: germ and gas on the GPU, the tools as bursts on binding 47's crater block (the Larder thread built the GPU crater, #113) |
| L-S | Sound and hit feel driven by the simulation's counts | *Add sound and hit feel to Larder* thread | nothing | **merged 11-12 (#96)**: reads only `Game` fields, no read-back; the heavy-shot threshold scales with density to the power 0.3. Next: `Game::density`, then sounds for the toolkit's bursts. Also the mould's one-body voice (game designer's design, asked for by the coordinator) |
| L-G | The world's look: shot damage drawn from the simulation | *Improve soldier and slime graphics* thread | the crater field (L-K) | the owner turned down both drawn versions (11-12 04:38Z, "Both of those look like stamps"); next a hole that is real in the simulation, and slower walkers |
| S-P | GPU performance and census: the game's frame time on the 1070, one read-back a step, the census on `GpuWorld` | *Port mould species and tiers to the GPU* thread | the owner's desktop for the 1070 timing | planned 11-12: a mapped readout buffer and a living-walker count pass; `Game::step` split into begin and finish halves with `step()` unchanged |
| L-OM | The player's own mould species (approved 11-11 14:54Z) | *Design the player's own mould species* thread | nothing (gardeners picked 11-12 04:45Z) | probe merged (#105); CPU gardeners build up (see L-O, Larder World) |
| L-CA | Campaign and onboarding: a goal the player can read, no soft lock, missions as map data (row W5) | *Design the Larder campaign and onboarding* thread | nothing (owner approved 11-12 04:44Z) | design merged (#95); layer and missions 1-7 built (row W5). Its `larder_world.rs` asks (an overrun loss, a reserve recruit, a stuck measure) ride in the Larder thread's PR 1, off by default |
| L-AI | Player AI for Larder playtests (`src/mould/larder_ai.rs`, `src/mould/larder_world_ai.rs`, `examples/larderai.rs`, `larder -- SEED ai=STYLE`) | Player AI thread | nothing | **built 11-11** on the CPU field: [report](Reports/larder-ai-2026-11-11.md), [page](wiki/world/player-ai.md). Plays through `Cmd`s only, sees a `View`, one brain per `Side`; five styles; boom, lure and adapt beat idle and random 16/16 held out. **Ported to the Larder world 11-12** (`src/mould/larder_world_ai.rs`, hands in `examples/larderworld.rs`): [report](Reports/larder-world-ai-2026-11-12.md); boom, claim, late, fort beat the scripted farm 15-16/16 held out, strike wiped the mould once. **Plan-by-tier matrix 11-12** ([report](Reports/larder-world-tiers-2026-11-12.md)): 7 plans × 8 tiers, seeds 53-60, main before craters; strike best on 7 tiers, the Sporer flips it to claim. Next: re-run after craters and core defence with toolkit and buildings hands (fresh seeds from 301, held-out map Rim), new hands (turret ring first), two AI teams once a second side exists, exploiter search |
| L-T2 | New tiers for the campaign ladder: the Drifter, Sprawler and Trunk (missions 1, 3 and 4 need them), then habituation (a mould that learns to avoid or swarm your last tool) | *Build new mould tiers for the campaign* thread (started 11-12 05:10Z) | nothing | **Built 11-12** ([report](Reports/mould-tiers-campaign-2026-11-12.md), [wiki](wiki/world/mould-tiers.md)): `Tier::{Drifter, Sprawler, Trunk}` on CPU and GPU (GPU within 2%), `Tier::Grudge` on the scar field (CPU, off by default; 8/8 seeds). Pre-crater census: Grudge hardest, strike best on every tier, the Drifter gentlest. Wary removed (random fire herds it off every crystal; dead end). Next: per-tier core values on the Larder thread's Core (Sprawler headless, Grudge pull), census with craters on 201-232, turtle and guard against Grudge, the scar's GPU port (binding 47 block 1, Params after 880) |
| L-PT | The morning playtest: one combined build on `main` and one page, "what changed, what to try" (each change, its key or command, its counts), so the owner plays the sum of the night's merges | integration thread | whatever has merged by about 11:00Z | done 11-12 11:10Z on a8669bd: [page](https://claude.ai/artifact/4aEKu43Wg4EMmTZen4oVo4), `Reports/morning-playtest-2026-11-12/` |

### Built and building: the scale and world steps

| # | Step | Owner | Waits on |
| --- | --- | --- | --- |
| S2 | GPU phase 2: offline 1024 x 640 with about 100k walkers, paired seeds; GPU timing test | GPU thread | offline dish **done 11-10** on the engine-only dish (economy left out under the owner's 20:55Z ruling): 32k and 100k walkers, videos and counts on the [4x Mould page](https://claude.ai/artifact/WMfUwTj9Dfdwp9MCfy8MHs). Timing test built (`examples/gputime.rs`: four wgpu compute passes a tick; on a software GPU its dish matches the CPU's, 165 loops against 163 at 100k). **Timing done 11-11** on the GTX 1070 through Remote Control: 100k walkers take 1.37 ms for a frame of two ticks, 1M (no crowd limit) 3.52 ms, against the 8 ms bar; the GPU dish matches the CPU's (115 loops against 114 at 20 s) ([plan, *GPU timing on the GTX 1070*](Reports/gpu-plan-2026-11-10.md)). *Review 11-11:* one seed, and that was the gate's timing part only; raw now kept |
| **Gate** | **The owner picks a target scale and look, or stops** | Scotttt | **decided 22:32Z** (card): 100,000 walkers on 1024 x 640; the GPU timing passed on 11-11, so the GPU goes ahead to S3. The owner asked (11-11 00:06Z) for every option the Jenson comparison tested as a sandbox control: that is S3 |
| S3 | GPU phase 3: the sandbox, every Jenson-comparison option as a control | GPU thread | **built 11-11** (`cargo run --release --bin mouldgpu`, [report](Reports/gpu-sandbox-2026-11-11.md)); on the GTX 1070 100k walkers take 1.45 ms of GPU a frame with drawing, 1M 8-10 ms ([page](https://claude.ai/artifact/1rdrxadVcNdxTmbxp2KJGy)). The twin test runs in CI on software Vulkan (one GPU tick against one CPU tick, to the bit on keys 1, 3, 4 and 5). **Independent review 11-11:** fed scent and jumps after the step added on the GPU; then on 8-16 seeds, on the software GPU and the 1070, no measure differs from the CPU after Bonferroni (report, *Independent review*). **Looks as controls 11-11:** the looks thread's rules (disc and haze, trail-set settings, deposit cap, floor, wander, contagion) on two panel pages and keys 9, 0, Shift+1-5, twin-tested on six dishes (to the bit on four, within 1.1e-7 with the cap; corrected 11-11); off they cost 0.01 ms a frame on the 1070, contagion 0.3 ms at 100k. **Outside rules as controls 11-11:** species (bands, cyclic, conversion, species 0's boost), explorers, blocked lay and keep, chance turn, size spread, area sensors, grazing; a fourth panel page and Shift+6-0, Ctrl+1-2; twin-tested on fifteen dishes (every walker, age and species; fields within 2.4e-8); compiled out when off. Next: the 1070's timing of these; the owner's hands; the tools as field writes. `species_share` and the other species rules are on the GPU engine (W3a) |
| W0 | Larder World: growth and starving per walker, CPU and GPU twins | GPU thread | **merged 11-11 (#73)** ([report](Reports/growth-2026-11-11.md), [wiki](wiki/model/growth.md), [page](https://claude.ai/artifact/EGgjauGkFxneuSRz8zTBei)): off by default; the GPU matches the CPU in a one-tick twin test in CI; fed 96,358 against starved 428 alive at 30 s (8 of 8); on the GTX 1070 +0.16 ms on a 1.82 ms frame at 100k; sandbox GROWTH page. With W1a merged: a walker's crystal take is food. On crystals size follows crystal area, not charge ([mould shape](Reports/mould-shape-2026-11-11.md)); `energy_flow` (CPU, off) turns the hoard into 15-24% more walkers. A GPU grazing-count fix (counts cleared only with `graze_w`) followed in its own PR |
| W1a | Larder World: crystals (run dry or refill, flow inside, hexagons), rock and wet terrain, rect world, player walls with health that leak scent; CPU big dish | Larder thread | **done 11-11**, PR #64 ([report](Reports/larder-world-roadmap-2026-11-11.md): defaults `wall_hp` 400, `wall_scent` 0.9, `crystal_flow` 0.01; with growth, charge sets how long the mould lasts, not how big it gets) |
| W1b | Larder World fields on the GPU engine (crystals, walls, terrain), checked by the CPU/GPU twin test | Larder thread (agreed with the GPU thread 11-11 17:02Z) | **built 11-11** ([report, *W1b*](Reports/larder-world-roadmap-2026-11-11.md)): one GPU tick matches one CPU tick on nine world dishes (fields within 3e-9, bits exact), a wall built on the GPU matches the CPU's, and all 17 planted bugs are caught. On the GTX 1070 the world adds 0.25 ms to the frame at 1024 × 640 (1.73 to 2.11 ms, three runs). PR #74, after #63 and W0 (both merged); #69 rebases on it |
| W1c | Crystal sizing on a world big enough that an idle mould keeps growing for minutes (the race start); crystals of different sizes | Larder thread | Folded into W2b (11-11): the idle hand on the W2 world, measured with the game's own harness, is this test |
| W2a | Larder in the world, engine hooks: per-unit touch, densest mould and harvest (`squad`), soldier smell, shots that stop at the first mould and strike | Larder thread | **built 11-11** ([report, *W2a*](Reports/larder-world-roadmap-2026-11-11.md)): one GPU frame matches `squad::frame` on the CPU exactly (16 shots, 15 units), all 16 planted bugs caught after the test was strengthened twice; on the GTX 1070, 8 and 40 units add 1.0-1.5 ms to a 2.1 ms frame at 1024 × 640, most of it the units pass's one thread a unit (to cut in W2c). Bodies as crystals move to W2b. PR #74 |
| W2b | Larder in the world, game layer: M3's rules over the world (orders, stances, harvesting crystals, walls, buying, replays) | Larder thread | **built 11-11** ([report, *W2b*](Reports/larder-world-roadmap-2026-11-11.md), [page](https://claude.ai/artifact/XJa5MHiudUsVVnUwyaXHLP)): `mould::larder_world` on the CPU race world and `gpu::larder_world` on the engine (`scale` engine ticks a game tick; a GPU test keeps the two moulds within 8% over 20 s). Soldiers walk distance maps round rock and fire only along a clear line (the first census had a rush put 196 of 200 shots into rock). Gate passed on held-out seeds 5-20: idle and random lose 16/16; farm and wall beat both 16-0 (p < 0.0001, Holm); no hand wipes the mould, one wall adds nothing over farming, a rush dies. Score v2 (11-12): a stalemate scores the charge taken and the mould's size; the gate still holds 16-0, the dry-crystal rule is on, and the fixed farm hand harvests twice the food. Turrets, bait, hunger and bodies not ported; the units-pass speedup moves to W2c |
| W2c | Larder in the world, the window: camera, controls and marks (#63), the look (#69), the game UI (#80), on `GpuWorld` | Larder thread, with the Camera and UI and graphics threads | **first cut built 11-12** ([report, *W2c*](Reports/larder-world-roadmap-2026-11-11.md)): `cargo run --release --bin larder-world -- map=fork` plays W2b's game on `GpuWorld` on a v2 map, with the camera, look, UI and real shots (rays to the first slime, kills, wounds); the owner's `mouldgpu` playtest had found the UI's stand-in squad, which has no rules. Two fixes rode along: no specks for dead walker slots, and the game's cuts leave the look's wound. The ground rebuild is down to 1.7-1.8 ms (11-12). Soil plays on the GPU since 11-12 (WM3). **Controls part 1 (11-12, soldier controls thread):** Shift order queue, attack-move, harvesters move on from dry crystals to safe ones, larder rally point, zoom to 24 ([report](Reports/soldier-controls-2026-11-12.md)); part 2 (11-12): soldiers raise buildings, `BuildRules::raise` 8 s, the gate holds 8-0. Part 3 (11-12): soldiers can't stand on a crystal (`Rules::crystal_solid`), they work from its rim. **One wait a step 11-12** ([report](Reports/gpu-readout-2026-11-12.md), GPU thread, from the technical lead review): a game step's four read-backs and the mould count's two whole-buffer reads are one packed readout (4.13 read-backs a step to 0.07), the engine counts the living walkers and sums the mould grid, the mirror follows rubble, `larder-world -- bench=SECS` times the window's frame without a window, and `larderworld census gpu=both` plays the census on both engines: over 32 seeds and five hands no score or size difference survives Holm. The step in halves (the window draws while the GPU runs a step), the mould grid once a game second, `campaign gpu=both`. **On the 1070 11-12** ([report, *On the GTX 1070*](Reports/gpu-readout-2026-11-12.md), GPU thread): the squad's unit passes on a workgroup each (at 100k walkers units 10.45 to 0.07 ms a step, smell 3.88 to 0.05, harvest 0.78 to 0.42; the first version raced on the 1070 and not on lavapipe), and a step spread over the frames until the next: at 100k walkers every frame p95 6.62 ms (20.39 with the step in one frame), so the frame part of the gate holds. Twins on the 1070 before a GPU PR merges ([wiki](wiki/model/gpu-sandbox.md), *Twins on the 1070*). The crowd limit holds on the GPU as on the CPU (it counted the slots and dropped above about 7,300 walkers), and overlapping cone craters no longer race (11-12, [report, *Two GPU fixes*](Reports/gpu-readout-2026-11-12.md)). Left: the Rim mission on both engines over 32 seeds (campaign thread; 4 of 4 hold on the CPU, 3 of 4 on the GPU); gate: the owner plays it |
| W3a | Larder World, several species on the GPU engine: the creature thread's species rules (species 0's share, conversion weighed over a square, a second kind of walker in one mould, each species' own settings), then the tiers' life cycle (spores, a dormant crust, the alarm, the strike) | GPU thread (agreed with the creature and tiers threads 11-11) | **Species settings built 11-11** ([report](Reports/species-gpu-2026-11-11.md)): off by default; one GPU tick matches one CPU tick on 12 dishes and a growth dish, every row sum of the square matches the CPU's, and all 28 planted bugs are caught; on the GTX 1070 they cost nothing off, two species pay 0.05 ms a frame more (2.08 to 2.13), the second kind 0.12-0.19 ms, and the square of radius 8 1.3 ms of a 2.3 ms frame. A W1b fix rode along: a wall built with growth on moved the empty slots under it (the wall twin test gains a growth dish). **Life cycle built 11-12** ([report](Reports/life-gpu-2026-11-11.md); the owner asked 11-11 20:12Z, through the tiers thread, for the tiers in the game): spores, dormancy and dying asleep (`sleep_death`), the alarm field, the player's strike and the squad shots' alarm on `gpu::Engine` under the CPU's keys (`Tier::settings`, the sandbox's `tier=sleeper`), off by default, no new binding. One GPU tick matches one CPU tick on 18 dishes (the three tiers among them), the squad's frame on four, and 74 of 77 planted bugs are caught (the other three harmless, or a race software Vulkan cannot show). Off it costs nothing measurable on the software GPU; on, the life passes take 2.6 ms a tick there at 100,000 walkers (`fate` 2.2). On the GTX 1070 (11-12, before memory was freed) each tier adds at most 0.74 ms a frame at 100,000 walkers ([report](Reports/life-gpu-2026-11-11.md), *On the GTX 1070*). Next: the tiers in the GPU world game |
| L-O | Larder World: the player's own mould, gardeners (owner, 11-12 04:45Z): a second species the soldiers feed, lay roads for and cut, fighting the enemy by conversion; `src/mould/own_mould.rs` with a thin `larder_world` hook | Player's own mould thread | nothing for the CPU build; the GPU port after the toolkit's and one body's rows | **Probe done 11-12** ([report](Reports/player-mould-2026-11-12.md), [wiki](wiki/world/own-mould.md)): forms trade eating for fighting (network out-eats hunks 14/16; dense forms win contact 7/7); `species_convert_per` (off, CPU). **CPU gardeners build 11-12** (`own_mould.rs`, `World::tend` hook, [page](https://claude.ai/artifact/7Z4b4NB1JG1H7QEf9TrUer)): sowing grows the player's mould 13x farm's (32/32) but doesn't shrink the enemy. Each side capped at half the slots (`species_cap`); a taken core stops only the enemy's births. Front probe: road pushes, sow thickens, fire (with craters) burns the enemy back, each in 20 s. An enemy that hunts the player's mould feeds it (converts on arrival), so hunt is off; one det cord does about what fire does but does not hold. Front hand built ([page](https://claude.ai/artifact/Rv4ZqMJ1hQDnJKiVcAXihx)): enemy smaller than sow2's on 6/8. **GPU port and window 11-12** (`larder-world own=1`, off by default): Sow on the command card, Road and the det cord on the tools page, shots spare the player's mould; 1070 twin passed (failures as main's), timing booked. Next: `species_cap` on the GPU, an attacker's edge in conversion |
| L-T | Larder World: mould tiers on the ladder (Sporer, Sleeper, Swarmer): a life cycle per walker and an alarm the player's fire lays, on the CPU big dish; `mould::tiers::Tier` names each for W2b | Mould tiers thread; the GPU port is W3a's | nothing. **Built 11-11** (#70, then the world re-test: [report](Reports/mould-tiers-2026-11-11.md), [reviews](Reports/mould-tiers-review-2026-11-11.md), [wiki](wiki/world/mould-tiers.md)). In the world race the Swarmer and the Sleeper's bait hold 8 of 8; the Sporer's edge is not significant. On the GPU engine too since 11-12 (W3a). Next: counters against random and idle hands with W2b's player AI; review 2's candidate tiers (supply lines, habituation) |
| WM1 | Maps: a map file and loader. A map is pictures plus a short text file: ground kind per cell (agar, rock, wet, wall and its health), crystal centres, sizes and charges, the mould's start and the squad's start. It loads into the CPU big dish and the GPU engine, and today's noise worlds can be saved as maps | Larder thread (it writes W1b's fields) | **built 11-11** ([report, *WM1*](Reports/larder-world-roadmap-2026-11-11.md)): `mould::map` reads `maps/<name>.png` (G: wet, rock, soil) and `.txt`; `BigDish::with_map`, `Engine::with_map` and `GpuWorld::with_map` build the world with each crystal its own size and charge, the blob on `mould`, the larder on `landing`, walls with their health; `larderworld map=NAME` runs a census on a map. Gate passed: the v2 Fork and Nest play within 1% on the CPU and the GPU (soil off on both), and a saved noise world loads back hash-identical. Found on the maps: a blob by the rim placed 3,752 of 5,000 (fixed); a soldier waits at a dry crystal for a full load (open: four rules that send it home each cost the planned hands, since the score counts a crystal with crumbs left as held). Soil on the GPU too since 11-12 (WM3's GPU side) |
| WM2 | Maps: the first designed maps, three to five, each made for a different plan (for example a race out of one corner, a choke between rock with crystals behind it, ground split by wet land, a world that is mostly mould) | *Design Larder world maps* thread, with the Larder thread | Planned 11-11 ([plan](Reports/larder-maps-2026-11-11.md): five paper maps, each drawn for a different winning style; loader files in `Reports/larder-maps-2026-11-11/maps/`). Waits on WM1 and W2b's harness. Gate per map: idle and random hands lose; across the maps the winning plan changes (a map one plan wins everywhere adds nothing) |
| WM3 | Maps: ground kinds the simulation uses, beyond rock, wet ground and rubble. Brainstormed from the model (what walkers and fields can respond to: step speed, how fast trail and scent fade or spread, where walkers may lay trail, food that grows back, ground the mould changes as it passes), not from the demo tools. Each kind is a field, twin-tested on CPU and GPU, and kept only if it changes a measured outcome and can be seen | *Design Larder world maps* thread; GPU side with the Larder thread | Ranked 11-11 ([plan](Reports/larder-maps-2026-11-11.md)). **Height parked** (owner, 20:06Z: "How much does height actually influence our slime mold behaviors? I am skeptical how much fun/complexity/strategy this adds"): the first step is sand, clay and barren ground, which act on the mould's strong terms, on paired seeds with wet ground as the speed baseline (maps thread, CPU big dish, off while no soil is set); then rebuild the maps on the graphics thread's terrain (#69). Ground word: soil kind in bits 12-15, mould-made ground 16-23, byte 3 reserved for height and left 0. **GPU side built 11-12** ([report, *Soil on the GPU*](Reports/larder-world-roadmap-2026-11-11.md), [page](https://claude.ai/artifact/1ifC4Qrw6fXe5Aq7vVSRzZ)): the engine reads the kind from the ground word and plays the CPU's three rules. One tick on soil bands matches the CPU to the bit, and the four soil maps' game twin stays within 2.1%. In the window the Rim's sand ring now holds the mould in its clay core. No step cost on software Vulkan |
| WM4 | Maps: the ground's look, drawn from the ground's fields (the picture is the data). #69 gives rock, wet ground, walls and crystals height and light; open agar is still dark because nothing is stored there but trail and scent, so WM3's fields are what the open ground can show | graphics thread | #69, WM3 |
| WM5 | Maps: an editor in the window (paint ground kinds, place crystals and starts, save, play) | to be chosen | WM1, W2c. Later: W4's persistent world and W5's campaign are made of these maps |
| W5 | Larder World campaign: eight missions on the v2 maps, one verb against one tier each; a mission layer over `larder_world::Game` (goal bar, stars, lose causes, reserve recruit, hints) as data in the map files | *Design the Larder campaign and onboarding* thread, with the Larder and integration threads | **Design approved 11-12 04:44Z** ([design](Reports/larder-campaign-2026-11-12.md)). Next: the mission layer, then missions 1-3 (Flats, Fork, Rim), gated per mission (idle and random lose; the verb's plan beats the plan without it; dead time under 15%). Missions 1-3 merged (11-12/13); **missions 4-7 built 11-14** (census v13: two winning plans each, idle and random lose; Spore season is now a held charge race); mission 8 (your own mould against the Grudge on the Passes, D4) next, after the window plays `own=1` on a map and mission |

The two branches after the gate:

| | If the GPU goes ahead | If it stops |
| --- | --- | --- |
| Drawing | E3 (multi-core renderer) is dropped; the GPU draws | E3: port the Pixel_Physics multi-core row renderer |
| Engine | S3 sandbox (read Bleuje, Lague, PolyPhy first), S4 game rules on the GPU (needs Jones 2011 uploaded), S5 port the dishes that survive | stay on the CPU |
| Forest floor (G5) | built after S3, as a large world | built small on the CPU |
| Bloomfront Stage 3 (three or four keepers, big map) | on the GPU; the rival becomes a second whole dish in S4 | the plate-and-colonies split on the CPU, frame under 10 ms on the owner's machine |

### Last

| # | Step | Owner | Waits on |
| --- | --- | --- | --- |
| S6 | New play that only scale allows | GPU thread | S5 |

## Larder World: shared rules and merge order

Agreed 2026-11-12 ~04:40Z by the game integration thread with the seven
threads that edit the Larder World game at once. The owner's standing
order still holds: tools and feel are built in parallel (owner, 11-12
04:35Z, "I want to work on tools and feel in parallel").

**One owner per mechanic, one path through the code.**

| Mechanic | One path | Owner | Users |
| --- | --- | --- | --- |
| Scent | `squad::UnitIn.smell` / `smell_r`: everything that smells is a unit in the squad frame | Larder thread (`Rules::smell`, the hoard) | soldiers; the lure (a still unit); mask and reek (per-soldier multipliers on `smell`) |
| Harvest | `squad::UnitIn.take` / `take_r` | Larder thread (rule); controls thread (where harvesters go, moving on) | soldiers; the pump (a still unit) |
| Shots | `squad::ShotIn`, `SHOT_ALARM`, `squad::Frame::crater`, `squad::Frame::sink` (`Params::shot_sink`, the word after the tend row) | Larder thread (free shots; each hit lays the crater, PR 2; the blast sinks into the slime, `Rules::sink`) | soldiers; turrets (their hits lay it too, through the same frame) |
| Area effects | one list in `squad::Frame` (planned `bursts`: centre, radius, kill, alarm, crater, sleep, germ), CPU and GPU | squad toolkit thread | blast, mine, det cord (a line of bursts), sleep gas, germ; the Siren (a burst with only alarm); sound reads it |
| Scorch | one crater field (a per-cell countdown; trail fades fast while it runs; `scorch_keep` 0.1, `scorch_full` 30 since PR 2), CPU and GPU (binding 47, one body took 46; `CRATER_AT` 864, laid by the `cut` pass); each writer gives its own strength and radius | squad toolkit thread | shots write it (Larder thread, PR 2, `Rules::crater` 10 s); blasts and det cord next; graphics draws it from the field; no Scorcher building (owner's card, 11-12 04:46Z) |
| Buildings | one `Building` type (kind, position, health, build progress) and the still-unit slots | buildings thread | turret, lure, pump, wall, depot, siren |
| Walls | Larder's wall cells and rules (`wall_hp`, `wall_scent`, `wall_bite`); a wall is a building kind over them | Larder thread (rules), buildings thread (the kind) | |
| Building anything | soldiers walk to a site and raise it over time (owner's card, 11-12 ~04:41Z) | soldier controls thread | every building kind |
| Larder and outpost | one kind, a depot; the larder is the first. Hoard smell and stealing apply to every depot | buildings thread (kind), controls thread (select, buy, rally), Larder thread (hoard, steal) | |
| The hole under fire | the shot rule: each hit lays the toolkit's crater on its strike disc (`Rules::crater` 10 game s, `squad::Frame::crater`), CPU and GPU, PR 2. The alarm read at -1 (`FLEE_WEIGHT`) is an opt-in: as a default it let random hold the core and win | Larder thread (the one body thread handed it over, 11-12 05:08Z) | graphics draws it |
| Cohesion, hunger, cuts | `big.rs` on the CPU, then a GPU port | one body thread | det cord's siege relies on a cut piece starving; graphics draws cuts |
| Shot look | drawn from the crater field and the walkers, no stamp (owner, 11-12 04:38Z) | graphics thread | sound |
| Sound | reads `squad::FrameOut`, `HudEvent`s and the burst list; writes no rule | sound thread | |
| The player's own mould | the gardeners shape (owner's card, 11-12 04:45Z): a second `Side` in its own file with a thin hook in `larder_world.rs`; its cut, road and feed reuse the burst list, the squad frame's per-unit writes and the harvest path | player's own mould thread | |
| Missions | data over the shared pieces (map, tier, `Rules`, tools allowed); a new end condition is the Larder thread's (`End`, `Game::score`) | campaign and onboarding thread | |
| GPU frame time and read-backs | one mapped readout a step in `gpu/mod.rs`, which also carries `World::mould_grid` on `GpuWorld`; a change to `squad::FrameOut`'s shape is announced before it merges, since three threads read it | GPU thread | campaign positional goals, player AI |

**Overnight questions (owner, 11-12 04:48Z).** "I cannot answer any not
questions tonight. When there is a question have the threads work together
to come up with the best answer and move forward with it." For the Larder
threads the integration thread settles a question with the game designer
and technical lead reviews, records the answer and its reasons here, and
the owner can reverse it in the morning.

**The campaign's carry-over is the owner's.** He approved the campaign
plan and "carry unlocked verbs only" (card, 11-12 04:44Z). The details
settled after it by the game designer review with the campaign thread
(a mission ends at the 15:00 cap with a graded result in stars; the first
mission teaches with first-time hints; the persistent world is a separate
later mode) follow that ruling; the owner may change them.

**Build first, balance later (owner's card, 11-12 17:26Z).** The gates in
this file that read "every plan beats random after Holm" are no longer entry
gates for a feature, a look or a controls fix: those land on tests plus a GIF
or a playable build, and a census clash becomes a tuning note to the owner of
the numbers. The two-block census below still gates a change to a shared
default every thread measures against, and AI-strategy work. No thread holds
its work for balance in another area
([rule](Reports/testing-practice-2026-11-12.md)).

**Build first, balance later** (owner, 11-12 17:26Z; CLAUDE.md, #150). A
census does not gate features, looks or controls, and balance never blocks
work: a clash becomes a tuning note to whoever owns the numbers. A census is
for strategy questions only (player AI, shared defaults, dominance before a
milestone or playtest, mission difficulty, an owner question). Name the
runner time first, and get the owner's OK for anything over about an hour.
The rules below apply only when a census is warranted. The census-reset
re-runs (#122, density) re-run only the baselines a strategy question still
needs.

**One census, two seed blocks** (11-12 ~10:10Z, with the technical lead).
A game-level table compared across threads comes from `examples/larderworld
census`, with hands, tiers, tools and missions added there as `hands=` or
flags; it prints a `provenance:` line (commit, seeds, cap, a hash of every
setting) to quote beside the table. Tables from another harness (the
toolkit's `kitplans`) say so and are not compared with it. The standing
gate plays two disjoint seed blocks, 5-36 and 201-232, and reports random's
wipe count beside the score: random wiped the plain mould on 5 of 16 of
seeds 201-216 against 2 of 32 of seeds 5-36 (Fisher p 0.033), so a gate
passed on one block can fail on the other. The window's own loop is in CI
since #116 (`the_window_plays_every_mission`,
`the_window_takes_every_command`); every thread keeps both green.
Every census also reports **win rate and the stalemate median separately**,
not only score v2's mean, which mixes wins and stalemates (one-body review,
11-12 17:10Z): farm, wall and fort hit the 15-min cap in 144 of 144 games of
the 48-seed core census. Until 11-12 the GPU dropped one-walker-a-cell,
without a word, when `capacity` passed half the agar; since the GPU
thread's fix it holds while the start walkers are at most half the agar,
as the CPU does, and warns when it drops (`gpu::crowd_holds`).

**Caps.** Buildings share `squad::UNITS_MAX` (64) with up to
`Rules::max_squad` (40) soldiers, and the frame drops units past the cap
silently. The buildings thread raises it to 96 (two workgroups for the
units pass) with a test at the cap in its first PR. A turret-ring hand joins the buildings census gate, since free
shots plus turrets could bring back the held ring of fire that once beat
every plan (game designer review).

**Green means green with current main** (integration thread, 11-13
00:05Z, at the technical lead's request). #165 (own mould, new `Args::own`)
and #174 (a test building `Args`) were each green on their own heads and
merged minutes apart, and main failed to build (E0063) until #181 carried
`own: false`. Before merging, a thread checks that its head contains
current main; if main moved since its last CI run, it merges main in and
waits for green again. That costs a CI cycle only when main moved, and the
sorted wiki frontmatter (#167) keeps those merges clean. GitHub's "require
branches to be up to date" setting on main would enforce it; that is the
owner's setting to turn on, if they want it.

**Merge order on `src/mould/larder_world.rs` and `squad.rs`.**

1. Larder thread: free shots, soldier scent (PR #106)
2. Controls part 1: order queue, attack-move, moving on (branch built 11-12)
3. GPU thread: `Game::step` split into begin and finish halves (`step()`
   stays both, so callers, the census and replays run as now)
4. Buildings batch 1: the `Building` type, turret, lure, pump, wall as a kind
5. Squad toolkit: the burst list, the crater field, the tools
6. Controls part 2: building over time, the larder as a depot
7. Buildings batch 2: siren (a burst), outpost (a depot)
8. The player's own mould as a second side (gardeners)

**Counts scale with density, not raw walkers.** The window and the census
play 5,000 walkers today; the GPU was built for 100,000. A threshold
written as a walker count (for example a look's walkers in a 5 x 5 window,
or a sound's kills per shot) breaks at the larger scale. The game publishes
one density, walkers per open cell at the start (Larder thread, on
`Game`), and a new count-based threshold is written as a function of it,
measured at two scales, not assumed in proportion (technical lead review,
11-12). The sound thread found kills a shot grow about 1.9 times for 10
times the walkers (35 at 5,000, 66 at 50,000, one rush), so its heavy-shot
threshold scales with density to the power 0.3.

**No slower mould clock.** The Larder thread dropped it (11-12 04:49Z):
the graphics thread timed a 3x slower clock and the hole still refilled in
0.53 to 0.73 s, so it is not the lever, and census baselines stay put. A
clock change would be a census reset: if one comes back, it re-runs every
baseline in its own PR, between other threads' censuses.

**Census reset: #122 (a8669bd, 11-12 11:00Z).** Soldiers now raise
buildings over time (`BuildRules::raise` 8 s; it was 0, buildings whole
when placed), the census hands send a builder, and carriers bank at the
nearest depot. Every census row from before a8669bd predates this and is
not comparable with rows after it. The buildings thread's first look at
seeds 5-12: turtle lower 7/8, guard and pumps 7/8, wall, ring and killzone
even, farm and army unchanged. The full gate (seeds 5-36 and 201-232, two
maps) is being re-run on the new baseline.

**Baseline breaks, 11-12 evening: #164, #169, #170 and #174.** Each of these
changes what the hands can do or what ends a round, so census rows from
either side of one don't compare. **#164** (cdccf127) makes every mould
headless (no king rule). **#169** (a412af4) lets a soldier build
anywhere, so the AI and census hands build differently (technical lead,
22:42Z). **#170** (670ca9b, 22:58Z) is the economy (rations, hunger, overrun
20 s, Regrow removed). **#174** (e425fed, 23:05Z) doubles a walker's bite on a
soldier at 5,000 walkers (`Rules::drain` 2 at any count, buildings keep 1 x
mass), adds the broken mould (`Rules::broken`) and makes the window's
mould shape C (25,000 walkers; the census harness stays at 5,000). No
rerun is due for any of them. Under *build first,
balance later* the next scheduled balance pass starts from e425fed or
later, and older rows stay as history. It includes a wall-fort hand: since
#198 a unit aims only at slime it can see, and a squad standing by a wall
killed 486 walkers in 26 s against 261, so walls plus guns may bring back
the fort plan that once dominated (design review, 11-13).

**Census reset held: mould density (11-12 16:20Z; the owner took the
dense mould for play, 22:19Z).** The window plays shape C, 25,000 walkers
(route A), since #174; the census harness's reset below still waits for
its slot. One change from the plan: a walker's bite on a soldier no
longer counts mass (#174, the owner: walking in "should be very deadly";
at 0.2 a walker 40 soldiers in C's thickest part lost none).
The one-body thread found that the owner's "specks" come from density.
At today's 3-5% of open cells holding a walker the mould draws as beads,
and at 15-20% it draws as smooth tubes. If the owner takes the dense mould,
it lands as **one** PR in a slot the integration thread schedules between
censuses, and every thread re-runs its baseline once afterwards. The rules
then count mould mass, not walkers (agreed with the Larder thread,
16:20Z): `Rules::reference_density` (0.0, off: every walker weighs 1) and
`Game::walker_mass()` = reference_density / density, with `Game::mass(n)`.
Drain and steal (larder_world.rs and buildings.rs) and `Rules::wade`
multiply by it. (As built: `Rules::mass` since #164; the bite on a
soldier counts whole walkers since #174, and wading since 2026-11-13,
`Reports/larder-wading-2026-11-13.md`; every reader is marked whole, by
mass or presence since the 11-13 audit, listed on the wiki's Larder page.) The score's size term (a ratio), the touching tests and the
calm subtraction stay as they are. `killed` stays a stat. The reset sets the
constant in the same commit as the new baseline, because a Fork-at-5k
constant is not 1.0 on other maps or at the GPU's 100k. Sound's touch curve
belongs to the sound thread. Route A (5x the walkers) or route
B (a coarser grid) is the owner's choice, with the GPU thread's frame cost
for both on the card.

**Shot impact is a rule, through the crater field.** The owner turned down
both drawn versions in the graphics thread's blind A/B (11-12 04:38Z:
"Neither. Both of those look like stamps. Placed on the slime that fade.
Maybe part of the issue is the slime/walkers move to fast.") and set the
bar at 04:39Z: "A group of soldiers shooting the slime should start carving
out a bigger and bigger hole that doesn't refill while being shot at." So
the hole is a rule, and its measure is the hole's area under sustained
fire, not its look. The Larder thread's candidate needs no new field: the
mould reads the alarm that shots already lay at a negative weight
(`alarm_weight` -1, keep 0.98). On seed 1 that held a growing hole open
under fire (110 dish cells, against 40 today) that refills once fire
stops. It becomes a default only if a 16-seed hole test and the census
agree. The 16-seed test agreed (11-12 05:10Z: 102 dish cells against 29
after 20 s of fire, 16 of 16, refilled about 10 s after the last shot), but
the census did not (06:20Z): random wiped the mould on 12 of 16 seeds by
holding its core, since the mould no longer overran soldiers there, and the
first touch on a squad came later. So it is an opt-in, and the next try is
shots that lay the toolkit's crater, with the gate's random and idle rows
run beside the hole. Any rule that keeps the mould off soldiers must keep it
defending its core (game designer review, 06:20Z), for every tier. That is the local push the *Skittish* dead end saw near a wound, not
the whole-body flight it was rejected for. Shots may still lay a little of
the toolkit's crater later, with values in `Rules`.

Sound and the graphics thread's look changes add no rule and merge
whenever ready. The one body thread's CPU rules merged (#103); its GPU port went first
(agreed with the integration thread 11-12 06:06Z). The order is a default, not a lock: a thread that
is ready before an earlier one has a PR open tells the integration thread
and goes. The second to merge merges `main` in and resolves its own side,
never another thread's.

**GPU engine.** New `Params` rows go after the soil row (848 bytes
since #92) in merge order: the one body row (`BODY_AT` 848, so 864 bytes), then the
toolkit's crater and burst rows. The one body port holds binding 46 (energy
sums and counts in one buffer); the crater (PR 2) holds binding 47 (two
blocks: the crater countdown, and a second block kept for the tiers' scar)
and the row at `CRATER_AT` 864; the one body thread's lone step holds
`LONE_AT` 880 (so 896 bytes; no binding, it reads `occ`), and the tiers'
scar row goes after it. Binding 48 is food's news (`wither`, #195, the
slime-enemy thread, 11-13: a word a walker, then three blocks of cells);
the next free binding is 49. New
passes append after the last; whoever merges second renumbers. The GPU
thread's readout buffer and living-walker pass add no `Params` row. The layout asserts in
`Engine::write_params` run in release builds, so a slipped row fails every
GPU test instead of moving every offset after it.

**Cloud plan picks (11-14 00:35Z; [plan](Reports/cloud-plan-2026-11-14.md)).**
Settled by the technical lead, the game designer and the integration thread
(tie-breaker) under the owner's 11-12 04:48Z rule:

- **D1, census on shape C:** yes, one PR (integration). The census harness
  takes C's rules (density, mass, shared food, wither, panic, drain) on a
  census-sized map at the same walkers per open cell, so runner time stays
  about what it is now; `shape=` and `walkers=` keep the full size. The PR
  checks once, on 4 seeds, that census-sized C and window-scale C agree in
  direction (who wins, rough mould size at 5:00), and marks a baseline
  break. Threads re-baseline at their next census; no forced rerun.
- **D2, `flow_loss`:** parked, not deleted (off on both engines). Withering
  is the owner's pick for a cut-off piece; `flow_loss` cost about 30% of
  C's size and the piece still died at once.
- **D3, counter table for C4** (designer; a miss is a tuning note): C: cut
  beats strike. Drifter: strike beats boom alone. Sprawler: fort beats
  strike. Trunk: cut or claim beats a strike on bodies. Sporer: claiming
  untaken crystals first beats a strike near food. Sleeper: bait then
  strike beats a slow claim. Swarmer: fort or kill zone beats repeated open
  strikes. Grudge: a spread boom or claim beats fort. Pass: no plan best on
  more than half the tiers, every plan best somewhere, idle loses
  everywhere, each counter beats random on at least 6 of 8 paired seeds.
- **D4, mission 8:** becomes "Your own mould" (`own=1`) against the
  Grudge on the Passes, with the lure verb and a take-share goal (the
  campaign thread's proposal), built from what exists so missions 1-8 ship
  on main. The rival pair moves to chapter 2: it needs its own design pass
  (do the species fight each other?) and the GPU's per-species birth
  counter (the player-mould thread's second PR) first, because three moulds
  share one GPU birth pool today.
- **D5, the start menu (C2):** menu, mission list, options with today's
  defaults, the key card; plus restart and mission select from pause, a key
  card showing only the verbs the mission has unlocked, and an "applies to
  this build" line. Saves later.
- **D6, the file split (C9):** first, before C2 and C10 branch. Prepared on
  the integration branch, then a freeze of under 45 minutes on
  `larder_world.rs` and `larder-world.rs` (freeze held from 11-14 00:36Z).
  Moves only, one module per subsystem, no public renames; the test count
  and the replay hash stay identical and the commit quotes both. Done 11-14:
  356 tests the same, census rows and the 14 larderai replay logs
  identical; `src/bin/mouldgpu.rs.orig` deleted.

**Spare `Params` words (11-13 04:30Z, technical lead).** `PARAMS_BYTES` is
still 896: #183 (`fed_full`, `fed_crowd`) and #193 (`flow_loss` as `eloss`)
spent padding words instead of growing the struct. The words still spare in
`struct Params` (`src/gpu/engine.wgsl`), by byte offset: **188** `_p0`
(u32), **228/232/236** `_p1`-`_p3` (f32), **460** `_k0` (u32),
**484/488/492** `_k1`-`_k3` (f32), **828** `_l5` (f32): nine in all. Each `Set` (the four species
blocks at 496) also has `_p` at +44, a per-species word. To use one, a
thread claims it here (offset, name, thread, PR) in its PR, and renames it
in the WGSL and in `Engine::write_params` in the same commit. A word sits
inside an older row, so the row's comment should say who borrowed it.
When the list runs out, **one** planned PR grows `Params` by a whole
16-byte row (896-911) and updates the
length asserts, booked through this thread so two PRs don't grow it in the
same week.

Claimed: #183, #193; then 360 `wither` (f32), 364 `s_wither` and 732
`wclock` (withering's lag, check interval and spread parity), the
slime-enemy thread's withering PR #195 (11-13); then 460 `panic_r2` (u32)
and 484 `panic` (f32), `wclock` bits 25-31 (the lean) and `Pile`'s spare
word (`centre`), the slime-enemy thread's panic PR (11-13); then 488
`panic_creep` (f32, the lean's pace), the slime-enemy thread's creep PR
(11-13); then 492 `panic_dash` (u32, the charge's dash), the slime-enemy
thread's dash PR (11-13); then each `Set`'s +44 `_p` as `cap` (u32, the species' birth cap, garden only), the player's own mould thread's cap PR (11-14). Still spare: **188**, **228/232/236**, **828**
(five).

## Keeping this page true

- A thread that passes a gate, opens or merges a PR for a step, or changes
  a step's owner updates its row here in the same PR.
- An owner answer moves into [`wiki/decisions.md`](wiki/decisions.md) as
  usual; then update the row it unblocks here.
- The GPU go-ahead (17:50Z) and the owner's GPU (GTX 1070 on Scott-Desktop,
  17:54Z) are recorded in `decisions.md`.
- Each thread merges its own PRs once both CI jobs are green on the exact
  head, with a merge commit (owner, 17:47Z). Any new report must be cited
  from a wiki page in the same PR; `python3 wiki/lint.py` fails otherwise.

## Archive: the paused bands (G, E, R)

Moved here 2026-11-12. The owner stepped back from today's dishes and their
tools (11-10 20:55Z); a thread that works on one of these directly may
still change it (11-11 15:17Z). Rows are as they stood.

| # | Step | Owner | Waits on | State |
| --- | --- | --- | --- | --- |
| S1 | GPU phase 1: frame profiler (also the review's E2), "GPU semantics" CPU arm, delayed-census arm, new rules off by default | GPU thread | E1 for the hash; owner go-ahead given 17:50Z | profiler done (PR #16: diffusion is 62% of the simulation, moving walkers 24%); arms measured on every dish: late census safe, fixed-point fields change play (fields stay `f32`), all-at-once move cuts seam fights (The Wild One's warden loses, p 0.0005); owner 20:55Z: today's dishes are not the true games, so gates use engine measures; scaling rule chosen 11-10 on 16 paired seeds (`mould::big`, `examples/bigdish.rs`): today's numbers in fine cells plus food smelled at today's reach; more steps a frame rejected. **Done** |
| E1 | Dish hash and determinism test | Pixel_Physics thread | nothing | done: PR #14 (`Dish::state_hash`, `every_dish_replays_bit_for_bit`, `mouldprobe` prints `hash=`) |
| E4 | Release profile re-measured on `mouldprobe` and one CI run | Pixel_Physics thread | E1 | done: `codegen-units = 1` kept (about 0.45 ms a frame), thin LTO dropped (no gain); hashes unchanged |
| E5 | Licence and notices gates, the JetBrains Mono OFL notice, `crispfont.py` | Pixel_Physics thread | nothing; must land before any build leaves the owner's machine | done: `scripts/licensecheck.sh`, `scripts/notices.sh` + `THIRD-PARTY-NOTICES.txt`, `notices.yml`, `scripts/crispfont.py` |
| R- | Bloomfront research (six brainstorm agents, then a red team, then a field retest and verb census) | RTS thread | nothing; design only | **done 11-10**: [red team](Reports/rts-design-2026-11-10/red-team/README.md), [field retest](Reports/rts-design-2026-11-10/field/README.md), [page](wiki/rts/directions.md). On The Wild One Taut fails and the counters are a ladder; on a 23-pile field Taut's pull wins and the counters cycle on one layout of two. Census: field salt passes on share; Wild One lamp moves share but not by 0.02. Later: the cycle holds on 1 layout of 10; salt overturns a 60 s lead on scatter. Now two demos (R-S, R-F) |
| R-S | Field skirmish playable demo (`src/mould/skirmish.rs`, `web/`) | RTS thread | nothing | v1 done and set aside by the owner (21:14Z); **v2, the budget skirmish, in the harness 11-10**: [report](Reports/rts-skirmish-2026-11-10.md), [page](wiki/rts/skirmish.md). Soft beats all 8 other plans 127/128, no cycle. **v2 page live 11-10**; round 3's counterplay rules (rally, rising spore price) did not stop soft (124/144). **Rounds 4-6 done 11-11**: side edge fixed; model changes (holder's bonus, supply lines, size upkeep, desperate fights) do not dethrone soft and keep; desperate fights ease the snowball. **Rounds 7-7b done 11-11** (economy sketch tests 1, 3 and half of 4): food after 1:00 and a held-ground score leave the 1:00 leader winning 80-85%; regrowth cuts wipe-outs; a counted-down Strike with a Fortify that kills attackers dethrones soft, but brace (strike and fortify) wins every pairing; page opens on Strike and Fortify. Next: Fortify's price and the exploiter search against brace |
| R-F | Two Fevers playable demo (`src/mould/fever.rs`, `fevers-web/`) | Two Fevers thread | nothing | **v3 done 11-10**: [report](Reports/two-fevers-2026-11-10/README.md), [page](wiki/rts/two-fevers.md), https://claude.ai/artifact/UMzPJmevoYJYmx8KRqG37a. Calls, fever by tube, foraging, pinch. Plans form a ladder (trunk > web > raider), not a cycle; the leader keeps the lead; waits on the owner's hand |

| # | Step | Owner | Waits on |
| --- | --- | --- | --- |
| P | **Play Keep It Alive and The Wild One by hand** | Scotttt | nothing. This is the one step that unblocks most of the rest |
| G1 | Keep It Alive: trace single games to find what sends one into the short or long hump. Measures only; the game does not change | Triage thread | **done 11-10**; owner picked "Trace it now" (card, 18:56Z): the first pile on mostly-asleep mould decides it ([report](Reports/mould-keep-alive-luck-2026-11-10.md)). Next: a 20 s landing preview, a game change |
| G2 + R0 | **The Wild One's middle (seconds 30 to 150), done as Bloomfront Stage 0.** First separate which of lamp, snip and oats needs the aim; then give the wild mould a scripted hand on a mirrored dish | one thread, to be chosen | P for G2; the owner's say for R0 (HANDOFF decision 7, "wait or replace") |
| G3 | The snip: make a cut-off piece visibly starve, or take it off The Wild One | Triage thread | P |
| E6 | Keyed random draws and the dead-dish slowdown | Pixel_Physics thread | timing only: see *One census reset*, below; the keyed draw itself is built inside S1 (`keyed`, used only with `Params::all_at_once`, off by default) |
| E7 | Sound for the game, from Pixel_Physics's `src/audio/` | Pixel_Physics thread | **built 11-10** (owner: "yes to sound", 19:22Z; [report](Reports/mould-sound-2026-11-10.md)). Next: the owner's ears on the Mould Sound page. **Larder World sound built 11-12** ([report](Reports/larder-sound-2026-11-12.md), [page](https://claude.ai/artifact/P4hk1fsSXQj3qXuwieffa7)): shots weigh the walkers they killed, a pulse as the mould nears; O toggles; levels await the owner's ears |
| G4 | Star bars re-checked on 16 or more seeds | Triage thread | G1-G3; and S5 if the GPU goes ahead, because S5 re-derives the bars anyway |
| R1-R2 | Bloomfront Stage 1 (spend to act; needs a currency first, since the food store sits at a median of 0, [brainstorm](Reports/rts-design-2026-11-10/brainstorm/README.md)) and Stage 2 (grow and sever) | RTS thread | R0's gate (equal hands 45-55%, sharp beats clumsy on 16 paired seeds); P (Stage 1's gate needs the owner's play); G3 (Stage 2's sever is the snip that bites) |
| R3-R4 | Bloomfront Stages 3 and 4 | RTS thread | the scale fork; Stage 4's lockstep needs the integer simulation (HANDOFF decision 6) |
| G6 | Later depth, in the owner's order: melting salt, spores, a rival the player steers, same-kind moulds joining | unowned | G1-G4 playing well |

### Dependencies (G, E, R and S)

```mermaid
flowchart LR
  P([Owner plays both dishes]):::owner
  E1[E1 dish hash] --> S1[S1 GPU phase 1]
  E1 --> E4[E4 release profile]
  S1 --> S2[S2 GPU phase 2]
  D([Owner at Scott-Desktop]):::owner --> S2
  S2 --> F{{Scale gate: owner}}:::owner
  F -- go --> S3[S3 sandbox] --> S4[S4 rules on GPU] --> S5[S5 port dishes] --> S6[S6 new play]
  J([Jones 2011 upload]):::owner --> S4
  F -- stop --> E3[E3 multi-core draw]
  S3 --> G5[G5 forest floor]
  F -- stop --> G5
  K([Card: trace now?]):::owner --> G1[G1 Keep It Alive luck]
  P --> G2[G2 + R0 Wild One middle / Stage 0]
  W([Bloomfront: wait or replace?]):::owner --> G2
  P --> G3[G3 snip that bites]
  G1 --> G4[G4 star bars]
  G2 --> G4
  G3 --> G4
  S5 -.-> G4
  G2 --> R1[R1 spend to act]
  P --> R1
  R1 --> R2[R2 grow and sever]
  G3 --> R2
  R2 --> R3[R3 three or four keepers]
  F --> R3
  S4 -.-> R3
  R3 --> R4[R4 species, online]
  I([Integer or float]):::owner --> R4
  S1 --> I
  classDef owner fill:#fde68a,stroke:#b45309,color:#000
```

Yellow is the owner. A dotted arrow is "if the GPU goes ahead".

### Where the plans touch each other

These are the places two threads could step on each other or redo work.
Each has a rule.

- **One census reset, not three.** Three changes would each make every old
  census number stop reproducing: the keyed random draws (E6), the
  all-integer simulation (S1's decision), and the GPU path itself (S5). They
  are the same idea: a random draw keyed on seed, walker and step is exactly
  what the GPU plan's integer design uses. So build E6 as part of S1, in the
  form the GPU plan needs, and land it in one reset together with the
  integer switch if the owner agrees to it. Land it between reports, never
  while G1 or G2 are mid-measurement, and re-run every dish's baseline once
  in the same PR.
- **Parallel draw is on hold.** The Pixel_Physics review puts the
  multi-core renderer third; the GPU plan holds it until the scale gate,
  because GPU drawing would replace it. Hold it (HANDOFF decision 6); it
  becomes the fallback if the owner stops at the gate.
- **The profiler is built once.** E2 and S1's profiler are the same tool;
  the GPU thread builds it.
- **The Wild One has one owner at a time.** G2, G3 and Bloomfront Stage 0
  all change The Wild One. The RTS report's own advice is that Stage 0
  answers G2's question, so do them as one piece of work in one thread.
  Which thread is the owner's call with decision 7.
- **`src/mould/mod.rs` is busy.** E1, S1 and E6 all edit it. Land E1 first;
  S1 builds on it.
- **The snip matters twice.** G3 is a game fix and Bloomfront's Stage 2
  depends on it (supply raids). Its report should say whether a sever now
  starves the far side, measured both ways.
- **Determinism is shared.** E1's dish hash is S1's first gate and
  Bloomfront Stage 4's lockstep check. All three use the same hash.
- **The forest floor needs two engine fixes** whichever branch is taken:
  stepping only near the mould (one open 256 x 160 floor costs about 1 ms a
  step), and `spread` still skipping the border row
  ([`Reports/mould-real-world-2026-11-09.md`](Reports/mould-real-world-2026-11-09.md), §5;
  hand-off triage). No thread owns it yet.

### What waits on the owner

In the order that unblocks the most. The numbers are HANDOFF's open
decisions.

1. **Play Keep It Alive and The Wild One.** Blocks G2, G3, G4, Bloomfront
   Stage 1's gate, and most of decision 1.
2. ~~Answer the trace card~~ (decision 1): answered 18:56Z, "Trace it now";
   done. What follows from it, a 20 s landing preview, changes the game and
   waits on the owner's go.
3. **Bloomfront's three questions** (decision 7): wait for the queued work or
   take over The Wild One's next steps; fog of war drawn from a held
   quantity; two keepers or three-to-four. Recommended: do G2 as Stage 0.
   *Since the red team (11-10):* Stage 0's gates would pass without testing
   the opening, so the RTS thread recommends a paired-clone verb census
   first; G2 can still use Stage 0's two-keeper plumbing.
4. **Hold the multi-core renderer** until the scale gate (part of decision
   6). Recommended: yes. Needed now, because it is third in the Pixel_Physics
   thread's order.
5. **Listen to the Mould Sound page** (https://claude.ai/artifact/6GXBj1swj93DzdytbUjqvA) and say whether the levels are right.
6. ~~**Be at Scott-Desktop with the Claude app open** when S2's timing test is
   ready.~~ Done 11-11; the owner gave a standing OK (00:06Z) for tests on
   that PC whenever it is reachable.
7. **Integer or float** (decision 6), after S1 has measured it. Recommended:
   integer.
8. **Upload Jones 2011** before S4.
9. **The scale gate** after S2.
10. Low urgency, best answered after playing: retire F7 and grains or
    glints (decisions 2 and 3), food flow (4), the Organ and the Wanderer (5).
