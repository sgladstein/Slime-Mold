#!/bin/bash
# Slow front prototype at density, one walker a cell held: step 0.15 in thin trail, 1 in thick (36 Points' step_mod).
cd /home/user/Slime-Mold
B=target/release/larder-world
AT=$(seq -s, 2 2 60),70,80,90,100,110,120
D=/tmp/claude-0/qs
mkdir -p $D/slow-dense $D/angle-dense
$B look=$D/slow-dense hand=idle map=fork at=$AT screen=1280x800 walkers=25000 upkeep=0.0002 capacity=50000 step=0.15 trail_scale=0.02 step_by=0.85:2 > $D/slow-dense.log 2>&1
$B look=$D/angle-dense hand=idle map=fork at=$AT screen=1280x800 walkers=25000 upkeep=0.0002 capacity=50000 sensor_angle=22.5 sensor_offset=5 > $D/angle-dense.log 2>&1
echo done
