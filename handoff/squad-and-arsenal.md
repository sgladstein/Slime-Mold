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

## State (kept current; newest first)

**2026-11-17, batch 1.**

- Done: Provoke and Still are finished and on
  (`Reports/gun-provoke-still-2026-11-17/`, clips at zoom 6 and 12). No
  card family follows until Milestone 1 passes.
- Open branch: `claude/project-thread-e2ip9o` (this lane's).
- Next, in the brief's order: control that gets in the way (job 2); the
  rifle's feel and shake and hit-stop as one Experiments toggle (job 3);
  shop offers for planets 1 to 3 (job 4); Heave and Corral as one card
  and the Mind quiz dropped from the records (job 5).
- Notes for whoever films next: `hand=squadwork` without `mode=squads`
  keeps the eight together as one group (the RTS round's control); with
  `mode=squads` it splits round a crystal. The commanded squad only
  defends itself on its own (`Game::target`), so a scripted hand that
  never volleys lets slime grow on a crystal beside it; that is the
  player's trigger, not a bug.
