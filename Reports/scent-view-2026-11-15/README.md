# The scent view (F2), 2026-11-15

Owner, 11-15 20:54Z: "Is there a way for me to view the mold scent?" There
was none in Larder World (the old `larder` demo had V; play.cmd doesn't open
it). Owner 20:56Z: yes to a toggle.

**What it is.** F2 in Larder World (`View::scent`; `scent=1` on the command
line for films) dims the world and washes over it the two fields a walker's
sensor adds (`sense_at` in `src/gpu/engine.wgsl`: trail plus scent): the
food scent, the look's byte 3 (1 - e^(-scent/60)), in blue with a contour
line every eighth of full; the trail, byte 0, in pink. Shader:
`scent_view` in `src/world/view.wgsl`. Contour widths are set in screen
pixels from the field's slope, so they are 1 to 3 px at any zoom (test
`the_scent_view_washes_scent_and_trail_and_its_lines_are_a_pixel_or_two_at_any_zoom`,
zoom 6 and 24). Crystals keep their drawn look; fully rock pixels only dim
(the first film drew the scent's lines along wall faces and hid crystals).

**What it shows** (Fork, seed 1, hand=rush, frames 2-31 s, zoom 5, llvmpipe):
the food scent is a tight halo of a few cells round each crystal; soldiers
carry a small scent of their own; over the rest of the map the trail is
what the slime follows.

**Cost.** Off: one flag test a pixel. On, llvmpipe 960x600, `bench=8
hand=rush`, alternating runs: GPU tail p50 off 90.99 / 88.78 ms, on
92.16 / 87.31 ms (within noise). Not yet timed on the GTX 1070.

Page: https://claude.ai/artifact/RhNbheKFwttFH2XpvUfH51 (copy: `index.html`
here with its clips and stills; frames from `larder-world look=DIR hand=rush
at=2..31 screen=960x600 centre=285,175,5 scent=0|1`, encoded with ffmpeg).
