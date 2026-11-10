#!/bin/bash
# usage: run.sh NAME "extra args" hands seeds
S=/tmp/claude-0/-home-user/fb694d0e-9c94-59b9-bea3-b6eeccf0da0f/scratchpad/tune
name=$1; extra=$2; hands=$3; seeds=$4
mkdir -p $S/$name
for h in $hands; do for s in $seeds; do echo "$h $s"; done; done | \
 RAYON_NUM_THREADS=1 xargs -P4 -n2 sh -c "/home/user/Pixel_Physics/target/release/examples/mouldprobe level=4 $extra players=\$0 seeds=\$1 > $S/$name/a16-\$0-\$1.txt 2>&1"
