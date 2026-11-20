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

**2026-11-20, batch 6 (the owner's play notes of 11-19 16:55Z; thread
"Improve squad control and gun loadouts").**

- Merged #524: attack-move stands only for a fight (`Game::fight_turn`,
  `Rules::fight_hold` 1 a member, at least 3, hold till half); ` cycles
  squads (Shift back); squad tiles shrink instead of "+N".
- Open branch `claude/project-thread-gdv9rz`: loadouts. The owner's rulings
  (19:22Z): a loadout is a gun plus cards; each squad and turret uses one;
  a card or gun in one loadout at a time; H opens the panel paused on a
  planet, no reload; a default for turrets, each changeable. Built:
  `run::Loadout`, `Run::{loadouts, editing, turret_loadout, field,
  field_squad}`, picks `Edit`, `NewLoadout`, `TurretsUse`, `SquadUses`;
  `Squad::loadout`; `Building::{deck, loadout}` with `Cmd::TurretLoad`
  (turret shots read `Game::mods_by(by, turret)`); the window's
  `open_loadouts`, `close_loadouts`, `arm_squads`, the turret card's
  Loadout button (`TURRET_LOADOUT`, L).
- Next: film the panel at 1080p and 150% UI (the bottom bar holds up to
  8 loadout buttons; check it fits); the run bot does not use loadouts yet.
- Unspent shop points carry to the next shop (`Run::won` only adds);
  planet food does not.

**2026-11-19, batch 5 (sleep wears off; Gas and Blast bite the squad).**

- Merged: #505 (a walker sleeps once, then shrugs off gas and Mind statuses
  for 20 s) and #513 (Gas's cloud dozes soldiers standing in it,
  `Rules::doze`; a Blast takes 40 off every soldier in its disc,
  `RoundRules::blast_hurt`; fire on its own holds a payload that would reach
  a soldier, `blast_margin` 1 / `gas_margin` 8, checked in `Rounds::fire` at
  the real hit; Mist lays no cloud by a soldier). On by default, Direction's
  call. Reports: `Reports/sleep-wears-off-2026-11-18/`,
  `Reports/own-goal-2026-11-18/` (planet 3 numbers).
- Open branch: `claude/project-thread-vzzzbe` (this handoff only).
- Next: a dozing soldier has no look of its own (Look; `Soldier::doze` is
  there to draw from). Planet 3's tuning waits on the owner (he said review
  first; the review found Gas, not the Trunk's hardening). The play log
  should carry the army round (Engine and tools).

**2026-11-18, batch 4 (auto-fire against loose walkers; Scott 17:01Z).**

- The commanded squad now fires by its own fire mode when not volleying
  (`Game::stance_of`): it was held to defend, so his "free" did nothing and
  its attack-move stood to fight slime it would not shoot. Defend reads only
  `Rules::defend` 7.5 cells round each soldier (`Game::sight`, the
  `UnitIn::sight` turn.rs passes the engine), unless guarding, clearing a
  crystal or on the core: with the whole 24-cell reach read, a thicker clump
  far off hid the walkers on the squad. His planet-1 log: 23.9 shots and 143
  kills a second with fire held, 4.5 and 11 on auto. Scene (planet 1 seed
  1795015831, eight held at the start, 150 s): old auto 804 shots, 3,058
  kills, health 60%; new defend 1,208, 3,166, 83%; new free 1,613, 3,204,
  75%; volley at the densest slime 1,632-1,700, 3,032-3,117, 41-49%.
- Merged as #495. Spreading the volley over the slime round the cursor was
  then tried at his request and not kept (wins +10-16% at a body's edge,
  loses at a stream past rock and among loose walkers; `Reports/dead-ends.md`).
- Look's A3 standing half (hold elbow room when a squad stands or drains):
  looked, not built. A squad at rest already keeps it (eight held on planet
  1, nearest neighbour 5.13 fine cells at least, elbow room 4.8). The heap is
  only drainers, each pulled to the rim point nearest it (`Game::rim`):
  greedy hand, planet 1 run 1, median 4.2-4.3 with up to 20 draining, 2.9-3.2
  with 35-42 on one crystal, where the ring has no room for them all at
  elbow room. Spacing them would stand some out of reach and cut the drain;
  the helmets drawn above the bodies (#494) carry the count. Revisit only if
  he says a draining squad still can't be counted.

**2026-11-18, batch 2 (one PR after #448: Heave and Corral merged, shop offers, Free recruits off the list).**

- Shop offers (job 4): `run::bring_shell` (the Sprawler and the Trunk ask
  for a Torch, the shop's first bullet while none is owned) and
  `Run::restock`'s rules (no shell twice, no plain standard bullet, no
  reader with nothing to read, two gun kinds and neither the one carried).
  From the first two shops of run seeds 1-12 before the change: Tinder was
  pushed with no fire on sale on 9 of 12, one shell twice on 4 of 24, one
  gun kind twice on 7, the carried rifle on 7. For Direction (prices are
  its numbers): before planet 2 the run has 200 to 300 points, Tinder is
  150 and the Torch rolled 40 to 150, so on about 1 seed in 4 a two-star
  win cannot buy both.
- Shake and hit-stop (job 3): the one Experiments toggle is the switch
  list's row `feel` (`try=feel`). Nothing more was built: only blasts
  trigger it, so the starting rifle shows nothing until a blast bullet is
  bought; a kick on every plain volley was ruled out when it was built
  (`feel` in main.rs: "in a fight every volley would").
- Free recruits (`Rules::reserve`) left the Experiments list on Direction's
  ask: a campaign-only rule (`NOT_SWITCHES`), `SETTLING` is false, three
  rows stand.


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
- The Lure is back in the run (Scott's card of 11-18 03:03Z, "Lure now, in
  the three planets"): the Lure and the lures' on/off switch are off
  `run_hidden` and never locked (`run_lock`), free from planet 1, 300 food
  and 150 health as before, mortal. #461 first sold it as a shop unlock
  (Lures, 150); Direction dropped the row the same hour (08:38Z) so planet
  1 has it. Direction's prices with it: Tinder 100 (`TINDER_PRICE`), a
  Torch at most 100 (`TORCH_MOST`). It stands on Enemy's field (#457: every
  `Kind::Lure` that is `up()` and on, plus `Game::lure_marks`; reach
  `Rules::lure_reach` 60, `run::LURE_PULL` 10 on every run planet; GPU
  only; `lure=X,Y[,T]` films a building, `lure_mark=T,X,Y,SECS` a mark).
  In Enemy's scene on this branch the Lure stood from 0:30 to under 1:00
  and 4,461 walkers were killed by 1:30 (1,813 with no lure;
  `Reports/lure-field-2026-11-18/`). The siren and the outpost stay out.
  Known: the Drifter's `lure_r` 15 (the pump smell's reach) also widens
  the Lure's own scent on planet 1. Harvest reach (the pile of 40 on one
  crystal) is on Direction's held list under D7.
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
- Next: what Scott's next play notes say about control and the gun; the
  dart (the Lure's second form) waits until he has played the Lure.

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
