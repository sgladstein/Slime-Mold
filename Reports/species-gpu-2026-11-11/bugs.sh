#!/bin/sh
# Planted bugs for the GPU species twin tests (2026-11-11): plant each bug in
# bugs.py, run the species and growth twin tests on the software GPU, put
# the file back, and say whether the tests caught it. From the repo root
# with no uncommitted change to the planted files (restored with git
# checkout). ONLY="16 18" plants only those. Rounds: 1 raw/bugs-round1.txt;
# 2, 26 bugs on the first rows pass, raw/bugs-round2.txt; 3, all 28 with
# the rows pass by workgroup tiles, raw/bugs-round3.txt (one missed); 4,
# the rows bugs again with the rows test added, and the wall fix,
# raw/bugs-round4.txt (ONLY="16 17 18 21 28").
D=Reports/species-gpu-2026-11-11
n=$(python3 -I -c "import sys; sys.path.insert(0,'$D'); from bugs import B; print(len(B))")
for i in ${ONLY:-$(seq 0 $((n-1)))}; do
  name=$(python3 -I - "$i" <<PY
import sys; sys.path.insert(0,'$D'); from bugs import B
name, p, old, new = B[int(sys.argv[1])]
s=open(p).read(); assert s.count(old)==1, name
open(p,'w').write(s.replace(old,new)); print(name)
PY
)
  out=$(MOULDGPU_TEST_GPU=1 cargo test --release --locked --bin mouldgpu -- --nocapture species_settings growth_matches rows_sum a_wall_builds 2>&1)
  git checkout -q src/gpu/engine.wgsl src/gpu/mod.rs
  if echo "$out" | grep -q "could not compile"; then verdict="DID NOT BUILD"
  elif echo "$out" | grep -q "test result: ok"; then verdict=MISSED; else verdict=caught; fi
  echo "== $i $name: $verdict"
  echo "$out" | grep -E "panicked at" -A1 | grep -v "^--$" | grep -vE "stack backtrace|note:" | tail -4
done
