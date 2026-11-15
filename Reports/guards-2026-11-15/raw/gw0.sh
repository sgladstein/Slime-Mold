#!/bin/bash
cd /home/user/Slime-Mold
S=$1; cov=$2; seed=$3
mkdir -p $S/gw/f0-$cov-$seed
GW_WORK=drain GW_COVER=$cov timeout 900 $S/lw-out0 mode=squads seed=$seed look=$S/gw/f0-$cov-$seed hand=guardwork at=60 2>&1 | grep "^GW" > $S/gw/edge-$cov-$seed.log
