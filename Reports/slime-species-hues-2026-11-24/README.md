# Each late slime its own colour (2026-11-24)

Look lane, Milestone 2's Look B (the slime itself), Direction's job
"planets 4 to 8 each read differently at zoom 6". Page:
https://claude.ai/artifact/9f8e61t7jJxyBc2ymhGzRA (source `index.html`
here, clips beside it).

**Before.** Planets 4 to 8 filmed at rest (run seed 1, idle hand, zoom 6,
centred on the heart at 1:00 and 2:00): the Swarmer (4), the Sporer (5)
and the Grudge (7) were the same bumpy gold. Their own looks (`Swell`,
`Spores`, `Scarred`) wake only once struck. The Sleeper (6) read apart by
its crust; planet 8 takes turns among the hardest three.

**After** (`species_hue` in `src/world/view.wgsl`, used for the slime's
hue in `cell_mat_c` and a plain mini-boss's skin). Swarmer `SWARM_HUE`
(246, 212, 74), a pale lemon; Sporer `SPORE_HUE` (238, 132, 108), coral;
Grudge `GRUDGE_HUE` (196, 118, 74), copper. The engine's tints, the
starving ash, the alarm's flush and the condensed bronze all stay on top.

Body median in grey (two boxes off the heart, frame 1:00): planet 4 126
to 137, planet 5 102 to 100, planet 7 124 to 107. Under fire (greedy hand,
0:30) the alarm's orange flush round the squad, gold on gold before,
reads on the lemon at once (`p4-alarm.jpg`).

**Tried and dropped** for the Grudge (same run, same frame): a rust
(220, 112, 46), which the gold's saturation push turned into the alarm's
own orange; a darker copper (168, 92, 58), median 93, a quarter down in
grey.

Cost: a four-way switch a cell in place of a constant.
