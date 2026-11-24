# The hunter, the third turret body (2026-11-24)

Squad and arsenal lane, for Direction's go of 11-24 12:33Z ("Then yes,
start the Hunter"), from the candidates sent at 11:35Z: picky targeting,
the slime on a crystal first.

What it is: `wiki/world/larder.md` (the turret bodies). Every 5 s it fires
one fast flat shell within 60 dish cells at the first of these on a clear
line: the slime on the nearest crystal it has taken (at least
`hunter_hold` 12, aimed at the thickest slime on it), a ripening fruit, a
mini-boss, a marching raid, the origin's heart, and last the thickest
slime it sees (`Game::hunter_mark`).

## Films

The Fork in squad mode (`mode=squads`) on software Vulkan (llvmpipe;
looks only, no timing). The squad holds at 125,280 on plain rounds. A
hunter with no cards stands from 20 s at 150,240. Frames 29.9 to 31.2 s
at 15 a second, which covers its third shell.

| Clip | Zoom | What to look for | Counts at 40 s |
|---|---|---|---|
| `plain6.mp4`, `plain6.jpg` | 6 | the tracer flying north to the crystal at 161,163 the slime is eating from, and the slime killed where it lands | 3 shells at the crystal, 1 at the slime (before it had taken a crystal); 116 walkers killed by its strikes |
| `plain12.mp4`, `plain12.jpg` | 12 | the long barrel and its scope turning to the mark and kicking, the muzzle flash, the tracer leaving | as above |

In the library test (`a_hunter_shoots_the_slime_on_a_crystal_first`,
race world, 16 s), all 4 of its shells went at crystals and killed 50.

## Fixes the first film made

The first take had three faults:

- At 150 dish cells a second the shell landed within the step it was
  fired, so it was never drawn. It now flies at 80, about 0.6 s over 40
  cells, where a mortar's takes 1.7 s.
- One stray walker on a crystal counted as held, so every shell went at a
  far crystal crossed by a few specks. Now it takes `hunter_hold` 12.
- It aimed at the crystal's middle while the slime ate round the rim:
  7 killed in 4 shells. It now aims at the thickest slime within the
  crystal's radius plus 2 cells.

The window also never turned a mortar or a hunter to the shell it had
just fired (`turret_fired` compared ticks with `>`). Mortars hid this
because they turn to their aim. A hunter keeps no aim between shots, so
it pointed at the slime's core the whole time.

```
# film.sh NAME CARDS ZOOM FROM SECS CX,CY
at=$(python3 -c "print(','.join('%.4f' % ($from + i / 15) for i in range(int($secs*15)+1)) + ',40')")
larder-world look=$out mode=squads hand=hold@125,280 bank=8000 rounds=plain at="$at" centre=$c,$zoom \
  hunter=150,240,20 screen=960x600
```
