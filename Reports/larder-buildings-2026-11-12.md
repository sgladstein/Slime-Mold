# Larder world buildings (2026-11-12)

The owner, 2026-11-12 04:23Z: "Start a thread that works in turrets, walls,
and other building". Proposed at 04:30Z: turret, lure, pump, then scorcher,
siren and outpost. The owner chose "All six in one go" (04:45Z), then
dropped the scorcher (04:46Z, on the integration thread's card: "Drop it,
scorch is the crater"). The siren waits on the squad toolkit's alarm-only
burst (`Frame.bursts`, CPU and GPU), which that thread is writing.

Code: `mould::larder_world::buildings` (`Building`, `Kind`, `BuildRules`,
`Game::build`, `Game::raise`, `Game::toggle`, `Game::drop_offs`); the
window's build page (Q) in `src/bin/larder-world.rs`; hands `ring`,
`turtle`, `killzone`, `pumps`, `guard` and `tier=` in
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

PAYOFF_TABLE

FINDINGS
