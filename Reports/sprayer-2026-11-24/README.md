# The sprayer, the second turret body (2026-11-24)

Squad and arsenal lane, for Direction's pick of 11-24 11:42Z: "Build the
Sprayer. ... the strip it lays must read on screen at zoom 6 as the
sprayer's work ... a Sprayer with Plain should be a weak buy."

What it is: `wiki/world/larder.md` (the turret bodies). Every 1.5 s it
sweeps a 60-degree cone at the slime within 24 dish cells and lays the
turrets' deck's next cast at 6 spots on the arc, on bare ground too.

## Films

The Fork in squad mode on software Vulkan (llvmpipe; looks only, no
timing). The squad holds at 125,280 on plain rounds; a sprayer stands
from 20 s at 150,240 with the turrets' deck set by `tload=0,CARDS`.
Frames 20.5 to 26.5 s at 15 a second, centred 158,250 (`film.sh` below).

| Clip | Deck | Zoom | What to look for | Counts at 40 s |
|---|---|---|---|---|
| `gas6.mp4`, `gas6.jpg` | Gas | 6 | the lilac jet fanning across the arc, then a band of gas on the ground; the slime's front pales asleep | 13 sweeps, 78 spots; 1,407 walkers gassed |
| `gas12.mp4`, `gas12.jpg` | Gas | 12 | the fan nozzle on the turret's base, the six jets | as above |
| `torch6.mp4`, `torch6.jpg` | Torch | 6 | gold jets, then a long scorched strip burned down the slime's arm; fire reaches near the squad when the slime is at its feet | 586 walkers killed by fire |
| `plain6.mp4` | Plain | 6 | the same jets, and only small nicks in the slime: the weak buy | 273 walkers killed (turrets') |

In the library test (`a_sprayer_lays_its_cast_in_a_strip`, race world,
12 s) a plain sprayer killed 93 to a plain turret's 206 in the same place.

```
# film.sh NAME CARDS ZOOM FROM SECS
at=$(python3 -c "print(','.join('%.4f' % ($from + i / 15) for i in range(int($secs*15)+1)) + ',40')")
larder-world look=$out hand=hold@125,280 bank=8000 rounds=plain at="$at" centre=158,250,$zoom \
  sprayer=150,240,20 tload=0,$cards screen=960x600
```

Seen: the jet starts at the nozzle after this film (it started half a
touch radius out); nothing else changed since.
