#!/bin/sh
# Paired runs of examples/pathprobe: batch.sh BIN OUTDIR [secs]
# Planets 1-4 on run seeds 1-5 and 1795038793, 60 soldiers in 4 squads.
BIN=$1; OUT=$2; SECS=${3:-120}
mkdir -p "$OUT"
for d in 1 2 3 4; do for s in 1 2 3 4 5 1795038793; do
  echo "$d $s"
done; done | xargs -P 4 -n 2 sh -c "$BIN depth=\$0 seed=\$1 n=60 squads=4 secs=$SECS out=$OUT/p\$0_\$1.csv > $OUT/p\$0_\$1.log 2>/dev/null"
