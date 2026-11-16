#!/bin/bash
# pull-films.sh OUTDIR: Pull stopping short (owner 2026-11-16 19:40Z: land
# the slime a few cells out in front of the squad, not on it;
# `RoundRules::pull_short` 6 dish cells). Run from the repo root after
# `cargo build --release --bin larder-world`; frames to clips with
# `python3 Reports/gun-shove-2026-11-16/raw/make_clips.py OUTDIR PAGE_DIR`.
#   close-*  the old pull clip's scene: the Fork, hold@150,265, one game to
#            29 s, then 3 s from 29 s, near view centre=170,258,10
#   march-*  the same game from 12 s, while the squad walks to its hold and
#            shoots slime at range: 3 s from 17 s, centre=140,215,5
#   *-10s    the same games counted over 10 s (no frames)
F=${1:?OUTDIR}; mkdir -p "$F"
B=./target/release/larder-world
S="hand=hold@150,265 bank=8000 rounds=plain"
clip() { # NAME LOAD FROM CENTRE CARDS
  n=$1; t=$2; a=$3; c=$4; cards=$5
  at=$(python3 -c "print(','.join('%.4f' % ($a + i / 30) for i in range(91)))")
  mkdir -p "$F/$n"
  $B look="$F/$n" $S load=$t,$cards turret=180,240,$t at="$at" centre=$c screen=960x600 > "$F/$n/log.txt" 2>&1
  echo "$n exit $?"
}
counts() { # NAME LOAD CARDS
  n=$1; t=$2; cards=$3
  mkdir -p "$F/$n"
  $B look="$F/$n" $S load=$t,$cards turret=180,240,$t at=$(python3 -c "print($t + 1 / 30)"),$((t + 10)) screen=320x200 > "$F/$n/log.txt" 2>&1
  echo "$n exit $?"
}
clip close-plain 29 29 170,258,10 plain &
clip close-pull 29 29 170,258,10 pull+heave+plain &
clip march-plain 12 17 140,215,5 plain &
clip march-pull 12 17 140,215,5 pull+heave+plain &
wait
counts close-plain-10s 29 plain &
counts close-pull-10s 29 pull+heave+plain &
counts march-plain-10s 12 plain &
counts march-pull-10s 12 pull+heave+plain &
wait
echo FILMS DONE
