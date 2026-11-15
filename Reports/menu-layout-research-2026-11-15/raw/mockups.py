#!/usr/bin/env python3
"""Mockups for the menu, shop and command card options (2026-11-15).

Draws each layout as an SVG in UI units, to scale: the frame is the worst
case, 1000 x 565 units (1920x1080 at 200%), and text is drawn at the
game's sizes in units. `page.html` holds the page; `{{svg:NAME}}` marks
where each drawing goes. Run from anywhere:

    python3 Reports/menu-layout-research-2026-11-15/raw/mockups.py

It writes `../page/index.html`. Text widths are estimated (Inter, about
0.55 em a character), so the script checks every label against its box
and stops on one that would not fit.
"""
import html
import math
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
OUT = HERE.parent / "page" / "index.html"

# The game's UI colours, read off today's renders.
BG = "#0e1318"
PANEL = "#151b22"
TILE = "#1b232c"
EDGE = "#2c3641"
TEXT = "#e7ecf1"
MUTED = "#8e9aa7"
DIM = "#5d6873"
BLUE = "#4ea6e6"
SEL = "#1d3348"
GOLD = "#e6b450"
GOOD = "#6cd08a"
BAD = "#ef7a6a"
KIND = {
    "bullet": "#d9a441",
    "blast": "#e07a3a",
    "bomb": "#e07a3a",
    "gas": "#9474e6",
    "germ": "#5fd45c",
    "flight": "#38b6a6",
    "mod": "#5d84e6",
    "squad": "#8e9aa7",
    "build": "#c8a25a",
    "tool": "#d0705a",
}

CHAR = 0.55  # Inter's mean advance, in em, for mixed text (estimate)


def tw(s, size, weight=400):
    """Estimated width of `s` at `size` units."""
    w = 0.0
    for ch in s:
        if ch in "il.,:;'|!·":
            w += 0.28
        elif ch in "mwMW":
            w += 0.85
        elif ch.isupper() or ch.isdigit():
            w += 0.64
        elif ch == " ":
            w += 0.27
        else:
            w += CHAR
    return w * size * (1.04 if weight >= 600 else 1.0)


def esc(s):
    return html.escape(s, quote=True)


class Svg:
    def __init__(self, w, h, label):
        self.w, self.h, self.label = w, h, label
        self.parts = []
        self.defs = []

    def add(self, s):
        self.parts.append(s)

    def rect(self, x, y, w, h, fill=TILE, stroke=EDGE, r=6, sw=1.0, op=None, dash=None):
        o = f' fill-opacity="{op}"' if op is not None else ""
        d = f' stroke-dasharray="{dash}"' if dash else ""
        s = f' stroke="{stroke}" stroke-width="{sw}"' if stroke else ""
        self.add(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{r}" fill="{fill}"{o}{s}{d}/>')

    def text(self, x, y, s, size=14, fill=TEXT, weight=400, anchor="start", fit=None, italic=False):
        """`y` is the baseline. `fit` is the width the text must fit."""
        if fit is not None and tw(s, size, weight) > fit + 0.5:
            raise SystemExit(f"{self.label}: '{s}' at {size}u is ~{tw(s, size, weight):.0f}u, box {fit:.0f}u")
        st = ' font-style="italic"' if italic else ""
        self.add(f'<text x="{x:.1f}" y="{y:.1f}" font-size="{size}" fill="{fill}" font-weight="{weight}" text-anchor="{anchor}"{st}>{esc(s)}</text>')

    def wrap(self, x, y, s, width, size=14, fill=TEXT, lead=None, weight=400):
        """Wraps `s` into `width`; returns the y after the last line."""
        lead = lead or round(size * 1.45)
        words, line, lines = s.split(), "", []
        for w in words:
            t = (line + " " + w).strip()
            if tw(t, size, weight) <= width:
                line = t
            else:
                lines.append(line)
                line = w
        if line:
            lines.append(line)
        for i, l in enumerate(lines):
            self.text(x, y + i * lead, l, size, fill, weight, fit=width)
        return y + (len(lines) - 1) * lead

    def line(self, x1, y1, x2, y2, stroke=EDGE, sw=1.0, dash=None):
        d = f' stroke-dasharray="{dash}"' if dash else ""
        self.add(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{stroke}" stroke-width="{sw}"{d}/>')

    def render(self, dish=True, dim=0.0, scrim=False):
        bg = []
        if dish:
            # The live dish behind the menus: a frame of today's start menu
            # (its right side, `dish.webp`), darkened like the game's scrim.
            bg.append(f'<rect width="{self.w}" height="{self.h}" fill="#1c2a1d"/>')
            bg.append(f'<image href="dish.webp" x="0" y="0" width="{self.w}" height="{self.h}" preserveAspectRatio="xMidYMid slice"/>')
            if dim:
                bg.append(f'<rect width="{self.w}" height="{self.h}" fill="{BG}" fill-opacity="{dim}"/>')
            if scrim:
                # The start menu's dark band behind its list, as in the game.
                gid = "scrim-" + re.sub(r"\W", "", self.label)
                bg.append(f'<defs><linearGradient id="{gid}" x1="0" x2="1" y1="0" y2="0"><stop offset="0" stop-color="{BG}" stop-opacity="0.88"/>'
                          f'<stop offset="0.55" stop-color="{BG}" stop-opacity="0.55"/><stop offset="0.8" stop-color="{BG}" stop-opacity="0"/></linearGradient></defs>')
                bg.append(f'<rect width="{self.w}" height="{self.h}" fill="url(#{gid})"/>')
        else:
            bg.append(f'<rect width="{self.w}" height="{self.h}" fill="{BG}"/>')
        body = "\n".join(bg + self.parts)
        return (
            f'<svg class="mock" viewBox="0 0 {self.w} {self.h}" role="img" aria-label="{esc(self.label)}" '
            f'font-family="Inter, system-ui, sans-serif" xmlns="http://www.w3.org/2000/svg">\n{body}\n</svg>'
        )


# ---------------------------------------------------------------- pieces

def icon(s, kind, cx, cy, r=9):
    c = KIND.get(kind, MUTED)
    if kind == "bullet":
        s.add(f'<rect x="{cx-r:.1f}" y="{cy-r*0.38:.1f}" width="{2*r:.1f}" height="{r*0.76:.1f}" rx="{r*0.38:.1f}" fill="none" stroke="{c}" stroke-width="1.8"/>')
    elif kind in ("blast", "bomb", "tool"):
        pts = []
        for i in range(16):
            a = math.pi * i / 8
            rr = r if i % 2 == 0 else r * 0.45
            pts.append(f"{cx+rr*math.cos(a):.1f},{cy+rr*math.sin(a):.1f}")
        s.add(f'<polygon points="{" ".join(pts)}" fill="none" stroke="{c}" stroke-width="1.6"/>')
    elif kind == "gas":
        for dx, dy, rr in [(-0.4, 0.15, 0.45), (0.1, -0.2, 0.55), (0.5, 0.2, 0.4)]:
            s.add(f'<circle cx="{cx+dx*r:.1f}" cy="{cy+dy*r:.1f}" r="{rr*r:.1f}" fill="none" stroke="{c}" stroke-width="1.6"/>')
    elif kind == "germ":
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.5:.1f}" fill="none" stroke="{c}" stroke-width="1.8"/>')
        for i in range(6):
            a = math.pi * i / 3
            s.add(f'<circle cx="{cx+r*0.9*math.cos(a):.1f}" cy="{cy+r*0.9*math.sin(a):.1f}" r="1.6" fill="{c}"/>')
    elif kind == "flight":
        s.add(f'<path d="M{cx-r} {cy} L{cx+r} {cy} M{cx+r*0.4} {cy-r*0.6} L{cx+r} {cy} L{cx+r*0.4} {cy+r*0.6}" fill="none" stroke="{c}" stroke-width="1.8"/>')
    elif kind == "mod":
        s.add(f'<path d="M{cx-r*0.7} {cy+r*0.1} L{cx} {cy-r*0.6} L{cx+r*0.7} {cy+r*0.1} M{cx-r*0.7} {cy+r*0.7} L{cx} {cy} L{cx+r*0.7} {cy+r*0.7}" fill="none" stroke="{c}" stroke-width="1.8"/>')
    elif kind == "gun":
        s.add(f'<path d="M{cx-r} {cy-r*0.25} L{cx+r} {cy-r*0.25} L{cx+r} {cy+r*0.1} L{cx-r*0.1} {cy+r*0.1} L{cx-r*0.35} {cy+r*0.7} L{cx-r*0.75} {cy+r*0.7} L{cx-r*0.55} {cy+r*0.1} L{cx-r} {cy+r*0.1} Z" fill="none" stroke="{TEXT}" stroke-width="1.5"/>')
    else:
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.8:.1f}" fill="none" stroke="{c}" stroke-width="1.7"/>')


