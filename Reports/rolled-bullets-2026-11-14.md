# Rolled bullets (2026-11-14)

The owner's spec, 16:46Z: "Bullets define damage and have multipliers(0.25 to 4x, but it can be decimals 1.4x) for speed, range, spread, there will also be base bullet sets that are more likely to have certain multipliers. The bullet cost will depend on how good the damage and multiplier role plus random luck."

Three later rulings shape it:

- 18:05Z: payloads (blast, bomb, gas, germ) are bullets.
- 18:07Z: luck comes from the normal distribution of each roll.
- 18:38Z: "The bank is just to build the gun. How many and which cards."

The roguelike thread asked for `roll(seed, depth)` with a cost, so the shop's armory can sell bullets. The owner asked for icons on guns and bullets (16:55Z).

## What a bullet is

A bullet card is `rounds::Card::Ammo(Ammo)`. The module is `src/mould/bullets.rs`. A bullet has a shell, a set, and four rolled numbers plus a cost.

- **Shell** is what the bullet does where it hits: plain, blast, bomb, gas or germ.
  - Today's payload cards (Blast, Bomb, Gas, Germ) and the new `Card::Plain` are the standard bullets. They have damage 10, every multiplier at 1x, and the cost they had (plain 1, blast and gas 3, bomb and germ 4).
- **Damage** is the hole it makes. A plain bullet's strike radius and a payload's burst radius are multiplied by `sqrt(damage / 10)` (`Ammo::reach`), so the area grows with the damage.
  - On the GPU the strike box is 15×15 (`squad::R_MAX` 7), so a plain bullet's strike stops growing at about 1.4x. Payload bursts have no cap.
- **Speed** multiplies how far the bullet moves each step (`Bullet::speed`).
- **Range** multiplies how far it flies before it drops.
- **Spread** multiplies the gun's spread.
- Each multiplier is kept within 0.25x to 4x. They are stored in hundredths, so 1.4x is exact.
- On screen, a bullet's size follows its reach and its colour follows its shell.

## Sets

A set is the lean that the rolls centre on.

| set | damage | speed | range | spread | reads as |
|---|---|---|---|---|---|
| Standard | 1 | 1 | 1 | 1 | the plain gun's numbers |
| Slug | 1.6 | 0.6 | 0.9 | 0.6 | heavy, slow, a little short, tight |
| Needle | 0.6 | 2.0 | 1.5 | 0.5 | light, fast, far, tight |
| Buckshot | 0.8 | 0.9 | 0.6 | 2.5 | short and wide |

## Rolls and cost

- `Ammo::roll(seed, depth)` picks a shell and a set, each equally likely, then rolls the numbers. `Ammo::roll_of(shell, set, seed, depth)` fixes the shell and set. The same seed and depth always give the same bullet.
- Damage is 10 × the set's lean × the depth factor × `exp(0.2 z)`.
- Each multiplier is the lean × `exp(0.25 z)`, kept within 0.25x to 4x.
- Every number has its own normal draw, as the guns do (18:07Z). Most land near the lean, some are much better or worse, and a few are far out. At depth 1, 198 bullets in 20,000 have a multiplier at a cap.
- Deeper planets roll harder bullets: damage rises 4% per planet, up to 1.4x. At depth 1 the median damage is 8.9 (90th percentile 16.9). At depth 8 it is 11.5 (90th percentile 21.6).
- **Grade** is how good the roll is. It is the exponential of (log damage/10 + ½ log speed + log range − ½ log spread) / 3, so harder, faster, farther and tighter is better.
  - At depth 1 the median grade is 1.05, the 90th percentile 1.31, the 99th 1.53 and the best 1.81. At depth 8 the median is 1.13.
- **Cost** is the shell's base × grade^1.5 × `exp(0.15 z)`, rounded, and at least 1. The last factor is its own draw: the "random luck" in the price.
- A card's cost is paid from the gun's budget when the gun is built, as any card's is. Firing costs nothing.

Cost over 20,000 rolls:

| shell | depth 1: median (10th to 90th percentile, most) | depth 8: median (10th to 90th, most) |
|---|---|---|
| plain | 1 (1 to 2, 3) | 1 (1 to 2, 3) |
| blast | 3 (1 to 5, 9) | 4 (2 to 5, 12) |
| gas | 3 (2 to 5, 10) | 4 (2 to 5, 10) |
| bomb | 4 (2 to 6, 11) | 5 (2 to 7, 14) |
| germ | 4 (2 to 6, 12) | 5 (2 to 7, 13) |

Costs are whole numbers on the gun's budget scale (a standard rifle has budget 10), so a plain bullet costs only 1 or 2. Finer prices need a finer budget scale, which is for the balance pass.

## In the game

