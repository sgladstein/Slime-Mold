#!/bin/bash
# play.sh BIN OUTDIR VARIANT: Tinder in play, one window filmed. Planet 2:1
# (the barrens, where plants hardly carry a fire), the squad-mode hand; at
# 25 s every squad loads VARIANT's deck (`torch` or `tinder-torch`), as in
# windows.sh's p2s1-25 window. Frames at 10 a second from 25 to 35 s, the
# near view at zoom 5 beside the squad (world 70,80). The window's counts
# come from the same game: windows.sh's p2s1-25 lines.
bin=$1; out=$2; v=$3
at=$(python3 -c "print(','.join('%.4f' % (25 + i / 10) for i in range(101)))")
mkdir -p "$out/play-$v"
"$bin" look="$out/play-$v" map=planet:2:1 hand=squadwork rounds=plain load=25,$v at="$at" centre=70,80,5 screen=960x600 > "$out/play-$v/log.txt" 2>&1
echo "play-$v exit $?"
