#!/usr/bin/env python3
"""Rewrite harness `duel` lines (round 7 on) to read the held-ground score:
share, at60 and by60 become held, held60 and hby60, so analyse.py,
screen.py and compare.py measure the points game from the same runs.

Usage: held.py FILE... > OUT
"""
import re
import sys

for f in sys.argv[1:]:
    for line in open(f):
        m = re.search(r"held=([\d.]+) held60=([\d.]+) hby60=([\d.]+)", line)
        if not m:
            continue
        line = re.sub(r"share=[\d.]+", f"share={m[1]}", line, count=1)
        line = re.sub(r"at60=[\d.]+", f"at60={m[2]}", line, count=1)
        line = re.sub(r"by60=[\d.]+", f"by60={m[3]}", line, count=1)
        sys.stdout.write(line)
