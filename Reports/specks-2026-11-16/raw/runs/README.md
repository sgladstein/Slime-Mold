# Raw readouts

larder-world's headless readout, frame paths left out. `squads` hand,
`mode=squads`, llvmpipe. "N breaking away" counts live walkers with the
breakaway bit (on their way to a crystal). The builds:

| Files | Build | Keys |
|---|---|---|
| `q-def`, `q-s10`, `q-p1` | first build: ripeness kept every step, chances phased by ripeness, no give-up, the run's share 0.5 | `q-s10` adds `roam_share=0.1` |
| `q-off`, `long-off`, `long1-off` | any build (`roam=0`: no specks, no bronze) | `roam=0` |
| `t-s50`, `t-s10`, `t-s05`, `t-p1s05` | second build: as the first, plus the give-up after `SPECK_TRIP` | `roam_share` 0.5 (the run's then), 0.1, 0.05, 0.05 |
| `long-on`, `long1-on` | second build, the run's share 0.05 | none |
| `v2-p2`, `v2-p1` | final build: chances on the slot's step, the crowd counted only then | none (the run's keys) |
| `film-on`, `film-off` | final build, the page's films (`raw/film.sh`, planet 1, every second to 4:00) | none, and `roam=0` |

`q-*`, `t-*`: `AT=10,20,30,40,50,60,75,90,105,120`; `long*`:
`at=60,120,180,240,300`; `v2-*`: both lists. Planet 2 is `run=2:2`,
planet 1 `run=1:1`.
