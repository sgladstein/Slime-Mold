#!/usr/bin/env python3
"""Builds page.html, the findings page of the strategy-bot league, from the
results Scott's PC pushed to pc/ (planet 3 only) and his own planet 3 log
(scott-planet3.csv). Run from anywhere: python3 page.py. Every number on the
page comes from these files; rerun it when new results land."""
import glob
import html
import json
import math
import os
import sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import league as L  # noqa: E402

SCOTT_SEED = 1795038793
SEEDS = [1, 2, 3, 4, 5, 6, 7, 8, SCOTT_SEED]
SEEDS17 = list(range(1, 17)) + [SCOTT_SEED]
# The game changed twice on 20 November, so a result belongs to the game it
# was played on, named by the `code` stamp (the src tree the worker built).
# "old": the 19 November game. "set": shots stop at set slime (5be6b0b,
# merged into the branch at 01:45Z). "now": Needle Blast bursts once per
# bullet (#532, merged 05:02Z) and any later code. The page's older
# sections show the old game only; mixing games made one plan's row a
# patchwork of two games (the first page after the fixes did).
OLD_CODES = {None, "9a4b83191084", "3052b1da7102", "68f057590a7b", "de884269a412"}
SET_CODES = {"49cc46f036ba"}


def era(r):
    c = r.get("code")
    return "old" if c in OLD_CODES else "set" if c in SET_CODES else "now"

FAMILIES = ["bot", "rush", "econ", "turtle", "scott", "lure", "boom", "taker"]
FAMILY_TEXT = {
    "bot": "The run bot as it plays today: drains crystals, buys soldiers, hunts the heaviest slime.",
    "rush": "Every point into soldiers and straight at the slime from the first second.",
    "econ": "Four pumps and their upgrades first, a small squad until the income is up, out at 40 soldiers.",
    "turtle": "Pumps behind turrets, walls and a lure; the squad stays home for a long time.",
    "scott": "Your planet 3 build: a pump, four turrets round it, walls, a second pump, out at 40.",
    "lure": "A kill zone of four turrets and a lure, no walls.",
    "boom": "Two pumps, no defences, a big squad early that takes crystals.",
    "taker": "The run bot, but it builds pumps on the crystals it takes instead of draining them.",
}
# The bots drawn against Scott on his planet: the best overall, the same
# plan without its deck, and the one that got furthest without a deck.
PICK = [
    ("econ,go=4,aim=6,cards=0,want=seek/wide/blast.needle", "Best bot, with the deck"),
    ("econ,go=4,aim=6,cards=0", "Same bot, no deck"),
    ("econ,cards=0", "Furthest without a deck"),
]
# Paired tests: (question, [(base plan, changed plan), ...]).
TESTS = [
    ("Aim 6 times a second instead of once", [("econ", "econ,aim=6"), ("rush", "rush,aim=6")]),
    ("Leave home at 4 soldiers", [("econ,aim=6", "econ,go=4,aim=6"), ("lure,aim=6", "lure,go=4,aim=6"), ("scott", "scott,go=4")]),
    ("Kite: back away from close slime", [("rush,aim=6", "rush,aim=6,kite=0.3"), ("rush,aim=6", "rush,aim=6,kite=0.5")]),
    ("Skip the card the planet asks for (Tinder)", [("econ", "econ,cards=0"), ("rush", "rush,cards=0"), ("scott", "scott,cards=0")]),
    ("Hold crystals with split groups", [("econ,go=4,aim=6", "econ,go=4,aim=6,split=2"), ("econ,go=4,aim=6", "econ,go=4,aim=6,split=4"), ("lure,go=4,aim=6", "lure,go=4,aim=6,split=2"), ("rush,aim=6", "rush,aim=6,split=2")]),
    ("Push gun (Heave)", [("econ,go=4,aim=6", "econ,go=4,aim=6,want=heave"), ("lure,go=4,aim=6", "lure,go=4,aim=6,want=heave"), ("rush,aim=6", "rush,aim=6,want=heave")]),
    ("Split groups with Heave", [("econ,go=4,aim=6", "econ,go=4,aim=6,split=2,want=heave"), ("econ,go=4,aim=6", "econ,go=4,aim=6,split=4,want=heave"), ("lure,go=4,aim=6", "lure,go=4,aim=6,split=4,want=heave")]),
    ("The gun lab's damage decks", [("econ,go=4,aim=6,cards=0", "econ,go=4,aim=6,cards=0,want=blast.needle"), ("econ,go=4,aim=6,cards=0", "econ,go=4,aim=6,cards=0,want=seek/wide/blast.needle"), ("econ,go=4,aim=6,cards=0", "econ,go=4,aim=6,cards=0,want=wide/rate/blast.needle"), ("lure,go=4,aim=6,cards=0", "lure,go=4,aim=6,cards=0,want=seek/wide/blast.needle")]),
    ("Mop up the last 300 walkers", [("econ,cards=0", "econ,cards=0,mop=300"), ("econ,go=4,aim=6", "econ,go=4,aim=6,mop=300"), ("scott", "scott,mop=300"), ("turtle,go=4,aim=6", "turtle,go=4,aim=6,mop=300")]),
]


