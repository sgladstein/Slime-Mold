#!/bin/bash
# windows.sh OUTDIR: Provoke and Still in play, in paired 20 s windows
# (the owner's measure for a card, 11-16 14:24Z: paired windows from one
# game). Run planets 1-3 (`run=1:P`, run seed 1), the squad-mode hand, where
# the slime raids and charges the squads and wipes them between 1 and 3
# minutes (the cut count's census). Each game plays to T (50 or 70 s); at T
# every squad loads the variant's deck (`load=T,CARDS`) and the window runs
# to T + 20 (Provoke's status lasts 12 s and a provoked part panics only
# after 6.5 s, so 10 s windows were too short). Variants: plain; provoke
# (provoke+plain: every other bullet provokes); still (still+plain).
# Run from the repo root after `cargo build --release --bin larder-world`
# (or B=a copy); table: `python3 windows_table.py OUTDIR`.
F=${1:?OUTDIR}; mkdir -p "$F"
B=${B:-./target/release/larder-world}
declare -A D=([plain]=plain [provoke]=provoke+plain [still]=still+plain)
for p in 1 2 3; do for t in 50 70; do for v in plain provoke still; do echo "$p|$t|$v|${D[$v]}"; done; done; done | xargs -P 4 -I{} bash -c '
  IFS="|" read -r p t v deck <<< "{}"
  d='"$F"'/p$p-$t-$v; mkdir -p $d
  '"$B"' look=$d run=1:$p hand=squadwork rounds=plain load=$t,$deck at=$(python3 -c "print($t + 1 / 30)"),$((t + 10)),$((t + 20)) screen=320x200 > $d/log.txt 2>&1
  echo "p$p-$t-$v exit $?"'
echo WINDOWS DONE
