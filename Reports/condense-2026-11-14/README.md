# Dense slime condenses (2026-11-14)

The owner, 2026-11-14 08:07Z: "if the slime gets to a high enough density,
concentration in an area or it stays near an energy crystal long enough, it
reabsorbs itself and reduces its density/coverage by half but gets twice as
strong and healthy. This makes it more difficult and stops the slime from
just filling the map." 08:34Z, on the look: "especially during the
transformation process. It has to look cool".

Built as `BigParams::condense` on both engines, off by default (the
`condense=` key, the start menu's Condense option).

## The rule

- **Ripe.** An ordinary walker is ripe on a step when it took crystal this
  step, or when the 5 x 5 square round its cell (its own cell left out)
  holds at least `condense_crowd` walkers (20 of 24 by default, so only the
  packed body ripens: at 16 the 2-3 cell tubes between crystals ripened too
  and broke into beads, *In the window* below). Its ripeness counts up a
  step while ripe and down while not.
- **Condensing.** At `condense` steps ripe (300, 10 game seconds, on the
  menu) a keyed draw splits the ripe walkers: half are absorbed (they die
  where they stand) and half condense, taking the absorbed one's energy.
- **Two walkers' worth.** A condensed walker pays twice the upkeep; births
  stop while the mould's mass (living walkers, a condensed one twice)
  reaches its slots, so the freed slots do not refill; its bite and its
  hold on a soldier's legs count twice (`squad::read`'s touching); and it
  has two lives: a shot, a blast, withering or starving breaks it back into
  an ordinary walker (counted as a kill), and only the next one kills it. A
  child is an ordinary walker.

**Why this strength.** Condensing makes no mass: the body keeps what it
held on half the ground. Per area the bite is unchanged (half the walkers,
each biting twice), so walking into thick slime stays as deadly as the
owner asked (11-12 22:22Z); what doubles is the work to clear it: a shot's
disc holds half as many walkers and each takes two hits, so a shot takes
half the mass it did. Health is the two lives; withering also only breaks a
condensed walker first, so a cut-off condensed piece thins at half the
speed. Faster growth or regeneration were not chosen: either would let it
fill the map again, which the owner wants stopped.

## Tests

- `big::tests::dense_slime_condenses_to_half_its_walkers_at_its_mass`
  (CPU, small crystal dish, 400 ticks): off changes nothing (same hash);
  on, 1,077 walkers against 1,388, 855 of them condensed, 1,594 absorbed and
  1,616 condensed in all (a fair split), mass at most the slots.
- `big::tests::a_condensed_walker_has_two_lives`: shot, blast, withering
  each break it first, then kill.
- `squad::tests::a_condensed_walker_touches_as_two_and_takes_two_shots`.
- GPU twins (`src/bin/mouldgpu.rs`, software Vulkan): the world tick
  ("condense: dense walkers ripen, condense and break": 22 absorbed, 20
  condensed, 682 broke in the tick; 0 slots, 0 words, energy L1 1.4e-6) and
  the squad frame ("condensed walkers take two shots and touch as two": 20
  broke, the squad's outputs and touching equal) match the CPU exactly.

## On the GPU

Params' fling row's three spare words (900 `condense`, 904 `ccrowd`,
908 `s_condense`); the state lives in binding 48 after the sleep gas's
slots (a 4-word header with the mass, a word a slot, a word a cell);
five passes: `ripen` before `life`, `massclear`/`masscount` before `breed`,
`dclear`/`dcount` before the frame's `units`. Off, none of them runs.
Frame cost on the 1070 not measured (no PC jobs, owner 11-14 00:28Z);
`ripen` reads 24 cells a walker, about what the lone step reads.

## The look

Reworked after the art director's review (11-14 08:47Z): the first copper
(215, 88, 30) sat between the panic's amber and the bolt's red, and the turn
was a fast dark crackle that read as damage. Now, all from the walkers' own
state (`View::set_dense`):

- **Ripening** warms the body toward the bronze by ripeness squared (about
  half way at the turn), and cools if it stops being ripe.
- **The turn is an implosion.** An absorbed walker's slot keeps
  `ABSORBED` and the steps since for `ABSORB_STEPS` (15, 0.5 s; both
  engines, nothing in the simulation reads it, a birth clears it). The view
  draws it as a warm-white streak from where it died toward the centre of
  the condensed walkers in the 9 x 9 cells round it (the engine's per-cell
  condensed count), accelerating and dimming. The streaks have their own
  half of the specks buffer, so the specks' calm/lean/bolt fields keep
  their room (the slime-look thread's ask).
- **Light from the data.** Absorptions in the last 0.3 s light their cells
  warm-white by how many (the fine grid's second half, above bit 16), so a
  big condensation flares and a trickle glints, then it cools into the
  bronze. A survivor's speck flares for 0.4 s (2 s read as noise).
- **Condensed** mould is a deep bronze-brown (92, 58, 34) mixed into the
  body after the gold's saturation push (mixed into the hue before it, the
  push turned it orange), with gloss near full, a steeper thickness curve
  (a fuller body with a short edge) and up to half again the height. Taking
  more of the sharp count instead made it lumpy and glittering.
- **Value check** (`page/grey-strip.png`, mean luma of real frames): gold
  body 138, ripening 121, condensed 84; the panic amber speck 185, the bolt
  red 105.
- **Frame cost** (llvmpipe, `bench=40 hand=farm screen=960x600`,
  `bench-off.txt`, `bench-on.txt`): frame p50 63.6 ms off, 64.8 on (+1.9%),
  p95 83.0 and 86.0. The 1070 is not measured.

The zoom 12 clips (`page/turn-z12-1x.mp4`, `page/turn-z12-half.mp4`,
`clip-z12.txt`): one patch by the home crystal from 9.0 to 11.8 s, every
game step.

## In the window

Fork, seed 1, `hand=farm`, shape C (the default), software Vulkan,
`larder-world look=DIR at=60,120,180,240,300` (raw: `window-off.txt`,
`window-on.txt`). Mould is walkers, not mass.

| | 1:00 | 2:00 | 3:00 | 4:00 | 5:00 | squad at 5:00 |
| --- | --- | --- | --- | --- | --- | --- |
| off | 28,274 | 21,727 | 29,628 | 37,725 | 44,806 | 1 (17 lost) |
| `condense=300 condense_crowd=20` | 17,147 | 17,434 | 24,072 | 27,613 | 26,904 | 3 (19 lost) |

Off, C covers most of the open ground by 5:00; on, it holds its crystals in
bronze lobes joined by thin tubes, about 60% of the walkers. One seed and
one hand, so the squad's line is no balance reading. At `condense_crowd` 16
(the first try) the tubes between crystals ripened and broke into beads by
2:00; 20 keeps them whole.

The transformation clip (`page/transform.mp4`, `clip-on.txt`): the start
blob on the home crystal, 6 to 30 s every half second at zoom 7. The blob
warms from the crystal outward, turns at 10 s (24,871 walkers at 6 s,
18,773 at 10 s), streams in and sits as a darker, raised bronze body.

Page: [Dense slime condenses](https://claude.ai/artifact/7L9QoQxqnk9Cazt1qyFpvR) (source
`page/`).

## Open

- The art director's review of the transformation (owner, 08:34Z).
- The owner's playtest: whether 10 s ripe and a crowd of 20 condense too
  much or too little, and whether it should be on in shape C.
- 1070 frame cost.