def pips(s, x, y, n, of=None, c=BLUE, r=1.9, gap=5.5):
    for i in range(of or n):
        fill = c if i < n else "none"
        s.add(f'<circle cx="{x+i*gap:.1f}" cy="{y:.1f}" r="{r}" fill="{fill}" stroke="{c}" stroke-width="0.9"/>')


def card_tile(s, x, y, w, h, name, kind, cost, price=None, state="", num=None, sub=None, size=12):
    """A gun card: a kind stripe, icon, name, budget pips, price."""
    fill, stroke, sw = TILE, EDGE, 1.0
    if state == "hover":
        fill, stroke, sw = SEL, BLUE, 1.6
    s.rect(x, y, w, h, fill, stroke, 5, sw)
    s.add(f'<rect x="{x+4:.1f}" y="{y+2:.1f}" width="{w-8:.1f}" height="2.2" rx="1" fill="{KIND[kind]}"/>')
    small = w < 140
    icon(s, kind, x + (13 if small else 17), y + h / 2 + 1, 7 if small else 8)
    tx = x + (25 if small else 31)
    if num is not None:
        s.text(x + 5, y + 14, str(num), 10, MUTED)
    box = x + w - 4 - tx
    pc = KIND[kind] if kind != "mod" else BLUE
    if tw(name, size, 500) <= box or " " not in name:
        s.text(tx, y + h / 2 - 2, name, size, TEXT, 500, fit=box)
        pips(s, tx + 2, y + h / 2 + 10, cost, c=pc)
    else:
        # Two lines for a long name on a small tile ("Buckshot gas"), the
        # pips under them.
        a, b = name.rsplit(" ", 1)
        s.text(tx, y + 18, a, size, TEXT, 500, fit=box)
        s.text(tx, y + 31, b, size, TEXT, 500, fit=box - (30 if price else 0))
        pips(s, tx + 2, y + h - 8, cost, c=pc)
    if price is not None:
        s.text(x + w - 6, y + h / 2 + 13, price, 12, BLUE, 500, anchor="end")
    if sub:
        s.text(tx, y + h - 7, sub, 11, MUTED)


def empty_tile(s, x, y, w, h, label="empty", num=None):
    s.rect(x, y, w, h, "none", EDGE, 5, 1.0, dash="4 3")
    if num is not None:
        s.text(x + 5, y + 14, str(num), 10, DIM)
    s.text(x + w / 2, y + h / 2 + 4, label, 11, DIM, anchor="middle")


def up_tile(s, x, y, w, h, name, level, of, price, state="", folder=False):
    """An upgrade: icon, name, level pips, price."""
    fill, stroke, sw = TILE, EDGE, 1.0
    if state == "hover":
        fill, stroke, sw = SEL, BLUE, 1.6
    s.rect(x, y, w, h, fill, stroke, 5, sw)
    icon(s, "squad", x + 14, y + 15, 7)
    s.text(x + 8, y + h - 9, name, 12, TEXT, 500, fit=w - 14)
    if folder:
        s.text(x + w - 8, y + 19, "›", 16, MUTED, anchor="end")
    else:
        pips(s, x + 28, y + 15, level, of)
        s.text(x + w - 6, y + 19, price, 12, BLUE, 500, anchor="end")


