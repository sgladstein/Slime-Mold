#!/bin/bash
# windows.sh OUTDIR: the route family in play, in paired 10 s windows (the
# owner's measure for a card, 11-16 14:24Z: paired windows from one game).
# Each scene plays one game to T; at T every squad loads the variant's deck
# (`load=T,CARDS`) and the window runs to T + 10. One card differs from the
# plain gun: a flare (`flare`: the bullet's own hit, then a burning alarm)
# or a road (`road`: every bullet lays the slime's trail along its flight).
# The scenes are seek's and the readers' (Reports/gun-seek-2026-11-16/raw/
# aim-windows.sh). Run from the repo root after `cargo build --release
# --bin larder-world`, with B=a copy of it so a rebuild mid-run cannot swap
# it; table: `python3 windows_table.py OUTDIR`.
F=${1:?OUTDIR}; mkdir -p "$F"
B=${B:-./target/release/larder-world}
{
  for t in 5 11; do echo "greedy|$t|hand=greedy"; done
  for t in 10 30; do echo "hold|$t|hand=hold@150,265 bank=8000"; done
  for s in "p1g2|map=planet:1:2 hand=guardwork" "p1g3|map=planet:1:3 hand=guardwork" "p3g4|map=planet:3:4 hand=guardwork" "p1s1|map=planet:1:1 hand=squadwork" "p2s1|map=planet:2:1 hand=squadwork"; do
    for t in 10 25 40; do echo "${s%%|*}|$t|${s#*|}"; done
  done
} | grep -v -e '^p1g2|10|' -e '^p1s1|10|' -e '^p3g4|10|' -e '^p3g4|25|' | while IFS='|' read -r n t a; do
  echo "$n|$t|plain|plain|$a"
  echo "$n|$t|flare|flare|$a"
  echo "$n|$t|road|road|$a"
done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t v c a <<< "{}"
  d='"$F"'/$n-$t-$v; mkdir -p $d
  e=$((t + 10)); case $n in greedy) e=$((t + 10 > 21 ? 21 : t + 10));; esac
  '"$B"' look=$d rounds=plain load=$t,$c at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-$v exit $?"'
echo WINDOWS DONE
