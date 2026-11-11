#!/usr/bin/env bash
# Scratch folder: $S (default /tmp/scr), with look/ inside. Paths as run were /tmp/claude-0/scr.
# usage: j.sh tag args...   (Jenson base: no food, no limit, 45/22.5/18)
tag=$1; shift
cd "$(git rev-parse --show-toplevel)"
./target/release/examples/bigdish preset=dense piles=1 food_deposit=0 crowd=0 walkers=1000000 deposit=0.5 rotate_angle=22.5 sensor_offset=18 report=2 threads=1 tag=$tag look=${S:-/tmp/scr}/look style=soft "$@" > ${S:-/tmp/scr}/$tag.txt 2>&1
