#!/bin/sh
# Planted bugs for the GPU growth twin test (2026-11-11): plant each bug in
# bugs.py into src/gpu/engine.wgsl, run growth_matches_the_cpu_for_one_tick
# on the software GPU, put the file back, and say whether the test caught
# it. From the repo root with no uncommitted change to engine.wgsl (it is
# restored with git checkout). Output in raw/bugs.txt.
D=Reports/growth-2026-11-11
n=$(python3 -I -c "import sys; sys.path.insert(0,'$D'); from bugs import B; print(len(B))")
i=0
while [ $i -lt $n ]; do
  name=$(python3 -I - "$i" <<PY
import sys; sys.path.insert(0,'$D'); from bugs import B
name, old, new = B[int(sys.argv[1])]
p='src/gpu/engine.wgsl'; s=open(p).read(); assert s.count(old)==1, name
open(p,'w').write(s.replace(old,new)); print(name)
PY
)
  out=$(MOULDGPU_TEST_GPU=1 cargo test --release --locked --bin mouldgpu growth_matches -- --nocapture 2>&1)
  git checkout -q src/gpu/engine.wgsl
  if echo "$out" | grep -q "test result: ok"; then verdict=MISSED; else verdict=caught; fi
  echo "== $i $name: $verdict"
  echo "$out" | grep -E "slots|panicked at" -A1 | grep -v "^--$" | grep -vE "stack backtrace|note:" | tail -3
  i=$((i+1))
done
