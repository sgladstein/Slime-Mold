# You can see the slime attack a soldier (2026-11-18)

The owner, 11-18 18:04Z: "When the slime is attacking/damagin my soliders,
I cannot really tell. It just looks like walkers are running past them",
and 18:08Z: "The issue isn't that the bite was too brite or too weak it
just looks like more of the same slime." Also Look A4
(`wiki/looks/art-direction.md`): no effect bigger than its damage. Page:
`page/index.html`, published at
https://claude.ai/artifact/Wk5DevY7VGNnCAN21hAPcb (listed in
`Reports/pages.md`).

## Why a bite could not be seen

A soldier is bitten by every walker in the cells within its touch, 1.5
dish cells (`squad::read`, `Rules::touch`; the count is
`Soldier::touching`). Its figure is drawn 1.15 touches across the radius
(`soldier_r`), so it covered every walker biting it: what showed was the
slime round it, which does no damage, flowing past. The bite's own signs
were an orange light, 3.5 dish cells across the radius, on the side the
soldier aimed at, and an orange flush on that side of the figure: the
slime's own colours, on gold slime, and on the side it aimed rather than
the side it was bitten from.

## Now

- **The biting slime in its own colour.** While a soldier is bitten
  (`touching` above 0), `draw.rs` sends its touch disc as a bite zone
  (`Light::bite`: a light whose lift is `BITE_LIFT`, which lights
  nothing and goes first when lights are culled). In `view.wgsl`'s
  `shaded`, the slime in a zone (`bite_share`, by the slime's thickness)
  takes `BITE_SLIME`, a wet crimson (178, 18, 44), with a glow and a
  brighter sheen. Its strength throbs from 0.75 to 1 with each flinch.
- **Drawn over the soldier.** In `compose`, a living soldier's figure
  within its touch shows the world's own pixel over it where biting
  slime lies (`slime_on`, up to 0.85 by thickness), so the crimson slime
  climbs over its body. The head marks are drawn above, untouched, and a
  soldier not bitten looks exactly as before.
- **The soldier flushes red.** A flinch (every `FLINCH_HP` 3 of damage)
  mixes the whole figure 0.45 toward `BITE_HOT`, now blood red (235, 40,
  48), except the helmet, so a bitten squad can still be counted.
- **The orange bite light is gone.**

## Every light the fight makes, against its damage

From `src/bin/larder-world/draw.rs`, in dish cells:

| Light | Reach | Its damage | |
|---|---|---|---|
| A shot's hit | the hit's radius (`tr.r`) | the same | equal |
| A round in flight | its strike's disc (`Bullet::r`) | the same | equal |
| A burst | the blast's radius (`burst.r`) | the same | equal |
| A flare | half its alarm's reach | none (a signal) | its own rule, smaller |
| Muzzle flash, a waiting round's ember | 1 (`MUZZLE_LIGHT`) | none | a point light |
| A bite (to 11-18) | 3.5, centred a touch out | the touch, 1.5 | 2.3 times as far; now gone |

The bite zone is the touch itself, so nothing about a bite is now bigger
than its damage.

## Films

The Fork held at 150,265 with plain rounds (not squad mode), the two
windows where the hold is most bitten (a scan every 2 s from 10 to 50 s:
3.9 walkers touching a soldier at 24 s and 6.7 at 42 s, 1.1 at most
elsewhere). Built from main at 7fca93a with this change, the before from
the heads-on-top build (the same game).

| Scene | Zoom | Soldiers standing | Walkers touching a soldier (mean) | Crimson pixels a frame, before and after (frames with any) | Picture changed (mean, most) |
|---|---|---|---|---|---|
| The slime wades in, 41 to 45 s | 12 | 7, then 5 | 5.4 | 0 and 482 (0 and 63 of 121) | 4.13%, 10.78% |
| The same | 6 | the same | the same | 0 and 107 (0 and 62) | 0.98%, 2.54% |
| The first wave, 23 to 27 s | 12 | 8, then 7 | 3.3 | 0 and 743 (0 and 81) | 2.82%, 7.09% |
| The same | 6 | the same | the same | 0 and 161 (0 and 81) | 0.68%, 1.70% |

A crimson pixel has red of at least 120, green under 0.35 of the red and
blue above green; the slime's gold and orange and the blood's dark red
all have blue below green, so before there were none. Changed is the
share of the picture where any channel differs by more than 12 of 255
between the two drawings of a frame.

## Seen

- A bitten soldier is plainly hurt at zoom 6: its figure flushes red
  while its white helmet stays, and the orange wash over the squad is
  gone.
- The crimson slime shows on a soldier's body and at its edges while it
  is bitten, and is gone the moment the slime leaves it.

## Cost

`shaded` loops over the frame's lights once more for a pixel with slime
(`bite_share`, at most 64 lights), and `compose` once for a living
soldier's pixel within its touch. Not timed on the GTX 1070 (the
software renderer is not for timing).

## Files

- `page/`: the page and its clips (H.264, 30 fps, the middle 960 x 600 at
  its own pixels) with posters from each pair's most changed frame.
- `raw/film.sh OUT OLD_GAME` films them; `raw/make_clips.py OUT PAGE` makes
  the clips and prints the soldiers standing, the walkers touching, the
  crimson pixels and how much of the picture changed.
