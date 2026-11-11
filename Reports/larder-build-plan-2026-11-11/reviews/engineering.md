# Engineering review (independent agent, 2026-11-11)

Prompt: see prompts.md (Reviewer 1). Report below, lightly condensed.

1. **The M2 counter-cycle gate can never pass in a solo game.** One number per plan per seed against a fixed mould is always a ladder on that seed. Change: "at least 2 plans each best on at least 4 of 16 seeds (or across 3 layouts), and a condition you can see on the dish predicts which wins"; keep "no plan best on 13 or more".

2. **"Slowed to 15 steps a game second" only slows the harness clock.** larderprobe redefines the game second (`SLOW_STEPS = SPS/8`, larderprobe.rs:498) and rescales only `wound` (:557). Every other timer still counts in `STEPS_PER_SECOND` = 120 (mod.rs:80): wake_grace, settle_seconds, FED_LOOK, crust_life, wall_half_life, oats expiry, pending drop times (:3426), the snip-mark flash (render.rs:404). The window steps `STEPS_PER_FRAME` = 2 per frame (mould.rs:735); the slowed game needs 0.25 a frame. Change: one time-scale conversion in M1 with a unit test that wounds, drops and the starvation window run in game seconds; a fractional-step accumulator in the window.

3. **Round-end rules and dead soldiers becoming piles are placed too late.** M1's gate asks that a body is eaten but death-to-pile is M2; the deciding measure needs wipe and cap rules from M1; "share of the map held" is undefined. Change: move death-to-pile, wipe and cap rules and a precise share measure into M1.

4. **M2 tunes balance on a squad M4 changes.** Change: merge soldier buying into M2, keep only upgrades in M4, re-run the M2 gate after M4.

5. **Firing has no public API and its cost grows with every walker.** `stroke_dab` is private (mod.rs:1634); the public stroke checks the phase and spends `snips_left` (:1586), forced to u32::MAX by the harness. Every dab runs `kill_in`, a loop over all walkers (:1689), and `on_pile` over all piles. 40 soldiers at the harness rate make ~256 dabs a step, ~0.8M distance checks, about a whole step's cost. Change: `Dish::fire(segment, width)` that bins walkers once per step, no snip budget or phase check; quote its cost at 4 and 40 soldiers.

6. **`snip_marks` is never pruned and the window draws it every frame** (mod.rs:1674; render.rs:404, look.rs:3159). A 15-minute round builds ~3.5M entries. The flash is also painted, against the 2026-11-09/10 ruling. Change: prune old marks; drop the flash for Larder or get an owner ruling.

7. **Pile indices shift when piles land or expire** (`piles.insert`, mod.rs:3430; `expire_oats`, :3402). Change: stable pile ids from M1.

8. **How soldiers touch the mould is unspecified.** Beacons only boost a fixed pile; a soldier as a moving pile would block its own fire near it (`!self.on_pile`, :1655). Change: soldiers as a scent deposit written each step, health drained by a walker count in radius from the walker grid; range scans once a game second.

9. **Drop smell before landing and "starving spreads faster" are new dish code**, not tuning (pending drops have no scent, mod.rs:3426; `Dish::ate` only with a rival). Change: new settings default off, in `state_hash`, listed as M2 dish tasks with frame cost.

10. **M3 should not be an entry inside `src/bin/mould.rs`**: left button only (:886), no Shift/Ctrl, no box drag; Space is fast-forward; G, digits and F2-F7 taken. Change: a new `src/bin/larder.rs` reusing `render` and `look`.

11. **Replay and frame gate too weak.** Change: record orders as (step, order), replay to identical dish and game hashes; gate the soldier layer at < 0.1 ms a step in the harness; soldiers get their own RNG.

12. **M6 underspecified**: different map sizes can't be compared; reading 100k walkers back from the GPU defeats it; the harness already breaks the "Dish methods only" boundary. Change: a small trait in M1 (walkers in a disc, densest cell, pile eaters, fire, deposit scent); twin test at the same map size.

Cost: a 15-minute round is 13,500 steps, ~14 s a run; M2's 6 x 6 x 16 is ~35 min on 4 cores.

**Top three:** (1) replace the cycle gate. (2) M1 owns game time, round-end rules, death-to-pile, stable pile ids and a binned `fire` API with its cost. (3) Soldier buying in M2, re-run M2's gate after M4.
