#!/bin/bash
cd /tmp/claude-0/lab/sort
{
for s in 1 2 3; do echo "p8s${s}_off $s:8 60 X"; echo "p8s${s}_on $s:8 60 LAB_SORT=1"; done
for s in 1 2 3 4; do echo "p2s${s}_off $s:2 120 X"; echo "p2s${s}_on $s:2 120 LAB_SORT=1"; done
} | xargs -P 4 -L 1 sh -c './runs.sh "$0" "$1" "$2" "$( [ "$3" = X ] && echo "" || echo "$3")"'
echo ALLDONE
