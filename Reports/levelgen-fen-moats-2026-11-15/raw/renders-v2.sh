#!/bin/bash
# After only, second try (rock lip + moat): fen nests in game on llvmpipe, hand=idle.
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
  :
  job after nest-3-$s map=planet:3:$s seed=$s at=1,30 screen=1280x800 centre=$(nest moat/after2 $s),3
  wait
  :
  job after close-3-$s map=planet:3:$s seed=$s at=30 screen=1280x800 centre=$(nest moat/after2 $s),5
  wait
done
job after film-3-4 map=planet:3:4 seed=4 at=$(seq -s, 1 3 60) screen=1280x800 centre=$(nest moat/after2 4),3 &
job after film-3-7 map=planet:3:7 seed=7 at=$(seq -s, 1 3 60) screen=1280x800 centre=$(nest moat/after2 7),3
wait
echo done >> frames/timing.txt
