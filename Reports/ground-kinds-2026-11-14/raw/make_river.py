#!/usr/bin/env python3
"""The River map (new ground kinds, 2026-11-14), version 2 after the owner's
18:01Z look ("It is just a blue curved line on the map. It doesn't look
professional"): a river whose course and width wander, a deep channel inside
a shallow margin, sand bars on the inside of bends and clay cut-banks on the
outside, two wide shallow fords with sand bars, an oxbow pool, a shallow
creek from the north, and a plank bridge in the middle on sand abutments.

  python3 make_river.py Reports/larder-maps-2026-11-11/maps

Ground bytes (`mould::map`): 0x01 wet (shallow), 0x41 deep (soil 4 + wet),
0x50 bridge (soil 5), soil kinds 1 sand, 2 clay in the high nibble.
Deterministic: fixed seeds, no randomness from the clock. Checks that the
landing reaches every crystal and the mould start on foot (as
`Map::cut_off`) before writing."""
import math
import sys
from collections import deque
from pathlib import Path

from PIL import Image

W, H = 1024, 640
WET, DEEP, BRIDGE, SAND, CLAY, ROCK = 0x01, 0x41, 0x50, 0x10, 0x20, 0x08


def hash2(i, j, s):
    h = (i * 374761393 + j * 668265263 + s * 2246822519) & 0xFFFFFFFF
    h = ((h ^ (h >> 13)) * 1274126177) & 0xFFFFFFFF
    return ((h ^ (h >> 16)) & 0xFFFFFF) / float(0xFFFFFF)


def vnoise(x, y, s):
    i, j = math.floor(x), math.floor(y)
    fx, fy = x - i, y - j
    ux, uy = fx * fx * (3 - 2 * fx), fy * fy * (3 - 2 * fy)
    a, b = hash2(i, j, s), hash2(i + 1, j, s)
    c, d = hash2(i, j + 1, s), hash2(i + 1, j + 1, s)
    return (a + (b - a) * ux) * (1 - uy) + (c + (d - c) * ux) * uy


def fbm(x, y, s, oct=4):
    v, amp, f, tot = 0.0, 1.0, 1.0, 0.0
    for o in range(oct):
        v += amp * vnoise(x * f, y * f, s + o * 17)
        tot += amp
        amp *= 0.5
        f *= 2.0
    return v / tot  # 0..1


def bump(x, c, r, flat=0.0):
    """1 within `flat` of c, smoothly to 0 at distance r."""
    t = min(max(abs(x - c) - flat, 0.0) / max(r - flat, 1e-6), 1.0)
    return 0.5 + 0.5 * math.cos(math.pi * t)


FORDS = (215.0, 820.0)
BRIDGE_X = (500, 524)


def centre(x):
    return 322 + 74 * math.sin(x / 170.0 + 0.9) + 18 * math.sin(x / 61.0 + 2.1) + 26 * (fbm(x / 90.0, 3.3, 5) - 0.5)


def slope(x):
    return (centre(x + 1) - centre(x - 1)) / 2.0


def half_deep(x):
    """Half width of the deep channel; pinched to 0 at the fords."""
    hw = 28 + 22 * fbm(x / 70.0, 1.7, 9)
    for f in FORDS:
        hw *= 1.0 - bump(x, f, 105, 32)
    return hw


def half_shallow(x):
    """Half width of the whole river (deep + shallow margin)."""
    hw = half_deep(x) + 9 + 10 * fbm(x / 40.0, 8.1, 13)
    for f in FORDS:
        hw += 22 * bump(x, f, 110)  # fords spread wide and shallow
    return hw


