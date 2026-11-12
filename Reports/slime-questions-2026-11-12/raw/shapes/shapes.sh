#!/bin/bash
# Each shape preset in the window, idle squad on the Fork, headless default and walker mass on.
cd /home/user/Slime-Mold
B=target/release/larder-world
D=/tmp/claude-0/headless/shapes
mkdir -p $D
for s in a b c d e; do
  mkdir -p $D/$s
  $B look=$D/$s hand=idle shape=$s at=30,60,90,120,150,180,240 screen=640x400 > $D/$s.log 2>&1
done
echo done
