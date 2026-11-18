#!/bin/bash
# What planets 1 to 3 do to you by minute 2 (2026-11-18, the Enemy lane).
# Run from the repo root after `cargo build --release --locked --bin
# larder-world`, on the software GPU (CLAUDE.md, "Cloud sessions have no
# GPU").
#
#   film.sh OUT        -> OUT/<name>/ frames and OUT/<name>.log
#
# Run seed 1. `hand=bot` is the run bot's hand (Engine and tools,
# `hands::bot_hand`): drain the richest crystal, buy soldiers, volley at
# the heaviest slime; no pumps. `hand=pumps` builds two pumps and holds by
# the first. The near view follows the squad unless `centre=` (world
# units) fixes it. `crustlook=1` adds the set walkers to the log.
set -e
out=${1:?film.sh OUT}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
film() { name=$1; shift; mkdir -p "$out/$name"; ./target/release/larder-world look="$out/$name" "$@" > "$out/$name.log" 2>&1; }

# The counts: the bot on each planet to 2:30.
film J1-bot run=1:1 mode=squads hand=bot at=30,60,90,120,150
film J2-bot run=1:2 mode=squads hand=bot at=30,60,90,120,150
film J3-bot run=1:3 mode=squads hand=bot at=30,60,90,120,150 crustlook=1

# The page's clips (make_clips.py cuts them): five frames a game second.
A1=$(python3 -c "print(','.join('%.1f' % (40 + i / 5) for i in range(301)))")
A2=$(python3 -c "print(','.join('%.1f' % (20 + i / 5) for i in range(501)))")
A3=$(python3 -c "print(','.join('%.1f' % (30 + i / 5) for i in range(501)))")
film K1-pumps run=1:1 mode=squads hand=pumps centre=330,40,6 at=$A1
film K2-bot   run=1:2 mode=squads hand=bot at=$A2
film K3-bot   run=1:3 mode=squads hand=bot crustlook=1 at=$A3
