#!/bin/bash
# Whole map at 1 s and 60 s, and the zoom-12 identity view (no nest/crystal), hand idle.
cd /home/user/review-aesthetics
mkdir -p frames/main
for d in 1 2 3 4 5 6 7 8; do for s in 1 2; do
  c=$(python3 -I -c "import json;p=json.load(open('points.json'))['planet-$d-$s']['ident'];print(f'{p[0]},{p[1]},12')")
  t0=$(date +%s)
  /home/user/sm-review-bin/larder-world look=frames/main map=planet:$d:$s seed=$s hand=idle at=1,60 screen=1280x800 centre=$c > frames/main/log-$d-$s.txt 2>&1
  echo "planet $d seed $s centre $c took $(( $(date +%s) - t0 )) s" >> frames/main/timing.txt
done; done
echo done >> frames/main/timing.txt
