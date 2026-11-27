#!/bin/bash
cd /home/user/Slime-Mold
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
PLAN=$(grep -v '^#' scripts/run-bot-plan.txt | grep -m1 .)
OUT=Reports/review-2026-11-26/raw
mkdir -p $OUT
printf '%s\n' "3 1" "3 2" "3 3" "6 1" "6 2" "6 3" "6 4" "6 5" "6 6" "3 4" "3 5" "3 6" "9 1" "9 2" | xargs -P 4 -L1 bash -c '
  f='"$OUT"'/$0-$1-fair.txt
  timeout 1800 target/release/larder-world league=$0:$1:0:fair "plan='"$PLAN"'" 2>&1 | grep -E "^league |error|panick" > "$f"'
echo ALLDONE
