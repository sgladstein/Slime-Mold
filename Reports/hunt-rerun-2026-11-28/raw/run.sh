#!/bin/bash
# arm seed
cd /tmp/claude-0/hunt
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
OFF="set=raid_every=0 set=fruit=0 set=wave=0 set=strikers=0 set=stand=0 set=last_stand=0 set=late_at=0 set=heart=0 set=star=0 set=boss_most=0"
arm=$1; seed=$2
case $arm in
  mk) P="plan=rush,cap=15,go=15,shop=squad,cards=0,kite=1.2"; X="";;
  hk) P="plan=rush,cap=15,go=15,shop=squad,cards=0,kite=1.2"; X="hunt_pull=30 trail_scale=0.05 step_by=1.5:1";;
  ms) P="plan=rush,cap=15,go=15,shop=squad,cards=0,kite=0"; X="";;
  hs) P="plan=rush,cap=15,go=15,shop=squad,cards=0,kite=0"; X="hunt_pull=10";;
  h10k) P="plan=rush,cap=15,go=15,shop=squad,cards=0,kite=1.2"; X="hunt_pull=10";;
  h30k) P="plan=rush,cap=15,go=15,shop=squad,cards=0,kite=1.2"; X="hunt_pull=30";;
esac
timeout 1500 ./target/release/larder-world league=2:$seed:300 $P $OFF $X > /tmp/claude-0/hr/$arm-$seed.txt 2>&1
