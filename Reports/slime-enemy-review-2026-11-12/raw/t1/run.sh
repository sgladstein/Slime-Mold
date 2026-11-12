#!/bin/bash
# T1: the one-walker-a-cell rule held on (slots 50k <= half the agar) vs the earlier films (slots 250k: rule off).
cd /home/user/Slime-Mold
B=target/release/larder-world
AT=$(seq -s, 2 2 60),70,80,90,100,110,120
D=raw/t1
mkdir -p $D/pure-on $D/dense-on
$B look=$D/pure-on hand=idle map=fork at=$AT screen=1280x800 walkers=25000 start=spread food=0 upkeep=0.00001 capacity=50000 > $D/pure-on.log 2>&1
$B look=$D/dense-on hand=idle map=fork at=$AT screen=1280x800 walkers=25000 upkeep=0.0002 capacity=50000 > $D/dense-on.log 2>&1
echo done
