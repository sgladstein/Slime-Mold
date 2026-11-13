# Window logs for the panic films (11-13)

`larder-world` on software Vulkan (llvmpipe), Fork, seed 1, farm hand.
Picture paths are left out.

On main after #197 (wading counts every walker) and #198 (a unit aims
only at slime it can see); the page's films. Today is `panic=0`, scared is
shape C's new default (`panic` 2):
- `c8-t.log`, `c8-p.log`: the charge scene, `cut=150,365,240,24
  cutsquad=365,256,150,4 centre=365,238,6`.
- `b8-t.log`, `b8-p.log`: the bolt scene, `cut=150,386,239,22,8,16
  centre=386,232,5`.
- `p5-t.log`, `p5-p.log`: no cut, frames every 10 s from 0:30 to 2:30.
- `s1-t.log`, `s1-p.log`: the last stand (11-13, after #200), `cut=150,365,240,32
  cutsquad=365,264,150,4 centre=365,238,6`: a bigger ring and four soldiers.
- `g1-p.log`: scene 1 with two soldiers (`cutsquad=365,256,150,2`), scared
  only, not filmed for the page.
- `s2-p.log`, `s3-p.log`: the last stand with the lean creeping at half and
  a quarter pace (`panic_creep=0.5`, `0.25`); `s2` is the page's right film.
- `c9-p.log`: scene 1 with `panic_creep=0.5`, not filmed for the page.

Earlier takes of the same: `c7-*`, `b7-*`, `p4-*` after #197 only;
`c6-*`, `b6-*`, `p3-*` (no cut, every 5 s from 0:20 to 2:00) before #197,
where today is the default and scared adds `panic=2`.

`cpu-fork-farm-today.log`, `cpu-fork-farm-scared.log`: the CPU world on
Fork, on main after #198 (`larderworld look=DIR seeds=1 hands=farm every=10 cap=150 map=fork
walkers=25000 upkeep=0.0002 keys=capacity:50000+energy_flow:0.05+wither:0.01[+panic:2]`),
with the scared walkers counted beside each picture.

Frames every 0.5 s (`at=147,147.5,...,180`). `mkfilm.py` made the films
(crop, a time label, 6 frames a second, H.264) from the frames;
`frame.py` names a frame file.