def button(s, x, y, w, h, label, primary=False, size=15, key=None):
    if primary:
        s.rect(x, y, w, h, "#3d8fd0", "#62b2ee", 6, 1.0)
    else:
        s.rect(x, y, w, h, "#1e2731", EDGE, 6, 1.0)
    s.text(x + 14, y + h / 2 + size * 0.36, label, size, "#ffffff" if primary else TEXT, 500, fit=w - 20)
    if key:
        s.text(x + w - 8, y + h - 7, key, 10, MUTED, anchor="end")


def top_bar(s, left, right, right_fill=BLUE):
    s.rect(0, 0, s.w, 40, "#0c1116", None, 0, op=0.94)
    s.line(0, 40, s.w, 40, EDGE)
    s.text(24, 26, left, 15, TEXT, 500, fit=s.w - 200)
    s.text(s.w - 24, 27, right, 17, right_fill, 600, anchor="end")


def bottom_bar(s, buttons, hint):
    y = s.h - 50
    s.rect(0, y - 6, s.w, 56, "#0c1116", None, 0, op=0.94)
    s.line(0, y - 6, s.w, y - 6, EDGE)
    x = 24
    for (label, w, primary) in buttons:
        button(s, x, y, w, 38, label, primary)
        x += w + 10
    if hint:
        s.text(s.w - 24, y + 24, hint, 12, MUTED, anchor="end", fit=s.w - 24 - x - 10)


def pane(s, x, y, w, h):
    s.rect(x, y, w, h, PANEL, EDGE, 8, 1.0, op=0.97)


def row_label(s, x, y, label):
    s.text(x, y, label, 11, MUTED, 600)


# ---------------------------------------------------------------- shop

W, H = 1000, 565
LX = 24          # left column
TX = 92          # tiles start (after the row labels)
RX = W - 24      # right edge
PX = 664         # info pane
PW = RX - PX


def bench(s, y0=50):
    """The carried gun's slots and the bag: pinned on every shop layout."""
    s.rect(12, y0 - 4, PX - 24, H - 56 - 8 - y0 + 4, PANEL, None, 8, op=0.55)
    s.text(TX, y0 + 14, "Long gun", 15, TEXT, 600)
    s.text(TX + 74, y0 + 14, "·  budget 8 of 16  ·  3 of 3 slots  ·  fired left to right", 12, MUTED, fit=PX - 16 - TX - 74)
    y = y0 + 22
    row_label(s, LX, y + 30, "GUN")
    tw_, th, gap = 104, 50, 8
    card_tile(s, TX, y, tw_, th, "Thrift", "mod", 0, None, num=1, sub=None)
    card_tile(s, TX + (tw_ + gap), y, tw_, th, "Slug bomb", "bomb", 6, None, num=2)
    card_tile(s, TX + 2 * (tw_ + gap), y, tw_, th, "Standard blast", "blast", 4, None, num=3)
    s.text(TX + 3 * (tw_ + gap) + 4, y + 30, "a gun has 2 to 6 slots", 11, DIM)
    y += th + 8
    row_label(s, LX, y + 30, "BAG")
    card_tile(s, TX, y, tw_, th, "Buckshot gas", "gas", 2, None)
    for i in range(1, 5):
        empty_tile(s, TX + i * (tw_ + gap), y, tw_, th, "")
    s.text(LX, y + 44, "5 at most", 10, DIM)
    return y + th


def info_card(s, name, kind, line, about, extra=(), buy=None, note=None, y0=50):
    pane(s, PX, y0, PW, H - 56 - 8 - y0)
    x = PX + 16
    icon(s, kind, x + 10, y0 + 26, 11)
    s.text(x + 30, y0 + 32, name, 20, TEXT, 600, fit=PW - 50)
    s.text(x, y0 + 56, line, 13, MUTED, fit=PW - 32)
    y = s.wrap(x, y0 + 84, about, PW - 32, 14, TEXT)
    y += 28
    for (k, v, c) in extra:
        s.text(x, y, k, 13, MUTED)
        s.text(PX + PW - 16, y, v, 13, c, 500, anchor="end")
        y += 21
    if note:
        s.wrap(x, y + 6, note, PW - 32, 12, MUTED)
    if buy:
        by = H - 56 - 8 - 54
        button(s, x, by, PW - 32, 40, buy, True)


def gun_compare(s, name, price, stats, about, y0=50):
    pane(s, PX, y0, PW, H - 56 - 8 - y0)
    x = PX + 16
    icon(s, "gun", x + 10, y0 + 26, 11)
    s.text(x + 30, y0 + 32, name, 20, TEXT, 600)
    s.text(x, y0 + 56, f"Gun · {price} points · against your Long gun", 13, MUTED, fit=PW - 32)
    y = s.wrap(x, y0 + 84, about, PW - 32, 14, TEXT) + 30
    s.text(x, y, "", 12, MUTED)
    s.text(PX + PW - 120, y - 2, "this", 11, MUTED, anchor="end")
    s.text(PX + PW - 70, y - 2, "yours", 11, MUTED, anchor="end")
    y += 18
    for (k, a, b, better) in stats:
        s.text(x, y, k, 14, TEXT)
        s.text(PX + PW - 120, y, a, 14, TEXT, 600, anchor="end")
        s.text(PX + PW - 70, y, b, 14, MUTED, anchor="end")
        if better is not None:
            s.text(PX + PW - 16, y, "▲" if better else "▼", 13, GOOD if better else BAD, anchor="end")
        y += 24
    by = H - 56 - 8 - 54
    button(s, x, by, PW - 32, 40, f"Buy  ·  {price} points", True)


CARBINE = [
    ("Range", "19", "35", False),
    ("Rate", "2.2/s", "0.6/s", True),
    ("Spread", "1.1", "0.3", False),
    ("Budget", "15", "16", False),
    ("Slots", "5", "3", True),
]
GOAL = "Planet 4 of 8  ·  Nezanus next  ·  Wipe out the slime: cut it to 2% of its peak"