def main(out):
    g = bytearray(W * H)
    for x in range(W):
        c, s, hd, hs = centre(x), slope(x), half_deep(x), half_shallow(x)
        # Distance measured across the stream, not straight down.
        k = math.sqrt(1 + s * s)
        for y in range(H):
            d = (y - c) / k
            ad = abs(d)
            edge = 5 * (fbm(x / 9.0, y / 9.0, 21) - 0.5)  # ragged banks
            i = y * W + x
            if ad < hd + edge * 0.6:
                g[i] = DEEP
            elif ad < hs + edge:
                g[i] = WET
                # Sand bars in the fords' shallows.
                if any(bump(x, f, 60) > 0.2 for f in FORDS) and fbm(x / 14.0, y / 14.0, 33) > 0.62:
                    g[i] = SAND | WET
            else:
                # Banks: sand on the inside of a bend, clay cut-banks on
                # the outside (the bend's side from the slope's change).
                # Banks: sand on the inside of a bend (deposits), clay
                # cut-banks on the outside, mixed by noise so neither
                # is a stripe; widest near the fords.
                bank = ad - hs
                curve = centre(x + 50) + centre(x - 50) - 2 * c  # >0: bend opens downward
                inside = (d > 0) == (curve < 0)
                lean = max(-1.0, min(1.0, curve / 30.0)) * (1 if d > 0 else -1)
                width = 8 + 26 * fbm(x / 34.0, y / 34.0, 41) + 20 * max(bump(x, f, 120) for f in FORDS)
                if bank < width:
                    n = fbm(x / 70.0, y / 70.0, 45) - 0.2 * lean
                    g[i] = SAND if n < 0.5 else CLAY
    # A pond north-east: deep middle, shallow rim, clay and sand round it.
    ox, oy = 760, 110
    for y in range(max(0, oy - 80), oy + 80):
        for x in range(ox - 120, ox + 120):
            r = math.hypot((x - ox) / 88.0, (y - oy) / 46.0) + 0.45 * (fbm(x / 40.0, y / 40.0, 51) - 0.5)
            i = y * W + x
            if r < 0.62:
                g[i] = DEEP
            elif r < 1.0:
                g[i] = WET
            elif r < 1.22 and g[i] == 0:
                g[i] = CLAY if fbm(x / 18.0, y / 18.0, 53) > 0.5 else SAND
    # A shallow creek from the north edge down to the river at x ~ 330.
    for y in range(0, H):
        cx = 330 + 30 * math.sin(y / 45.0) + 14 * (fbm(1.1, y / 30.0, 61) - 0.5)
        if y > centre(cx) - half_shallow(cx):
            break
        for x in range(int(cx - 6), int(cx + 7)):
            if 0 <= x < W and abs(x - cx) < 4 + 2 * fbm(x / 6.0, y / 6.0, 63):
                g[y * W + x] = WET
    # Rock outcrops by the banks.
    for (rx, ry, rr) in ((120, 430, 22), (610, 470, 18), (930, 230, 20), (420, 205, 14)):
        for y in range(ry - rr - 8, ry + rr + 8):
            for x in range(rx - rr - 8, rx + rr + 8):
                if math.hypot(x - rx, y - ry) < rr + 6 * (fbm(x / 7.0, y / 7.0, 71) - 0.5) and g[y * W + x] in (0, SAND, CLAY):
                    g[y * W + x] = ROCK
    # The bridge: a straight deck across the whole river, on sand abutments.
    cmid = centre(sum(BRIDGE_X) / 2)
    reach = half_shallow(sum(BRIDGE_X) / 2) + 14
    for x in range(BRIDGE_X[0] - 8, BRIDGE_X[1] + 8):
        for y in range(int(cmid - reach - 14), int(cmid + reach + 14)):
            i = y * W + x
            on_deck = BRIDGE_X[0] <= x < BRIDGE_X[1] and abs(y - cmid) < reach
            if on_deck:
                g[i] = BRIDGE
            elif abs(y - cmid) >= reach - 2 and abs(y - cmid) < reach + 14 - abs(x - sum(BRIDGE_X) / 2) * 0.5 and g[i] not in (DEEP, WET):
                g[i] = SAND
    items = {
        "landing": (512, 96),
        "mould": (512, 566),
        "crystals": [(512, 592, 34, 384), (200, 170, 26, 384), (470, 110, 24, 384), (250, 520, 26, 384), (800, 520, 26, 384), (70, 120, 22, 256), (960, 470, 22, 256)],
    }
    items["crystals"] = [c for c in items["crystals"] if c[2] > 0]
    # Crystals and starts sit on dry ground: clear their discs.
    bad = set()
    for (cx, cy, r, _) in items["crystals"]:
        for y in range(cy - r - 4, cy + r + 5):
            for x in range(cx - r - 4, cx + r + 5):
                if 0 <= x < W and 0 <= y < H and math.hypot(x - cx, y - cy) <= r + 4:
                    if g[y * W + x] in (DEEP, WET, BRIDGE):
                        bad.add((cx, cy))
                    if g[y * W + x] == ROCK:
                        g[y * W + x] = 0
    Image.new("RGBA", (W, H)).save(out / "_tmp.png") if False else None
    if bad:
        preview(g, out)
        raise SystemExit(f"crystals on water: {sorted(bad)}")
    check(g, items)
    preview(g, out)
    img = Image.new("RGBA", (W, H))
    img.putdata([(0, v, 0, 255) for v in g])
    img.save(out / "river.png")
    lines = [
        "# River v2 (new ground kinds, 2026-11-14): a wandering river, deep channel",
        "# in a shallow margin, sand bars and clay cut-banks, fords at x ~215 and",
        "# ~820, a plank bridge at x 500-523, a pond and a creek.",
        "# Made by Reports/ground-kinds-2026-11-14/raw/make_river.py.",
        f"landing {items['landing'][0]} {items['landing'][1]}",
        f"mould {items['mould'][0]} {items['mould'][1]}",
    ] + [f"crystal {x} {y} {r} {c}" for (x, y, r, c) in items["crystals"]]
    (out / "river.txt").write_text("\n".join(lines) + "\n")
    counts = {k: sum(1 for v in g if v == b) for k, b in (("deep", DEEP), ("shallow", WET), ("bridge", BRIDGE))}
    print("written;", counts)


