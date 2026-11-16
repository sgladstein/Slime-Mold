#!/bin/bash
# walk-windows.sh OUTDIR: line-windows.sh's paired windows for the card as
# built (version 7): a seeking bullet stops at a live walker on its line as
# well as at slime's look, on every leg, sunk as any hit (`squad::SEEKS`,
# `squad::trace`); on its last leg its catch beside the line (`CATCH`).
# Variants (name|cards|extra keys):
#   plain        no card
#   walk         seek+plain, no catch, no steering
#   walkturn     seek+plain, no catch, version 3's steering (`seek_turn`)
#   walkturn2    seek+plain, as built: catch 2 dish cells and the steering
# Run from the repo root after `cargo build --release --bin larder-world`;
# table: `python3 trace_table.py OUTDIR plain walk walkturn walkturn2`.
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
  echo "$n|$t|walk|seek+plain|$a set=seek_turn=0 set=seek_catch=0"
  echo "$n|$t|walkturn|seek+plain|$a set=seek_catch=0"
  echo "$n|$t|walkturn2|seek+plain|$a"
done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t v c a <<< "{}"
  d='"$F"'/$n-$t-$v; mkdir -p $d
  e=$((t + 10)); case $n in greedy) e=$((t + 10 > 21 ? 21 : t + 10));; esac
  '"$B"' look=$d rounds=plain load=$t,$c at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-$v exit $?"'
echo WINDOWS DONE