def rows():
    out = []
    for f in sorted(glob.glob(os.path.join(HERE, "pc", "0*.jsonl"))) + sorted(glob.glob(os.path.join(HERE, "pc", "p3", "gen*.jsonl"))) + sorted(glob.glob(os.path.join(HERE, "pc", "p3n", "gen*.jsonl"))):
        for line in open(f):
            try:
                r = json.loads(line)
            except ValueError:
                continue
            if "error" in r or int(r["depth"]) != 3:
                continue
            out.append(r)
    return out


def table(rs):
    """canon plan -> seed -> its latest game; canon plan -> shortest spelling."""
    by, names = defaultdict(dict), {}
    for r in rs:
        c = L.canon(r["plan"])
        by[c][int(r["seed"])] = r
        if c not in names or len(r["plan"]) < len(names[c]):
            names[c] = r["plan"]
    return by, names


def mmss(s):
    s = int(round(s))
    return f"{s // 60}:{s % 60:02d}"


def sign_p(a, b):
    n, k = a + b, max(a, b)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k, n + 1)) / 2**n) if n else 1.0


def cleared(r):
    return max(0.0, 1.0 - r["mould"] / max(1.0, r["peak_mould"]))


def esc(s):
    return html.escape(str(s), quote=True)


def cell(r):
    if r is None:
        return '<td class="c none" aria-label="not played">·</td>'
    tip = f'{r["how"]} at {mmss(r["secs"])} · slime left {int(r["mould"]):,} of a peak {int(r["peak_mould"]):,} · killed {int(r["killed"]):,} · soldiers alive {int(r["alive"])}, bought {int(r["bought"])} · cards {r.get("shop", "")}'
    if r["how"] == "won":
        s = r["secs"]
        k = "w1" if s < 420 else "w2" if s < 540 else "w3"
        return f'<td class="c {k}" data-tip="{esc(tip)}"><b>{mmss(s)}</b></td>'
    pct = cleared(r)
    lab = f"✕ {mmss(r['secs'])}" if r["how"] == "lost" else f"{pct * 100:.0f}%"
    cls = "lost" if r["how"] == "lost" else "open"
    return f'<td class="c {cls}" data-tip="{esc(tip)}"><span>{lab}</span><i style="width:{pct * 100:.0f}%"></i></td>'


def grid(by, names, plans, caption, seeds=SEEDS):
    head = "".join(f"<th>{s}</th>" if s != SCOTT_SEED else '<th class="you">yours</th>' for s in seeds)
    body = []
    for c in plans:
        g = by[c]
        ss = [s for s in seeds if s in g]
        w = [g[s] for s in ss if g[s]["how"] == "won"]
        sc = sum(L.score(g[s]) for s in ss) / max(1, len(ss))
        cells = "".join(cell(g.get(s)) for s in seeds)
        body.append(f'<tr><th class="plan"><code>{esc(names[c])}</code></th>{cells}<td class="num">{len(w)}/{len(ss)}</td><td class="num">{sc:.2f}</td></tr>')
    return f'<div class="scroll"><table class="grid"><caption>{caption}</caption><thead><tr><th class="plan">plan</th>{head}<th>wins</th><th>score</th></tr></thead><tbody>{"".join(body)}</tbody></table></div>'


