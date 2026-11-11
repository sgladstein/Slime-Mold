# Open-source slime-mould code beyond the first seven (2026-11-11)

The owner asked, 03:00Z, for a wider search for open-source slime-mould
simulations. A research helper searched GitHub and the paper indexes on
2026-11-11 and read the kernel or shader code of every repo marked "code
read" (shallow clones in a scratch folder, not kept here). This is its
report, lightly edited. Already covered elsewhere and left out: Jenson,
Bleuje, Lague, PolyPhy/MCPM, gpu-io, Barradeau, nicoptere, cherouvim, Jones
2010-2015, Tero 2010 (`research/sources/`).

Licences: only MIT, BSD, Apache or public-domain code may be reused (with
its notice); GPL, CC-NC and unlicensed code are ideas only.

## Sources

| # | Source | Licence | Language | What is different in its rule |
|---|---|---|---|---|
| 1 | Primordia, github.com/skulitom/primordia (code read) | MIT | Rust/wgpu WGSL | Fixed per-agent size 2^(spread x u), u in [-1,1] hashed from the index, scaling sensor reach and stride; soft crowding on deposit n/(1+n/c), c = 8; sensing saturation t/(1+t/K); a signed 4x4 species matrix (sense = dot(trail_rgba, row_s)); a cyclic preset (self 1.0, next +0.6, previous -1.0); constant curl turn; a fertility terrain scaling decay and size; swirl advection |
| 2 | Primordia "Symbiosis", same repo (code read) | MIT | WGSL | Agents coupled to Gray-Scott chemistry: in "Graze" agents sense T(1-1.5k) + 3k V/(0.12+V), and their trail raises V's kill rate by 0.004s + 0.012s^2, s = kT/(1+T); a slow soil layer worn by traffic |
| 3 | slime-mould-computer, github.com/serhiileniv/slime-mould-computer (code read) | MIT | WebGL2 | Geodesic food field F = fall x max(F over 3x3), zero on walls, fall = 0.05^(1/maxdim), quarter resolution; agents sense trail + 40.5 ln F; starvation decay trail x mix(0.86, 1, smoothstep(0.02, 0.30, F)); "explore" and "optimise" parameter sets |
| 4 | Jones and Adamatzky 2012, arxiv.org/abs/1212.0023 | paper | | One agent a cell; a blocked agent keeps its heading, does not deposit and retries; pID 0.01; SA 90, RA 45, SO 9-15, damping 0.07. Density surges, travelling waves, blob migration |
| 5 | Mind's Crawl, github.com/physarumAdv/minds_crawl (code read) | MIT | CUDA | Jones exclusion with an oscillation flag; agents divide if the 9x9 count is 0-10 and die unless the 5x5 count is 0-24, every 5 steps |
| 6 | physarum-slime-mold, github.com/KarstenSiemer/physarum-slime-mold (code read) | MIT | WebGPU | Race-free GPU exclusion: claim pass with atomicMax(claims[target], hash(id, frame)), settle pass moves only the winner into an empty cell; losers take a random heading |
| 7 | Reginato, Proverbio and Giordano 2024, arxiv.org/abs/2412.19790 (GAMA code on Zenodo, not reached) | not checked | GAMA | Two states. Explorers steer to *lower* concentration with +/- pi/8 jitter and count steps without food; at 100 + N(0,10) they become networkers that follow the trail and deposit 1 (evaporation 0.7); death at 250 or over 18 neighbours; duplication on food |
| 8 | inconvergent/hyphae (code read) | MIT | Python | Self-avoiding tip growth: a second branch from a node gets radius x 0.92 and generation +1; jitter (1-(g+1)^-0.1) N(0,1) pi; a node is rejected if 2d <= R_i + r. Trunk-to-capillary taper |
| 9 | Physarum-experiments 7-15, github.com/Domenicobrz/Physarum-experiments (code read) | MIT | three.js | Heading += 0.01 x fluid velocity, fluid dye into the trail; decay exp(-0.017 x blurred sum); per-agent sensor length from sin(index) |
| 10 | Defei-Wang/physarum v1-v7 (README and code) | none: ideas only | Taichi/JS | Explorers deposit 0.05 and turn returners (2.5) after touching food; two channels; depleted food repels (-15); a constant subtracted in decay; a sinusoid on deposit; branching probability P0 a^g |
| 11 | agnescameron/slime-moulds (Gunji vacant-particle CA, code read) | none: ideas only | C++ | A mass-conserving body: an empty "bubble" enters at the edge nearest food and random-walks through the body |
| 12 | Kay et al. 2022, Sci Rep (PMC8789834) | code on request | | Food relay: a food site attracts until n/10 agents reach it, then spawns a new population of n there |
| 13 | fogleman/physarum (code read) | MIT | Go | Random signed attraction table between species: diagonal N(1, 0.25), off-diagonal N(-1, 0.25); optional turn in proportion to sensor gaps |
| 14 | AntSim_V2, github.com/nilsgollub/AntSim_V2 (code read) | MIT | TS | RTS-style reference: four pheromones, quality-weighted deposit, rain washes trails, rival colony raids |

MoldLab (github.com/qsters/moldlab, Apache-2.0) is a shipped game but its
step is plain Lague. Xenomycelium has no licence and adds a Gray-Scott term
to the sensor.

## Rules ranked for play by the helper

- **A. Cyclic species** (1): three trail channels, own +1, next +0.6,
  previous -1; SA 20, SO 32, RA 20, step 2, keep 0.9. Expected: ribbons
  chasing each other, never settling. **Built** (`BigParams::species_cycle`).
- **B. Blocked walkers keep their heading** (4, 5, 6): travelling density
  waves and crawling blobs at one agent a cell. **Built**
  (`blocked_keep`, `blocked_reset`).
- **C. Explorer to networker** (7, 10): a self-avoiding front, veins behind
  it. **Built** without food or death (`explore_steps`,
  `explore_deposit`).
- **D. Geodesic food pull and starvation decay** (3): pruning to arteries
  that bend round walls. Not built (needs food on the big dish).
- **E. Per-agent size spread with soft crowding** (1): trunks and
  capillaries from one species. **Built** without the crowding
  (`size_spread`).
- **F. Grazing a regrowing chemical** (2): crescents and spirals over a
  living resource map. Not built (needs a Gray-Scott field).

Sources said to reproduce feathery fronts: hyphae (tip growth with
collision), source 7's explorers, Defei v2-v3 (branching, space
colonisation), Primordia's Dendrites. Trunk hierarchy: Primordia, hyphae,
slime-mould-computer's starvation pruning. Travelling waves made by the
simulation: Jones and Adamatzky's exclusion, Primordia's cyclic species and
grazing; the waves in slime-mould-computer and Defei v3.1 are painted in the
display shader, which the owner's rule excludes.
