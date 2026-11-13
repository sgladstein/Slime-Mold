# Window logs for the panic films (11-13)

`larder-world` on software Vulkan (llvmpipe), Fork, seed 1, farm hand,
before #197 (wading). Picture paths are left out.

- `c6-t.log`, `c6-p.log`: the charge scene, `cut=150,365,240,24
  cutsquad=365,256,150,4 centre=365,238,6`, today and `panic=2`.
- `b6-t.log`, `b6-p.log`: the bolt scene, `cut=150,386,239,22,8,16
  centre=386,232,5`, today and `panic=2`.
- `p3-t.log`, `p3-p.log`: no cut, frames every 5 s from 0:20 to 2:00,
  today and `panic=2` (what the rule changes before any cut).

Frames every 0.5 s (`at=147,147.5,...,180`). `mkfilm.py` made the films
(crop, a time label, 6 frames a second, H.264) from the frames;
`frame.py` names a frame file.
