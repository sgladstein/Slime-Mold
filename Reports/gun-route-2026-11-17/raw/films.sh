#!/bin/bash
# films.sh BIN OUTDIR: the eight paired route clips, four at a time.
B=${1:?BIN}; F=${2:?OUTDIR}; D=$(dirname "$0")
SW="alarm_keep=0.98 alarm_weight=3"
SL="sleep_rate=0.01 sleep_deposit=0.3 wake_scent=1 wake_alarm=1 alarm_keep=0.98 alarm_relay=20 alarm_weight=3 sleep_rest=600 sleep_death=0.0001"
{
  echo "flinch-none|363 244 19.9 30|hand=idle"
  echo "flinch-flare|363 244 19.9 30|hand=idle boom=20,363,244,flare"
  echo "swarm-none|392 222 19.9 30|hand=idle $SW"
  echo "swarm-flare|392 222 19.9 30|hand=idle $SW boom=20,392,222,flare"
  echo "sleep-none|392 222 19.9 30|hand=idle $SL"
  echo "sleep-flare|392 222 19.9 30|hand=idle $SL boom=20,392,222,flare"
  echo "road-plain|142 239 9.9 20|hand=hold@150,265 bank=8000 load=10,plain"
  echo "road-road|142 239 9.9 20|hand=hold@150,265 bank=8000 load=10,road"
} | xargs -P 4 -I{} bash -c 'IFS="|" read -r n p a <<< "{}"; '"$D"'/film.sh '"$B"' '"$F"' $n $p $a'
echo FILMS DONE
