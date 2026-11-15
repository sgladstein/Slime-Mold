#!/usr/bin/env bash
# The run's planets by depth (2026-11-15, run and enemy progression thread):
# for each planet 1-8 of run seed 1, an idle squad (the slime's mass and
# regrowth, and one still a tier) and a squad on attack-move to the core (`hand=stand`: it stops where the slime comes in range and fires) carrying the average
# rifle of that depth (`gun=rifle`, `GunKind::average_at`): its kills a
# second. One seed each, no census. Writes OUT/{idle,stand}-dD.txt and the
# stills; on llvmpipe (4 cores) about 1 s of wall time a game second on
# planets 1-4, 11 s on planet 8.
#   Reports/run-progression-2026-11-15/depths.sh BIN OUT [DEPTHS]
set -u
bin=$1; out=$2; depths=${3:-"1 2 3 4 5 6 7 8"}
mkdir -p "$out"
for d in $depths; do
  for hand in idle stand; do
    s=$(date +%s)
    extra=""; [ "$hand" = stand ] && extra="gun=rifle"
    "$bin" look="$out/$hand-d$d" run=1:$d hand=$hand $extra at=1,30,60,90,120 > "$out/$hand-d$d.txt" 2>&1
    echo "wall $(( $(date +%s) - s )) s" >> "$out/$hand-d$d.txt"
  done
done