- **Loading.** Bullets take slots and budget as any card does (`rounds::fit_gun`). Each shot fires the next bullet in turn. A modifier before a bullet works on it as before (Wide, Scatter, Quiet and the rest). A scattergun's whole fan fires the same bullet.
- **Flight.** A shot's numbers come from its bullet. `turn.rs` reads `BulletMods::ammo` for the strike, reach and spread, and `shots.rs` reads it for the speed.
- **HUD.**
  - A bullet card shows its shell's icon. Plain uses a new bullet icon.
  - A payload bullet of a set other than Standard also carries its set's badge in the bottom right (`Icon::Slug`, `Icon::Needle`, `Icon::Buckshot`).
  - The tooltip gives the title ("Needle gas", "Slug bullet") and its numbers and cost.
  - A load with rolled bullets is named "Rolled bullets".
- **Text.** A bullet's text form is `ammo/SHELL/SET/DEPTH/DAMAGE/SPEED/RANGE/SPREAD/COST`, all whole numbers. It works in any round's text (`rounds::parse`), so `Cmd::Load` and replays carry bullets.
- **Test keys.**
  - Ctrl+H rolls a new load for the active squad's gun (`input::bullet_load`): as many bullets as fit its slots and budget.
  - `ammo=N` starts the first squad with the Nth load.
  - `ammo=ROUND` starts it with those cards, for films. The log names the load.
  - In a run, Ctrl+H says that the gun holds what was bought.
- **For the shop**: `rounds::Ammo::roll(seed, depth)`, `a.cost`, `a.title()`, `a.line()` and `Card::Ammo(a)`. Icons come from `draw::ammo_icon(set)` and `card_face`.

## Films

Setup: Fork map, seed 1, `hand=hold@150,195 bank=20000`, from 0:19 to 0:22, standard rifle unless named, on llvmpipe. Counts are at 0:22, from the start of the game. The page is `rolled-bullets-2026-11-14/index.html` ([Artifact](https://claude.ai/artifact/5LDB46fERtjeUTL9Bexz6U)).

| load | killed | shots |
|---|---|---|
| standard bullet (plain) | 655 | 156 |
| slug bullet (damage 16, speed 0.6x, range 0.9x, spread 0.6x) | 957 | 148 |
| needle bullet (6, 2x, 1.5x, 0.5x) | 434 | 151 |
| buckshot bullet (8, 0.9x, 0.6x, 2.5x) | 541 | 151 |
| standard gas, then standard blast | 2,888 | 113 |
| needle gas (6, 2x, 1.5x, 0.5x), then slug blast (16, 0.6x, 0.9x, 0.6x) | 2,543 | 107 |
| rolled load `ammo=1`: needle blast (5.2, 3.0x, 1.3x, 0.5x, cost 4), buckshot bomb (8.7, 0.8x, 0.4x, 1.5x, 2), buckshot germ (9.7, 0.7x, 0.6x, 2.7x, 2), buckshot blast (8.6, 1.0x, 0.6x, 2.1x, 2) | 1,278 | 152 |
| the same load on a scattergun (the first three fit) | 977 | 344 |
| check: rifle on `lull` | 2,816 | 124 |

The `lull` check matches #288's after-film exactly, so standard cards play as before.

At this close front, damage decides most of the result. A slug kills half again what a standard bullet does, and a needle two-thirds as much. Speed and range matter little when the mould is within a few cells. They should count on a longer field, which these films do not test.

In the plain-set clip the four bullets are hard to tell apart. Most hit within a cell or two of the squad, so there is little flight to see. The payload pair shows the difference plainly: a slug blast's ring is about a quarter wider (1.26x) and needle gas's cloud about a quarter narrower (0.77x). These films are from one seed, so read them as a tuning note.

## Checks

- **`bullets.rs` tests**:
  - `a_standard_bullet_is_the_fixed_card`;
  - `rolls_are_the_same_for_a_seed_and_stay_in_bounds`;
  - `each_set_leans_its_rolls_the_way_it_says`;
  - `a_better_roll_and_a_payload_cost_more`;
  - `deeper_planets_roll_more_damage`.
- **`a_bullet_cards_numbers_ride_its_shots`** (rounds), on a round of a slug bullet, Wide and a needle blast:
  - the round reads back from its text and costs 9;
  - the plain bullet casts no burst, and the blast with damage 40 bursts twice as wide before Wide;
  - in play, the slug's bullets fly at half speed and half range with a wider strike, and the needle blast's at twice the speed.
- **`rolled_bullets_load_within_the_gun_and_show_their_numbers`** (window): for each gun kind and 20 loads at depth 3, the load fits the gun's budget and slots. Every card has an icon, a bullet card's name is its title, and its tooltip gives its damage.
- The full suite, clippy and the wiki lint pass.

**Frame cost**: one array lookup per shot and per bullet each step. Not timed.

## Next

1. A finer budget scale, so plain bullets can differ in price.
2. The trigger card.
3. Time a scattergun on blast on the 1070.
4. Lift the GPU strike box's cap for heavy plain bullets (`R_MAX` 7).
