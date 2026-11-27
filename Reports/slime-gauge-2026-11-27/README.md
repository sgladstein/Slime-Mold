# The slime gauge (2026-11-27)

The owner, 11-27 05:42Z: "The size of the slime should be shown much more
visually and intuitively, it should be easy to tell how big it is, compared
to its start size and peak size. Everything should be as unobtrusive on the
screen as possible ... I saw a planet where the top center slime goal text
was too long and it went onto multiple lines; that shouldn't happen."

Page: https://claude.ai/artifact/9dhVLdDGiE4pAq3MrFsyzU (source `index.html`,
pictures beside it).

## What changed

- In a run the top bar's middle is the slime's gauge (`ui::SlimeGauge`,
  `gauge_bar`, built by `slime_gauge` in the window's `draw.rs`): a bar the
  width of the slime's peak, filled to its walkers now, a white mark at the
  landing's first count (hidden while that is the peak), the last
  `GAUGE_CUT_SECS` (3) counts' cut pale on the fill's end, the Swarmer's
  last-stand line as a red mark (its "N% of its peak, all rush at 25%" pill
  went), the slime's name and its count. Hover gives the numbers.
- The goal sentence is no longer drawn in a run; `Model::objective` keeps
  the mission's own words for the pause menu's THE GOAL.
- The heart and the stalemate, tails on the goal sentence until now, are red
  pills (`Model::alarms`, `run_alarms`) beside the tell and the twist.
- All pills ride in the top bar's row where they fit, else in one row under
  it; the middle stays centred when a pill comes or goes. Outside a run a
  long goal shrinks to 12 px and is then cut (whole on hover) instead of
  taking a row of its own, which only a middle under `GOAL_LEAST` units keeps.
- The slime's count left the resources pill; the run's shop points show once
  above 0 or with some coming in.

## Checks

- `world::ui::tests::the_slime_gauge_keeps_its_row_and_off_the_numbers`:
  four screens (1280x800 to 3840x2160) at all ten UI sizes; the gauge's count
  and three pills on screen and off every other word; one row on 1080 lines
  and up to 125%.
- The world UI tests (437) and the three window tests that read the goal pass;
  clippy clean.
- Films (software Vulkan, run seed 1): planets 2, 3 and 4 before, the same
  after, planet 2 at 200%, and a 0:04 to 2:00 clip of planet 2.

## Rejected

- A white landing mark at the bar's very end: on planet 1 the slime stayed at
  its landing size and the mark read as a divider before the count.
- Centring the gauge and its pills as one group: the gauge jumped when the
  cold snap's pill went.
