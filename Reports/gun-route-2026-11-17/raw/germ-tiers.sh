#!/bin/bash
# germ-tiers.sh BIN OUTDIR [KEYS...]: the germ retune's re-check (the game
# designer's call, 11-17 05:40Z: "A re-check of 3 tiers on seed 0 is
# enough"): every run planet's slime with the game's defaults (`tier=`; C
# is planets 1-3's shape with no tier), the race world (seed 0, idle hand,
# plain rounds), one free germ cast on the Fork lobe at 20 s
# (`boom=20,363,244,germ`) against none; the slime's mass at 40 s. KEYS go
# after the tier's (the tuning runs passed `germ_resist=` and
# `set=sick_upkeep=` here).
B=${1:?BIN}; F=${2:?OUTDIR}; shift 2; K="$*"
for t in c drifter sprawler trunk swarmer sporer sleeper grudge; do
  k=""; [ "$t" != c ] && k="tier=$t"
  echo "$t-none|$k"
  echo "$t-germ|boom=20,363,244,germ $k $K"
done | xargs -P 4 -I{} bash -c 'IFS="|" read -r n a <<< "{}"; mkdir -p '"$F"'/$n; '"$B"' look='"$F"'/$n hand=idle rounds=plain at=20,40 screen=160x100 $a > '"$F"'/$n/log.txt 2>&1'
for t in c drifter sprawler trunk swarmer sporer sleeper grudge; do
  a=$(grep -o 't 40 s: mould [0-9]*' $F/$t-none/log.txt | grep -o '[0-9]*$')
  b=$(grep -o 't 40 s: mould [0-9]*' $F/$t-germ/log.txt | grep -o '[0-9]*$')
  echo "$t: none $a, germ $b, down $(python3 -c "print(round(100 * (1 - $b / $a), 1))")%"
done
