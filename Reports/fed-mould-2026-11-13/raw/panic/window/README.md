# Window logs for the panic films (11-13)

`larder-world` on software Vulkan (llvmpipe), Fork, seed 1, farm hand.
Picture paths are left out.

After merging #197 (wading counts every walker); the page's films. Today
is `panic=0`, scared is shape C's new default (`panic` 2):
- `c7-t.log`, `c7-p.log`: the charge scene, `cut=150,365,240,24
  cutsquad=365,256,150,4 centre=365,238,6`.
- `b7-t.log`, `b7-p.log`: the bolt scene, `cut=150,386,239,22,8,16
  centre=386,232,5`.
- `p4-t.log`, `p4-p.log`: no cut, frames every 10 s from 0:30 to 2:30.

Before #197 (today is the default, scared adds `panic=2`): `c6-*`, `b6-*`
(the same scenes) and `p3-*` (no cut, every 5 s from 0:20 to 2:00).

`cpu-fork-farm-today.log`, `cpu-fork-farm-scared.log`: the CPU world on
Fork (`larderworld look=DIR seeds=1 hands=farm every=10 cap=150 map=fork
walkers=25000 upkeep=0.0002 keys=capacity:50000+energy_flow:0.05+wither:0.01[+panic:2]`),
with the scared walkers counted beside each picture.

Frames every 0.5 s (`at=147,147.5,...,180`). `mkfilm.py` made the films
(crop, a time label, 6 frames a second, H.264) from the frames;
`frame.py` names a frame file.
