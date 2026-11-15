"""Where to centre each picture, world units (map cells / 2): each planet's
nests, and the spot 60 world units (120 map cells, a bowl's inner rim) from
nest 0 toward the landing, where the door is, before and after."""
import json, math
PLANETS = [(1, 2), (2, 3), (3, 1), (4, 2), (5, 1), (6, 3), (7, 2), (8, 2)]
out = {}
for tag in ("before", "after"):
    for d, s in PLANETS:
        base = f"{tag}/planet-{d}-{s}"
        rows = [l.split() for l in open(base + ".txt") if l.strip()]
        nests = [tuple(map(float, r[1:3])) for r in rows if r[0] == "mould"]
        land = [tuple(map(float, r[1:3])) for r in rows if r[0] == "landing"][0]
        q = nests[0]
        l = math.hypot(land[0] - q[0], land[1] - q[1])
        door = (q[0] + (land[0] - q[0]) / l * 120, q[1] + (land[1] - q[1]) / l * 120)
        out[f"{tag}-{d}-{s}"] = {"nests": [(x / 2, y / 2) for x, y in nests], "door": (door[0] / 2, door[1] / 2)}
json.dump(out, open("points.json", "w"), indent=1)
for k, v in out.items():
    print(k, v)
