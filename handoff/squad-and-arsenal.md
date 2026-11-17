# Lane brief: Squad and arsenal

Model and effort: Fable, high. Read `00-all-lanes.md` in this folder first.

## Your job

Everything the player's side does: how the squad moves and obeys, and how
its guns, cards, turrets and buildings work. You own
`larder_world/{squads,steer,paths,funnel,cover,cmd,shots,buildings,doors,raising}.rs`,
the window's `input.rs` and `squadmode.rs`,
`src/mould/{rounds,guns,bullets,kit}.rs`, and the gun sound. For Milestone 1
your target is exit test 5: the starting rifle feels good, and one shop
visit changes how the gun plays.

Start from `handoff/gun-cards.md` and `handoff/turrets.md` as history.

## First jobs, in order

1. **Finish Provoke and Still.** The work is on the unmerged branch
   `claude/project-thread-vx9t3p-wip` (commit 9294ed83); the steps to
   resume are in `handoff/gun-cards.md`, "Open branch". Rebase, film,
   merge. Then **stop adding card families**: gun steps E and F and the
   next turret body are parked until Milestone 1 passes.
2. **Control that gets in the way.** Read Scott's play notes of 11-14 to
   11-17 in `wiki/decisions-game.md` for anything about controlling the
   squad that is still open, and fix those first. He plays squad mode with
   the mouse as the aim.
3. **The rifle's feel, with Look and interface.** You own what the gun
   does (rate, spread, recoil timing, hit-stop and camera shake as
   rules, the sound); they own how it is drawn. Camera shake and hit-stop
   (D6) are built and off: make them one Experiments toggle, not two
   keys. Scott, 11-15: the guns "should really be one of the draws of the
   game. They need to be really satisfying."
4. **Shop offers that give planets 1 to 3 distinct builds**: what is
   offered after planets 1 and 2 should make the second and third planet
   play differently from the first.
5. **Small pending picks that are now yours**: merge Heave and Corral into
   one card (the plan's recommendation); drop the Mind bullets quiz.

## Rules particular to this lane

- Guns never recharge and shots stop at the first slime (Scott's
  rulings). Bullets do not pass door grates (11-17).
- The run's state, the gun cards (`Rounds`) and the tools (`Kit`) live in
  the window program today, outside the game's rules. Engine and tools is
  moving them into the library. Expect that change in your files and
  agree its timing with that lane through the coordinator.
