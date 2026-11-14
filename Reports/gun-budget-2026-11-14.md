# Gun budget and three new cards (2026-11-14)

This works through the owner's notes of 11-14 15:06Z (relayed by the
coordinator): "Each card should have a cost and guns have x amount of bank
to spend. You have bullet types, modifiers (2-4x distance or rate of fire,
trigger next card on hit, reduce cost, piercing, disrupt scent, explosion,
etc), modifier can be placed on top of bullets". The test loadouts stay
for now.

## The budget (PR #275)

On a card at 15:34Z the owner picked "Capacity at loadout": a gun takes
cards only if their costs fit its budget, and equipped cards fire on the
recharge as before. When the owner asked whether this limits the system,
the answer was mostly no. A cost-cut card still frees room, each card
already weighs on the recharge, and a cost is one number a card, so mana
per shot can be laid on later if fights lack tension.

- `Card::cost`: bomb and germ 4; blast, gas, scorch and mist 3; wide,
  scatter, pierce, split, bait, range, rate and smother 2; quiet, delay and
  thick 1; thrift 0.
- `Card::tier` (1-3): the roguelike thread uses it to gate rewards by depth.
- `rounds::fit`, `Rounds::build`, and `RoundRules::budget` 10. Every H preset fits.
- The HUD reads "Cost N of 10" under the cards, and each tooltip names its cost.

## New cards

| card | kind | cost | what it does |
|---|---|---|---|
| `rate` | flight | 2 | every soldier fires `1 + rate_card` (2) times as often a card; `Game::shot_gap` |
| `thrift` | modifier | 0 | the next card costs 2 less (`THRIFT`); thrifts stack, not below 0 |
| `smother` | trail | 2 | every bullet wipes the food scent along its flight (`Burst::smother`; the CPU's `lay_bait` and the GPU's `baitlay` with keep 0) |

`range` goes from +50% a card to +100% (`Rules::range_card` 1.0), so one
card doubles the reach to 48 cells, two give 72 and three give 96. This
follows the owner's "2-4x". The new loadouts are `rapid` (rate+rate+blast),
`thrift` (thrift+bomb+thrift+bomb) and `smother` (smother+smother).

"Trigger next card on hit" waits on the owner's answer about what a bullet
type is.

## Films and counts

Page: [Gun Budget Cards](https://claude.ai/artifact/SdVwwiKmXXwFRSttsJb9CK) (`gun-budget-2026-11-14/`).
The setup is Fork seed 1, `hand=hold@150,195 bank=20000`, 0:19 to 0:22,
`centre=165,218,4.5`, on llvmpipe.

| loadout | mould | killed | shots |
|---|---|---|---|
| plain | 24,657 | 655 | 156 |
| rapid (rate+rate+blast) | 22,889 | 2,569 | 400 |
| sniper (range+range+pierce) | 22,809 | 2,501 | 176 |
| smother+smother | 24,606 | 697 | 152 |

This is one seed, so it's a tuning note. Two Range cards at 72 cells let a
holding squad reach the thick body well beyond the front. That kills about
as much as two Rate cards plus blasts, and makes sniper the strongest
budget-6 loadout here. Smother barely shows over 3 s, because the scent it
wipes refills and the mould in front of the squad is already on the
squad's own scent. Its use is to cut a line between the mould and a
crystal. A longer clip of that case is still to film.

## Cost

- Range: one card makes each soldier's sight disc 4x the area, and two
  make it 9x (GPU `units` pass). This hasn't been timed on the 1070, which
  is busy.
- Smother: one extra pass per trail disc, the same as bait.

## Tests

- `a_gun_takes_only_the_cards_its_budget_holds`: covers the budget, thrift
  and the slot cap.
- `every_test_loadout_fits_the_default_budget`.
- `a_rate_card_fires_twice_as_often`.
- `a_range_card_lets_soldiers_engage_further_and_shoot_further`, now at 48 cells.
- `trail_bursts_lay_the_same_fields_on_the_engine_as_on_the_cpu` now
  smothers the bait line on both engines and compares the scent cell for
  cell.
