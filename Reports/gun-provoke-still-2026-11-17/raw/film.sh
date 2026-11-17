#!/bin/bash
# film.sh BIN OUTDIR VARIANT: Provoke and Still in play, one window filmed.
# Run planet 1 (`run=1:1`, run seed 1), the squad-mode hand: at 50 s every
# squad loads VARIANT's deck (plain, provoke+plain or still+plain), as in
# windows.sh's p1-50 window; the slime surges at 54 s and raids the squad
# at 59 s. Frames at 8 a second from 50 to 72 s, the near view at zoom 5 on
# the squad (341, 70). The window's counts come from the same game.
bin=$1; out=$2; v=$3
declare -A D=([plain]=plain [provoke]=provoke+plain [still]=still+plain)
at=$(python3 -c "print(','.join('%.4f' % (50 + 1 / 30 + i / 8) for i in range(177)))")
mkdir -p "$out/play-$v"
"$bin" look="$out/play-$v" run=1:1 hand=squadwork rounds=plain load=50,${D[$v]} at="$at" centre=341,70,5 screen=960x600 > "$out/play-$v/log.txt" 2>&1
echo "play-$v exit $?"
