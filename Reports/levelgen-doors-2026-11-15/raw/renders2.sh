#!/bin/bash
# Second pass: each planet's nest 0 closer (zoom 5), before and after, at 1 s and 30 s.
cd /home/user/doorpics
pt() { python3 -I -c "import json;p=json.load(open('points.json'))['$1'];v=$2;print(f'{v[0]:.1f},{v[1]:.1f}')"; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for ds in "1 2" "2 3" "3 1" "4 2" "5 1" "6 3" "7 2" "8 2"; do
  set -- $ds
  job before close-$1-$2 map=planet:$1:$2 seed=$2 at=1,30 screen=1280x800 centre=$(pt before-$1-$2 "p['nests'][0]"),5 &
  job after close-$1-$2 map=planet:$1:$2 seed=$2 at=1,30 screen=1280x800 centre=$(pt after-$1-$2 "p['nests'][0]"),5
  wait
done
echo done2 >> frames/timing.txt
