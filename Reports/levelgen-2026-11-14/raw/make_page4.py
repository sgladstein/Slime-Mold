#!/usr/bin/env python3
"""Writes the core-world arenas page (PAGE/index.html and PAGE/maps/): the
core world (planet 8) of seeds 1-6 before and after the arenas, straight from
the map files in false colour, with each planet's numbers from the planets
example's survey beside it.

  cargo run --release --example planets -- seeds=1-6 depths=8 out=BEFORE   # at the commit before
  cargo run --release --example planets -- seeds=1-6 depths=8 out=AFTER
  python3 make_page4.py PAGE BEFORE AFTER SURVEY_BEFORE SURVEY_AFTER

Colours: ground olive, rock grey, shallow water blue, deep water dark blue,
soft-rock walls (hedges) brown, crystals cyan, nests orange rings, the
landing white.
"""
import html
import re
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

page, before, after = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
surveys = [Path(sys.argv[4]), Path(sys.argv[5])]
SEEDS = range(1, 7)


def view(base, scale=0.5):
    im = np.array(Image.open(f"{base}.png").convert("RGBA"))
    g = im[:, :, 1].astype(int)
    out = np.zeros(g.shape + (3,), np.uint8)
    out[:] = (95, 100, 60)
    out[(g & 1) != 0] = (60, 100, 140)
    out[g == 0x41] = (25, 40, 80)
    out[(g & 8) != 0] = (150, 150, 155)
    pic = Image.fromarray(out)
    d = ImageDraw.Draw(pic)
    for line in open(f"{base}.txt"):
        w = line.split()
        if not w:
            continue
        if w[0] == "wall":
            a, b, c, e, t = map(float, w[1:6])
            d.line([(a, b), (c, e)], fill=(130, 80, 40), width=max(2, int(t)))
        elif w[0] == "crystal":
            x, y, r = map(float, w[1:4])
            d.ellipse([x - r, y - r, x + r, y + r], fill=(60, 220, 230))
        elif w[0] == "mould":
            x, y = map(float, w[1:3])
            d.ellipse([x - 30, y - 30, x + 30, y + 30], outline=(240, 150, 30), width=8)
        elif w[0] == "landing":
            x, y = map(float, w[1:3])
            d.ellipse([x - 20, y - 20, x + 20, y + 20], fill=(255, 255, 255))
    return pic.resize((int(pic.width * scale), int(pic.height * scale)), Image.LANCZOS)


def survey(path):
    rows = {}
    for line in path.read_text().splitlines():
        m = re.match(r"8 (\d+) (\S+) core \[(\S+) (\S+) (\S+) (\S+)\] (\d+) \d+ \| (.*?) \| (.*?) \| (.*?) \|", line)
        if m:
            s, name, o, w, so, pat, att, nums, walk, opn = m.groups()
            b = nums.split()
            rows[int(s)] = {"name": name, "dials": f"{o} · {w} · {so} soil · {pat}", "attempt": int(att), "crystals": b[0], "rock": b[1], "walls": b[6], "open": opn.strip()}
    return rows


old, new = survey(surveys[0]), survey(surveys[1])


def mean(rows, key):
    v = [float(r[key].rstrip("%")) for r in rows.values()]
    return sum(v) / len(v)


(page / "maps").mkdir(parents=True, exist_ok=True)
bands = []
for s in SEEDS:
    for tag, src in (("before", before), ("after", after)):
        view(src / f"planet-8-{s}").save(page / "maps" / f"core-{s}-{tag}.png", optimize=True)
    a, b = old.get(s), new.get(s)
    cells = []
    for tag, r in (("before", a), ("after", b)):
        nums = "".join(f"<div><dt>{k}</dt><dd>{html.escape(v)}</dd></div>" for k, v in (("rock", r["rock"]), ("walls", r["walls"]), ("charge open", r["open"]), ("attempt", str(r["attempt"]))))
        cells.append(f'<figure class="film"><img src="maps/core-{s}-{tag}.png" alt="Core world seed {s}, {tag} the arenas" loading="lazy"><figcaption><span class="tag">{tag}</span><dl class="nums">{nums}</dl></figcaption></figure>')
    bands.append(f'<section class="planet"><div class="phead"><div class="pnum">{s}</div><div><div class="pname">{html.escape(b["name"])}, seed {s}</div><div class="what">{html.escape(b["dials"])}</div></div></div><div class="films">{"".join(cells)}</div></section>')

