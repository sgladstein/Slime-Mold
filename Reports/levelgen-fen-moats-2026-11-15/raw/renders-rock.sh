#!/bin/bash
# The A/B's rock twin: fen nests with lakes 3 to 1 and plain rock rims (Recipe::moat off), in game on llvmpipe, hand=idle.
cd /home/user/moatpics
nest() { python3 -I -c "
rows=[l.split() for l in open('/home/user/$1/planet-3-$2.txt') if l.strip()]
q=[tuple(map(float,r[1:3])) for r in rows if r[0]=='mould'][0]
print(f'{q[0]/2:.1f},{q[1]/2:.1f}')"; }
job() { # tag name args...
  local tag=$1 name=$2; shift 2
  mkdir -p frames/$tag/$name
  t0=$(date +%s)
  ./larder-world-$tag look=frames/$tag/$name hand=idle "$@" > frames/$tag/$name.log 2>&1
  echo "$tag $name $(( $(date +%s) - t0 )) s" >> frames/timing.txt
}
for s in 1 4 7 15; do
  job rock nest-3-$s map=planet:3:$s seed=$s at=1,30 screen=1280x800 centre=$(nest moat/rock $s),3 &
  job rock close-3-$s map=planet:3:$s seed=$s at=30 screen=1280x800 centre=$(nest moat/rock $s),5
  wait
done
echo done-rock >> frames/timing.txt
