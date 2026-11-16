#!/bin/bash
# counts.sh BIN OUTDIR: film.sh's six games again, frames only at 29, 32
# and 39 s, for the count lines (rounds casts, kills, turret kills);
# plaindeck loads the turrets with one plain card, so its count line prints.
bin=$1; out=$2
for v in "plain:" "plaindeck:plain" "blast:blast" "bomb:bomb" "gas:gas" "corral:corral+heave+plain" "multicast:multicast+bomb+blast"; do
  name=${v%%:*}; cards=${v#*:}; mkdir -p "$out/$name"
  extra=(); [ -n "$cards" ] && extra+=("tload=29,$cards")
  "$bin" look="$out/$name" hand=hold@150,265 bank=8000 rounds=plain at=29,32,39 centre=170,252,10 \
    turret=172,240,29 turret=180,240,29 turret=188,242,29 screen=960x600 "${extra[@]}" > "$out/$name/log.txt" 2>&1
  echo "$name exit $?"
done
