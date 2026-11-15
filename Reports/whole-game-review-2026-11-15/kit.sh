#!/bin/sh
# The whole-game review's capture kit (2026-11-15): one proof run of each
# capture kind, from the repo root, on software Vulkan (llvmpipe), after
#   cargo build --release --locked --bin larder-world
# Each run's printed counts go to OUT/NAME.log, its wall time to OUT/times.txt.
OUT=${1:-/tmp/wgr/kit}
W=./target/release/larder-world
mkdir -p "$OUT"
: > "$OUT/times.txt"
cap() {
  name=$1; shift
  s=$(date +%s.%N)
  $W look="$OUT/$name" "$@" > "$OUT/$name.log" 2>&1
  rc=$?
  e=$(date +%s.%N)
  echo "$name rc=$rc wall=$(echo "$e - $s" | bc) s" | tee -a "$OUT/times.txt"
}
cap squads      mode=squads map=fork seed=1 hand=squads at=1,20
cap mission     mission=1-landfall hand=claim at=1,20
cap run1        run=1 hand=farm at=1,20
cap run-d4      run=1:4 hand=idle at=1,20
cap run-d8      run=1:8 hand=idle at=1,20
cap run-shop    run=1 menu=shop at=1
cap run-shopmid run=1 menu=shop-mid at=1
cap buildings   map=fork seed=1 hand=idle turret=135,208 pump=55,224 centre=95,210,6 at=1,10
cap fire        map=fork seed=1 hand=idle rounds=bomb boom=3,200,105 centre=200,105,8 at=6,12
cap own         own=1 hand=sow at=1,20
cap condense    map=fork seed=1 hand=farm condense=300 roam=300 at=1,40
cap range       range=7 hand=stand at=1,10
cap range-shop  range=7 menu=shop hand=stand at=1
cap ai-take     map=fork seed=1 hand=take_claim at=1,40
cap zoom24      map=fork seed=1 hand=farm centre=262,126,24 at=20
cap burst       map=fork seed=1 hand=rush at=6,6.1,6.2,6.3,6.4,6.5
