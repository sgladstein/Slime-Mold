#!/bin/bash
# windows.sh OUTDIR [SCENES]: Tinder in play, in paired 10 s windows (the
# owner's measure for a card, 11-16 14:24Z: paired windows from one game).
# Each scene plays one game to T; at T every squad loads the variant's deck
# (`load=T,CARDS`) and the window runs to T + 10. The variants:
#   scorch          plain bullets with the scorch trail (it burns and
#                   lights a share of its line)
#   tinder-scorch   the same with tinder: each disc is laid, then lit
#   tinder-torch    torch bullets with tinder: the line burns from the hit
# Plain and torch are the element report's windows (same scenes, same
# build behaviour: with no laid fuel the plant rule is unchanged), re-run
# here on p1g2 as a check. The scenes are the readers' planet scenes
# (planets 1-3). VARIANTS="tinder-scorch tinder-torch" re-runs only those
# (2026-11-17, after the brush gap, `RoundRules::trail_fuel_gap`; the
# scorch windows have no tinder and stand). Run from the repo root after
# `cargo build --release --bin larder-world` (or B=a copy of it); table:
# `python3 windows_table.py OUTDIR`.
F=${1:?OUTDIR}; mkdir -p "$F"; shift
B=${B:-./target/release/larder-world}
S=${*:-p1g2 p1g3 p1s1 p2s1 p3g4}
declare -A A=([p1g2]="map=planet:1:2 hand=guardwork" [p1g3]="map=planet:1:3 hand=guardwork" [p1s1]="map=planet:1:1 hand=squadwork" [p2s1]="map=planet:2:1 hand=squadwork" [p3g4]="map=planet:3:4 hand=guardwork")
{
for s in $S; do
  for t in 25 40; do
    for v in ${VARIANTS:-scorch tinder-scorch tinder-torch}; do echo "$s|$t|$v|${A[$s]}"; done
  done
done
[ -z "$VARIANTS" ] && for t in 25 40; do for v in plain torch; do echo "p1g2|$t|$v|${A[p1g2]}"; done; done
} | xargs -P 4 -I{} bash -c '
  IFS="|" read -r n t v a <<< "{}"
  d='"$F"'/$n-$t-$v; mkdir -p $d
  e=$((t + 10))
  '"$B"' look=$d rounds=plain load=$t,$v at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 $a > $d/log.txt 2>&1
  echo "$n-$t-$v exit $?"'
echo WINDOWS DONE
