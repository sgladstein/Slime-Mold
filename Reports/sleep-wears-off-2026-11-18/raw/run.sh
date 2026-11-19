#!/bin/bash
# usage: run.sh NAME SEED ROUND [extra keys...]
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cd /home/claude/Slime-Mold
n=$1; seed=$2; r=$3; shift 3
./target/release/lw-sim look=/tmp/claude-0/sim/out/$n run=$seed:3 mode=squads hand=greedy at=60,120,150 load=0,$r crustlook=150 "$@" > /tmp/claude-0/sim/out/$n.log 2>&1
