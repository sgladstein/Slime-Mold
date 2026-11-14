# Calmer hits: smaller shot lights (2026-11-14)

The owner played main at 20:03Z: "It seems like all of the bullets have new large area animations. even the sniper which is just range and pierce send outs these big circular affects. It is not good." At 20:09Z: "I think it is just your animation for when bullets hit. It changes but each bullet that hit has a huge wave that goes out. it is too much".

## Causes

Two separate causes produced the effect.

1. **Shot lights** (`src/bin/larder-world/draw.rs`). Each bullet in flight carried a light that reached 7 cells at strength 0.9.
   - Over the slime, that light is a bright disc 14 cells across. A piercing bullet flies on through the body after its hit, so the disc swept outward from every hit.
   - A plain hit lit 4 cells, a soldier's muzzle 5, a turret's 6, and a payload burst 3 of its radii (after Noita's 8).
   - Since #288 every shot carries a card, so a payload squad also lit a burst on every hit.
   - Turning off the bullet and hit lights removed the discs on the sniper. The check was a diagnostic build on Fork seed 1, with the camera at zoom 7 from 0:17.5.
2. **Fire in the plants** (#290, merged 19:40Z). Stand-in plants are on by default. Each blast calls `ignite` on a ring two cells past its crater, and fire spreads as a glowing halo.
   - With no recharge, a blast round blasts every other shot. On `lull` at 0:18, a wide band of fire crosses the screen. With `plants=0` the same scene has none.
   - Fire is the plants thread's to tune. It was sent the owner's messages at 20:15Z.

## Change

Shot lights are now a quarter of their old size and dimmer. `SHOT_LIGHT` is 2.5 cells.

| light | before | after |
|---|---|---|
| bullet in flight | 7 cells × wide, 0.9 (quiet 0.35) | 2.5 cells × wide, 0.5 (quiet 0.2) |
| plain hit | 4 cells, 0.8 | 2.5 cells, 0.5 |
| soldier's muzzle | 5 cells, 0.7 | 2.5 cells, 0.5 |
| turret muzzle | 6 cells, 0.9 | 3.75 cells, 0.6 |
| payload burst | 3 radii, f²(1.6 + killed/120 up to 2 + thrown) | 1.5 radii, f²(1 + killed/200 up to 1 + thrown/2) |

Nothing in the simulation changed. The lights only shade the slime under them.

## Films

Fork map, seed 1, eight soldiers holding from 0:17.5 to 0:18.7 at zoom 7, on llvmpipe. The page is `shot-lights-2026-11-14/index.html` ([Artifact](https://claude.ai/artifact/5AXYwqMaM1ztLkcm83bX8v)). It shows:

- the sniper before and after;
- `lull` before and after;
- `lull` at 0:18 with plants on and with `plants=0`.

## Checks

The window tests (29) and clippy pass. Frame cost is unchanged: the same number of lights, each over a smaller area.

## Follow-up: each light the size of its damage (20:22Z)

The owner, after #296: "the blast light affect should only be the size of the damage. The same for all affects."

| light | #296 | now |
|---|---|---|
| bullet in flight | 2.5 cells × wide | its strike's radius (`Bullet::r`: 2.5 cells for a standard bullet, a rolled bullet's damage widens it) |
| plain hit | 2.5 cells | its strike's radius (`Shot::r`, new) |
| payload burst | 1.5 radii (a bomb's of its throw ring) | its own radius, a bomb's kill core |
| soldier's or turret's muzzle, a lodged cast's ember | 2.5 or 3.75 cells, 3 cells | 1 cell (`MUZZLE_LIGHT`; no damage of their own) |

The full suite (446 + 29 + 39) and clippy pass.
