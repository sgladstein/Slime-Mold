#!/usr/bin/env bash
S=${S:-/tmp/scr}
B="haze=0.3 seconds=20 report=10 at=20 style=raw respawn=0.001 sense_scale=0.02 step_mod=1:1"
n=0
for s in 3 4 5 6 7 8; do
  [ $s != 3 ] && { $S/j.sh P-disc-s$s seeds=$s blob=1 blob_r=45 $B & n=$((n+1)); (( n % 3 == 0 )) && wait; }
  $S/j.sh P-flat-s$s seeds=$s blob=0 $B & n=$((n+1)); (( n % 3 == 0 )) && wait
done
wait
echo done > $S/trunk.done
