#!/usr/bin/env python3
"""Writes the round-2 layouts page (PAGE/index.html) from PAGE/counts.json
(make_gifs.py), the planets example's table for seeds 1-2 (STATS), and the
grey thumbnails in PAGE/grey/ (make_grey.py: round 1 and round 2 at 1 s).

  python3 make_page2.py PAGE STATS
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
    ("Barrens", "Was: scattered barren patches that read as plain ground. Now: three wide barren bands right across the route, meandering, ragged at the edges.", "Each band starves the slime as it crosses."),
    ("Marsh", "Was: small scattered pools. Now: big pools from one broad noise, so they run together, on clay banks.", "It crawls."),
    ("Canyon", "Was: short rock stubs and mesas that read as debris. Now: two ridges right across the map, about 50-80 cells wide, each with one or two passes (sometimes one gated), and fewer, longer walls between bigger regions; no mesas.", "Chokepoints."),
    ("Dunes", "Was: exact clay circles under the crystals. Now: islands a third bigger with ragged, noisy edges.", "It races, and holds on the islands."),
    ("Claypan", "Was: mostly clay in soft patches. Now: clay nearly everywhere, cracked into polygons of plain ground, the cracks broken in places.", "It digs in."),
    ("Thicket", "Was: straight gate bars and perfect arcs that read as UI. Now: every hedge, ring and gate a chain of short pieces that wobbles and changes thickness (11-21 cells).", "Walls to chew through."),
    ("Spires", "Was: a few rock blobs. Now: up to 210 small pillars (radius 7-15 cells), each ragged, plus a few mesas.", "Cover the slime grows behind."),
    ("Core", "Every ground at once, three nests, rings and gates; its soft rock grown like the Thicket's.", "The core world, the last of eight."),
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
        grey = f"""<div class=greys><figure><img src="grey/r1-{name}.png" width="266" height="164" alt="Round 1, grey" loading="lazy"><figcaption>Round 1, grey</figcaption></figure><figure><img src="grey/r2-{name}.png" width="266" height="164" alt="Round 2, grey" loading="lazy"><figcaption>Round 2, grey</figcaption></figure></div>"""
        films.append(
            f"""<figure class=film>
  <img src="gif/{name}.gif" width="531" height="328" alt="Planet {d}, seed {s}: {html.escape(st['name'])}, the slime spreading over two minutes with nobody playing" loading="lazy">
  <figcaption>
    <div class=cap-head><span class=pname>{html.escape(st['name'])}</span><code>map=planet:{d}:{s}</code></div>
    {grey}
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
<thead><tr><th>Biome</th><th>Crystals</th><th>Rock</th><th>Sand</th><th>Clay</th><th>Barren</th><th>Wet</th><th>Soft-rock pieces</th><th>Walk to the nearest nest</th><th>First try passes</th></tr></thead>
<tbody>
SURVEY_ROWS
</tbody></table>"""

rows = []
for line in Path(sys.argv[3]).read_text().splitlines():
    c = [x.strip() for x in line.strip().strip("|").split("|")]
    rows.append("<tr>" + "".join(f"<td>{x}</td>" for x in (c[0].title(), *c[1:8], c[8], c[11])) + "</tr>")
survey = survey.replace("SURVEY_ROWS", "\n".join(rows))

sheet = """<section class=band>
  <h2>The grey thumbnail test</h2>
  <p class=lede>All sixteen planets at one second, in grey, small, before and after. The test from the art direction: each biome should be recognisable in a grey thumbnail.</p>
  <div class=sheets><figure><img src="grey/sheet-r1.png" alt="Round 1, sixteen planets in grey" loading="lazy"><figcaption>Round 1 (#277)</figcaption></figure><figure><img src="grey/sheet-r2.png" alt="Round 2, sixteen planets in grey" loading="lazy"><figcaption>Round 2 (this change)</figcaption></figure></div>
  <p class=muted>Rows: planets 1-2, 3-4, 5-6, 7-8; seed 1 then seed 2 of each. In the game the ground kinds differ little in grey value, so the layout shows mostly through rock and soft rock; the ground's own look is the terrain shaders' work.</p>
  <h2>The layouts themselves</h2>
  <p class=lede>The same sixteen maps drawn straight from their files in false colour, so the layout reads apart from the shaders: rock light grey, sand tan, clay brown, barren grey-green, wet blue, soft rock beige, crystals cyan, nests orange rings, the landing a white square.</p>
  <div class=sheets><figure><img src="grey/layout-r2.png" alt="Round 2 layouts in false colour" loading="lazy"><figcaption>Round 2 layouts, false colour</figcaption></figure><figure><img src="grey/layout-r2-grey.png" alt="Round 2 layouts in false colour, grey" loading="lazy"><figcaption>The same, grey</figcaption></figure></div>
</section>"""

nav = "".join(f'<a href="#planet-{d}"><span>{d}</span>{b}</a>' for d, (b, _, _) in enumerate(BIOMES, start=1))

doc = f"""<title>Planet Layouts Round 2</title>
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
.greys {{ display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }}
.greys figure, .sheets figure {{ margin: 0; display: grid; gap: 4px; min-width: 0; }}
.greys img, .sheets img {{ width: 100%; height: auto; display: block; border-radius: 3px; }}
.greys figcaption, .sheets figcaption {{ font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }}
.sheets {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 460px), 1fr)); gap: 20px; }}
ul {{ margin: 0; padding-left: 20px; display: grid; gap: 6px; max-width: 72ch; }}
@media (max-width: 520px) {{ .nums {{ grid-template-columns: repeat(2, minmax(0, 1fr)); }} }}
@media (prefers-reduced-motion: reduce) {{ * {{ scroll-behavior: auto; }} }}
</style>
<div class=wrap>
  <header class=top>
    <div class=eyebrow>Larder World · roguelike run</div>
    <h1>Planet layouts, round 2</h1>
    <p class=lede>The graphics lead's review of the first planets: in grey, Barrens, Canyon, Claypan and Spires were the same picture. Each biome now has one signature shape at zoom 1. Below, the same two seeds of each, filmed in the game for two minutes with nobody playing, with the round-1 and round-2 maps in grey beside them. Only the layout changed; the ground's materials are the terrain shaders'.</p>
    <nav class=route aria-label="The run's planets">{nav}</nav>
    <div class=play>
      <div><strong>Play one:</strong> on the start menu, <strong>Random planet</strong> lands on a new planet; the <strong>Planet</strong> option (Options page) picks which of the eight; <strong>N</strong> in the round rolls another.</div>
      <div class=muted>From a command line: <code>larder-world map=planet:3:42</code> (planet 3, seed 42). The map editor opens one too: <code>larder-edit map=planet:3:42</code>.</div>
    </div>
  </header>
  {sheet}
  {''.join(sections)}
  {clip_html}
  <section class=band>
    <h2>Every planet is checked before you land</h2>
    <p class=lede>A planet that fails a check is rebuilt from the next attempt. The checks walk the map the way a squad does: every crystal and slime nest reachable from the landing, a crystal close to the landing, the far nest across the map on foot, half the crystals' charge open without breaking soft rock. Over 400 planets (seeds 1-50, planets 1-8) all passed within CENSUS_LINE, once a level. Medians, with the walking range in brackets (the map is 1,024 cells wide):</p>
    <div class=scroll>{survey}</div>
  </section>
  <section class=band>
    <h2>Next</h2>
    <ul>
      <li><strong>Planets grow:</strong> 60% of today's map on planet 1, today's size by planet 3 or 4, six times the area on the core world. Every planet here is still today's size.</li>
      <li><strong>The ground's materials</strong> (sand ripples, clay cracks, barren ash, a gloss on water) are the terrain thread's shader work.</li>
    </ul>
  </section>
</div>
"""
doc = doc.replace("CENSUS_LINE", sys.argv[4])
(page / "index.html").write_text(doc)
print(page / "index.html")
