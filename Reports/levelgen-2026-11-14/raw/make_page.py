#!/usr/bin/env python3
"""Writes the planets page (PAGE/index.html) from PAGE/counts.json (make_gifs.py)
and the planets example's table for seeds 1-2 (STATS).

  python3 make_page.py PAGE STATS
"""
import html
import json
import sys
from pathlib import Path

page, stats_path = Path(sys.argv[1]), Path(sys.argv[2])
counts = json.loads((page / "counts.json").read_text())
clips = sorted(p.name for p in (page / "clip").glob("*.gif")) if (page / "clip").exists() else []

stats = {}
for line in stats_path.read_text().splitlines()[1:]:
    if "|" not in line:
        continue
    parts = [p.strip() for p in line.split("|")]
    a, b, c = parts[0].split(), parts[1].split(), parts[2].split()
    stats[(int(a[0]), int(a[1]))] = {
        "name": a[2], "biome": a[3], "crystals": b[0], "rock": b[1], "sand": b[2], "clay": b[3],
        "barren": b[4], "wet": b[5], "walls": b[6], "nest": c[0], "first": c[1], "crystal": c[2], "open": parts[3],
    }

BIOMES = [
    ("Barrens", "Bands of barren ground between the landing and the slime. Barren ground starves it: no births, three times the upkeep.", "The easy first planet."),
    ("Marsh", "Wet pools on clay banks. Wet ground halves the slime's step.", "It crawls."),
    ("Canyon", "Wide rock ridges across the route, each with a narrow pass, some shut with soft rock.", "Chokepoints."),
    ("Dunes", "Sand everywhere, clay islands under the crystals. Sand spreads the slime (about 60% bigger).", "It races. The first planet with two nests."),
    ("Claypan", "Mostly clay. Clay keeps its trail, so the slime stays where it is.", "It digs in."),
    ("Thicket", "Soft-rock hedges, gates across passes and a ring round each nest with its gap toward you.", "Walls to chew through."),
    ("Spires", "Rock pillars and mesas on sand and clay.", "Cover the slime grows behind, so the squad has to move."),
    ("Core", "Every ground at once, three nests, rings and gates.", "The core world, the last of eight."),
]


def k(n):
    return f"{n / 1000:.1f}k" if n >= 1000 else str(n)


def film_counts(name):
    rows = counts.get(name, [])
    if not rows:
        return "<p class=muted>No film.</p>"
    by_t = {r["t"]: r for r in rows}
    pick = [by_t[t] for t in (1, 60, 120) if t in by_t]
    cells = "".join(f"<tr><td>{r['t']} s</td><td>{k(r['mould'])}</td><td>{r['killed']}</td><td>{r['left'] * 100:.1f}%</td></tr>" for r in pick)
    return f"<table class=counts><thead><tr><th>Time</th><th>Walkers</th><th>Killed by the idle squad</th><th>Charge left</th></tr></thead><tbody>{cells}</tbody></table>"


def planet_numbers(st):
    soil = [f"{v} {n}" for n, v in (("sand", st["sand"]), ("clay", st["clay"]), ("barren", st["barren"]), ("wet", st["wet"])) if v != "0%"]
    return (
        f"<dl class=nums>"
        f"<div><dt>Crystals</dt><dd>{st['crystals']}</dd></div>"
        f"<div><dt>Rock</dt><dd>{st['rock']}</dd></div>"
        f"<div><dt>Ground</dt><dd>{', '.join(soil) or 'plain'}</dd></div>"
        f"<div><dt>Soft-rock walls</dt><dd>{st['walls']}</dd></div>"
        f"<div><dt>Walk to the slime</dt><dd>{st['nest']} cells</dd></div>"
        f"<div><dt>Walk to a crystal</dt><dd>{st['crystal']} cells</dd></div>"
        f"</dl>"
    )


sections = []
for d, (biome, what, why) in enumerate(BIOMES, start=1):
    films = []
    for s in (1, 2):
        st = stats.get((d, s))
        name = f"planet-{d}-{s}"
        if st is None:
            continue
        films.append(
            f"""<figure class=film>
  <img src="gif/{name}.gif" width="531" height="328" alt="Planet {d}, seed {s}: {html.escape(st['name'])}, the slime spreading over two minutes with nobody playing" loading="lazy">
  <figcaption>
    <div class=cap-head><span class=pname>{html.escape(st['name'])}</span><code>map=planet:{d}:{s}</code></div>
    {planet_numbers(st)}
    {film_counts(name)}
  </figcaption>
</figure>"""
        )
    sections.append(
        f"""<section class=planet id="planet-{d}">
  <header class=phead>
    <div class=pnum>{d}</div>
    <div><h2>{biome}</h2><p class=what>{html.escape(what)} <strong>{html.escape(why)}</strong></p></div>
  </header>
  <div class=films>{''.join(films)}</div>
</section>"""
    )

