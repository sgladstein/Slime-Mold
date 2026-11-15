#!/bin/bash
cd /home/user/review-aesthetics
until [ -f frames/close/done.txt ]; do sleep 5; done
mkdir -p frames/gif
T=$(python3 -c "print(','.join(f'{55+i/10:.1f}' for i in range(41)))")
F=$(python3 -c "print(','.join(f'{12+i/10:.1f}' for i in range(41)))")
run(){ t0=$(date +%s); /home/user/sm-review-bin/larder-world "$@" > frames/gif/$(echo $2 | tr ':/=' '___').log 2>&1; echo "$* took $(( $(date +%s) - t0 )) s" >> frames/gif/timing.txt; }
run look=frames/gif/fire-p1s1 map=planet:1:1 seed=1 hand=rush rounds=scorch at=$F screen=1280x800 centre=118,185,12
run look=frames/gif/front-p3s1 map=planet:3:1 seed=1 hand=idle at=$T screen=1280x800 centre=124,161,12
run look=frames/gif/front-p1s1 map=planet:1:1 seed=1 hand=idle at=$T screen=1280x800 centre=74,69,12
run look=frames/gif/front-p4s1 map=planet:4:1 seed=1 hand=idle at=$T screen=1280x800 centre=316,104,12
run look=frames/gif/front-p5s1 map=planet:5:1 seed=1 hand=idle at=$T screen=1280x800 centre=469,227,12
echo done >> frames/gif/timing.txt