def seed_row(by):
    cells = []
    for s in SEEDS:
        played = [c for c in by if s in by[c]]
        won = [by[c][s] for c in played if by[c][s]["how"] == "won"]
        cells.append((s, len(won), len(played), min((r["secs"] for r in won), default=None)))
    return cells


def tests(by):
    out = []
    for q, pairs in TESTS:
        items = []
        for a, b in pairs:
            ca, cb = L.canon(a), L.canon(b)
            if ca not in by or cb not in by:
                continue
            common = [s for s in SEEDS if s in by[ca] and s in by[cb]]
            if len(common) < 5:
                continue
            better = sum(L.score(by[cb][s]) > L.score(by[ca][s]) + 1e-9 for s in common)
            worse = sum(L.score(by[ca][s]) > L.score(by[cb][s]) + 1e-9 for s in common)
            wa = sum(by[ca][s]["how"] == "won" for s in common)
            wb = sum(by[cb][s]["how"] == "won" for s in common)
            items.append((a, b, better, worse, len(common) - better - worse, wa, wb, len(common), sign_p(better, worse)))
        out.append((q, items))
    return out


def tests_html(ts, ncomp):
    rows_ = []
    for q, items in ts:
        if not items:
            rows_.append(f'<tr class="q"><th colspan="4">{esc(q)}</th></tr><tr><td colspan="4" class="pending">Playing on Scott\'s PC now.</td></tr>')
            continue
        rows_.append(f'<tr class="q"><th colspan="4">{esc(q)}</th></tr>')
        for a, b, better, worse, tie, wa, wb, n, p in items:
            sig = "sig" if p * ncomp <= 0.05 else ("hint" if p <= 0.1 else "")
            bar = (
                f'<div class="div" role="img" aria-label="better on {better}, worse on {worse}, same on {tie} of {n} seeds">'
                f'<span class="neg" style="width:{worse / n * 100:.1f}%"></span><span class="mid"></span><span class="pos" style="width:{better / n * 100:.1f}%"></span></div>'
            )
            change = b[len(a) + 1:] if b.startswith(a + ",") else b
            rows_.append(
                f'<tr><td class="pair"><code>{esc(a)}</code> <span class="plus">+ <code>{esc(change)}</code></span></td>'
                f'<td class="bar">{bar}</td><td class="num">{better}↑ {worse}↓{f" {tie}=" if tie else ""}</td>'
                f'<td class="num {sig}">wins {wa}→{wb} · p {p:.2f}</td></tr>'
            )
    return f'<div class="scroll"><table class="tests"><thead><tr><th>change</th><th><span class="lg neg"></span>worse · better<span class="lg pos"></span></th><th>seeds</th><th>wins, sign test</th></tr></thead><tbody>{"".join(rows_)}</tbody></table></div>'


DECKS = ["blast.needle", "seek/wide/blast.needle", "wide/rate/blast.needle"]
DECK_NAMES = {"blast.needle": "Needle Blast alone", "seek/wide/blast.needle": "Seek + Wide + Needle Blast", "wide/rate/blast.needle": "Wide + Rate + Needle Blast"}
DECK_BASES = ["econ,go=4,aim=6,cards=0", "lure,go=4,aim=6,cards=0"]


def wins(g):
    return [g[s] for s in SEEDS if s in g and g[s]["how"] == "won"]


def on_yours(r):
    if r is None:
        return "-"
    if r["how"] == "won":
        return f"won in {mmss(r['secs'])}"
    if r["how"] == "lost":
        return f"wiped at {mmss(r['secs'])}, {cleared(r) * 100:.0f}% cleared"
    return f"time ran out, {int(r['mould']):,} walkers left"


