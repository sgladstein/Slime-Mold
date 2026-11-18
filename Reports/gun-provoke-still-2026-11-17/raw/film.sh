#!/bin/bash
# film.sh BIN OUTDIR SCENE VARIANT ZOOM: Provoke and Still in play, one clip's
# frames. Run planet 1 (`run=1:1`, run seed 1), frames at 8 a
# second, 960 x 600. Each scene is one game filmed twice from the same
# second: once with plain bullets and once with the card in every other
# slot (`load=T,CARDS`), so the pair differs only by the card.
#
# - SCENE still (VARIANT plain or still): eight soldiers standing in the
#   open as one group (`hand=squadwork` without `mode=squads`, so they stay
#   together; in squad mode that hand splits up round a crystal and the
#   raid reads less clearly). The deck loads at 50 s, the slime surges at
#   54 s and raids the group at 59 s. Frames 56 to 72 s on it (338, 70).
# - SCENE provoke (VARIANT plain or provoke): the squad holds beside the
#   crystal the slime's stream feeds (`hand=hold@222,70`); the deck loads
#   at 14 s. Frames 16 to 32 s on the crystal and the squad (217, 64).
#   With plain bullets the slime's film grows over the crystal; provoked,
#   it believes it is cut off, leaves the crystal and charges the guns.
#
# ZOOM 6 or 12 (the lanes' rule: a clip at each). The counts beside each
# clip come from the same game's log (`log.txt`).
bin=$1; out=$2; scene=$3; v=$4; z=$5
case $scene in
  still)   hand=squadwork;     t0=56; load=50; c=338,70; mode= ;;
  provoke) hand=hold@222,70;   t0=16; load=14; c=217,64; mode=mode=squads ;;
  *) echo "scene: still or provoke"; exit 2 ;;
esac
deck=plain; [ "$v" != plain ] && deck=$v+plain
at=$(python3 -c "print(','.join('%.4f' % ($t0 + 1 / 30 + i / 8) for i in range(129)))")
d="$out/$scene-$v-z$z"; mkdir -p "$d"
"$bin" look="$d" run=1:1 $mode hand=$hand rounds=plain load=$load,$deck at="$at" centre=$c,$z screen=960x600 > "$d/log.txt" 2>&1
echo "$scene-$v-z$z exit $?"
