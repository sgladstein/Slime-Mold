# The Dozer's nap (2026-11-24)

Look lane, Milestone 2's Look B (the slime itself), for Direction's job
"planets 4 to 8 each read differently at zoom 6". Page: https://claude.ai/artifact/FkUnHaEXoaA2e3EFcrJoxT (source
`index.html` here, clips beside it).

**Before.** Planet 8's own slime, the Dozer (Enemy, 11-24: the Swarmer's
lunge and last stand on a body that sleeps away from its crystals), was
given the Sleeper's `SlimeLook::Dormant`: a grey matte crust asleep, gold
awake. Filmed on run 1 planet 8 at zoom 6, it read as planet 6, and its
grey crust over the core's dark ground read as gravel.

**After** (`SlimeLook::Doze`, look 10, `src/world/view.rs`; `view.wgsl`'s
`crust_look()`, `NAP_RGB` (255, 240, 200), `NAP_DIM` 0.95, `NAP_MATTE`
0.2): asleep it is a pale straw crust that keeps most of its gloss, and
awake it is the Swarmer's lemon (`species_hue`) with its swell. The cost is
nil: the same crust pass, with a colour chosen by look.

**Rejected.** A tan crust (255, 199, 128 x 0.75, matte 0.6): at rest the
whole view is crust, and the tan read as bare dirt. Two straws in between
(255, 232, 170 x 0.85 and 255, 214, 150 x 0.8, both matte 0.3) were closer
to the ground at rest.

**Open.** At rest, any crust covering the whole view reads more as ground
than slime; the Sleeper has the same problem. The lemon waking through it
is what reads.

Clips (before left, after right, 720 x 400 crops of 1280 x 800, 4 fps):
`z6.mp4` (idle, heart at 434,120, from 1:00), `z12.mp4` (the same at zoom
12), `fight.mp4` (greedy hand, following the squad, from 0:38).