def decks(old, now):
    """The gun lab's decks before the two fixes and now: wins of the nine
    maps, and now against the same plan with no cards, map by map."""
    out, ncomp = [], 0
    for base in DECK_BASES:
        cb = L.canon(base)
        if cb not in now:
            continue
        b_now, b_old = now[cb], old.get(cb, {})
        out.append(f'<tr class="q"><th colspan="6"><code>{esc(base)}</code></th></tr>')
        out.append(f'<tr><th>No cards</th><td class="num">{len(wins(b_old))}</td><td class="num">{len(wins(b_now))}</td><td class="num">-</td><td class="num">{mmss(sum(r["secs"] for r in wins(b_now)) / len(wins(b_now))) if wins(b_now) else "-"}</td><td>{on_yours(b_now.get(SCOTT_SEED))}</td></tr>')
        for d in DECKS:
            c = L.canon(base + ",want=" + d)
            if c not in now:
                continue
            g, go = now[c], old.get(c, {})
            common = [s for s in SEEDS if s in g and s in b_now]
            better = sum(L.score(g[s]) > L.score(b_now[s]) + 1e-9 for s in common)
            worse = sum(L.score(b_now[s]) > L.score(g[s]) + 1e-9 for s in common)
            ncomp += 1
            p = sign_p(better, worse)
            out.append((better, worse, p, f'<tr><th>{DECK_NAMES[d]}</th><td class="num">{len(wins(go))}</td><td class="num">{len(wins(g))}</td><td class="num {{SIG}}">{better}↑ {worse}↓ · p {p:.3f}</td><td class="num">{mmss(sum(r["secs"] for r in wins(g)) / len(wins(g))) if wins(g) else "-"}</td><td>{on_yours(g.get(SCOTT_SEED))}</td></tr>'))
    rows_ = []
    for x in out:
        if isinstance(x, tuple):
            better, worse, p, row = x
            rows_.append(row.replace("{SIG}", "sig" if p * ncomp <= 0.05 else ("hint" if p <= 0.1 else "")))
        else:
            rows_.append(x)
    return (
        '<div class="scroll"><table class="tests plain"><caption>Wins of the nine maps, before the two fixes and now. The arrows count the maps where the deck did better or worse than no cards, on the game as it is now. '
        f'Bold survives a correction for the {ncomp} comparisons in this table (p below 0.05/{ncomp}).</caption>'
        '<thead><tr><th>deck</th><th class="num">19 Nov</th><th class="num">now</th><th class="num">against no cards</th><th class="num">mean win</th><th>your planet</th></tr></thead>'
        f'<tbody>{"".join(rows_)}</tbody></table></div>'
    )


