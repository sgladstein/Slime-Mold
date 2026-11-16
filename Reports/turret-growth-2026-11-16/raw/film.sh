#!/bin/bash
# film.sh BIN OUTDIR NAME [CARDS]: the shove page's kill box (the Fork,
# squad mode, `hand=hold@150,265`, three turrets at 172,240 180,240 188,242
# standing from 29 s) with the squad on plain rounds, and from 29 s the
# turrets loaded with CARDS (`tload=29,CARDS`; none: plain turrets). Frames
# 29-32 s at 30 a second for the clip, plus 39 s for the 10 s counts.
bin=$1; out=$2; name=$3; cards=$4
start=29
at=$(python3 -c "print(','.join('%.4f' % ($start + i / 30) for i in range(91)) + ',39')")
mkdir -p "$out/$name"
extra=()
[ -n "$cards" ] && extra+=("tload=$start,$cards")
"$bin" look="$out/$name" hand=hold@150,265 bank=8000 rounds=plain at="$at" centre=170,252,10 \
  turret=172,240,$start turret=180,240,$start turret=188,242,$start screen=960x600 "${extra[@]}" > "$out/$name/log.txt" 2>&1
echo "$name exit $?"
