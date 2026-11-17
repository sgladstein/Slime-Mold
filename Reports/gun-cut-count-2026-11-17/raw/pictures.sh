#!/bin/bash
# pictures.sh OUTDIR: the count again with pinholes filled (`mould::necks`
# after the first census, `necks.sh`, read speckle in wide sheets as necks),
# idle hand only (the first census's squadwork hand matched idle wherever
# both still played), run planets 1-8 at 60, 120 and 180 s, run seed 1.
# Each count prints the filled line and the first census's (pinholes open)
# and writes `planet-P-1-idle-TTTs-necks.png` (3 px a dish cell: red cut by
# a neck of 2, orange by a neck of 4 only, tan fed slime, violet slime
# already without food, blue food) beside the frame of the whole map.
# Run from the repo root after `cargo build --release --bin larder-world`
# (or B=a copy); table: `python3 necks_table.py OUTDIR`. About 45 min on
# four cores.
F=${1:?OUTDIR}; mkdir -p "$F"
B=${B:-./target/release/larder-world}
printf '%s\n' 6 8 7 5 4 3 2 1 | xargs -P 4 -I{} bash -c '
  p={}; d='"$F"'/p$p-idle; mkdir -p $d
  '"$B"' look=$d run=1:$p hand=idle necks=60,120,180 at=60,120,180 screen=960x600 > $d/log.txt 2>&1
  echo "p$p exit $?"'
echo PICS DONE