def curves(by, names):
    """Scott's slime and the closest bots' on his planet 3, as SVG."""
    scott = []
    for line in open(os.path.join(HERE, "scott-planet3.csv")):
        if line[0].isdigit():
            t, m = line.split(",")[:2]
            scott.append((int(t), int(m)))
    on = [(c, by[c][SCOTT_SEED]) for c in by if SCOTT_SEED in by[c]]

    def marks(r):
        pts = []
        for mk in r["marks"].split(","):
            t, v = mk.split(":")
            pts.append((int(t), int(v.split("/")[1])))
        if r["how"] != "open":
            pts.append((r["secs"], r["mould"]))
        return pts

    best_all = max((c for c in by if all(s in by[c] for s in SEEDS)), key=lambda c: sum(L.score(by[c][s]) for s in SEEDS))
    closest = sorted(on, key=lambda x: (x[1]["mould"] if x[1]["how"] != "lost" else 1e9))
    pick = [(L.canon(p), lab) for p, lab in PICK if L.canon(p) in by and SCOTT_SEED in by[L.canon(p)]]
    pick = pick or [(c, names[c]) for c in [best_all] + [c for c, _ in closest if c != best_all][:2]]
    series = [("You", scott, "won at 7:21")]
    for c, lab in pick:
        r = by[c][SCOTT_SEED]
        end = f"won at {mmss(r['secs'])}" if r["how"] == "won" else (f"wiped at {mmss(r['secs'])}" if r["how"] == "lost" else f"{int(r['mould']):,} left at 12:00")
        series.append((lab, marks(r), end))
    W, H, ml, mr, mt, mb = 800, 340, 56, 190, 16, 40
    ymax = max(v for _, pts, _ in series for _, v in pts)
    ymax = math.ceil(ymax / 20000) * 20000
    X = lambda t: ml + (W - ml - mr) * t / 720  # noqa: E731
    Y = lambda v: mt + (H - mt - mb) * (1 - v / ymax)  # noqa: E731
    g = []
    for v in range(0, ymax + 1, 20000):
        g.append(f'<line class="grid" x1="{ml}" x2="{W - mr}" y1="{Y(v):.1f}" y2="{Y(v):.1f}"/><text class="tick" x="{ml - 8}" y="{Y(v) + 4:.1f}" text-anchor="end">{v // 1000}k</text>')
    for m in range(0, 13, 2):
        g.append(f'<text class="tick" x="{X(m * 60):.1f}" y="{H - mb + 18}" text-anchor="middle">{m}:00</text>')
    g.append(f'<text class="tick" x="{ml}" y="{H - 4}">minutes into planet 3</text>')
    lines, labels, data = [], [], []
    ends = []
    for i, (name, pts, end) in enumerate(series):
        d = " ".join(f"{'M' if k == 0 else 'L'}{X(t):.1f},{Y(v):.1f}" for k, (t, v) in enumerate(pts))
        lines.append(f'<path class="s s{i + 1}" d="{d}"/>')
        t, v = pts[-1]
        lines.append(f'<circle class="dot s{i + 1}" cx="{X(t):.1f}" cy="{Y(v):.1f}" r="4"/>')
        ends.append([Y(v), i, name, end, X(t)])
        data.append({"name": name, "end": end, "pts": pts})
    ends.sort()
    last = -99
    for e in ends:
        e[0] = max(e[0], last + 32)
        last = e[0]
    floor = H - mb - 10
    for e in reversed(ends):
        e[0] = min(e[0], floor)
        floor = e[0] - 32
    for y, i, name, end, x in ends:
        labels.append(f'<text class="lab" x="{W - mr + 10}" y="{y:.1f}"><tspan class="sw s{i + 1}">■</tspan> {esc(name)}</text><text class="lab sub" x="{W - mr + 22}" y="{y + 14:.1f}">{esc(end)}</text>')
    early = early_table(scott_rows(), [(lab, names[c], by[c][SCOTT_SEED]) for c, lab in pick])
    svg = (
        f'<svg id="curves" viewBox="0 0 {W} {H}" role="img" aria-label="Slime on Scott\'s planet 3 over time: Scott and the bots">'
        + "".join(g) + "".join(lines) + "".join(labels)
        + f'<line id="hair" class="hair" x1="0" x2="0" y1="{mt}" y2="{H - mb}" visibility="hidden"/><rect id="hit" x="{ml}" y="{mt}" width="{W - ml - mr}" height="{H - mt - mb}" fill="transparent"/></svg>'
    )
    geo = {"W": W, "ml": ml, "mr": mr, "span": 720}
    return svg, json.dumps({"series": data, "geo": geo}), early


def scott_rows():
    out = []
    for line in open(os.path.join(HERE, "scott-planet3.csv")):
        if line[0].isdigit():
            t, m, sol, killed = line.split(",")[:4]
            out.append((int(t), int(m), int(sol), int(killed)))
    return out


def early_table(scott, bots):
    """Killed by 2:00 and 4:00, slime and soldiers at 4:00, on Scott's planet."""
    def at(pts, t):
        return max((p for p in pts if p[0] <= t), key=lambda p: p[0])
    rows_ = []
    s2, s4 = at(scott, 121), at(scott, 241)
    rows_.append(("You", s2[3], s4[3], s4[1], s4[2], "won at 7:21"))
    for lab, name, r in bots:
        pts = []
        for mk in r["marks"].split(","):
            t, v = mk.split(":")
            k, m, a, _ = (int(x) for x in v.split("/"))
            pts.append((int(t), m, a, k))
        b2, b4 = at(pts, 120), at(pts, 240)
        end = f"won at {mmss(r['secs'])}" if r["how"] == "won" else (f"wiped at {mmss(r['secs'])}" if r["how"] == "lost" else f"{int(r['mould']):,} left at 12:00")
        rows_.append((f"{esc(lab)}<br><code>{esc(name)}</code>", b2[3], b4[3], b4[1], b4[2], end))
    body = "".join(f"<tr><th>{n}</th><td class=\"num\">{k2:,}</td><td class=\"num\">{k4:,}</td><td class=\"num\">{m4:,}</td><td class=\"num\">{a4}</td><td>{e}</td></tr>" for n, k2, k4, m4, a4, e in rows_)
    return f'<div class="scroll"><table class="plain"><thead><tr><th></th><th>killed by 2:00</th><th>killed by 4:00</th><th>slime at 4:00</th><th>soldiers at 4:00</th><th>end</th></tr></thead><tbody>{body}</tbody></table></div>'


