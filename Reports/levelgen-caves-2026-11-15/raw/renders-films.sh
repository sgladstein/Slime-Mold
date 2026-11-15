#!/bin/bash
# Films of the slime's first minute from its first nest at zoom 2, cave seeds 8 and 2, before and after
# (the landing films in renders.sh showed nothing moving: the slime doesn't reach the landing in a minute).
cd /home/user/caves
nest() { awk '$1=="mould" {printf "%.0f,%.0f\n", $2/2, $3/2; exit}' v2/planet-7-$1.txt; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle screen=1280x800 "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for s in 8 2; do
  N=$(nest $s)
  job before nestfilm-$s map=planet:7:$s seed=$s at=$(seq -s, 1 3 60) centre=$N,2 &
  job after nestfilm-$s map=planet:7:$s seed=$s at=$(seq -s, 1 3 60) centre=$N,2
  wait
done
echo done-films >> frames/timing.txt
