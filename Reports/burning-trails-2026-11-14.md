# Burning Scorch, and crater and alarm drawn on the ground (2026-11-14)

The owner asked about the trail cards (#244) at 08:40Z: "how are scotch and
alarm different. It just makes a virtual wall for the slime?". When told
the two trails showed only through the mould's answer, they said "They
should be visible though" and "100%". At 08:41Z they added: "Scorch actually burn the slime caught on the line - yes".

## What changed

- **Scorch burns.** A scorch trail burst now also kills the walkers within
  its radius (`squad::Burst::burn`, `RoundRules::trail_burn`, on by
  default). On the CPU a burn counts at once, the same as a kill. On the GPU
  a burn skips the per-burst read-back, and `World::burned` reads the count
  once a frame after the whole trail, so a trail costs one read-back
  instead of one per disc. The burned walkers count in `Rounds::killed` and
  `Game::killed`, and on their own in `Rounds::burned`.
- **Crater and alarm drawn from their fields** (the engine's `draw` pass,
  in the world only, on the agar):
  - Crater is an ember (`+ (120, 36, 8)` x steps left / `scorch_full`). It
    sits on exactly the cells the mould can't enter and fades as their
    count runs out.
  - Alarm is a green haze (`+ (30, 110, 40)` x `1 - exp(-alarm / 8)`).
  - Both are added to the colour, as the gas haze is, so the mould on them
    keeps its own colour, and the view's soil carries the tint. On 11-12 at
    04:38Z the owner turned down a char painted at every hit. This isn't
    paint: it is the field itself, for as long as the field lasts.
  - A first try at full ember brightness (`sqrt`, `(150, 46, 10)`) washed
    the whole front in red. A yellow alarm (`(80, 74, 14)` / 20) was lost
    on the gold mould.
- **The Alarm card's text was wrong.** The default mould (C) has
  `alarm_weight` 0, so it ignores alarm. The Sleeper, Swarmer and Grudge
  walk toward it (+3). Making the plain mould flee alarm is a dead end
  (`Reports/dead-ends.md`, the flee rows). The text now says what the card
  does, and what it should become is on an owner card.

## Films and counts

The page is [Burning Scorch Trails](https://claude.ai/artifact/9BaN41r6yri6gmEQ73e7nn)
(`burning-trails-2026-11-14/`). The setup is Fork seed 1,
`hand=hold@150,195 bank=20000`, 0:19 to 0:22, `centre=165,218,4.5`,
screen 800x500, on llvmpipe.

| loadout | mould | killed | burned | shots | trail bursts |
|---|---|---|---|---|---|
| plain | 24,651 | 678 | 0 | 156 | 0 |
| scorch+scorch | 23,215 | 2,297 | 1,493 | 136 | 1,892 |
| alarm | 24,651 | 678 | 0 | 156 | 1,424 |

The alarm row matches plain to the walker, because the plain mould doesn't
read alarm. All three rows are one seed, so treat them as a tuning note. Scorch now
kills 3.4 times what plain bullets do over these 3 s. If that proves too
strong in play, `trail_r` or `trail_burn` is the knob.

## Tests

- `a_scorch_round_burns_the_slime_on_the_line`: on a race world, scorch
  burns walkers, and the game kills more than with no round.
- `trail_cards_lay_discs_along_the_leg_and_cast_nothing`: only scorch
  burns, and `trail_burn` off turns it off.
- The CPU/GPU trail twin (`mouldgpu`) still matches with the burn.

## Not done

- The CPU picture (`BigDish::image`) draws neither the gas, the crater nor
  the alarm. The window plays on the GPU.
- The frame cost on the 1070, which is busy. The draw adds two reads per
  cell. The burn adds one read-back a frame while scorch fires.
