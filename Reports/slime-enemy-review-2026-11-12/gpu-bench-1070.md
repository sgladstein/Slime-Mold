# 1070 bench of density A and B (GPU thread, 2026-11-12 16:47Z)

Build bb08d9e (main 9f5e4a4 + squad speedups; look unchanged). `bench=20 hand=idle map=fork spread=0 passes=30`, 1280x800, 60 fps. PC quiet (2.2-2.3 GB RAM free of 8, GPU 0% at start). Wall ms per frame p50 / p95 / max.

| World | every frame | frames with a step | GPU ticks | draw | passes total |
|---|---|---|---|---|---|
| Today (scale 2, 5,000 walkers; mould 2,704 at 20 s) | 0.59 / 1.35 / 3.19 | 1.29 / 1.40 / 1.98 | 0.39 ms | 0.06 ms | 0.417 |
| A `walkers=25000 upkeep=0.0002` (mould 24,999) | 0.60 / 1.40 / 3.11 | 1.36 / 1.46 / 1.96 | 0.46 ms | 0.06 ms | 0.483 |
| B `scale=1` (256x160; mould 4,801) | 0.58 / 0.90 / 2.68 | 0.85 / 0.94 / 1.76 | 0.11 ms | 0.03 ms | 0.117 |

Per pass today (ms a step): trail 0.132, scent 0.119, sense 0.029, pilesum 0.021, walk 0.016, units 0.014, life 0.011, breed 0.010, eat 0.008, born 0.008, smell 0.007, flow 0.005, harvest 0.005, draw 0.033.
A: trail 0.133, scent 0.119, sense 0.051, walk 0.031, life 0.022, breed 0.021, pilesum 0.021, born 0.011, eat 0.009.
B: trail 0.022, scent 0.021, sense 0.012, units 0.009, draw 0.009, pilesum 0.008, walk 0.007.

Spreading on: today 0.73 / 1.00 / 3.17, A 0.77 / 1.00 / 3.09, B 0.57 / 0.89 / 2.66.

A costs +0.07 ms GPU a step (+16%); invisible in the frame. B saves 0.3 ms. All far under 16.7 ms. Raw text: Reports/gpu-readout-2026-11-12/raw/r3-timing.txt (GPU thread's next PR).
