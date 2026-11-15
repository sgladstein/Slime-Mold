---
title: Guns, bullets and gun cards
kind: design
status: open
updated: 2026-11-15
sources:
  - Reports/card-text-2026-11-15.md
  - Reports/gun-budget-2026-11-14.md
  - Reports/gun-look-2026-11-15/README.md
  - Reports/gun-order-2026-11-14.md
  - Reports/gun-range-2026-11-15.md
  - Reports/guns-2026-11-14.md
  - Reports/no-recharge-2026-11-14.md
  - Reports/rolled-bullets-2026-11-14.md
  - Reports/spray-share-ab-2026-11-15/README.md
code:
  - Ammo::roll
  - Gun::roll
  - Run::range
  - Squad::gun
  - muzzle_flash
  - spray_one
---

# Guns, bullets and gun cards

## Now

A squad owns its gun (`Squad::gun`): numbers for range, rate of fire, spread,
bullets a shot, a card budget and slots, rolled around one of four kinds
(rifle, carbine, long gun, scattergun; `Gun::roll`). A squad with no gun fires
the standard rifle, so the campaign plays as before. The gun fires its cards
left to right with no recharge: every shot carries the next card, and a
payload (blast, bomb, gas, germ) is a bullet. Bullets are rolled cards
(`Ammo::roll`): damage plus speed, range and spread multipliers (0.25x to 4x),
priced by how good the roll is; modifier cards stack (range, rate, thrift,
smother). Cards cost, the gun's budget caps the total, and a split squad's both
halves keep the gun and cards. Card text is one short sentence with its key
number. Deeper planets roll bigger budgets and better stats; the run's shop
sells guns and cards, and the gun range (`Run::range`, start menu) is a free
armory to try any of them against slime ([the run](run.md)). The H loadouts
are test presets, not the game's way to arm. Nothing here is balanced.

**The look** (owner 11-15 08:30Z: the guns "should really be one of the
draws of the game"): a muzzle flash at the barrel's mouth whose jets are
the gun kind's (rifle three prongs, carbine short and wide, long gun a
lance, scattergun a fan), a recoil kick, tracers that glide between steps
from the barrel, a hit light when the bullet gets there, and the spray:
every walker a shot kills flies out of the hit back toward the shooter as
a droplet and dies where it lands, so kills are as without it
(`Rules::spray_secs`, `spray=0` turns it off;
[gun look](../../Reports/gun-look-2026-11-15/README.md)). How much of the
kill flies (`Rules::spray_share`, 1 all) is a blind A/B for the owner
([spray A/B](../../Reports/spray-share-ab-2026-11-15/README.md)).

## Owner's word

The quoted rulings, with times, are in [game decisions](../decisions-game.md):
2026-11-14 16:46-16:47 (the squad owns the gun, bullets are rolled), 18:05-18:38
(no recharge), 15:06-15:35 (costs and a budget) and 2026-11-15 02:57 (short
card text) and 08:30 (the guns a draw of the game).

## Open

- Whether per-shot mana is layered on if fights lack tension (owner 11-14).
- Balance of budgets, prices and rolls waits for the owner's playtest.

## Sources

- [guns](../../Reports/guns-2026-11-14.md), [rolled bullets](../../Reports/rolled-bullets-2026-11-14.md), [no recharge](../../Reports/no-recharge-2026-11-14.md), [gun budget](../../Reports/gun-budget-2026-11-14.md), [gun order](../../Reports/gun-order-2026-11-14.md), [card text](../../Reports/card-text-2026-11-15.md), [gun range](../../Reports/gun-range-2026-11-15.md), [gun look](../../Reports/gun-look-2026-11-15/README.md).