def main():
    every = rows()
    rs = [r for r in every if era(r) == "old"]
    by, names = table(rs)
    by_set, names_set = table([r for r in every if era(r) == "set"])
    now_rs = [r for r in every if era(r) == "now"]
    by_now, names_now = table(now_rs)
    full = [c for c in by if all(s in by[c] for s in SEEDS)]
    score = {c: sum(L.score(by[c][s]) for s in SEEDS) / len(SEEDS) for c in full}
    ranked = sorted(full, key=lambda c: -score[c])
    fams = [L.canon(f) for f in FAMILIES if L.canon(f) in by]
    best = [c for c in ranked if c not in fams][:12]
    ts = tests(by)
    ncomp = sum(len(i) for _, i in ts)
    svg, curve_json, early = curves(by, names)
    seeds = seed_row(by)
    top = ranked[0]
    tw = [by[top][s] for s in SEEDS if by[top][s]["how"] == "won"]
    near = min((by[c][SCOTT_SEED] for c in by if SCOTT_SEED in by[c] and by[c][SCOTT_SEED]["how"] == "open"), key=lambda r: r["mould"])
    games = len(every)
    mine = by[top].get(SCOTT_SEED)
    yours = mmss(mine["secs"]) if mine and mine["how"] == "won" else "not yet"
    base = L.canon(",".join(x for x in names[top].split(",") if not x.startswith("want=")))
    base_w = sum(by[base][s]["how"] == "won" for s in SEEDS if s in by.get(base, {})) if base in by else None
    yours_won = sum(1 for r in every if int(r["seed"]) == SCOTT_SEED and r["how"] == "won")
    yours_n = sum(1 for r in every if int(r["seed"]) == SCOTT_SEED)
    won_any = sum(1 for r in every if r["how"] == "won")
    plans_all = len({L.canon(r["plan"]) for r in every})
    # The game with set-slime shots: the best plans without a Needle deck on
    # its 17 maps, and the first bot win on Scott's planet without one.
    full_set = [c for c in by_set if all(s in by_set[c] for s in SEEDS17)]
    needle = lambda c: "needle" in dict(c[1]).get("want", "")  # noqa: E731
    nodeck = [c for c in full_set if not needle(c)]
    set_nd = max((sum(by_set[c][s]["how"] == "won" for s in SEEDS17) for c in nodeck), default=0)
    set_won = sorted((by_set[c][SCOTT_SEED]["secs"], c) for c in by_set if not needle(c) and SCOTT_SEED in by_set[c] and by_set[c][SCOTT_SEED]["how"] == "won")
    set_yours = set_won[0][0] if set_won else None
    set_yours_plan = names_set[set_won[0][1]] if set_won else "-"
    set_deck = max((sum(by_set[c][s]["how"] == "won" for s in SEEDS17) for c in full_set if needle(c)), default=0)
    # The game now: the decks, and the search's best plans once they have
    # played all 17 maps.
    seek = by_now.get(L.canon("econ,go=4,aim=6,cards=0,want=seek/wide/blast.needle"), {})
    wrn = by_now.get(L.canon("econ,go=4,aim=6,cards=0,want=wide/rate/blast.needle"), {})
    nob = by_now.get(L.canon("econ,go=4,aim=6,cards=0"), {})
    wrn_better = sum(L.score(wrn[s]) > L.score(nob[s]) + 1e-9 for s in SEEDS if s in wrn and s in nob)
    full_now = [c for c in by_now if all(s in by_now[c] for s in SEEDS17)]
    score17 = {c: sum(L.score(by_now[c][s]) for s in SEEDS17) / len(SEEDS17) for c in full_now}
    best_now = sorted(full_now, key=lambda c: -score17[c])[:10]
    if best_now:
        grid_now = grid(by_now, names_now, best_now, f"The best plans on the game as it is now, by mean score over 17 maps ({len(now_rs):,} games so far)", SEEDS17)
    else:
        grid_now = f'<p class="pending">The search on the game as it is now has played {len(now_rs):,} games; its first plans finish all 17 maps within a few hours.</p>'
    deck_plans = [L.canon(b) for b in DECK_BASES[:1]] + [L.canon(DECK_BASES[0] + ",want=" + d) for d in DECKS]
    deck_grid = grid(by_now, names_now, [c for c in deck_plans if c in by_now], "The econ bot with each deck, map by map, on the game as it is now")
    seed_cells = "".join(
        f'<td class="num{" you" if s == SCOTT_SEED else ""}">{w}/{n}<small>{mmss(f) if f else "never"}</small></td>' for s, w, n, f in seeds
    )
    fam_rows = "".join(f'<tr><th><code>{f}</code></th><td>{esc(FAMILY_TEXT[f])}</td></tr>' for f in FAMILIES)
    tpl = open(os.path.join(HERE, "page-template.html")).read()
    out = (
        tpl.replace("{{GAMES}}", f"{games:,}")
        .replace("{{PLANS}}", str(plans_all))
        .replace("{{WON}}", f"{won_any:,}")
        .replace("{{TOP}}", esc(names[top]))
        .replace("{{TOP_WINS}}", f"{len(tw)} of {len(SEEDS)}")
        .replace("{{TOP_MEAN}}", mmss(sum(r["secs"] for r in tw) / len(tw)) if tw else "-")
        .replace("{{TOP_FAST}}", mmss(min(r["secs"] for r in tw)) if tw else "-")
        .replace("{{TOP_YOURS}}", yours)
        .replace("{{BASE}}", esc(names.get(base, base)))
        .replace("{{BASE_WINS}}", f"{base_w} of {len(SEEDS)}" if base_w is not None else "-")
        .replace("{{YOURS_WON}}", str(yours_won))
        .replace("{{YOURS_N}}", str(yours_n))
        .replace("{{NEAR_PLAN}}", esc(near["plan"]))
        .replace("{{NEAR_LEFT}}", f"{int(near['mould']):,}")
        .replace("{{CURVES}}", svg)
        .replace("{{CURVE_JSON}}", curve_json)
        .replace("{{EARLY}}", early)
        .replace("{{SEED_CELLS}}", seed_cells)
        .replace("{{GRID_BEST}}", grid(by, names, best, "The best plans found so far, by mean score over the nine planets"))
        .replace("{{GRID_FAMILIES}}", grid(by, names, sorted(fams, key=lambda c: -score.get(c, 0)), "The eight starting families as they are"))
        .replace("{{TESTS}}", tests_html(ts, ncomp))
        .replace("{{NCOMP}}", str(ncomp))
        .replace("{{FAMILY_ROWS}}", fam_rows)
        .replace("{{NOW_SEEK}}", str(len(wins(seek))))
        .replace("{{NOW_BASE}}", str(len(wins(nob))))
        .replace("{{NOW_SEEK_MEAN}}", mmss(sum(r["secs"] for r in wins(seek)) / len(wins(seek))) if wins(seek) else "-")
        .replace("{{NOW_SEEK_YOURS}}", on_yours(seek.get(SCOTT_SEED)))
        .replace("{{NOW_WRN}}", str(len(wins(wrn))))
        .replace("{{NOW_WRN_B}}", str(wrn_better))
        .replace("{{SET_ND}}", str(set_nd))
        .replace("{{SET_DECK}}", str(set_deck))
        .replace("{{SET_GAMES}}", f"{sum(len(g) for g in by_set.values()):,}")
        .replace("{{SET_YOURS}}", mmss(set_yours) if set_yours else "-")
        .replace("{{SET_YOURS_PLAN}}", esc(set_yours_plan))
        .replace("{{DECK_TABLE}}", decks(by, by_now))
        .replace("{{DECK_GRID}}", deck_grid)
        .replace("{{GRID_NOW}}", grid_now)
    )
    open(os.path.join(HERE, "page.html"), "w").write(out)
    print(f"page.html: {games} games ({len(now_rs)} on the game now), {plans_all} plans, old-game top {names[top]} {score[top]:.3f}, {ncomp} paired comparisons, {len(full_now)} plans with all 17 maps now")


if __name__ == "__main__":
    main()
