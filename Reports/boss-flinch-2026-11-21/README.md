# A mini-boss flinches out of a hit (2026-11-21)

Direction's job from the Look lane's exit test 2: the wanderer read as a flat
disc and showed no hurt but a tint drawn on its members. Engine side only
(`Settings::boss_flinch`, engine.wgsl `lump_flinch`, GPU only); the view
side is the Look lane's.

**What it does.** A mini-boss's member standing on a fresh crater cell (one
a shot hurt but did not kill) runs down the crater's slope, out of the hole,
at 1.5 times its pace, for the first half of the crater's countdown (about
1 s; `boss_flinch` 0.5 in `run::enemy_keys`). With `crater_shut` keeping the
hole empty for the same time, a hit bites a hole into the body where it
landed; then the leash and the pull close it.

**Measured.**
- Race world (GPU test `a_mini_bosss_walkers_flinch_out_of_a_hit`, llvmpipe):
  a lump with 108 members within 3 cells of a hit kept 103 there a quarter
  of the crater later standing, 51 flinching; none died of it (179 in all
  both times).
- Planet 2, run 1795106014, every boss a wanderer, greedy: a wanderer of 87
  shot at point blank broke 1 s after rooting standing, 2 s flinching for
  half the crater; flinching for the whole crater blew it into specks that
  never regrouped and it lived 4 s.
- Smoke games to 3:00, run 1, greedy, planets 2 and 3, off against on: squad
  health 88 against 85 on planet 2 (its first wanderer broke at 2:40 against
  2:25), 89 against 77 on planet 3, where the wanderer never came within 118
  units of the squad (the games diverge on the GPU, so that gap is not the
  flinch).

**Stills.** `zoom12.jpg`: the same game before and after, 0:47.25 and
0:47.50, as the squad opens up on a rooted wanderer: before it is shot into
blotches; after it bends into a crescent round the hole the hits bit out.
`zoom6.jpg`: the same seed at zoom 6, 0:44.0 and 0:44.5 (the two games had
already drifted apart).

**Tried for the body and dropped** (same afternoon): a third of the
wanderer's members on a leash twice as long (a fringe), a rooted wanderer on
its walking leash and a fifth of its pull, and a gathering one on its
walking leash. None changed the picture: in the thick of the slime and
rooted under its crystal the wanderer still read as a pale slab, so its
flat look there is how the view draws a packed body, not how the members
lie.
