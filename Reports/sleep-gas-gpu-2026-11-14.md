# Sleep gas on the GPU engine

2026-11-14, gun rounds thread, cloud plan item C5
([plan](cloud-plan-2026-11-14.md)). The squad toolkit thread is porting the
germ, so after this only the germ is still CPU-only among the toolkit's
tools.

## What was built

The squad toolkit's sleep gas (`BigDish::gas`, `gassed_near`,
`BigParams::gas_death`) now runs on `gpu::Engine` and the GPU world.
`GpuWorld::has_gas` is true, so the window's Sleep gas button, the gas
rounds (`gas`, `lull`) and the toolkit's gas now work in the native game.
No file in the game layer changed.

- `Engine::gas` (pass `gas`). Every living ordinary or dormant walker in
  the disc falls dormant with its timer at 0 and stays gassed for the
  burst's steps. `fate` reads the gas. While gassed, a sleeper ignores
  scent, so only alarm wakes it, and it dies asleep at
  `max(gas_death, sleep_death)`. It uses the CPU's draw, `DRAW_SLEEP_DIE`.
- **Each slot holds the step its gas runs out at.** The CPU counts every
  slot's `gassed` down each step. On the GPU a slot is gassed while the
  step is before its word, so no pass runs while nobody is gassed, and a
  tick costs one extra read for each dormant walker. A birth clears the
  child's word.
- **No new binding.** wgpu's limit is 48 storage buffers a shader stage,
  and binding 48 is the 48th (software Vulkan reports 48; the first try at
  binding 49 failed to make the layout). The gas words therefore sit in
  `news` (binding 48, food's news) after its blocks: a 4-word header, 96
  calm points, 96 calm counts, then a word a slot. A buffer added later
  must share an existing binding in the same way, or two buffers must be
  merged first.
- **Calm** (`Engine::calm`, pass `calm`; the CPU's `gassed_near`): the
  gassed sleepers near each unit, which the game takes off the unit's
  touch, so a gassed mould doesn't bite. `GpuWorld::calm` runs it after
  a frame. It costs a pass and a read-back each frame, but only while some
  gas lasts (`Engine::gas_until`). The read-back is a sync point for those
  frames. Moving the calm into the frame (no read-back) would need each
  unit's calm radius in the squad buffer; that change belongs in the
  game files, which are frozen for the split today.
- **CPU change:** `gas` and `gassed_near` now compare squared distance
  instead of `hypot`, so both engines include exactly the same walkers.
- **Params:** word 232, `_p2` in the sick row, is now `gas_death`
  (`Settings::gas_death`, the `gas_death=` key). The germ port takes 228
  and 828.

## Tests (software Vulkan)

- `sleep_gas_matches_the_cpu` (`mouldgpu`) runs three dishes: plain (the
  gas turns the life states on), dormancy with alarm, and growth on
  crystals. It gasses about 270 walkers at 30 steps with `gas_death` 0.05,
  then runs 40 ticks with no re-inject.
  - States and gas words match exactly on every slot at 10, 20, 30 and 40
    ticks.
  - The calm matches at three points.
  - The gas kills sleepers and runs out on both engines, and the calm
    reads zero afterwards.
  - Planted: with the GPU's `gas_death` at 0, 105 states differ after 10
    ticks. With its gas 1 step longer, 265 gas words differ. The test
    catches both.
- `a_gassed_mould_spares_the_squad_on_the_engine_as_on_the_cpu` runs the
  race world, seed 3. The squad attacks, and the gas lands on the first
  soldier bitten. With the CPU dish injected each frame, the squad's
  health matches to the bit on all 240 frames. With the gas, total health
  at 16 s was 379.2; with none, 377.9. Only 8 walkers are near the soldier
  when the mould first bites in the thin race world.
- The window smoke tests (`hand=every` presses every command on the card)
  now press Sleep gas on the GPU world too.

## Frame cost

- Unchanged per tick while nobody is gassed: one extra word read for each
  dormant walker in `fate`, and one word written for each birth.
- While a gas lasts: one `calm` pass over the slots, plus a read-back,
  each frame.
- Not timed on the GTX 1070 (the owner's PC is busy). The 1070 twin is
  pending. Book it with the C6 batch.
