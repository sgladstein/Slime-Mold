#!/bin/bash
# runm.sh NAME SEED "ENV" [args]: planet 4, the squad holds at its landing, raids forced early
name=$1; seed=$2; envs=$3; shift 3
o=/tmp/claude-0/lab/march/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
env $envs LAB_MARCH_LOG=${F:-0.8} ./target/release/larder-world look=$o run=$seed:4 mode=squads hand=${HAND:-hold} bank=${BANK:-3000} at=${AT:-1,60,120,180} set=raid_first=${FIRST:-30} set=raid_every=${EVERY:-40} "$@" > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
