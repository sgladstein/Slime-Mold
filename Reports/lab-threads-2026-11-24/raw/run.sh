#!/bin/bash
# run.sh NAME SEED ZOOM "ENV..." [extra args]  -> /tmp/claude-0/lab/NAME
name=$1; seed=$2; zoom=$3; envs=$4; shift 4
o=/tmp/claude-0/lab/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
start=$(date +%s)
env $envs LAB_DUMP=$o/dump ./target/release/larder-world look=$o run=$seed:2 mode=squads hand=idle at=${AT:-60,120,180} centre=${CENTRE:-390,165},$zoom "$@" > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-start ))s" >> $o/log.txt
python3 /tmp/claude-0/lab/measure.py $o/dump 60 120 140 | sed "s/dump/$name/"
grep "^  t 180" $o/log.txt | cut -c1-120
