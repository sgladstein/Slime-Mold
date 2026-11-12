#!/bin/sh
# Planted bugs for the GPU life cycle (2026-11-11): plant each bug in
# bugs.py, run the life cycle's twin test, the squad test, the wall test and
# the alarm-alone test on the software GPU, put the file back, and say whether the tests caught
# it. From the repo root with no uncommitted change to the planted files
# (restored with git checkout). ONLY="3 7" plants only those. Round 1
# (the first 60 against the first test): raw/bugs-round1.txt. Round 2 (all
# 61 against the strengthened test, on main after #82): raw/bugs-round2.txt.
# Round 3 (all, against the final test, on main after #85):
# raw/bugs-round3.txt.
D=Reports/life-gpu-2026-11-11
n=$(python3 -I -c "import sys; sys.path.insert(0,'$D'); from bugs import B; print(len(B))")
for i in ${ONLY:-$(seq 0 $((n-1)))}; do
  name=$(python3 -I - "$i" <<PY
import sys; sys.path.insert(0,'$D'); from bugs import B
name, p, old, new = B[int(sys.argv[1])]
s=open(p).read(); assert s.count(old)==1, name
open(p,'w').write(s.replace(old,new)); print(name)
PY
)
  out=$(MOULDGPU_TEST_GPU=1 cargo test --release --locked --bin mouldgpu -- --nocapture the_life_cycle squad_matches a_wall_builds an_alarm_without 2>&1)
  git checkout -q src/gpu/engine.wgsl src/gpu/mod.rs src/mould/squad.rs
  if echo "$out" | grep -q "could not compile"; then verdict="DID NOT BUILD"
  elif echo "$out" | grep -q "test result: ok"; then verdict=MISSED; else verdict=caught; fi
  echo "== $i $name: $verdict"
  echo "$out" | grep -E "panicked at" -A1 | grep -v "^--$" | grep -vE "stack backtrace|note:" | tail -4
done
