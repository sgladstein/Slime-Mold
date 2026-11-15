#!/bin/bash
# Before (4494ad4f, the reviewed build) and after (the map fixes): in-game frames on llvmpipe.
cd /home/user/mapfix
pt() { python3 -I -c "import json;p=json.load(open('points.json'))['$1'];v=$2;print(f'{v[0]:.1f},{v[1]:.1f}')"; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for tag in before after; do
  job $tag core-1 map=planet:8:1 seed=1 at=1,10,20,30,40,50,60 screen=1600x1000 &
  job $tag core-2 map=planet:8:2 seed=2 at=1,10,20,30,40,50,60 screen=1600x1000
  wait
  job $tag jungle-1 map=planet:5:1 seed=1 at=1 screen=1280x800 &
  job $tag valley-2 map=planet:4:2 seed=2 at=1 screen=1280x800
  wait
  job $tag bridge-4-2 map=planet:4:2 seed=2 at=1 screen=1280x800 centre=$(pt $tag-4-2 "p['bridge']"),12 &
  job $tag nest-8-2 map=planet:8:2 seed=2 at=1 screen=1280x800 centre=$(pt $tag-8-2 "p['nests'][0]"),6
  wait
  job $tag nest-5-1 map=planet:5:1 seed=1 at=1 screen=1280x800 centre=$(pt $tag-5-1 "p['nests'][1]"),6 &
  job $tag bridge-4-6 map=planet:4:6 seed=6 at=1 screen=1280x800 centre=$(pt $tag-4-6 "p['bridge']"),12
  wait
done
echo done >> frames/timing.txt
