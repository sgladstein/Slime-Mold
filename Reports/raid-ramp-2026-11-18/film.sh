#!/bin/bash
# Films for the raid ramp (2026-11-18): planet 1 of run seed 1 with the
# pumps hand to 12:00, a whole-map frame every 6 game seconds, once with
# main before the change (OLD, a build of 1e99aa5f) and once with it (NEW),
# then the pump's smell at 0 and 8 (NEW) to 3:00, then the owner's own
# run seed on planets 1 and 2 with the run bot (NEW). Software GPU:
#   apt-get install -y mesa-vulkan-drivers
#   export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
# usage: film.sh OLD_BIN NEW_BIN OUT_DIR   (run from the repo root)
set -e
OLD=$1; NEW=$2; OUT=$3
AT=$(seq -s, 6 6 720)
mkdir -p $OUT/old $OUT/new $OUT/smell0 $OUT/smell8 $OUT/his1 $OUT/his2
$OLD look=$OUT/old run=1:1 mode=squads hand=pumps at=$AT > $OUT/old/out.txt 2> $OUT/old/err.txt &
$NEW look=$OUT/new run=1:1 mode=squads hand=pumps at=$AT > $OUT/new/out.txt 2> $OUT/new/err.txt
wait
for s in 0 8; do
  $NEW look=$OUT/smell$s run=1:1 mode=squads hand=pumps at=30,60,90,120,150,180 set=pump_smell=$s > $OUT/smell$s/out.txt 2> $OUT/smell$s/err.txt &
done
wait
# The owner's run of 11-18 (his play log's run seed).
$NEW look=$OUT/his1 run=1795015831:1 mode=squads hand=bot at=600 > $OUT/his1/out.txt 2> $OUT/his1/err.txt &
$NEW look=$OUT/his2 run=1795015831:2 mode=squads hand=bot at=300 > $OUT/his2/out.txt 2> $OUT/his2/err.txt
wait
