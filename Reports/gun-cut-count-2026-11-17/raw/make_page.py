# make_page.py RECOUNT_DIR OPEN_MAP PAGE_DIR: the report's page. RECOUNT_DIR
# is pictures.sh's output (with planet 8's 120 and 180 s finished in a
# second run copied in: see the README); OPEN_MAP is the Sleeper planet's
# 120 s map drawn with pinholes left open (the first census's rule, from a
# build before the fill); PAGE_DIR gets index.html and the images. Each
# planet shows at its count with the most slime behind necks 4 wide: the
# cut map beside the game's own frame of the whole map, with the counts.
import html
import os
import re
import shutil
import sys

from PIL import Image

D, OPEN, OUT = sys.argv[1:4]
os.makedirs(OUT, exist_ok=True)
LINE = re.compile(r"necks t (\d+) s: (\d+) walkers, (\d+) without food, (\d+) crystals with charge; "
                  r"necks 2: (\d+) cuttable \(([\d.]+)%\), largest (\d+), (\d+) parts of 500\+; "
                  r"necks 4: (\d+) cuttable \(([\d.]+)%\), largest (\d+), (\d+) parts of 500\+")
RAW = re.compile(r"necks t (\d+) s, pinholes open: necks 2: (\d+) cuttable, largest (\d+), (\d+) parts of 500\+; "
                 r"necks 4: (\d+) cuttable, largest (\d+), (\d+) parts of 500\+")
NAMES = {1: "Drifter", 2: "Sprawler", 3: "Trunk", 4: "Swarmer", 5: "Sporer", 6: "Sleeper", 7: "Grudge", 8: "late mix"}
counts = {}
for p in range(1, 9):
    log = os.path.join(D, f"p{p}-idle", "log.txt")
    text = open(log).read()
    for m in LINE.finditer(text):
        t = int(m.group(1))
        v = m.groups()
        counts[(p, t)] = dict(t=t, walkers=int(v[1]), unfed=int(v[2]), food=int(v[3]), c2=int(v[4]), s2=float(v[5]), l2=int(v[6]), b2=int(v[7]),
                              c4=int(v[8]), s4=float(v[9]), l4=int(v[10]), b4=int(v[11]))
    for m in RAW.finditer(text):
        t = int(m.group(1))
        if (p, t) in counts:
            counts[(p, t)].update(o2=int(m.group(2)), o4=int(m.group(5)), ob4=int(m.group(7)))
    m = re.search(r"t +(\d+) s: .*ended: ([a-z ]+)", text)
    if m:
        counts[(p, "end")] = f"ended at {m.group(1)} s, {m.group(2)}"


def pct(a, b):
    return 100 * a / max(b, 1)


def img(src, name, jpeg=False):
    if jpeg:
        Image.open(src).convert("RGB").save(os.path.join(OUT, name), quality=82, optimize=True)
    else:
        shutil.copy(src, os.path.join(OUT, name))
    return name


rows = []
figs = []
for p in range(1, 9):
    ts = sorted(t for (q, t) in counts if q == p and t != "end")
    for t in ts:
        c = counts[(p, t)]
        rows.append(f"<tr><td>{p} {NAMES[p]}</td><td>{t} s</td><td>{c['walkers']:,}</td>"
                    f"<td>{c['s2']:.1f}%</td><td>{c['s4']:.1f}%</td><td>{c['l4']:,} <span class=dim>({pct(c['l4'], c['walkers']):.1f}%)</span></td><td>{c['b4']}</td>"
                    f"<td class=dim>{pct(c.get('o4', 0), c['walkers']):.1f}%</td></tr>")
    if not ts:
        continue
    t = max(ts, key=lambda t: counts[(p, t)]["s4"])
    c = counts[(p, t)]
    base = f"planet-{p}-1-idle-{t:03d}s"
    m = img(os.path.join(D, f"p{p}-idle", base + "-necks.png"), f"p{p}-{t}-map.png")
    f = img(os.path.join(D, f"p{p}-idle", base + "-all.png"), f"p{p}-{t}-frame.jpg", jpeg=True)
    end = counts.get((p, "end"), "")
    figs.append(f"""
  <article class=planet>
    <header><h3>Planet {p}: {NAMES[p]}</h3><span class=tag>{t} s, its worst count{', ' + html.escape(end) if end else ''}</span></header>
    <div class=pair>
      <figure><img class=still src="{m}" alt="Cut map of planet {p} at {t} s" loading=lazy><figcaption>The cut map</figcaption></figure>
      <figure><img class=still src="{f}" alt="The game's frame of planet {p} at {t} s" loading=lazy><figcaption>The game at the same moment</figcaption></figure>
    </div>
    <dl class=nums>
      <div><dt>Slime</dt><dd>{c['walkers']:,} walkers</dd></div>
      <div><dt>Behind necks 2</dt><dd class=red>{c['s2']:.1f}%</dd></div>
      <div><dt>Behind necks 4</dt><dd class=orange>{c['s4']:.1f}%</dd></div>
      <div><dt>Largest part</dt><dd>{c['l4']:,} ({pct(c['l4'], c['walkers']):.1f}%)</dd></div>
      <div><dt>Parts of 500+</dt><dd>{c['b4']}</dd></div>
    </dl>
  </article>""")

