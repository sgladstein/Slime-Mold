#!/bin/bash
# necks.sh OUTDIR: the gun-card plan's cut count (2026-11-17, "before any
# Cut card is built, count how often a run planet has a part worth
# cutting"). Each run planet 1-8 (run seed 1, `run=1:P`), two hands: idle
# (the slime as it grows with nobody shooting) and squadwork (squads that
# take crystals and fight). At 60, 120, 180 and 300 s the headless key
# `necks=` prints the walkers in parts of the slime that only necks 2 or 4
# dish cells wide tie to a crystal with charge (`mould::necks`), with a
# frame of the whole map at each time to look at. A game that ends early
# (the squad wiped) stops counting. Run from the repo root after
# `cargo build --release --bin larder-world` (or B=a copy of it); table:
# `python3 necks_table.py OUTDIR`.
F=${1:?OUTDIR}; mkdir -p "$F"
B=${B:-./target/release/larder-world}
for p in 1 2 3 4 5 6 7 8; do for h in idle squadwork; do echo "$p|$h"; done; done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r p h <<< "{}"
  d='"$F"'/p$p-$h; mkdir -p $d
  '"$B"' look=$d run=1:$p hand=$h necks=60,120,180,300 at=60,120,180,300 screen=960x600 > $d/log.txt 2>&1
  echo "p$p-$h exit $?"'
echo NECKS DONE
