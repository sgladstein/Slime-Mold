#!/bin/bash
# mortar_film.sh BIN OUTDIR: the mortar's first film (turrets thread,
# 2026-11-17). The Fork, squad mode, the squad holding at 150,265 on plain
# rounds; a mortar standing from 20 s at 70,196, about 60 dish cells behind
# it. Frames 30-35 s at 30 a second (the near view over the mortar and
# where its shells land), and 40 s for the counts (the `mortar t` line).
bin=$1; out=$2
at=$(python3 -c "print(','.join('%.4f' % (30 + i / 30) for i in range(151)) + ',40')")
mkdir -p "$out"
"$bin" look="$out" hand=hold@150,265 bank=8000 rounds=plain at="$at" centre=125,236,4 \
  mortar=70,196,20 screen=960x600 > "$out/log.txt" 2>&1
echo "exit $?"
