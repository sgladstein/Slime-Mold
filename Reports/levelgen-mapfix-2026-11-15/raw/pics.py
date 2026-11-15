"""Numbers beside each picture on the page: per planet-seed, before and after."""
import sys, runpy
import numpy as np
sys.argv = ["measure.py", "before", "after"]
src = open("measure.py").read().split("\nbefore, after")[0]
ns = {}
exec(src, ns)
for d, s in ((8, 1), (8, 2), (5, 1), (4, 2), (4, 6), (4, 1), (8, 5)):
    for tag in ("before", "after"):
        m = ns["measure"](f"{tag}/planet-{d}-{s}")
        ap = f"{m['apart']:.2f}" if m["apart"] else "-"
        print(f"{d}-{s} {tag:6} nearest edge {min(m['edge']):.0f}  apart {ap}  hedges on water {m['on_water']}/{m['walls']}  water {m['water']:.3f}  width {m['w']}")
