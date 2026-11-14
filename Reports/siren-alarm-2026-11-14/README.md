# A siren's call drawn from the real alarm field (2026-11-14)

The art director's follow-up to the building looks (#271). There, a siren's call was two rings drawn out to `siren_r` from the call's time and radius, which is a drawn effect under the owner's rule against painted pulses. The ask was to drive the call from the engine's real alarm field.

Page: https://claude.ai/artifact/MFM16Vc9siMWV4gYyNC3UQ (source in `page/`).

## What changed

- **Field in the view.** The view now binds the engine's two germ fields at bindings 25-26 (`View::set_alarm`). The alarm is their second block of cells. A uniform says which side is current (`set_alarm_side`, `Engine::done` % 2, each frame) and gives the alarm that reads full (the siren's `siren_alarm`, 150). The engine and the look word are unchanged. The slime-look thread agreed the bindings and preferred them to the look word.
- **Drawing.** In `shaded`, a pale cool-white haze, 0.10 × (1 − e^(−a / (0.5 full))), plus a front by the field's slope, 0.45 × (1 − e^(−|∇a| / (0.12 full))). A burst's disc therefore shows its edge, which softens, widens and fades as the engine diffuses it (3×3 mean × `siren_keep` 0.95 a tick), in about 1.5 s. Other alarm in the field shows too.
- **Colour.** The haze is pale, not red, at the slime-look thread's request: red already means bolting walkers (specks go (235, 52, 40) when food news is stale), and a red rim confused the morning's black-spot report.
- **Siren mark.** The drawn rings are gone. The lamp still flares red with a light for `SIREN_CALL` (0.7 s), since that is the building's own state.

## Measures

- Films: llvmpipe, fork seed 1, `hand=idle` with the #271 staged buildings (`siren=120,178` and the rest), compared against #271's head b2af744. Zoom 12 is `centre=120,178,12` from 0:07.9 to 0:09.4; zoom 6 is `centre=115,185,6` from 0:07.8 to 0:10. One call lands at 0:08.0. The GIFs were re-encoded with a palette taken from six frames and dithered, because a single middle frame's palette turned the pale haze green.
- Frame cost: llvmpipe, paired `bench=30 hand=build`, one pair each, GPU tail p50.
  - First cut, five bilinear samples a world pixel: 86.7 → 95.3 ms (+10%), rejected.
  - With an early-out where the field is under 1% of full (one bilinear sample there): 84.5 → 87.7 ms (+3.8%).
  - The 1070 timing waits for the PC.
- Test: `the_alarm_field_shows_its_front_on_the_current_side` (view, GPU-gated). A disc of alarm lifts its edge most (more than 60 over bare and 30 over its middle) and leaves far outside within 6. The other side, or no alarm bound, shows nothing.
