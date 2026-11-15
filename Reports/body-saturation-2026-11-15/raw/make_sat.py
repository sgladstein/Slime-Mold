"""Write a diagnostic copy of src/world/view.wgsl that colours each pixel by
how saturated the slime body's walker count is: x = 25 n / ws * kf^2 / n0,
the exponent of `prep`'s `near` (near = 1 - exp(-x)), stored in `mcell`'s
spare word and read back in `base`.

    python3 make_sat.py OUT.wgsl
    LARDER_VIEW_WGSL=OUT.wgsl larder-world look=DIR ... at=...

Colours: dark no body; blue x < 1 (near < 0.63); green 1-3 (0.63-0.95);
yellow 3-6, red 6-12, white over 12 (saturated). `classify.py` counts them.
"""
import sys
s = open('src/world/view.wgsl').read()
old = '''    let near = 1.0 - exp(-25.0 * n / ws * kf * kf / n0);
    let wide = 1.0 - exp(-25.0 * nb / wb * kf * kf / n0);'''
assert s.count(old) == 1
s = s.replace(old, old + '''
    let dbg_x = 25.0 * n / ws * kf * kf / n0;''')
old = '''        mcell[i] = mat_pack(m1);
        return;'''
assert s.count(old) == 1
s = s.replace(old, '''        var pk1 = mat_pack(m1); pk1.w = bitcast<u32>(dbg_x);
        mcell[i] = pk1;
        return;''')
old = '''    mcell[i] = mat_pack(mt);
}'''
assert s.count(old) == 1
s = s.replace(old, '''    var pk = mat_pack(mt); pk.w = bitcast<u32>(dbg_x);
    mcell[i] = pk;
}''')
old = '''    out[y * P.stride + x] = pack(c);'''
assert s.count(old) == 1
s = s.replace(old, '''    if (!in_minimap(x, y)) {
        let wxx = P.cx + (f32(x) + 0.5) / P.zoom;
        let wyy = P.cy + (f32(y) + 0.5) / P.zoom;
        let f = fpos(wxx, wyy);
        let xs = bitcast<f32>(mcell[ridx(i32(round(f.x)), i32(round(f.y)))].w);
        var col = vec3<f32>(20.0, 20.0, 20.0);
        if (xs > 0.05) { col = vec3<f32>(30.0, 60.0, 200.0); }
        if (xs > 1.0) { col = vec3<f32>(40.0, 180.0, 60.0); }
        if (xs > 3.0) { col = vec3<f32>(230.0, 210.0, 40.0); }
        if (xs > 6.0) { col = vec3<f32>(230.0, 60.0, 30.0); }
        if (xs > 12.0) { col = vec3<f32>(255.0, 255.0, 255.0); }
        c = mix(c, col, 0.85);
    }
    out[y * P.stride + x] = pack(c);''')
open(sys.argv[1], 'w').write(s)