def preview(g, out):
    """A flat colour picture of the ground beside the map (not a map file)."""
    pal = {0: (57, 54, 43), WET: (40, 70, 80), DEEP: (12, 30, 52), BRIDGE: (122, 92, 58), SAND: (150, 134, 92), SAND | WET: (90, 110, 100), CLAY: (104, 54, 38), ROCK: (94, 94, 93)}
    im = Image.new("RGB", (W, H))
    im.putdata([pal.get(v, (255, 0, 255)) for v in g])
    im.save(Path("/tmp") / "river-preview.png")


def check(g, items):
    ok = lambda v: v & ROCK == 0 and v >> 4 != 4
    lx, ly = items["landing"]
    seen = bytearray(W * H)
    q = deque([ly * W + lx])
    seen[ly * W + lx] = 1
    while q:
        i = q.popleft()
        x, y = i % W, i // W
        for j in ((i - 1) if x > 0 else -1, (i + 1) if x + 1 < W else -1, (i - W) if y > 0 else -1, (i + W) if y + 1 < H else -1):
            if j >= 0 and not seen[j] and ok(g[j]):
                seen[j] = 1
                q.append(j)
    def near(cx, cy, r):
        return any(seen[y * W + x] for y in range(max(0, int(cy - r)), min(H, int(cy + r) + 1)) for x in range(max(0, int(cx - r)), min(W, int(cx + r) + 1)) if math.hypot(x + 0.5 - cx, y + 0.5 - cy) <= r)
    for (cx, cy, r, _) in items["crystals"]:
        assert near(cx, cy, r + 8), f"crystal {(cx, cy)} cut off"
    assert near(*items["mould"], 8), "mould cut off"


if __name__ == "__main__":
    main(Path(sys.argv[1]))
