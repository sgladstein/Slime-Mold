#!/bin/sh
# The films behind this report (2026-11-17, the Enemy lane's switch
# settlement). Run from the repo root after
#   cargo build --release --locked --bin larder-world
# with OUT the folder for the frames; on a machine with no GPU,
# VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json (llvmpipe).
# The "before" stills are the first three commands on main at d08583d.
OUT=${1:?out dir}
LW=./target/release/larder-world
mkdir -p "$OUT"
f() { n=$1; shift; mkdir -p "$OUT/$n"; $LW look="$OUT/$n" "$@" > "$OUT/$n.log" 2>&1; }
# Planets 1 to 3 with the scripted rush hand, stills at 0:20, 1:00, 2:00.
f p1 run=1:1 mode=squads hand=greedy at=20,60,120
f p2 run=1:2 mode=squads hand=greedy at=20,60,120
f p3 run=1:3 mode=squads hand=greedy crustlook=1 at=20,60,120
# The Trunk idle, with its crust pull and without.
f p3idle-pull run=1:3 mode=squads hand=idle at=60,120,180
f p3idle-nopull run=1:3 mode=squads hand=idle crust_weight=0 crust_fast=0 at=60,120,180
# Eight rifles standing at a front: the walls form, then burn. A plain
# light on two set patches (light= takes dish cells, half the world units).
f p3fire run=1:3 mode=squads hand=hold@344,169 gun=rifle crustlook=1 light=76,187,58,4 light=76,158.5,60.5,4 at=75,76.2,77,78,80,84,90,100
# The same game with no fire: the control. It is the first try at the fire
# film, whose lights were given in world units, so they fell off the map
# and lit nothing.
f p3nofire run=1:3 mode=squads hand=hold@344,169 gun=rifle crustlook=1 light=76,374,116,8 light=76,317,121,8 at=75,76.2,76.5,77,78,80,84,90
# The clips: the same game at zoom 6 (the walls forming 0:20-1:15, then two
# Torch shots at 1:16) and at zoom 12 (six seconds of the fight, then the fire).
f w6 run=1:3 mode=squads hand=hold@344,169 gun=rifle crustlook=1 centre=345,140,6 boom=76,374,116,torch boom=76,317,121,torch at=$(seq -s, 20 1 75),$(seq -s, 76 0.5 100)
f w12 run=1:3 mode=squads hand=hold@344,169 gun=rifle crustlook=1 centre=345,140,12 boom=76,374,116,torch boom=76,317,121,torch at=$(seq -s, 60 0.1 66),$(seq -s, 76 0.25 90)
