#!/bin/bash
cd /home/user/review-aesthetics
until grep -q done frames/main/timing.txt; do sleep 5; done
t0=$(date +%s)
bash scripts/close_cmds.sh
echo "close done $(( $(date +%s) - t0 )) s" > frames/close/done.txt
