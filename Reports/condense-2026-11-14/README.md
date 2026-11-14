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

**Follow-up (art director, 11-14 10:58Z):** the settled bronze read as a
flat plate, because the plateau curve fed its height too. Now only its
colour takes the plateau; its height takes the raw thickness, veins and
all, raised by up to 0.6, so the slime-look thread's broad sheen (#263)
runs over it like dense wet metal (`page/bronze-texture-before-after.png`,
`page/bronze-texture.mp4`). Patch luma 84 before, 99 after, still under
gold's 138 and amber's 185. No added samples. Approved by the art director (12:08Z), with a watch item: in grey it is now 99 against the bolt red's 105, told apart by hue and gloss, so later passes must not brighten the bronze.

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

## Breakaway (`BigParams::roam`, 11-14 afternoon)

The owner, 15:06Z: "When slime condensed it can become a separate entity
that behaves different exploring further, eating crystals." 15:21Z, on
whether it is a new slime type or the same slime: "Not sure yet. There will
probably be both in the future, but we need to start building and play
testing and iterate." Built as the same slime, off by default (`roam=`, the
start menu's Breakaway, needs Condense):

- Every `roam` steps after it condensed (300, 10 s), a condensed walker
  picks the nearest crystal with at least `ROAM_FRESH` (0.8) of its charge
  left and at least `roam_far` (30) dish cells away. A keyed draw per crystal
  and step sends a `roam_share` (0.5) of the choices, so walkers that
  condensed together and chose the same crystal go or stay together.
- On the way it steers as a panicking walker does (sensors read only the
  way there, at its own pace, not slowed alone), stays condensed, and
  neither withers nor panics. It lands within `ROAM_LAND` (3) cells of the
  crystal's centre, or when the crystal runs dry, and eats there.
- Tests: `big::tests::condensed_slime_breaks_away_for_a_far_crystal` (off:
  the same hash; on: 2,026 left, 436 landed, up to 1,177 on the way at
  once); GPU twin "roam: condensed walkers break away, run and land" (1,364
  on the way, 58 left and 26 landed in the tick; 0 words and 0 positions
  off).
- GPU: Params grew a row, 912 to 928 (`ROAM_AT`: `roam`, `roam_far`,
  `roam_land` in fine cells, `roam_share`); the word's `ROAM` bit 18 and its
  crystal in bits 19-24. No new pass.
- Window (Fork, seed 1, farm, condense 300; `roam-window-*.txt`):
  condense only 18,422 / 19,316 / 26,563 walkers at 1:00 / 2:00 / 5:00 and
  2 soldiers at 5:00; with breakaway 19,428 / 21,285 / 28,104 and none. The
  pieces reach every far crystal by 1:00, and the farm hand harvests 4,654
  against 6,254. Clip: `page/breakaway.mp4`.
- On the way a piece stretches into a file of specks along the walls
  rather than travelling as a blob; holding it together is the obvious
  next step if the owner wants it to read as a creature.

## Open

- The art director's review of the transformation (owner, 08:34Z).
- The owner's playtest: whether 10 s ripe and a crowd of 20 condense too
  much or too little, and whether it should be on in shape C.
- 1070 frame cost.
- Breakaway: the owner's playtest; whether a piece should hold together as
  a blob on the way; whether it becomes its own slime type later.
