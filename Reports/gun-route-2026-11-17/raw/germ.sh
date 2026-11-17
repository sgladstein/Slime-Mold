#!/bin/bash
# germ.sh BIN OUTDIR: how far one germ shot spreads (the game designer's
# check, 11-17 05:02Z), through plain shape C and through the Sporer: the
# race world (seed 0, idle hand, plain rounds), one free germ cast on the
# Fork's thick lobe at 20 s (`boom=20,363,244,germ`) against none; the
# slime's mass at 20, 25, 30 and 40 s.
B=${1:?BIN}; F=${2:?OUTDIR}
SP="spore_scent=1 spore_reach=36 spore_land=1 spore_deposit=0.2 spore_rate=0.00001 spore_burst=0.05 spore_alarm=1 alarm_keep=0.98"
{
  echo "c-none|"
  echo "c-germ|boom=20,363,244,germ"
  echo "sporer-none|$SP"
  echo "sporer-germ|$SP boom=20,363,244,germ"
} | xargs -P 4 -I{} bash -c 'IFS="|" read -r n a <<< "{}"; mkdir -p '"$F"'/$n; '"$B"' look='"$F"'/$n hand=idle rounds=plain at=20,25,30,40 screen=320x200 $a > '"$F"'/$n/log.txt 2>&1; echo "$n exit $?"'
for n in c-none c-germ sporer-none sporer-germ; do echo "$n: $(grep -o 't [0-9]* s: mould [0-9]*' $F/$n/log.txt | sed 's/: mould/:/' | tr '\n' ' ')"; done
