#!/bin/bash
# miss-windows.sh OUTDIR: the paired census for seek (version 3). Each
# window plays one game with no cards to T, then plain or seek+plain for
# 10 s (the Fork's greedy game ends at 21 s, so its windows are 5-15 and
# 11-21). The counts at T+1/30 and at the window's end give its shots,
# misses and kills. Run from the repo root after
# `cargo build --release --bin larder-world`; 4 at a time.
F=${1:?OUTDIR}; mkdir -p "$F"
B=./target/release/larder-world
{
  for t in 5 11; do echo "greedy|$t|hand=greedy"; done
  for t in 10 30; do echo "hold|$t|hand=hold@150,265 bank=8000"; done
  for s in "p1g2|map=planet:1:2 hand=guardwork" "p1g3|map=planet:1:3 hand=guardwork" "p3g4|map=planet:3:4 hand=guardwork" "p1s1|map=planet:1:1 hand=squadwork" "p2s1|map=planet:2:1 hand=squadwork"; do
    for t in 10 25 40; do echo "${s%%|*}|$t|${s#*|}"; done
  done
} | while IFS='|' read -r n t a; do
  for c in plain seek+plain; do echo "$n|$t|$c|$a"; done
done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t c a <<< "{}"
  d='"$F"'/$n-$t-${c%%+*}; mkdir -p $d
  e=$((t + 10)); case $n in greedy) e=$((t + 10 > 21 ? 21 : t + 10));; esac
  '"$B"' look=$d rounds=plain load=$t,$c at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-${c%%+*} exit $?"'
echo WINDOWS DONE
