#!/bin/sh
# Walker-count sweep, cloud stopgap (2026-11-11 16:2xZ) while the 1070's disk is full: the same
# b1f0383 mouldgpu on llvmpipe (software Vulkan: positions differ from the 1070 in the last bits;
# timings are not GPU numbers). 30 s only (the deciding measure: loops at 30 s), crowd on first,
# then crowd off at the same counts as the same-device control. Two runs at a time.
S=${S:?set S to a scratch folder}
B=${B:?set B to the absolute path of mouldgpu built at b1f0383}
mkdir -p $S/sweep/raw $S/sweep/pics; cd $S/sweep
for crowd in 1 0; do for n in 32000 100000 200000; do for s in 1 2 3 4 5 6 7 8; do echo "$crowd $n $s"; done; done; done |
  xargs -P 2 -L 1 sh -c 'timeout 1800 '"$B"' preset=3 crowd=$0 walkers=$1 seed=$2 seconds=30 look='"$S"'/sweep/pics tag=crowd$0-$1-s$2 > raw/cloud-crowd$0-$1-s$2-30s.txt 2>&1'
