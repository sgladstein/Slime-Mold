#!/bin/sh
# The build pages' pictures (Reports/build-pages-2026-11-18): the menu's
# pages over planet 1, as a PC whose last round was on main at ecacd8f
# (2026-11-17, #450) sees them, with the third thing to try (camera shake
# and hit-stop, on 11-18) ticked, and a round with an experiment on.
#   film.sh GAME OUT     (from the repo's root; software Vulkan)
set -e
GAME=$1; OUT=$2
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
mkdir -p "$OUT"
python3 "$(dirname "$0")/film_settings.py" "$OUT/played.txt" ecacd8f 2026-11-17 3
shot() { # NAME ARGS...
  n=$1; shift
  LARDER_SETTINGS="$OUT/played.txt" "$GAME" look="$OUT/$n" run=1:1 mode=squads screen=1920x1080 "$@" > "$OUT/$n.log" 2>&1
}
shot main      hand=idle at=2 menu=main hover=188,361 &
shot news      hand=idle at=2 menu=news &
shot try       hand=idle at=2 menu=news-try &
shot main200   hand=idle at=2 menu=main ui=200% try=feel &
wait
shot news200   hand=idle at=2 menu=news ui=200% &
shot try200    hand=idle at=2 menu=news-try ui=200% &
shot play      hand=greedy at=20 try=feel &
shot play2     hand=greedy at=20 try=feel try=crystal_regrow hover=1850,64 &
wait
# The page's pictures from the frames.
python3 "$(dirname "$0")/make_pics.py" "$OUT" "$(dirname "$0")/../page/img"
