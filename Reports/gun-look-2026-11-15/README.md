# The gun look pass (2026-11-15)

The owner, 2026-11-15 08:30Z: "The gun graphics need work. They should
really be one of the draws of the game. They need to be really
satisfying." The coordinator's brief: a big pass on muzzle, trails,
impacts, slime hits and blasts, all crisp at close zoom, after Noita
([Noita's spell look](../noita-spell-look-2026-11-14.md)); show clips.

Page: [Gun Look](https://claude.ai/artifact/MNvwqnxE9nqKr8hmfx26ce) (archived in `page/`).

## What changed

Every piece below is a game state drawn as it is (the art direction's
rule 1): the flash is the shot, the streak is the bullet's own flight, and
the spray is the slime the shot killed, still walkers in the dish.

- **Muzzle flash** (`Shape::Flash`, `marks::muzzle_flash`, `view.wgsl`
  `flash_at`): a white-hot ball at the barrel's mouth with jets by gun
  kind, coloured by the card's flame. Rifle: three prongs (a forward jet
  and two side jets). Carbine: shorter and wider. Long gun: a long lance
  with thin side jets. Scattergun: a fan of five. Each flash has its own
  jitter (by the soldier's shot count) and shrinks over `FLASH` (0.07 s,
  about 4 frames at 60 fps). It was a soft disc under the soldier's
  body. Its light stays 1 cell (`MUZZLE_LIGHT`, owner 11-14 20:22) and
  now sits at the mouth.
- **Recoil**: the soldier kicks back 0.14 of its body radius along its
  aim as it fires (`KICK`), easing back over the flash.
- **Tracers glide** (`App::between`): the drawn bullet moves between game
  steps at its own speed (15 steps a second, so a bullet used to jump),
  leaves the barrel (not the soldier's middle), eases onto the aim line
  over 4 body radii, and its core tapers to the tail. A hit's light waits
  for the drawn bullet to arrive (`Tracer::delay`) and is white-hot for its
  first half, then gold, at 0.9 falling as the square of what is left (it
  was gold at 0.5 falling straight from the step's start). A volley's hits
  share one hit's light: a scattergun's pellets each light their share, so
  a fan landing on one spot lights it as one hit does. In the first films,
  at 1.4 and summed, four pellets landing 2 cells out, or one rifle round
  in slime at a soldier's feet, lit the soldier and the ground round it
  white.
- **The spray** (`squad::Frame::spray`; `Rules::spray_secs` 0.3 s,
  `spray_reach` 5 cells): every walker a shot kills flies out of the hit
  as a droplet instead of vanishing. It flies straight out from the hit's
  middle, turned back toward the shooter when it would fly on the way the
  shot went (mirrored across the shot's line), so it shows over the open
  ground, not inside the bright body. Each droplet's flight and pace come
  from its slot (`big::spray_timer`, the longer half of the 9 fine steps;
  `big::spray_pace`, 0.35 to 1 of the first speed), and it slows to 0.4 of
  its speed as it lands (drag). It lays nothing and dies where it lands.
  It is counted killed at the hit, so a frame's kills are the same with
  and without it.
- **Flung walkers draw as droplets** (`draw_specks`): a round head zoom/4
  pixels across the radius, drawn out behind along 0.8 of a tick's flight
  (the engine's `fly` writes it in the walker's fourth word; half a cell
  to three), so a bomb's throw and a shot's spray read as matter flying
  and the frames join into one motion. They were squares, 2 zoom/4 + 1
  pixels a side.

## Measured

**CPU and GPU agree.** The squad twin test
(`the_squad_matches_the_cpu_for_one_frame`) has three new cases with the
spray on (3 fine steps, two ticks): sinking shots on the plain dish,
spores and dormancy, and condensed walkers. Kills with and without the
spray: 28/28, 22/22 and 19/19; thrown 28, 22 and 11 (condensed walkers
break instead); in flight after the ticks 17, 11 and 4; GPU = CPU on every
slot's life, state and timer, every walker's place (0 off), the flung
marks, trail and scent.

**In the game** (Fork, seed 1, `hand=hold@150,265 bank=8000 rounds=plain`,
software Vulkan): by 30 s, 221 shots killed 1,177 walkers before, after, and after with
`spray=0`; by 31 s, 1,259 both. Mould at 30 s: 24,412 before and with
`spray=0`, 24,415 after (a droplet lives until it lands). Squad mode by
31 s, each gun kind: rifle 149 shots 1,599 killed, long gun 93 and 1,379,
scattergun 436 and 1,514, the same before and after; carbine 178 and
1,662. Bombs by 30 s: 2,304 killed before, 2,330 after (bombs throw the
living slime, and the runs drift about 1%). Droplets in the air at once: 3 at the 30 s count; at about 80 kills a second and 0.15-0.3 s of flight,
about 12-25 at a time in a steady fight (derived, not counted).

**Frame cost** (llvmpipe, the bench, before and after): frame p50 62.1 -> 64.7 ms (+2.6 ms, +4%), p95 85.6 -> 90.2 ms; record
p50 0.37 -> 1.19 ms (the glide records every frame); GPU tail p50 54.0 ->
56.8 ms. One run each, 2,100 frames at 960 x 600, zoom 24, the same 248
shots and 1,473 kills by 35 s. The
1070's timing waits for the owner's PC (art direction, *How a look is
judged*).

## Rejected on the way

- **A splash that threw the living slime ahead of the hit and let it
  live** (out to 2 strike radii, past the strike, on the far side from the
  shooter, as a bomb's fling throws). Kills by 30 s fell 1,165 -> 908
  (-22%): the thrown walkers left the next shots' discs. Only 1-29 walkers
  flew at a time, mostly into the bright body, so it barely showed
  (`Reports/dead-ends.md`).
- **A spray flight by distance from the hit's middle, slowing to a stop**:
  most of a strike's walkers sit near its rim, so most droplets flew a
  fraction of a cell and the spray hung round the hole. Now by slot.
- **Screen shake on bursts**: not built. Noita shakes the camera on
  explosions (`camera_shake` 7.5), but in an RTS where every shot can
  carry a payload the camera would shake all fight. A small shake on bombs
  alone is an option for the owner.

## Files

- `page/index.html` and its clips: the Artifact page's source.
- Clips: `larder-world look=DIR hand=hold@150,265 bank=8000
  rounds=plain|bomb at=30,30.033,... centre=X,Y,ZOOM screen=960x600` on main
  (bef8f1fa) and on this branch; frames between steps are taken as the
  window would draw them (`headless`: `App::sub`).
- `raw/make_clips.py`: the frames to H.264 MP4 at 30 fps (`python3
  make_clips.py FILMS_DIR PAGE_DIR`, FILMS_DIR holding `before/` and
  `after/` film folders of `f000.png`...). Gun kinds: `mode=squads
  gun=rifle|carbine|long|scatter` at `centre=160,268,24`.
- Bench: `larder-world bench=35 hand=hold@150,265 bank=8000 rounds=plain
  centre=160,268,24 screen=960x600`, each build alone on the machine.
