---
id: jones-2015-survey
title: Applications of Multi-Agent Slime Mould Computing
authors: Jeff Jones
year: 2015 (preprint; journal 2016)
venue: arXiv 1511.05774; International Journal of Parallel, Emergent and Distributed Systems
url: https://arxiv.org/abs/1511.05774
read: full, 2026-11-09 (Fig. 3 viewed)
copy: ../../Reports/mould-game-2026-11-08/depth-2026-11-09/papers/jones-2015-applications-multi-agent-slime-mould-computing.pdf (+ .txt)
licence: author's and publisher's copyright; private repo, reading only
used-in: ../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md, ../../Reports/gpu-plan-2026-11-10.md
---

# Jones 2015 survey: the model as a "virtual material"

## Key facts for us

- Engulfment: a covered food node stops smelling (p. 10); we already do this
  (`Params::fed_scent`).
- Shrink-wrap (convex hull), inflate-and-halt growth, path planning with
  obstacles that repel only when uncovered (pp. 11-18).
- Voronoi borders between colonies need an explicit mutual-avoidance rule
  (pp. 15-16).
- Sclerotium: drying mould shrinks into a disc at its centroid by random
  walker loss, p = 0.0005 per step (pp. 23-26).
- Computing runs use SA 60 / RA 60 / SO 7 (Fig. 4, p. 8).

Full notes with page refs: `../../Reports/mould-game-2026-11-08/depth-2026-11-09/sources-research.md` section 3b.
