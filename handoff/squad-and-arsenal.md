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

**2026-11-18, batch 2 (Heave and Corral merged; the next PR after #448).**

- Done: Heave and Corral are one card, `Card::Heave`; `corral` still loads
  as `heave`. Numbers and the reverted stop-short trial:
  `Reports/gun-shove-2026-11-16/README.md`, *2026-11-18*. The Mind quiz is
  out of the records.
- Squad control (job 2): the play notes of 11-14 to 11-17 hold no
  squad-control complaint still open after the 11-16 movement rework. Its
  leftovers (big squads in caves, places fitted to the ground) are not
  Milestone 1. One thing seen and left alone: 40 soldiers draining one
  crystal stand as an overlapping pile, because only soldiers within the
  rim plus 2 dish cells harvest (`turn.rs`, Engine and tools'). Tidy rings
  would need a longer harvest reach, which changes the rush economy (D7),
  so that is Direction's call, not a control fix.
- New job from Scott's card of 11-18 03:03Z ("Lure now, in the three
  planets"): the Lure building back in the run on Enemy's steering field.
  Shop item "Lures", an unlock, first price 150, no prerequisite; the
  building costs 300 food, has 150 health and is mortal; siren and outpost
  stay out. Interface agreed with Enemy 11-18 03:13Z: the field comes from
  every `Kind::Lure` that is `up()` and on, plus `Game::lure_marks`
  (Enemy's, in `raids.rs`); reach `Rules::lure_reach` 60, engine key
  `lure_pull` 10; GPU only; `lure=T,X,Y,SECS` films a mark. Enemy's branch
  is `claude/project-thread-s5k9cw`. The Lure is not sold before the field
  is on main, so this lane's PR queues after Enemy's.
- From Enemy (11-17 22:28Z, #451): planet 3's Trunk sets into amber walls
  where it is shot and fire burns them; the shop before planet 3 does not
  push a Torch (`run::bring` names a card for the Sprawler only, and the
  Torch is a rolled shell). Job 4 should make planet 3 ask for one.
- From Look (11-18 03:00Z): once the rose flash fades a stilled or
  provoked part has no look of its own; Look took that job.
- Rules that started 11-17 (#449): a PR that changes the game adds a line
  to `news/whats-new.txt`; a wiki page's Now may not grow (the card
  families in `wiki/world/guns.md` now sit under their own heading); an
  off-by-default key needs a row in `src/build/switches.rs`. Run
  `cargo test --release --locked --bin larder-world` (5 minutes) as well
  as the library tests before pushing: #448 bounced once on a window test.
- Next, in order: the Lure (above); shake and hit-stop as one Experiments
  toggle and the rifle's feel (job 3; the switch row exists); shop offers
  for planets 1 to 3 (job 4).

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
