#!/bin/bash
# films.sh OUTDIR: every clip and 10 s count run on the seek page. Run from
# the repo root after `cargo build --release --bin larder-world`.
F=${1:?OUTDIR}; mkdir -p "$F"
B=./target/release/larder-world
R=Reports/gun-seek-2026-11-16/raw/film.sh
# One game to 29 s with plain bullets, then each loadout for 3 s.
for L in buckshot seek-buck split=split+blast seek-split=seek+split+blast; do
  N=${L%%=*}; C=${L#*=}
  START=29 $R $B "$F" "$N" plain load=29,"$C" &
done
wait
# A mini-boss formed at 29 s east of the squad (`boss=T,X,Y`).
for L in boss-plain=plain boss-seek=seek+plain; do
  N=${L%%=*}; C=${L#*=}
  START=29 $R $B "$F" "$N" plain load=29,"$C" boss=29,185,250 &
done
wait
# The 10 s counts, no frames.
for L in plain=plain seek=seek+plain buckshot=buckshot seek-buck=seek-buck split=split+blast seek-split=seek+split+blast; do
  N=${L%%=*}; C=${L#*=}
  mkdir -p "$F/$N-10s"
  $B look="$F/$N-10s" hand=hold@150,265 bank=8000 rounds=plain load=29,$C at=29,39 centre=176,266,12 screen=960x600 > "$F/$N-10s/log.txt" 2>&1 &
done
wait
for L in boss-plain=plain boss-seek=seek+plain boss-buck=buckshot boss-seek-buck=seek-buck; do
  N=${L%%=*}; C=${L#*=}
  mkdir -p "$F/$N-10s"
  $B look="$F/$N-10s" hand=hold@150,265 bank=8000 rounds=plain load=29,$C boss=29,185,250 at=29,39 centre=176,266,12 screen=960x600 > "$F/$N-10s/log.txt" 2>&1 &
done
wait
echo FILMS DONE
# The fly-over prototype (`Rules::seek_over`, on): the mini-boss scene's
# hunt from 34 s, and its 10 s counts.
for L in over-boss-buck=buckshot over-boss-seek-buck=seek-buck; do
  N=${L%%=*}; C=${L#*=}
  START=34 $R $B "$F" "$N" plain load=29,"$C" boss=29,185,250 &
done
wait
