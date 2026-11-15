#!/bin/bash
# Cave gates off pools in game, before (main b016fd0f) and after, on llvmpipe, hand=idle, at 1 s:
# three gates that stood in a pool before (dish cells, from the before maps), at zoom 3 and zoom 6.
cd /home/user/gates
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle screen=1280x800 "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for spot in 2:377,321 11:399,490 16:296,225; do
  s=${spot%:*}; c=${spot#*:}
  for z in 3 6; do
    job before gate-$s-z$z map=planet:7:$s seed=$s at=1 centre=$c,$z &
    job after gate-$s-z$z map=planet:7:$s seed=$s at=1 centre=$c,$z
    wait
  done
done
echo done >> frames/timing.txt
