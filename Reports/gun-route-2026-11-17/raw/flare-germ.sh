#!/bin/bash
# flare-germ.sh BIN OUTDIR: the game designer's Sleeper combo check (11-17
# 06:36Z: "Flare, then one germ cast on the wave as it wakes, seed 0, body
# lost 20 s after"). The race world, seed 0, idle hand, plain rounds, the
# Sleeper (`tier=sleeper`); a flare at 20 s on open ground beside its
# crust (392, 222, the flare clips' spot), a germ on the same spot as the
# woken wave arrives (24, 26 or 28 s), against no cast, the flare alone
# and the germ alone. The slime's mass at 20, 44, 46 and 48 s.
B=${1:?BIN}; F=${2:?OUTDIR}; P=392,222
{
  echo "none|"
  echo "flare|boom=20,$P,flare"
  echo "germ26|boom=26,$P,germ"
  echo "flare-germ24|boom=20,$P,flare boom=24,$P,germ"
  echo "flare-germ26|boom=20,$P,flare boom=26,$P,germ"
  echo "flare-germ28|boom=20,$P,flare boom=28,$P,germ"
} | xargs -P 4 -I{} bash -c 'IFS="|" read -r n a <<< "{}"; mkdir -p '"$F"'/$n; '"$B"' look='"$F"'/$n tier=sleeper hand=idle rounds=plain at=20,44,46,48 screen=160x100 $a > '"$F"'/$n/log.txt 2>&1'
for n in none flare germ26 flare-germ24 flare-germ26 flare-germ28; do
  echo "$n: $(grep -o 't [0-9]* s: mould [0-9]*' $F/$n/log.txt | sed 's/: mould/:/' | tr '\n' ' ')"
done
