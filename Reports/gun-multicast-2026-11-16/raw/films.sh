#!/bin/bash
# films.sh OUTDIR: every clip and 10 s count run on the page. Run from the
# repo root after `cargo build --release --bin larder-world`.
F=${1:?OUTDIR}; mkdir -p "$F"
B=./target/release/larder-world
R=Reports/gun-multicast-2026-11-16/raw/film.sh
# `triple` and `multi-trigger` were presets when these were filmed; they
# are over a default gun's budget, so they load as card lists now.
for L in plain-blast multi-blast bomb-gas multi-bomb-gas \
         triple=multicast+multicast+plain+blast+gas \
         multi-trigger=multicast+trigger+plain+blast+gas; do
  N=${L%%=*}; C=${L#*=}
  START=29 $R $B "$F" "$N" plain load=29,"$C"
done
for L in bomb-gas trigger-bomb-gas multi-bomb-gas plain-blast multi-blast; do
  mkdir -p "$F/$L-10s"
  $B look="$F/$L-10s" hand=hold@150,265 bank=8000 rounds=plain load=29,$L at=29,39 centre=176,266,12 screen=960x600 > "$F/$L-10s/log.txt" 2>&1
  echo "$L-10s exit $?"
done
