#!/bin/bash
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cd /home/claude/Slime-Mold
n=$1; seed=$2; r=$3; shift 3
T="turret=121.125,225.227 turret=72.125,252.477 turret=45.875,186.977 turret=116.625,200.34 turret=55.25,182.34 turret=128.25,235.136 turret=35.125,175.66 wall=118.5,186.85,136.75,239.48 wall=69.5,180.23,30.25,164.48"
./target/release/lw-new look=/tmp/claude-0/sim/out4/$n run=$seed:4 mode=squads hand=idle at=60,120,180 tload=0,$r $T set=raid_first=30 set=raid_first_sd=0 set=raid_early=0 centre=85,215,6 "$@" > /tmp/claude-0/sim/out4/$n.log 2>&1
