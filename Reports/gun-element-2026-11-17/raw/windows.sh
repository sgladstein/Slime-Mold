#!/bin/bash
# windows.sh OUTDIR [SCENES]: the torch in play, in paired 10 s windows (the
# owner's measure for a card, 11-16 14:24Z: paired windows from one game).
# Each scene plays one game to T; at T every squad loads the variant's deck
# (`load=T,CARDS`) and the window runs to T + 10. The variants:
#   plain   plain bullets
#   torch   torch bullets (`Card::Torch`)
#   blast   blast bullets, the dearer kill the torch is set against
# The scenes are the readers' planet scenes (planets 1-3,
# Reports/gun-readers-2026-11-17/raw/windows.sh), and planets 4 and 5 (the
# second run, `p4s1 p5s1`). Run from the repo root after
# `cargo build --release --bin larder-world` (or B=a copy of it, so a
# rebuild mid-run cannot swap it); table: `python3 windows_table.py OUTDIR`.
F=${1:?OUTDIR}; mkdir -p "$F"; shift
B=${B:-./target/release/larder-world}
S=${*:-p1g2 p1g3 p1s1 p2s1 p3g4}
declare -A A=([p1g2]="map=planet:1:2 hand=guardwork" [p1g3]="map=planet:1:3 hand=guardwork" [p1s1]="map=planet:1:1 hand=squadwork" [p2s1]="map=planet:2:1 hand=squadwork" [p3g4]="map=planet:3:4 hand=guardwork" [p4s1]="map=planet:4:1 hand=squadwork" [p5s1]="map=planet:5:1 hand=squadwork")
for s in $S; do
  for t in 25 40; do
    for v in plain torch blast; do echo "$s|$t|$v|${A[$s]}"; done
  done
done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t v a <<< "{}"
  d='"$F"'/$n-$t-$v; mkdir -p $d
  e=$((t + 10))
  '"$B"' look=$d rounds=plain load=$t,$v at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-$v exit $?"'
echo WINDOWS DONE
