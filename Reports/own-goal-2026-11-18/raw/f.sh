#!/bin/bash
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cd /home/claude/Slime-Mold
n=$1; z=$2; shift 2
./target/release/larder-world look=/tmp/claude-0/clip/$n run=1:1 mode=squads hand=greedy at=38.0,38.5,39.0,39.5,40.0,40.5,41.0,41.5,42.0,42.5,43.0,43.5,44.0,44.5,45.0,45.5,46.0,46.5,47.0,47.5,48.0,48.5,49.0,49.5,50.0,50.5,51.0,51.5,52.0,52.5,53.0,53.5,54.0,54.5,55.0,55.5,56.0 centre=226.6,52.1,$z "$@" > /tmp/claude-0/clip/$n.log 2>&1
