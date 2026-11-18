#!/bin/bash
# film.sh BIN ROOT OUT T0 SECS FPS [args...]: one clip's frames on software
# Vulkan (llvmpipe), run from ROOT (the checkout BIN was built from, for its
# data files). The four films of this report, each 1280x800 at 30 frames a
# game second, about 15 minutes for the four side by side on four cores:
#
#   film.sh MAIN_BIN MAIN_ROOT FILMS/before-z6  14.5 6 30
#   film.sh NEW_BIN  .         FILMS/after-z6   14.5 6 30
#   film.sh MAIN_BIN MAIN_ROOT FILMS/before-z12 15.5 6 30 centre=126,152,12
#   film.sh NEW_BIN  .         FILMS/after-z12  15.5 6 30 centre=126,152,12
#
# "before" is main at 41bb764; the near camera follows the squad at zoom 6,
# and centre= holds it on the fight at zoom 12.
BIN=$1; ROOT=$2; OUT=$3; T0=$4; SECS=$5; FPS=$6; shift 6
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
AT=$(python3 -c "print(','.join(f'{$T0 + i/$FPS:.3f}' for i in range(int($SECS*$FPS))))")
mkdir -p "$OUT"
SECONDS=0
cd "$ROOT" && "$BIN" look="$OUT" run=1:1 mode=squads hand=squads screen=1280x800 at="$AT" "$@" > "$OUT.txt" 2>&1
echo "$(basename $OUT): $SECONDS s wall, $(ls $OUT | grep -c near) near frames"
grep -E "^  t " "$OUT.txt" | sed -n '1p;$p'
