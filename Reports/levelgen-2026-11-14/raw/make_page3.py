#!/usr/bin/env python3
"""Writes the round-3 biomes page (PAGE/index.html) from PAGE/counts.json
(make_gifs.py on the round-3 films), the planets example's survey of seeds
1-3 (SURVEY: dials and numbers a planet), the contact sheets in PAGE/sheet/
(biome_sheet.py) and the report's round-3 table (REPORT).

  python3 make_page3.py PAGE SURVEY REPORT
"""
import html
import json
import re
import sys
from pathlib import Path

page, survey_path, report = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
counts = json.loads((page / "counts.json").read_text())

# depth seed name biome [openness water soil pattern] attempt ms | crystals rock sand clay barren wet walls | walk: nest first crystal | open | failed
stats = {}
for line in survey_path.read_text().splitlines()[1:]:
    m = re.match(r"(\d+) (\d+) (\S+) (\S+) \[(\S+) (\S+) (\S+) (\S+)\] (\d+) (\d+) \| (.*?) \| (.*?) \| (.*?) \|", line)
    if not m:
        continue
    d, s, name, biome, o, w, so, pat, att, ms, nums, walk, opn = m.groups()
    b, c = nums.split(), walk.split()
    stats[(int(d), int(s))] = {
        "name": name, "biome": biome.replace("-", " "), "dials": (o, w, so, pat.replace("-", " ")), "attempt": int(att),
        "crystals": b[0], "rock": b[1], "sand": b[2], "clay": b[3], "barren": b[4], "wet": b[5], "walls": b[6],
        "nest": c[0], "crystal": c[2],
    }

DIAL_WORDS = {
    "Open": "open", "Outcrops": "outcrops", "Pillars": "pillars", "Ridges": "ridges", "Rooms": "rocky rooms", "Caves": "caves",
    "Dry": "dry", "Pools": "pools", "Lakes": "lakes", "Rivers": "rivers", "Streams": "streams",
    "Plain": "plain soil", "Sand": "sand", "Dunes": "dunes", "Bands": "barren bands", "Oases": "oases in barren", "Clay": "clay", "Cracks": "cracked clay", "Mixed": "mixed soil",
}

# What each biome rolls (its odds in `Recipe::of`) and what that does to play.
BIOMES = [
    ("Meadow", "Open ground, a few outcrops or pillars; pools, a stream or dry; plain soil with some sand or clay; any crystal pattern.", "The slime's first planet: room to learn it."),
    ("Barrens", "Open or outcrops, always dry; either three barren bands across the route or barren everywhere but a few oases, where the crystals sit.", "Barren starves the slime: it crosses in thin lines, or holds the oases."),
    ("Fen", "Mostly open; big pools or lakes with a deep middle, on clay; crystals often in veins along the shore.", "Water slows the slime, clay holds it: a fight at the banks."),
    ("River valley", "Rivers right across the route, deep in the middle with one or two fords; sand dunes on the banks; ridges or outcrops.", "Fords are the chokes (deep water will stop soldiers once the ground thread lands it)."),
    ("Jungle", "Outcrops; streams or pools on clay; soft-rock hedges everywhere, rings round the nests, gated passes.", "Walls to chew or blast through; short sight lines."),
    ("Highlands", "Rocky rooms or long ridges, joined by passes, on cracked clay; mostly dry; fields of crystals or one jackpot.", "Rooms to hold and passes to wall: a builder's planet."),
    ("Cave", "Half the ground rock: caverns joined by tunnels; pools in some caverns; veins or a jackpot.", "Tunnels to hold, close fighting, the slime coming round the corner."),
    ("Core", "Every dial at once, three nests, more regions, hedges and rings (no rings in rocky rooms).", "The slime's home world, the last of eight."),
]


def k(n):
    return f"{n / 1000:.1f}k" if n >= 1000 else str(n)


def film_counts(name):
    rows = counts.get(name, [])
    if not rows:
        return "<p class=muted>No film.</p>"
    by_t = {r["t"]: r for r in rows}
    pick = [by_t[t] for t in (1, 60, 120) if t in by_t]
    cells = "".join(f"<tr><td>{r['t']} s</td><td>{k(r['mould'])}</td><td>{r['left'] * 100:.1f}%</td></tr>" for r in pick)
    return f"<table class=counts><thead><tr><th>Time</th><th>Slime walkers</th><th>Crystal charge left</th></tr></thead><tbody>{cells}</tbody></table>"


