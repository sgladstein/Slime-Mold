#!/bin/bash
# Clips of wading (2026-11-13): shape C on the Fork, seed 1, llvmpipe.
# $1 = name, $2 = hand, $3 = from, $4 = to, $5 = camera centre, rest = keys.
cd "$(dirname "$0")"
N=$1; H=$2; A=$3; Z=$4; C=$5; shift 5
AT=$(python3 -c "print(','.join(f'{$A+k*0.5:.2f}' for k in range(int(($Z-$A)/0.5)+1)))")
rm -rf clip-$N; mkdir -p clip-$N
./larder-world look=./clip-$N map=fork shape=c "hand=$H" centre=$C at=$AT "$@" > clip-$N.txt 2>&1
echo done $N
