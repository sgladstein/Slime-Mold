#!/usr/bin/env python3
"""Variants of the view shader, each with one part of the base pass taken
out, so a GPU timing run can say which part of `base` costs what.

    python3 Reports/view-cost-2026-11-21/variants.py OUT_DIR

writes OUT_DIR/<name>.wgsl for each variant below, from src/world/view.wgsl.
Time each with the game's own view timer, the shader swapped in by
LARDER_VIEW_WGSL (no rebuild), e.g.

    LARDER_VIEW_WGSL=OUT_DIR/soil.wgsl larder-world run=1:3 mode=squads \
        bench=60 hand=strat:rush,aim=6 centre=larder,6 viewpasses=30

and compare the `base` line with the `full` variant's (the shader as it is).
A variant's saving is what that part costs, give or take what the
compiler then drops with it. The pictures are wrong by design: these are
for timing only.
"""
import re
import sys
from pathlib import Path

SRC = Path(__file__).resolve().parents[2] / "src/world/view.wgsl"


def stub(src, sig, body):
    """Replace the function whose header starts `sig` with `body`, keeping
    the original under a new name, so the shader still compiles."""
    i = src.index(sig)
    name = re.match(r"fn (\w+)\(", sig).group(1)
    header_end = src.index("{", i)
    header = src[i:header_end]
    return src[:i] + header + "{ " + body + " }\n" + header.replace(f"fn {name}(", f"fn {name}_kept(", 1) + src[header_end:]


VARIANTS = {
    # The shader as it is: the baseline.
    "full": lambda s: s,
    # Soil grain and material (sand ripples, clay cracks, ...).
    "soil": lambda s: stub(s, "fn soil_detail(x: f32, y: f32, bare: f32) -> vec4<f32>", "return vec4<f32>(0.0);"),
    # The plants: their fields and their drawn layer (tufts, crowns, reeds).
    "plants": lambda s: stub(stub(s, "fn plant_layer(x: f32, y: f32, d: vec4<f32>, hot: bool) -> PlantLayer",
        "var o: PlantLayer; o.rgb = vec3<f32>(0.0); o.cover = 0.0; o.fl = vec3<f32>(0.0); o.fc = 0.0; o.fw = 0.0; o.dark = 1.0; return o;"),
        "fn plant_px(x: f32, y: f32) -> PlantPx",
        "var o: PlantPx; o.d = vec4<f32>(0.0); o.heat = 0.0; o.char = 0.0; o.fresh = 0.0; return o;"),
    # The set slime's crust grain.
    "crust": lambda s: stub(s, "fn crust_over(a: vec3<f32>, x: f32, y: f32, gv: f32) -> vec3<f32>", "return a;"),
    # Water: its share, shore and depth.
    "water": lambda s: stub(s, "fn water_share(x: f32, y: f32) -> vec2<f32>", "return vec2<f32>(0.0);"),
    # Crystals: their cells, height, shadows.
    "gems": lambda s: stub(s, "fn gem_near(x: f32, y: f32) -> u32", "return 0u;"),
    # Rock, walls and doors drawn over the ground (their tops stay).
    "terrain": lambda s: stub(s, "fn terrain_over(c0: vec3<f32>, x: f32, y: f32) -> vec3<f32>", "return c0;"),
    # Every point light (bullets, fire, the bite): no loop over them.
    "lights": lambda s: s.replace("j < P.n_lights; j++", "j < min(P.n_lights, 0u); j++"),
    # `shaded` down to the material's colour: what all its lighting costs.
    "flat": lambda s: s.replace("fn shaded(x: f32, y: f32, e: f32) -> vec3<f32> {\n    var m = mat_at(x, y);",
        "fn shaded(x: f32, y: f32, e: f32) -> vec3<f32> {\n    var m = mat_at(x, y);\n    if (P.sw > 0u) { return m.albedo; }", 1),
}


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 else "view-cost")
    out.mkdir(parents=True, exist_ok=True)
    src = SRC.read_text()
    for name, f in VARIANTS.items():
        v = f(src)
        assert name == "full" or v != src, f"variant {name} changed nothing: the shader moved on"
        (out / f"{name}.wgsl").write_text(v)
        print(out / f"{name}.wgsl")


if __name__ == "__main__":
    main()
