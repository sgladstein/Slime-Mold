#!/bin/bash
# The king rule on film, core undefended (core_pull=0) so a plain rush can reach it: race world seeds 5 and 6.
cd /home/user/Slime-Mold
B=target/release/larder-world
AT=$(seq -s, 4 4 180)
for s in 5 6; do
  d=/tmp/claude-0/qs/king0-$s; mkdir -p $d
  timeout 1200 $B look=$d hand=rush map=noise seed=$s core_pull=0 at=$AT screen=1280x800 > $d.log 2>&1
  echo "seed $s: $(grep -E 't 180 s|ended' $d.log | tail -1)"
done
echo done