def shop_board(hover="card"):
    """One board: every offer on screen at once, smaller tiles."""
    s = Svg(W, H, "Shop A at 1000 by 565 units")
    top_bar(s, GOAL, "670 points")
    y = bench(s) + 10
    s.line(TX, y, PX - 16, y, EDGE, 1, "3 3")
    s.text(LX, y + 4, "FOR SALE", 10, DIM, 600)
    y += 8
    tw_, th, gap = 104, 50, 8
    row_label(s, LX, y + 30, "CARDS")
    cards = [("Needle bullet", "bullet", 1, "100"), ("Standard bullet", "bullet", 1, "90"), ("Scorch", "flight", 3, "150"), ("Smother", "flight", 2, "150"), ("Wide", "flight", 2, "80")]
    for i, (n, k, c, p) in enumerate(cards):
        card_tile(s, TX + i * (tw_ + gap), y, tw_, th, n, k, c, p, "hover" if hover == "card" and n == "Scorch" else "")
    y += th + 8
    row_label(s, LX, y + 30, "GUNS")
    gw = (5 * tw_ + 4 * gap - 2 * gap) / 3
    for i, (n, sub, p) in enumerate([("Rifle", "armory", "free"), ("Carbine", "for sale", "290"), ("Carbine", "for sale", "240")]):
        x = TX + i * (gw + gap)
        st = "hover" if hover == "gun" and p == "290" else ""
        s.rect(x, y, gw, th, SEL if st else TILE, BLUE if st else EDGE, 5, 1.6 if st else 1.0)
        icon(s, "gun", x + 18, y + th / 2, 10)
        s.text(x + 36, y + 21, n, 13, TEXT, 500)
        s.text(x + 36, y + 38, sub, 11, MUTED)
        s.text(x + gw - 8, y + 38, p, 12, BLUE, 500, anchor="end")
    y += th + 8
    row_label(s, LX, y + 30, "SQUAD")
    ups = [("Armour", 1, 4, "240"), ("Heavier shots", 0, 4, "120"), ("Faster trigger", 0, 4, "120"), ("Longer barrels", 0, 3, "100"), ("Bigger landing", 0, 4, "150"),
           ("Supply drop", 0, 4, "100"), ("Quick hands", 0, 3, "100"), ("More cards", 0, 2, "100")]
    for i, (n, l, o, p) in enumerate(ups):
        r, c = divmod(i, 5)
        up_tile(s, TX + c * (tw_ + gap), y + r * (th + 8), tw_, th, n, l, o, p)
    up_tile(s, TX + 3 * (tw_ + gap), y + th + 8, tw_, th, "Buildings", 0, 0, "", folder=True)
    up_tile(s, TX + 4 * (tw_ + gap), y + th + 8, tw_, th, "Tools", 0, 0, "", folder=True)
    end = y + 2 * th + 8
    s.text(TX, end + 18, "Buildings and Tools open in place of these two rows, with a Back tile", 11, DIM, fit=PX - 16 - TX)
    if hover == "card":
        info_card(s, "Scorch", "flight", "Bullet modifier  ·  150 points", "Bullets burn a line the slime avoids.",
                  [("Budget", "3", TEXT), ("Your gun after it", "11 of 16", GOOD), ("Free slot", "none: goes to the bag", MUTED)],
                  "Buy  ·  150 points", "Or drag it onto a slot: it takes that card's place, which goes to the bag.")
    else:
        gun_compare(s, "Carbine", "290", CARBINE, "Fast and short, a little wild. More slots.")
    bottom_bar(s, [("Land on the next planet", 220, True), ("Give up the run", 150, False)], "Click selects  ·  Enter buys  ·  drag into the gun  ·  Esc back")
    return s.render(dim=0.74)


def shop_tabs(tab="cards"):
    """Bench pinned, a tabbed counter below it."""
    s = Svg(W, H, "Shop B at 1000 by 565 units")
    top_bar(s, GOAL, "670 points")
    y = bench(s) + 12
    tabs = [("Cards", "5", "cards"), ("Guns", "3", "guns"), ("Squad", "8", "squad"), ("Build and tools", "12", "build")]
    x = LX
    for (n, c, k) in tabs:
        w = tw(n, 14, 500) + tw(c, 12) + 34
        on = k == tab
        s.rect(x, y, w, 30, SEL if on else "#141a20", BLUE if on else EDGE, 6, 1.4 if on else 1.0)
        s.text(x + 12, y + 20, n, 14, TEXT if on else MUTED, 600 if on else 500)
        s.text(x + w - 10, y + 20, c, 12, BLUE if on else DIM, 500, anchor="end")
        x += w + 8
    s.text(PX - 16, y + 20, "1 2 3 4 switch", 11, DIM, anchor="end")
    y += 38
    ch = H - 56 - 8 - y
    s.rect(LX, y, PX - 16 - LX, ch, PANEL, EDGE, 8, 1.0, op=0.9)
    if tab == "cards":
        cards = [("Needle bullet", "bullet", 1, "100", "bullet · fast, thin"), ("Standard bullet", "bullet", 1, "90", "bullet"),
                 ("Scorch", "flight", 3, "150", "modifier"), ("Smother", "flight", 2, "150", "modifier"), ("Wide", "flight", 2, "80", "modifier")]
        cw, chh, gap = 190, 62, 10
        for i, (n, k, c, p, sub) in enumerate(cards):
            r, col = divmod(i, 3)
            card_tile(s, LX + 12 + col * (cw + gap), y + 12 + r * (chh + gap), cw, chh, n, k, c, p,
                      "hover" if n == "Scorch" else "", sub=None, size=14)
        s.text(LX + 12, y + ch - 12, "room for up to 9 cards on offer here", 11, DIM)
        info_card(s, "Scorch", "flight", "Bullet modifier  ·  150 points", "Bullets burn a line the slime avoids.",
                  [("Budget", "3", TEXT), ("Your gun after it", "11 of 16", GOOD), ("Free slot", "none: goes to the bag", MUTED)],
                  "Buy  ·  150 points", "Or drag it onto a slot: it takes that card's place, which goes to the bag.")
    elif tab == "guns":
        cols = [("Gun", LX + 16, "start"), ("Range", 300, "end"), ("Rate", 362, "end"), ("Spread", 430, "end"), ("Budget", 498, "end"), ("Slots", 552, "end"), ("Price", 620, "end")]
        for (n, cx, a) in cols:
            s.text(cx, y + 24, n, 12, MUTED, 600, anchor=a)
        rows = [
            ("Long gun (yours)", ["35", "0.6/s", "0.3", "16", "3"], "", None),
            ("Rifle (armory)", ["24", "1.0/s", "0.5", "10", "4"], "free", [False, True, False, False, True]),
            ("Carbine", ["19", "2.2/s", "1.1", "15", "5"], "290", [False, True, False, False, True]),
            ("Carbine", ["18", "3.6/s", "1.8", "10", "5"], "240", [False, True, False, False, True]),
        ]
        ry = y + 34
        for i, (n, vals, p, better) in enumerate(rows):
            hov = i == 2
            if i == 0:
                s.rect(LX + 6, ry, PX - 16 - LX - 12, 34, "#18222c", EDGE, 5)
            if hov:
                s.rect(LX + 6, ry, PX - 16 - LX - 12, 34, SEL, BLUE, 5, 1.6)
            icon(s, "gun", LX + 26, ry + 17, 9)
            s.text(LX + 44, ry + 22, n, 14, TEXT, 500)
            for j, v in enumerate(vals):
                c = TEXT if better is None else (GOOD if better[j] else BAD)
                s.text(cols[j + 1][1], ry + 22, v, 14, c, 500, anchor="end")
            s.text(620, ry + 22, p, 13, BLUE, 500, anchor="end")
            ry += 40
        s.text(LX + 16, ry + 14, "green: better than yours  ·  red: worse", 11, DIM)
        gun_compare(s, "Carbine", "290", CARBINE, "Fast and short, a little wild. More slots.")
    bottom_bar(s, [("Land on the next planet", 220, True), ("Give up the run", 150, False)], "Click selects  ·  Enter buys  ·  drag into the gun  ·  Esc back")
    return s.render(dim=0.74)


