#!/usr/bin/env bash
# The run's idle smoke check (whole-game review W9, 2026-11-15): on planets
# 1-3 of run seed 1, an idle squad must lose, or be clearly losing, by 5:00:
# here, at 300 s it has fewer soldiers than it landed with and the slime has
# more walkers than at 60 s. On llvmpipe about 3-6 min of wall time a planet.
#   Reports/run-progression-2026-11-15/smoke.sh BIN OUT [DEPTHS]
set -u
bin=$1; out=$2; depths=${3:-"1 2 3"}
mkdir -p "$out"; fail=0
for d in $depths; do
  "$bin" look="$out/d$d" run=1:$d hand=idle at=1,60,300 > "$out/idle-d$d.txt" 2>&1
  read -r s0 m60 s300 m300 end <<< "$(awk '
    /^  t 1 s:/   { split($0,a,"soldiers "); split(a[2],b,","); s0=b[1] }
    /^  t 60 s:/  { split($0,a,"mould "); split(a[2],b,","); m60=b[1] }
    /^  t 300 s:/ { split($0,a,"mould "); split(a[2],b,","); m300=b[1]; split($0,c,"soldiers "); split(c[2],e,","); s300=e[1] }
    /ended:/      { end=1 }
    END { print s0+0, m60+0, s300+0, m300+0, end+0 }' "$out/idle-d$d.txt")"
  if [ "$end" = 1 ] || { [ "$s300" -lt "$s0" ] && [ "$m300" -gt "$m60" ]; }; then v=ok; else v=FAIL; fail=1; fi
  echo "planet $d: soldiers $s0 -> $s300 by 5:00, slime $m60 at 1:00 -> $m300 at 5:00, ended early $end: $v"
done
exit $fail
