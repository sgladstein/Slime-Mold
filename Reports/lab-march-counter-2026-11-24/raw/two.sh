#!/bin/bash
# two.sh NAME SEED X,Y|- ENV|-: raids at 2:00 then every 60 s, mini-bosses off, to 4:30
e=$4; [ "$e" = "-" ] && e=""
h=hold; [ "$3" != "-" ] && h=hold@$3
HAND=$h FIRST=120 EVERY=60 AT=1,150,240,270 bash /tmp/claude-0/lab/march/runm.sh $1 $2 "$e" set=boss_first=9999
