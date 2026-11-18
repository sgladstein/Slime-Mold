#!/bin/bash
# run.sh OUTDIR NAME "ENV=.. ENV2=.." args...   (one headless film, log beside it)
out=$1; name=$2; envs=$3; shift 3
mkdir -p "$out"
cd /home/claude/Slime-Mold
env VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json LAB_LOG=1 $envs ./target/release/larder-world look="$out/$name" "$@" > "$out/$name.log" 2>&1
