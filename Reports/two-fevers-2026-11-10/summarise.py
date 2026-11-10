#!/usr/bin/env python3
"""Summarise a `fevers census` output: per seed, a's part of the summed score
averaged over its two games (once as Ember, once as Tide); seeds a won; a
one-sided sign test; and how often the side ahead at 1:30 still led at the
bell. Usage: summarise.py raw/plan-v-random.txt [...]"""
import sys
from math import comb


def sign_p(k, n):
    """One-sided: chance of k or more wins of n at even odds."""
    return sum(comb(n, i) for i in range(k, n + 1)) / 2 ** n


for path in sys.argv[1:]:
    seeds, held, games = {}, 0, 0
    for line in open(path):
        f = line.split()
        if len(f) < 8 or not f[0].isdigit():
            continue
        seed, part, lead = int(f[0]), float(f[4]), f[5]
        seeds.setdefault(seed, []).append(part)
        games += 1
        held += (lead == "a") == (part > 0.5)
    if not seeds:
        print(f"{path}: no games yet")
        continue
    per = {s: sum(v) / len(v) for s, v in seeds.items()}
    won = sum(p > 0.5 for p in per.values())
    vals = sorted(per.values())
    n = len(vals)
    print(f"{path}: {n} seeds, {games} games; a's part median {vals[n // 2]:.3f} "
          f"(range {vals[0]:.3f}-{vals[-1]:.3f}); a won {won}/{n} seeds, sign p {sign_p(won, n):.2g}; "
          f"leader at 1:30 still led at the bell in {held}/{games} games")