def armory():
    """The gun range's armory: same frame, tabs, and the bullet grid."""
    s = Svg(W, H, "Armory at 1000 by 565 units")
    top_bar(s, "The gun range's armory  ·  everything is free  ·  guns rolled as on planet 3", "Free")
    s.text(TX, 64, "Rifle", 15, TEXT, 600)
    s.text(TX + 48, 64, "·  budget 4 of 10  ·  2 of 4 slots", 12, MUTED)
    y = 72
    tw_, th, gap = 104, 50, 8
    row_label(s, LX, y + 30, "GUN")
    card_tile(s, TX, y, tw_, th, "Needle gas", "gas", 3, None, num=1)
    card_tile(s, TX + tw_ + gap, y, tw_, th, "Wide", "flight", 2, None, num=2)
    empty_tile(s, TX + 2 * (tw_ + gap), y, tw_, th, "empty", 3)
    empty_tile(s, TX + 3 * (tw_ + gap), y, tw_, th, "empty", 4)
    y += th + 8
    row_label(s, LX, y + 30, "BAG")
    for i in range(5):
        empty_tile(s, TX + i * (tw_ + gap), y, tw_, th, "")
    y += th + 12
    x = LX
    for (n, c, on) in [("Guns", "8", False), ("Bullets", "20", True), ("Modifiers", "14", False)]:
        w = tw(n, 14, 500) + tw(c, 12) + 34
        s.rect(x, y, w, 30, SEL if on else "#141a20", BLUE if on else EDGE, 6, 1.4 if on else 1.0)
        s.text(x + 12, y + 20, n, 14, TEXT if on else MUTED, 600 if on else 500)
        s.text(x + w - 10, y + 20, c, 12, BLUE if on else DIM, 500, anchor="end")
        x += w + 8
    y += 38
    ch = H - 56 - 8 - y
    s.rect(LX, y, PX - 16 - LX, ch, PANEL, EDGE, 8, 1.0, op=0.9)
    shapes = ["Standard", "Slug", "Needle", "Buckshot"]
    loads = [("bullet", "Bullet"), ("blast", "Blast"), ("bomb", "Bomb"), ("gas", "Gas"), ("germ", "Germ")]
    cost = {"bullet": [1, 2, 1, 1], "blast": [3, 3, 4, 2], "bomb": [4, 6, 6, 3], "gas": [3, 6, 3, 1], "germ": [4, 6, 5, 3]}
    hx, cw, rh = LX + 92, 102, 44
    for j, (k, n) in enumerate(loads):
        s.text(hx + j * cw + (cw - 8) / 2, y + 20, n, 12, KIND[k], 600, anchor="middle")
    for i, sh in enumerate(shapes):
        ry = y + 28 + i * (rh + 6)
        s.text(LX + 14, ry + rh / 2 + 5, sh, 13, TEXT, 500)
        for j, (k, n) in enumerate(loads):
            x = hx + j * cw
            hov = sh == "Needle" and k == "gas"
            s.rect(x, ry, cw - 8, rh, SEL if hov else TILE, BLUE if hov else EDGE, 5, 1.6 if hov else 1.0)
            s.add(f'<rect x="{x+4:.1f}" y="{ry+2:.1f}" width="{cw-16:.1f}" height="2" rx="1" fill="{KIND[k]}"/>')
            icon(s, k, x + 18, ry + rh / 2 + 1, 8)
            pips(s, x + 34, ry + rh / 2 + 1, cost[k][i], c=KIND[k])
    info_card(s, "Needle gas", "gas", "Bullet  ·  budget 3  ·  free", "Puts slime where it hits to sleep. Needle: its own damage, speed, range and spread.",
              [("Budget", "3", TEXT), ("Your gun after it", "7 of 10", GOOD)], "Put in the gun", "Rows are the bullet's shape, columns what it does. Drag a tile onto a slot, or click it and press Enter.")
    # Bottom: two buttons and the five range tools.
    yb = H - 50
    s.rect(0, yb - 6, W, 56, "#0c1116", None, 0, op=0.94)
    s.line(0, yb - 6, W, yb - 6, EDGE)
    button(s, 24, yb, 170, 38, "Back to the range", True)
    button(s, 204, yb, 140, 38, "Leave the range")
    x = 372
    for n in ["New rolls", "Deeper", "Shallower", "Empty the gun", "Empty the bag"]:
        w = tw(n, 12, 500) + 22
        s.rect(x, yb + 4, w, 30, "#1e2731", EDGE, 5)
        s.text(x + 11, yb + 23, n, 12, TEXT, 500)
        x += w + 6
    assert x < W - 20, x
    return s.render(dim=0.74)