o = counts[(6, 120)]
open_map = img(OPEN, "p6-120-open.png")
filled_map = img(os.path.join(D, "p6-idle", "planet-6-1-idle-120s-necks.png"), "p6-120-filled.png")
n = len([k for k in counts if k[1] != "end"])
small = sum(1 for k, c in counts.items() if k[1] != "end" and pct(c["l4"], c["walkers"]) < 2)
s4 = sorted(c["s4"] for k, c in counts.items() if k[1] != "end")
med = s4[len(s4) // 2]

page = f"""<title>Slime Cut Count</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@600;700&family=IBM+Plex+Sans:wght@400;600&family=IBM+Plex+Mono:wght@400;500&display=swap">
<style>
/* Same system as the Mind, reader, route, element and Tinder pages. Accents: the cut map's own red and orange. */
:root{{
  --bg:#eef0ee; --panel:#e0e4e1; --fg:#161a19; --muted:#56605c; --line:#c5cbc7;
  --red:#c42a2a; --orange:#b86a00; --crystal:#137f95;
  --display:"Chakra Petch","Segoe UI",system-ui,sans-serif;
  --body:"IBM Plex Sans","Segoe UI",system-ui,sans-serif;
  --mono:"IBM Plex Mono",ui-monospace,Consolas,monospace;
}}
@media (prefers-color-scheme: dark){{:root:not([data-theme="light"]){{
  --bg:#101413; --panel:#1a201e; --fg:#e8ece9; --muted:#9aa5a0; --line:#2b3330;
  --red:#ff6b5c; --orange:#ffb03a; --crystal:#5fd0e3; color-scheme:dark}}}}
:root[data-theme="dark"]{{
  --bg:#101413; --panel:#1a201e; --fg:#e8ece9; --muted:#9aa5a0; --line:#2b3330;
  --red:#ff6b5c; --orange:#ffb03a; --crystal:#5fd0e3; color-scheme:dark}}
body{{background:var(--bg);color:var(--fg);font:16.5px/1.55 var(--body);
  padding-left:max(16px,env(safe-area-inset-left,0px));padding-right:max(16px,env(safe-area-inset-right,0px));padding-block:28px 64px}}
main{{max-width:1180px;margin:0 auto;display:flex;flex-direction:column;gap:38px}}
h1,h2,h3{{font-family:var(--display);text-wrap:balance;line-height:1.12;margin:0;letter-spacing:.01em}}
h1{{font-size:2.5rem;font-weight:700}} h2{{font-size:1.5rem;font-weight:700}} h3{{font-size:1.15rem;font-weight:700}}
p{{margin:0;max-width:70ch}}
section{{display:flex;flex-direction:column;gap:14px}}
.eyebrow{{font:12px/1 var(--mono);letter-spacing:.1em;text-transform:uppercase;color:var(--muted)}}
.lede{{color:var(--muted);font-size:1.04rem;max-width:72ch}}
.verdict{{border-left:4px solid var(--red);background:var(--panel);padding:14px 18px;border-radius:0 6px 6px 0;max-width:80ch}}
.verdict strong{{font-family:var(--display);font-size:1.2rem}}
ul{{margin:0;padding-left:1.2em;display:flex;flex-direction:column;gap:8px;max-width:76ch}}
code{{font:.86em var(--mono);background:var(--panel);padding:1px 4px;border-radius:3px}}
.legend{{display:flex;flex-wrap:wrap;gap:8px 18px;font-size:.92rem}}
.legend span{{display:inline-flex;align-items:center;gap:6px}}
.legend i{{width:14px;height:14px;border-radius:3px;display:inline-block;border:1px solid var(--line)}}
.pair{{display:grid;grid-template-columns:repeat(2,1fr);gap:14px}}
@media (max-width:760px){{.pair{{grid-template-columns:1fr}}}}
figure{{margin:0;display:flex;flex-direction:column;gap:6px;min-width:0}}
figcaption{{font-size:.88rem;color:var(--muted)}}
img.still{{width:100%;max-width:100%;border-radius:6px;border:1px solid var(--line);display:block;background:#16181c}}
.planet{{display:flex;flex-direction:column;gap:10px;padding-top:18px;border-top:1px solid var(--line)}}
.planet header{{display:flex;flex-wrap:wrap;gap:6px 14px;align-items:baseline}}
.tag{{font:500 12px/1.3 var(--mono);letter-spacing:.06em;text-transform:uppercase;color:var(--muted)}}
.nums{{display:flex;flex-wrap:wrap;gap:8px 26px;margin:0}}
.nums div{{display:flex;flex-direction:column}}
.nums dt{{font:12px/1.3 var(--mono);letter-spacing:.06em;text-transform:uppercase;color:var(--muted)}}
.nums dd{{margin:0;font:600 1.15rem/1.3 var(--display);font-variant-numeric:tabular-nums}}
.red{{color:var(--red)}} .orange{{color:var(--orange)}} .dim{{color:var(--muted)}}
.scroll{{overflow-x:auto;max-width:100%}}
table{{border-collapse:collapse;font-variant-numeric:tabular-nums;font-size:.93rem;min-width:640px}}
th,td{{text-align:right;padding:5px 10px;border-bottom:1px solid var(--line);white-space:nowrap}}
th:first-child,td:first-child{{text-align:left}}
th{{font:500 12px/1.3 var(--mono);letter-spacing:.05em;text-transform:uppercase;color:var(--muted)}}
</style>
<main>
<header style="display:flex;flex-direction:column;gap:12px">
  <span class=eyebrow>Gun cards, step D, the cut family · 2026-11-17</span>
  <h1>Is there slime worth cutting off?</h1>
  <p class=lede>Before a Cut card is built, the plan asked how often a run planet has a part that one thin cut would leave without food.
  The owner's worry (11-16): the slime sits on crystals all over, so there is rarely a big piece to cut.</p>
  <div class=verdict><strong>No Cut card.</strong> The largest part one cut could starve is under 2% of the slime in {small} of {n} counts
  (median share behind necks 4 cells wide: {med:.1f}%). The one exception is the Swarmer's first minute: arms round the rock, 19% of the slime, the biggest part 9%.
  Walls and positioning already cut there, and withering and panic make that cut show.</div>
</header>

<section>
  <h2>How to read a cut map</h2>
  <p>One square per dish cell. A part is cuttable when the only slime joining it to a crystal with charge is a neck 2 or 4 dish cells wide.
  A part held by two necks counts too, so this is the most one cut could give.</p>
  <div class=legend>
    <span><i style="background:rgb(220,40,40)"></i>cut off by a neck of 2</span>
    <span><i style="background:rgb(245,150,30)"></i>cut off only by a neck of 4</span>
    <span><i style="background:rgb(150,135,100)"></i>slime on food</span>
    <span><i style="background:rgb(120,80,160)"></i>slime already without food</span>
    <span><i style="background:rgb(60,140,230)"></i>a crystal with charge (food)</span>
  </div>
</section>

<section>
  <h2>The first count was wrong on the Sleeper</h2>
  <p>The first census said 30-56% of the Sleeper planet's slime hung on necks 4 cells wide. Its map showed why: a wide sheet is speckled with empty cells,
  and the count read the speckle as necks. Empty cells on open ground with 5 of their 8 neighbours slime now count as slime for the shape (rock is never filled).
  Same moment, same slime: {pct(o['o4'], o['walkers']):.1f}% before, {o['s4']:.1f}% after.</p>
  <div class=pair>
    <figure><img class=still src="{open_map}" alt="Sleeper planet at 120 s, pinholes left open" loading=lazy><figcaption>Pinholes left open: {pct(o['o4'], o['walkers']):.1f}% behind necks 4, {o['ob4']} parts of 500+</figcaption></figure>
    <figure><img class=still src="{filled_map}" alt="Sleeper planet at 120 s, pinholes filled" loading=lazy><figcaption>Pinholes filled: {o['s4']:.1f}% behind necks 4, {o['b4']} parts of 500+</figcaption></figure>
  </div>
</section>

<section>
  <h2>Each planet at its worst count</h2>
  <p>Run seed 1, nobody shooting (the squad-taking hand in the first census matched it wherever both still played), counted at 1, 2 and 3 minutes.</p>
  {''.join(figs)}
</section>

<section>
  <h2>Every count</h2>
  <p>Shares are of all the slime's walkers at that moment. The last column is the first census's rule (pinholes left open), for comparison.</p>
  <div class=scroll><table>
    <thead><tr><th>Planet</th><th>Time</th><th>Walkers</th><th>Necks 2</th><th>Necks 4</th><th>Largest part</th><th>500+</th><th>Open, necks 4</th></tr></thead>
    <tbody>{''.join(rows)}</tbody>
  </table></div>
</section>

<section>
  <h2>What follows</h2>
  <ul>
    <li>No Cut card, by the game designer's rule: "a cut bullet only earns a slot if the shapes are there".</li>
    <li>A thin cut refills in about 2 s unless held (one-body report, 11-12), so where the shapes are, a wall or a squad in the gap is the cut.</li>
    <li>If a new slime spreads in long veins, re-run the count before reopening the card. It would say yes if the largest part were 5% or more of the slime in most counts.</li>
    <li>The Heave and Corral merge goes in the next playtest pack as your pick.</li>
  </ul>
  <p class=dim>Source: <code>Reports/gun-cut-count-2026-11-17/</code> (the count is <code>mould::necks</code>, the key <code>larder-world necks=T,...</code>).</p>
</section>
</main>
"""
open(os.path.join(OUT, "index.html"), "w").write(page)
print(f"{n} counts, {small} with the largest part under 2%, median necks 4 {med:.1f}%")