def planet_numbers(st):
    soil = [f"{v} {n}" for n, v in (("sand", st["sand"]), ("clay", st["clay"]), ("barren", st["barren"]), ("water", st["wet"])) if v != "0%"]
    return (
        f"<dl class=nums>"
        f"<div><dt>Crystals</dt><dd>{st['crystals']}</dd></div>"
        f"<div><dt>Rock</dt><dd>{st['rock']}</dd></div>"
        f"<div><dt>Ground</dt><dd>{', '.join(soil) or 'plain'}</dd></div>"
        f"<div><dt>Soft-rock pieces</dt><dd>{st['walls']}</dd></div>"
        f"<div><dt>Walk to the slime</dt><dd>{st['nest']} cells</dd></div>"
        f"<div><dt>Walk to a crystal</dt><dd>{st['crystal']} cells</dd></div>"
        f"</dl>"
    )


def dials(st):
    o, w, so, pat = st["dials"]
    chips = [("Openness", DIAL_WORDS.get(o, o)), ("Water", DIAL_WORDS.get(w, w)), ("Soil", DIAL_WORDS.get(so, so)), ("Crystals", pat)]
    return "<ul class=dials>" + "".join(f"<li><span>{a}</span>{html.escape(b)}</li>" for a, b in chips) + "</ul>"


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
    {dials(st)}
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

# The report's round-3 table, as it stands.
txt = report.read_text()
part = txt[txt.index("## Round 3"):]
part = part[: part.find("\n## ", 3) if "\n## " in part[3:] else len(part)]  # that section only
lines = [l for l in part.splitlines() if l.startswith("| ")]
head = [c.strip() for c in lines[0].strip("|").split("|")]
body = [[c.strip() for c in l.strip("|").split("|")] for l in lines[1:]]
keep = [0, 1, 2, 3, 4, 5, 6, 7, 8, 11]
survey = "<table class=survey><thead><tr>" + "".join(f"<th>{head[i]}</th>" for i in keep) + "</tr></thead><tbody>" + "".join(
    "<tr>" + "".join(f"<td>{r[i].capitalize() if i == 0 else r[i]}</td>" for i in keep) + "</tr>" for r in body) + "</tbody></table>"

sheet = """<section class=band id=sheet>
  <h2>Three seeds of every planet</h2>
  <p class=lede>Seeds 1 to 3 of planets 1 to 8, drawn straight from the map files in false colour so the layout reads apart from the shaders: rock light grey, sand tan, clay brown, barren grey-green, water blue, soft rock beige, crystals cyan, slime nests orange rings, the landing a white square. Each tile is labelled with the dials its planet rolled. Planets 1 and 2 are smaller maps drawn at the same width.</p>
  <div class=sheets><figure><img src="sheet/sheet.png" alt="Planets 1-8, seeds 1-3, false colour" loading="lazy"><figcaption>False colour</figcaption></figure><figure><img src="sheet/sheet-grey.png" alt="The same, grey" loading="lazy"><figcaption>The same, grey</figcaption></figure></div>
</section>"""

# The grey thumbnail test in the game (graphics lead): seed 1 of each planet
# at 1 s, before and after the terrain thread's soil materials (#281, #286),
# from PAGE/grey/ (the sheets are made by the step in the report, *Seen*).
if (page / "grey" / "sheet-after.png").exists():
    sheet += """<section class=band id=grey>
  <h2>In the game, in grey</h2>
  <p class=lede>The art direction's test: each biome recognisable from a small grey thumbnail. Seed 1 of planets 1-8 at one second, in the game, 266 x 164 each: first as filmed for this page, then with the terrain thread's new soil materials (sand ripples, cracked clay, ash on barren), which reached main as this page was made.</p>
  <div class=sheets><figure><img src="grey/sheet-before.png" alt="Seed 1 of planets 1-8 in grey, before the soil materials" loading="lazy"><figcaption>Round 3, before the soil materials</figcaption></figure><figure><img src="grey/sheet-after.png" alt="The same with the soil materials" loading="lazy"><figcaption>Round 3 with the soil materials</figcaption></figure></div>
  <figure class=wide><img src="grey/sheet-after-colour.png" alt="The same in colour" loading="lazy"><figcaption>The same in colour. Rows: planets 1-4, then 5-8.</figcaption></figure>
</section>"""

# Again once the ground thread's river look (#292) and the plants (#290)
# reached main, with deep water and plants in the planets (make_grey3.py).
if (page / "grey" / "sheet-water.png").exists():
    sheet += """<section class=band id=water>
  <h2>With deep water, bridges and plants, in grey</h2>
  <p class=lede>The same test after the ground thread's river look (deep water darkening to its middle, plank bridges, water darker than ground) and the plants reached main, and the planets gained deep water, bridges and each biome's plants: seed 1 of planets 1-8 at one second, 266 x 164 each. Planets 5-8 are now twice today's area, drawn at the same width.</p>
  <div class=sheets><figure><img src="grey/sheet-water.png" alt="Seed 1 of planets 1-8 in grey with deep water and plants" loading="lazy"><figcaption>Grey</figcaption></figure><figure><img src="grey/sheet-water-colour.png" alt="The same in colour" loading="lazy"><figcaption>Colour. Rows: planets 1-4, then 5-8.</figcaption></figure></div>
</section>"""