clip_html = ""
if clips:
    figs = "".join(f'<figure class=clip><img src="clip/{c}" alt="{c[:-4]}" loading="lazy"><figcaption>{"Canyon (planet 3)" if "3" in c else "Thicket (planet 6)"}, seed 1, near the squad, 40-46 s at a quarter-second a frame</figcaption></figure>' for c in clips)
    clip_html = f"""<section class=band>
  <h2>Close in</h2>
  <p class=lede>Near the landing at the game's opening zoom, a frame every quarter second: the slime's first explorers reaching the squad across each planet's ground.</p>
  <div class=clips>{figs}</div>
</section>"""

survey = """<table class=survey>
<thead><tr><th>Biome</th><th>Crystals</th><th>Rock</th><th>Sand</th><th>Clay</th><th>Barren</th><th>Wet</th><th>Walls</th><th>Walk to the nearest nest</th><th>First try passes</th></tr></thead>
<tbody>
<tr><td>Barrens</td><td>16</td><td>6%</td><td>0%</td><td>0%</td><td>38%</td><td>0%</td><td>1</td><td>794 (643-942)</td><td>45/50</td></tr>
<tr><td>Marsh</td><td>16</td><td>4%</td><td>0%</td><td>30%</td><td>0%</td><td>19%</td><td>1</td><td>794 (640-956)</td><td>50/50</td></tr>
<tr><td>Canyon</td><td>16</td><td>17%</td><td>26%</td><td>0%</td><td>0%</td><td>0%</td><td>2</td><td>882 (661-1144)</td><td>43/50</td></tr>
<tr><td>Dunes</td><td>22</td><td>3%</td><td>49%</td><td>30%</td><td>0%</td><td>0%</td><td>0</td><td>578 (484-770)</td><td>48/50</td></tr>
<tr><td>Claypan</td><td>17</td><td>7%</td><td>0%</td><td>70%</td><td>1%</td><td>0%</td><td>1</td><td>610 (493-860)</td><td>50/50</td></tr>
<tr><td>Thicket</td><td>17</td><td>5%</td><td>0%</td><td>46%</td><td>0%</td><td>0%</td><td>80</td><td>587 (488-900)</td><td>45/50</td></tr>
<tr><td>Spires</td><td>17</td><td>7%</td><td>28%</td><td>21%</td><td>0%</td><td>0%</td><td>0</td><td>545 (485-906)</td><td>49/50</td></tr>
<tr><td>Core</td><td>21</td><td>8%</td><td>14%</td><td>32%</td><td>5%</td><td>7%</td><td>61</td><td>542 (478-652)</td><td>36/50</td></tr>
</tbody></table>"""

nav = "".join(f'<a href="#planet-{d}"><span>{d}</span>{b}</a>' for d, (b, _, _) in enumerate(BIOMES, start=1))

