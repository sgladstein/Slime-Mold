#!/bin/bash
# run2.sh PLANET_ID ARM(base|pair): one smoke test, 2:00, run seed 1 on the base tier's planet
id=$1; arm=$2
IFS='|' read pid base hand keys < <(grep "^$id|" /tmp/claude-0/lab/lap2/pairs.txt)
o=/tmp/claude-0/lab/lap2/p${id}_$arm; rm -rf $o; mkdir -p $o
[ "$arm" = base ] && keys=""
cd /home/claude/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
s=$(date +%s)
LAB_DUMP=$o/dump LAB_DUMP_LIFE=1 LAB_DUMP_SCAR=1 ./target/release/larder-world look=$o run=1:$base mode=squads hand=$hand bank=3000 at=1,60,120 $keys > $o/log.txt 2>&1
echo "p$id $arm wall $(( $(date +%s)-s ))s" >> $o/log.txt