nav = "".join(f'<a href="#planet-{d}"><span>{d}</span>{b}</a>' for d, (b, _, _) in enumerate(BIOMES, start=1))

CSS_TEXT = """<style>
/* Layout: a header with the route of eight planets, the contact sheet, then one band a planet, two films side by side with their dials and counts under each. */
:root {
  --bg: #11161b; --panel: #182028; --line: #2a3540; --fg: #e3e9ee; --muted: #93a2ae;
  --slime: #e8a43a; --crystal: #3fc6dc;
  --display: "Chakra Petch", "Segoe UI", system-ui, sans-serif;
  --body: "IBM Plex Sans", "Segoe UI", system-ui, sans-serif;
  --mono: "IBM Plex Mono", ui-monospace, "Cascadia Mono", Consolas, monospace;
  color-scheme: dark;
}
@media (prefers-color-scheme: light) { :root:not([data-theme="dark"]) { --bg: #eef1f3; --panel: #ffffff; --line: #d3dae0; --fg: #172028; --muted: #55636f; --slime: #a8650a; --crystal: #0e7f93; color-scheme: light; } }
:root[data-theme="light"] { --bg: #eef1f3; --panel: #ffffff; --line: #d3dae0; --fg: #172028; --muted: #55636f; --slime: #a8650a; --crystal: #0e7f93; color-scheme: light; }
body { background: var(--bg); color: var(--fg); font: 15px/1.55 var(--body); }
.wrap { max-width: 1160px; margin: 0 auto; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); padding-block: 32px 64px; display: grid; gap: 40px; }
h1, h2 { font-family: var(--display); font-weight: 600; text-wrap: balance; margin: 0; letter-spacing: 0.01em; }
h1 { font-size: clamp(30px, 5vw, 44px); line-height: 1.1; }
h2 { font-size: 24px; line-height: 1.2; }
p { margin: 0; max-width: 68ch; }
code { font-family: var(--mono); font-size: 0.86em; color: var(--crystal); }
.muted, .lede { color: var(--muted); }
.top { display: grid; gap: 14px; }
.top .eyebrow { font: 500 12px/1 var(--mono); text-transform: uppercase; letter-spacing: 0.12em; color: var(--slime); }
.route { display: flex; flex-wrap: wrap; gap: 6px; margin-top: 6px; }
.route a { display: inline-flex; align-items: center; gap: 8px; padding: 6px 10px 6px 6px; border: 1px solid var(--line); border-radius: 6px; color: var(--fg); text-decoration: none; font: 500 13px/1 var(--body); }
.route a span { display: inline-grid; place-items: center; width: 20px; height: 20px; border-radius: 4px; background: var(--line); font: 600 12px/1 var(--mono); }
.route a:last-child span { background: var(--slime); color: var(--bg); }
.route a:hover, .route a:focus-visible { border-color: var(--slime); outline: none; }
.play { display: grid; gap: 6px; padding: 14px 16px; border-left: 3px solid var(--slime); background: var(--panel); }
.planet { display: grid; gap: 16px; padding-top: 24px; border-top: 1px solid var(--line); }
.phead { display: flex; gap: 16px; align-items: flex-start; }
.pnum { flex: none; width: 44px; height: 44px; display: grid; place-items: center; border: 1px solid var(--slime); color: var(--slime); font: 600 22px/1 var(--display); border-radius: 50%; }
.what strong { font-weight: 600; color: var(--fg); }
.what { color: var(--muted); margin-top: 4px; }
.films { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 420px), 1fr)); gap: 20px; }
.film { margin: 0; display: grid; gap: 10px; min-width: 0; }
.film img, .clip img { width: 100%; height: auto; display: block; border-radius: 4px; background: #000; }
.cap-head { display: flex; flex-wrap: wrap; gap: 8px 14px; align-items: baseline; }
.pname { font: 600 17px/1.2 var(--display); }
.nums { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 6px 14px; margin: 0; }
.nums div { min-width: 0; }
.nums dt { font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }
.nums dd { margin: 0; font-variant-numeric: tabular-nums; }
table { border-collapse: collapse; font-variant-numeric: tabular-nums; }
.counts { width: 100%; font-size: 13px; }
th, td { text-align: left; padding: 5px 10px 5px 0; border-bottom: 1px solid var(--line); vertical-align: top; }
th { font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--muted); }
.scroll { overflow-x: auto; }
.survey { min-width: 760px; font-size: 13.5px; }
.band { display: grid; gap: 14px; padding-top: 24px; border-top: 1px solid var(--line); }
.clips { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 420px), 1fr)); gap: 20px; }
.clip { margin: 0; display: grid; gap: 8px; min-width: 0; }
.clip figcaption { color: var(--muted); font-size: 13px; }
.greys { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }
.greys figure, .sheets figure { margin: 0; display: grid; gap: 4px; min-width: 0; }
.greys img, .sheets img { width: 100%; height: auto; display: block; border-radius: 3px; }
.greys figcaption, .sheets figcaption { font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }
.wide { margin: 0; display: grid; gap: 4px; }
.wide img { width: 100%; height: auto; display: block; border-radius: 3px; }
.wide figcaption { font: 500 11px/1.3 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }
.sheets { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(100%, 460px), 1fr)); gap: 20px; }
ul { margin: 0; padding-left: 20px; display: grid; gap: 6px; max-width: 72ch; }
.dials { list-style: none; padding: 0; margin: 0; display: flex; flex-wrap: wrap; gap: 6px; max-width: none; }
.dials li { display: inline-flex; gap: 6px; align-items: baseline; padding: 3px 8px; border: 1px solid var(--line); border-radius: 4px; font-size: 13px; }
.dials li span { font: 500 10.5px/1 var(--mono); text-transform: uppercase; letter-spacing: 0.08em; color: var(--muted); }
@media (max-width: 520px) { .nums { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
@media (prefers-reduced-motion: reduce) { * { scroll-behavior: auto; } }
</style>"""