# ---------------------------------------------------------------- menus

SUBS = [
    ("Run", "Planet after planet toward the slime's home world; upgrades between"),
    ("Sandbox", "Free play on the map, with today's defaults"),
    ("Campaign", "8 missions, one new verb each"),
    ("Random planet", "A new generated planet, planet 1 of a run (Options: Planet)"),
    ("Gun range", "Every gun, card and bullet, free: build a gun and shoot the slime"),
]
UTIL = [("Options", "Screen, controls, the mould's rules and keys"), ("Edit maps", "Draw a map in the map editor"), ("Quit", "")]


def title(s):
    s.text(32, 62, "Larder World", 36, TEXT, 700)
    s.text(32, 86, "A mould that grows, feeds and fights back", 14, MUTED)


def menu_list():
    """Name-only rows; one caption box beside the list."""
    s = Svg(W, H, "Main menu A at 1000 by 565 units")
    title(s)
    y, x, w, rh = 108, 32, 280, 44
    rows = [(n, sub) for n, sub in SUBS] + [None] + UTIL[:2] + [None] + UTIL[2:]
    focus_y = None
    for r in rows:
        if r is None:
            y += 12
            continue
        n, sub = r
        on = n == "Run"
        s.rect(x, y, w, rh - 4, "#3d8fd0" if on else PANEL, "#62b2ee" if on else EDGE, 6, 1.0, op=None if on else 0.92)
        s.text(x + 16, y + 26, n, 18, "#ffffff" if on else TEXT, 600 if on else 500, fit=w - 24)
        if on:
            focus_y = y
        y += rh
    # The caption box beside the focused row.
    cx, cw = x + w + 18, 360
    s.rect(cx, focus_y, cw, 84, PANEL, EDGE, 8, 1.0, op=0.95)
    s.add(f'<path d="M{cx} {focus_y+14} L{cx-8} {focus_y+20} L{cx} {focus_y+26}" fill="{PANEL}" stroke="{EDGE}"/>')
    s.text(cx + 16, focus_y + 28, "Run", 18, TEXT, 600)
    s.wrap(cx + 16, focus_y + 52, SUBS[0][1], cw - 32, 14, "#c9d2db")
    s.text(32, H - 18, "Up/Down and Enter, or click  ·  F1 keys", 12, MUTED)
    s.text(cx, focus_y + 102, "the box moves with the mouse or the keys", 11, DIM)
    return s.render(dim=0.1, scrim=True)


def menu_stack():
    """The ways to play as cards with their subtitles; a utility row."""
    s = Svg(W, H, "Main menu B at 1000 by 565 units")
    title(s)
    y, x, w, rh = 106, 32, 540, 66
    for n, sub in SUBS:
        on = n == "Run"
        s.rect(x, y, w, rh - 6, "#3d8fd0" if on else PANEL, "#62b2ee" if on else EDGE, 7, 1.0, op=None if on else 0.92)
        s.text(x + 16, y + 25, n, 17, "#ffffff" if on else TEXT, 600)
        s.text(x + 16, y + 46, sub, 13, "#e6f2fc" if on else MUTED, fit=w - 32)
        y += rh
    y += 6
    bx = x
    for n, w_ in [("Options", 130), ("Edit maps", 130)]:
        button(s, bx, y, w_, 38, n, size=15)
        bx += w_ + 10
    button(s, x + w - 100, y, 100, 38, "Quit", size=15)
    s.text(32, H - 18, "Up/Down and Enter, or click  ·  F1 keys", 12, MUTED)
    return s.render(dim=0.1, scrim=True)


def options():
    """Tabs, one-line rows and a note pane."""
    s = Svg(W, H, "Options at 1000 by 565 units")
    s.rect(0, 0, W, 48, "#0c1116", None, 0, op=0.94)
    s.line(0, 48, W, 48, EDGE)
    s.text(24, 31, "Options", 20, TEXT, 700)
    x = 130
    for n, on in [("Game", False), ("Planet", False), ("The mould", True), ("Keys", False)]:
        w = tw(n, 14, 500) + 26
        s.rect(x, 9, w, 30, SEL if on else "#141a20", BLUE if on else EDGE, 6, 1.4 if on else 1.0)
        s.text(x + 13, 29, n, 14, TEXT if on else MUTED, 600 if on else 500)
        x += w + 8
    button(s, W - 24 - 110, 6, 110, 36, "Back", key="Esc")
    rows = [("Panic", "2", False), ("Wither", "0.01", False), ("Shared food", "0.05", False), ("Condense", "0", False),
            ("Breakaway", "0", False), ("Thicken", "2", False), ("Surge", "1800", True), ("Flinch", "0.9", False)]
    lx, lw, y, rh = 24, 540, 62, 44
    s.rect(lx, y, lw, len(rows) * rh + 8, PANEL, EDGE, 8, 1.0, op=0.92)
    y += 4
    for (n, v, changed) in rows:
        on = n == "Surge"
        if on:
            s.rect(lx + 4, y, lw - 8, rh - 4, SEL, BLUE, 6, 1.6)
        s.text(lx + 18, y + 26, n, 16, TEXT, 500)
        if changed:
            s.add(f'<circle cx="{lx + 26 + tw(n, 16, 500):.1f}" cy="{y+21}" r="3" fill="{GOLD}"/>')
        bx = lx + lw - 200
        s.rect(bx, y + 7, 26, 26, "#1e2731", EDGE, 5)
        s.text(bx + 13, y + 25, "<", 14, TEXT, 500, anchor="middle")
        s.text(bx + 92, y + 26, v, 15, GOLD if changed else TEXT, 600, anchor="middle")
        s.rect(bx + 158, y + 7, 26, 26, "#1e2731", EDGE, 5)
        s.text(bx + 171, y + 25, ">", 14, TEXT, 500, anchor="middle")
        y += rh
    # The note pane.
    px, pw = 584, W - 24 - 584
    pane(s, px, 62, pw, len(rows) * rh + 8)
    s.text(px + 16, 94, "Surge", 20, TEXT, 600)
    yy = s.wrap(px + 16, 122, "When the slime has eaten enough from its crystals, the slime near your soldiers and buildings creeps toward them, then lunges.", pw - 32, 14, TEXT)
    yy = s.wrap(px + 16, yy + 30, "The value is the longest wait between surges, in steps: 1800 is 60 s. 0 is off.", pw - 32, 14, "#c9d2db")
    s.text(px + 16, yy + 34, "Now 1800  ·  default 900 (30 s)", 13, MUTED)
    button(s, px + 16, yy + 50, 190, 36, "Back to the default", key="Bksp")
    s.text(24, H - 18, "Up/Down pick  ·  Left/Right change  ·  Tab next tab  ·  Esc back", 12, MUTED)
    return s.render(dim=0.74)


