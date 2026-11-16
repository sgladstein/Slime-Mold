#!/bin/bash
# film.sh BIN OUTDIR NAME ROUNDS [extra args...]: 3 s of frames from START
# (default 30) game seconds, 30 a second, the near view at CENTRE.
bin=$1; out=$2; name=$3; rounds=$4; shift 4
start=${START:-30}
at=$(python3 -c "print(','.join('%.4f' % ($start + i / 30) for i in range(90)))")
mkdir -p "$out/$name"
"$bin" look="$out/$name" hand=hold@150,265 bank=8000 rounds="$rounds" at="$at" centre=${CENTRE:-176,266,12} screen=960x600 "$@" > "$out/$name/log.txt" 2>&1
echo "$name exit $?"
