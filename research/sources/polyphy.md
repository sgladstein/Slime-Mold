---
id: polyphy
title: PolyPhy
authors: PolyPhyHub (polyphy.io)
year: about 2022 onward
url: https://github.com/PolyPhyHub/PolyPhy
read: kernel/common.py (agent step, relaxation, drawing) and core defaults, 2026-11-10
copy: none
licence: MIT (code may be reused with its notice)
used-in: ../../Reports/jenson-compare-2026-11-10.md, ../../Reports/jenson-physarum-2026-11-10.md, ../../Reports/gpu-plan-2026-11-10.md (phase 3 reading)
---

# PolyPhy

MCPM in Python on Taichi (GPU via CUDA, Vulkan or Metal; CPU fallback).
Parameters: sensing distance, sensing angle, sampling exponent, step size,
data deposit, agent deposit, deposit and trace attenuation.

## Key facts for us (code, 2026-11-10)

- Agent step: a second direction at heading +/- the sensing angle
  (default 1.5 rad); compare the deposit straight ahead with it, and turn
  part way (steering rate 0.5) either when it is higher or, in the
  probabilistic mode, by chance weighted by deposit^exponent (default 2).
- Sensing distance and step may be scaled each step by a random factor
  (exponential or Maxwell-Boltzmann); defaults 0.005 and 0.0005 of the
  domain (ratio 10, as ours).
- Two fields: the deposit (keep 0.9, 3x3 diffusion) guides agents; the
  trace (keep 0.96, jittered to avoid quantisation) records where they went.
  Both are drawn, the trace through a log curve and gamma 1/2.2.
- No one-a-cell limit; edges wrap or reset agents.

The only reference implementation here whose code we may reuse.