# ---------------------------------------------------------------- the card

CW, CH, CG = 116, 42, 6


def cmd_icon(s, name, cx, cy, c=TEXT):
    """A small line icon for a command, close to the game's own."""
    st = f'fill="none" stroke="{c}" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"'
    n = name.split(":")[0].lower()
    r = 7
    if n.startswith("attack"):
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.65:.1f}" {st}/><path d="M{cx-r} {cy}H{cx-r*0.35} M{cx+r*0.35} {cy}H{cx+r} M{cx} {cy-r}V{cy-r*0.35} M{cx} {cy+r*0.35}V{cy+r}" {st}/>')
    elif n.startswith("burn") or n.startswith("det"):
        s.add(f'<path d="M{cx} {cy-r} C{cx+r} {cy-r*0.2} {cx+r*0.8} {cy+r} {cx} {cy+r} C{cx-r*0.8} {cy+r} {cx-r} {cy} {cx-r*0.3} {cy-r*0.3} C{cx-r*0.2} {cy+r*0.2} {cx+r*0.2} {cy+r*0.1} {cx} {cy-r}Z" {st}/>')
    elif n.startswith("drain") or n.startswith("pump"):
        pts = " ".join(f"{cx+r*math.cos(math.pi/3*i+math.pi/6):.1f},{cy+r*math.sin(math.pi/3*i+math.pi/6):.1f}" for i in range(6))
        s.add(f'<polygon points="{pts}" {st}/>')
        if n.startswith("pump"):
            s.add(f'<path d="M{cx} {cy-r*0.45}V{cy+r*0.45} M{cx-r*0.35} {cy+r*0.1}L{cx} {cy+r*0.45}L{cx+r*0.35} {cy+r*0.1}" {st}/>')
    elif n.startswith("hold") or n.startswith("back") or n.startswith("cancel"):
        s.add(f'<path d="M{cx-r*0.7} {cy-r*0.7}L{cx+r*0.7} {cy+r*0.7}M{cx+r*0.7} {cy-r*0.7}L{cx-r*0.7} {cy+r*0.7}" {st}/>')
    elif n.startswith("defend") or n.startswith("fire"):
        s.add(f'<path d="M{cx-r} {cy}Q{cx} {cy-r*1.1} {cx+r} {cy}Q{cx} {cy+r*1.1} {cx-r} {cy}Z" {st}/><circle cx="{cx}" cy="{cy}" r="{r*0.3:.1f}" {st}/>')
    elif n.startswith("free"):
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.9:.1f}" {st}/><circle cx="{cx}" cy="{cy}" r="{r*0.35:.1f}" {st}/>')
    elif n.startswith("heal"):
        s.add(f'<path d="M{cx} {cy-r*0.8}V{cy+r*0.8}M{cx-r*0.8} {cy}H{cx+r*0.8}" {st} stroke-width="2.2"/>')
    elif n.startswith("buy"):
        s.add(f'<circle cx="{cx-r*0.2}" cy="{cy-r*0.4}" r="{r*0.38:.1f}" {st}/><path d="M{cx-r*0.9} {cy+r*0.8}Q{cx-r*0.2} {cy-r*0.1} {cx+r*0.5} {cy+r*0.8} M{cx+r*0.7} {cy-r*0.7}V{cy-r*0.1}M{cx+r*0.4} {cy-r*0.4}H{cx+r}" {st}/>')
    elif n.startswith("build") or n.startswith("turret"):
        s.add(f'<path d="M{cx-r*0.7} {cy+r*0.8}H{cx+r*0.7} M{cx-r*0.4} {cy+r*0.8}V{cy-r*0.1}H{cx+r*0.4}V{cy+r*0.8} M{cx} {cy-r*0.1}L{cx+r*0.6} {cy-r*0.9}" {st}/>')
    elif n.startswith("tools") or n.startswith("blast") or n.startswith("mine"):
        s.add(f'<circle cx="{cx}" cy="{cy+r*0.2}" r="{r*0.65:.1f}" {st}/><path d="M{cx} {cy-r*0.45}V{cy-r*0.85}H{cx+r*0.5}" {st}/>')
    elif n.startswith("guards"):
        s.add(f'<path d="M{cx} {cy-r}L{cx+r*0.8} {cy-r*0.6}V{cy}Q{cx+r*0.8} {cy+r*0.7} {cx} {cy+r}Q{cx-r*0.8} {cy+r*0.7} {cx-r*0.8} {cy}V{cy-r*0.6}Z" {st}/>')
    elif n.startswith("wall") or n.startswith("lay"):
        s.add(f'<path d="M{cx-r} {cy-r*0.6}H{cx+r}V{cy+r*0.6}H{cx-r}Z M{cx-r} {cy}H{cx+r} M{cx-r*0.3} {cy-r*0.6}V{cy} M{cx+r*0.3} {cy}V{cy+r*0.6}" {st}/>')
    elif n.startswith("lure"):
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.35:.1f}" {st}/><path d="M{cx-r*0.7} {cy-r*0.7}Q{cx-r} {cy} {cx-r*0.7} {cy+r*0.7} M{cx+r*0.7} {cy-r*0.7}Q{cx+r} {cy} {cx+r*0.7} {cy+r*0.7}" {st}/>')
    elif n.startswith("siren"):
        s.add(f'<path d="M{cx-r*0.7} {cy+r*0.5}Q{cx-r*0.7} {cy-r} {cx} {cy-r}Q{cx+r*0.7} {cy-r} {cx+r*0.7} {cy+r*0.5}Z M{cx-r} {cy+r*0.8}H{cx+r}" {st}/>')
    elif n.startswith("germ"):
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.45:.1f}" {st}/><circle cx="{cx+r*0.8}" cy="{cy-r*0.5}" r="1.4" fill="{c}"/><circle cx="{cx-r*0.8}" cy="{cy+r*0.5}" r="1.4" fill="{c}"/><circle cx="{cx-r*0.6}" cy="{cy-r*0.7}" r="1.4" fill="{c}"/>')
    elif n.startswith("sleep"):
        s.add(f'<path d="M{cx-r*0.6} {cy-r*0.6}H{cx+r*0.6}L{cx-r*0.6} {cy+r*0.6}H{cx+r*0.6}" {st}/>')
    else:
        s.add(f'<circle cx="{cx}" cy="{cy}" r="{r*0.6:.1f}" {st}/>')


