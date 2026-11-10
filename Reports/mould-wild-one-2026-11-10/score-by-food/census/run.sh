#!/bin/bash
# usage: run.sh OUTDIR  -- lines on stdin: NAME|ENV|HAND ; runs seeds 1-8 each, 4 wide
O=$1; mkdir -p $O
while IFS='|' read -r name env hand; do for s in 1 2 3 4 5 6 7 8; do echo "$name|$env|$hand|$s"; done; done | \
 xargs -P4 -I{} bash -c 'IFS="|" read -r n e h s <<< "{}"; env RAYON_NUM_THREADS=1 $e /home/user/slime-mold/target/release/examples/mouldprobe level=5 players=$h seeds=$s > '$O'/$n-$s.txt 2>&1'
