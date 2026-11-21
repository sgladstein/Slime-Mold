#!/bin/bash
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
B=/home/claude/Slime-Mold/target-bot/release/larder-world
s=$1; k=$2
[ "$k" = 0 ] && exit 0
$B league=1:$s:600 "plan=rush,aim=6,drops=$k" > /tmp/claude-0/cen/income/p1-s$s-d$k.txt 2>&1
