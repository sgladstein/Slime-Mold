#!/bin/bash
# run6.sh NAME "ENV" [args]: planet 6 (the Sleeper), run seed 1
# The report's runs (frames: A120 = 1,30,60,90,117,120,...,180 every 3; A30 = 1,27,30,...,90):
#   HAND=wake@120,334,452 AT=$A120 CENTRE=360,430,4 ./run6.sh w120_off "LAB_DUMP_LIFE=1"
#   HAND=wake@120,334,452 AT=$A120 CENTRE=360,430,4 ./run6.sh w120_on  "LAB_DREAM=3e-3 LAB_DUMP_LIFE=1"
#   (and LAB_DREAM=1e-2; and wake@30 with $A30)
#   python3 m6.py w30_off w30_on w120_off w120_on
name=$1; envs=$2; shift 2
o=/tmp/claude-0/lab/g6/$name; rm -rf $o; mkdir -p $o
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
env $envs LAB_DUMP=$o/dump ./target/release/larder-world look=$o run=1:6 mode=squads hand=${HAND:-pumps} bank=${BANK:-3000} at=${AT:-1,30,60,120} ${CENTRE:+centre=$CENTRE} "$@" > $o/log.txt 2>&1
echo "$name wall $(( $(date +%s)-s ))s" >> $o/log.txt
