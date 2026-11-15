# Pumps as the main economy (2026-11-15)

Owner, 2026-11-15 03:07Z: "I'm thinking we might need to move primarily to
the pumps instead of harvesting crystals. It's honestly just so easy the
beginning of every map. I just run to the biggest Crystal harvest the whole
thing and I have a full squad of 40 within the first 30 seconds. ... When
you go to place it, it should automatically center itself on the crystal or
it should probably just change the whole appearance of the crystal like
you've built some sort of structure around it. I think for the most part
the monitor that says your energy flow should always be positive. ... all
energy costs should just be an instantaneous removal. Or I guess it could
be proportional as you build it but it should just come out of the bank."
03:07:51Z: "Obviously if this is the case pumps do not start locked you
should be able to upgrade them. Pumps on bigger crystals should give you
more".

Page: https://claude.ai/artifact/6jYuCSBxzV1XsXNbTvYfJ4 (`page/index.html` here; frames from `larder-world map=fork mode=squads hand=pumps at=1,1.5,..,40 centre=262,108,9 screen=960x600` on llvmpipe, joined by ffmpeg at 4 fps).

## What changed

- **Hand draining cut to a fifth** with no larder (the squad mode and the
  run): `Rules::hand` 0.2 of `Rules::harvest`, so a soldier drains 6 food a
  second, not 30. With the larder on, carrying is its own brake and nothing
  changes. Guarding (`Rules::guard`, 1/120 of a crystal's charge left a
  second, not used up) is unchanged: the owner's ruling that guarding is
  worth more in the long run stands.
- **The pump draws by its crystal's size** (`Game::pump_draw`): 1/150 of the
  crystal's starting food a second (`BuildRules::pump_share`), at least
  `pump_rate` (5). On the Fork that is 5.9 food a second on the 885-food
  crystals, 12.5 on the 1,876 one and 51.8 on the two 7,768 ones; before it
  was 5 on every crystal. It draws over the whole crystal from its middle
  (`take_r` is the crystal's radius plus a cell; it was at most 4 cells
  from where it stood), and the mould wears it over the crystal
  (`touch` is the crystal's radius).
- **Upgrades** (`Game::upgrade`, `Cmd::Upgrade`, line `upgrade B`): two
  levels past the first, 400 then 800 food from the bank at once
  (`pump_up_cost`), each 1.5x the draw (`pump_up_rate`) and 1.25x the food
  each charge is worth (`pump_up_yield`): a level-3 pump on a 7,768 crystal
  banks 182 food a second and 1.56x the crystal in all. In the window,
  the Pump button on a crystal that has a pump upgrades it (the cursor's
  note gives the price).
- **The pump snaps to its crystal's middle** (`build_blocked`, `build`; the
  crystal is found as the harvest click finds it, `crystal_at`, so a click
  on the rim works). One pump a crystal. While placing, the ghost is a ring
  round the crystal, not a disc at the cursor (`Model::ghost_snap`).
- **The look: a rig built round the crystal** (`Marks::pump_rig`, building
  kind 6 in `view.wgsl`'s `pump_rig`): a steel hexagon just outside the
  crystal's own, a pylon and lamp at each corner and three struts to the
  pump head on the crystal's middle. Shapes at screen resolution, so crisp
  at any zoom. It is laid corner by corner clockwise as it is built (the
  rest a cream plan), darkens when hurt, and the struts' charge lines and
  lamps glow by the draw this frame (`Building::drew` of its full draw);
  each level adds an inner ring and a level mark (II, III). The crystal in
  it is the simulation's and dims as it is pumped. Before, the pump stood
  by the crystal with its hose turned in and a drawn line of charge.
- **The food readout's rate is income only** (`Game::income`, harvested +
  pumped + guarded, over the last 10 s), never negative. It was the bank's
  net change, so placing a building showed as a negative rate for 10 s and
  it stayed there after the building stood. Costs were already taken from
  the bank at once when placed; that is unchanged (a ghost called off still
  gives its whole price back).
- **The run**: pumps are never locked; the shop's "Pumps" unlock is now
  "Pump power" (`pump+`, 3 levels, a quarter more draw each).

## Checks

- New tests: `a_pump_snaps_to_its_crystals_middle_one_a_crystal`,
  `a_bigger_crystal_pumps_more`,
  `an_upgraded_pump_draws_more_and_costs_from_the_bank` (level 3 banks over
  twice level 1 in 5 s, the income is the pumping, the draw matches
  `pump_draw x pump_yield` within 5%), and
  `building_comes_out_of_the_bank_not_the_income` (a turret's price leaves
  the bank, the income does not move, and never falls over 10 s).
  `a_pump_draws_its_crystal_into_the_bank` now expects `pump_draw`.
