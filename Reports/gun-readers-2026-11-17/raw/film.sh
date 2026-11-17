#!/bin/bash
# film.sh BIN OUTDIR NAME X Y T0 T1 [ARGS...]: one of the paired reader
# clips (the gun-system review's step D, readers, 2026-11-17). Every clip
# plays the same game (the race world, seed 0, idle hand, plain rounds);
# ARGS are its free casts (`boom=T,X,Y,CARDS`, `Rounds::cast_cards_at`), so
# a pair differs only by the reader cast after the setter. Frames at 15 a
# second from T0 to T1, the near view at zoom 8 on (X, Y), and each second
# the walkers within 12 cells of it and from 12 to 24 (`watch=`).
bin=$1; out=$2; name=$3; x=$4; y=$5; t0=$6; t1=$7; shift 7
at=$(python3 -c "import sys; a, b = float(sys.argv[1]), float(sys.argv[2]); n = int(round((b - a) * 15)) + 1; print(','.join('%.4f' % (a + i / 15) for i in range(n)))" "$t0" "$t1")
mkdir -p "$out/$name"
"$bin" look="$out/$name" hand=idle rounds=plain watch=$x,$y,12 at="$at" centre=$x,$y,8 screen=960x600 "$@" > "$out/$name/log.txt" 2>&1
echo "$name exit $?"
