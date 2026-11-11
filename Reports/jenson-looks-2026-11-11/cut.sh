#!/usr/bin/env bash
S=${S:-/tmp/scr}
B="blob=0 deposit_cap=100 seconds=7 report=0.5 infect=4 sick_steps=1200 immune_steps=2400 sick_catch=5 sick_trail=3 sick_turn=2 sick_reach=0.5"
for s in 1 2 3 4 5 6 7 8; do
  $S/j.sh Cno-s$s seeds=$s $B
  $S/j.sh Ccut-s$s seeds=$s $B cut=4 at=5,6,7 style=sick
done
echo done > $S/cut.done
