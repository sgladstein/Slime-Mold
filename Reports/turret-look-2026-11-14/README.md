# Turrets that look like guns, 2026-11-14

The owner's playtest (08:15Z, via the art director): "turrets need to look
better". Before: one tile (an octagon with a head and barrels fixed toward
+x) drawn 0.7 touch radii, smaller than a soldier (1.15). It never turned,
so it never pointed at what it shot, and only a small lamp lit when it fired.

## What changed

- **A head that turns on its base.** The sprite is split: `turret` is the
  base (the concrete octagon and a bolt ring), and a new tile 16,
  `turret-head` (the atlas is now 17 tiles, 2176 x 160), is a bigger head
  with twin barrels. The view turns the head to the turret's facing, which
  the mark carries in its second colour word (`Marks::turret`).
- **The facing is the game's aim.** When the turret fires
  (`Game::turret_shots`, its own place) the head snaps to the shot's line,
  so the shot always leaves the barrels. Between shots it turns toward
  `Building::aim`, what it would shoot, at 5 rad/s (`TURRET_TURN`); with
  nothing in reach it holds. A new turret faces the mould's core.
- **The shot kicks and lights.** For 0.16 s (`TURRET_SHOT`) the head kicks
  back along the barrels by up to 0.22 radii, the muzzle flash shows, and a
  light at the muzzle lights the ground, as a soldier's flash does.
  Standing turrets are drawn after the bullets, so a turret's bullet (which
  the game starts at its middle) comes out of the barrels instead of lying
  over them. In ray mode the tracer starts at the muzzle.
- **State in the sprite.** A lamp on the head's back: dark when idle (no
  mould in reach), amber when tracking, red with the head greyed when the
  bank can't pay for a shot (`Rules::price`; shots are free by default, so
  today a turret is never dry). Rising (cream plan and seam) and hurt
  (scorch) are kept, and standing buildings get the soldier's rim light.
- **Drawn soldier-sized.** One touch radius (at least 11 pixels), from 0.7.
- **Range.** A faint dashed ring of the turret's reach round the turret
  under the cursor, and round the spot while placing one. Buildings can't
  be selected, so hovering stands in for selection. The ring is 72 short
  line marks, which claim only their own pixels (a ring mark walks its
  whole box).
- **Bars over figures.** A ghost turret is drawn before its progress bar,
  a standing one before its health bar, so the barrels never hide them
  (the first build hid both).

## Tried and changed

- The bullet streak first lay over the barrels: the game starts a turret's
  bullet at its middle. Drawing the turret over the bullets fixed it
  without moving the game's bullet.
- The muzzle light was first pushed before the frame clears its lights,
  so it never showed; it is now pushed with the turret after the bullets.

## Films

`larder-world map=fork hand=idle turret=135,208 centre=142,214,ZOOM`,
0:06 to 0:09, on llvmpipe. `turret=X,Y` is new: a turret standing whole
from the start (paid for, not raised), for films. With it the turret fires
16 shots in 10 s at the mould (36 shots without it, 52 with).

Page: [Turrets That Shoot](page/index.html) (https://claude.ai/artifact/MWPMvLxgCvwuVTtkAxu7nv).

## Measured

Test `a_turret_faces_its_shot_and_its_lamp_is_the_game_state`: a shot
turns the head down its line with a full kick; a tenth of a second turns it
0.5 rad toward its aim; the lamp reads tracking, dry with price 1 and bank
0.5, idle with no aim.

Cost: paired llvmpipe `bench=30 hand=build` (two turrets on screen), main then this change, twice: GPU tail p50 69.1 -> 65.1 and 67.0 -> 69.5 ms, frame p50 88.2 -> 83.0 and 86.5 -> 88.6 ms. No measurable cost (the turret samples its head tile in each outline tap and once more for the rim). The 1070 timing waits for the PC.
