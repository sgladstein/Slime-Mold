#!/bin/bash
# run.sh OUTDIR NAME "ENV=.. ENV2=.." args...
# One headless film on the software GPU from the lab branch's build
# (claude/project-thread-4tc75h), its log beside it. Run from the repo root.
out=$1; name=$2; envs=$3; shift 3
mkdir -p "$out"
env VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json LAB_LOG=1 $envs ./target/release/larder-world look="$out/$name" "$@" > "$out/$name.log" 2>&1
