# Larder world buildings (2026-11-12)

The owner, 2026-11-12 04:23Z: "Start a thread that works in turrets, walls,
and other building". Proposed at 04:30Z: turret, lure, pump, then scorcher,
siren and outpost. The owner chose "All six in one go" (04:45Z), then
dropped the scorcher (04:46Z, on the integration thread's card: "Drop it,
scorch is the crater"). The siren waits on the squad toolkit's alarm-only
burst (`Frame.bursts`, CPU and GPU), which that thread is writing.

Picture page: [Larder World Buildings](https://claude.ai/artifact/Xutp7S9tyPoz4ghxhaCSrj) (`larder-buildings-2026-11-12/page.html`).

Code: `mould::larder_world::buildings` (`Building`, `Kind`, `BuildRules`,
`Game::build`, `Game::raise`, `Game::toggle`, `Game::drop_offs`); the
window's build page (Q) in `src/bin/larder-world.rs`; hands `ring`,
`turtle`, `killzone`, `pumps`, `guard`, `army` and `tier=` in
`examples/larderworld.rs`; the lure test `examples/worldbuild.rs`.

## What was built

| Kind | What it does, through the model | Cost, health |
|---|---|---|
| Turret | A unit that does not move: sees `turret_range`, fires a squad shot at its densest block every `turret_every` (paid per shot at `Rules::price`, 0 now); its hits lay the Swarmer's alarm | 500, 300 |
| Lure | A unit's smell: `lure_smell` x a soldier's scent over `lure_r`, burning `lure_burn` food a second while on (`Cmd::Toggle`) | 300, 150 |
| Pump | A unit's take on its crystal: `pump_rate` food a second into the bank | 500, 200 |
| Outpost | A depot: smells of the bank at `outpost_hoard` of the larder, and walkers touching it steal from the bank; `Game::drop_offs` lists it for the controls thread's carriers | 400, 300 |
| Wall | The old wall as a kind (`Kind::Wall { to }`); its cells go into the world when it is whole | by length |

Every building is placed as a ghost (`built` 0) with a tenth of its health
and gains the rest as it rises (`Game::raise`, the soldier controls
thread's build order); with `BuildRules::raise` 0, today, it is whole when
placed. The mould wears any building it touches, ghosts too, at
`Rules::drain` a walker a second, as it wears soldiers.

**Why still units.** The squad frame (`squad::Frame`) is already one path
on the CPU and the GPU, tested to the bit (W2a). A turret, a lure, a pump
and an outpost are each a unit of it, so the buildings need no new engine
pass or binding. The frame held 64 units, which 40 soldiers, the larder and
20 buildings nearly filled, and dropped units past it silently, so
`squad::UNITS_MAX` is now 96: the WGSL arrays grow, and the units pass runs
two workgroups of 64 (one before, so units 64-95 would have been read by
nobody).

## Twin tests (software Vulkan, llvmpipe)

- `buildings_play_on_the_engine_as_on_the_cpu` (mouldgpu): a pump on the
  larder's crystal, a lure and a turret by the core, 20 s at scale 2: pumped
  100.000175 food on both, lures burnt 40.00008 on both, turret shots 38 on
  both, kills 293 against 272 (the moulds drift within the race world's 8%).
- `the_squad_frame_holds_its_cap_on_the_gpu`: 96 units, every output equal
  to the CPU's; of units 64-95, 10 touch, 32 aim and 10 harvest.
- CPU: ten tests in `buildings.rs` (placement rules, ghosts, a turret's
  kills and wear, a lure's burn and scent, a pump's take into the bank and
  the score, outposts as drop-offs, a full squad and every building within
  the frame, replays).

## Does a lure pull the mould?

`worldbuild seeds=1-8` (race world, scale 2, 5,000 walkers): a lure 0.35 of
the way from the core to the larder, with one soldier on Hold fire on it in
both arms, on against switched off. **Named before the run:** walkers
within 8 dish cells at 60 s.

| Lure | at 20 s, more on / fewer / tied | at 60 s (named), more / fewer, p |
|---|---|---|
| 2x a soldier over 3 cells (first) | 7 / 0 / 1, by a few walkers (1-42 against 0-34) | 6 / 2, 0.29 |
| 8x over 6 cells | 8 / 0 / 0, p 0.008 (26-93 against 7-34) | 7 / 1, 0.07 |
| 32x over 6 cells | 8 / 0 / 0 | 6 / 2, 0.29 |

The named measure missed what happens: a strong lure fills with mould
within 20 s, and then the mould wears it down (it died within the minute on
3 of 8 seeds at 8x), so by 60 s the crowd has gone. A lure is bait the
mould takes, not a standing beacon. The default is now 8x over 6 cells, the
weakest setting that pulls a crowd you can see. Raw: `lure*.txt`.

## Census

`larderworld census seeds=5-12 price=0` (free shots, the Larder thread's
coming default), score v2, paired by seed. Hands, besides idle, random and
farm: **ring** (turrets round the larder: the design review's risk that free
turrets bring back the held ring of fire), **turtle** (a pump and a guarding
turret on each farmed crystal), **killzone** (two turrets and a lure on the
road to the core). Most runs used the first lure (2x over 3 cells); three
were run by Haiku runners and some were stopped short by their 30-minute
limit (seeds as listed).

Mean score v2 by hand; **bold** the best mean; "best" is how many seeds
each hand topped (ties dropped). Sign tests are against farm, paired by
seed. Before main's moving-on rule (`Rules::seek`) and before random could
build; the gate after both is the next section.

| World, tier (seeds) | idle | random | farm | ring | turtle | killzone | best on a seed |
|---|---|---|---|---|---|---|---|
| race, plain (8) | -0.322 | -1.046 | +0.197 | +0.190 | **+0.325** | +0.210 | turtle 8; turtle beats farm 8-0, p 0.008 |
| race, swarmer (8) | -0.325 | -1.161 | +0.133 | +0.124 | **+0.303** | +0.130 | turtle 8; 8-0, p 0.008 |
| race, sporer (7) | -0.403 | -1.419 | +0.039 | +0.093 | **+0.793** | +0.087 | turtle 7; 7-0, p 0.016 (the mould wiped on 3) |
| race, sleeper (8) | -0.221 | -0.807 | +0.254 | +0.254 | **+0.296** | +0.256 | turtle 6, farm 2; 6-2, p 0.29 |
| nest, plain (8) | -0.228 | -1.462 | -0.232 | **+0.047** | +0.046 | -1.662 | ring 6, turtle 2; killzone's squad wiped 8 of 8 |
| fork, plain (3) | -1.278 | -1.454 | +0.353 | +0.353 | +0.355 | +0.353 | within 0.002 |
| fork, swarmer (5) | -1.328 | -0.823 | +0.352 | +0.354 | +0.351 | +0.354 | within 0.003 |

Every building hand beats idle and random on every world (8-0 where 8
seeds ran). Two hands split the turtle in half on the plain race: **guard**
(its turrets, no pumps) +0.341 and **pumps** (its pumps, no turrets) +0.213
to +0.280; the turrets carry it.

What a nerf does to the turtle (plain race, against +0.325):

| Change | turtle | guard |
|---|---|---|
| turret cost 500 to 900 | +0.329 | +0.335 |
| a turret shot every 1 s, not 0.5 s | +0.311 | +0.317 |
| pump rate 5 to 2.5 food a second | +0.316 | |
| pump cost 500 to 1000 | +0.301 | |
| pumps smell of food (`pump_smell` 8) | +0.377 | +0.326 |
| lure 8x over 6 (killzone, against +0.210) | | killzone +0.208 |

None moves it much. **The army control**: farm, then spend the bank on
soldiers up to the squad's most, all on free fire. Army +0.326, turtle
+0.325, guard +0.341; each beats farm 8-0 (p 0.008); best on a seed turtle
3, guard 2, army 3.

## Findings

- **Guns at the crystals win the race world, and a turret is just one
  way to buy a gun.** Turtle and guard top every race tier; army, the
  same spare food spent on soldiers, ties them. The building hands and
  farm all end with 20-35k food unspent, and farm stops buying at its 8
  soldiers. While food does not bind, any hand that turns it into guns
  wins, so the edge is the economy's, not the turret's. A spend-everything
  hand (army) is the fair comparison for the Larder thread's economy
  retune, which takes this up (integration thread, 06:57Z). The buildings
  ship at these defaults.
- **The open risk**: one family of plans, guns at the crystals, wins on the
  race world at every tier, which the "best plan changes" gate would fail
  if it were a buildings issue. The nest (ring best, killzone ruined) and
  the fork (no hand ahead) are the places the best plan does change.
- **A killzone on the road to the core is a trap on the nest**: its squad
  died on 8 of 8 seeds.
- **The sporer is the turtle's best tier** (+0.79, the mould wiped on 3 of
  7 seeds): turrets shoot its fruiting bodies before they spore.
- **Frame cost**: the buildings add units to the squad frame, which is one
  pass on both engines; no new pass. The window's model and HUD stayed at
  0.03-0.04 ms a frame with five buildings (look run, `look/run.txt`).

## The gate after main's moving-on rule, with random building too

The technical lead's condition before merging (11-12 06:57Z): random's
order pool now holds buildings (a turret, lure or outpost within a
soldier's reach wherever it wanders, the core too, or a pump on any
crystal, and lures switched now and then), and random and idle must still
lose. `larderworld census seeds=5-12 price=0
hands=idle,random,farm,turtle,guard,army`, with main's `Rules::seek` on
(`census-random-builds.txt`, 19 minutes on 4 threads):

| Hand | mean | ends |
|---|---|---|
| idle | -0.322 | cap 8 |
| random | -1.133 | squad wiped 5, cap 3 |
| farm | +0.201 | cap 8 |
| turtle | +0.325 | cap 8 |
| guard | +0.339 | cap 8 |
| army | +0.336 | cap 8 |

Every plan beats idle and random 8-0 (p 0.008 each; 8 comparisons, all
hold under Holm). Turtle and guard beat farm 8-0; army 7-1 (p 0.07). Best
on a seed: army 6, turtle 1, guard 1. Random does build: on seeds 5-7 it
placed 4, 8 and 3 buildings and lost 0-3 of them (`random-builds-look.txt`),
but it spends its food as it goes and its bank stays near 0, so it never
holds the crystals.

After main's #106 (free shots by default, soldier scent) the same gate
plays to the digit as above (`census-gate-after-106.txt`). The Larder
thread's #113 (shots dig craters, turret shots too, through the same
frame) will move it; the payoff table is re-run after it lands.

## The payoff on the new baseline (craters, 8 s builds, core defence)

After #113 (shots dig craters, turret shots too), #122 (buildings rise
over 8 s with builders) and #126 (the mould defends its core), main at
9f5e4a4, all nine hands, cap 900 s. Four runners and this thread ran the
standing gate's seeds; seeds 217-224 were lost to a container restart and
not re-run, since the owner's 17:26Z rule (build first, balance later)
makes the census a strategy question, not a gate. Summaries from
`gate.py` (raw files and provenance lines in `runs/`):

| World (seeds) | idle | random | farm | ring | turtle | killzone | guard | pumps | army | best on a seed |
|---|---|---|---|---|---|---|---|---|---|---|
| race 5-36 (32) | -0.396 | -1.431 | +0.274 | +0.284 | +0.318 | +0.267 | +0.319 | +0.324 | **+0.390** | army 22, guard 6, pumps 3, turtle 1 |
| race 201-232 (24) | -0.320 | -1.276 | +0.295 | +0.297 | +0.335 | +0.288 | +0.335 | +0.323 | **+0.393** | army 13, guard 7, pumps 2, turtle 1 |
| nest 5-12 (8) | -0.228 | -1.785 | -0.594 | **+0.045** | -0.111 | -1.629 | **+0.045** | +0.034 | -0.269 | ring 4, guard 3, army 1 |

- Every plan beats idle and random on both race blocks (Holm holds;
  random wipes the mould on 2 of 32 and 1 of 24).
- **On the race world, buying soldiers now beats building.** Army beats
  every building hand on 5-36 (guard 8-24, p 0.007; the rest 0-4 wins of
  32) and all but guard on 201-232 (guard 8-15, p 0.21). Build time and
  craters made turrets slower to pay back than soldiers.
- **On the nest the order turns over.** Ring and guard are best, army
  scores below idle (idle beats it on 2 of 8) and below ring and guard on
  7 of 8 (p 0.07 each, not significant at 8 seeds), and the killzone squad
  still dies. So the best plan does change with the map: soldiers in the
  open, turrets in the nest's narrow ground.
