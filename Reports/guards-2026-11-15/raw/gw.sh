#!/bin/bash
cd /home/user/Slime-Mold
S=$1; work=$2; cov=$3; seed=$4
mkdir -p $S/gw/f-$work-$cov-$seed
GW_WORK=$work GW_COVER=$cov timeout 900 ./target/release/larder-world mode=squads seed=$seed look=$S/gw/f-$work-$cov-$seed hand=guardwork at=60 2>&1 | grep "^GW" > $S/gw/$work-$cov-$seed.log
