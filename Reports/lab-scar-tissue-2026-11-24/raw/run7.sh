#!/bin/bash
# run7.sh NAME "ENV" [args]: planet 7, run seed 1, squad holding at the edge of the colony at 709,386
name=$1; envs=$2; shift 2
o=/tmp/claude-0/lab/g7/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
env $envs LAB_DUMP=$o/dump LAB_DUMP_SCAR=1 ./target/release/larder-world look=$o run=1:7 mode=squads hand=${HAND:-hold@770,400} bank=3000 at=${AT:-30,60,90,120,150} centre=${CENTRE:-745,395},${ZOOM:-6} "$@" > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
