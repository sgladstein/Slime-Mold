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

**2026-11-24, fused shots (pathfinding thread).** #810 (bot dials) and
#813 (the card-look options page) merged. The owner picked fused shots
(B, 15:51Z): `rounds::shots` groups the gun's cards by shot, `Shop::shots`
and `plain_shot` carry it, `start/shop.rs` frames each shot (Look's file;
the HUD thread knew first). Next: Engine's builder curve arm on the 1070
(batch x, about 2 h from 15:23Z), then fix what it shows broken in the kit.

**2026-11-24, dead cards and the bot's kit (pathfinding thread).** #802
merged (the hunter). Direction 13:42Z: no more turret bodies for this
milestone. Next PR: the cards dead alone (carriers held to two bullets
owned, device and sprayer trail cards and lone plain pops, gun-only cards
off devices; `wiki/world/guns.md`). Then a small PR for Engine: strategy
bot dials `mortars`, `sprayers`, `hunters` (placed round guarded pumps,
unlocks bought first) and `body=grenade|mine`, and a `bodies=` league
field, so the curve can use the new kit (CI's plan bot|rush buys none).
#806 merged (the dead cards). The dials PR is open on this lane's branch;
Direction asks Engine to add a "builder" arm to the curve with
`pumps=1,guard=1,hunters=1,body=grenade,blast=40`. Next: whatever that
arm shows broken or useless in the kit.

**2026-11-24, the hunter (pathfinding thread).** #796 merged (the
sprayer) and #799 (its Torch holds off its squad, droplet jets) merged. Next
PR: the third turret body, the hunter (`Kind::Hunter`,
`Game::hunter_mark`, `HunterMark`, `Game::hunter_marks`/`hunter_killed`,
`BuildRules::hunter_*`, view kind 9 `hunter_head`, build page (0,5) key
X, shop item "hunter" after "sprayer"). Its shells are mortar shells
(`fire_shells`, shared with the mortar) with a near-instant flight and no
arc. The same PR gives sprayers and hunters the Turret power upgrade
(the sprayer was left out in #796). Next: whatever Direction picks.

**2026-11-24, the sprayer (pathfinding thread).** #790 merged (Fever,
Backdraft). Next PR: the second turret body, the sprayer (`Kind::Sprayer`,
`Game::sprays`, `Rounds::spray`, `BuildRules::sprayer_*`, view kind 8,
shop item "sprayer" after "mortar"). Next candidate body: the hunter
(picky targeting, slime on a crystal first); Direction wants it after.

**2026-11-24, relics (pathfinding thread).** #781 and #786 merged (gun step
F: shop levers, Lasts, Carrion). Next PR: Fever (`sick_trail` 2 through
`Run::engine_keys`, Direction's OK 11:28Z; Enemy told by Direction) and
Backdraft (`RoundRules::backdraft_*`, `Game::trodden`); the shop now offers
one unowned relic drawn by the seed. Death gas (marked walkers puff gas
when they die) is parked with Enemy until after their planet 7, curve and
Dozer work. Then: two or three candidate turret bodies to Direction before
building (each must deliver a shot unlike the mortar and the turret).

**2026-11-24, gun steps E4 and F, the dart (pathfinding thread).** #778
merged (E4 autocast, `autocast.rs`; the Dart, `Card::Dart`). #781 queued: F
part 1, the shop's luck levers (`Run::luck`, `family`, `Pick::Hold`,
`Pick::Reroll`, `since_rare`; Trigger and Expire tier 3). F part 2 next PR:
`Ammo::lasts` (Blast, Bomb, Flare, Dart, Mind; not Gas or Mist, Direction
09:51Z) and the first relic, Carrion (`run::Relic`, `RoundRules::carrion_*`,
shops after planets 3, 5, 7). Next: more relics (the two slime-rule ones go
to Enemy through Direction), then whatever Direction sends.

