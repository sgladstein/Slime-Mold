# Buildings that show their work, and soldiers that flinch (2026-11-14)

The art director's item after the turret look (#258): bring the lure, pump,
outpost and siren up to the turret's standard. Each should be soldier-scale
with an outline and rim light, read by its silhouette alone at zoom 6, and
show what it is doing from the game's state. Then soldiers' hit reactions: "a
flinch and a brief light where slime bites, from the real damage".

Page: https://claude.ai/artifact/VL9ogWubo1dD6anisCLZVa (source in `page/`).

## What changed

- **Size.** Every building is drawn one touch radius across, at least 11
  px. Before, they were 0.7 radii, smaller than a soldier's 1.15; the turret
  went first in #258. The larder went from 1.6 to 2.2 cells so it stays the
  biggest.
- **Lure.** While it burns (`working`, switched on), its light is a *scent
  light*: a `Light` with lift below 0, lit at each spot only as far as the
  engine's scent there (the look's byte 3, `look_scent`). It also glows a
  little of itself, 0.22, as the gas cloud does, because bare ground's dark
  albedo hid the light alone. So what glows is the scent field: it spreads
  while the lure burns and fades with the field when it stops. Its reach is
  1.6 × `lure_r` cells.
- **Pump.** The sprite turns whole (`Marks::building_facing`, facing in the
  second colour's word as the turret's) so its hose points to its crystal's
  middle. A cyan line runs from the crystal's middle to the hose tip, with a
  light on the crystal. Both are as strong as this frame's draw
  (`Building::drew`, new, read-only) against a full pump's (`pump_rate` ×
  frame time), so they dim as a crystal runs dry.
- **Outpost.** It is now a mini larder in its sandbag ring: the larder's roof,
  rack and lamp at 0.55 scale (`buildings.svg`). It flares its cells cyan for
  `BANKED` (0.6 s) when a load is banked there.
- **Larder flare.** It now comes from carriers too. `step_notes` sees a
  living soldier's pack empty, which only banking does, and flares the
  drop-off nearest it (`Game::drop_offs`, as the game's `drop_off`). Before,
  the larder flared whenever `harvested` rose, so a pump's draw or a load
  banked at an outpost lit the larder.
- **Siren.** On each real call (`working` and on, every `siren_every` 4 s)
  the lens and horns flare red. Two rings, the second 0.3 behind, ease out to
  the alarm's reach (`siren_r` 15 cells) over `SIREN_CALL` (0.7 s) and fade,
  with a red light. The store's flare colour was cyan, which turned the
  siren white-cyan, so the shader now flares red (`CALL_RED`) for anything
  but the larder and outpost. Before, the horn arcs were lit whenever the
  siren was switched on.
- **Bites.** `step_notes` adds up a soldier's hp lost while the slime touches
  it. Every `FLINCH_HP` (3 hp; each touching walker bites 2 hp/s) it flinches
  for `FLINCH` (0.2 s). The body jerks back a fifth of its radius from the
  bite, its side toward the bite flushes hot (`BITE_HOT`, in the soldier
  mark's second word: a byte of strength and a 16-bit angle), and a hot light
  falls there. The bite's side is `Soldier::aim`, the densest mould in sight,
  else the core. In the clip 3.2 walkers touch a soldier on average and 7 at
  most, so a soldier at an edge flinches now and then and one wading in
  flinches several times a second.

Window notes: `TurretLook` became `BuildingLook` (`blooks`; facing, shot,
call, bank), and `Look` gained hp, carry, hurt, flinch and bite. `step_notes`
runs wherever `turret_fired` does: each step the window sees, including
`catch_up`.

## Not done

- No painted pulse on the lure. Its scent is continuous, so a pulse would be
  a drawn rhythm. The glow comes and goes with the real field instead.
- The siren's rings are drawn from the call's time and radius, not from the
  engine's alarm field. That field is not in the view's look buffer, and
  adding it means changing the engine's look word.

## Measures

- Films (llvmpipe, fork seed 1, 1280×800 cropped). Main is `origin/main`
  57ed474 plus only the building-staging arguments.
  - `hand=idle centre=95,200,6`, 0:06 to 0:10.5, and `centre=108,190,12`,
    0:07.5 to 0:09.5, with `pump=55,224 lure=95,200 siren=120,178
    outpost=85,250 turret=135,208`. The siren calls at 0:08.0. The pump drew
    50 food in 10 s.
  - `hand=farm outpost=275,140 centre=275,135,6`, 0:31.5 to 0:33.5. One bank
    at the outpost at 0:32.4. Over 0:25 to 0:37 the larder flared about once
    a second and the outpost once.
  - `hand=idle centre=256,112,12` (and 6), 0:06 to 0:08: soldiers at the
    larder as the mould arrives.
- Frame cost: llvmpipe, paired `bench=30 hand=build`, main then this change,
  one pair. GPU tail p50 went 80.3 → 82.5 ms and frame p50 89.5 → 92.7 ms,
  inside the ±4 ms spread of the turret pair. Per pixel this adds a
  bilinear scent sample under a burning lure's light and a dot product on a
  bitten soldier. The 1070 timing waits for the PC.
- Tests:
  - `a_step_shows_a_siren_call_a_bank_at_an_outpost_and_a_bite` (GPU-gated):
    the siren's call, an outpost bank that does not flare the larder, a
    larder bank, no flinch under `FLINCH_HP`, one past it from the aim's
    side.
  - `a_pump_draws_its_crystal_into_the_bank` now also checks that the sum of
    `Building::drew` equals `pumped`.
- New film arguments: `lure=X,Y pump=X,Y outpost=X,Y siren=X,Y`, alongside
  `turret=X,Y`.
