#!/usr/bin/env python3
"""Writes a settings file for the build pages' films: a PC whose last round
was on main at COMMIT (the lines of news/whats-new.txt in that commit are
seen, the rest are new), with the things to try at TICKS (1-based) ticked.

  film_settings.py OUT COMMIT DATE [TICKS,...]

The ids are `src/build/news.rs`'s: FNV-1a over the line's words."""
import subprocess, sys

def fnv(line):
    h = 0x811C9DC5
    for b in " ".join(line.split()).encode():
        h = ((h ^ b) * 0x01000193) & 0xFFFFFFFF
    return h

def lines(text):
    return [l.strip() for l in text.splitlines() if l.strip() and not l.strip().startswith("#")]

out, commit, date = sys.argv[1:4]
ticks = [int(t) for t in sys.argv[4].split(",")] if len(sys.argv) > 4 else []
then = subprocess.run(["git", "show", f"{commit}:news/whats-new.txt"], capture_output=True, text=True, check=True).stdout
seen = ",".join(f"{fnv(l):08x}" for l in lines(then))
tries = lines(open("news/things-to-try.txt").read())[:5]
tried = ",".join(f"{fnv(tries[t - 1]):08x}" for t in ticks)
open(out, "w").write(f"last_build=0:{commit}:{date}\nnews_seen={seen}\ntried={tried}\n")
print(f"{len(lines(then))} lines seen, {len(ticks)} ticked")
