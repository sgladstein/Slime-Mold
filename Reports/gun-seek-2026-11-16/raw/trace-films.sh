#!/bin/bash
# trace-films.sh OUTDIR: miss-films.sh's paired clips and 10 s counts for
# the card as built (version 6, homing in the shot trace; the seek runs add
# $SEEK, extra keys, if set). Run from the repo root
# after `cargo build --release --bin larder-world`. Each pair plays one
# game with no cards to T, then plain or seek+plain: a guard squad on
# planet 1 (seed 3) from 40 s, and the Fork's greedy hand at its larder
# from 12 s. 4 s of frames, 30 a second, the near view at zoom 6.
F=${1:?OUTDIR}; mkdir -p "$F"
B=./target/release/larder-world
clip() { # NAME T CENTRE CARDS ARGS...
  n=$1; t=$2; c=$3; cards=$4; shift 4
  at=$(python3 -c "print(','.join('%.4f' % ($t + (i + 1) / 30) for i in range(120)))")
  mkdir -p "$F/$n"
  $B look="$F/$n" rounds=plain load=$t,$cards at="$at" centre=$c screen=960x600 "$@" > "$F/$n/log.txt" 2>&1
  echo "$n exit $?"
}
counts() { # NAME T END CARDS ARGS...
  n=$1; t=$2; e=$3; cards=$4; shift 4
  mkdir -p "$F/$n"
  $B look="$F/$n" rounds=plain load=$t,$cards at=$(python3 -c "print($t + 1 / 30)"),$e screen=320x200 "$@" > "$F/$n/log.txt" 2>&1
  echo "$n exit $?"
}
G="map=planet:1:3 hand=guardwork"
K="hand=greedy"
clip guard-plain 40 90,45,6 plain $G &
clip guard-seek 40 90,45,6 seek+plain $G $SEEK &
clip fork-plain 12 395,55,6 plain $K &
clip fork-seek 12 395,55,6 seek+plain $K $SEEK &
wait
counts guard-plain-10s 40 50 plain $G &
counts guard-seek-10s 40 50 seek+plain $G $SEEK &
counts fork-plain-10s 12 21 plain $K &
counts fork-seek-10s 12 21 seek+plain $K $SEEK &
wait
echo FILMS DONE
