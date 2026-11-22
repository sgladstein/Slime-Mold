# A laid mine's look (2026-11-22)

The owner: "When I place a mine, I don't see anything visible." The mine
was laid (`Kit::mines`) and worked, but nothing in the window ever drew it.
Now each laid mine is one `Marks::star` of kind `STAR_MINE`, painted in
`view.wgsl` (`star_at`, MINE):

- an olive casing (the team's colour) with a lit bevel, a cream rim on its
  lit side and a dark one on the other, a steel pressure cap, four bolts
  and a shadow, 0.8 of a soldier's drawn radius (so it keeps the soldiers'
  10-pixel floor when zoomed out);
- an amber lamp (the mine tool's aiming colour) that blinks every 1.4 s,
  faster and whiter as slime gathers in its reach (`Kit::mine_near`, the
  count `Kit::frame` already took each step, over the trigger);
- a faint dashed ring at the trigger reach (`mine_r`);
- the placement cue: over 0.6 s the ring sweeps in bright from a quarter
  larger, the casing drops in from a third bigger and the lamp burns
  steady.

Filmed on llvmpipe, `run=1:1 mode=squads hand=greedy`, with mines laid by
`LARDER_MINES=T,X,Y;..` (new, headless only):

- `mine-still.jpg`: zoom 6 at 0.1, 0.3, 0.7 s after laying and blinking;
  zoom 12 laying and resting.
- `mines-by-slime.jpg`: three mines at zoom 6, one on thin slime.
- `mine-lay.mp4`: zoom 6 and 12 side by side; the first ten frames are
  0.1 s apart (the lay), then one a second to 30 s, at 5 frames a second.

Cost: one mark a mine; no new readback or count (the walkers near each
mine were already counted each step to set it off).