doc = f"""<title>Roguelike Planets</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@500;600&family=IBM+Plex+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap">
<style>
/* Layout: a header with the route of eight planets, then one band a planet, two films side by side with their counts under each. */
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
.wrap {{ max-width: 1160px; margin: 0 auto; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); padding-block: 32px 64px; display: grid; gap: 40px; }}
h1, h2 {{ font-family: var(--display); font-weight: 600; text-wrap: balance; margin: 0; letter-spacing: 0.01em; }}
h1 {{ font-size: clamp(30px, 5vw, 44px); line-height: 1.1; }}
h2 {{ font-size: 24px; line-height: 1.2; }}
p {{ margin: 0; max-width: 68ch; }}
code {{ font-family: var(--mono); font-size: 0.86em; color: var(--crystal); }}
.muted, .lede {{ color: var(--muted); }}
.top {{ display: grid; gap: 14px; }}
.top .eyebrow {{ font: 500 12px/1 var(--mono); text-transform: uppercase; letter-spacing: 0.12em; color: var(--slime); }}
.route {{ display: flex; flex-wrap: wrap; gap: 6px; margin-top: 6px; }}
.route a {{ display: inline-flex; align-items: center; gap: 8px; padding: 6px 10px 6px 6px; border: 1px solid var(--line); border-radius: 6px; color: var(--fg); text-decoration: none; font: 500 13px/1 var(--body); }}
.route a span {{ display: inline-grid; place-items: center; width: 20px; height: 20px; border-radius: 4px; background: var(--line); font: 600 12px/1 var(--mono); }}
.route a:last-child span {{ background: var(--slime); color: var(--bg); }}
.route a:hover, .route a:focus-visible {{ border-color: var(--slime); outline: none; }}
.play {{ display: grid; gap: 6px; padding: 14px 16px; border-left: 3px solid var(--slime); background: var(--panel); }}
.planet {{ display: grid; gap: 16px; padding-top: 24px; border-top: 1px solid var(--line); }}
.phead {{ display: flex; gap: 16px; align-items: flex-start; }}
.pnum {{ flex: none; width: 44px; height: 44px; display: grid; place-items: center; border: 1px solid var(--slime); color: var(--slime); font: 600 22px/1 var(--display); border-radius: 50%; }}
.what strong {{ font-weight: 600; color: var(--fg); }}
.what {{ color: var(--muted); margin-top: 4px; }}
.films {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 420px), 1fr)); gap: 20px; }}
.film {{ margin: 0; display: grid; gap: 10px; min-width: 0; }}
.film img, .clip img {{ width: 100%; height: auto; display: block; border-radius: 4px; background: #000; }}
.cap-head {{ display: flex; flex-wrap: wrap; gap: 8px 14px; align-items: baseline; }}
.pname {{ font: 600 17px/1.2 var(--display); }}
.nums {{ display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 6px 14px; margin: 0; }}
.nums div {{ min-width: 0; }}
.nums dt {{ font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }}
.nums dd {{ margin: 0; font-variant-numeric: tabular-nums; }}
table {{ border-collapse: collapse; font-variant-numeric: tabular-nums; }}
.counts {{ width: 100%; font-size: 13px; }}
th, td {{ text-align: left; padding: 5px 10px 5px 0; border-bottom: 1px solid var(--line); vertical-align: top; }}
th {{ font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--muted); }}
.scroll {{ overflow-x: auto; }}
.survey {{ min-width: 760px; font-size: 13.5px; }}
.band {{ display: grid; gap: 14px; padding-top: 24px; border-top: 1px solid var(--line); }}
.clips {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 420px), 1fr)); gap: 20px; }}
.clip {{ margin: 0; display: grid; gap: 8px; min-width: 0; }}
.clip figcaption {{ color: var(--muted); font-size: 13px; }}
ul {{ margin: 0; padding-left: 20px; display: grid; gap: 6px; max-width: 72ch; }}
@media (max-width: 520px) {{ .nums {{ grid-template-columns: repeat(2, minmax(0, 1fr)); }} }}
@media (prefers-reduced-motion: reduce) {{ * {{ scroll-behavior: auto; }} }}
</style>
<div class=wrap>
  <header class=top>
    <div class=eyebrow>Larder World · roguelike run</div>
    <h1>Eight planets to the core world</h1>
    <p class=lede>Each level of the run is a generated planet: a seed and a planet number make the map, the same on every machine. The biomes come in a set order, each built from ground the slime already feels. Below, two seeds of each, filmed in the game for two minutes with nobody playing, so you can see the ground shape the slime.</p>
    <nav class=route aria-label="The run's planets">{nav}</nav>
    <div class=play>
      <div><strong>Play one:</strong> on the start menu, <strong>Random planet</strong> lands on a new planet; the <strong>Planet</strong> option (Options page) picks which of the eight; <strong>N</strong> in the round rolls another.</div>
      <div class=muted>From a command line: <code>larder-world map=planet:3:42</code> (planet 3, seed 42). The map editor opens one too: <code>larder-edit map=planet:3:42</code>.</div>
    </div>
  </header>
  {''.join(sections)}
  {clip_html}
  <section class=band>
    <h2>Every planet is checked before you land</h2>
    <p class=lede>A planet that fails a check is rebuilt from the next attempt. The checks walk the map the way a squad does: every crystal and slime nest reachable from the landing, a crystal close to the landing, the far nest across the map on foot, half the crystals' charge open without breaking soft rock. Over 400 planets (seeds 1-50, planets 1-8) all passed within five attempts, 366 on the first, and a planet takes 55 ms to build, once a level. Medians, with the walking range in brackets (the map is 1,024 cells wide):</p>
    <div class=scroll>{survey}</div>
  </section>
  <section class=band>
    <h2>Next</h2>
    <ul>
      <li><strong>Planets grow:</strong> 60% of today's map on planet 1, today's size by planet 3 or 4, six times the area on the core world. Every planet here is still today's size; the biggest gets a timing run on the 1070 first.</li>
      <li><strong>Twists</strong> shown before landing, such as Flooded or Rich crystals.</li>
      <li><strong>Rescue planets</strong>, rare: base ruins and stranded soldiers.</li>
    </ul>
  </section>
</div>
"""
(page / "index.html").write_text(doc)
print(page / "index.html")
