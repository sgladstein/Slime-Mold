#!/bin/bash
# Before (the map fixes, PR #323) and after (front doors and back roads): in-game frames on llvmpipe, hand=idle.
cd /home/user/doorpics
pt() { python3 -I -c "import json;p=json.load(open('points.json'))['$1'];v=$2;print(f'{v[0]:.1f},{v[1]:.1f}')"; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for tag in before after; do
  for ds in "1 2" "2 3" "3 1" "4 2" "5 1" "6 3"; do
    set -- $ds
    job $tag nest-$1-$2 map=planet:$1:$2 seed=$2 at=1,30 screen=1280x800 centre=$(pt $tag-$1-$2 "p['nests'][0]"),3 &
    job $tag door-$1-$2 map=planet:$1:$2 seed=$2 at=1 screen=1280x800 centre=$(pt $tag-$1-$2 "p['door']"),8
    wait
  done
  job $tag nest-7-2 map=planet:7:2 seed=2 at=1,30 screen=1280x800 centre=$(pt $tag-7-2 "p['nests'][0]"),3 &
  job $tag nest-8-2 map=planet:8:2 seed=2 at=1,30 screen=1280x800 centre=$(pt $tag-8-2 "p['nests'][0]"),3
  wait
  job $tag door-7-2 map=planet:7:2 seed=2 at=1 screen=1280x800 centre=$(pt $tag-7-2 "p['door']"),8 &
  job $tag door-8-2 map=planet:8:2 seed=2 at=1 screen=1280x800 centre=$(pt $tag-8-2 "p['door']"),8
  wait
done
# Films, after only: the slime's first minute in its bowl.
job after film-1-2 map=planet:1:2 seed=2 at=$(seq -s, 1 3 60) screen=1280x800 centre=$(pt after-1-2 "p['nests'][0]"),3 &
job after film-7-2 map=planet:7:2 seed=2 at=$(seq -s, 1 3 60) screen=1280x800 centre=$(pt after-7-2 "p['nests'][0]"),3
wait
echo done >> frames/timing.txt
