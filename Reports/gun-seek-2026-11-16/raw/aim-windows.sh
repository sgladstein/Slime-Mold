#!/bin/bash
# aim-windows.sh OUTDIR: walk-windows.sh's paired windows for the card as
# built (version 8): a seeking bullet stops at live walkers on its line
# (version 7), and its catch beside the line rides the legs within the
# catch of its aim point (`Bullet::aim`, how far its soldier aimed; a
# soldier's bullet flies on to its full range) and its last leg.
# Variants (name|cards|extra keys):
#   plain     no card
#   aim2      seek+plain, catch 2 dish cells a card, no steering
#   aim4      seek+plain, catch 4, no steering
#   aim2turn  seek+plain as built: catch 2 and version 3's steering
# Run from the repo root after `cargo build --release --bin larder-world`;
# table: `python3 trace_table.py OUTDIR plain aim2 aim4 aim2turn`.
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
  echo "$n|$t|aim2|seek+plain|$a set=seek_turn=0"
  echo "$n|$t|aim4|seek+plain|$a set=seek_turn=0 set=seek_catch=4"
  echo "$n|$t|aim2turn|seek+plain|$a"
done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t v c a <<< "{}"
  d='"$F"'/$n-$t-$v; mkdir -p $d
  e=$((t + 10)); case $n in greedy) e=$((t + 10 > 21 ? 21 : t + 10));; esac
  '"$B"' look=$d rounds=plain load=$t,$c at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-$v exit $?"'
echo WINDOWS DONE
