#!/bin/bash
# Cave planets 2, 8 and 13 in game, before (main 17f2e5b8) and after, on llvmpipe, hand=idle:
# the landing at zoom 3 at 1 s and 30 s, halfway to the first nest and the widest room at zoom 3 at 30 s,
# the whole map at zoom 1 at 30 s; and films of the landing's first minute on seed 8.
cd /home/user/caves
spot() { PYTHONPATH=/home/user/review-fun/pylib python3 /home/user/Slime-Mold/Reports/levelgen-caves-2026-11-15/raw/spots.py v2 $1 | awk -v k=$2 '$1==k {print $2}'; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle screen=1280x800 "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for s in 2 8 13; do
  L=$(spot $s landing); M=$(spot $s mid); R=$(spot $s room)
  for tag in before after; do
    job $tag land-$s map=planet:7:$s seed=$s at=1,30 centre=$L,3 &
    job $tag mid-$s map=planet:7:$s seed=$s at=30 centre=$M,3
    wait
    job $tag room-$s map=planet:7:$s seed=$s at=30 centre=$R,3 &
    job $tag far-$s map=planet:7:$s seed=$s at=30 centre=512,320,1
    wait
  done
done
L=$(spot 8 landing)
job before film-8 map=planet:7:8 seed=8 at=$(seq -s, 1 3 60) centre=$L,3 &
job after film-8 map=planet:7:8 seed=8 at=$(seq -s, 1 3 60) centre=$L,3
wait
echo done >> frames/timing.txt
