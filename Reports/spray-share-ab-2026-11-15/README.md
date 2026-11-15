# Spray share, a blind A/B (2026-11-15)

The whole-game review (11-15, item W26; `Reports/whole-game-review-2026-11-15/`
in PR #344) found plain hits close to invisible on main and asked for a few
walkers flung a hit, with a blind A/B of how many. The gun look pass
([gun-look-2026-11-15](../gun-look-2026-11-15/README.md)) already throws
every walker a shot kills out of the hit as a droplet. Flinging *living*
walkers was tried there and rejected (kills -22%, `dead-ends.md`), and the
review thread agreed. So the A/B here is how much of the kill flies.

Page: [Spray A/B](https://claude.ai/artifact/KmajibGgb5iWouC1fqyFHF)
(archived in `page/`). The key is in [`raw/ab-key.txt`](raw/ab-key.txt),
off the page.

## What was built

`squad::Frame::spray_share` (`Rules::spray_share`, default 1;
`spray=SECS,SHARE` in the window): each killed walker flies by its slot's
own draw (`big::spray_picked`, a hash apart from the flight's and pace's,
so fast and slow droplets are picked alike), and the rest die where they
stand. The engine takes the share as a fraction of 65,536 in the spray
row's last word, above the fine steps a tick (`spray_row`; Params did not
grow). The kills are the same at any share.

## Measured

- **CPU = GPU.** The squad twin test has a case at a third
  ("plain, a third of the kills spray"): 19 killed with and without the
  spray, 6 thrown (0.32), 3 in flight after the ticks, 0 flung marks off;
  the other spray cases are unchanged (28/28, 22/22, 19/19).
- **The films**: the gun range (`range=7 hand=stand`, zoom 12, 2.5-4 s):
  16 shots, 175 killed by 4 s in both. The Fork hold (`hand=hold@150,265
  bank=8000 rounds=plain`, zoom 12, 30-31.5 s): 226 shots, 1,230 killed by
  30 s in both; mould 24,414 with every kill flying and 24,412 with a
  quarter (droplets still in the air count as mould).

## Files

- `page/index.html` and the four clips (20 fps, frames 0.05 s apart, as
  the review asked).
- Frames: `larder-world look=DIR range=7 hand=stand [spray=0.3,0.25]
  centre=258,121,12 at=2.5,2.55,...` and `hand=hold@150,265 bank=8000
  rounds=plain [spray=0.3,0.25] centre=158,268,12 at=30,30.05,...`; clips
  by ffmpeg at 20 fps (as `gun-look-2026-11-15/raw/make_clips.py`, at 20).
