# You can see which slime a Mind round holds (2026-11-18)

Look A5 (`wiki/looks/art-direction.md`): a Mind status (blind, sour,
spin, provoke, still) reads on the slime it holds. Owed from the Look
lane's 11-18 reviews of Provoke and Still (#448): once the round's rose
flash fades, a stilled or provoked body has no look of its own. Page:
`page/index.html`, published at
https://claude.ai/artifact/2or1C2HtnaRqNMX7pbznFz (listed in
`Reports/pages.md`).

## Before

A Mind round (the gun cards' Mind family: Blind, Sour and Spin for 6 s,
Provoke for 12 s, Still for 10 s) gives every walker within `mind_r` of
its hit a status, one word a walker slot in the engine's `news` buffer
(kind in the top three bits, the step it ends in the low 29;
`engine.wgsl`'s `mind_of`). The view never read it: the only sign was the
round's pink tracer and its flash, so which slime was held, and for how
long, could not be seen.

## Now

- **Each walker under a status lays a soft rose disc** a cell across the
  radius (`MIND_HALO`, at least 1.5 px) in the specks' third screen
  (`draw_mind` in `view.wgsl`'s `draw_specks`; a value under 256, by
  `atomicMax`, so the discs of a held body make one haze and the walker
  itself is its brightest point). Its strength is the time left over the
  last 1.5 s (`View::set_minds`' `fade`), so the rose fades as the status
  runs out.
- **`base` tints the slime there toward `MIND_ROSE`** (255, 96, 196, the
  Mind cards' family colour), up to `MIND_TINT` 0.6: over the thick body
  by how much slime is under it, and on a lone walker's core by the
  disc's own strength, so scattered walkers show rose cores.
- **One hue for the family,** as the cards have one. Which status it is
  shows in what the slime does: blind walks straight, sour turns from its
  food, spin curls, still stops, provoked goes grey and charges.
- **Read from the simulation:** the view binds the engine's `news` and
  `state` (the step now) and is told where the words start
  (`gas_at` / 4 + `GAS_SLOTS` + walkers; one call in the window's
  `main.rs`, Engine and tools' file). The two `V` words that were spare
  since the alarm haze went (11-17) carry the offset and the fade.
- No switch: on in every game, drawn only where a walker holds a status.

## Films

The Fork held at 150,265 with 8,000 food (`hand=hold@150,265
bank=8000`), firing Spin rounds, then Still rounds (`rounds=spin`,
`rounds=still`; they kill nothing, so the slime wades on in), 22 to 26 s,
when the first statuses are on the slime coming in (the rounds land from
about 21 s). Centred north of the squad at 150,250. One build, from main
at ef56c00 with this change; the before is that build drawn with main's
`view.wgsl` (`LARDER_VIEW_WGSL`), which leaves the two words unread, so
both sides are one game.

| Scene | Zoom | Soldiers standing | Rose pixels a frame, before and after | Picture changed (mean, most) |
|---|---|---|---|---|
| Spin rounds | 12 | 8, then 8 | 296 and 2,651 | 4.15%, 8.73% |
| The same | 6 | the same | 121 and 1,713 | 5.39%, 5.85% |
| Still rounds | 12 | 8, then 8 | 344 and 3,595 | 4.96%, 7.81% |
| The same | 6 | the same | 92 and 3,328 | 4.75%, 5.53% |

A rose pixel has red of at least 140, blue of at least 90, and blue more
than 15 above green; the slime's gold and orange and the blood's dark red
have blue below green, so before only the rounds' own tracers count.
Changed is the share of the picture where any channel differs by more
than 12 of 255 between the two drawings of a frame.

## Seen

- At zoom 12 the walkers in the scattered fringe show rose cores where
  they were gold, and the thick body under a status takes a pink wash
  over its gold; slime beside it with no status is unchanged.
- At zoom 6 the held part reads as a rose patch on the gold.
- Before, the same frames show only the rounds' tracers in pink.

## Test

`slime_under_a_mind_status_turns_rose_and_fades_as_it_runs_out`
(`src/world/view.rs`): two pieces of gold slime, one under a Spin status;
the held one turns toward rose and the other keeps its gold, a status run
out draws nothing, and half way through its fade the rose is weaker, at
zoom 12 and 6. Against main's shader it fails (the held piece stays
gold).

## Cost

`draw_specks` reads one word a walker more, and a walker under a status
visits a box of about (2 x zoom)² pixels (576 at zoom 12) and writes the
disc in it by `atomicMax`; `base` does one compare a pixel. Not timed on the GTX 1070
(the software renderer is not for timing); it goes in the milestone's
batch.

## Files

- `page/`: the page and its clips (H.264, 30 fps, the middle 960 x 600 at
  its own pixels) with posters from each pair's most changed frame.
- `raw/film.sh OUT OLD_VIEW_WGSL` films them; `raw/make_clips.py OUT PAGE`
  makes the clips and prints the soldiers standing, the rose pixels and
  how much of the picture changed.
