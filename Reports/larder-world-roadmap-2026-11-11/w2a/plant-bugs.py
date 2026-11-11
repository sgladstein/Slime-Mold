import subprocess, sys, re, os
os.chdir(sys.argv[1])
W='src/gpu/engine.wgsl'; M='src/gpu/mod.rs'
bugs = [
 ("shots fly through walls", W, "        if (agar[i] == 0u) { break; }\n        if (trail_in[i] >= P.slime) { hit = i; break; }", "        if (trail_in[i] >= P.slime) { hit = i; break; }"),
 ("shots hit the last slime on their line", W, "if (trail_in[i] >= P.slime) { hit = i; break; }", "if (trail_in[i] >= P.slime) { hit = i; }"),
 ("a struck walker keeps its cell's count", W, "            atomicSub(&occ[c], 1u);\n            atomicStore(&claim[c], NONE);\n            energy[k] = -1.0;", "            atomicStore(&claim[c], NONE);\n            energy[k] = -1.0;"),
 ("a walker counts for every shot that strikes it", W, "            atomicAdd(&kills[q], 1u);\n            return;", "            atomicAdd(&kills[q], 1u);"),
 ("empty slots are struck too", W, "    if (k >= P.n || !lives(k)) { return; }\n    let wk = walkers[k];\n    let c = cell(wk.x, wk.y);\n    for (var q = 0u; q < P.nshots; q++) {\n        if (struck(c, q)) {", "    if (k >= P.n) { return; }\n    let wk = walkers[k];\n    let c = cell(wk.x, wk.y);\n    for (var q = 0u; q < P.nshots; q++) {\n        if (struck(c, q)) {"),
 ("strikes leave the trail", W, "    if (struck(i, q)) { trail_in[i] = 0.0; }", "    if (struck(i, q)) { trail_in[i] = trail_in[i]; }"),
 ("a strike one cell wider", W, "    return u32(dx * dx + dy * dy) <= SQ.shots[q].r2;", "    return u32(dx * dx + dy * dy) <= SQ.shots[q].r2 + 2u * u32(isqrt(SQ.shots[q].r2)) + 1u;"),
 ("smell on walls and rock", W, "                if (agar[i] != 0u) { scent_in[i] += un.smell; }", "                scent_in[i] += un.smell;"),
 ("smell laid twice", W, "                if (agar[i] != 0u) { scent_in[i] += un.smell; }", "                if (agar[i] != 0u) { scent_in[i] += 2.0 * un.smell; }"),
 ("touch counted over the square, not the disc", W, "y >= h || u32(dx * dx + dy * dy) > un.touch2) { continue; }", "y >= h) { continue; }"),
 ("aim ties go to the last block", W, "            if (n > best) { best = n; bi = u32(by * nbw + bx); }", "            if (n >= best && n > 0u) { best = n; bi = u32(by * nbw + bx); }"),
 ("aim tests the block's corner, not its centre", W, "            let mx = bx * b + b / 2 - cx;\n            let my = by * b + b / 2 - cy;", "            let mx = bx * b - cx;\n            let my = by * b - cy;"),
 ("harvest takes whole cells", W, "                        let t = min(c, rem);", "                        let t = c;"),
 ("harvest in reverse unit order", W, "    for (var u = 0u; u < P.nunits; u++) {\n        let un = SQ.units[u];\n        var took = 0.0;", "    for (var v = 0u; v < P.nunits; v++) {\n        let u = P.nunits - 1u - v;\n        let un = SQ.units[u];\n        var took = 0.0;"),
 ("reads before the ticks", M, """        if due {
            // The squad's frame start (`squad.rs`): shots, their strikes,
            // then the smell, on the trail and scent the first tick reads.
            let mut pre = Vec::new();""", """        if due && nunits > 0 {
            dispatch(enc, &self.groups[(self.done % 2) as usize], &[(UNITS, 1), (HARVEST, 1)]);
        }
        if due {
            let mut pre = Vec::new();"""),
 ("smell after the ticks", M, """            if nunits > 0 {
                pre.push((SMELL, 1));
            }""", """            if false {
                pre.push((SMELL, 1));
            }"""),
]
only = sys.argv[2:]
for k,(name, path, old, new) in enumerate(bugs):
    if only and str(k) not in only: continue
    src=open(path).read()
    if src.count(old)!=1:
        print(f"[{k}] {name}: PATTERN COUNT {src.count(old)}", flush=True); continue
    s2=src.replace(old,new)
    if name=="reads before the ticks":
        # and drop the real frame-end dispatch
        s2=s2.replace("""        if due && nunits > 0 {
            // The squad's frame end: reads, then the harvest.
            dispatch(enc, &self.groups[(self.done % 2) as usize], &[(UNITS, 1), (HARVEST, 1)]);
        }""","")
    if name=="smell after the ticks":
        s2=s2.replace("""            dispatch(enc, &self.groups[(self.done % 2) as usize], &[(UNITS, 1), (HARVEST, 1)]);""","""            dispatch(enc, &self.groups[(self.done % 2) as usize], &[(SMELL, 1), (UNITS, 1), (HARVEST, 1)]);""")
    open(path,'w').write(s2)
    try:
        # Build first, then run the test alone under a 120 s limit: it
        # takes about half a second, so a run that times out has hung.
        subprocess.run("cargo test --release --locked --bin mouldgpu --no-run 2>&1", shell=True, capture_output=True, text=True)
        r=subprocess.run("MOULDGPU_TEST_GPU=1 timeout 120 cargo test --release --locked --bin mouldgpu -- squad --test-threads 1 2>&1", shell=True, capture_output=True, text=True)
        out=r.stdout
        res = "CAUGHT" if "FAILED" in out else ("NOT CAUGHT" if "test result: ok" in out else ("CAUGHT (the test hung; killed after 120 s)" if r.returncode == 124 else "BUILD/OTHER"))
        print(f"[{k}] {name}: {res}", flush=True)
        lines=out.splitlines()
        for i,l in enumerate(lines):
            if "panicked" in l and i+1 < len(lines):
                print("    ", lines[i+1][:300], flush=True)
        if res=="BUILD/OTHER":
            print(out[-1500:], flush=True)
    finally:
        open(path,'w').write(src)