**2026-11-24, gun steps E2 and E3 (pathfinding thread).** #764 (trodden
ground, unlit stars) merged. Branch `claude/project-thread-4xnivn`, one PR:
`trample_r` 2.5; E2, devices carry a loadout's cards (`Run::device_loadout`,
`Kit::carry`, `KitRules::cord_cards`); E3, the four bodies (`bodies.rs`:
Grenade, Mine, Line, Canister; rolled reach, cooldown share, 1-3 slots,
budget 2-6; `Run::bodies`, `body_offers`, `Pick::Body`, `Pick::CarryBody`,
`body_cards`, a deck a body at `DEVICE_DECKS` + tool; old unlocks load as
plain bodies; the Canister puffs every second for 4 s). A device's fire
brings its own brush (`RoundRules::device_brush` 1), so a Torch Grenade at a
held squad's feet costs it 78 health of 800 (7 before, trodden), none from
12 cells. Bodies sit in the shop's Tools tab, not on the gun rack (the rack
draws gun stats); two guns still on offer. E4 autocast follows in its own
PR (`autocast.rs`, `Cmd::Auto`, `Body::auto`, `Pick::BodyAuto`; rule:
thickest slime in reach, 12 cells clear of the squad). Next: step F (luck;
shop parts to Direction first), then the dart.

**2026-11-24, planet 3 fire (Direction's jobs, pathfinding thread).**
#750 merged (starfall gold landing, first star beside the squad). Branch
`claude/project-thread-4xnivn`, PR "IGNITE pass in engine.wgsl": trodden
ground (`trample.rs`, `Rules::trample_to` 0.1, `trample_r` 1.5,
`trample_speed` 2; GPU only, IGNITE mode 2) and `Burst::unlit` for star
craters. Bot games: unlit stars took no-card planet 3 map 3 from 37 burned
to 0; the firebreak barely moves the rush bot's Tinder burns (127 to 124
over maps 3-6), because its squads walk while they fight (net 1-9 cells a
second); a held squad in the GPU test loses 3 health against 20. The run
bot's plans never buy Fireproof (not in `Plan::shop`'s default list).
Next: gun steps E (device bodies) and F (luck), then the dart.

**2026-11-23, formations (own thread, "Rework squad formations with spread
control").** Scott's review cut the six formations to two shapes, blob and
ring, plus a spread of five steps (`SPREADS`, `Squad::spread`,
`Cmd::Formation(q, shape, step)`; old replay lines still parse). Line, wedge
and column are gone, and with them the facing turns and the column's
always-funnel. `[`/`]` step the spread; the Formation page shows shapes on
top, steps below, each tile drawn at the squad's spread. A standing squad
forms up on a pick (members on Hold get a Move round the middle); `stray`
and the selection ring's link grow with the spread. The bot never sets a
formation. Offsets are still worked at a gap of 1 then scaled: changing that
rounding sent planet 8's squad of 40 on another walk
(`a_squad_crosses_planet_8_and_settles`).

**2026-11-22, exit test 5: the rifle's feel (Direction's job, same
thread).** #701 and #705 merged. Filmed the plain rifle on planets 1 to 3
at zoom 6 and 12: rate, tracer and hits read well; the fault was free
fire, Guard and Clear aiming at the thickest block in the whole 24-cell
reach, about 550 px at zoom 12 against about 300 px of screen. Direction
picked option A (06:15Z): those read `Rules::near_first` (12 cells) first
and the whole reach only for `cmd::FAR_FRAMES` (4) frames after a near read
finds nothing (`Game::sight_read`, `Game::read_aim`, `Soldier::far`; the
read-back moved from `turn.rs` into `read_aim`). Option B (shorter reach)
is out; C (an edge tick in `draw.rs`, Look's) only if far shots still read
as fire into nothing after A. The run bot and gun lab fire Free, so their
baselines shift; tell Engine through Direction when it merges. #708 merged
07:21Z (three seed-tuned tests keep `near_first` 0). Then, Direction's
call: a mini-boss beyond `near_first` takes the shot only when the
soldier's own aim holds no slime that near (`Game::target`,
`bosses.rs boss_aim_within`), so Enemy's origin (#699) inherits it once
it goes through `boss_aim`.

