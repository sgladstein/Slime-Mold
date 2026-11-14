# Ground refresh timing on software Vulkan (2026-11-14)

`larder-world bench=20 seed=1 map=planet:8:1:AREA` (rush hand), llvmpipe in
the cloud. `old-*`: the whole charge buffer read back and every cell's code
rebuilt once a game second (`old-merged-6x` is the same on main after #285
and #292). `rows-*`: the sparse refresh with a copy for each run of
crystal or wall cells (rejected). `gather-*`: one compute pass and one copy
(kept). `-spread0`: whole steps instead of steps in shares.
