#!/bin/bash
# run4.sh NAME "ENV" [args]: planet 4 (the Swarmer), run seed 1; extra args are engine keys
name=$1; envs=$2; shift 2
o=/tmp/claude-0/lab/g4/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
env $envs LAB_DUMP=$o/dump LAB_DUMP_LIFE=1 ./target/release/larder-world look=$o run=${RUN:-1:4} mode=squads hand=${HAND:-greedy} bank=${BANK:-0} at=${AT:-1,30,60,90,120,150,180} ${CENTRE:+centre=$CENTRE} "$@" > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
