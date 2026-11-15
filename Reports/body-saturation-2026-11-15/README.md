# Does the slime body's count saturate in dense slime? (2026-11-15)

The whole-game review (11-15, item W18) read `view.wgsl`'s `body()` and
judged that the drawn body saturates in dense slime, because `n0` (the
walkers that make a body two-thirds thick) scales only with the world's
average living density (`View::set_body_density`), so a hit in a dense
body would not show. It asked for `n0` normalised by local density, with
an A/B at zoom 6 and 14 and the frame cost.

**Measured: it does not saturate, so the change was not made.** Normalising
by local density would raise `n0` everywhere inside the body (the body is
always denser than the world's average) and thin the whole body, without
making a hit read better.

## How

`body()` itself is no longer called; the live count is `prep`'s `near`
and `wide` (`near = 1 - exp(-x)`, `x = 25 n / ws * kf^2 / n0`, the walkers
in a 5 x 5-cell window over `n0`). A diagnostic copy of the view shader
(`raw/make_sat.py`, swapped in with `LARDER_VIEW_WGSL`) colours each pixel
by `x`: blue under 1 (near under 0.63, where a kill shows in proportion),
green 1-3 (0.63-0.95), yellow 3-6 and red over 6 (saturated: a kill of
half barely changes it). `raw/classify.py` counts the pixels of each.

| Scene (`larder-world look=DIR ...`) | Body pixels | x < 1 | 1-3 | 3 and over |
|---|---|---|---|---|
| Fork hold, 30 s, zoom 6 (`hand=hold@150,265 bank=8000 rounds=plain centre=158,268,6`) | 129,874 | 93.9% | 5.3% | 0.0% |
| Fork hold, 90 s | 52,850 | 77.4% | 21.8% | 0.1% |
| Planet 1, 30 s, whole map (`map=planet:1:1 seed=1 mode=squads hand=squads`) | 103,514 | 97.6% | 2.2% | 0.0% |
| Planet 1, 90 s, near the squad | 109,897 | 86.0% | 14.0% | 0.0% |
| Fork squads, 3:00 (`mode=squads hand=squads seed=1`; mass 40,630, 9,908 thick) | 47,110 | 59.9% | 38.6% | 0.3% |
| Fork squads, 6:00 (mass 63,975, 14,834 thick) | 55,367 | 59.0% | 39.3% | 0.2% |

(About 1% more of each frame classes as white, over 12: soldiers' and
tracers' white, not body.) The world-average scale already does the work:
at 6:00, 49,141 living walkers on 512 x 320 cells make `n0` 59, so a body
cell with 5 walkers is at x about 2. With a fixed `n0` of 6 it would be 21.

## What does hide a plain hit

The body's own churn. At zoom 14 in the gun range with the spray off
(`range=7 hand=stand spray=0 centre=258,121,14`, frames 0.05 s apart, 2.5
to 3 s), each game step changes about 23,000 to 28,000 pixels of the
960 x 600 frame by more than 40 grey levels as the walkers move (a pair
of frames with no step between them changes under 3,000). A hole a shot opens is
one change among those. The gun look pass's spray (every killed walker
flies out of the hit back toward the shooter) is what makes a plain hit
read; how much of it flies is the owner's blind A/B
([spray A/B](../spray-share-ab-2026-11-15/README.md)).

## Files

- `fork-hold-30s.png`, `planet1-30s-all.png`, `fork-squads-3min.png`,
  `fork-squads-6min.png`: the false-colour frames (palette-reduced).
- `raw/make_sat.py` (run from the repo root: `python3 raw/make_sat.py
  OUT.wgsl`), `raw/classify.py FRAME.png...`.
