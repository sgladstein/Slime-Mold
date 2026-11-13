# Raw files for the wading report

All from `larder-world look=...` (shape C, the Fork, seed 1, software
Vulkan) unless named otherwise; each file holds the game's own t lines and
the squad lines (walkers touching, health) at each saved frame.

- `clip-*.txt`: the page's clips. `push-before` is `hand=push wade=30` (the
  old rule's count on C), `push-now` the default; `hold-sink0` and
  `hold-sink1` are `hand=hold@318,148` with `sink=0` and the default.
  Made by `run.sh`; `clip.py` cuts the side-by-side videos.
- `preview-*.txt`: runs before the clips. `a0`/`a1` is the dropped
  arming rule off/on (`a1` always with wading whole, `w6`; `a0` with the
  old count, `w30`), `b0`/`b45` rays or bullets. `r-` is `hand=rush`,
  `idle-` four idle soldiers, `p-hold`/`p-push` the hold (at the larder)
  and push hands, `h2-` hold at 277,155, `pl-` the push to its wipe, `r-w6`
  the rush with wading alone.
- `clip-hold-seen.txt`, `clip-hold-seen-sink0.txt`: the same stand with
  aims only at slime a unit can see (the PR after #197), sink 1 and 0;
  `preview-near-s0/1.txt` a stand nearer the channel, sink 0 and 1.
- `aimcost.rs`: the CPU cost probe for the aim's sight check (copy into
  `examples/` to build); `aim-cost/` its output (`before.txt`, `after.txt`, `alternating.txt`)
  at #197's head and with the check.
- `smoke-c-aim.txt`: the smoke games with the check (the wading smoke's
  command at the default `wade`).
- `thickprobe/`: `examples/thickprobe.rs` on the CPU race world, seeds 1-4,
  drain x1, squads of 10 and 40; `new` is arming with wading whole, `old`
  neither. `new-n10.txt` there was first run with rations and overrun on,
  which ended seeds 1, 2 and 4 before the order; it was rerun without.
- `arming-not-merged.patch`: the dropped rule against main 23601fc.
- `smoke-c-wade6.txt`, `smoke-c-wade30.txt`: the smoke games
  (`examples/larderworld.rs census`; the command is in the report).
