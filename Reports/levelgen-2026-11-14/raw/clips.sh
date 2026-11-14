#!/bin/bash
# Short clips near the squad: planets 3 (canyon) and 6 (thicket), seed 1, 40-46 s every 0.25 s.
cd /home/user/Slime-Mold
S=/tmp/claude-0/-home-user/1b1457c7-c5ca-5b76-8a11-b0d6a22bd68a/scratchpad/film
while [ ! -f $S/films.done ]; do sleep 20; done
AT=$(python3 -c "print(','.join(f'{40+k*0.25:g}' for k in range(25)))")
./target/release/larder-world map=planet:3:1 seed=1 look=$S/clip3 hand=idle at=$AT screen=1280x800 > $S/clip3.log 2>&1 &
./target/release/larder-world map=planet:6:1 seed=1 look=$S/clip6 hand=idle at=$AT screen=1280x800 > $S/clip6.log 2>&1
wait
echo done > $S/clips.done
