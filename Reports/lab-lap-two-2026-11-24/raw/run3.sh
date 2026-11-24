#!/bin/bash
# run3.sh NAME PLANET HAND "KEYS" [extra args]: a retest, 2:00, run seed 1
name=$1; planet=$2; hand=$3; keys=$4; shift 4
o=/tmp/claude-0/lab/lap2/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
LAB_DUMP=$o/dump LAB_DUMP_LIFE=1 ./target/release/larder-world look=$o run=1:$planet mode=squads hand=$hand bank=3000 at=${AT:-1,30,60,120} $keys "$@" > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
