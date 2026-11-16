#!/bin/bash
# film.sh BIN OUTDIR NAME: one of the paired Mind clips (the gun-system
# review's step D, 2026-11-16). Every clip plays the same game to 19.9 s
# (the race world, seed 0, idle hand, plain rounds); at 19.9 s every squad
# loads NAME (plain, blind, sour or spin), and at 20 s its round goes off,
# free, on the vein at (110, 264) with a 10 s status (`set=mind_secs=10`;
# the card's default is 6 s). Frames at 15 a second from 19.9 s to 31 s,
# the near view at zoom 8 on the hit, and each second the walkers within
# 8 cells of it and from 8 to 16 (`watch=`).
bin=$1; out=$2; name=$3; shift 3
at=$(python3 -c "print(','.join('%.4f' % (19.9 + i / 15) for i in range(167)))")
mkdir -p "$out/$name"
"$bin" look="$out/$name" hand=idle rounds=plain load=19.9,$name boom=20,110,264 set=mind_secs=10 watch=110,264,8 at="$at" centre=110,264,8 screen=960x600 "$@" > "$out/$name/log.txt" 2>&1
echo "$name exit $?"
