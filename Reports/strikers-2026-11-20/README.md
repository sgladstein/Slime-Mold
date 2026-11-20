# Condensed strikers (2026-11-20)

The fourth rule from the thread "Make the slime a more interesting enemy",
and A of the balance thread's late-threat brief, which the owner picked on
its card on 11-20 ("Condensed strikers + late lunge"; the lunge, B, is
the balance thread's).

Page: https://claude.ai/artifact/LqNVaUtqhshZNLzEfPbW6m (source in `page/`).

## What was built

`src/mould/larder_world/strikers.rs` (`StrikerRules`,
`Game::time_strikers`), on in every run (`run::ENEMY_RULES`' `strikers`).
It is GPU only: `World::set_strikers`, `World::condense`,
`gpu::Engine::condense`, and engine.wgsl's `condense` and `condlead`.
It also uses `armoured`, `dcount`, `panic_to`, `lumpform` and `ripen`.

Under 40% of its peak, once a game second:

- **Who tries.** Each ordinary walker within the guns' reach plus 12
  dish cells of a soldier tries, with chance 0.5. The reach is measured by
  the units' walking distance.
- **Who takes in whom.** In each 2 x 2 block of fine cells, the first
  walker that tries takes in up to three of the rest.
- **What a condensed walker does.** It takes one blow per walker it holds,
  plus one, and bites once more per walker it holds. It charges the
  nearest soldier at 1.5 times its pace.

The look is view.wgsl's `draw_striker`: a lit bead in the droplets' layer,
sized by what it holds, with a pulse every 48 steps. It has no bronze.

## Measured

Engine test
(`condensed_strikers_take_in_their_neighbours_charge_and_take_more_blows`,
race world, one soldier 25 cells from home):

- **Condensing.** 198 walkers were taken in by 107 strikers, which hold
  198 between them. The slime lost exactly 198.
- **The charge.** In 6 s the strikers closed from 13.6 to 5.4 cells, while
  the plain slime went from 35.4 to 38.7.
- **A strike.** One strike left 58 of 58 strikers alive, each holding one
  fewer, and 0 of 5 plain walkers.

Play, one game each, planet 2, run seed 1, `hand=strat:econ,go=4,aim=6
bank=20000`:

- **On.** Strikers formed from 1:03, with 157 to 254 alive until 1:16 and
  629 walkers taken in by then. At 1:10 the squad had 45 soldiers at 0.53
  health.
- **Off.** At 1:10 the squad had 47 soldiers at 0.52 health.

A first version measured reach in a straight line. On planet 3 it
condensed 2,000 walkers across deep water from the squad. They pressed on
the bank for a minute while the bot shot across. Measuring by walking
fixed that. On planet 3 few form now, because the bot mostly fights
across water.

The deciding measure is the balance thread's: 9 maps on planet 3, on
against `set=strikers=0`, soldiers bitten to death in the last quarter.

## Commands

```
larder-world look=DIR run=1:2 mode=squads hand=strat:econ,go=4,aim=6 bank=20000 [set=strikers=0] centre=355,135,ZOOM at=62,62.5,..,76
python3 make_clips.py DIR OUT.mp4
```
