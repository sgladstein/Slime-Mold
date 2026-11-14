# The census plays shape C (cloud plan D1, 2026-11-14)

`larderworld census` with no `shape=`, `walkers=` or `scale=` now plays shape
C (the window's mould since #174) at census size: scale 1, 6,250 walkers
(C's walkers per open cell), C's keys (upkeep 0.0002, slots in proportion,
`energy_flow` 0.05, `wither` 0.01, `panic` 2) and four times the crystal
charge a fine cell, since a scale-1 pile holds a quarter of the cells.

**The direction check** (seeds 1-4, idle, farm, strike, cap 300 s, CPU, main
3d31dd3 with rations off):

| | census size, no charge fix | census size (the default) | window size (`shape=c`) |
|---|---|---|---|
| idle | -0.333 on 4/4 | -0.333 on 4/4 | -0.333 on 4/4 |
| strike against farm | farm ahead 4/4 | strike ahead 4/4 | strike ahead 4/4 |
| strike's squad at 5:00 | 14-27 | 30 | 30 |
| banks at 5:00 (farm, strike) | 0.6-2.9k | 14-28k | 16-24k |
| mould at 5:00 | 76-100% of its slots | 78-100% | 100% |
| runner time | 37 s | 37 s | 5 min 28 s |

Without the charge fix census-size C starved the economy and flipped strike
against farm; with it every direction matches. Files: `census-size.txt`,
`window-size.txt`, `census-size-no-charge-fix.txt`.