def cmd(s, x, y, name, key, cost=None, on=False, empty=False, moved=False, w=CW):
    if empty:
        s.rect(x, y, w, CH, "none", EDGE, 6, 1.0, dash="4 3")
        return
    s.rect(x, y, w, CH, SEL if on else "#1b222b", BLUE if on else ("#c9a24a" if moved else EDGE), 6, 1.6 if on or moved else 1.0)
    cmd_icon(s, name, x + 16, y + CH / 2)
    if cost:
        s.text(x + 30, y + 18, name, 12, TEXT, 500, fit=w - 36)
        s.text(x + 30, y + 33, cost, 11, BLUE, 500)
    else:
        s.text(x + 30, y + 26, name, 12, TEXT, 500, fit=w - 36)
    s.text(x + w - 6, y + CH - 6, key, 10, MUTED, anchor="end")


def card(label, cells, cols=6, note=None):
    rows = (len(cells) + cols - 1) // cols
    w = 2 * 10 + cols * CW + (cols - 1) * CG
    h = 2 * 10 + rows * CH + (rows - 1) * CG + (18 if note else 0)
    s = Svg(w, h, label)
    s.rect(0, 0, w, h, PANEL, EDGE, 10)
    for i, c in enumerate(cells):
        r, k = divmod(i, cols)
        x, y = 10 + k * (CW + CG), 10 + r * (CH + CG)
        if c is None:
            cmd(s, x, y, "", "", empty=True)
        else:
            cmd(s, x, y, *c)
    if note:
        s.text(10, h - 7, note, 11, MUTED)
    return s.render(dish=False)


MOVED = (None, False, False, True)


def cards():
    out = {}
    out["card_one"] = card("Card 1", [
        ("Attack-move", "R"), ("Burn line", "F"), ("Drain crystal", "G"), ("Hold fire", "Z"), ("Defend", "X", None, True), ("Free fire", "C"),
        ("Heal", "L") + MOVED, ("Buy soldier", "B", "400"), ("Pump", "U", "500", False, False, True), ("Build", "Q"), ("Tools", "E"), ("Guards: some", "I") + MOVED,
    ], note="gold edge: moved here (Heal and Guards from the squad panel, Pump from the Build page)")
    out["card_two"] = card("Card 2", [
        ("Attack-move", "R"), ("Burn line", "F"), ("Drain crystal", "G"), ("Fire: defend", "X", None, True), ("Heal", "L") + MOVED,
        ("Buy soldier", "B", "400"), ("Pump", "U", "500", False, False, True), ("Build", "Q"), ("Tools", "E"), ("Guards: some", "I") + MOVED,
    ], cols=5, note="one fire button: a click goes hold, defend, free; Z, X and C still pick one")
    out["build_today"] = card("Build page today", [
        ("Turret", "T", "500"), ("Lure", "L", "300"), ("Pump", "U", "500"), None, ("Lures on/off", "J"), ("Siren", "Y", "300"),
        None, None, ("Cancel a build", "C"), None, None, ("Back", "Q"),
    ])
    out["build_new"] = card("Build page proposed", [
        ("Turret", "T", "500"), ("Pump", "U", "500"), ("Wall", "V", "20/cell", False, False, True), ("Lure", "L", "300"), ("Siren", "Y", "300"), None,
        ("Lures on/off", "J"), None, None, None, None, ("Back", "Q"),
    ], note="row 1: what you place, most built first; row 2: the switch and Back (Outpost fills the gap in RTS mode)")
    out["build_tools"] = card("Build and tools on one page", [
        ("Turret", "T", "500"), ("Pump", "U", "500"), ("Wall", "V", "20/cell", False, False, True), ("Lure", "L", "300"), ("Siren", "Y", "300"), ("Lures on/off", "J"),
        ("Blast", "B", "60", False, False, True), ("Det cord", "C", "3/cell", False, False, True), ("Mine", "R", "80", False, False, True), ("Germ", "G", "300", False, False, True), ("Sleep gas", "F", "150", False, False, True), ("Back", "Q"),
    ], note="gold edge: the Tools page's five, with the keys they have today; the main card's Tools tile is freed")
    return out


def main():
    svgs = {
        "menu_a": menu_list(),
        "menu_b": menu_stack(),
        "shop_a_card": shop_board("card"),
        "shop_a_gun": shop_board("gun"),
        "shop_b_card": shop_tabs("cards"),
        "shop_b_gun": shop_tabs("guns"),
        "armory": armory(),
        "options": options(),
    }
    svgs.update(cards())
    page = (HERE / "page.html").read_text()
    def sub(m):
        return svgs[m.group(1)]
    out = re.sub(r"\{\{svg:(\w+)\}\}", sub, page)
    missing = set(svgs) - set(re.findall(r"\{\{svg:(\w+)\}\}", page))
    if missing:
        print("not on the page:", sorted(missing))
    OUT.write_text(out)
    print(f"wrote {OUT} ({len(out)//1024} KB)")


if __name__ == "__main__":
    main()
