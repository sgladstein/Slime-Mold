#!/usr/bin/env python3
"""Rewrite harness `duel` lines (round 7b on, which carry `gone=`) so a
wipe-out is the survivor's game whatever the score: share becomes 1 when
the right mould is gone and 0 when the left one is (`Skirmish::winner`).
Compose with held.py for the held-ground score:

    held.py FILE... | wiped.py /dev/stdin > OUT

Prints, to stderr, how many wiped-out sides had led on the score.

Usage: wiped.py FILE... > OUT
"""
import re
import sys

led = n = 0
for f in sys.argv[1:]:
    for line in open(f):
        m = re.search(r"share=([\d.]+) .* gone=([LR-])", line)
        if not m:
            continue
        if m[2] != "-":
            n += 1
            x = float(m[1])
            led += (m[2] == "L" and x > 0.5) or (m[2] == "R" and x < 0.5)
            line = re.sub(r"share=[\d.]+", "share=" + ("1.000" if m[2] == "R" else "0.000"), line, count=1)
        sys.stdout.write(line)
print(f"wiped.py: {n} wipe-outs; the side wiped out had led on the score in {led}", file=sys.stderr)