- Full `cargo test --release --locked`: 544 passed, 0 failed (473 library, 32 window, 39 GPU twin). Clippy clean.
- Clip (`hand=pumps`, a new headless hand: the squad pumps the crystals
  nearest it, placing each by a click on its rim, raising it together, and
  upgrading it to level 3 before the next; no draining): Fork seed 1, squad
  mode, 0:01-0:40 at zoom 9 on llvmpipe. Income +16/s at 0:06 with one
  level-3 pump on an 885 crystal, +71/s at 0:40 with two. By 1:30 it had
  four pumps and 5,124 food in; the pump on the slimed crystal (165,150)
  was worn down three times and stood on the fourth try.
- Smoke, the owner's opening before and after (`hand=greedy`, new: every
  soldier drains the biggest crystal and a soldier is bought whenever the
  bank can pay; Fork seed 1, squad mode, one seed; before is `Rules::hand`
  1): soldiers at 0:15 / 0:30 / 1:00 / 1:30 were 12 / 27 / 33 / 40, now
  9 / 11 / 15 / 22; food in 1,838 / 7,611 / 9,979 / 15,777, now 320 / 1,104 /
  2,704 / 5,364. The pumping hand above banked 5,124 by 1:30 with four
  soldiers and no draining.
- Frame cost: one more mark a pump; the rig is a few distance functions a
  pixel over its box (about 2.6 crystal radii square). Not timed on the
  1070.

## Not done / for later

- Numbers are first numbers, to be tuned in play (build first, balance
  later). A pump's health (200) is unchanged though the mould now wears it
  over the whole crystal.
- Guarding a big crystal still pays well (the Fork's 7,768 crystal pays 65 a
  second guarded while full). If guarding becomes the new easy opening,
  that is the next lever; the owner's ruling keeps it above draining.
- The roguelike thread owns run pacing and the win threshold and was told.

## Pumped crystals last forever (2026-11-15 17:14Z)

Owner: "when building a pump on a crystal, it shouldn't drain it away. That
should be one of the advantages of pumping over harvesting. Pumped crystals
last forever".

- A pump no longer takes charge from the dish (`UnitIn::take` 0 for a pump).
  `Game::building_out` banks `pump_draw x pump_yield` a second while
  `Game::pump_flows` holds: the crystal has more than `PUMP_DRY` (1%) of its
  starting charge. Before, a level-1 pump took 1/150 of the charge a second
  and emptied its crystal in 2.5 minutes (faster upgraded).
- Hand draining still empties a crystal. The slime's eating is unchanged:
  the slime eating a pumped crystal down to crumbs is now the only thing
  that stops a pump (apart from the slime wearing the pump down), so a
  pumped crystal is still worth defending.
- The pump still lights the crystal's working spot (`Game::last_takes`, a
  take by no soldier with the charge the food stands for), and the rig's
  glow (`Building::drew`) now stays full until the slime eats the crystal.
- Score: a pump's food no longer counts in `Game::taken` (no charge left the
  crystal), so `take_share` (the W2b cap score) and the "eaten" tests count
  only real charge gone.
- Tests: `a_pump_draws_its_crystal_into_the_bank` now checks the crystal
  keeps its charge after 10 s and the pump took none;
  `a_pumped_crystal_lasts_until_the_mould_eats_it` (new) empties the
  crystal's cells and checks the pump stops paying. Full
  `cargo test --release --locked` passes; clippy clean.
- Smoke (no census; build first, balance later): mission 7 Spore season,
  CPU, seeds 1-4, `hands=idle,pumps+boom,turtle+boom`, main 4a040a4e
  against this change. Seeds 2-4 are identical on both (their pumps never
  ran a crystal dry inside the mission). Seed 1: pumps+boom went from 0 to 1
  star (best 0.37 to 0.57), 5,702 to 5,700 food harvested and 8,412 to
  9,375 slime killed. Goal met 1/4 on both; idle 0/4 on both. Nothing
  soft-locks: worst stall 41 s on both.
- Run and campaign economy: pumps now pay without end, so a long planet
  gives more food than before. Not retuned here; if a run gets too rich,
  the levers are `BuildRules::pump_share` and the run's Pump power item.
- Frame cost: less than before (no harvest disc per pump on either engine;
  one `crystals()` read per standing pump a frame on the CPU side).