doc = f"""<title>Planet Biomes Round 3</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@500;600&family=IBM+Plex+Sans:wght@400;500;600&family=IBM+Plex+Mono:wght@400;500&display=swap">
CSS
<div class=wrap>
  <header class=top>
    <div class=eyebrow>Larder World · roguelike run</div>
    <h1>Eight biomes that play differently</h1>
    <p class=lede>From your answers to the brainstorm: each biome is now a range on four dials, rolled for every planet with that biome's odds. <strong>Openness</strong> (open, outcrops, pillars, ridges, rocky rooms, caves), <strong>water</strong> (dry, pools, lakes, rivers with fords, streams), <strong>soil</strong> (plain, sand, dunes, barren bands, oases in barren, clay, cracked clay, mixed) and the <strong>crystal pattern</strong> (scattered, fields round a big crystal, veins, one jackpot, at the oases, richer toward the slime). The order runs open early and mazes late. Below, two seeds of each filmed in the game for two minutes with nobody playing, and three seeds of each as maps.</p>
    <nav class=route aria-label="The run's planets">{nav}</nav>
    <div class=play>
      <div><strong>Play one:</strong> on the start menu, <strong>Random planet</strong> lands on a new planet; the <strong>Planet</strong> option (Options page) picks which of the eight; <strong>N</strong> in the round rolls another.</div>
      <div class=muted>From a command line: <code>larder-world map=planet:6:42</code> (planet 6, seed 42). The map editor opens one too: <code>larder-edit map=planet:6:42</code>.</div>
    </div>
    <div class=play>
      <div><strong>Not in yet:</strong> deep water that stops soldiers (the lakes' and rivers' deep middles are ordinary water until the ground thread's deep water is in the game), plants (the plants thread's new simulation), and the terrain shaders' new looks for each ground.</div>
    </div>
  </header>
  {sheet}
  {''.join(sections)}
  <section class=band>
    <h2>Every planet is checked before you land</h2>
    <p class=lede>A planet that fails a check is rebuilt from the next attempt: every crystal and slime nest reachable from the landing, a crystal close to the landing, the far nest across the map on foot, half the crystals' charge open without breaking soft rock. Over 400 planets (seeds 1-50, planets 1-8) all passed within four attempts, 354 on the first, in 75 ms each (median), once a level. Medians over 50 seeds, ranges in brackets, walks in map cells:</p>
    <div class=scroll>{survey}</div>
  </section>
  <section class=band>
    <h2>Next</h2>
    <ul>
      <li><strong>Deep water, plants and new ground</strong> written into the maps as their threads land them.</li>
      <li><strong>Every strategy can win:</strong> a check that each nest has an open approach and a choke, so gunners, builders and sneakers each have a way in.</li>
      <li><strong>Bigger planets</strong> (today's size by planet 3, six times the area on the core world) after the timing on the GTX 1070.</li>
    </ul>
  </section>
</div>
"""
(page / "index.html").write_text(doc.replace("CSS", CSS_TEXT, 1))
print(page / "index.html")