(page / "index.html").write_text(f"""<title>Core World Arenas</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@500;600&family=IBM+Plex+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap">
<style>
/* One band a seed: the core world's layout before and after, side by side, with its numbers under each. */
:root {{
  --bg: #11161b; --panel: #182028; --line: #2a3540; --fg: #e3e9ee; --muted: #93a2ae;
  --slime: #e8a43a; --crystal: #3fc6dc;
  --display: "Chakra Petch", "Segoe UI", system-ui, sans-serif;
  --body: "IBM Plex Sans", "Segoe UI", system-ui, sans-serif;
  --mono: "IBM Plex Mono", ui-monospace, "Cascadia Mono", Consolas, monospace;
  color-scheme: dark;
}}
@media (prefers-color-scheme: light) {{ :root:not([data-theme="dark"]) {{ --bg: #eef1f3; --panel: #ffffff; --line: #d3dae0; --fg: #172028; --muted: #55636f; --slime: #a8650a; --crystal: #0e7f93; color-scheme: light; }} }}
:root[data-theme="light"] {{ --bg: #eef1f3; --panel: #ffffff; --line: #d3dae0; --fg: #172028; --muted: #55636f; --slime: #a8650a; --crystal: #0e7f93; color-scheme: light; }}
body {{ background: var(--bg); color: var(--fg); font: 15px/1.55 var(--body); }}
.wrap {{ max-width: 1160px; margin: 0 auto; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); padding-block: 32px 64px; display: grid; gap: 36px; }}
h1 {{ font-family: var(--display); font-weight: 600; text-wrap: balance; margin: 0; font-size: clamp(30px, 5vw, 44px); line-height: 1.1; }}
p {{ margin: 0; max-width: 68ch; }}
.top {{ display: grid; gap: 14px; }}
.eyebrow {{ font: 500 12px/1 var(--mono); text-transform: uppercase; letter-spacing: 0.12em; color: var(--slime); }}
.muted {{ color: var(--muted); }}
.key {{ display: flex; flex-wrap: wrap; gap: 6px 16px; font-size: 13px; color: var(--muted); }}
.key i {{ display: inline-block; width: 12px; height: 12px; border-radius: 2px; margin-right: 6px; vertical-align: -1px; }}
.play {{ display: grid; gap: 6px; padding: 14px 16px; border-left: 3px solid var(--slime); background: var(--panel); }}
.planet {{ display: grid; gap: 14px; padding-top: 22px; border-top: 1px solid var(--line); }}
.phead {{ display: flex; gap: 16px; align-items: center; }}
.pnum {{ flex: none; width: 40px; height: 40px; display: grid; place-items: center; border: 1px solid var(--slime); color: var(--slime); font: 600 20px/1 var(--display); border-radius: 50%; }}
.pname {{ font: 600 17px/1.2 var(--display); }}
.what {{ color: var(--muted); font-size: 13px; }}
.films {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 420px), 1fr)); gap: 20px; }}
.film {{ margin: 0; display: grid; gap: 8px; min-width: 0; }}
.film img {{ width: 100%; height: auto; display: block; border-radius: 4px; }}
.tag {{ font: 500 11px/1 var(--mono); text-transform: uppercase; letter-spacing: 0.1em; color: var(--slime); }}
.nums {{ display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 6px 14px; margin: 6px 0 0; }}
.nums div {{ min-width: 0; }}
.nums dt {{ font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }}
.nums dd {{ margin: 0; font-variant-numeric: tabular-nums; }}
</style>
<div class="wrap">
<header class="top">
<div class="eyebrow">Roguelike run · planet 8</div>
<h1>Core World Arenas</h1>
<p>The graphics lead found the core world too busy to read the way to each nest. It now has a few open arenas: three on a normal map, and more on a bigger one at the same spacing. Each arena is a field region, away from the nests and the landing, cleared of rock, pillars and hedges, with all its borders opened. Outside rocky rooms, every nest now gets a hedge ring.</p>
<p class="muted">Seeds 1-6, before and after, drawn straight from the map files. The rest of each map is drawn again, because placing the arenas uses the generator's dice. Over seeds 1-50: rock {mean(old, 'rock'):.1f}% → {mean(new, 'rock'):.1f}% of the map, soft-rock walls {mean(old, 'walls'):.0f} → {mean(new, 'walls'):.0f}, the share of charge reachable without breaking a wall {mean(old, 'open'):.1f}% → {mean(new, 'open'):.1f}%. The first try passed its checks on {sum(r["attempt"] == 0 for r in old.values())} → {sum(r["attempt"] == 0 for r in new.values())} of {len(new)} worlds.</p>
<div class="key"><span><i style="background:#5f643c"></i>ground</span><span><i style="background:#96969b"></i>rock</span><span><i style="background:#3c648c"></i>shallow water</span><span><i style="background:#192850"></i>deep water</span><span><i style="background:#825028"></i>hedge</span><span><i style="background:#3cdce6"></i>crystal</span><span><i style="background:#f0961e"></i>nest</span><span><i style="background:#fff;border:1px solid var(--line)"></i>landing</span></div>
<div class="play"><strong>Play it</strong><span class="muted">Once the change has merged, double-click <code>play.cmd</code>, set Options → Planet to 8 and press Random planet: a new core world each time.</span></div>
</header>
{"".join(bands)}
</div>
""")
print("wrote", page / "index.html")
