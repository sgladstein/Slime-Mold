#!/bin/bash
# one.sh NAME SEED X,Y ENV NOBOSS: hold on the crystal at X,Y; NOBOSS=boss turns mini-bosses off
e=$4; [ "$e" = "-" ] && e=""
extra=""; [ "$5" = "boss" ] && extra="set=boss_first=9999"
HAND=hold@$3 bash /tmp/claude-0/lab/march/runm.sh $1 $2 "$e" $extra