**2026-11-22, exit test 5: every shop gun card visible (Direction's job,
in the pathfinding thread).** The gun lab's arena showed Wide, Scatter,
Delay, Thick, Quiet and Thrift playing the same game as no card (a plain
cast made no burst) and a lone Tinder doing nothing. Fixed in one PR: a
modifier on the plain bullet pops (`rounds::plain_bursts`, sized by the
shot's strike, `RoundRules::plain_pop`/`plain_wide`; the strike rides on
`Rounds::strike` and `Pending::strike`); `tinder_spark` 0.01 lights a
Tinder's own brush; the shop holds Thrift until a bullet card is owned and
Quiet and the Flare until `run::ALARM_FROM`; plain pops don't shake
(`main.rs feel`); `set=strike=` is a rule key for the gun lab. Heavier
shots checked, works. Second PR, Direction's calls: Seek's round flies on
past its aim (`Rules::seek_reach` 0.5, `shots.rs fly`), and Smother's trail
discs also calm (`MIND_STILL`; trails now take `RoundRules::burst_rules`,
since the toolkit's carry no Mind length). Third PR: a calm disc the shot's strike
across where Smother's line hits (`smother_calm`), so the rose shows. Note for the shop DPS readout (#697, `dps.rs`): it
reads `rounds::bursts`, and the plain pop is `plain_bursts`
([report](../Reports/gun-cards-visible-2026-11-22/README.md)).

**2026-11-21, controls and HUD review (its own thread, the owner's ask
06:16Z).** Review doc: https://claude.ai/artifact/12jZrjC3MHe7R54S7V7Ktq
(ranked problems, all done but "keys falling through card pages"). #655:
a WASD tap no longer drops queued orders, buy/merge keep the stance, the
run's top bar no longer reads "0%". #660: **drag pick** (owner 06:53Z) is
a temporary band folded into `Game::linked` (`Squad::band`, `Cmd::Band`,
`BAND_SPREAD` caps the fan-out offsets), so every link fan-out (orders,
fire, Hold fire, formation, heal, rings) reaches a box's squads; it ends on
taking another squad, Esc or a split. **One device slot per squad, on a
cooldown** (owner's card answer 06:55Z, option A): `Cmd::Device`,
`Game::device`, `Squad::device_at`/`device_secs`, `KitRules::cooldown`
(blast 20 s, mine 25, cord 30, gas 35; Tools+ cuts them 20 % a level); free
per use, the carry rule stays for the RTS round; the strategy bot throws
through the slot (`strat::tools`); the Tools button stays in a run with
nothing owned and opens the locks (it hid until 11-22; the owner read it as lost). The follow-up: Esc closes a card page first, Space jumps
to the newest alert, only a Shift drag splits, a number takes command with
none commanded (it did nothing), and Ctrl swaps the crosshair for the pick
cursor (`cursor.rs`, Look's #658; its box and device looks needed no other
wiring). Next: step E's rest (device bodies, cards, autocast) in
`wiki/arsenal/guns.md`.

**2026-11-21, squad controls (its own thread, the owner's ask 05:04Z).**
Lookouts removed (`cover.rs` keeps only the work's end and heal's end;
`Cover`, `CoverRules`, `Soldier::cover`, `work_boost`, `Cmd::Cover` and the
`guardwork` film hand are gone; workers fire at full rate). Hold fire is its
own toggle in the squad mode (`App::squad_hold_fire`, `App::unheld` keeps
each held soldier's mode); the fire button picks defend or free. The card's
Formation button (T, `FORM_PAGE`) opens a page of six formations, two new
(`Formation::Column`, `Formation::Spread`), and a 6 x 2 card (`ui::CARD_COLS`, buttons 104 wide, the
gun strip's floor at 0.7) with Heal and Wall on the main page. A column faces where it walks and counts as a pair wide to
the funnel. The bot check (before/after, planets 1-3) is in the PR.

**2026-11-20, pathfinding review (its own thread, the owner's ask).**
Five fixes to how squads and soldiers walk: `Game::steer_point` (a steered
squad's point ahead no longer falls in rock), `guide`'s `whole` and
`slide`'s 45-degree turn (slanting walls), `paths::onto_map` (an anchor or
member just off its map), steered members walk to the anchor
(`squad_way`), and `want_round` counts no soldier nearer than its straight
distance. Steered squads reached 4.8 crystals a map against 1.9 over 24
maps. Tools for the next pass: `examples/pathprobe`, `LARDER_PATHLOG=FILE`
on headless runs, `hand=steer@X,Y[,N]`, and
`Reports/pathfinding-2026-11-20/tools/` (faults, sign test, track plots).
Routing round slime was tried (11-21) and not kept (dead-ends); bots merge
squads at any distance ([report](../Reports/pathfinding-2026-11-20/README.md)).
11-21: a squad on a plain move walks round fire (`fireway.rs`,
`Rules::fireway`, on; Direction's call): fire marks from burn and light
bursts and from soldiers that feel heat, 15 s each, and a coarse plan
round them. Fire on the way 413 -> 274 on the 24 maps, trips no slower.
Test with `hand=movetour,N,clear,fire` or `examples/pathprobe fire=6`.
11-21: walking maps build in about 60% of the time (`MapBuild::advance`'s
inner cells skip bounds checks, same distances), and stale maps rebuild at
a quarter pace when no goal waits: planet 3's `pump_maps` spikes of 5 to
7 ms are now 4.2 at most (a new order's map), stale steps under 1 ms.
11-21: a squad answers a new order in 0.07 s median (sent) and 0.2 s
(steered about-turn) on the 24 maps; nothing to fix. A squad of 60 sent
out of a walled base by a 4-cell door stood inside for good: the
column's places fell past the base's walls (`column_places` now keeps
each on its trail point's side, `a_squad_leaves_a_walled_base_by_its_
door`; `fort_numbers` is the harness). Turrets and pumps don't block
soldiers at all (only walls, doors, crystals and the bulwark do).
11-21 (Look's exit test 2 pass): soldiers at a crystal no longer stack.
`Game::rim_spots` (paths.rs, once a step from `squad_turn`) gives each
soldier whose goal is in a crystal its own spot round the rim, `SPOT` 3.5
dish cells apart, ring 0 filled all the way round first (closing down to
elbow room) since only ring 0 is in harvest reach. Nearest neighbour 2.1
-> 3.5 cells median on planet 1; harvest at 60 s 4,081 -> 4,199.
Follow-up: only move, guard and drain orders take spots (the run bot's
rush, an attack-move onto the slime's home crystal, spread round the rim
and planets ran 16% longer on CI's run bot); rings inside reach first.

**2026-11-20, batch 7 (the balance review's proposals 4 and 5, through
the coordinator).** `rounds::casts` gives leftover modifiers (and Delay or
Thick) a plain cast, so they work on the plain bullet. Restock holds back
carriers until a bullet card is owned. The last card slot draws from owned
cards too, so a second loadout can get its own copy.

**2026-11-20, batch 6b (a review of batch 6).** Nine fixes: the cycle key
skips linked squads; L on a turret steps back to the default; a gunless
loadout arms squads with the rifle plus upgrades (`Run::squad_gun`); a
turret deck is cut to its rack (`Run::turret_deck_of`); closing the panel
keeps the pause and only re-arms on a changed choice (`Rounds::set_round`
keeps live state); attack-move's hold counts only fighters; build reach is
per turret. Window test for the panel added.

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
