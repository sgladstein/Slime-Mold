#!/bin/bash
# Clips of the bite: shape C on the Fork, seed 1, llvmpipe. $1 = hand, $2 = bite, $3 = from, $4 = to.
cd "$(dirname "$0")"
H=$1; B=$2; A=$3; Z=$4
AT=$(python3 -c "print(','.join(f'{$A+k*0.5:.2f}' for k in range(int(($Z-$A)/0.5)+1)))")
D=$H-b$B
mkdir -p $D
./larder-world look=./$D map=fork shape=c hand=$H bite=$B centre=254,264 at=$AT > $D.txt 2>&1
echo done $D
