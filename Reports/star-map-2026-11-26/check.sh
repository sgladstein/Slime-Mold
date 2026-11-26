#!/bin/bash
# The star map's bot check (2026-11-26): the run bot's plan on the Safe,
# Fair and Hard planets of steps 3, 6 and 9 of run seeds 1-8, the route
# before them Fair. One `league` line a game in raw/STEP-SEED-PIP.txt.
# Usage: check.sh [PARALLEL]  (from the repo root; software Vulkan is fine)
cd "$(dirname "$0")/../.."
export VK_ICD_FILENAMES=${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}
PLAN=$(grep -v '^#' scripts/run-bot-plan.txt | grep -m1 .)
OUT=Reports/star-map-2026-11-26/raw
jobs=()
for step in 3 6 9; do for seed in 1 2 3 4 5 6 7 8; do for pip in safe fair hard; do
  jobs+=("$step $seed $pip")
done; done; done
printf '%s\n' "${jobs[@]}" | xargs -P "${1:-2}" -L1 bash -c '
  f='"$OUT"'/$0-$1-$2.txt
  [ -s "$f" ] && exit 0
  timeout 2400 target/release/larder-world league=$0:$1:0:$2 "plan='"$PLAN"'" 2>&1 | grep -E "^league |error|panick" > "$f.tmp"; mv "$f.tmp" "$f"'
