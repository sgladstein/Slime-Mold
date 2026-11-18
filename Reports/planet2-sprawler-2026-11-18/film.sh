#!/bin/bash
# The films behind this report (2026-11-18, the Enemy lane), on main after
# #478 (`src/` unchanged through #481), software GPU. Each is the run bot
# (`hand=bot`, Engine and tools' `hands::bot_hand`) landing straight on a run
# seed's planet 2 (`run=SEED:2`): 9 soldiers, 0 food, no shop bought. The
# bot prints a `BOT` line every 10 game seconds on stderr; the tables are
# read off those lines.
#
# Two of the four kinds need a local change that was not committed:
# - back off: hands.rs's `quiet` test counts walkers over twice the gun's
#   range (not once) and the walk for a quiet crystal is weighed at 3
#   ranges (not half one), under `BOT_FAR=1`. The patch is in this
#   report's README.
# - size: `Run::level` multiplies `Level::walkers` by `RUN_SIZE` (the
#   start walkers and the slots, as a smaller size roll would).
set -e
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
out=${1:-/tmp/planet2}
B=./target/release/larder-world
film() { name=$1; seed=$2; at=$3; shift 3; mkdir -p "$out/$name"; "$B" look="$out/$name" run="$seed":2 mode=squads hand=bot at="$at" "$@" > "$out/$name/out.txt" 2> "$out/$name/err.txt"; }
for s in 1 2 3 4; do film bot$s $s 30,60,120,180; done
for s in 1 3 4; do film nos$s $s 30,60,120,180 set=smell=0.5 set=smell_r=2; done
# With the patches built in:
# for s in 1 3 4; do BOT_FAR=1 film far$s $s 30,60,120,180; done
# for s in 1 3 4; do RUN_SIZE=0.6 film size$s $s 30,60,120,180; RUN_SIZE=0.8 film s8a$s $s 30,60,120,180; done
# RUN_SIZE=0.6 film s6a2 2 30,60,120,180
# The clips' films: a whole-map frame every 2 game seconds to 2:30.
at=$(seq -s, 2 2 150)
film clip1 1 "$at"
film clip2 2 "$at"
# python3 Reports/planet2-sprawler-2026-11-18/make_clips.py "$out" Reports/planet2-sprawler-2026-11-18/page
