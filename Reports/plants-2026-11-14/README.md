# Plants (2026-11-14)

A new per-cell plant field for Larder World, fully separate from the old
grazing grass (owner, 2026-11-14 17:02Z). [Brainstorm](brainstorm.md):
ten couplings; recommended first build is food, cover and fire.

## Built (2026-11-14)

Owner's picks (17:29Z): no eating, harvest or sowing; fire, limited;
shots, thickets, slime-eating plants and fertile battlefields to try;
slow ground to test; cover to think about.

- `mould::plants`: growth and spread, four kinds, fire, char. CPU
  (`BigDish`) and GPU (engine.wgsl `plant` .. `ignite`), bit-exact twin
  (`plants_and_fire_run_on_the_gpu_as_on_the_cpu`).
- Map format agreed with levelgen: B = kind << 6 | density 1-63.
- Tuning (`examples/plantprobe.rs`, [stills](stills/)): growth 0.002/0.004
  greened a planet from one cell in 40 in 20 s, now 0.0007/0.0015; fire
  catch 0.2 went out in 4 s, now 0.08; `fire_damp` 1 never reached the
  body (2 killed in 50 s), 3 killed 501 in 80 s while the slime grew.
- The game ticks 15 a second, so the race world steps plants every tick
  (`plant_every` 1); at 8 fire crept 8x slower in the game than the probe.
- [Page](page/index.html) (films, counts): two bombs light a wildfire
  that stops at the slime body; +3.1 ms frame on llvmpipe.
