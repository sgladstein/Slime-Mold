#!/usr/bin/env bash
S=${S:-/tmp/scr}
B="blob=0 deposit_cap=100 seconds=20 report=1 at=6,9,20 style=sick,raw sick_steps=240 immune_steps=6000 sick_catch=5 sick_turn=2 sick_reach=0.5"
n=0
for s in 1 2 3 4 5 6 7 8; do
  $S/j.sh Rsick-s$s seeds=$s $B infect=4 & n=$((n+1)); (( n % 3 == 0 )) && wait
  $S/j.sh Rctl-s$s seeds=$s $B & n=$((n+1)); (( n % 3 == 0 )) && wait
done
wait
echo done > $S/rewire.done
