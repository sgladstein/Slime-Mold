#!/bin/bash
# Rivers off the nests in game, before (main 2f6ac3f4) and after, on llvmpipe, hand=idle:
# the whole river valley at zoom 2 at 1 s (seeds 1 and 5), and films of the slime's first minute
# (every 3 s) from the second nest at zoom 3 on valley seeds 1 and 5 and the core world seed 11
# (the second nest), where the river ran by the nest before.
cd /home/user/rivers
nest() { awk -v k=$3 '$1=="mould" {n++; if (n==k+1) {printf "%.0f,%.0f\n", $2/2, $3/2; exit}}' /home/user/opening/after/planet-$1-$2.txt; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle screen=1280x800 "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for s in 1 5; do
  job before whole-4-$s map=planet:4:$s seed=$s at=1 centre=304,188,2 &
  job after whole-4-$s map=planet:4:$s seed=$s at=1 centre=304,188,2
  wait
done
for ds in 4:1 4:5 8:11; do
  d=${ds%:*}; s=${ds#*:}
  N=$(nest $d $s 1)
  job before film-$d-$s map=planet:$d:$s seed=$s at=$(seq -s, 1 3 60) centre=$N,3 &
  job after film-$d-$s map=planet:$d:$s seed=$s at=$(seq -s, 1 3 60) centre=$N,3
  wait
done
echo done >> frames/timing.txt
