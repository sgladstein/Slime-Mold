#!/bin/bash
# runb.sh NAME PLANET SEED X,Y ARM: the squad of 8 on the crystal at X,Y; every mini-boss a wanderer.
# ARM off: hold, no feeding. bait: hold, LAB_FEED=1.0. drain: drain@X,Y (drains it, buys soldiers), LAB_FEED=1.0.
name=$1; planet=$2; seed=$3; xy=$4; arm=$5
o=/tmp/claude-0/lab/bait/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
case $arm in off) h=hold@$xy; e="";; bait) h=hold@$xy; e="LAB_FEED=1.0";; drain) h=drain@$xy; e="LAB_FEED=1.0";; esac
s=$(date +%s)
env $e LAB_MARCH_LOG=0.8 ./target/release/larder-world look=$o run=$seed:$planet mode=squads hand=$h bank=3000 at=1,${AT:-90,180} set=boss_kind=2 > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
