# No recharge: every shot fires the next card (2026-11-14)

The owner answered the gun thread's four questions at 18:05Z and followed up:

1. Payloads are bullets: "Yes" (blast, bomb, gas and germ are bullets, not modifiers).
2. The scattergun: "2-6 bullets per shot" (18:08Z).
3. "Why does the gun have a recharge at all. That shouldn't be a thing. It has a rate of fire. It never has to stop to recharge or switch to regular bullets."
4. "Both halves get a copy of the gun."

Two later notes changed the gun rolls, and a third ruled out paying for shots:

- 18:07Z: "this should just come out of the normal distribution (unless you suggest a different distribution) that you use for the randomness. chance for every stat to be slightly better or slightly worse, and smaller chance for significantly better or worse, and really small change for way better or way worse. The luckiness of the gun depends on all those rolls, but the whole thing is a distribution."
- 18:18Z: "Nothing needs to be labeled lucky".
- 18:38Z, answering the thread's card on payload prices: "The bank is just to build the gun. How many and which cards."

## What changed

- **No recharge.**
  - Every shot a soldier fires carries the next cast of its squad's cards in turn, back to the first after the last. A cast is the modifiers and trigger before a payload, plus the payload. The gun fires at its own rate of fire and never stops.
  - The cast goes off where that bullet hits slime. A bullet that hits nothing casts nothing.
  - The cast index lives in the game: `Soldier::next` is the next cast, `Bullet::cast` and `Shot::cast` are the cast a bullet carries, and `BulletMods::casts` is the round's count of casts.
  - A scattergun's whole fan carries the same cast. Each pellet that hits casts it.
  - A piercing bullet casts at each strike. Split pieces are plain.
  - `Rounds::after_step` casts what each hit carried, from the squad's own cards or the army's round.
- **What went.**
  - In the game: `RoundRules::recharge` (2 s), `payload_weight`, `card_weight`, `wide_weight`, `Card::weight` and `rounds::recharge`.
  - In `Rounds`: each soldier's chamber, along with `Rounds::gun` and `Rounds::next_cast`.
  - In the HUD: the recharge bar and its "Recharging" state. The strip now says "A card a shot".
  - In the alert: H's "recharge N s".
  - Until now a gun recharged for 2 s after its last cast, plus 1 s per payload, 1 s for Wide and 0.5 s for any other card, and shot plain bullets meanwhile. A hit, not a shot, took the next cast.
- **Payload bullets are free to fire** (owner 18:38Z).
  - With no recharge, the toolkit's bank prices (blast 60, gas 150, germ 300, bomb 60) emptied the bank within seconds. On Fork seed 1, eight soldiers on `lull` (gas and blast in turn) spent 373 a second at 0:20, "dry in 34 s" from a 20,000 test bank. That would empty a game's 2,000 start bank in about 5 s, and then every shot would be plain.
  - The thread asked the owner, and the answer was that the bank only builds the gun. The per-cast price is gone: `Cast::price`, `RoundRules::bomb_price`, `Rounds::spent` and `Rounds::broke`, along with the price on a payload card's tooltip. `rounds::casts` no longer takes the rules.
  - The squad toolkit's own tools (the E page) still cost bank.
- **Splits.** Both halves of a split carry a copy of the gun and the cards (`Squad::offshoot`). From 17:40Z to now the new half took the standard rifle and the army's round.
- **Gun rolls from the normal curve alone.**
  - Each stat is the kind's average times `exp(sigma z)`, as before, with no extra lucky chance. From 17:40Z each draw had its own 1-in-6 lift of 1.5 sigma or more.
  - Slots are the kind's slots plus `0.7 z` plus 0.1 a planet deeper (up to 0.6), rounded. A quarter of guns get one more slot, a quarter one fewer, and one in 60 two more.
  - A scattergun's bullets a shot are `4 + z`, rounded and kept within 2 to 6: 2 and 6 about 1 gun in 14 each, 4 two guns in five.
  - At a rate sigma of 0.15, two stats in three land within 15% of average, one in 44 is 35% better or more, and one in 740 is 57% better or more. The bad side is the same.
  - Over 6,000 rolls, `grade` has a median of 0.995. The 90th percentile is 1.105, the 99th 1.205, and the best gun 1.33. The roguelike shop prices by grade.
- **No lucky label.** `Gun::lucky` and `LUCKY_GRADE` are gone, and `Gun::title` is the kind's name. The text form drops the flag, and `from_words` still reads #283's lines.

## Films

Setup: Fork seed 1, `hand=hold@150,195 bank=20000`, from 0:19 to 0:22, `centre=165,218,4.5`, on llvmpipe. Before is `origin/main` at f1c64cf4 (#283) and after is this branch. The counts are at 0:22, from the start of the game. The page is `no-recharge-2026-11-14/index.html` ([Artifact](https://claude.ai/artifact/YPVWVXcwbPaPQsTGzRvDZF)).

| film | killed | shots (bullets) | bank, from 20,000 | mould |
|---|---|---|---|---|
| rifle, `lull` (quiet+gas+quiet+blast), before | 2,143 | 121 | 14,387 | 23,304 |
| rifle, `lull`, after | 2,816 | 124 | 18,386 | 22,671 |
| scattergun (4 a shot), `blast`, before | 1,166 | 316 | 16,001 | 24,127 |
| scattergun, `blast`, after | 1,734 | 300 | 18,134 | 23,698 |

Every shot now carries a card, so the same front kills about a third more on the rifle and half again on the scattergun. In the new films the bank only falls from the game's other spending; the guns spend none. These come from one seed, so they are a tuning note. Balance is the next pass's job, not a gate.

## Checks

- **New test** `every_shot_fires_the_next_card_in_turn_and_the_gun_never_recharges`, with the rifle and the scattergun on `blast+wide+gas`, 30 s on the attack:
  - every soldier's shots run blast, gas, blast, gas with never a plain one;
  - a scattergun's fan carries one cast;
  - every hit casts what its bullet carried;
  - the rifle's casts pass the old recharge's ceiling;
  - a `delay+blast` round lodges and goes off;
  - an empty bank still blasts and pays nothing.
- **Gun and split tests**:
  - `luck_is_the_normal_curve_in_each_roll_and_the_grade_sums_them` checks the grade's spread, finds a great range on an ordinary gun and confirms #283's text lines still read;
  - the scattergun's fans are mostly 3 to 5;
  - `each_squad_fires_its_own_gun_and_both_halves_of_a_split_copy_it` and the deck test check that a split copies the gun and cards.
- The loadout panel test checks that a payload card shows its budget cost and no bank price.
- The full suite, clippy and the wiki lint pass.

**Frame cost**: one `mods(q)` lookup each soldier shot, and the after-step no longer tracks per-soldier chambers. Every shot carrying a card means more casts a second, so a payload squad does more burst work a frame than before. The window's "model and HUD" line was 0.05-0.07 ms a frame in both builds on the cloud. Not timed on the 1070, where a scattergun squad on blast (four bursts a shot) is the case to time.

## Next

1. Rolled bullets: damage plus 0.25-4x multipliers on speed, range and spread, bullet sets that bias the multipliers, and a cost from the roll (`Bullet::roll(seed, depth)`) for the roguelike shop. Plain, blast, bomb, gas and germ will be bullets.
2. The trigger card.
