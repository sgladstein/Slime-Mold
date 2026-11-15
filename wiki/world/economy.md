---
title: The squad economy
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/pump-economy-2026-11-15/README.md
  - Reports/squad-mode-2026-11-14/README.md
code:
  - BuildRules::pump_share
  - Game::income
  - Game::pump_draw
  - Game::pump_flows
  - Game::upgrade
  - Rules::hand
---

# The squad economy

## Now

Food comes from crystals and goes into one bank; everything is paid out of the
bank at once, and nothing costs food to keep (upkeep removed 2026-11-14, PR
#298). **Pumps are the main income** (2026-11-15, PR #322): one pump a crystal,
it snaps to the crystal's middle, draws 1/150 of the crystal's starting food a
second (at least 5; `Game::pump_draw`, `BuildRules::pump_share`), so a big
crystal pays far more, and two upgrades (400 then 800 food) each draw 1.5x.
A pump never drains its crystal (owner, 2026-11-15 17:14Z: "Pumped crystals
last forever"): it pays for as long as the crystal holds more than 1% of its
charge (`Game::pump_flows`), so only the slime eating the crystal stops it.
Draining by hand is a fifth of the old rate (`Rules::hand`); guarding a crystal
pays its charge left over 120 s. The food readout shows income only, never
negative (`Game::income`). All numbers are first numbers, to be tuned in play.
Detail: [report](../../Reports/pump-economy-2026-11-15/README.md).

## Owner's word

- 2026-11-15 03:07Z: "I'm thinking we might need to move primarily to the pumps instead of harvesting crystals. It's honestly just so easy the beginning of every map."
- 2026-11-15 17:14Z: "when building a pump on a crystal, it shouldn't drain it away. That should be one of the advantages of pumping over harvesting. Pumped crystals last forever"
- 2026-11-14 04:53Z: "There should be no maintenance costs in this game" (upkeep, rations and hunger removed).
- 2026-11-14 04:28Z: "guarding has to be more valuable in the long run".

## Open

- Guarding a big crystal still pays well; if it becomes the easy opening it needs a look (report, *Not done*).
- The run sells *Pump power* in place of the pump unlock ([the run](run.md)).

## Sources

- [pump economy](../../Reports/pump-economy-2026-11-15/README.md), [squad mode](../../Reports/squad-mode-2026-11-14/README.md).
