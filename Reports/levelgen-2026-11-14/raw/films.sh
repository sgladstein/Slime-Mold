#!/bin/bash
# Films: each planet depth 1-8, seeds 1 and 2, idle hand, frames every 10 s to 120 s.
cd /home/user/Slime-Mold
S=/tmp/claude-0/-home-user/1b1457c7-c5ca-5b76-8a11-b0d6a22bd68a/scratchpad/film
AT=1,10,20,30,40,50,60,70,80,90,100,110,120
run() { d=$1; s=$2; ./target/release/larder-world map=planet:$d:$s seed=$s look=$S/p$d-$s hand=idle at=$AT screen=1280x800 > $S/p$d-$s.log 2>&1; }
for s in 1 2; do
  for d in 1 3 5 7; do run $d $s & run $((d+1)) $s; wait; done
done
echo done > $S/films.done
