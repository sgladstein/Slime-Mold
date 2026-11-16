# Review brief: the squad movement rework (2026-11-16)

Given to one independent reviewer (a fresh agent with no other context),
under the owner's 05:18Z OK ("Feel free to use 1-3 independent sub agents
to review your work as you proceed").

---

You are reviewing a rework of how squads of soldiers move in a real-time
strategy game written in Rust. The owner's words: squad movement "is still
clunky overall", start over rather than patch. The goal is movement that
feels smooth and reads well on screen for squads of 12 to 200 soldiers,
on generated planets full of rock, narrow gaps and crystals soldiers can't
stand on, with thick slime slowing soldiers. The owner's standing rules:
fun over fidelity; everything on screen is the simulation; build first,
balance later.

The repository is checked out at `WORKTREE` (read-only for you; do not
edit, build or run anything there). The change is the diff in
`WORKTREE/review.diff` against `origin/main`. Read `CLAUDE.md`, then the
new module `src/mould/larder_world/steer.rs` (its header explains the
design and what was wrong), then `squads.rs` (`squad_turn`, `guide`,
`stop_point`, `rerank`), `turn.rs` (where the walk runs each step),
`paths.rs` (`walk`, `elbow_room`, `walk_line`, `solid_at`), and the
window side in `src/bin/larder-world/main.rs` (`prepare`, `Look`,
`TURN_RATE`) and `squadmode.rs` (camera follow). The measuring harness
is `src/mould/larder_world/move_tests.rs`; its first numbers are in
`WORKTREE/numbers.txt`.

Lead with the big picture, in this order:

1. **Is the approach right for the goal?** It keeps the old structure (a
   squad anchor moving along a walking map, formation places round it)
   and changes each soldier to a velocity steered toward its place, in
   step with the anchor, with separation and queuing, one walking map per
   squad, and interpolated drawing. Would a different structure (pure flow
   field plus flocking with no anchor, formation slots fitted to the
   ground, continuum crowds, ORCA-style avoidance, Age of Empires-style
   formation objects that deform at choke points) serve the owner better?
   Say which and why, concretely.
2. **What do the measurements miss?** The metrics are in
   `move_tests::feel` (still, jerk, turn, pace, spread, back, settle, map
   waits, movement ms). Are they the right ones for "clunky"? Which
   scenarios are missing (squads crossing each other, steering into rock,
   reversing, fighting while moving, wading in slime, several squads at
   one crystal, linked squads)? Any confound in how they are measured?
3. **What would a player still see as clunky** after this change? Think
   about the 15-steps-a-second game step, the camera, figure turning,
   arrival, crowding at crystals, and the formation shapes (blob, line,
   wedge, ring).
4. **Ranked next directions**, each with a first cheap experiment.

Then correctness: bugs, deadlocks or soft-locks (a squad that never
arrives, members that never settle, an anchor that waits for good), replay
determinism, anything that changed behaviour outside the squad mode (the
RTS round and campaign must play as before), and the cost per step at 200
soldiers. Cite `file:line` for each finding. Keep the report under 900
words.
