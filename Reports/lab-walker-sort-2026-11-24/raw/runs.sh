#!/bin/bash
# runs.sh NAME SEED:PLANET SECS "ENV": a sort check, greedy hand, bank 3000
name=$1; run=$2; secs=$3; envs=$4
o=/tmp/claude-0/lab/sort/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
env $envs LAB_DUMP=$o/dump LAB_SORT_LOG=1 ./target/release/larder-world look=$o run=$run mode=squads hand=greedy bank=3000 at=1,$secs > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
