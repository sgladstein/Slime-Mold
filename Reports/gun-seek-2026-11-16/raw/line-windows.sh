#!/bin/bash
# line-windows.sh OUTDIR: trace-windows.sh's paired windows for the card as
# built (version 6): on its last leg (the frame it reaches its aim point or
# a wall, `Game::catches`), a seeking bullet whose line misses strikes the
# first live walker on that line, or within its catch beside it the first
# cell that stops a shot or holds a walker (`squad::trace`). The four
# windows with no shots in versions 3 and 4 (p1g2-10, p1s1-10, p3g4-10,
# p3g4-25) are left out. Two earlier runs of this script were stopped part
# way: the catch on every leg that missed (`raw/line-windows-every-leg.txt`)
# and on the last leg without walkers (`raw/line-windows-last-leg.txt`,
# with a fifth variant catch4turn).
# Variants (name|cards|extra keys):
#   plain      no card
#   catch2     seek+plain, catch 2 dish cells a card, no steering
#   catch4     seek+plain, catch 4, no steering
#   catch2turn seek+plain, catch 2 and version 3's steering (`seek_turn`)
# Run from the repo root after `cargo build --release --bin larder-world`;
# table: `python3 trace_table.py OUTDIR plain catch2 catch4 catch2turn`.
F=${1:?OUTDIR}; mkdir -p "$F"
B=./target/release/larder-world
{
  for t in 5 11; do echo "greedy|$t|hand=greedy"; done
  for t in 10 30; do echo "hold|$t|hand=hold@150,265 bank=8000"; done
  for s in "p1g2|map=planet:1:2 hand=guardwork" "p1g3|map=planet:1:3 hand=guardwork" "p3g4|map=planet:3:4 hand=guardwork" "p1s1|map=planet:1:1 hand=squadwork" "p2s1|map=planet:2:1 hand=squadwork"; do
    for t in 10 25 40; do echo "${s%%|*}|$t|${s#*|}"; done
  done
} | grep -v -e '^p1g2|10|' -e '^p1s1|10|' -e '^p3g4|10|' -e '^p3g4|25|' | while IFS='|' read -r n t a; do
  echo "$n|$t|plain|plain|$a"
  echo "$n|$t|catch2|seek+plain|$a set=seek_turn=0"
  echo "$n|$t|catch4|seek+plain|$a set=seek_turn=0 set=seek_catch=4"
  echo "$n|$t|catch2turn|seek+plain|$a"
done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t v c a <<< "{}"
  d='"$F"'/$n-$t-$v; mkdir -p $d
  e=$((t + 10)); case $n in greedy) e=$((t + 10 > 21 ? 21 : t + 10));; esac
  '"$B"' look=$d rounds=plain load=$t,$c at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-$v exit $?"'
echo WINDOWS DONE
