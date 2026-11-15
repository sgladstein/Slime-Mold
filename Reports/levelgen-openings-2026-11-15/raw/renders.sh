#!/bin/bash
# Each biome's opening in game, before (main bef8f1fa) and after, on llvmpipe: the landing at zoom 3 at 1 s
# (hand=idle), seed 2 of planets 1-8; and films of the first minute with the greedy hand on the jungle and
# the highlands, seed 2. The films went unused: the greedy hand harvested nothing on either planet, before or
# after (bank 0, the same squad path), and the jungle's squad was wiped by 25 s both times.
cd /home/user/opening
land() { awk '$1=="landing" {printf "%.0f,%.0f\n", $2/2, $3/2; exit}' after/planet-$1-2.txt; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name screen=1280x800 "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for d in 1 2 3 4 5 6 7 8; do
  L=$(land $d)
  job before land-$d hand=idle map=planet:$d:2 seed=2 at=1 centre=$L,3 &
  job after land-$d hand=idle map=planet:$d:2 seed=2 at=1 centre=$L,3
  wait
done
for d in 5 6; do
  L=$(land $d)
  job before film-$d hand=greedy map=planet:$d:2 seed=2 at=$(seq -s, 1 3 60) centre=$L,3 &
  job after film-$d hand=greedy map=planet:$d:2 seed=2 at=$(seq -s, 1 3 60) centre=$L,3
  wait
done
echo done >> frames/timing.txt
