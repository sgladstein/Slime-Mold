#!/bin/bash
# film.sh BIN OUTDIR NAME [ARGS...]: one clip of the game designer's
# crossing test (2026-11-17, 10:14Z: "food beyond a lit Tinder line,
# counting walkers that cross in 30 s, weight 0 against -5"). Every clip
# plays the same game: planet 1:3, idle hand, plain rounds. At 16 s a
# stream of walkers runs from the slime's arm by the big crystal north-west
# to the crystals across the meadow; the line L crosses it (40 world units,
# the crystals beyond, on its left). ARGS: `trail=16,$L,tinder,lit` lays a
# tinder line there and lights it whole; `fire_weight=-5` puts heat in the
# sensors. Frames at 10 a second from 15 to 46 s, the near view at zoom 4
# on the line's middle, and each second `cross=`'s counts: walkers beyond
# the line (within 30), walkers that crossed since the game began (minus
# the count at 16 s: those in the 30 s), cells burning within 6 of the
# line and the fire's kills.
bin=$1; out=$2; name=$3; shift 3
L=162.6,190.8,191.4,163.2
at=$(python3 -c "print(','.join('%.4f' % (15 + i / 10) for i in range(311)))")
mkdir -p "$out/$name"
"$bin" look="$out/$name" map=planet:1:3 hand=idle rounds=plain cross=$L,30 at="$at" centre=177,177,4 screen=960x600 "$@" > "$out/$name/log.txt" 2>&1
echo "$name exit $?"
# The clips:
#   film.sh BIN OUT none
#   film.sh BIN OUT heat-0 trail=16,$L,tinder,lit
#   film.sh BIN OUT heat-5 trail=16,$L,tinder,lit fire_weight=-5
#   film.sh BIN OUT alarm-50 trail=16,$L,tinder,lit fire_alarm=50
# (the last with a build that has the simulation-use thread's fire_alarm,
# C4; its heat-0 matched this branch's every second)
