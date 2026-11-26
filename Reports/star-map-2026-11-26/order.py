"""Pairs the bot check's games by step and seed (check.sh) and counts how
often Hard costs more than Fair and Fair more than Safe: soldiers lost
(`fell_end`) and time to win (`secs`, a loss costlier than any win)."""
import glob, os, re, sys
from math import comb
raw = os.path.join(os.path.dirname(os.path.abspath(__file__)), "raw")
games = {}
for f in glob.glob(os.path.join(raw, "*.txt")):
    step, seed, pip = os.path.basename(f)[:-4].split("-")
    line = next((l for l in open(f) if l.startswith("league ")), None)
    if not line:
        continue
    kv = dict(re.findall(r"(\w+)=(\S+)", line))
    won = kv.get("how") == "won"
    games[(int(step), int(seed), pip)] = {"won": won, "secs": float(kv["secs"]), "lost": int(kv["fell_end"]), "tier": kv["tier"], "curve": kv["curve"], "walkers": kv["walkers"]}
def cost_t(g): return (0 if g["won"] else 1, g["secs"] if g["won"] else -g["secs"])
def cost_l(g): return (g["lost"],)
def sign_p(k, n):  # one-sided sign test, ties dropped
    return sum(comb(n, i) for i in range(k, n + 1)) / 2 ** n if n else 1.0
print("step seed | safe | fair | hard  (tier walkers: won/lost secs lost)")
for step in sorted({k[0] for k in games}):
    for seed in sorted({k[1] for k in games if k[0] == step}):
        row = []
        for pip in ("safe", "fair", "hard"):
            g = games.get((step, seed, pip))
            row.append("-" if not g else f'{g["tier"]} {g["walkers"]}: {"W" if g["won"] else "L"} {g["secs"]:.0f}s {g["lost"]}')
        print(f"{step:>4} {seed:>4} | " + " | ".join(row))
for step in sorted({k[0] for k in games}):
    for a, b in (("safe", "fair"), ("fair", "hard"), ("safe", "hard")):
        for name, cost in (("time", cost_t), ("lost", cost_l)):
            pairs = [(games[(step, s, a)], games[(step, s, b)]) for s in range(1, 99) if (step, s, a) in games and (step, s, b) in games]
            up = sum(cost(y) > cost(x) for x, y in pairs)
            down = sum(cost(y) < cost(x) for x, y in pairs)
            print(f"step {step} {a}<{b} {name}: {up} up, {down} down, {len(pairs)-up-down} tied of {len(pairs)}; sign p={sign_p(up, up+down):.3f}")
