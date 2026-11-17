# Turrets, grown up: tech tree or cards (2026-11-16)

The owner (2026-11-16 21:07Z): "Start a thread to explore improving and
expanding on the turrets. Two ideas: 1) We build a tech tree for turrents
that can give you new types of turrets and abilities. 2) We integrate the
turrets into the card system and you can modify turrets with your cards."

Page: https://claude.ai/artifact/V16Dp6AyxrZ6KQ4KMPzbwr (copy: `page.html`
and `page/`).

## Where turrets stood

One kind (`Kind::Turret`): a plain strike at the densest slime in sight every
0.5 s (`BuildRules::turret_every`), 15 cells, 300 health, 500 food; one
in-mission upgrade, Render (600 food, kills in range pay 0.25 food); in the
run shop an unlock and three "Turret power" levels (`run::ITEMS`). Turret
shots carried no cards (`rounds.rs` said "turret rounds later"; the gun
review's *What else we missed* listed a turret deck). Cards reached turrets
only from outside: `Corral` shoves slime toward the nearest turret. Since
PR #398 turrets fire over walls.

## The two ideas, and the pick

- **Tech tree.** New bodies do what a squad can't (lob over walls, hold a
  field, herd); bases read at a glance; branches give a run its identity in
  the shop. Against: authored combos only, and a second weapons system beside
  the cards (the reviewers' warning, 11-14).
- **Cards.** Reuses every card built (shells, trigger, expire, multicast,
  shoves); combos in a fixed spot; giving cards to turrets instead of squads
  is a real trade, which keeps turtling honest. Against: alone, a carded
  turret is a squad gun that can't walk, and per-turret decks mean clicking.
- **Recommended and picked: both.** The tree unlocks turret *bodies*, each a
  different way to deliver a shot (gun turret, mortar over walls, sprayer
  laying a field, fan herding); cards fill each body's barrel. Owner's card,
  21:36Z: "Both: tree bodies, card barrels". The tree lives in the run shop
  only, not as research during a mission (owner 21:41Z: "I lean shop").
- **Still open** (defaults in brackets): one deck per body, per turret, or
  copied from the builder [one per body]; turret cards from the squads' bag
  or a shelf of their own [the squads' bag]; which body first after the gun
  turret [mortar].

## First step, built: turrets fire a turret deck

- `Game::turret_deck` (None: plain strikes as before), loaded by
  `Cmd::Load(TURRET_DECK, ..)` (`TURRET_DECK` 255, so replays carry it:
  `load 255 blast`). One deck for every turret, as one gun is a squad's.
- `Game::building_units`: a turret's shot carries the deck's next cast
  (`Building::next`, as `Soldier::next`), its bullet card's strike, set
  pellets and range; its multicast volley; the deck's Range and Rate cards
  stretch every turret's reach and rate (`Rules::range_card`,
  `rate_card`). A shot is a turret's when `by` is None, so no new field:
  `Game::turret_mods`, `Game::mods_by`.
- `shots.rs`: flight cards (pierce, split, trigger, expire) read
  `mods_by`; a turret's let-out cast flies at a turret's strike and reach,
  over walls as its shells do.
- `rounds.rs` (`Rounds::after_step`, `lay_trails`): a turret hit casts from
  `rounds::turret_casts`; its trail cards lay trails. A `Pull` on a turret
  shot finds no gun to drag toward and shoves on along its line.
- Film key `tload=T,NAME` (a preset or card list, as `load=`); the headless
  rounds line now prints when the turrets carry a deck.
- Not built yet: any menu to set the deck (the armory), budget or slots for
  it, and the bodies.

Test: `buildings::tests::turrets_fire_the_turret_deck` (seed 3, a turret by
the core, the squad holding fire, 10 s): plain 0 casts; blast 19 shots, all
19 carrying and casting, 687 killed against 307 plain; the command
round-trips. Full `cargo test --release --locked` and clippy pass.

## Films

The shove page's kill box (`raw/film.sh`): the Fork, `hand=hold@150,265
bank=8000 rounds=plain`, three turrets at 172,240, 180,240 and 188,242 from
29 s, the turrets loaded from 29 s with the cards named; near view
`centre=170,252,10`, 960 x 600, llvmpipe. Clips 29-32 s at 30 fps
(`raw/make_clips.py`); counts over 29-39 s from `raw/counts.sh` (frames only
at 29, 32, 39 s; raw logs in `raw/counts/`). One game each: a look, not a
census.

| 10 s, 29-39 s | Turrets' shots killed | Casts | Cast effect | All kills | Slime at 39 s |
|---|---|---|---|---|---|
| plain turrets (`plaindeck`: one plain card) | 231 | 0 | | 661 | 24,669 |
| blast | 93 | 19 | 261 killed | 868 | 24,475 |
| sleep gas | 236 | 26 | 4,373 walkers gassed | 1,299 | 24,009 |
| corral + heave + plain | 427 | 57 shoves | 1,623 thrown | 847 | 24,465 |
| bomb | 61 | 28 | 4 killed, 1,648 flung | 541 | 24,810 |
| multicast + bomb + blast | 96 | 30 (51 beside another) | 43 killed, 1,093 flung | 601 | 24,730 |

"All kills" is the game's whole count, squad included; the squad fires the
same plain rounds in every row. Gas doubled the game's kills (the front
sleeps and everyone shoots it); corral threw the slime at the turrets' feet
(427 against 231); bomb flung it away from the guns and lowered kills: a
trade, which is the reason a turret deck is worth choosing.

Frame cost: no new pass; turret shots and casts use the soldiers' paths
(bursts batched per step since #387). Not timed on the 1070.

## Review

An independent agent reviews the design and the code from [review-brief.md](review-brief.md) (owner 21:37Z: "once you finish all this, use an independent agent to review").

The review (2026-11-16 ~21:55Z), in short:

- **Design.** The turret deck is free today: there's no budget, no slots and
  no shared bag, so the trade that keeps turtling honest doesn't exist yet.
  Each Range card doubles every turret's reach (`range_card` 1.0), and each
  Rate card doubles their rate. The shared bag, the slots and an armory
  entry must come before any body. The mortar has lost its reason to exist,
  since every turret fires over walls since #398. Bodies should come from
  what the model offers: targeting rules (thinnest link, leading edge,
  nearest the core), a contact mine on the `Thick` trigger's count, a scent
  or repellent writer, or a turret grown from the player's own mould.
  Bloons-style branch caps would push players to specialise. The slime
  needs a counter to carded turrets.
- **Tests.** One seed and a 10 s window with no pressure on the base: kills
  there are churn, and the slime grew in every row. The next measure is
  base defence under a raid, at equal card budgets, on paired seeds (about
  10 seeds x 6 arms), with slime mass near the base at 90 s as the deciding
  measure.
- **Code.** Cast kills by turrets are credited to soldiers (`turret_killed`
  counts frame shots only). Render ignored Range cards and pays only on
  frame shots. A Pull on a turret shot pushes away from the turret. `by`
  None as "a turret" is debt that step 2 must pay with a shooter id. Ammo
  spread wasn't read on turrets, and the fire interval had no 1-tick floor.
  `turret_mods` was rebuilt per turret. Range cards widen the aim search
  as reach squared, which hasn't been timed. The review checked that plain
  turrets are unchanged in this scene (plain and plaindeck counts match).

Done in this PR after the review: Render's reach follows the deck's Range
cards; the fire interval has a floor of 1 tick; the turret mods are read
once a step; turret aim ignoring ammo spread is now documented. Left for
step 2: the shared bag, slots and an armory entry; a shooter id in place of
`by` None, which also fixes cast-kill credit, Render on casts and Pull
toward the firing turret; and timing Range cards on the 1070.

## Step 2a: each shot knows which turret fired it (2026-11-16)

The review's shooter id, with the gun-card thread's OK (it owns
`shots.rs` and `rounds.rs` and had nothing in flight after #399).

- `Bullet::turret`, `Shot::turret` and the release carry the firing turret's
  `Game::buildings` index; the frame's shooters are `Firer` (soldier, cast,
  turret). `by` still means the soldier.
- **Cast kills are the turrets'.** `Rounds::flush` adds a turret cast's kills
  (shots, lodged Delay/Thick casts, shoves) to `Game::turret_killed`, so the
  `fort` line's soldier kills (killed - turret - fire) no longer book them
  to the squad.
- **Render pays on casts.** Every cast's kills in a rendering turret's reach
  pay as a shot's do ("whoever fired", `Game::render_pay`), soldiers' casts
  included.
- **Pull drags toward the gun that fired**: the soldier, or the turret
  (`Rounds::shove` takes the gun's position).

Tests: `turrets_fire_the_turret_deck` now also has every kill of a blasting
turret counted as the turret's (687 of 687) and its render paying 171.8 food
against 76.8 for a plain one in 10 s; `a_pull_heads_for_the_gun_that_fired`.

## Step 2b: the turrets' cards come out of the squad's bag (2026-11-16)

The review's other point: a free turret deck was a pure buff to staying
home. Now the run shop has one bag for both.

- With turrets owned, the shop's bottom bar has **Edit the turrets' cards**
  (`Pick::EditTurrets`). The gun slots then show the turrets' rack, titled
  "Every turret": 2 slots and a budget of 4, plus a slot and 2 budget for
  each level of Turret power (`Run::turret_rack`). Click again, buy or pick
  a gun, and the slots go back to the squad's gun.
- A card on the turrets has left the bag, so the squad doesn't fire it. The
  two decks keep their order across the switch (`Run::aside`).
- At landing the squad loads `Run::squad_cards` and the game's turrets
  load `Run::turret_cards` (`Cmd::Load(TURRET_DECK, ..)`, so it replays).
  With no turrets, or nothing on them, nothing changes.
- Headless: `larder-world run=1 menu=shop-turrets` opens a run three
  planets in with the slots on the turrets and the first bag card that fits
  on them.

Tests: `the_turrets_take_cards_from_the_squads_bag` (run.rs) and the
landing check added to `a_run_plays_a_planet_then_the_shop_then_the_next`
(the game's turret deck is the gas put on it; the squad still fires blast
and wide).

Not yet measured: whether giving cards to the turrets is a fair trade
against the squad's gun. That's the paired raid-on-base census the review
suggested, and it waits for the balance pass ("build first, balance later").

## Step 3: the mortar, the first turret body (2026-11-17)

The owner's card (2026-11-17 00:47Z): "Mortar anyway", over a placement
mine, a picky-targeting turret and a sprayer. Every turret already fires over
walls (#398), so the mortar's point is the one the owner gave it on 11-16
at 22:10Z: "a longer range than anything else in the game".

**What it is** (`Kind::Mortar`, `BuildRules::mortar_*`):
- A building on the build page (B, 600 food, 200 health). In a run it's
  locked until the run shop's **Mortars** item (250 points), which needs
  Turrets first. That's the tree's first node. Turret power levels apply
  to mortars too. On the build page it takes slot (1, 2). That page is full only in the
  RTS round with the player's own mould, where the rally point moves there,
  so the mortar is left off in that one case.
- Every 4 s it lobs a shell at the thickest slime it sees within **80 dish
  cells**, over walls. The long gun averages 40 on the first planet and 56
  at its deepest, so the mortar starts longest. Its reach isn't capped
  (owner, 22:11Z), and Range cards stretch it.
- It can't hit slime within **8 cells** of itself, its blind ring. Slime
  at its foot is for turrets and soldiers.
- **The shell is in the air** for 0.6 s plus its distance over 35 cells a
  second (`Game::shells`, `buildings::Shell`), so 1.3 to 2.9 s. Nothing on
  its way is hit. It comes down as a short leg into its landing point, a
  strike 3 cells wide (6 fine at scale 2; a turret's is 2.5), fired by the
  mortar (`Firer`). That means the turrets' deck's next cast goes off
  there, and its kills, Render and Pull go to it as a turret's do. Pellets
  and multicast fan shells round the aim.
- The window draws each shell from those numbers: along its arc, lifted
  toward the top of the screen, over a shadow that darkens as it drops. The
  mortar is the turret's base with a short wide tube turned to its last
  shell, drawn as shapes in the view's shader (building kind 7). It sinks
  back and its bore flares as a shell leaves. Placing or hovering one shows
  its reach and blind rings.
- Frame cost: it looks only on the frame before it may fire (sight 0
  otherwise). Its sight then reads about 5,000 four-cell blocks at reach 160
  fine cells, about 80,000 cell reads once every 4 s (an estimate, not
  timed). The 1070 timing is still to do, as for Range cards.

**Seen** (`raw/mortar_film.sh`, `page/mortar.mp4`, counts in
`raw/counts/mortar.txt`): on the Fork the squad held at 150,265 on plain
rounds, with a mortar standing from 20 s at 70,196, about 60 cells behind
it. Its shells came down on the trunk beside the squad and on the arm by
the left crystal: 5 shells in 20 s killed 390 walkers, about 78 a shell.
That's about 20 a second; one plain turret in the kill box killed about 8 a
second. One film is a look, not a balance result. If play shows it's too
strong, the knobs are `mortar_every` and `mortar_blast`. In the first try
(every 2 s, 3.5 cells) 10 shells killed 731 in 20 s.

Tests: `a_mortar_lobs_shells_that_strike_where_they_land`. Three shells in
12 s on the race world landed 19 ticks after firing, 23 cells out, and
killed 76. Every kill was the mortar's and counted as the turrets'. With
its blind ring past everything it fires nothing, and `Cmd::Build(Mortar)`
round-trips. `command_ids_are_one_registry` now takes MORTAR (37) and
sprite 7.

Next bodies, as the owner picks: the sprayer, the fan, the picky turret,
the mine. A deck for each body (the proposal's option A) is still open; for
now the mortar fires the one turret deck.
